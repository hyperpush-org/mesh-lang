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
const SHARED_COOKIE: &str = "mesh-m042-s01-cookie";
const STANDALONE_NODE: &str = "standalone@local";

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
        .join("m042-s01")
        .join(format!("{test_name}-{stamp}"));
    fs::create_dir_all(&dir).expect("failed to create e2e artifact dir");
    dir
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
        ..
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

fn json_body(response: &HttpResponse, expected_status: u16, context: &str) -> Value {
    assert!(
        response.status_code == expected_status,
        "expected HTTP {expected_status} for {context}, got raw response:\n{}",
        response.raw
    );
    serde_json::from_str(&response.body).unwrap_or_else(|error| {
        panic!(
            "expected JSON body for {context}, got {error}: {}",
            response.body
        )
    })
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

fn wait_for_membership(port: u16, expected_self: &str, expected_membership: &[String]) {
    let start = Instant::now();
    let mut last_body = String::new();

    while start.elapsed() < Duration::from_secs(12) {
        match try_get_json(port, "/membership") {
            Ok(response) => {
                if response.status_code == 200 {
                    let json: Value =
                        serde_json::from_str(&response.body).unwrap_or_else(|error| {
                            panic!(
                                "membership response was not JSON: {error}: {}",
                                response.body
                            )
                        });
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
                        return;
                    }
                } else {
                    last_body = response.raw;
                }
            }
            Err(error) => {
                last_body = error.to_string();
            }
        }
        sleep(Duration::from_millis(100));
    }

    panic!(
        "membership did not converge on :{} within timeout; last body: {}",
        port, last_body
    );
}

fn wait_for_completed_status(port: u16, request_key: &str, timeout: Duration) -> Value {
    let start = Instant::now();
    let path = format!("/work/{request_key}");
    let mut last_body = String::new();

    while start.elapsed() < timeout {
        match try_get_json(port, &path) {
            Ok(response) => {
                if response.status_code == 200 {
                    let json: Value =
                        serde_json::from_str(&response.body).unwrap_or_else(|error| {
                            panic!("status response was not JSON: {error}: {}", response.body)
                        });
                    last_body = response.body.clone();
                    if required_str(&json, "phase") == "completed" {
                        return json;
                    }
                } else {
                    last_body = response.raw;
                }
            }
            Err(error) => {
                last_body = error.to_string();
            }
        }
        sleep(Duration::from_millis(100));
    }

    panic!("request {request_key} never reached completed state; last body: {last_body}");
}

