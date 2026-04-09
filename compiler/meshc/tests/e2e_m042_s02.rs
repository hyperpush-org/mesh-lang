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
const SHARED_COOKIE: &str = "mesh-m042-s02-cookie";

static BUILD_CLUSTER_PROOF_ONCE: Once = Once::new();

#[derive(Clone, Debug)]
struct ClusterProofConfig {
    node_basename: String,
    advertise_host: String,
    cluster_port: Option<u16>,
    http_port: u16,
    work_delay_ms: Option<u64>,
}

struct SpawnedClusterProof {
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

fn unused_http_port() -> u16 {
    TcpListener::bind((LOOPBACK_V4, 0))
        .expect("failed to bind ephemeral HTTP port")
        .local_addr()
        .expect("failed to read ephemeral HTTP port")
        .port()
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

fn artifact_dir(test_name: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let dir = repo_root()
        .join(".tmp")
        .join("m042-s02")
        .join(format!("{test_name}-{stamp}"));
    fs::create_dir_all(&dir).expect("failed to create e2e artifact dir");
    dir
}

fn write_artifact(path: &Path, contents: impl AsRef<str>) {
    fs::write(path, contents.as_ref())
        .unwrap_or_else(|error| panic!("failed to write artifact {}: {error}", path.display()));
}

fn spawn_cluster_proof(config: ClusterProofConfig, artifacts: &Path) -> SpawnedClusterProof {
    let binary = cluster_proof_binary();
    assert!(
        binary.exists(),
        "cluster-proof binary not found at {}. Run `meshc build cluster-proof` first.",
        binary.display()
    );

    let stdout_path = artifacts.join(format!("{}.stdout.log", config.node_basename));
    let stderr_path = artifacts.join(format!("{}.stderr.log", config.node_basename));
    let stdout = File::create(&stdout_path).expect("failed to create stdout log");
    let stderr = File::create(&stderr_path).expect("failed to create stderr log");

    let mut cmd = Command::new(binary);
    cmd.current_dir(repo_root().join("cluster-proof"))
        .env("PORT", config.http_port.to_string())
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr));

    if let Some(delay_ms) = config.work_delay_ms {
        cmd.env("CLUSTER_PROOF_WORK_DELAY_MS", delay_ms.to_string());
    }

    if let Some(cluster_port) = config.cluster_port {
        cmd.env("MESH_CLUSTER_PORT", cluster_port.to_string())
            .env("CLUSTER_PROOF_COOKIE", SHARED_COOKIE)
            .env("MESH_DISCOVERY_SEED", DISCOVERY_SEED)
            .env("CLUSTER_PROOF_NODE_BASENAME", &config.node_basename)
            .env("CLUSTER_PROOF_ADVERTISE_HOST", &config.advertise_host);
    }

    let child = cmd.spawn().expect("failed to start cluster-proof");

    SpawnedClusterProof {
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
    } = spawned;

    let _ = child.kill();
    let _ = child.wait();

    let stdout = fs::read_to_string(&stdout_path).unwrap_or_default();
    let stderr = fs::read_to_string(&stderr_path).unwrap_or_default();
    let combined = format!("{stdout}{stderr}");

