use std::any::Any;
use std::fs::{self, File};
use std::io::{Read as _, Write as _};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::Once;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde_json::Value;
use sha2::{Digest, Sha256};

const LOOPBACK_V4: &str = "127.0.0.1";
const LOOPBACK_V6: &str = "::1";
const DISCOVERY_SEED: &str = "localhost";
const SHARED_COOKIE: &str = "mesh-m044-s03-cookie";
const MEMBERSHIP_TIMEOUT: Duration = Duration::from_secs(20);
const DIAGNOSTIC_TIMEOUT: Duration = Duration::from_secs(20);

static BUILD_MESH_RT_ONCE: Once = Once::new();
static BUILD_CLUSTER_PROOF_ONCE: Once = Once::new();

#[derive(Clone, Debug)]
struct ClusterProofConfig {
    node_basename: String,
    advertise_host: String,
    cluster_port: u16,
    http_port: u16,
    work_delay_ms: u64,
    cluster_role: String,
    promotion_epoch: u64,
}

struct SpawnedProcess {
    child: Child,
    stdout_path: PathBuf,
    stderr_path: PathBuf,
}

struct StoppedProcess {
    stdout: String,
    stderr: String,
    combined: String,
    stdout_path: PathBuf,
    stderr_path: PathBuf,
}

struct HttpResponse {
    status_code: u16,
    body: String,
    raw: String,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn meshc_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_meshc"))
}

