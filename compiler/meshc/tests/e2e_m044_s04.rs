use std::any::Any;
use std::fs::{self, File};
use std::io::{Read as _, Write as _};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::{Once, OnceLock};
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const LOOPBACK_V4: &str = "127.0.0.1";
const LOOPBACK_V6: &str = "::1";
const DISCOVERY_SEED: &str = "localhost";
const SHARED_COOKIE: &str = "mesh-m043-s02-cookie";
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
    child: Child,
    stdout_path: PathBuf,
    stderr_path: PathBuf,
}

struct StoppedClusterProof {
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

struct ExpectedMembershipTruth<'a> {
    self_name: &'a str,
    membership: &'a [String],
    cluster_role: &'a str,
    promotion_epoch: u64,
    replication_health: &'a str,
}

struct ExpectedWorkTruth<'a> {
    request_key: &'a str,
    attempt_id: Option<&'a str>,
    phase: &'a str,
    result: &'a str,
    ingress_node: &'a str,
    owner_node: &'a str,
    replica_node: &'a str,
    replica_status: &'a str,
    cluster_role: &'a str,
    promotion_epoch: u64,
    replication_health: &'a str,
    execution_node: &'a str,
    routed_remotely: bool,
    fell_back_locally: bool,
    ok: bool,
    error: &'a str,
    conflict_reason: &'a str,
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
        .join("m044-s04")
        .join(format!("{test_name}-{stamp}"));
    fs::create_dir_all(&dir).expect("failed to create e2e artifact dir");
    dir
}

fn write_artifact(path: &Path, contents: impl AsRef<str>) {
    fs::write(path, contents.as_ref())
        .unwrap_or_else(|error| panic!("failed to write artifact {}: {error}", path.display()));
}

fn node_log_paths(log_dir: &Path, node_basename: &str) -> (PathBuf, PathBuf) {
    let stdout_path = log_dir.join(format!("{node_basename}.stdout.log"));
    let stderr_path = log_dir.join(format!("{node_basename}.stderr.log"));
    (stdout_path, stderr_path)
}