fn wait_for_remote_owner_submit(port: u16, ingress_node: &str) -> Value {
    for idx in 0..16 {
        let request_key = format!("m042-s01-remote-key-{idx}");
        let body = format!(r#"{{"request_key":"{request_key}","payload":"hello-{idx}"}}"#);
        let response = json_body(&post_json(port, "/work", &body), 200, "remote-owner submit");
        if required_str(&response, "owner_node") != ingress_node {
            return response;
        }
    }

    panic!("failed to find a request key that routed to a remote owner");
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
fn continuity_api_standalone_keyed_submit_status_and_retry_contract() {
    build_cluster_proof();

    let artifacts = artifact_dir("continuity-api-standalone");
    let http_port = unused_http_port();
    let spawned = spawn_cluster_proof(
        ClusterProofConfig {
            node_basename: "standalone".to_string(),
            advertise_host: LOOPBACK_V4.to_string(),
            cluster_port: None,
            http_port,
            work_delay_ms: None,
        },
        &artifacts,
    );

    let result = std::panic::catch_unwind(|| {
        wait_for_membership(http_port, "", &[]);

        let create = json_body(
            &post_json(
                http_port,
                "/work",
                r#"{"request_key":"m042-s01-key","payload":"hello"}"#,
            ),
            200,
            "initial keyed submit",
        );
        let attempt_id = required_str(&create, "attempt_id");
        assert_eq!(required_str(&create, "request_key"), "m042-s01-key");
        assert_eq!(required_str(&create, "phase"), "submitted");
        assert_eq!(required_str(&create, "result"), "pending");
        assert_eq!(required_str(&create, "ingress_node"), STANDALONE_NODE);
        assert_eq!(required_str(&create, "owner_node"), STANDALONE_NODE);
        assert_eq!(required_str(&create, "replica_node"), "");
        assert_eq!(required_str(&create, "replica_status"), "unassigned");
        assert_eq!(required_str(&create, "execution_node"), "");
        assert!(required_bool(&create, "ok"));
        assert!(!required_bool(&create, "routed_remotely"));
        assert!(required_bool(&create, "fell_back_locally"));

        let completed =
            wait_for_completed_status(http_port, "m042-s01-key", Duration::from_secs(10));
        assert_eq!(required_str(&completed, "attempt_id"), attempt_id);
        assert_eq!(required_str(&completed, "phase"), "completed");
        assert_eq!(required_str(&completed, "result"), "succeeded");
        assert_eq!(required_str(&completed, "execution_node"), STANDALONE_NODE);
        assert!(required_bool(&completed, "ok"));

        let duplicate = json_body(
            &post_json(
                http_port,
                "/work",
                r#"{"request_key":"m042-s01-key","payload":"hello"}"#,
            ),
            200,
            "same-key same-payload retry",
        );
        assert_eq!(required_str(&duplicate, "attempt_id"), attempt_id);
        assert_eq!(required_str(&duplicate, "phase"), "completed");
        assert_eq!(required_str(&duplicate, "result"), "succeeded");
        assert!(required_bool(&duplicate, "ok"));
        assert_eq!(required_str(&duplicate, "conflict_reason"), "");

        let conflict = json_body(
            &post_json(
                http_port,
                "/work",
                r#"{"request_key":"m042-s01-key","payload":"different"}"#,
            ),
            409,
            "same-key conflicting retry",
        );
        assert_eq!(required_str(&conflict, "attempt_id"), attempt_id);
        assert_eq!(required_str(&conflict, "phase"), "completed");
        assert_eq!(required_str(&conflict, "result"), "succeeded");
        assert_eq!(
            required_str(&conflict, "conflict_reason"),
            "request_key_conflict"
        );
        assert!(!required_bool(&conflict, "ok"));

        let missing = json_body(
            &get_json(http_port, "/work/missing-key"),
            404,
            "missing status",
        );
        assert_eq!(required_str(&missing, "request_key"), "missing-key");
        assert_eq!(required_str(&missing, "phase"), "missing");
        assert_eq!(required_str(&missing, "result"), "unknown");
        assert_eq!(required_str(&missing, "error"), "request_key_not_found");
        assert!(!required_bool(&missing, "ok"));
    });

    let logs = stop_cluster_proof(spawned);
    if let Err(payload) = result {
        panic!(
            "standalone continuity_api failed: {}\nartifacts: {}\nstdout:\n{}\nstderr:\n{}",
            panic_payload_to_string(payload),
            artifacts.display(),
            logs.stdout,
            logs.stderr
        );
    }

    assert_log_contains(&logs, "continuity=runtime-native");
    assert_log_contains(
        &logs,
        "[cluster-proof] keyed dispatch request_key=m042-s01-key",
    );
    assert_log_contains(
        &logs,
        "[cluster-proof] work executed request_key=m042-s01-key",
    );
}

#[test]
fn continuity_api_two_node_cluster_syncs_status_between_ingress_and_owner() {
    build_cluster_proof();

    let artifacts = artifact_dir("continuity-api-two-node");
    let cluster_port = dual_stack_cluster_port();
    let ingress_config = ClusterProofConfig {
        node_basename: "node-a".to_string(),
        advertise_host: LOOPBACK_V4.to_string(),
        cluster_port: Some(cluster_port),
        http_port: unused_http_port(),
        work_delay_ms: Some(200),
    };
    let owner_config = ClusterProofConfig {
        node_basename: "node-b".to_string(),
        advertise_host: LOOPBACK_V6.to_string(),
        cluster_port: Some(cluster_port),
        http_port: unused_http_port(),
        work_delay_ms: Some(200),
    };
    let ingress_node = expected_node_name(&ingress_config);
    let owner_node_name = expected_node_name(&owner_config);

    let spawned_ingress = spawn_cluster_proof(ingress_config.clone(), &artifacts);
    let spawned_owner = spawn_cluster_proof(owner_config.clone(), &artifacts);

    let result = std::panic::catch_unwind(|| {
        wait_for_membership(
            ingress_config.http_port,
            &ingress_node,
            &[ingress_node.clone(), owner_node_name.clone()],
        );
        wait_for_membership(
            owner_config.http_port,
            &owner_node_name,
            &[ingress_node.clone(), owner_node_name.clone()],
        );

        let submitted = wait_for_remote_owner_submit(ingress_config.http_port, &ingress_node);
        let request_key = required_str(&submitted, "request_key");
        let attempt_id = required_str(&submitted, "attempt_id");
        let owner_node = required_str(&submitted, "owner_node");
        let replica_node = required_str(&submitted, "replica_node");
        assert_ne!(owner_node, ingress_node, "expected a remote owner");
        assert_eq!(owner_node, owner_node_name);
        assert_eq!(replica_node, ingress_node);
        assert_eq!(required_str(&submitted, "phase"), "submitted");
        assert_eq!(required_str(&submitted, "result"), "pending");
        assert_eq!(required_str(&submitted, "replica_status"), "mirrored");
        assert!(required_bool(&submitted, "routed_remotely"));
        assert!(!required_bool(&submitted, "fell_back_locally"));
        assert!(required_bool(&submitted, "ok"));

        let completed_ingress = wait_for_completed_status(
            ingress_config.http_port,
            &request_key,
            Duration::from_secs(10),
        );
        let completed_owner = wait_for_completed_status(
            owner_config.http_port,
            &request_key,
            Duration::from_secs(10),
        );

        for status in [&completed_ingress, &completed_owner] {
            assert_eq!(required_str(status, "attempt_id"), attempt_id);
            assert_eq!(required_str(status, "phase"), "completed");
            assert_eq!(required_str(status, "result"), "succeeded");
            assert_eq!(required_str(status, "owner_node"), owner_node_name);
            assert_eq!(required_str(status, "replica_node"), ingress_node);
            assert_eq!(required_str(status, "replica_status"), "mirrored");
            assert_eq!(required_str(status, "execution_node"), owner_node_name);
            assert!(required_bool(status, "ok"));
        }

        let duplicate_body = format!(r#"{{"request_key":"{}","payload":"hello-0"}}"#, request_key);
        let duplicate = json_body(
            &post_json(ingress_config.http_port, "/work", &duplicate_body),
            200,
            "cluster duplicate submit",
        );
        assert_eq!(required_str(&duplicate, "attempt_id"), attempt_id);
        assert_eq!(required_str(&duplicate, "phase"), "completed");
        assert_eq!(required_str(&duplicate, "replica_status"), "mirrored");
        assert!(required_bool(&duplicate, "ok"));

        let conflict_body = format!(
            r#"{{"request_key":"{}","payload":"different"}}"#,
            request_key
        );
        let conflict = json_body(
            &post_json(ingress_config.http_port, "/work", &conflict_body),
            409,
            "cluster conflicting submit",
        );
        assert_eq!(required_str(&conflict, "attempt_id"), attempt_id);
        assert_eq!(
            required_str(&conflict, "conflict_reason"),
            "request_key_conflict"
        );
        assert!(!required_bool(&conflict, "ok"));
    });

    let ingress_logs = stop_cluster_proof(spawned_ingress);
    let owner_logs = stop_cluster_proof(spawned_owner);
    if let Err(payload) = result {
        panic!(
            "two-node continuity_api failed: {}\nartifacts: {}\ningress stdout:\n{}\ningress stderr:\n{}\nowner stdout:\n{}\nowner stderr:\n{}",
            panic_payload_to_string(payload),
            artifacts.display(),
            ingress_logs.stdout,
            ingress_logs.stderr,
            owner_logs.stdout,
            owner_logs.stderr
        );
    }

    assert_log_contains(&ingress_logs, "continuity=runtime-native");
    assert_log_contains(&owner_logs, "continuity=runtime-native");
    assert_log_contains(
        &owner_logs,
        "[cluster-proof] work executed request_key=m042-s01-remote-key-0",
    );
}