fn command_output_text(output: &Output) -> String {
    format!(
        "status: {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn panic_payload_to_string(payload: Box<dyn Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<&str>() {
        (*message).to_string()
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else {
        "non-string panic payload".to_string()
    }
}

fn assert_command_success(output: &Output, description: &str) {
    assert!(
        output.status.success(),
        "{description} failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn ensure_mesh_rt_staticlib() {
    BUILD_MESH_RT_ONCE.call_once(|| {
        let output = Command::new("cargo")
            .current_dir(repo_root())
            .args(["build", "-q", "-p", "mesh-rt"])
            .output()
            .expect("failed to invoke cargo build -p mesh-rt");
        assert_command_success(&output, "cargo build -p mesh-rt");
    });
}

fn build_cluster_proof() {
    BUILD_CLUSTER_PROOF_ONCE.call_once(|| {
        ensure_mesh_rt_staticlib();
        let output = Command::new(meshc_bin())
            .current_dir(repo_root())
            .args(["build", "cluster-proof"])
            .output()
            .expect("failed to invoke meshc build cluster-proof");
        assert_command_success(&output, "meshc build cluster-proof");
    });
}

fn cluster_proof_binary() -> PathBuf {
    repo_root().join("cluster-proof").join("cluster-proof")
}

fn dual_stack_cluster_port() -> u16 {
    for _ in 0..64 {
        let listener = TcpListener::bind((LOOPBACK_V4, 0))
            .expect("failed to bind IPv4 loopback for ephemeral cluster port");
        let port = listener
            .local_addr()
            .expect("failed to read IPv4 ephemeral cluster port")
            .port();
        drop(listener);

        if TcpListener::bind((LOOPBACK_V4, port)).is_ok()
            && TcpListener::bind((LOOPBACK_V6, port)).is_ok()
        {
            return port;
        }
    }

    panic!("failed to find a dual-stack cluster port");
}

fn unused_http_port() -> u16 {
    TcpListener::bind((LOOPBACK_V4, 0))
        .expect("failed to bind ephemeral HTTP port")
        .local_addr()
        .expect("failed to read ephemeral HTTP port")
        .port()
}

fn artifact_dir(test_name: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = repo_root()
        .join(".tmp")
        .join("m044-s03")
        .join(format!("{test_name}-{stamp}"));
    fs::create_dir_all(&dir).expect("failed to create e2e artifact dir");
    dir
}

fn write_artifact(path: &Path, contents: impl AsRef<str>) {
    fs::write(path, contents.as_ref())
        .unwrap_or_else(|error| panic!("failed to write artifact {}: {error}", path.display()));
}

fn node_log_paths(log_dir: &Path, label: &str) -> (PathBuf, PathBuf) {
    let stdout_path = log_dir.join(format!("{label}.stdout.log"));
    let stderr_path = log_dir.join(format!("{label}.stderr.log"));
    (stdout_path, stderr_path)
}

fn spawn_cluster_proof(config: ClusterProofConfig, artifacts: &Path) -> SpawnedProcess {
    let binary = cluster_proof_binary();
    assert!(
        binary.exists(),
        "cluster-proof binary not found at {}",
        binary.display()
    );

    let (stdout_path, stderr_path) = node_log_paths(artifacts, &config.node_basename);
    let stdout = File::create(&stdout_path).expect("failed to create stdout log");
    let stderr = File::create(&stderr_path).expect("failed to create stderr log");

    let child = Command::new(&binary)
        .current_dir(repo_root().join("cluster-proof"))
        .env("PORT", config.http_port.to_string())
        .env("MESH_CLUSTER_PORT", config.cluster_port.to_string())
        .env("MESH_CLUSTER_COOKIE", SHARED_COOKIE)
        .env("MESH_DISCOVERY_SEED", DISCOVERY_SEED)
        .env("MESH_NODE_NAME", expected_node_name(&config))
        .env(
            "CLUSTER_PROOF_WORK_DELAY_MS",
            config.work_delay_ms.to_string(),
        )
        .env("MESH_CONTINUITY_ROLE", &config.cluster_role)
        .env(
            "MESH_CONTINUITY_PROMOTION_EPOCH",
            config.promotion_epoch.to_string(),
        )
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .spawn()
        .expect("failed to start cluster-proof");

    SpawnedProcess {
        child,
        stdout_path,
        stderr_path,
    }
}

fn stop_process(mut spawned: SpawnedProcess) -> StoppedProcess {
    let _ = spawned.child.kill();
    let _ = spawned.child.wait();

    let stdout = fs::read_to_string(&spawned.stdout_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", spawned.stdout_path.display(), e));
    let stderr = fs::read_to_string(&spawned.stderr_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", spawned.stderr_path.display(), e));
    let combined = format!("{stdout}{stderr}");

    StoppedProcess {
        stdout,
        stderr,
        combined,
        stdout_path: spawned.stdout_path,
        stderr_path: spawned.stderr_path,
    }
}

fn send_request(port: u16, request: &str) -> std::io::Result<HttpResponse> {
    let mut stream = TcpStream::connect((LOOPBACK_V4, port))?;
    stream.set_read_timeout(Some(Duration::from_secs(15)))?;
    stream.write_all(request.as_bytes())?;

    let mut raw = String::new();
    stream.read_to_string(&mut raw)?;
    let mut parts = raw.splitn(2, "\r\n\r\n");
    let headers = parts.next().unwrap_or("");
    let body = parts.next().unwrap_or("").to_string();
    let status_code = headers
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse::<u16>().ok())
        .unwrap_or(0);

    Ok(HttpResponse {
        status_code,
        body,
        raw,
    })
}

fn try_get_json(port: u16, path: &str) -> std::io::Result<HttpResponse> {
    let request = format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
    send_request(port, &request)
}

fn post_json(port: u16, path: &str, body: &str) -> HttpResponse {
    let request = format!(
        "POST {path} HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    send_request(port, &request).expect("POST request failed")
}

fn parse_json_response(
    artifacts: &Path,
    name: &str,
    response: &HttpResponse,
    context: &str,
) -> Value {
    let raw_path = artifacts.join(format!("{name}.http"));
    write_artifact(&raw_path, &response.raw);
    match serde_json::from_str::<Value>(&response.body) {
        Ok(json) => {
            let json_path = artifacts.join(format!("{name}.json"));
            write_artifact(
                &json_path,
                serde_json::to_string_pretty(&json).expect("json pretty print failed"),
            );
            json
        }
        Err(error) => {
            let body_path = artifacts.join(format!("{name}.body.txt"));
            write_artifact(&body_path, &response.body);
            panic!(
                "expected JSON body for {context}, got {error}. raw response: {} body artifact: {}",
                raw_path.display(),
                body_path.display()
            );
        }
    }
}

fn required_str(json: &Value, field: &str) -> String {
    json[field]
        .as_str()
        .unwrap_or_else(|| panic!("missing string field `{field}` in {json}"))
        .to_string()
}

fn required_u64(json: &Value, field: &str) -> u64 {
    json[field]
        .as_u64()
        .unwrap_or_else(|| panic!("missing u64 field `{field}` in {json}"))
}

fn required_string_list(json: &Value, field: &str) -> Vec<String> {
    json[field]
        .as_array()
        .unwrap_or_else(|| panic!("missing array field `{field}` in {json}"))
        .iter()
        .map(|value| {
            value
                .as_str()
                .unwrap_or_else(|| panic!("non-string entry in `{field}`: {json}"))
                .to_string()
        })
        .collect()
}

fn sorted(values: &[String]) -> Vec<String> {
    let mut copy = values.to_vec();
    copy.sort();
    copy
}

fn expected_node_name(config: &ClusterProofConfig) -> String {
    let host = if config.advertise_host.contains(':') {
        format!("[{}]", config.advertise_host)
    } else {
        config.advertise_host.clone()
    };
    format!("{}@{}:{}", config.node_basename, host, config.cluster_port)
}

fn wait_for_membership(
    artifacts: &Path,
    name: &str,
    port: u16,
    expected_self: &str,
    expected_membership: &[String],
    expected_role: &str,
    expected_epoch: u64,
    expected_health: &str,
) -> Value {
    let start = Instant::now();
    let mut last_response = None;
    while start.elapsed() < MEMBERSHIP_TIMEOUT {
        match try_get_json(port, "/membership") {
            Ok(response) => {
                let json = parse_json_response(artifacts, name, &response, "membership response");
                last_response = Some(json.clone());
                if required_str(&json, "self") == expected_self
                    && sorted(&required_string_list(&json, "membership"))
                        == sorted(expected_membership)
                    && required_str(&json, "cluster_role") == expected_role
                    && required_u64(&json, "promotion_epoch") == expected_epoch
                    && required_str(&json, "replication_health") == expected_health
                {
                    return json;
                }
            }
            Err(_) => {}
        }
        sleep(Duration::from_millis(200));
    }

    panic!(
        "membership did not converge on :{} within {:?}; last response: {:?}",
        port, MEMBERSHIP_TIMEOUT, last_response
    );
}

fn run_meshc_cluster(artifacts: &Path, label: &str, args: &[&str], cookie: &str) -> Output {
    let output = Command::new(meshc_bin())
        .current_dir(repo_root())
        .env("MESH_CLUSTER_COOKIE", cookie)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("failed to run meshc cluster command {:?}: {error}", args));
    write_artifact(
        &artifacts.join(format!("{label}.log")),
        command_output_text(&output),
    );
    output
}

fn deterministic_sort_score(value: &str) -> u64 {
    let digest = Sha256::digest(value.as_bytes());
    u64::from_le_bytes(digest[..8].try_into().expect("digest score slice"))
}

fn placement_score(request_key: &str, node_name: &str) -> u64 {
    deterministic_sort_score(&format!("{request_key}::{node_name}"))
}

fn request_key_matches_placement(
    request_key: &str,
    desired_owner: &str,
    desired_replica: &str,
) -> bool {
    let owner_score = placement_score(request_key, desired_owner);
    let replica_score = placement_score(request_key, desired_replica);
    owner_score < replica_score
}

fn submit_request_for_owner(
    artifacts: &Path,
    port: u16,
    desired_owner: &str,
    desired_replica: &str,
    prefix: &str,
) -> Value {
    let mut first_success = None;

    for idx in 0..128 {
        let request_key = format!("{prefix}-key-{idx}");
        if !request_key_matches_placement(&request_key, desired_owner, desired_replica) {
            continue;
        }
        let payload = format!("payload-{idx}");
        let body = format!(r#"{{"request_key":"{request_key}","payload":"{payload}"}}"#);
        let response = post_json(port, "/work", &body);
        let json = parse_json_response(
            artifacts,
            &format!("submit-{idx}"),
            &response,
            "submit response",
        );
        if response.status_code == 200 || response.status_code == 202 {
            assert_eq!(required_str(&json, "request_key"), request_key);
            if required_str(&json, "owner_node") == desired_owner
                && required_str(&json, "replica_node") == desired_replica
            {
                return json;
            }
            if first_success.is_none() {
                first_success = Some(json);
            }
        }
    }

    if let Some(json) = first_success {
        return json;
    }

    panic!(
        "failed to find deterministic request key for owner={} replica={}",
        desired_owner, desired_replica
    );
}

fn wait_for_diagnostic_transition(
    artifacts: &Path,
    label: &str,
    target: &str,
    request_key: &str,
    transition: &str,
) -> Value {
    let start = Instant::now();
    let mut last_json = None;
    while start.elapsed() < DIAGNOSTIC_TIMEOUT {
        let output = run_meshc_cluster(
            artifacts,
            &format!("{label}-poll"),
            &["cluster", "diagnostics", target, "--json"],
            SHARED_COOKIE,
        );
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let json: Value = serde_json::from_str(&stdout)
                .unwrap_or_else(|error| panic!("invalid diagnostics JSON: {error}\n{stdout}"));
            last_json = Some(json.clone());
            let matched = json["entries"]
                .as_array()
                .map(|entries| {
                    entries.iter().any(|entry| {
                        entry["transition"].as_str() == Some(transition)
                            && entry["request_key"].as_str() == Some(request_key)
                    })
                })
                .unwrap_or(false);
            if matched {
                let out_path = artifacts.join(format!("{label}.json"));
                write_artifact(
                    &out_path,
                    serde_json::to_string_pretty(&json)
                        .expect("serialize matched diagnostics json"),
                );
                return json;
            }
        }
        sleep(Duration::from_millis(250));
    }

    panic!(
        "diagnostic transition {} for {} did not appear within {:?}; last diagnostics: {:?}",
        transition, request_key, DIAGNOSTIC_TIMEOUT, last_json
    );
}

#[test]
fn m044_s03_operator_status_json_reports_runtime_truth_and_auth_failures_fail_closed() {
    build_cluster_proof();
    let artifacts = artifact_dir("operator-status-truth");
    let cluster_port = dual_stack_cluster_port();
    let primary = ClusterProofConfig {
        node_basename: "operator-primary".to_string(),
        advertise_host: LOOPBACK_V4.to_string(),
        cluster_port,
        http_port: unused_http_port(),
        work_delay_ms: 0,
        cluster_role: "primary".to_string(),
        promotion_epoch: 0,
    };
    let standby = ClusterProofConfig {
        node_basename: "operator-standby".to_string(),
        advertise_host: LOOPBACK_V6.to_string(),
        cluster_port,
        http_port: unused_http_port(),
        work_delay_ms: 0,
        cluster_role: "standby".to_string(),
        promotion_epoch: 0,
    };

    let primary_node = expected_node_name(&primary);
    let standby_node = expected_node_name(&standby);
    let expected_membership = vec![primary_node.clone(), standby_node.clone()];

    let primary_proc = spawn_cluster_proof(primary.clone(), &artifacts);
    let standby_proc = spawn_cluster_proof(standby.clone(), &artifacts);

    let result = std::panic::catch_unwind(|| {
        wait_for_membership(
            &artifacts,
            "membership-primary",
            primary.http_port,
            &primary_node,
            &expected_membership,
            "primary",
            0,
            "local_only",
        );
        wait_for_membership(
            &artifacts,
            "membership-standby",
            standby.http_port,
            &standby_node,
            &expected_membership,
            "standby",
            0,
            "local_only",
        );

        let status = run_meshc_cluster(
            &artifacts,
            "cluster-status-primary",
            &["cluster", "status", &primary_node, "--json"],
            SHARED_COOKIE,
        );
        assert!(
            status.status.success(),
            "cluster status should succeed:\n{}",
            command_output_text(&status)
        );
        let status_json: Value =
            serde_json::from_slice(&status.stdout).expect("valid cluster status JSON");
        assert_eq!(
            required_str(&status_json["membership"], "local_node"),
            primary_node
        );
        assert_eq!(
            sorted(&required_string_list(
                &status_json["membership"],
                "peer_nodes"
            )),
            vec![standby_node.clone()]
        );
        assert_eq!(
            sorted(&required_string_list(&status_json["membership"], "nodes")),
            sorted(&expected_membership)
        );
        assert_eq!(
            required_str(&status_json["authority"], "cluster_role"),
            "primary"
        );
        assert_eq!(
            required_u64(&status_json["authority"], "promotion_epoch"),
            0
        );
        assert_eq!(
            required_str(&status_json["authority"], "replication_health"),
            "local_only"
        );

        let membership_after = parse_json_response(
            &artifacts,
            "membership-primary-after-cli",
            &try_get_json(primary.http_port, "/membership")
                .expect("membership after cluster status"),
            "membership after cluster status",
        );
        assert_eq!(
            sorted(&required_string_list(&membership_after, "membership")),
            sorted(&expected_membership),
            "meshc cluster status must not add the CLI as a visible peer"
        );

        let auth_failure = Command::new(meshc_bin())
            .current_dir(repo_root())
            .args([
                "cluster",
                "status",
                &primary_node,
                "--json",
                "--cookie",
                "wrong-cookie",
            ])
            .output()
            .expect("failed to run auth failure cluster status");
        write_artifact(
            &artifacts.join("cluster-status-auth-failure.log"),
            command_output_text(&auth_failure),
        );
        assert!(
            !auth_failure.status.success(),
            "wrong cookie should fail closed"
        );
        let stderr = String::from_utf8_lossy(&auth_failure.stderr);
        assert!(
            stderr.contains("error:")
                && (stderr.contains("cookie mismatch")
                    || stderr.contains("authentication failed")
                    || stderr.contains("handshake")),
            "auth failure should mention the transient query/auth seam, got:\n{stderr}"
        );
    });

    let primary_logs = stop_process(primary_proc);
    let standby_logs = stop_process(standby_proc);
    write_artifact(
        &artifacts.join("primary.combined.log"),
        &primary_logs.combined,
    );
    write_artifact(
        &artifacts.join("standby.combined.log"),
        &standby_logs.combined,
    );

    if let Err(payload) = result {
        panic!("{}", panic_payload_to_string(payload));
    }
}

#[test]
fn m044_s03_operator_continuity_and_diagnostics_report_runtime_truth() {
    build_cluster_proof();
    let artifacts = artifact_dir("operator-continuity-diagnostics");
    let cluster_port = dual_stack_cluster_port();
    let primary = ClusterProofConfig {
        node_basename: "continuity-primary".to_string(),
        advertise_host: LOOPBACK_V4.to_string(),
        cluster_port,
        http_port: unused_http_port(),
        work_delay_ms: 6_000,
        cluster_role: "primary".to_string(),
        promotion_epoch: 0,
    };
    let standby = ClusterProofConfig {
        node_basename: "continuity-standby".to_string(),
        advertise_host: LOOPBACK_V6.to_string(),
        cluster_port,
        http_port: unused_http_port(),
        work_delay_ms: 0,
        cluster_role: "standby".to_string(),
        promotion_epoch: 0,
    };

    let primary_node = expected_node_name(&primary);
    let standby_node = expected_node_name(&standby);
    let expected_membership = vec![primary_node.clone(), standby_node.clone()];

    let primary_proc = spawn_cluster_proof(primary.clone(), &artifacts);
    let standby_proc = spawn_cluster_proof(standby.clone(), &artifacts);
    let mut selected_request_key = None;
    let mut selected_owner_node = None;
    let mut selected_replica_node = None;
    let mut selected_query_target = None;
    let mut selected_kill_node = None;
    let mut selected_transition = None;

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        wait_for_membership(
            &artifacts,
            "continuity-membership-primary",
            primary.http_port,
            &primary_node,
            &expected_membership,
            "primary",
            0,
            "local_only",
        );
        wait_for_membership(
            &artifacts,
            "continuity-membership-standby",
            standby.http_port,
            &standby_node,
            &expected_membership,
            "standby",
            0,
            "local_only",
        );

        let submitted = submit_request_for_owner(
            &artifacts,
            primary.http_port,
            &primary_node,
            &standby_node,
            "m044-s03",
        );
        let request_key = required_str(&submitted, "request_key");
        let owner_node = required_str(&submitted, "owner_node");
        let replica_node = required_str(&submitted, "replica_node");
        selected_request_key = Some(request_key.clone());
        selected_owner_node = Some(owner_node.clone());
        selected_replica_node = Some(replica_node.clone());

        let submit_status = run_meshc_cluster(
            &artifacts,
            "cluster-status-owner-probe",
            &["cluster", "status", &owner_node, "--json"],
            SHARED_COOKIE,
        );
        let continuity_target = if submit_status.status.success() {
            owner_node.clone()
        } else {
            replica_node.clone()
        };
        let transition = if continuity_target == owner_node {
            "degraded"
        } else {
            "owner_lost"
        };
        let kill_node = if continuity_target == owner_node {
            replica_node.clone()
        } else {
            owner_node.clone()
        };
        selected_query_target = Some(continuity_target.clone());
        selected_kill_node = Some(kill_node.clone());
        selected_transition = Some(transition.to_string());

        let continuity = run_meshc_cluster(
            &artifacts,
            "cluster-continuity",
            &[
                "cluster",
                "continuity",
                &continuity_target,
                &request_key,
                "--json",
            ],
            SHARED_COOKIE,
        );
        assert!(
            continuity.status.success(),
            "cluster continuity should succeed:\n{}",
            command_output_text(&continuity)
        );
        let continuity_json: Value =
            serde_json::from_slice(&continuity.stdout).expect("valid continuity JSON");
        let record = &continuity_json["record"];
        assert_eq!(required_str(record, "request_key"), request_key);
        assert_eq!(required_str(record, "owner_node"), owner_node);
        assert_eq!(required_str(record, "replica_node"), replica_node);
        assert_eq!(
            required_str(record, "declared_handler_runtime_name"),
            "Work.execute_declared_work"
        );
        assert!(
            ["submitted", "processing", "completed"]
                .contains(&required_str(record, "phase").as_str()),
            "continuity phase should be a real runtime value: {}",
            continuity_json
        );
    }));

    let mut primary_proc = primary_proc;
    let mut standby_proc = standby_proc;

    if result.is_ok() {
        let request_key = selected_request_key
            .clone()
            .expect("submit should record a request_key");
        let owner_node = selected_owner_node
            .clone()
            .expect("submit should record an owner node");
        let replica_node = selected_replica_node
            .clone()
            .expect("submit should record a replica node");
        let query_target = selected_query_target
            .clone()
            .expect("submit should record a query target");
        let kill_node = selected_kill_node
            .clone()
            .expect("submit should record a kill target");
        let transition = selected_transition
            .clone()
            .expect("submit should record an expected transition");

        if kill_node == primary_node {
            let _ = primary_proc.child.kill();
            let _ = primary_proc.child.wait();
        } else if kill_node == standby_node {
            let _ = standby_proc.child.kill();
            let _ = standby_proc.child.wait();
        } else {
            panic!("unexpected kill node returned by submit response: {kill_node}");
        }

        let diagnostics = wait_for_diagnostic_transition(
            &artifacts,
            "cluster-diagnostics",
            &query_target,
            &request_key,
            &transition,
        );
        let matched = diagnostics["entries"]
            .as_array()
            .and_then(|entries| {
                entries.iter().find(|entry| {
                    entry["transition"].as_str() == Some(transition.as_str())
                        && entry["request_key"].as_str() == Some(request_key.as_str())
                })
            })
            .expect("expected diagnostic entry should be present");
        if transition == "degraded" {
            assert_eq!(
                matched["replica_node"].as_str(),
                Some(replica_node.as_str())
            );
            assert!(matched["reason"]
                .as_str()
                .unwrap_or("")
                .contains("replica_lost"));
        } else {
            assert_eq!(matched["owner_node"].as_str(), Some(owner_node.as_str()));
            assert!(matched["reason"]
                .as_str()
                .unwrap_or("")
                .contains("owner_lost"));
        }
    }

    let primary_logs = stop_process(primary_proc);
    let standby_logs = stop_process(standby_proc);
    write_artifact(
        &artifacts.join("primary.combined.log"),
        &primary_logs.combined,
    );
    write_artifact(
        &artifacts.join("standby.combined.log"),
        &standby_logs.combined,
    );

    if let Err(payload) = result {
        panic!("{}", panic_payload_to_string(payload));
    }
}