    StoppedClusterProof {
        stdout,
        stderr,
        combined,
        stdout_path,
        stderr_path,
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
    format!(
        "{}@{}:{}",
        config.node_basename,
        host,
        config.cluster_port.expect("cluster port expected")
    )
}

fn wait_for_membership(
    artifacts: &Path,
    name: &str,
    port: u16,
    expected_self: &str,
    expected_membership: &[String],
    timeout: Duration,
) -> Value {
    const REQUIRED_STABLE_POLLS: usize = 5;

    let start = Instant::now();
    let mut last_body = String::new();
    let mut stable_polls = 0usize;
    let mut last_match: Option<Value> = None;

    while start.elapsed() < timeout {
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
        timeout,
        timeout_path.display()
    );
}

fn wait_for_status_condition<F>(
    artifacts: &Path,
    name: &str,
    port: u16,
    request_key: &str,
    timeout: Duration,
    predicate_description: &str,
    mut predicate: F,
) -> Value
where
    F: FnMut(&Value) -> bool,
{
    let start = Instant::now();
    let path = format!("/work/{request_key}");
    let mut last_body = String::new();

    while start.elapsed() < timeout {
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

fn wait_for_completed_status(
    artifacts: &Path,
    name: &str,
    port: u16,
    request_key: &str,
    timeout: Duration,
) -> Value {
    wait_for_status_condition(
        artifacts,
        name,
        port,
        request_key,
        timeout,
        "completed continuity status",
        |json| required_str(json, "phase") == "completed",
    )
}

fn deterministic_sort_score(value: &str) -> u64 {
    let digest = Sha256::digest(value.as_bytes());
    let mut acc = 0u64;
    for idx in 0..15 {
        let byte = digest[idx / 2];
        let nibble = if idx % 2 == 0 { byte >> 4 } else { byte & 0x0f };
        acc = (acc << 4) | u64::from(nibble);
    }
    acc
}

fn placement_score(request_key: &str, node_name: &str) -> u64 {
    deterministic_sort_score(&format!("{request_key}::{node_name}"))
}

fn placement_tie_breaker(node_name: &str) -> u64 {
    deterministic_sort_score(&format!("member::{node_name}"))
}

fn request_key_matches_placement(
    request_key: &str,
    desired_owner: &str,
    desired_replica: &str,
) -> bool {
    let owner_score = placement_score(request_key, desired_owner);
    let replica_score = placement_score(request_key, desired_replica);
    owner_score < replica_score
        || (owner_score == replica_score
            && placement_tie_breaker(desired_owner) < placement_tie_breaker(desired_replica))
}

fn find_submit_matching_placement(
    artifacts: &Path,
    prefix: &str,
    port: u16,
    desired_owner: &str,
    desired_replica: &str,
    max_attempts: usize,
) -> SubmittedRequest {
    let search_root = artifacts.join(format!("{prefix}-search"));
    fs::create_dir_all(&search_root).expect("failed to create placement search artifact root");

    for idx in 0..max_attempts {
        let request_key = format!("{prefix}-key-{idx}");
        let payload = format!("payload-{idx}");
        if !request_key_matches_placement(&request_key, desired_owner, desired_replica) {
            continue;
        }

        let chosen_path = search_root.join("chosen.json");
        write_artifact(
            &chosen_path,
            &format!(
                "{{\n  \"request_key\": \"{request_key}\",\n  \"payload\": \"{payload}\",\n  \"owner_node\": \"{desired_owner}\",\n  \"replica_node\": \"{desired_replica}\"\n}}"
            ),
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
fn continuity_api_archives_non_json_http_response_as_contract_failure() {
    let artifacts = artifact_dir("continuity-api-malformed-response");
    let response = HttpResponse {
        status_code: 200,
        body: "not-json".to_string(),
        raw: "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 8\r\nConnection: close\r\n\r\nnot-json".to_string(),
    };

    let result = std::panic::catch_unwind(|| {
        let _ = parse_json_response(
            &artifacts,
            "malformed-response",
            &response,
            "malformed response contract",
        );
    });

    assert!(result.is_err(), "non-JSON response should fail closed");
    assert!(artifacts.join("malformed-response.http").is_file());
    assert!(artifacts.join("malformed-response.body.txt").is_file());
    assert_eq!(
        fs::read_to_string(artifacts.join("malformed-response.body.txt")).unwrap(),
        "not-json"
    );
}

#[test]
fn continuity_api_single_node_cluster_rejects_replica_required_submit_and_replays_status() {
    build_cluster_proof();

    let artifacts = artifact_dir("continuity-api-single-node-rejection");
    let cluster_port = dual_stack_cluster_port();
    let config = ClusterProofConfig {
        node_basename: "node-solo".to_string(),
        advertise_host: LOOPBACK_V4.to_string(),
        cluster_port: Some(cluster_port),
        http_port: unused_http_port(),
        work_delay_ms: None,
    };
    let expected_self = expected_node_name(&config);
    let spawned = spawn_cluster_proof(config.clone(), &artifacts);

    let result = std::panic::catch_unwind(|| {
        wait_for_membership(
            &artifacts,
            "membership-node-solo",
            config.http_port,
            &expected_self,
            std::slice::from_ref(&expected_self),
            Duration::from_secs(12),
        );

        let invalid = json_body(
            &artifacts,
            "invalid-submit",
            &post_json(
                config.http_port,
                "/work",
                r#"{"request_key":"","payload":"hello"}"#,
            ),
            400,
            "invalid keyed submit body",
        );
        assert_eq!(required_str(&invalid, "phase"), "invalid");
        assert_eq!(required_str(&invalid, "result"), "rejected");
        assert_eq!(required_str(&invalid, "error"), "request_key is required");
        assert!(!required_bool(&invalid, "ok"));

        let rejected = json_body(
            &artifacts,
            "rejected-submit",
            &post_json(
                config.http_port,
                "/work",
                r#"{"request_key":"m042-s02-rejected-key","payload":"hello"}"#,
            ),
            503,
            "replica-required submit without peer",
        );
        let attempt_id = required_str(&rejected, "attempt_id");
        assert_eq!(
            required_str(&rejected, "request_key"),
            "m042-s02-rejected-key"
        );
        assert_eq!(required_str(&rejected, "phase"), "rejected");
        assert_eq!(required_str(&rejected, "result"), "rejected");
        assert_eq!(required_str(&rejected, "ingress_node"), expected_self);
        assert_eq!(required_str(&rejected, "owner_node"), expected_self);
        assert_eq!(required_str(&rejected, "replica_node"), "");
        assert_eq!(required_str(&rejected, "replica_status"), "rejected");
        assert_eq!(required_str(&rejected, "execution_node"), "");
        assert_eq!(
            required_str(&rejected, "error"),
            "replica_required_unavailable"
        );
        assert!(!required_bool(&rejected, "routed_remotely"));
        assert!(required_bool(&rejected, "fell_back_locally"));
        assert!(!required_bool(&rejected, "ok"));

        let duplicate = json_body(
            &artifacts,
            "rejected-duplicate",
            &post_json(
                config.http_port,
                "/work",
                r#"{"request_key":"m042-s02-rejected-key","payload":"hello"}"#,
            ),
            503,
            "duplicate rejected submit replay",
        );
        assert_eq!(required_str(&duplicate, "attempt_id"), attempt_id);
        assert_eq!(required_str(&duplicate, "phase"), "rejected");
        assert_eq!(required_str(&duplicate, "result"), "rejected");
        assert_eq!(required_str(&duplicate, "replica_status"), "rejected");
        assert_eq!(
            required_str(&duplicate, "error"),
            "replica_required_unavailable"
        );
        assert_eq!(required_str(&duplicate, "conflict_reason"), "");
        assert!(!required_bool(&duplicate, "ok"));

        let status = json_body(
            &artifacts,
            "rejected-status",
            &get_json(config.http_port, "/work/m042-s02-rejected-key"),
            200,
            "stored rejected status",
        );
        assert_eq!(required_str(&status, "attempt_id"), attempt_id);
        assert_eq!(required_str(&status, "phase"), "rejected");
        assert_eq!(required_str(&status, "result"), "rejected");
        assert_eq!(required_str(&status, "replica_status"), "rejected");
        assert_eq!(
            required_str(&status, "error"),
            "replica_required_unavailable"
        );
        assert!(!required_bool(&status, "ok"));

        let conflict = json_body(
            &artifacts,
            "rejected-conflict",
            &post_json(
                config.http_port,
                "/work",
                r#"{"request_key":"m042-s02-rejected-key","payload":"different"}"#,
            ),
            409,
            "conflicting retry after rejected admission",
        );
        assert_eq!(required_str(&conflict, "attempt_id"), attempt_id);
        assert_eq!(required_str(&conflict, "phase"), "rejected");
        assert_eq!(required_str(&conflict, "result"), "rejected");
        assert_eq!(required_str(&conflict, "replica_status"), "rejected");
        assert_eq!(
            required_str(&conflict, "conflict_reason"),
            "request_key_conflict"
        );
        assert!(!required_bool(&conflict, "ok"));
    });

    let logs = stop_cluster_proof(spawned);
    if let Err(payload) = result {
        panic!(
            "single-node replica rejection failed: {}\nartifacts: {}\nstdout:\n{}\nstderr:\n{}",
            panic_payload_to_string(payload),
            artifacts.display(),
            logs.stdout,
            logs.stderr
        );
    }

    assert_log_contains(&logs, "continuity=runtime-native");
    assert_log_contains(
        &logs,
        "[mesh-rt continuity] transition=rejected request_key=m042-s02-rejected-key",
    );
    assert_log_contains(
        &logs,
        "[cluster-proof] keyed rejected request_key=m042-s02-rejected-key",
    );
}

#[test]
fn continuity_api_two_node_local_owner_mirrors_status_between_owner_and_replica() {
    build_cluster_proof();

    let artifacts = artifact_dir("continuity-api-two-node-mirrored");
    let cluster_port = dual_stack_cluster_port();
    let owner_config = ClusterProofConfig {
        node_basename: "node-a".to_string(),
        advertise_host: LOOPBACK_V4.to_string(),
        cluster_port: Some(cluster_port),
        http_port: unused_http_port(),
        work_delay_ms: Some(1200),
    };
    let replica_config = ClusterProofConfig {
        node_basename: "node-b".to_string(),
        advertise_host: LOOPBACK_V6.to_string(),
        cluster_port: Some(cluster_port),
        http_port: unused_http_port(),
        work_delay_ms: Some(1200),
    };
    let owner_node = expected_node_name(&owner_config);
    let replica_node = expected_node_name(&replica_config);
    let membership = vec![owner_node.clone(), replica_node.clone()];

    let mut selected_request_key: Option<String> = None;
    let mut selected_payload: Option<String> = None;
    let spawned_owner = spawn_cluster_proof(owner_config.clone(), &artifacts);
    let spawned_replica = spawn_cluster_proof(replica_config.clone(), &artifacts);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        wait_for_membership(
            &artifacts,
            "membership-node-a",
            owner_config.http_port,
            &owner_node,
            &membership,
            Duration::from_secs(12),
        );
        wait_for_membership(
            &artifacts,
            "membership-node-b",
            replica_config.http_port,
            &replica_node,
            &membership,
            Duration::from_secs(12),
        );
        sleep(Duration::from_secs(1));

        let submitted = find_submit_matching_placement(
            &artifacts,
            "mirrored-local-owner",
            owner_config.http_port,
            &owner_node,
            &replica_node,
            64,
        );
        selected_request_key = Some(submitted.request_key.clone());
        selected_payload = Some(submitted.payload.clone());
        assert_eq!(
            submitted.status_code, 200,
            "expected mirrored submit to be accepted"
        );

        let create = submitted.response;
        let attempt_id = required_str(&create, "attempt_id");
        assert_eq!(required_str(&create, "phase"), "submitted");
        assert_eq!(required_str(&create, "result"), "pending");
        assert_eq!(required_str(&create, "ingress_node"), owner_node);
        assert_eq!(required_str(&create, "owner_node"), owner_node);
        assert_eq!(required_str(&create, "replica_node"), replica_node);
        assert_eq!(required_str(&create, "replica_status"), "mirrored");
        assert_eq!(required_str(&create, "execution_node"), "");
        assert!(!required_bool(&create, "routed_remotely"));
        assert!(required_bool(&create, "fell_back_locally"));
        assert!(required_bool(&create, "ok"));

        let pending_owner = wait_for_status_condition(
            &artifacts,
            "pending-owner-status",
            owner_config.http_port,
            selected_request_key.as_deref().unwrap(),
            Duration::from_secs(5),
            "pending mirrored status on owner",
            |json| {
                required_str(json, "phase") == "submitted"
                    && required_str(json, "result") == "pending"
                    && required_str(json, "replica_status") == "mirrored"
            },
        );
        let pending_replica = wait_for_status_condition(
            &artifacts,
            "pending-replica-status",
            replica_config.http_port,
            selected_request_key.as_deref().unwrap(),
            Duration::from_secs(5),
            "pending mirrored status on replica",
            |json| {
                required_str(json, "phase") == "submitted"
                    && required_str(json, "result") == "pending"
                    && required_str(json, "replica_status") == "mirrored"
            },
        );

        for status in [&pending_owner, &pending_replica] {
            assert_eq!(required_str(status, "attempt_id"), attempt_id);
            assert_eq!(required_str(status, "owner_node"), owner_node);
            assert_eq!(required_str(status, "replica_node"), replica_node);
            assert!(required_bool(status, "ok"));
        }

        let completed_owner = wait_for_completed_status(
            &artifacts,
            "completed-owner-status",
            owner_config.http_port,
            selected_request_key.as_deref().unwrap(),
            Duration::from_secs(12),
        );
        let completed_replica = wait_for_completed_status(
            &artifacts,
            "completed-replica-status",
            replica_config.http_port,
            selected_request_key.as_deref().unwrap(),
            Duration::from_secs(12),
        );

        for status in [&completed_owner, &completed_replica] {
            assert_eq!(required_str(status, "attempt_id"), attempt_id);
            assert_eq!(required_str(status, "phase"), "completed");
            assert_eq!(required_str(status, "result"), "succeeded");
            assert_eq!(required_str(status, "owner_node"), owner_node);
            assert_eq!(required_str(status, "replica_node"), replica_node);
            assert_eq!(required_str(status, "replica_status"), "mirrored");
            assert_eq!(required_str(status, "execution_node"), owner_node);
            assert!(required_bool(status, "ok"));
        }

        let duplicate_body = format!(
            r#"{{"request_key":"{}","payload":"{}"}}"#,
            selected_request_key.as_deref().unwrap(),
            selected_payload.as_deref().unwrap()
        );
        let duplicate = json_body(
            &artifacts,
            "completed-duplicate",
            &post_json(owner_config.http_port, "/work", &duplicate_body),
            200,
            "duplicate mirrored submit replay",
        );
        assert_eq!(required_str(&duplicate, "attempt_id"), attempt_id);
        assert_eq!(required_str(&duplicate, "phase"), "completed");
        assert_eq!(required_str(&duplicate, "replica_status"), "mirrored");
        assert!(required_bool(&duplicate, "ok"));
    }));

    let owner_logs = stop_cluster_proof(spawned_owner);
    let replica_logs = stop_cluster_proof(spawned_replica);
    if let Err(payload) = result {
        panic!(
            "two-node mirrored continuity_api failed: {}\nartifacts: {}\nowner stdout:\n{}\nowner stderr:\n{}\nreplica stdout:\n{}\nreplica stderr:\n{}",
            panic_payload_to_string(payload),
            artifacts.display(),
            owner_logs.stdout,
            owner_logs.stderr,
            replica_logs.stdout,
            replica_logs.stderr
        );
    }

    assert_log_contains(&owner_logs, "continuity=runtime-native");
    assert_log_contains(&replica_logs, "continuity=runtime-native");
    assert_log_contains(&owner_logs, "transition=replica_ack");
    if let Some(request_key) = selected_request_key.as_deref() {
        assert_log_contains(
            &owner_logs,
            &format!("[cluster-proof] keyed dispatch request_key={request_key}"),
        );
        assert_log_contains(
            &owner_logs,
            &format!("[cluster-proof] work executed request_key={request_key}"),
        );
    }
}

#[test]
fn continuity_api_replica_loss_degrades_pending_mirrored_status() {
    build_cluster_proof();

    let artifacts = artifact_dir("continuity-api-two-node-degraded");
    let cluster_port = dual_stack_cluster_port();
    let owner_config = ClusterProofConfig {
        node_basename: "node-a".to_string(),
        advertise_host: LOOPBACK_V4.to_string(),
        cluster_port: Some(cluster_port),
        http_port: unused_http_port(),
        work_delay_ms: Some(5000),
    };
    let replica_config = ClusterProofConfig {
        node_basename: "node-b".to_string(),
        advertise_host: LOOPBACK_V6.to_string(),
        cluster_port: Some(cluster_port),
        http_port: unused_http_port(),
        work_delay_ms: Some(5000),
    };
    let owner_node = expected_node_name(&owner_config);
    let replica_node = expected_node_name(&replica_config);
    let membership = vec![owner_node.clone(), replica_node.clone()];

    let mut selected_request_key: Option<String> = None;
    let mut spawned_owner = Some(spawn_cluster_proof(owner_config.clone(), &artifacts));
    let mut spawned_replica = Some(spawn_cluster_proof(replica_config.clone(), &artifacts));
    let mut replica_logs: Option<StoppedClusterProof> = None;

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        wait_for_membership(
            &artifacts,
            "membership-node-a",
            owner_config.http_port,
            &owner_node,
            &membership,
            Duration::from_secs(12),
        );
        wait_for_membership(
            &artifacts,
            "membership-node-b",
            replica_config.http_port,
            &replica_node,
            &membership,
            Duration::from_secs(12),
        );

        let submitted = find_submit_matching_placement(
            &artifacts,
            "degraded-local-owner",
            owner_config.http_port,
            &owner_node,
            &replica_node,
            64,
        );
        selected_request_key = Some(submitted.request_key.clone());
        assert_eq!(
            submitted.status_code, 200,
            "expected mirrored submit to be accepted before replica loss"
        );

        let create = submitted.response;
        let attempt_id = required_str(&create, "attempt_id");
        assert_eq!(required_str(&create, "phase"), "submitted");
        assert_eq!(required_str(&create, "result"), "pending");
        assert_eq!(required_str(&create, "owner_node"), owner_node);
        assert_eq!(required_str(&create, "replica_node"), replica_node);
        assert_eq!(required_str(&create, "replica_status"), "mirrored");
        assert!(required_bool(&create, "ok"));

        let pending_owner = wait_for_status_condition(
            &artifacts,
            "pending-owner-status",
            owner_config.http_port,
            selected_request_key.as_deref().unwrap(),
            Duration::from_secs(5),
            "pending mirrored status before replica loss",
            |json| {
                required_str(json, "phase") == "submitted"
                    && required_str(json, "result") == "pending"
                    && required_str(json, "replica_status") == "mirrored"
            },
        );
        assert_eq!(required_str(&pending_owner, "attempt_id"), attempt_id);

        replica_logs = spawned_replica.take().map(stop_cluster_proof);

        let degraded = wait_for_status_condition(
            &artifacts,
            "degraded-owner-status",
            owner_config.http_port,
            selected_request_key.as_deref().unwrap(),
            Duration::from_secs(12),
            "degraded continuity status after replica loss",
            |json| {
                required_str(json, "phase") == "submitted"
                    && required_str(json, "result") == "pending"
                    && required_str(json, "replica_status") == "degraded_continuing"
            },
        );
        assert_eq!(required_str(&degraded, "attempt_id"), attempt_id);
        assert_eq!(required_str(&degraded, "owner_node"), owner_node);
        assert_eq!(required_str(&degraded, "replica_node"), replica_node);
        assert_eq!(required_str(&degraded, "execution_node"), "");
        assert_eq!(required_str(&degraded, "error"), "");
        assert!(required_bool(&degraded, "ok"));
    }));

    let owner_logs = stop_cluster_proof(spawned_owner.take().unwrap());
    let replica_logs =
        replica_logs.unwrap_or_else(|| stop_cluster_proof(spawned_replica.take().unwrap()));
    if let Err(payload) = result {
        panic!(
            "replica-loss degraded continuity_api failed: {}\nartifacts: {}\nowner stdout:\n{}\nowner stderr:\n{}\nreplica stdout:\n{}\nreplica stderr:\n{}",
            panic_payload_to_string(payload),
            artifacts.display(),
            owner_logs.stdout,
            owner_logs.stderr,
            replica_logs.stdout,
            replica_logs.stderr
        );
    }

    assert_log_contains(&owner_logs, "continuity=runtime-native");
    assert_log_contains(&replica_logs, "continuity=runtime-native");
    if let Some(request_key) = selected_request_key.as_deref() {
        assert_log_contains(
            &owner_logs,
            &format!("transition=degraded request_key={request_key}"),
        );
        assert_log_contains(
            &owner_logs,
            &format!("[cluster-proof] keyed status request_key={request_key}"),
        );
    }
}
