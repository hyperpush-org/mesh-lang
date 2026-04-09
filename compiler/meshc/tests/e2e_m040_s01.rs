use std::fs::{self, File};
use std::io::{Read as _, Write as _};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde_json::Value;

const LOOPBACK_V4: &str = "127.0.0.1";
const LOCAL_NODE: &str = "standalone@local";

struct HttpResponse {
    status_code: u16,
    body: String,
    raw: String,
}

struct SpawnedClusterProof {
    child: Child,
    stdout_path: PathBuf,
    stderr_path: PathBuf,
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
    let output = Command::new(meshc_bin())
        .current_dir(repo_root())
        .args(["build", "cluster-proof"])
        .output()
        .expect("failed to invoke meshc build cluster-proof");
    assert_command_success(&output, "meshc build cluster-proof");
}

fn unused_http_port() -> u16 {
    TcpListener::bind((LOOPBACK_V4, 0))
        .expect("failed to bind ephemeral HTTP port")
        .local_addr()
        .expect("failed to read ephemeral HTTP port")
        .port()
}

fn artifact_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let dir = repo_root()
        .join(".tmp")
        .join("m040-s01")
        .join("e2e")
        .join(format!("run-{stamp}"));
    fs::create_dir_all(&dir).expect("failed to create e2e artifact dir");
    dir
}

fn spawn_cluster_proof(http_port: u16, artifacts: &Path) -> SpawnedClusterProof {
    let binary = cluster_proof_binary();
    assert!(
        binary.exists(),
        "cluster-proof binary not found at {}. Run `meshc build cluster-proof` first.",
        binary.display()
    );

    let stdout_path = artifacts.join("cluster-proof.stdout.log");
    let stderr_path = artifacts.join("cluster-proof.stderr.log");
    let stdout = File::create(&stdout_path).expect("failed to create stdout log");
    let stderr = File::create(&stderr_path).expect("failed to create stderr log");

    let child = Command::new(binary)
        .current_dir(repo_root().join("cluster-proof"))
        .env("PORT", http_port.to_string())
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .spawn()
        .expect("failed to start cluster-proof");

    SpawnedClusterProof {
        child,
        stdout_path,
        stderr_path,
    }
}

fn stop_cluster_proof(mut spawned: SpawnedClusterProof) -> (String, String) {
    let _ = spawned.child.kill();
    let _ = spawned.child.wait();

    let stdout = fs::read_to_string(&spawned.stdout_path).unwrap_or_default();
    let stderr = fs::read_to_string(&spawned.stderr_path).unwrap_or_default();
    (stdout, stderr)
}