#[test]
fn m044_s03_scaffold_generated_project_builds_and_reports_runtime_truth() {
    ensure_mesh_rt_staticlib();
    let artifacts = artifact_dir("scaffold-clustered");
    let temp = tempfile::tempdir().expect("create scaffold tempdir");
    let project_name = "scaffolded-clustered";
    let project_dir = temp.path().join(project_name);

    let init = Command::new(meshc_bin())
        .current_dir(temp.path())
        .args(["init", "--clustered", project_name])
        .output()
        .expect("failed to run meshc init --clustered");
    write_artifact(&artifacts.join("init.log"), command_output_text(&init));
    assert!(
        init.status.success(),
        "meshc init --clustered should succeed:\n{}",
        command_output_text(&init)
    );

    let manifest_source = fs::read_to_string(project_dir.join("mesh.toml"))
        .expect("read scaffolded clustered mesh.toml");
    let main_source = fs::read_to_string(project_dir.join("main.mpl"))
        .expect("read scaffolded clustered main.mpl");
    let work_source = fs::read_to_string(project_dir.join("work.mpl"))
        .expect("read scaffolded clustered work.mpl");
    let readme_source = fs::read_to_string(project_dir.join("README.md"))
        .expect("read scaffolded clustered README.md");
    write_artifact(&artifacts.join("mesh.toml"), &manifest_source);
    write_artifact(&artifacts.join("main.mpl"), &main_source);
    write_artifact(&artifacts.join("work.mpl"), &work_source);
    write_artifact(&artifacts.join("README.md"), &readme_source);

    assert!(manifest_source.contains("[package]"));
    assert!(!manifest_source.contains("[cluster]"));

    assert!(main_source.contains("Node.start_from_env()"));
    assert!(main_source.contains("BootstrapStatus"));
    assert!(main_source.contains("runtime bootstrap"));
    assert!(!main_source.contains("Continuity.submit_declared_work"));
    assert!(!main_source.contains("HTTP.serve"));
    assert!(!main_source.contains("/health"));
    assert!(!main_source.contains("/work"));
    assert!(!main_source.contains("MESH_CLUSTER_COOKIE"));
    assert!(!main_source.contains("MESH_NODE_NAME"));
    assert!(!main_source.contains("MESH_DISCOVERY_SEED"));
    assert!(!main_source.contains("Node.start("));

    assert!(work_source.contains("declared_work_runtime_name"));
    assert!(work_source.contains("clustered(work)"));
    assert!(work_source.contains("Work.execute_declared_work"));
    assert!(work_source.contains("1 + 1"));
    assert!(!work_source.contains("Continuity.submit_declared_work"));
    assert!(!work_source.contains("Continuity.mark_completed"));
    assert!(!work_source.contains("Timer.sleep"));
    assert!(!work_source.contains("owner_node"));
    assert!(!work_source.contains("replica_node"));

    assert!(readme_source.contains("Node.start_from_env()"));
    assert!(readme_source.contains("meshc cluster status"));
    assert!(readme_source.contains("meshc cluster continuity"));
    assert!(readme_source.contains("meshc cluster diagnostics"));
    assert!(readme_source.contains("MESH_CLUSTER_COOKIE"));
    assert!(readme_source.contains("MESH_NODE_NAME"));
    assert!(!readme_source.contains("Continuity.submit_declared_work"));
    assert!(!readme_source.contains("HTTP.serve"));
    assert!(!readme_source.contains("/health"));
    assert!(!readme_source.contains("/work"));

    let build = Command::new(meshc_bin())
        .current_dir(temp.path())
        .args(["build", project_name])
        .output()
        .expect("failed to run meshc build on clustered scaffold");
    write_artifact(&artifacts.join("build.log"), command_output_text(&build));
    assert!(
        build.status.success(),
        "clustered scaffold build should succeed:\n{}",
        command_output_text(&build)
    );

    let binary = project_dir.join(project_name);
    assert!(
        binary.exists(),
        "expected scaffold binary at {}",
        binary.display()
    );
}