fn spawn_cluster_proof(config: ClusterProofConfig, artifacts: &Path) -> SpawnedClusterProof {
    let binary = cluster_proof_binary();
    assert!(
        binary.exists(),
        "cluster-proof binary not found at {}. Run `meshc build cluster-proof` first.",
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

    SpawnedClusterProof {
        config,
        child,
        stdout_path,
        stderr_path,
    }
}

fn stop_cluster_proof(spawned: SpawnedClusterProof) -> StoppedClusterProof {
    let SpawnedClusterProof {
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
    child
        .wait()
        .expect("failed to collect cluster-proof exit status");

    let stdout = fs::read_to_string(&stdout_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", stdout_path.display(), e));
    let stderr = fs::read_to_string(&stderr_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", stderr_path.display(), e));
    let combined = format!("{stdout}{stderr}");

    StoppedClusterProof {
        stdout,
        stderr,
        combined,
        stdout_path,
        stderr_path,
    }
}

fn kill_cluster_proof(spawned: SpawnedClusterProof) -> StoppedClusterProof {
    stop_cluster_proof(spawned)
}

fn assert_cluster_proof_running(spawned: &mut SpawnedClusterProof, context: &str) {
    if let Some(status) = spawned.child.try_wait().unwrap_or_else(|e| {
        panic!(
            "failed to probe {} exit status: {}",
            spawned.config.node_basename, e
        )
    }) {
        panic!(
            "cluster-proof node {} exited early while {}: status={:?}; stdout_log={}; stderr_log={}",
            spawned.config.node_basename,
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

fn get_json(port: u16, path: &str) -> HttpResponse {
    try_get_json(port, path).expect("GET request failed")
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

fn membership_truth_matches(json: &Value, expected: &ExpectedMembershipTruth<'_>) -> bool {
    required_str(json, "mode") == "cluster"
        && required_str(json, "self") == expected.self_name
        && sorted(&required_string_list(json, "membership")) == sorted(expected.membership)
        && required_str(json, "cluster_role") == expected.cluster_role
        && required_u64(json, "promotion_epoch") == expected.promotion_epoch
        && required_str(json, "replication_health") == expected.replication_health
        && required_str(json, "discovery_provider") == "dns"
        && required_str(json, "discovery_seed") == DISCOVERY_SEED
}

fn assert_membership_truth(json: &Value, expected: &ExpectedMembershipTruth<'_>) {
    assert!(
        membership_truth_matches(json, expected),
        "membership truth mismatch. expected self={} membership={:?} cluster_role={} promotion_epoch={} replication_health={}, got {}",
        expected.self_name,
        expected.membership,
        expected.cluster_role,
        expected.promotion_epoch,
        expected.replication_health,
        json
    );
}

fn attempt_id_matches(actual: &str, expected: Option<&str>) -> bool {
    let format_ok = actual
        .strip_prefix("attempt-")
        .and_then(|value| value.parse::<u64>().ok())
        .is_some();
    format_ok && expected.map(|expected| expected == actual).unwrap_or(true)
}

fn work_truth_matches(json: &Value, expected: &ExpectedWorkTruth<'_>) -> bool {
    attempt_id_matches(&required_str(json, "attempt_id"), expected.attempt_id)
        && required_str(json, "request_key") == expected.request_key
        && required_str(json, "phase") == expected.phase
        && required_str(json, "result") == expected.result
        && required_str(json, "ingress_node") == expected.ingress_node
        && required_str(json, "owner_node") == expected.owner_node
        && required_str(json, "replica_node") == expected.replica_node
        && required_str(json, "replica_status") == expected.replica_status
        && required_str(json, "cluster_role") == expected.cluster_role
        && required_u64(json, "promotion_epoch") == expected.promotion_epoch
        && required_str(json, "replication_health") == expected.replication_health
        && required_str(json, "execution_node") == expected.execution_node
        && required_bool(json, "routed_remotely") == expected.routed_remotely
        && required_bool(json, "fell_back_locally") == expected.fell_back_locally
        && required_bool(json, "ok") == expected.ok
        && required_str(json, "error") == expected.error
        && required_str(json, "conflict_reason") == expected.conflict_reason
}

fn assert_work_truth(json: &Value, expected: &ExpectedWorkTruth<'_>) {
    assert!(
        work_truth_matches(json, expected),
        "work truth mismatch. expected request_key={} phase={} result={} ingress={} owner={} replica={} replica_status={} cluster_role={} promotion_epoch={} replication_health={} execution={} routed={} fell_back={} ok={} error={} conflict={}, got {}",
        expected.request_key,
        expected.phase,
        expected.result,
        expected.ingress_node,
        expected.owner_node,
        expected.replica_node,
        expected.replica_status,
        expected.cluster_role,
        expected.promotion_epoch,
        expected.replication_health,
        expected.execution_node,
        expected.routed_remotely,
        expected.fell_back_locally,
        expected.ok,
        expected.error,
        expected.conflict_reason,
        json
    );
}

fn wait_for_membership(
    artifacts: &Path,
    name: &str,
    port: u16,
    watched_nodes: &mut [&mut SpawnedClusterProof],
    expected: &ExpectedMembershipTruth<'_>,
) -> Value {
    const REQUIRED_STABLE_POLLS: usize = 5;

    let start = Instant::now();
    let mut last_body = String::new();
    let mut stable_polls = 0usize;

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
                    if membership_truth_matches(&json, expected) {
                        stable_polls += 1;
                        if stable_polls >= REQUIRED_STABLE_POLLS {
                            return json;
                        }
                    } else {
                        stable_polls = 0;
                    }
                } else {
                    archive_raw_response(artifacts, name, &response);
                    last_body = response.raw.clone();
                    stable_polls = 0;
                }
            }
            Err(error) => {
                last_body = error.to_string();
                write_artifact(&artifacts.join(format!("{name}.error.txt")), &last_body);
                stable_polls = 0;
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

fn wait_for_work_truth(
    artifacts: &Path,
    name: &str,
    port: u16,
    watched_nodes: &mut [&mut SpawnedClusterProof],
    expected: &ExpectedWorkTruth<'_>,
    predicate_description: &str,
) -> Value {
    let start = Instant::now();
    let path = format!("/work/{}", expected.request_key);
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
                    if work_truth_matches(&json, expected) {
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
        "request {} never satisfied {}; last body archived at {}",
        expected.request_key,
        predicate_description,
        timeout_path.display()
    );
}

fn wait_for_auto_recovery_transition(
    artifacts: &Path,
    name: &str,
    port: u16,
    watched_nodes: &mut [&mut SpawnedClusterProof],
    request_key: &str,
    previous_attempt_id: &str,
    expected_owner_node: &str,
    expected_phase: &str,
    expected_result: &str,
    predicate_description: &str,
) -> Value {
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
                    let attempt_id = required_str(&json, "attempt_id");
                    if attempt_id != previous_attempt_id
                        && required_str(&json, "phase") == expected_phase
                        && required_str(&json, "result") == expected_result
                        && required_str(&json, "owner_node") == expected_owner_node
                        && required_str(&json, "cluster_role") == "primary"
                        && required_u64(&json, "promotion_epoch") == 1
                    {
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
        "request {} never satisfied {}; last body archived at {}",
        request_key,
        predicate_description,
        timeout_path.display()
    );
}

fn stable_hash_u64(value: &str) -> u64 {
    let digest = Sha256::digest(value.as_bytes());
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&digest[..8]);
    u64::from_be_bytes(bytes)
}

fn request_key_matches_placement(
    request_key: &str,
    desired_owner: &str,
    desired_replica: &str,
) -> bool {
    let mut membership = vec![desired_owner.to_string(), desired_replica.to_string()];
    membership.sort_by_key(|value| (stable_hash_u64(value), value.clone()));
    let owner_index =
        (stable_hash_u64(&format!("request::{request_key}")) as usize) % membership.len();
    membership[owner_index] == desired_owner
        && membership[(owner_index + 1) % membership.len()] == desired_replica
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
        if !request_key_matches_placement(&request_key, desired_owner, desired_replica) {
            continue;
        }

        write_artifact(
            &search_root.join("chosen.json"),
            serde_json::to_string_pretty(&json!({
                "request_key": request_key,
                "payload": payload,
                "owner_node": desired_owner,
                "replica_node": desired_replica,
            }))
            .expect("failed to serialize chosen placement"),
        );

        let body = format!(r#"{{"request_key":"{request_key}","payload":"{payload}"}}"#);
        let response = post_json(port, "/work", &body);
        let json = parse_json_response(&search_root, "selected", &response, "placement search");
        return SubmittedRequest {
            request_key,
            payload,
            response: json,
            status_code: response.status_code,
        };
    }

    panic!(
        "failed to find deterministic request key for owner={desired_owner} replica={desired_replica}; search artifacts: {}",
        search_root.display()
    );
}

fn assert_log_contains(logs: &StoppedClusterProof, needle: &str) {
    assert!(
        logs.combined.contains(needle),
        "expected log `{}` in {} / {}\nstdout:\n{}\nstderr:\n{}",
        needle,
        logs.stdout_path.display(),
        logs.stderr_path.display(),
        logs.stdout,
        logs.stderr
    );
}

fn assert_log_absent(logs: &StoppedClusterProof, needle: &str) {
    assert!(
        !logs.combined.contains(needle),
        "did not expect log `{}` in {} / {}\nstdout:\n{}\nstderr:\n{}",
        needle,
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

fn command_output_text(output: &Output) -> String {
    format!(
        "status: {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn ensure_mesh_rt_staticlib() {
    static BUILD_ONCE: OnceLock<()> = OnceLock::new();
    BUILD_ONCE.get_or_init(|| {
        let output = Command::new("cargo")
            .current_dir(repo_root())
            .args(["build", "-p", "mesh-rt"])
            .output()
            .expect("failed to invoke cargo build -p mesh-rt");
        assert!(
            output.status.success(),
            "cargo build -p mesh-rt failed:
{}",
            command_output_text(&output)
        );
    });
}

fn build_only_mesh(source: &str, artifacts: &Path) -> Output {
    ensure_mesh_rt_staticlib();

    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
    let project_dir = temp_dir.path().join("project");
    fs::create_dir_all(&project_dir).expect("failed to create project dir");

    let main_mesh = project_dir.join("main.mpl");
    fs::write(&main_mesh, source).expect("failed to write main.mpl");
    write_artifact(&artifacts.join("main.mpl"), source);

    let output = Command::new(meshc_bin())
        .current_dir(repo_root())
        .args(["build", project_dir.to_str().unwrap()])
        .output()
        .expect("failed to invoke meshc build");

    write_artifact(&artifacts.join("build.log"), command_output_text(&output));

    output
}

fn build_and_run_mesh(source: &str, envs: &[(&str, &str)], artifacts: &Path) -> Output {
    let build_output = build_only_mesh(source, artifacts);
    assert!(
        build_output.status.success(),
        "meshc build failed:
{}
artifacts: {}",
        command_output_text(&build_output),
        artifacts.display()
    );

    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
    let project_dir = temp_dir.path().join("project");
    fs::create_dir_all(&project_dir).expect("failed to create project dir");
    let main_mesh = project_dir.join("main.mpl");
    fs::write(&main_mesh, source).expect("failed to write main.mpl");

    let rebuild_output = Command::new(meshc_bin())
        .current_dir(repo_root())
        .args(["build", project_dir.to_str().unwrap()])
        .output()
        .expect("failed to invoke meshc build for runnable project");
    assert!(
        rebuild_output.status.success(),
        "meshc rebuild failed:
{}
artifacts: {}",
        command_output_text(&rebuild_output),
        artifacts.display()
    );

    write_artifact(
        &artifacts.join("build-runnable.log"),
        command_output_text(&rebuild_output),
    );

    let binary = project_dir.join("project");
    let mut command = Command::new(&binary);
    command.current_dir(&project_dir);
    for (key, value) in envs {
        command.env(key, value);
    }
    let run_output = command
        .output()
        .unwrap_or_else(|error| panic!("failed to run {}: {error}", binary.display()));

    write_artifact(&artifacts.join("run.log"), command_output_text(&run_output));
    run_output
}

fn stdout_lines(output: &Output) -> Vec<String> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

const AUTHORITY_RUNTIME_SOURCE: &str = r##"
fn print_authority(label :: String, value :: ContinuityAuthorityStatus) do
  println("#{label}|ok|#{value.cluster_role}|#{value.promotion_epoch}|#{value.replication_health}")
end

fn print_authority_status(label :: String) do
  case Continuity.authority_status() do
    Ok( value) -> print_authority(label, value)
    Err( reason) -> println("#{label}|err|#{reason}")
  end
end

fn seed_attempt_id() -> String ! String do
  case Continuity.submit(
    "req-1",
    "hash-1",
    "standby@node",
    "primary@node",
    "replica@node",
    0,
    false,
    true
  ) do
    Ok( decision) -> Ok(decision.record.attempt_id)
    Err( reason) -> Err(reason)
  end
end

fn acknowledge_replica(attempt_id :: String) do
  case Continuity.acknowledge_replica("req-1", attempt_id) do
    Ok( record) -> println("ack|ok|#{record.replica_status}|#{record.replication_health}")
    Err( reason) -> println("ack|err|#{reason}")
  end
end

fn main() do
  print_authority_status("before")
  case seed_attempt_id() do
    Ok( attempt_id) -> do
      acknowledge_replica(attempt_id)
      print_authority_status("after_ack")
    end
    Err( reason) -> println("submit|err|#{reason}")
  end
end
"##;

const PRIMARY_AUTHORITY_RUNTIME_SOURCE: &str = r##"
fn main() do
  case Continuity.authority_status() do
    Ok( value) -> println("status|ok|#{value.cluster_role}|#{value.promotion_epoch}|#{value.replication_health}")
    Err( reason) -> println("status|err|#{reason}")
  end
end
"##;

const MANUAL_PROMOTION_DISABLED_SOURCE: &str = r##"
fn main() do
  case Continuity.promote() do
    Ok( value) -> println("#{value.cluster_role}")
    Err( reason) -> println(reason)
  end
end
"##;

#[test]
fn m044_s04_auto_promotion_declared_handler_contract_lives_in_work() {
    let manifest = fs::read_to_string(repo_root().join("cluster-proof/mesh.toml"))
        .expect("failed to read cluster-proof/mesh.toml");
    assert!(
        manifest.contains("Work.execute_declared_work"),
        "cluster-proof manifest should declare Work.execute_declared_work:\n{manifest}"
    );
    assert!(
        !manifest.contains("WorkContinuity.execute_declared_work"),
        "cluster-proof manifest should not keep the wrapper-era target:\n{manifest}"
    );

    let work_source = fs::read_to_string(repo_root().join("cluster-proof/work.mpl"))
        .expect("failed to read cluster-proof/work.mpl");
    assert!(
        work_source.contains("pub fn declared_work_target() -> String do")
            && work_source.contains("\"Work.execute_declared_work\"")
            && work_source.contains("pub fn execute_declared_work(request_key :: String, attempt_id :: String) -> Int do"),
        "cluster-proof/work.mpl should own the declared-work target and handler:\n{work_source}"
    );

    let continuity_source =
        fs::read_to_string(repo_root().join("cluster-proof/work_continuity.mpl"))
            .expect("failed to read cluster-proof/work_continuity.mpl");
    for needle in [
        "Continuity.mark_completed(",
        "pub fn execute_declared_work(request_key :: String, attempt_id :: String) -> Int do",
        "fn declared_work_target() -> String do",
        "keyed completion failed",
    ] {
        assert!(
            !continuity_source.contains(needle),
            "cluster-proof/work_continuity.mpl should not contain `{needle}`:\n{continuity_source}"
        );
    }
}

#[test]
fn m044_s04_auto_resume_fences_stale_primary_after_rejoin() {
    build_cluster_proof();

    let artifacts = artifact_dir("continuity-api-failover-promotion-rejoin");
    let cluster_port = dual_stack_cluster_port();
    let primary_config = ClusterProofConfig {
        node_basename: "primary".to_string(),
        advertise_host: LOOPBACK_V4.to_string(),
        cluster_port,
        http_port: unused_http_port(),
        work_delay_ms: 5_000,
        cluster_role: "primary".to_string(),
        promotion_epoch: 0,
    };
    let standby_config = ClusterProofConfig {
        node_basename: "standby".to_string(),
        advertise_host: LOOPBACK_V6.to_string(),
        cluster_port,
        http_port: unused_http_port(),
        work_delay_ms: 1_200,
        cluster_role: "standby".to_string(),
        promotion_epoch: 0,
    };
    let primary_node = expected_node_name(&primary_config);
    let standby_node = expected_node_name(&standby_config);
    let membership = vec![primary_node.clone(), standby_node.clone()];

    let mut selected_request_key: Option<String> = None;
    let mut owner_attempt_id: Option<String> = None;
    let mut failover_attempt_id: Option<String> = None;
    let mut spawned_primary_run1 = Some(spawn_cluster_proof(primary_config.clone(), &artifacts));
    let mut spawned_standby = Some(spawn_cluster_proof(standby_config.clone(), &artifacts));
    let mut stopped_primary_run1_logs: Option<StoppedClusterProof> = None;
    let mut spawned_primary_run2: Option<SpawnedClusterProof> = None;

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        {
            let primary = spawned_primary_run1
                .as_mut()
                .expect("primary run1 missing before convergence");
            let standby = spawned_standby
                .as_mut()
                .expect("standby process missing before convergence");
            let mut watched = [primary, standby];
            wait_for_membership(
                &artifacts,
                "membership-primary-run1",
                primary_config.http_port,
                &mut watched,
                &ExpectedMembershipTruth {
                    self_name: &primary_node,
                    membership: &membership,
                    cluster_role: "primary",
                    promotion_epoch: 0,
                    replication_health: "local_only",
                },
            );
        }
        {
            let primary = spawned_primary_run1
                .as_mut()
                .expect("primary run1 missing before convergence");
            let standby = spawned_standby
                .as_mut()
                .expect("standby process missing before convergence");
            let mut watched = [primary, standby];
            wait_for_membership(
                &artifacts,
                "membership-standby-run1",
                standby_config.http_port,
                &mut watched,
                &ExpectedMembershipTruth {
                    self_name: &standby_node,
                    membership: &membership,
                    cluster_role: "standby",
                    promotion_epoch: 0,
                    replication_health: "local_only",
                },
            );
        }

        let submitted = {
            let primary = spawned_primary_run1
                .as_mut()
                .expect("primary run1 missing before placement search");
            let standby = spawned_standby
                .as_mut()
                .expect("standby process missing before placement search");
            let mut watched = [primary, standby];
            find_submit_matching_placement(
                &artifacts,
                "failover-promotion-rejoin",
                primary_config.http_port,
                &mut watched,
                &primary_node,
                &standby_node,
                64,
            )
        };
        selected_request_key = Some(submitted.request_key.clone());
        assert_eq!(submitted.status_code, 200, "initial submit must succeed");
        let created_attempt = required_str(&submitted.response, "attempt_id");
        owner_attempt_id = Some(created_attempt.clone());

        write_artifact(
            &artifacts.join("scenario-meta.json"),
            serde_json::to_string_pretty(&json!({
                "request_key": submitted.request_key,
                "payload": submitted.payload,
                "attempt_id": created_attempt,
                "primary_node": primary_node,
                "standby_node": standby_node,
            }))
            .expect("failed to serialize scenario metadata"),
        );

        assert_work_truth(
            &submitted.response,
            &ExpectedWorkTruth {
                request_key: selected_request_key.as_deref().unwrap(),
                attempt_id: owner_attempt_id.as_deref(),
                phase: "submitted",
                result: "pending",
                ingress_node: &primary_node,
                owner_node: &primary_node,
                replica_node: &standby_node,
                replica_status: "mirrored",
                cluster_role: "primary",
                promotion_epoch: 0,
                replication_health: "healthy",
                execution_node: "",
                routed_remotely: false,
                fell_back_locally: true,
                ok: true,
                error: "",
                conflict_reason: "",
            },
        );
        fs::copy(
            artifacts
                .join("failover-promotion-rejoin-search")
                .join("selected.http"),
            artifacts.join("submit-primary.http"),
        )
        .expect("failed to retain raw primary submit response");
        write_artifact(
            &artifacts.join("submit-primary.json"),
            serde_json::to_string_pretty(&submitted.response).unwrap(),
        );

        stopped_primary_run1_logs = spawned_primary_run1.take().map(kill_cluster_proof);
        if let Some(logs) = stopped_primary_run1_logs.as_ref() {
            fs::copy(&logs.stdout_path, artifacts.join("primary-run1.stdout.log"))
                .expect("failed to preserve primary run1 stdout log");
            fs::copy(&logs.stderr_path, artifacts.join("primary-run1.stderr.log"))
                .expect("failed to preserve primary run1 stderr log");
        }

        let auto_pending = {
            let standby = spawned_standby
                .as_mut()
                .expect("standby process missing after primary loss");
            let mut watched = [standby];
            wait_for_auto_recovery_transition(
                &artifacts,
                "auto-recovery-pending-standby",
                standby_config.http_port,
                &mut watched,
                selected_request_key.as_deref().unwrap(),
                owner_attempt_id.as_deref().unwrap(),
                &standby_node,
                "submitted",
                "pending",
                "runtime-owned auto-recovery pending truth on standby",
            )
        };
        let recovered_attempt = required_str(&auto_pending, "attempt_id");
        failover_attempt_id = Some(recovered_attempt.clone());
        assert_ne!(recovered_attempt, owner_attempt_id.as_deref().unwrap());
        assert_work_truth(
            &auto_pending,
            &ExpectedWorkTruth {
                request_key: selected_request_key.as_deref().unwrap(),
                attempt_id: failover_attempt_id.as_deref(),
                phase: "submitted",
                result: "pending",
                ingress_node: &standby_node,
                owner_node: &standby_node,
                replica_node: "",
                replica_status: "unassigned",
                cluster_role: "primary",
                promotion_epoch: 1,
                replication_health: "local_only",
                execution_node: "",
                routed_remotely: false,
                fell_back_locally: true,
                ok: true,
                error: "",
                conflict_reason: "",
            },
        );
        {
            let standby = spawned_standby
                .as_mut()
                .expect("standby process missing after auto-promotion");
            let mut watched = [standby];
            wait_for_membership(
                &artifacts,
                "auto-promoted-membership-standby",
                standby_config.http_port,
                &mut watched,
                &ExpectedMembershipTruth {
                    self_name: &standby_node,
                    membership: std::slice::from_ref(&standby_node),
                    cluster_role: "primary",
                    promotion_epoch: 1,
                    replication_health: "local_only",
                },
            );
        }
        {
            let standby = spawned_standby
                .as_mut()
                .expect("standby process missing during auto-recovery completion");
            let mut watched = [standby];
            wait_for_work_truth(
                &artifacts,
                "auto-recovery-completed-standby",
                standby_config.http_port,
                &mut watched,
                &ExpectedWorkTruth {
                    request_key: selected_request_key.as_deref().unwrap(),
                    attempt_id: failover_attempt_id.as_deref(),
                    phase: "completed",
                    result: "succeeded",
                    ingress_node: &standby_node,
                    owner_node: &standby_node,
                    replica_node: "",
                    replica_status: "unassigned",
                    cluster_role: "primary",
                    promotion_epoch: 1,
                    replication_health: "local_only",
                    execution_node: &standby_node,
                    routed_remotely: false,
                    fell_back_locally: true,
                    ok: true,
                    error: "",
                    conflict_reason: "",
                },
                "runtime-owned auto-recovery completion truth on standby",
            );
        }

        spawned_primary_run2 = Some(spawn_cluster_proof(primary_config.clone(), &artifacts));

        {
            let primary = spawned_primary_run2
                .as_mut()
                .expect("primary run2 missing before rejoin membership");
            let standby = spawned_standby
                .as_mut()
                .expect("standby process missing before rejoin membership");
            let mut watched = [primary, standby];
            wait_for_membership(
                &artifacts,
                "membership-primary-run2",
                primary_config.http_port,
                &mut watched,
                &ExpectedMembershipTruth {
                    self_name: &primary_node,
                    membership: &membership,
                    cluster_role: "standby",
                    promotion_epoch: 1,
                    replication_health: "healthy",
                },
            );
        }
        {
            let primary = spawned_primary_run2
                .as_mut()
                .expect("primary run2 missing before standby rejoin membership");
            let standby = spawned_standby
                .as_mut()
                .expect("standby process missing before standby rejoin membership");
            let mut watched = [primary, standby];
            wait_for_membership(
                &artifacts,
                "membership-standby-run2",
                standby_config.http_port,
                &mut watched,
                &ExpectedMembershipTruth {
                    self_name: &standby_node,
                    membership: &membership,
                    cluster_role: "primary",
                    promotion_epoch: 1,
                    replication_health: "local_only",
                },
            );
        }
        {
            let primary = spawned_primary_run2
                .as_mut()
                .expect("primary run2 missing during post-rejoin status");
            let standby = spawned_standby
                .as_mut()
                .expect("standby process missing during post-rejoin status");
            let mut watched = [primary, standby];
            wait_for_work_truth(
                &artifacts,
                "post-rejoin-primary-status",
                primary_config.http_port,
                &mut watched,
                &ExpectedWorkTruth {
                    request_key: selected_request_key.as_deref().unwrap(),
                    attempt_id: failover_attempt_id.as_deref(),
                    phase: "completed",
                    result: "succeeded",
                    ingress_node: &standby_node,
                    owner_node: &standby_node,
                    replica_node: "",
                    replica_status: "unassigned",
                    cluster_role: "standby",
                    promotion_epoch: 1,
                    replication_health: "healthy",
                    execution_node: &standby_node,
                    routed_remotely: false,
                    fell_back_locally: true,
                    ok: true,
                    error: "",
                    conflict_reason: "",
                },
                "fenced old primary status after rejoin",
            );
        }
        {
            let primary = spawned_primary_run2
                .as_mut()
                .expect("primary run2 missing during promoted standby status");
            let standby = spawned_standby
                .as_mut()
                .expect("standby process missing during promoted standby status");
            let mut watched = [primary, standby];
            wait_for_work_truth(
                &artifacts,
                "post-rejoin-standby-status",
                standby_config.http_port,
                &mut watched,
                &ExpectedWorkTruth {
                    request_key: selected_request_key.as_deref().unwrap(),
                    attempt_id: failover_attempt_id.as_deref(),
                    phase: "completed",
                    result: "succeeded",
                    ingress_node: &standby_node,
                    owner_node: &standby_node,
                    replica_node: "",
                    replica_status: "unassigned",
                    cluster_role: "primary",
                    promotion_epoch: 1,
                    replication_health: "local_only",
                    execution_node: &standby_node,
                    routed_remotely: false,
                    fell_back_locally: true,
                    ok: true,
                    error: "",
                    conflict_reason: "",
                },
                "promoted standby status after rejoin",
            );
        }

        let stale_guard = json_body(
            &artifacts,
            "stale-guard-primary",
            &post_json(
                primary_config.http_port,
                "/work",
                &format!(
                    r#"{{"request_key":"{}","payload":"{}"}}"#,
                    selected_request_key.as_deref().unwrap(),
                    submitted.payload
                ),
            ),
            200,
            "same-key submit on fenced old primary",
        );
        assert_work_truth(
            &stale_guard,
            &ExpectedWorkTruth {
                request_key: selected_request_key.as_deref().unwrap(),
                attempt_id: failover_attempt_id.as_deref(),
                phase: "completed",
                result: "succeeded",
                ingress_node: &standby_node,
                owner_node: &standby_node,
                replica_node: "",
                replica_status: "unassigned",
                cluster_role: "standby",
                promotion_epoch: 1,
                replication_health: "healthy",
                execution_node: &standby_node,
                routed_remotely: false,
                fell_back_locally: true,
                ok: true,
                error: "",
                conflict_reason: "",
            },
        );
    }));

    let standby_logs = stop_cluster_proof(
        spawned_standby
            .take()
            .expect("standby process missing during cleanup"),
    );
    let primary_run2_logs = spawned_primary_run2.take().map(stop_cluster_proof);
    let primary_run1_logs = match stopped_primary_run1_logs {
        Some(logs) => logs,
        None => stop_cluster_proof(
            spawned_primary_run1
                .take()
                .expect("primary run1 missing during cleanup"),
        ),
    };

    if let Err(payload) = result {
        let run2_stdout = primary_run2_logs
            .as_ref()
            .map(|logs| logs.stdout.as_str())
            .unwrap_or("");
        let run2_stderr = primary_run2_logs
            .as_ref()
            .map(|logs| logs.stderr.as_str())
            .unwrap_or("");
        panic!(
            "failover promotion + fenced rejoin assertions failed: {}
artifacts: {}
primary run1 stdout:
{}
primary run1 stderr:
{}
primary run2 stdout:
{}
primary run2 stderr:
{}
standby stdout:
{}
standby stderr:
{}",
            panic_payload_to_string(payload),
            artifacts.display(),
            primary_run1_logs.stdout,
            primary_run1_logs.stderr,
            run2_stdout,
            run2_stderr,
            standby_logs.stdout,
            standby_logs.stderr
        );
    }

    fs::copy(
        &standby_logs.stdout_path,
        artifacts.join("standby-run1.stdout.log"),
    )
    .expect("failed to preserve standby stdout log");
    fs::copy(
        &standby_logs.stderr_path,
        artifacts.join("standby-run1.stderr.log"),
    )
    .expect("failed to preserve standby stderr log");

    let request_key = selected_request_key
        .as_deref()
        .expect("selected request key missing after successful run");
    let old_attempt = owner_attempt_id
        .as_deref()
        .expect("owner attempt id missing after successful run");
    let new_attempt = failover_attempt_id
        .as_deref()
        .expect("failover attempt id missing after successful run");
    let primary_run2_logs =
        primary_run2_logs.expect("primary run2 logs missing after successful run");

    fs::copy(
        &primary_run2_logs.stdout_path,
        artifacts.join("primary-run2.stdout.log"),
    )
    .expect("failed to preserve primary run2 stdout log");
    fs::copy(
        &primary_run2_logs.stderr_path,
        artifacts.join("primary-run2.stderr.log"),
    )
    .expect("failed to preserve primary run2 stderr log");

    assert_log_contains(
        &primary_run1_logs,
        &format!("[cluster-proof] Config loaded mode=cluster node={primary_node}"),
    );
    assert_log_contains(
        &standby_logs,
        &format!("[cluster-proof] Config loaded mode=cluster node={standby_node}"),
    );
    assert_log_contains(
        &primary_run1_logs,
        "[cluster-proof] Runtime authority ready cluster_role=primary promotion_epoch=0",
    );
    assert_log_contains(
        &standby_logs,
        "[cluster-proof] Runtime authority ready cluster_role=standby promotion_epoch=0",
    );
    assert_log_contains(
        &standby_logs,
        &format!(
            "[mesh-rt continuity] transition=automatic_promotion disconnected_node={} previous_epoch=0 next_epoch=1",
            primary_node
        ),
    );
    assert_log_contains(
        &standby_logs,
        "[mesh-rt continuity] transition=promote previous_role=standby previous_epoch=0 next_role=primary next_epoch=1",
    );
    assert_log_contains(
        &standby_logs,
        &format!(
            "[mesh-rt continuity] transition=automatic_recovery request_key={request_key} previous_attempt_id={old_attempt} next_attempt_id={new_attempt} runtime_name=Work.execute_declared_work"
        ),
    );
    assert_log_contains(
        &standby_logs,
        &format!(
            "[mesh-rt continuity] transition=recovery_rollover request_key={request_key} previous_attempt_id={old_attempt} next_attempt_id={new_attempt}"
        ),
    );
    assert_log_absent(
        &standby_logs,
        &format!("[cluster-proof] keyed submit request_key={request_key} attempt_id={new_attempt}"),
    );
    assert_log_contains(
        &standby_logs,
        &format!(
            "[cluster-proof] work executed request_key={request_key} attempt_id={new_attempt} execution={standby_node}"
        ),
    );
    assert_log_absent(
        &primary_run1_logs,
        "[cluster-proof] keyed completion failed",
    );
    assert_log_absent(&standby_logs, "[cluster-proof] keyed completion failed");
    assert_log_absent(
        &primary_run1_logs,
        &format!(
            "[mesh-rt continuity] transition=completed request_key={request_key} attempt_id={old_attempt}"
        ),
    );
    assert_log_contains(
        &primary_run2_logs,
        &format!(
            "[mesh-rt continuity] transition=fenced_rejoin request_key={request_key} attempt_id={new_attempt} previous_role=primary previous_epoch=0 next_role=standby next_epoch=1"
        ),
    );
    assert_log_contains(
        &primary_run2_logs,
        &format!(
            "[cluster-proof] keyed status request_key={request_key} attempt_id={new_attempt} phase=completed result=succeeded owner={standby_node} replica= source={primary_node} replica_status=unassigned cluster_role=standby promotion_epoch=1"
        ),
    );
    assert_log_absent(
        &primary_run2_logs,
        "[cluster-proof] keyed completion failed",
    );
    assert_log_absent(
        &primary_run2_logs,
        &format!(
            "[cluster-proof] work executed request_key={request_key} attempt_id={new_attempt} execution={primary_node}"
        ),
    );
}

#[test]
fn m044_s04_auto_promotion_promotes_and_resumes_without_retry() {
    m044_s04_auto_resume_fences_stale_primary_after_rejoin();
}

#[test]
fn continuity_api_authority_status_round_trip_runtime_truth() {
    let artifacts = artifact_dir("continuity-api-authority-runtime");
    let output = build_and_run_mesh(
        AUTHORITY_RUNTIME_SOURCE,
        &[
            ("MESH_CONTINUITY_ROLE", "standby"),
            ("MESH_CONTINUITY_PROMOTION_EPOCH", "0"),
        ],
        &artifacts,
    );

    assert!(
        output.status.success(),
        "mesh program failed:\n{}\nartifacts: {}",
        command_output_text(&output),
        artifacts.display()
    );

    let lines = stdout_lines(&output);
    assert_eq!(
        lines,
        vec![
            "before|ok|standby|0|local_only",
            "ack|ok|mirrored|healthy",
            "after_ack|ok|standby|0|healthy",
        ],
        "unexpected runtime authority output; artifacts: {}",
        artifacts.display()
    );
}

#[test]
fn continuity_api_primary_authority_status_preserves_runtime_truth() {
    let artifacts = artifact_dir("continuity-api-primary-authority-status");
    let output = build_and_run_mesh(PRIMARY_AUTHORITY_RUNTIME_SOURCE, &[], &artifacts);

    assert!(
        output.status.success(),
        "mesh program failed:\n{}\nartifacts: {}",
        command_output_text(&output),
        artifacts.display()
    );

    let lines = stdout_lines(&output);
    assert_eq!(
        lines,
        vec!["status|ok|primary|0|local_only"],
        "unexpected primary authority output; artifacts: {}",
        artifacts.display()
    );
}

#[test]
fn m044_s04_manual_surface_manual_promotion_is_disabled() {
    let artifacts = artifact_dir("continuity-api-manual-promotion-disabled");
    let output = build_only_mesh(MANUAL_PROMOTION_DISABLED_SOURCE, &artifacts);

    assert!(
        !output.status.success(),
        "manual continuity promotion should fail compilation; artifacts: {}",
        artifacts.display()
    );

    let combined = command_output_text(&output);
    assert!(
        combined.contains("Continuity.promote()")
            || combined.contains("automatic-only")
            || combined.contains("authority_status"),
        "compile failure should explain that manual promotion is disabled:\n{}\nartifacts: {}",
        combined,
        artifacts.display()
    );
}

#[test]
fn m044_s04_manual_surface_promote_wrong_arity_fails_at_compile_time() {
    let artifacts = artifact_dir("continuity-api-promote-wrong-arity");
    let output = build_only_mesh(
        r#"
fn main() do
  let _ = Continuity.promote("extra")
  println("unreachable")
end
"#,
        &artifacts,
    );

    assert!(
        !output.status.success(),
        "wrong-arity promote call should fail compilation; artifacts: {}",
        artifacts.display()
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = command_output_text(&output);
    assert!(
        stderr.contains("promote")
            || combined.contains("Continuity.promote()")
            || combined.contains("automatic-only")
            || combined.contains("argument"),
        "compile failure should mention that manual promotion is disabled:\n{}\nartifacts: {}",
        combined,
        artifacts.display()
    );
}

#[test]
fn continuity_api_authority_status_wrong_result_shape_fails_at_compile_time() {
    let artifacts = artifact_dir("continuity-api-authority-status-wrong-shape");
    let output = build_only_mesh(
        r##"
fn main() do
  let impossible = Continuity.authority_status() + 1
  println("#{impossible}")
end
"##,
        &artifacts,
    );

    assert!(
        !output.status.success(),
        "wrong-result-shape authority_status call should fail compilation; artifacts: {}",
        artifacts.display()
    );

    let combined = command_output_text(&output);
    assert!(
        combined.contains("authority_status")
            || combined.contains("type")
            || combined.contains("Result"),
        "compile failure should mention the bad authority_status result usage:\n{}\nartifacts: {}",
        combined,
        artifacts.display()
    );
}