fn send_request(port: u16, request: &str) -> std::io::Result<HttpResponse> {
    let mut stream = TcpStream::connect((LOOPBACK_V4, port))?;
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
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

fn get_json(port: u16, path: &str) -> HttpResponse {
    let request = format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
    send_request(port, &request).expect("GET request failed")
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

fn wait_for_membership(port: u16, timeout: Duration) {
    let start = Instant::now();
    let mut last_error = String::new();

    while start.elapsed() < timeout {
        match send_request(
            port,
            "GET /membership HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        ) {
            Ok(response) if response.status_code == 200 => return,
            Ok(response) => last_error = format!("unexpected HTTP {}", response.status_code),
            Err(error) => last_error = error.to_string(),
        }
        sleep(Duration::from_millis(100));
    }

    panic!("cluster-proof did not become ready within timeout: {last_error}");
}

fn wait_for_completed_status(port: u16, request_key: &str, timeout: Duration) -> Value {
    let start = Instant::now();
    let path = format!("/work/{request_key}");
    let mut last_body = String::new();

    while start.elapsed() < timeout {
        let response = get_json(port, &path);
        if response.status_code == 200 {
            let json: Value = serde_json::from_str(&response.body).unwrap_or_else(|error| {
                panic!("status response was not JSON: {error}: {}", response.body)
            });
            last_body = response.body.clone();
            if required_str(&json, "phase") == "completed" {
                return json;
            }
        } else {
            last_body = response.raw;
        }
        sleep(Duration::from_millis(100));
    }

    panic!("request {request_key} never reached completed state; last body: {last_body}");
}

#[test]
fn e2e_m040_s01_keyed_submit_status_and_retry_contract() {
    build_cluster_proof();

    let artifacts = artifact_dir();
    let http_port = unused_http_port();
    let spawned = spawn_cluster_proof(http_port, &artifacts);

    let result = std::panic::catch_unwind(|| {
        wait_for_membership(http_port, Duration::from_secs(10));

        let create = json_body(
            &post_json(
                http_port,
                "/work",
                r#"{"request_key":"m040-s01-key","payload":"hello"}"#,
            ),
            200,
            "initial keyed submit",
        );
        let attempt_id = required_str(&create, "attempt_id");
        assert_eq!(required_str(&create, "request_key"), "m040-s01-key");
        assert_eq!(required_str(&create, "phase"), "submitted");
        assert_eq!(required_str(&create, "result"), "pending");
        assert_eq!(required_str(&create, "ingress_node"), LOCAL_NODE);
        assert_eq!(required_str(&create, "owner_node"), LOCAL_NODE);
        assert_eq!(required_str(&create, "replica_node"), "");
        assert_eq!(required_str(&create, "replica_status"), "unassigned");
        assert_eq!(required_str(&create, "execution_node"), "");
        assert_eq!(create["routed_remotely"].as_bool(), Some(false));
        assert_eq!(create["fell_back_locally"].as_bool(), Some(true));
        assert_eq!(create["ok"].as_bool(), Some(true));

        let completed =
            wait_for_completed_status(http_port, "m040-s01-key", Duration::from_secs(10));
        assert_eq!(required_str(&completed, "attempt_id"), attempt_id);
        assert_eq!(required_str(&completed, "phase"), "completed");
        assert_eq!(required_str(&completed, "result"), "succeeded");
        assert_eq!(required_str(&completed, "execution_node"), LOCAL_NODE);
        assert_eq!(completed["ok"].as_bool(), Some(true));

        let duplicate = json_body(
            &post_json(
                http_port,
                "/work",
                r#"{"request_key":"m040-s01-key","payload":"hello"}"#,
            ),
            200,
            "same-key same-payload retry",
        );
        assert_eq!(required_str(&duplicate, "attempt_id"), attempt_id);
        assert_eq!(required_str(&duplicate, "phase"), "completed");
        assert_eq!(required_str(&duplicate, "result"), "succeeded");
        assert_eq!(duplicate["ok"].as_bool(), Some(true));
        assert_eq!(required_str(&duplicate, "conflict_reason"), "");

        let conflict = json_body(
            &post_json(
                http_port,
                "/work",
                r#"{"request_key":"m040-s01-key","payload":"different"}"#,
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
        assert_eq!(conflict["ok"].as_bool(), Some(false));

        let missing = json_body(
            &get_json(http_port, "/work/missing-key"),
            404,
            "missing status",
        );
        assert_eq!(required_str(&missing, "request_key"), "missing-key");
        assert_eq!(required_str(&missing, "phase"), "missing");
        assert_eq!(required_str(&missing, "result"), "unknown");
        assert_eq!(required_str(&missing, "error"), "request_key_not_found");
        assert_eq!(missing["ok"].as_bool(), Some(false));
    });

    let (stdout, stderr) = stop_cluster_proof(spawned);
    if let Err(payload) = result {
        let message = if let Some(text) = payload.downcast_ref::<String>() {
            text.clone()
        } else if let Some(text) = payload.downcast_ref::<&str>() {
            text.to_string()
        } else {
            "non-string panic payload".to_string()
        };
        panic!(
            "M040/S01 e2e failed: {message}\nartifacts: {}\nstdout:\n{}\nstderr:\n{}",
            artifacts.display(),
            stdout,
            stderr
        );
    }
}
