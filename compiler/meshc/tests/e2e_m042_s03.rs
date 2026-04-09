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

const LOOPBACK_V4: &str = "127.0.0.1";
const LOOPBACK_V6: &str = "::1";
const DISCOVERY_SEED: &str = "localhost";
const SHARED_COOKIE: &str = "mesh-m042-s03-cookie";
const MEMBERSHIP_TIMEOUT: Duration = Duration::from_secs(20);
const STATUS_TIMEOUT: Duration = Duration::from_secs(16);

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

struct SpawnedClusterProof {
    config: ClusterProofConfig,
    incarnation: usize,
    child: Child,
    stdout_path: PathBuf,
    stderr_path: PathBuf,
}

struct StoppedClusterProof {
    incarnation: usize,
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

struct SubmittedRequest {
    request_key: String,
    payload: String,
    response: Value,
    status_code: u16,
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

fn cluster_proof_binary() -> PathBuf {
    repo_root().join("cluster-proof").join("cluster-proof")
}

fn assert_command_success(output: &Output, description: &str) {
    assert!(
        output.status.success(),
        "{description} failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn build_cluster_proof() {
    BUILD_CLUSTER_PROOF_ONCE.call_once(|| {
        let runtime_output = Command::new("cargo")
            .current_dir(repo_root())
            .args(["build", "-p", "mesh-rt"])
            .output()
            .expect("failed to invoke cargo build -p mesh-rt");
        assert_command_success(&runtime_output, "cargo build -p mesh-rt");

        let output = Command::new(meshc_bin())
            .current_dir(repo_root())
            .args(["build", "cluster-proof"])
            .output()
            .expect("failed to invoke meshc build cluster-proof");
        assert_command_success(&output, "meshc build cluster-proof");
    });
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
        .join("m042-s03")
        .join(format!("{test_name}-{stamp}"));
    fs::create_dir_all(&dir).expect("failed to create e2e artifact dir");
    dir
}

fn write_artifact(path: &Path, contents: impl AsRef<str>) {
    fs::write(path, contents.as_ref())
        .unwrap_or_else(|error| panic!("failed to write artifact {}: {error}", path.display()));
}

fn node_log_paths(log_dir: &Path, node_basename: &str, incarnation: usize) -> (PathBuf, PathBuf) {
    let stdout_path = log_dir.join(format!("{node_basename}-run{incarnation}.stdout.log"));
    let stderr_path = log_dir.join(format!("{node_basename}-run{incarnation}.stderr.log"));
    (stdout_path, stderr_path)
}

fn spawn_cluster_proof(
    config: ClusterProofConfig,
    artifacts: &Path,
    incarnation: usize,
) -> SpawnedClusterProof {
    let binary = cluster_proof_binary();
    assert!(
        binary.exists(),
        "cluster-proof binary not found at {}. Run `meshc build cluster-proof` first.",
        binary.display()
    );

    let (stdout_path, stderr_path) = node_log_paths(artifacts, &config.node_basename, incarnation);
    let stdout = File::create(&stdout_path).expect("failed to create stdout log");
    let stderr = File::create(&stderr_path).expect("failed to create stderr log");

    let child = Command::new(&binary)
        .current_dir(repo_root().join("cluster-proof"))
        .env("PORT", config.http_port.to_string())
        .env("MESH_CLUSTER_PORT", config.cluster_port.to_string())
        .env("CLUSTER_PROOF_COOKIE", SHARED_COOKIE)
        .env("MESH_DISCOVERY_SEED", DISCOVERY_SEED)
        .env("CLUSTER_PROOF_NODE_BASENAME", &config.node_basename)
        .env("CLUSTER_PROOF_ADVERTISE_HOST", &config.advertise_host)
        .env("MESH_CONTINUITY_ROLE", &config.cluster_role)
        .env(
            "MESH_CONTINUITY_PROMOTION_EPOCH",
            config.promotion_epoch.to_string(),
        )
        .env(
            "CLUSTER_PROOF_WORK_DELAY_MS",
            config.work_delay_ms.to_string(),
        )
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .spawn()
        .expect("failed to start cluster-proof");

    SpawnedClusterProof {
        config,
        incarnation,
        child,
        stdout_path,
        stderr_path,
    }
}

fn collect_stopped_cluster_proof(
    mut child: Child,
    incarnation: usize,
    stdout_path: PathBuf,
    stderr_path: PathBuf,
) -> StoppedClusterProof {
    child
        .wait()
        .expect("failed to collect cluster-proof exit status");

    let stdout = fs::read_to_string(&stdout_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", stdout_path.display(), e));
    let stderr = fs::read_to_string(&stderr_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", stderr_path.display(), e));
    let combined = format!("{stdout}{stderr}");

    StoppedClusterProof {
        incarnation,
        stdout,
        stderr,
        combined,
        stdout_path,
        stderr_path,
    }
}

fn stop_cluster_proof(spawned: SpawnedClusterProof) -> StoppedClusterProof {
    let SpawnedClusterProof {
        incarnation,
        mut child,
        stdout_path,
        stderr_path,
        ..
    } = spawned;

    let _ = Command::new("kill")
        .args(["-TERM", &child.id().to_string()])
        .status();
    sleep(Duration::from_millis(250));
    if child
        .try_wait()
        .expect("failed to probe cluster-proof exit status")
        .is_none()
    {
        let _ = child.kill();
    }

    collect_stopped_cluster_proof(child, incarnation, stdout_path, stderr_path)
}

fn kill_cluster_proof(spawned: SpawnedClusterProof) -> StoppedClusterProof {
    let SpawnedClusterProof {
        incarnation,
        mut child,
        stdout_path,
        stderr_path,
        ..
    } = spawned;

    let _ = Command::new("kill")
        .args(["-TERM", &child.id().to_string()])
        .status();
    sleep(Duration::from_millis(250));
    if child
        .try_wait()
        .expect("failed to probe cluster-proof exit status")
        .is_none()
    {
        let _ = child.kill();
    }

    collect_stopped_cluster_proof(child, incarnation, stdout_path, stderr_path)
}

fn assert_cluster_proof_running(spawned: &mut SpawnedClusterProof, context: &str) {
    if let Some(status) = spawned.child.try_wait().unwrap_or_else(|e| {
        panic!(
            "failed to probe {} exit status: {}",
            spawned.config.node_basename, e
        )
    }) {
        panic!(
            "cluster-proof node {} run{} exited early while {}: status={:?}; stdout_log={}; stderr_log={}",
            spawned.config.node_basename,
            spawned.incarnation,
            context,
            status,
            spawned.stdout_path.display(),
            spawned.stderr_path.display()
        );
    }
}

fn send_request(port: u16, request: &str) -> std::io::Result<HttpResponse> {
    let mut stream = TcpStream::connect((LOOPBACK_V4, port))?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
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

fn archive_raw_response(artifacts: &Path, name: &str, response: &HttpResponse) -> PathBuf {
    let path = artifacts.join(format!("{name}.http"));
    write_artifact(&path, &response.raw);
    path
}

fn parse_json_response(
    artifacts: &Path,
    name: &str,
    response: &HttpResponse,
    context: &str,
) -> Value {
    let raw_path = archive_raw_response(artifacts, name, response);
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

fn json_body(
    artifacts: &Path,
    name: &str,
    response: &HttpResponse,
    expected_status: u16,
    context: &str,
) -> Value {
    if response.status_code != expected_status {
        let raw_path = archive_raw_response(artifacts, name, response);
        panic!(
            "expected HTTP {expected_status} for {context}, got raw response in {}:\n{}",
            raw_path.display(),
            response.raw
        );
    }
    parse_json_response(artifacts, name, response, context)
}

fn required_str(json: &Value, field: &str) -> String {
    json[field]
        .as_str()
        .unwrap_or_else(|| panic!("missing string field `{field}` in {json}"))
        .to_string()
}

fn required_bool(json: &Value, field: &str) -> bool {
    json[field]
        .as_bool()
        .unwrap_or_else(|| panic!("missing bool field `{field}` in {json}"))
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
    watched_nodes: &mut [&mut SpawnedClusterProof],
    expected_self: &str,
    expected_membership: &[String],
) -> Value {
    const REQUIRED_STABLE_POLLS: usize = 5;

    let start = Instant::now();
    let mut last_body = String::new();
    let mut stable_polls = 0usize;
    let mut last_match: Option<Value> = None;

    while start.elapsed() < MEMBERSHIP_TIMEOUT {
        for spawned in watched_nodes.iter_mut() {
            assert_cluster_proof_running(
                spawned,
                &format!("waiting for membership convergence on :{port}"),
            );
        }

        match try_get_json(port, "/membership") {
            Ok(response) => {
                if response.status_code == 200 {
                    let json =
                        parse_json_response(artifacts, name, &response, "membership response");
                    last_body = response.body.clone();
                    let self_name = required_str(&json, "self");
                    let membership = json["membership"]
                        .as_array()
                        .unwrap_or_else(|| panic!("membership array missing from {json}"))
                        .iter()
                        .map(|value| value.as_str().unwrap().to_string())
                        .collect::<Vec<_>>();
                    if self_name == expected_self
                        && sorted(&membership) == sorted(expected_membership)
                    {
                        stable_polls += 1;
                        last_match = Some(json.clone());
                        if stable_polls >= REQUIRED_STABLE_POLLS {
                            return last_match.expect("stable membership snapshot missing");
                        }
                    } else {
                        stable_polls = 0;
                        last_match = None;
                    }
                } else {
                    archive_raw_response(artifacts, name, &response);
                    last_body = response.raw.clone();
                    stable_polls = 0;
                    last_match = None;
                }
            }
            Err(error) => {
                last_body = error.to_string();
                write_artifact(&artifacts.join(format!("{name}.error.txt")), &last_body);
                stable_polls = 0;
                last_match = None;
            }
        }
        sleep(Duration::from_millis(100));
    }

    let timeout_path = artifacts.join(format!("{name}.timeout.txt"));
    write_artifact(&timeout_path, &last_body);
    panic!(
        "membership did not converge on :{} within {:?}; last body archived at {}",
        port,
        MEMBERSHIP_TIMEOUT,
        timeout_path.display()
    );
}

fn wait_for_status_condition<F>(
    artifacts: &Path,
    name: &str,
    port: u16,
    watched_nodes: &mut [&mut SpawnedClusterProof],
    request_key: &str,
    predicate_description: &str,
    mut predicate: F,
) -> Value
where
    F: FnMut(&Value) -> bool,
{
    let start = Instant::now();
    let path = format!("/work/{request_key}");
    let mut last_body = String::new();

    while start.elapsed() < STATUS_TIMEOUT {
        for spawned in watched_nodes.iter_mut() {
            assert_cluster_proof_running(
                spawned,
                &format!("waiting for {predicate_description} on :{port}"),
            );
        }

        match try_get_json(port, &path) {
            Ok(response) => {
                if response.status_code == 200 {
                    let json =
                        parse_json_response(artifacts, name, &response, predicate_description);
                    last_body = response.body.clone();
                    if predicate(&json) {
                        return json;
                    }
                } else {
                    archive_raw_response(artifacts, name, &response);
                    last_body = response.raw.clone();
                }
            }
            Err(error) => {
                last_body = error.to_string();
                write_artifact(&artifacts.join(format!("{name}.error.txt")), &last_body);
            }
        }
        sleep(Duration::from_millis(100));
    }

    let timeout_path = artifacts.join(format!("{name}.timeout.txt"));
    write_artifact(&timeout_path, &last_body);
    panic!(
        "request {request_key} never satisfied {predicate_description}; last body archived at {}",
        timeout_path.display()
    );
}

fn find_submit_matching_placement(
    artifacts: &Path,
    prefix: &str,
    port: u16,
    watched_nodes: &mut [&mut SpawnedClusterProof],
    desired_owner: &str,
    desired_replica: &str,
    max_attempts: usize,
) -> SubmittedRequest {
    let search_root = artifacts.join(format!("{prefix}-search"));
    fs::create_dir_all(&search_root).expect("failed to create placement search artifact root");

    for idx in 0..max_attempts {
        for spawned in watched_nodes.iter_mut() {
            assert_cluster_proof_running(
                spawned,
                &format!("searching stable placement for {prefix} candidate {idx}"),
            );
        }

        let request_key = format!("{prefix}-key-{idx}");
        let payload = format!("payload-{idx}");
        let body = format!(r#"{{"request_key":"{request_key}","payload":"{payload}"}}"#);
        let response = post_json(port, "/work", &body);
        let response_name = format!("candidate-{idx}");
        let json = parse_json_response(&search_root, &response_name, &response, "placement search");
        let owner = required_str(&json, "owner_node");
        let replica = required_str(&json, "replica_node");
        if owner == desired_owner && replica == desired_replica {
            return SubmittedRequest {
                request_key,
                payload,
                response: json,
                status_code: response.status_code,
            };
        }
    }

    panic!(
        "failed to find submit request routed to owner={desired_owner} replica={desired_replica}; search artifacts: {}",
        search_root.display()
    );
}

fn parse_attempt_token(attempt_id: &str) -> u64 {
    attempt_id
        .strip_prefix("attempt-")
        .unwrap_or_else(|| panic!("attempt id must start with `attempt-`, got {attempt_id:?}"))
        .parse::<u64>()
        .unwrap_or_else(|error| {
            panic!("attempt id must end with an integer token, got {attempt_id:?}: {error}")
        })
}

fn assert_log_contains(logs: &StoppedClusterProof, needle: &str) {
    assert!(
        logs.combined.contains(needle),
        "expected log `{}` in run{} {} / {}\nstdout:\n{}\nstderr:\n{}",
        needle,
        logs.incarnation,
        logs.stdout_path.display(),
        logs.stderr_path.display(),
        logs.stdout,
        logs.stderr
    );
}

fn assert_log_absent(logs: &StoppedClusterProof, needle: &str) {
    assert!(
        !logs.combined.contains(needle),
        "did not expect log `{}` in run{} {} / {}\nstdout:\n{}\nstderr:\n{}",
        needle,
        logs.incarnation,
        logs.stdout_path.display(),
        logs.stderr_path.display(),
        logs.stdout,
        logs.stderr
    );
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

#[test]
fn continuity_api_owner_loss_retry_rollover_survivor_status_is_truthful() {
    build_cluster_proof();

    let artifacts = artifact_dir("continuity-api-owner-loss-recovery");
    let cluster_port = dual_stack_cluster_port();
    let owner_config = ClusterProofConfig {
        node_basename: "node-a".to_string(),
        advertise_host: LOOPBACK_V4.to_string(),
        cluster_port,
        http_port: unused_http_port(),
        work_delay_ms: 5_000,
        cluster_role: "primary".to_string(),
        promotion_epoch: 0,
    };
    let survivor_config = ClusterProofConfig {
        node_basename: "node-b".to_string(),
        advertise_host: LOOPBACK_V6.to_string(),
        cluster_port,
        http_port: unused_http_port(),
        work_delay_ms: 1_200,
        cluster_role: "primary".to_string(),
        promotion_epoch: 0,
    };
    let owner_node = expected_node_name(&owner_config);
    let survivor_node = expected_node_name(&survivor_config);
    let membership = vec![owner_node.clone(), survivor_node.clone()];

    let mut selected_request_key: Option<String> = None;
    let mut owner_attempt_id: Option<String> = None;
    let mut retry_attempt_id: Option<String> = None;
    let mut spawned_owner = Some(spawn_cluster_proof(owner_config.clone(), &artifacts, 1));
    let mut spawned_survivor = Some(spawn_cluster_proof(survivor_config.clone(), &artifacts, 1));
    let mut killed_owner_logs: Option<StoppedClusterProof> = None;

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        {
            let owner = spawned_owner
                .as_mut()
                .expect("owner process missing before convergence");
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing before convergence");
            let mut watched = [owner, survivor];
            wait_for_membership(
                &artifacts,
                "membership-node-a",
                owner_config.http_port,
                &mut watched,
                &owner_node,
                &membership,
            );
        }
        {
            let owner = spawned_owner
                .as_mut()
                .expect("owner process missing before convergence");
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing before convergence");
            let mut watched = [owner, survivor];
            wait_for_membership(
                &artifacts,
                "membership-node-b",
                survivor_config.http_port,
                &mut watched,
                &survivor_node,
                &membership,
            );
        }

        let submitted = {
            let owner = spawned_owner
                .as_mut()
                .expect("owner process missing before placement search");
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing before placement search");
            let mut watched = [owner, survivor];
            find_submit_matching_placement(
                &artifacts,
                "owner-loss-recovery",
                owner_config.http_port,
                &mut watched,
                &owner_node,
                &survivor_node,
                64,
            )
        };
        selected_request_key = Some(submitted.request_key.clone());
        assert_eq!(submitted.status_code, 200, "initial submit must succeed");
        let created = submitted.response;
        let created_attempt = required_str(&created, "attempt_id");
        owner_attempt_id = Some(created_attempt.clone());
        assert_eq!(required_str(&created, "phase"), "submitted");
        assert_eq!(required_str(&created, "result"), "pending");
        assert_eq!(required_str(&created, "owner_node"), owner_node);
        assert_eq!(required_str(&created, "replica_node"), survivor_node);
        assert_eq!(required_str(&created, "replica_status"), "mirrored");
        assert_eq!(required_str(&created, "execution_node"), "");
        assert!(!required_bool(&created, "routed_remotely"));
        assert!(required_bool(&created, "fell_back_locally"));
        assert!(required_bool(&created, "ok"));

        {
            let owner = spawned_owner
                .as_mut()
                .expect("owner process missing before pending-owner status");
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing before pending-owner status");
            let mut watched = [owner, survivor];
            let pending_owner = wait_for_status_condition(
                &artifacts,
                "pre-loss-owner-status",
                owner_config.http_port,
                &mut watched,
                selected_request_key.as_deref().unwrap(),
                "pending mirrored status on owner before owner loss",
                |json| {
                    required_str(json, "attempt_id") == created_attempt
                        && required_str(json, "phase") == "submitted"
                        && required_str(json, "result") == "pending"
                        && required_str(json, "replica_status") == "mirrored"
                },
            );
            assert_eq!(required_str(&pending_owner, "owner_node"), owner_node);
            assert_eq!(required_str(&pending_owner, "replica_node"), survivor_node);
        }

        killed_owner_logs = spawned_owner.take().map(kill_cluster_proof);

        {
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing after owner loss");
            let mut watched = [survivor];
            wait_for_membership(
                &artifacts,
                "degraded-membership-node-b",
                survivor_config.http_port,
                &mut watched,
                &survivor_node,
                std::slice::from_ref(&survivor_node),
            );
        }

        let owner_lost_status = {
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing during owner-loss status check");
            let mut watched = [survivor];
            wait_for_status_condition(
                &artifacts,
                "owner-lost-status",
                survivor_config.http_port,
                &mut watched,
                selected_request_key.as_deref().unwrap(),
                "owner-lost continuity status on surviving replica",
                |json| {
                    required_str(json, "attempt_id") == created_attempt
                        && required_str(json, "phase") == "submitted"
                        && required_str(json, "result") == "pending"
                        && required_str(json, "replica_status") == "owner_lost"
                },
            )
        };
        assert_eq!(required_str(&owner_lost_status, "owner_node"), owner_node);
        assert_eq!(
            required_str(&owner_lost_status, "replica_node"),
            survivor_node
        );
        assert_eq!(required_str(&owner_lost_status, "execution_node"), "");
        assert_eq!(required_str(&owner_lost_status, "error"), "");
        assert!(required_bool(&owner_lost_status, "ok"));

        let retry_payload = format!(
            r#"{{"request_key":"{}","payload":"{}"}}"#,
            selected_request_key.as_deref().unwrap(),
            submitted.payload
        );
        let retry = json_body(
            &artifacts,
            "retry-rollover",
            &post_json(survivor_config.http_port, "/work", &retry_payload),
            200,
            "same-key recovery retry after owner loss",
        );
        let recovered_attempt = required_str(&retry, "attempt_id");
        retry_attempt_id = Some(recovered_attempt.clone());
        assert_ne!(
            recovered_attempt, created_attempt,
            "retry must roll attempt id"
        );
        assert!(
            parse_attempt_token(&recovered_attempt) > parse_attempt_token(&created_attempt),
            "retry attempt token must move forward"
        );
        assert_eq!(required_str(&retry, "phase"), "submitted");
        assert_eq!(required_str(&retry, "result"), "pending");
        assert_eq!(required_str(&retry, "ingress_node"), survivor_node);
        assert_eq!(required_str(&retry, "owner_node"), survivor_node);
        assert_eq!(required_str(&retry, "replica_node"), "");
        assert_eq!(required_str(&retry, "replica_status"), "unassigned");
        assert_eq!(required_str(&retry, "execution_node"), "");
        assert!(!required_bool(&retry, "routed_remotely"));
        assert!(required_bool(&retry, "fell_back_locally"));
        assert!(required_bool(&retry, "ok"));

        {
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing during retry pending status");
            let mut watched = [survivor];
            let retry_pending = wait_for_status_condition(
                &artifacts,
                "retry-pending-status",
                survivor_config.http_port,
                &mut watched,
                selected_request_key.as_deref().unwrap(),
                "pending retry status on survivor",
                |json| {
                    required_str(json, "attempt_id") == recovered_attempt
                        && required_str(json, "phase") == "submitted"
                        && required_str(json, "result") == "pending"
                        && required_str(json, "owner_node") == survivor_node
                        && required_str(json, "replica_status") == "unassigned"
                },
            );
            assert_eq!(required_str(&retry_pending, "replica_node"), "");
            assert_eq!(required_str(&retry_pending, "execution_node"), "");
        }

        let retry_completed = {
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing during retry completion");
            let mut watched = [survivor];
            wait_for_status_condition(
                &artifacts,
                "retry-completed-status",
                survivor_config.http_port,
                &mut watched,
                selected_request_key.as_deref().unwrap(),
                "completed retry status on survivor",
                |json| {
                    required_str(json, "attempt_id") == recovered_attempt
                        && required_str(json, "phase") == "completed"
                        && required_str(json, "result") == "succeeded"
                        && required_str(json, "owner_node") == survivor_node
                        && required_str(json, "execution_node") == survivor_node
                },
            )
        };
        assert_eq!(required_str(&retry_completed, "replica_node"), "");
        assert_eq!(
            required_str(&retry_completed, "replica_status"),
            "unassigned"
        );
        assert!(required_bool(&retry_completed, "ok"));
    }));

    let survivor_logs = stop_cluster_proof(
        spawned_survivor
            .take()
            .expect("survivor process missing during cleanup"),
    );
    let owner_logs = match killed_owner_logs {
        Some(logs) => logs,
        None => stop_cluster_proof(
            spawned_owner
                .take()
                .expect("owner process missing during cleanup"),
        ),
    };

    if let Err(payload) = result {
        panic!(
            "owner-loss recovery assertions failed: {}\nartifacts: {}\nowner stdout:\n{}\nowner stderr:\n{}\nsurvivor stdout:\n{}\nsurvivor stderr:\n{}",
            panic_payload_to_string(payload),
            artifacts.display(),
            owner_logs.stdout,
            owner_logs.stderr,
            survivor_logs.stdout,
            survivor_logs.stderr
        );
    }

    let request_key = selected_request_key
        .as_deref()
        .expect("selected request key missing after successful run");
    let old_attempt = owner_attempt_id
        .as_deref()
        .expect("owner attempt id missing after successful run");
    let new_attempt = retry_attempt_id
        .as_deref()
        .expect("retry attempt id missing after successful run");

    assert_log_contains(&owner_logs, "continuity=runtime-native");
    assert_log_contains(&survivor_logs, "continuity=runtime-native");
    assert_log_contains(
        &owner_logs,
        &format!(
            "[cluster-proof] work executed request_key={request_key} attempt_id={old_attempt} execution={owner_node}"
        ),
    );
    assert_log_absent(
        &owner_logs,
        &format!(
            "[mesh-rt continuity] transition=completed request_key={request_key} attempt_id={old_attempt}"
        ),
    );
    assert_log_contains(
        &survivor_logs,
        &format!(
            "[mesh-rt continuity] transition=owner_lost request_key={request_key} attempt_id={old_attempt}"
        ),
    );
    assert_log_contains(
        &survivor_logs,
        &format!(
            "[mesh-rt continuity] transition=recovery_rollover request_key={request_key} previous_attempt_id={old_attempt} next_attempt_id={new_attempt}"
        ),
    );
    assert_log_contains(
        &survivor_logs,
        &format!(
            "[cluster-proof] work executed request_key={request_key} attempt_id={new_attempt} execution={survivor_node}"
        ),
    );
    assert_log_contains(
        &survivor_logs,
        &format!(
            "[mesh-rt continuity] transition=completed request_key={request_key} attempt_id={new_attempt} execution={survivor_node}"
        ),
    );
}

#[test]
fn continuity_api_same_identity_rejoin_preserves_newer_attempt_truth() {
    build_cluster_proof();

    let artifacts = artifact_dir("continuity-api-owner-loss-rejoin");
    let cluster_port = dual_stack_cluster_port();
    let owner_config = ClusterProofConfig {
        node_basename: "node-a".to_string(),
        advertise_host: LOOPBACK_V4.to_string(),
        cluster_port,
        http_port: unused_http_port(),
        work_delay_ms: 5_000,
        cluster_role: "primary".to_string(),
        promotion_epoch: 0,
    };
    let survivor_config = ClusterProofConfig {
        node_basename: "node-b".to_string(),
        advertise_host: LOOPBACK_V6.to_string(),
        cluster_port,
        http_port: unused_http_port(),
        work_delay_ms: 1_200,
        cluster_role: "primary".to_string(),
        promotion_epoch: 0,
    };
    let owner_node = expected_node_name(&owner_config);
    let survivor_node = expected_node_name(&survivor_config);
    let membership = vec![owner_node.clone(), survivor_node.clone()];

    let mut selected_request_key: Option<String> = None;
    let mut owner_attempt_id: Option<String> = None;
    let mut retry_attempt_id: Option<String> = None;
    let mut spawned_owner_run1 = Some(spawn_cluster_proof(owner_config.clone(), &artifacts, 1));
    let mut spawned_survivor = Some(spawn_cluster_proof(survivor_config.clone(), &artifacts, 1));
    let mut killed_owner_run1_logs: Option<StoppedClusterProof> = None;
    let mut spawned_owner_run2: Option<SpawnedClusterProof> = None;

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        {
            let owner = spawned_owner_run1
                .as_mut()
                .expect("owner run1 missing before convergence");
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing before convergence");
            let mut watched = [owner, survivor];
            wait_for_membership(
                &artifacts,
                "membership-node-a-run1",
                owner_config.http_port,
                &mut watched,
                &owner_node,
                &membership,
            );
        }
        {
            let owner = spawned_owner_run1
                .as_mut()
                .expect("owner run1 missing before convergence");
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing before convergence");
            let mut watched = [owner, survivor];
            wait_for_membership(
                &artifacts,
                "membership-node-b-run1",
                survivor_config.http_port,
                &mut watched,
                &survivor_node,
                &membership,
            );
        }

        let submitted = {
            let owner = spawned_owner_run1
                .as_mut()
                .expect("owner run1 missing before placement search");
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing before placement search");
            let mut watched = [owner, survivor];
            find_submit_matching_placement(
                &artifacts,
                "owner-loss-rejoin",
                owner_config.http_port,
                &mut watched,
                &owner_node,
                &survivor_node,
                64,
            )
        };
        selected_request_key = Some(submitted.request_key.clone());
        assert_eq!(submitted.status_code, 200, "initial submit must succeed");
        let created = submitted.response;
        let created_attempt = required_str(&created, "attempt_id");
        owner_attempt_id = Some(created_attempt.clone());
        assert_eq!(required_str(&created, "phase"), "submitted");
        assert_eq!(required_str(&created, "result"), "pending");
        assert_eq!(required_str(&created, "owner_node"), owner_node);
        assert_eq!(required_str(&created, "replica_node"), survivor_node);
        assert_eq!(required_str(&created, "replica_status"), "mirrored");
        assert!(required_bool(&created, "ok"));

        {
            let owner = spawned_owner_run1
                .as_mut()
                .expect("owner run1 missing before pre-loss owner status");
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing before pre-loss owner status");
            let mut watched = [owner, survivor];
            wait_for_status_condition(
                &artifacts,
                "pre-loss-owner-status",
                owner_config.http_port,
                &mut watched,
                selected_request_key.as_deref().unwrap(),
                "pending mirrored status on owner before rejoin proof",
                |json| {
                    required_str(json, "attempt_id") == created_attempt
                        && required_str(json, "phase") == "submitted"
                        && required_str(json, "result") == "pending"
                        && required_str(json, "replica_status") == "mirrored"
                },
            );
        }

        killed_owner_run1_logs = spawned_owner_run1.take().map(kill_cluster_proof);

        {
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing after owner loss");
            let mut watched = [survivor];
            wait_for_membership(
                &artifacts,
                "degraded-membership-node-b",
                survivor_config.http_port,
                &mut watched,
                &survivor_node,
                std::slice::from_ref(&survivor_node),
            );
        }
        {
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing during owner-lost status");
            let mut watched = [survivor];
            wait_for_status_condition(
                &artifacts,
                "owner-lost-status",
                survivor_config.http_port,
                &mut watched,
                selected_request_key.as_deref().unwrap(),
                "owner-lost continuity status before rejoin",
                |json| {
                    required_str(json, "attempt_id") == created_attempt
                        && required_str(json, "phase") == "submitted"
                        && required_str(json, "result") == "pending"
                        && required_str(json, "replica_status") == "owner_lost"
                },
            );
        }

        let retry_payload = format!(
            r#"{{"request_key":"{}","payload":"{}"}}"#,
            selected_request_key.as_deref().unwrap(),
            submitted.payload
        );
        let retry = json_body(
            &artifacts,
            "retry-rollover",
            &post_json(survivor_config.http_port, "/work", &retry_payload),
            200,
            "same-key recovery retry before rejoin",
        );
        let recovered_attempt = required_str(&retry, "attempt_id");
        retry_attempt_id = Some(recovered_attempt.clone());
        assert_ne!(
            recovered_attempt, created_attempt,
            "retry must roll attempt id"
        );
        assert!(
            parse_attempt_token(&recovered_attempt) > parse_attempt_token(&created_attempt),
            "retry attempt token must move forward"
        );
        assert_eq!(required_str(&retry, "phase"), "submitted");
        assert_eq!(required_str(&retry, "result"), "pending");
        assert_eq!(required_str(&retry, "owner_node"), survivor_node);
        assert_eq!(required_str(&retry, "replica_node"), "");
        assert_eq!(required_str(&retry, "replica_status"), "unassigned");
        assert!(required_bool(&retry, "ok"));

        {
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing during retry pending status");
            let mut watched = [survivor];
            wait_for_status_condition(
                &artifacts,
                "retry-pending-status",
                survivor_config.http_port,
                &mut watched,
                selected_request_key.as_deref().unwrap(),
                "pending retry status before rejoin",
                |json| {
                    required_str(json, "attempt_id") == recovered_attempt
                        && required_str(json, "phase") == "submitted"
                        && required_str(json, "result") == "pending"
                        && required_str(json, "owner_node") == survivor_node
                        && required_str(json, "replica_status") == "unassigned"
                },
            );
        }

        {
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing during retry completion");
            let mut watched = [survivor];
            let completed = wait_for_status_condition(
                &artifacts,
                "retry-completed-status",
                survivor_config.http_port,
                &mut watched,
                selected_request_key.as_deref().unwrap(),
                "completed retry status before rejoin",
                |json| {
                    required_str(json, "attempt_id") == recovered_attempt
                        && required_str(json, "phase") == "completed"
                        && required_str(json, "result") == "succeeded"
                        && required_str(json, "owner_node") == survivor_node
                        && required_str(json, "execution_node") == survivor_node
                },
            );
            assert_eq!(required_str(&completed, "replica_node"), "");
            assert_eq!(required_str(&completed, "replica_status"), "unassigned");
            assert!(required_bool(&completed, "ok"));
        }

        spawned_owner_run2 = Some(spawn_cluster_proof(owner_config.clone(), &artifacts, 2));

        {
            let owner = spawned_owner_run2
                .as_mut()
                .expect("owner run2 missing before membership convergence");
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing before membership convergence");
            let mut watched = [owner, survivor];
            wait_for_membership(
                &artifacts,
                "membership-node-a-run2",
                owner_config.http_port,
                &mut watched,
                &owner_node,
                &membership,
            );
        }
        {
            let owner = spawned_owner_run2
                .as_mut()
                .expect("owner run2 missing before survivor membership convergence");
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing before survivor membership convergence");
            let mut watched = [owner, survivor];
            wait_for_membership(
                &artifacts,
                "membership-node-b-run2",
                survivor_config.http_port,
                &mut watched,
                &survivor_node,
                &membership,
            );
        }

        {
            let owner = spawned_owner_run2
                .as_mut()
                .expect("owner run2 missing during rejoined-node status");
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing during rejoined-node status");
            let mut watched = [owner, survivor];
            let status = wait_for_status_condition(
                &artifacts,
                "post-rejoin-node-a-status",
                owner_config.http_port,
                &mut watched,
                selected_request_key.as_deref().unwrap(),
                "completed retry status on rejoined owner identity",
                |json| {
                    required_str(json, "attempt_id") == recovered_attempt
                        && required_str(json, "phase") == "completed"
                        && required_str(json, "result") == "succeeded"
                        && required_str(json, "owner_node") == survivor_node
                        && required_str(json, "execution_node") == survivor_node
                },
            );
            assert_eq!(required_str(&status, "replica_node"), "");
            assert_eq!(required_str(&status, "replica_status"), "unassigned");
            assert!(required_bool(&status, "ok"));
        }
        {
            let owner = spawned_owner_run2
                .as_mut()
                .expect("owner run2 missing during survivor post-rejoin status");
            let survivor = spawned_survivor
                .as_mut()
                .expect("survivor process missing during survivor post-rejoin status");
            let mut watched = [owner, survivor];
            let status = wait_for_status_condition(
                &artifacts,
                "post-rejoin-node-b-status",
                survivor_config.http_port,
                &mut watched,
                selected_request_key.as_deref().unwrap(),
                "completed retry status on survivor after rejoin",
                |json| {
                    required_str(json, "attempt_id") == recovered_attempt
                        && required_str(json, "phase") == "completed"
                        && required_str(json, "result") == "succeeded"
                        && required_str(json, "owner_node") == survivor_node
                        && required_str(json, "execution_node") == survivor_node
                },
            );
            assert_eq!(required_str(&status, "replica_node"), "");
            assert_eq!(required_str(&status, "replica_status"), "unassigned");
            assert!(required_bool(&status, "ok"));
        }

        let stale_guard = json_body(
            &artifacts,
            "stale-completion-guard",
            &post_json(owner_config.http_port, "/work", &retry_payload),
            200,
            "duplicate submit on rejoined node must preserve newer attempt",
        );
        assert_eq!(required_str(&stale_guard, "attempt_id"), recovered_attempt);
        assert_eq!(required_str(&stale_guard, "phase"), "completed");
        assert_eq!(required_str(&stale_guard, "result"), "succeeded");
        assert_eq!(required_str(&stale_guard, "owner_node"), survivor_node);
        assert_eq!(required_str(&stale_guard, "replica_node"), "");
        assert_eq!(required_str(&stale_guard, "replica_status"), "unassigned");
        assert_eq!(required_str(&stale_guard, "execution_node"), survivor_node);
        assert_eq!(required_str(&stale_guard, "conflict_reason"), "");
        assert!(required_bool(&stale_guard, "ok"));
    }));

    let survivor_logs = stop_cluster_proof(
        spawned_survivor
            .take()
            .expect("survivor process missing during cleanup"),
    );
    let owner_run2_logs = spawned_owner_run2.take().map(stop_cluster_proof);
    let owner_run1_logs = match killed_owner_run1_logs {
        Some(logs) => logs,
        None => stop_cluster_proof(
            spawned_owner_run1
                .take()
                .expect("owner run1 missing during cleanup"),
        ),
    };

    if let Err(payload) = result {
        let run2_stdout = owner_run2_logs
            .as_ref()
            .map(|logs| logs.stdout.as_str())
            .unwrap_or("");
        let run2_stderr = owner_run2_logs
            .as_ref()
            .map(|logs| logs.stderr.as_str())
            .unwrap_or("");
        panic!(
            "rejoin truth assertions failed: {}\nartifacts: {}\nowner run1 stdout:\n{}\nowner run1 stderr:\n{}\nowner run2 stdout:\n{}\nowner run2 stderr:\n{}\nsurvivor stdout:\n{}\nsurvivor stderr:\n{}",
            panic_payload_to_string(payload),
            artifacts.display(),
            owner_run1_logs.stdout,
            owner_run1_logs.stderr,
            run2_stdout,
            run2_stderr,
            survivor_logs.stdout,
            survivor_logs.stderr
        );
    }

    let request_key = selected_request_key
        .as_deref()
        .expect("selected request key missing after successful run");
    let old_attempt = owner_attempt_id
        .as_deref()
        .expect("owner attempt id missing after successful run");
    let new_attempt = retry_attempt_id
        .as_deref()
        .expect("retry attempt id missing after successful run");
    let owner_run2_logs = owner_run2_logs.expect("owner run2 logs missing after successful run");

    assert_log_contains(&owner_run1_logs, "continuity=runtime-native");
    assert_log_contains(&owner_run2_logs, "continuity=runtime-native");
    assert_log_contains(&survivor_logs, "continuity=runtime-native");
    assert_log_absent(
        &owner_run1_logs,
        &format!(
            "[mesh-rt continuity] transition=completed request_key={request_key} attempt_id={old_attempt}"
        ),
    );
    assert_log_contains(
        &survivor_logs,
        &format!(
            "[mesh-rt continuity] transition=owner_lost request_key={request_key} attempt_id={old_attempt}"
        ),
    );
    assert_log_contains(
        &survivor_logs,
        &format!(
            "[mesh-rt continuity] transition=recovery_rollover request_key={request_key} previous_attempt_id={old_attempt} next_attempt_id={new_attempt}"
        ),
    );
    assert_log_contains(
        &survivor_logs,
        &format!(
            "[mesh-rt continuity] transition=completed request_key={request_key} attempt_id={new_attempt} execution={survivor_node}"
        ),
    );
    assert_log_contains(
        &owner_run2_logs,
        &format!(
            "[cluster-proof] keyed status request_key={request_key} attempt_id={new_attempt} phase=completed result=succeeded owner={survivor_node} replica= source={owner_node} replica_status=unassigned"
        ),
    );
    assert_log_contains(
        &owner_run2_logs,
        &format!(
            "[cluster-proof] keyed duplicate request_key={request_key} attempt_id={new_attempt} phase=completed result=succeeded owner={survivor_node} replica="
        ),
    );
}
