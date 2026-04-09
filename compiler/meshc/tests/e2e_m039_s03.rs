use std::any::Any;
use std::fs::{self, File};
use std::io::{Read as _, Write as _};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde_json::Value;

mod support;
use support::m046_route_free as route_free;

const LOOPBACK_V4: &str = "127.0.0.1";
const LOOPBACK_V6: &str = "::1";
const DISCOVERY_SEED: &str = "localhost";
const SHARED_COOKIE: &str = "mesh-m039-s03-cookie";
const MEMBERSHIP_TIMEOUT: Duration = Duration::from_secs(20);
const WORK_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Clone, Debug)]
struct ClusterProofConfig {
    node_basename: String,
    advertise_host: String,
    cluster_port: u16,
    http_port: u16,
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

#[derive(Clone, Debug)]
struct MembershipSnapshot {
    self_name: String,
    peers: Vec<String>,
    membership: Vec<String>,
    raw_body: String,
}

struct HttpResponse {
    status_code: u16,
    body: String,
    raw: String,
}

#[derive(Clone, Debug)]
struct WorkSnapshot {
    ok: bool,
    request_id: String,
    ingress_node: String,
    target_node: String,
    execution_node: String,
    routed_remotely: bool,
    fell_back_locally: bool,
    timed_out: bool,
    error: String,
    raw_body: String,
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
    route_free::cluster_proof_fixture_root().join("cluster-proof")
}

fn assert_cluster_proof_build_succeeds() {
    let fixture_root = route_free::cluster_proof_fixture_root();
    let output = Command::new(meshc_bin())
        .current_dir(repo_root())
        .arg("build")
        .arg(fixture_root.to_str().unwrap())
        .output()
        .expect("failed to invoke meshc build cluster-proof fixture");

    assert_command_success(
        &output,
        "meshc build scripts/fixtures/clustered/cluster-proof",
    );
}

fn assert_cluster_proof_tests_pass() {
    let fixture_tests = route_free::cluster_proof_fixture_root().join("tests");
    let output = Command::new(meshc_bin())
        .current_dir(repo_root())
        .arg("test")
        .arg(fixture_tests.to_str().unwrap())
        .output()
        .expect("failed to invoke meshc test cluster-proof fixture tests");

    assert_command_success(
        &output,
        "meshc test scripts/fixtures/clustered/cluster-proof/tests",
    );
}

fn assert_command_success(output: &Output, description: &str) {
    assert!(
        output.status.success(),
        "{description} failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn dual_stack_cluster_port() -> u16 {
    for _ in 0..64 {
        let listener = TcpListener::bind((LOOPBACK_V4, 0))
            .expect("failed to bind IPv4 loopback for ephemeral cluster port");
        let port = listener
            .local_addr()
            .expect("failed to read IPv4 ephemeral port")
            .port();
        drop(listener);

        if TcpListener::bind((LOOPBACK_V4, port)).is_ok()
            && TcpListener::bind((LOOPBACK_V6, port)).is_ok()
        {
            return port;
        }
    }

    panic!("failed to find a cluster port that is free on both 127.0.0.1 and ::1");
}

fn unused_http_port() -> u16 {
    TcpListener::bind((LOOPBACK_V4, 0))
        .expect("failed to bind IPv4 loopback for ephemeral HTTP port")
        .local_addr()
        .expect("failed to read IPv4 ephemeral HTTP port")
        .port()
}

fn proof_logs_dir(test_name: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_nanos();
    let dir = repo_root()
        .join(".tmp")
        .join("m039-s03")
        .join(format!("{test_name}-{stamp}"));
    fs::create_dir_all(&dir)
        .unwrap_or_else(|e| panic!("failed to create {}: {}", dir.display(), e));
    dir
}

fn write_artifact(path: &Path, contents: &str) {
    fs::write(path, contents)
        .unwrap_or_else(|e| panic!("failed to write {}: {}", path.display(), e));
}

fn node_log_paths(log_dir: &Path, node_basename: &str, incarnation: usize) -> (PathBuf, PathBuf) {
    let stdout_path = log_dir.join(format!("{node_basename}-run{incarnation}.stdout.log"));
    let stderr_path = log_dir.join(format!("{node_basename}-run{incarnation}.stderr.log"));
    (stdout_path, stderr_path)
}

fn spawn_cluster_proof(
    config: ClusterProofConfig,
    log_dir: &Path,
    incarnation: usize,
) -> SpawnedClusterProof {
    let binary = cluster_proof_binary();
    assert!(
        binary.exists(),
        "cluster-proof binary not found at {}. Run `meshc build scripts/fixtures/clustered/cluster-proof` first.",
        binary.display()
    );

    let (stdout_path, stderr_path) = node_log_paths(log_dir, &config.node_basename, incarnation);
    let stdout_file = File::create(&stdout_path)
        .unwrap_or_else(|e| panic!("failed to create {}: {}", stdout_path.display(), e));
    let stderr_file = File::create(&stderr_path)
        .unwrap_or_else(|e| panic!("failed to create {}: {}", stderr_path.display(), e));

    let child = Command::new(&binary)
        .current_dir(route_free::cluster_proof_fixture_root())
        .env("PORT", config.http_port.to_string())
        .env("MESH_CLUSTER_PORT", config.cluster_port.to_string())
        .env("CLUSTER_PROOF_COOKIE", SHARED_COOKIE)
        .env("MESH_DISCOVERY_SEED", DISCOVERY_SEED)
        .env("CLUSTER_PROOF_NODE_BASENAME", &config.node_basename)
        .env("CLUSTER_PROOF_ADVERTISE_HOST", &config.advertise_host)
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file))
        .spawn()
        .unwrap_or_else(|e| panic!("failed to spawn {}: {}", binary.display(), e));

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
    std::thread::sleep(Duration::from_millis(250));
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

    let _ = child.kill();

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

fn send_http_request(port: u16, path: &str) -> std::io::Result<HttpResponse> {
    let mut stream = TcpStream::connect((LOOPBACK_V4, port))?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;

    let request = format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
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

fn parse_membership_snapshot(
    response: HttpResponse,
    description: &str,
    artifact_path: &Path,
) -> MembershipSnapshot {
    assert!(
        response.status_code == 200,
        "expected HTTP 200 for {description}, got raw response (artifact={}):\n{}",
        artifact_path.display(),
        response.raw
    );

    let json: Value = serde_json::from_str(&response.body).unwrap_or_else(|e| {
        panic!(
            "expected JSON body for {description}, got parse error {e} (artifact={}): {}",
            artifact_path.display(),
            response.body
        )
    });

    let self_name = required_str_field(&json, &response.body, "self", description, artifact_path);
    let peers = required_string_list(&json, &response.body, "peers", description, artifact_path);
    let membership = required_string_list(
        &json,
        &response.body,
        "membership",
        description,
        artifact_path,
    );

    for node_name in membership
        .iter()
        .chain(peers.iter())
        .chain(std::iter::once(&self_name))
    {
        if !node_name.is_empty() {
            assert!(
                node_name.contains('@'),
                "{description} returned malformed node identity {:?} (artifact={}): {}",
                node_name,
                artifact_path.display(),
                response.body
            );
        }
    }

    MembershipSnapshot {
        self_name,
        peers,
        membership,
        raw_body: response.body,
    }
}

fn parse_work_snapshot(
    response: HttpResponse,
    description: &str,
    artifact_path: &Path,
) -> WorkSnapshot {
    assert!(
        response.status_code == 200,
        "expected HTTP 200 for {description}, got raw response (artifact={}):\n{}",
        artifact_path.display(),
        response.raw
    );

    let json: Value = serde_json::from_str(&response.body).unwrap_or_else(|e| {
        panic!(
            "expected JSON body for {description}, got parse error {e} (artifact={}): {}",
            artifact_path.display(),
            response.body
        )
    });

    WorkSnapshot {
        ok: required_bool_field(&json, &response.body, "ok", description, artifact_path),
        request_id: required_str_field(
            &json,
            &response.body,
            "request_id",
            description,
            artifact_path,
        ),
        ingress_node: required_str_field(
            &json,
            &response.body,
            "ingress_node",
            description,
            artifact_path,
        ),
        target_node: required_str_field(
            &json,
            &response.body,
            "target_node",
            description,
            artifact_path,
        ),
        execution_node: required_str_field(
            &json,
            &response.body,
            "execution_node",
            description,
            artifact_path,
        ),
        routed_remotely: required_bool_field(
            &json,
            &response.body,
            "routed_remotely",
            description,
            artifact_path,
        ),
        fell_back_locally: required_bool_field(
            &json,
            &response.body,
            "fell_back_locally",
            description,
            artifact_path,
        ),
        timed_out: required_bool_field(
            &json,
            &response.body,
            "timed_out",
            description,
            artifact_path,
        ),
        error: required_str_field(&json, &response.body, "error", description, artifact_path),
        raw_body: response.body,
    }
}

fn required_str_field(
    json: &Value,
    raw_body: &str,
    field: &str,
    description: &str,
    artifact_path: &Path,
) -> String {
    json[field]
        .as_str()
        .unwrap_or_else(|| {
            panic!(
                "{description} missing string field `{field}` (artifact={}): {}",
                artifact_path.display(),
                raw_body
            )
        })
        .to_string()
}

fn required_bool_field(
    json: &Value,
    raw_body: &str,
    field: &str,
    description: &str,
    artifact_path: &Path,
) -> bool {
    json[field].as_bool().unwrap_or_else(|| {
        panic!(
            "{description} missing bool field `{field}` (artifact={}): {}",
            artifact_path.display(),
            raw_body
        )
    })
}

fn required_string_list(
    json: &Value,
    raw_body: &str,
    field: &str,
    description: &str,
    artifact_path: &Path,
) -> Vec<String> {
    let values = json[field].as_array().unwrap_or_else(|| {
        panic!(
            "{description} missing array field `{field}` (artifact={}): {}",
            artifact_path.display(),
            raw_body
        )
    });

    values
        .iter()
        .map(|value| {
            value
                .as_str()
                .unwrap_or_else(|| {
                    panic!(
                        "{description} field `{field}` must contain only strings (artifact={}): {}",
                        artifact_path.display(),
                        raw_body
                    )
                })
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
    let normalized_host = if config.advertise_host.contains(':') {
        format!("[{}]", config.advertise_host)
    } else {
        config.advertise_host.clone()
    };
    format!(
        "{}@{}:{}",
        config.node_basename, normalized_host, config.cluster_port
    )
}

fn wait_for_membership(
    config: &ClusterProofConfig,
    watched_nodes: &mut [&mut SpawnedClusterProof],
    expected_self: &str,
    expected_membership: &[String],
    expected_peers: &[String],
    description: &str,
    artifact_path: &Path,
) -> MembershipSnapshot {
    let deadline = Instant::now() + MEMBERSHIP_TIMEOUT;
    let mut last_snapshot: Option<MembershipSnapshot> = None;
    let mut last_issue = String::new();

    while Instant::now() < deadline {
        for spawned in watched_nodes.iter_mut() {
            assert_cluster_proof_running(
                spawned,
                &format!("waiting for {description} on :{}", config.http_port),
            );
        }

        match send_http_request(config.http_port, "/membership") {
            Ok(response) if response.status_code == 200 => {
                write_artifact(artifact_path, &response.body);
                let snapshot = parse_membership_snapshot(response, description, artifact_path);

                last_issue = format!(
                    "last snapshot self={} peers={:?} membership={:?}",
                    snapshot.self_name, snapshot.peers, snapshot.membership
                );

                if snapshot.self_name == expected_self
                    && sorted(&snapshot.membership) == sorted(expected_membership)
                    && sorted(&snapshot.peers) == sorted(expected_peers)
                {
                    return snapshot;
                }

                last_snapshot = Some(snapshot);
            }
            Ok(response) => {
                write_artifact(artifact_path, &response.raw);
                last_issue = format!(
                    "unexpected HTTP {} from {description}: {}",
                    response.status_code, response.raw
                );
            }
            Err(error) => {
                let error_text = format!("socket_error: {error}");
                write_artifact(artifact_path, &error_text);
                last_issue = format!("GET /membership failed for {description}: {error}");
            }
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    panic!(
        "membership did not converge for {} while {} within timeout; artifact={}; {}; last_snapshot={:?}",
        config.node_basename,
        description,
        artifact_path.display(),
        last_issue,
        last_snapshot
    );
}

fn wait_for_work_response(
    port: u16,
    watched_nodes: &mut [&mut SpawnedClusterProof],
    description: &str,
    artifact_path: &Path,
) -> WorkSnapshot {
    let deadline = Instant::now() + WORK_TIMEOUT;
    let mut last_issue = String::new();

    while Instant::now() < deadline {
        for spawned in watched_nodes.iter_mut() {
            assert_cluster_proof_running(spawned, &format!("waiting for {description} on :{port}"));
        }

        match send_http_request(port, "/work") {
            Ok(response) if response.status_code == 200 => {
                write_artifact(artifact_path, &response.body);
                return parse_work_snapshot(response, description, artifact_path);
            }
            Ok(response) => {
                write_artifact(artifact_path, &response.raw);
                last_issue = format!(
                    "unexpected HTTP {} from {description}: {}",
                    response.status_code, response.raw
                );
            }
            Err(error) => {
                let error_text = format!("socket_error: {error}");
                write_artifact(artifact_path, &error_text);
                last_issue = format!("GET /work failed for {description}: {error}");
            }
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    panic!(
        "timed out waiting for {description}; artifact={}; last_issue={last_issue}",
        artifact_path.display()
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

fn request_token_from_id(request_id: &str) -> u64 {
    let token = request_id
        .strip_prefix("work-")
        .unwrap_or_else(|| panic!("request_id must start with `work-`, got {request_id:?}"));

    token.parse::<u64>().unwrap_or_else(|error| {
        panic!("request_id must end with an integer token, got {request_id:?}: {error}")
    })
}

fn assert_dispatch_log_contains(
    logs: &StoppedClusterProof,
    request_id: &str,
    ingress_node: &str,
    target_node: &str,
    routed_remotely: bool,
) {
    let expected = format!(
        "[cluster-proof] work dispatched request_id={} ingress={} target={} routed_remotely={}",
        request_id, ingress_node, target_node, routed_remotely
    );
    assert!(
        logs.combined.contains(&expected),
        "expected dispatch log `{}` in run{} {} / {}\nstdout:\n{}\nstderr:\n{}",
        expected,
        logs.incarnation,
        logs.stdout_path.display(),
        logs.stderr_path.display(),
        logs.stdout,
        logs.stderr
    );
}

fn assert_execution_log_contains(logs: &StoppedClusterProof, execution_node: &str) {
    let expected = format!("[cluster-proof] work executed execution={}", execution_node);
    assert!(
        logs.combined.contains(&expected),
        "expected execution log `{}` in run{} {} / {}\nstdout:\n{}\nstderr:\n{}",
        expected,
        logs.incarnation,
        logs.stdout_path.display(),
        logs.stderr_path.display(),
        logs.stdout,
        logs.stderr
    );
}

fn assert_remote_work_snapshot(
    response: &WorkSnapshot,
    expected_ingress: &str,
    expected_peer: &str,
    phase: &str,
) {
    assert!(
        response.ok,
        "{phase} response must be ok: {}",
        response.raw_body
    );
    assert_eq!(
        response.error, "",
        "{phase} response must not report an error"
    );
    assert!(
        !response.timed_out,
        "{phase} response must not report timeout"
    );
    assert!(
        response.routed_remotely,
        "{phase} response must report remote routing"
    );
    assert!(
        !response.fell_back_locally,
        "{phase} response must not claim local fallback: {}",
        response.raw_body
    );
    assert_eq!(
        response.ingress_node, expected_ingress,
        "{phase} ingress node mismatch"
    );
    assert_eq!(
        response.target_node, expected_peer,
        "{phase} target node mismatch"
    );
    assert_eq!(
        response.execution_node, expected_peer,
        "{phase} execution node must match the peer"
    );
}

fn assert_local_work_snapshot(response: &WorkSnapshot, expected_self: &str, phase: &str) {
    assert!(
        response.ok,
        "{phase} response must be ok: {}",
        response.raw_body
    );
    assert_eq!(
        response.error, "",
        "{phase} response must not report an error"
    );
    assert!(
        !response.timed_out,
        "{phase} response must not report timeout"
    );
    assert!(
        !response.routed_remotely,
        "{phase} response must not claim remote routing: {}",
        response.raw_body
    );
    assert!(
        response.fell_back_locally,
        "{phase} response must report local fallback: {}",
        response.raw_body
    );
    assert_eq!(
        response.ingress_node, expected_self,
        "{phase} ingress node mismatch"
    );
    assert_eq!(
        response.target_node, expected_self,
        "{phase} target node mismatch"
    );
    assert_eq!(
        response.execution_node, expected_self,
        "{phase} execution node must match the survivor"
    );
}

#[test]
fn e2e_m039_s03_degrades_safely_and_serves_locally_after_peer_loss() {
    assert_cluster_proof_tests_pass();
    assert_cluster_proof_build_succeeds();

    let cluster_port = dual_stack_cluster_port();
    let config_a = ClusterProofConfig {
        node_basename: "node-a".to_string(),
        advertise_host: LOOPBACK_V4.to_string(),
        cluster_port,
        http_port: unused_http_port(),
    };
    let config_b = ClusterProofConfig {
        node_basename: "node-b".to_string(),
        advertise_host: LOOPBACK_V6.to_string(),
        cluster_port,
        http_port: unused_http_port(),
    };

    let expected_a = expected_node_name(&config_a);
    let expected_b = expected_node_name(&config_b);
    let artifact_dir = proof_logs_dir("e2e-m039-s03-degrade");
    let pre_loss_membership_a = artifact_dir.join("pre-loss-node-a-membership.json");
    let pre_loss_membership_b = artifact_dir.join("pre-loss-node-b-membership.json");
    let pre_loss_work = artifact_dir.join("pre-loss-work.json");
    let degraded_membership = artifact_dir.join("degraded-node-a-membership.json");
    let degraded_work = artifact_dir.join("degraded-work.json");

    let mut spawned_a = Some(spawn_cluster_proof(config_a.clone(), &artifact_dir, 1));
    let mut spawned_b_run1 = Some(spawn_cluster_proof(config_b.clone(), &artifact_dir, 1));
    let mut killed_b_run1_logs: Option<StoppedClusterProof> = None;

    let run_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        {
            let a = spawned_a
                .as_mut()
                .expect("node-a process missing before convergence");
            let b = spawned_b_run1
                .as_mut()
                .expect("node-b run1 missing before convergence");
            let mut watched = [a, b];
            let snapshot = wait_for_membership(
                &config_a,
                &mut watched,
                &expected_a,
                &[expected_a.clone(), expected_b.clone()],
                std::slice::from_ref(&expected_b),
                "pre-loss membership on node-a",
                &pre_loss_membership_a,
            );
            assert!(
                snapshot.membership.contains(&expected_b),
                "pre-loss node-a membership must include node-b: {}",
                snapshot.raw_body
            );
        }
        {
            let a = spawned_a
                .as_mut()
                .expect("node-a process missing before convergence");
            let b = spawned_b_run1
                .as_mut()
                .expect("node-b run1 missing before convergence");
            let mut watched = [a, b];
            let snapshot = wait_for_membership(
                &config_b,
                &mut watched,
                &expected_b,
                &[expected_a.clone(), expected_b.clone()],
                std::slice::from_ref(&expected_a),
                "pre-loss membership on node-b",
                &pre_loss_membership_b,
            );
            assert!(
                snapshot.membership.contains(&expected_a),
                "pre-loss node-b membership must include node-a: {}",
                snapshot.raw_body
            );
        }

        let pre_loss_response = {
            let a = spawned_a
                .as_mut()
                .expect("node-a process missing before pre-loss work");
            let b = spawned_b_run1
                .as_mut()
                .expect("node-b run1 missing before pre-loss work");
            let mut watched = [a, b];
            wait_for_work_response(
                config_a.http_port,
                &mut watched,
                "pre-loss /work on node-a",
                &pre_loss_work,
            )
        };
        assert_remote_work_snapshot(&pre_loss_response, &expected_a, &expected_b, "pre-loss");
        assert_eq!(request_token_from_id(&pre_loss_response.request_id), 0);

        let stopped_b_run1 = kill_cluster_proof(
            spawned_b_run1
                .take()
                .expect("node-b run1 missing before loss injection"),
        );
        killed_b_run1_logs = Some(stopped_b_run1);

        let degraded_membership_snapshot = {
            let a = spawned_a
                .as_mut()
                .expect("node-a process missing after peer loss");
            let mut watched = [a];
            wait_for_membership(
                &config_a,
                &mut watched,
                &expected_a,
                std::slice::from_ref(&expected_a),
                &[],
                "degraded membership on node-a",
                &degraded_membership,
            )
        };
        assert_eq!(
            degraded_membership_snapshot.membership,
            vec![expected_a.clone()],
            "survivor membership must shrink to self only after peer loss; response={}",
            degraded_membership_snapshot.raw_body
        );
        assert!(
            degraded_membership_snapshot.peers.is_empty(),
            "survivor peers must be empty after peer loss; response={}",
            degraded_membership_snapshot.raw_body
        );

        let degraded_response = {
            let a = spawned_a
                .as_mut()
                .expect("node-a process missing during degraded /work");
            let mut watched = [a];
            wait_for_work_response(
                config_a.http_port,
                &mut watched,
                "degraded /work on node-a",
                &degraded_work,
            )
        };
        assert_local_work_snapshot(&degraded_response, &expected_a, "degraded");
        assert_eq!(request_token_from_id(&degraded_response.request_id), 1);
        assert_ne!(pre_loss_response.request_id, degraded_response.request_id);

        (pre_loss_response, degraded_response)
    }));

    let logs_a = stop_cluster_proof(
        spawned_a
            .take()
            .expect("node-a process missing during cleanup"),
    );
    let logs_b_run1 = match killed_b_run1_logs {
        Some(logs) => logs,
        None => stop_cluster_proof(
            spawned_b_run1
                .take()
                .expect("node-b run1 missing during cleanup"),
        ),
    };

    match run_result {
        Ok((pre_loss_response, degraded_response)) => {
            assert_dispatch_log_contains(
                &logs_a,
                &pre_loss_response.request_id,
                &expected_a,
                &expected_b,
                true,
            );
            assert_dispatch_log_contains(
                &logs_a,
                &degraded_response.request_id,
                &expected_a,
                &expected_a,
                false,
            );
            assert_execution_log_contains(&logs_b_run1, &expected_b);
            assert_execution_log_contains(&logs_a, &expected_a);
        }
        Err(payload) => {
            panic!(
                "S03 degrade assertions failed: {}\nartifact_dir: {}\npre_loss_membership_a: {}\npre_loss_membership_b: {}\npre_loss_work: {}\ndegraded_membership: {}\ndegraded_work: {}\nnode-a stdout ({}):\n{}\nnode-a stderr ({}):\n{}\nnode-b run1 stdout ({}):\n{}\nnode-b run1 stderr ({}):\n{}",
                panic_payload_to_string(payload),
                artifact_dir.display(),
                pre_loss_membership_a.display(),
                pre_loss_membership_b.display(),
                pre_loss_work.display(),
                degraded_membership.display(),
                degraded_work.display(),
                logs_a.stdout_path.display(),
                logs_a.stdout,
                logs_a.stderr_path.display(),
                logs_a.stderr,
                logs_b_run1.stdout_path.display(),
                logs_b_run1.stdout,
                logs_b_run1.stderr_path.display(),
                logs_b_run1.stderr
            );
        }
    }
}

#[test]
fn e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair() {
    assert_cluster_proof_tests_pass();
    assert_cluster_proof_build_succeeds();

    let cluster_port = dual_stack_cluster_port();
    let config_a = ClusterProofConfig {
        node_basename: "node-a".to_string(),
        advertise_host: LOOPBACK_V4.to_string(),
        cluster_port,
        http_port: unused_http_port(),
    };
    let config_b = ClusterProofConfig {
        node_basename: "node-b".to_string(),
        advertise_host: LOOPBACK_V6.to_string(),
        cluster_port,
        http_port: unused_http_port(),
    };

    let expected_a = expected_node_name(&config_a);
    let expected_b = expected_node_name(&config_b);
    let artifact_dir = proof_logs_dir("e2e-m039-s03-rejoin");
    let pre_loss_membership_a = artifact_dir.join("pre-loss-node-a-membership.json");
    let pre_loss_membership_b = artifact_dir.join("pre-loss-node-b-membership.json");
    let pre_loss_work = artifact_dir.join("pre-loss-work.json");
    let degraded_membership = artifact_dir.join("degraded-node-a-membership.json");
    let degraded_work = artifact_dir.join("degraded-work.json");
    let post_rejoin_membership_a = artifact_dir.join("post-rejoin-node-a-membership.json");
    let post_rejoin_membership_b = artifact_dir.join("post-rejoin-node-b-membership.json");
    let post_rejoin_work = artifact_dir.join("post-rejoin-work.json");

    let mut spawned_a = Some(spawn_cluster_proof(config_a.clone(), &artifact_dir, 1));
    let mut spawned_b_run1 = Some(spawn_cluster_proof(config_b.clone(), &artifact_dir, 1));
    let mut killed_b_run1_logs: Option<StoppedClusterProof> = None;
    let mut spawned_b_run2: Option<SpawnedClusterProof> = None;

    let run_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        {
            let a = spawned_a
                .as_mut()
                .expect("node-a process missing before convergence");
            let b = spawned_b_run1
                .as_mut()
                .expect("node-b run1 missing before convergence");
            let mut watched = [a, b];
            let snapshot = wait_for_membership(
                &config_a,
                &mut watched,
                &expected_a,
                &[expected_a.clone(), expected_b.clone()],
                std::slice::from_ref(&expected_b),
                "pre-loss membership on node-a",
                &pre_loss_membership_a,
            );
            assert!(
                snapshot.membership.contains(&expected_b),
                "pre-loss node-a membership must include node-b: {}",
                snapshot.raw_body
            );
        }
        {
            let a = spawned_a
                .as_mut()
                .expect("node-a process missing before convergence");
            let b = spawned_b_run1
                .as_mut()
                .expect("node-b run1 missing before convergence");
            let mut watched = [a, b];
            let snapshot = wait_for_membership(
                &config_b,
                &mut watched,
                &expected_b,
                &[expected_a.clone(), expected_b.clone()],
                std::slice::from_ref(&expected_a),
                "pre-loss membership on node-b",
                &pre_loss_membership_b,
            );
            assert!(
                snapshot.membership.contains(&expected_a),
                "pre-loss node-b membership must include node-a: {}",
                snapshot.raw_body
            );
        }

        let pre_loss_response = {
            let a = spawned_a
                .as_mut()
                .expect("node-a process missing before pre-loss work");
            let b = spawned_b_run1
                .as_mut()
                .expect("node-b run1 missing before pre-loss work");
            let mut watched = [a, b];
            wait_for_work_response(
                config_a.http_port,
                &mut watched,
                "pre-loss /work on node-a",
                &pre_loss_work,
            )
        };
        assert_remote_work_snapshot(&pre_loss_response, &expected_a, &expected_b, "pre-loss");
        assert_eq!(request_token_from_id(&pre_loss_response.request_id), 0);

        let stopped_b_run1 = kill_cluster_proof(
            spawned_b_run1
                .take()
                .expect("node-b run1 missing before loss injection"),
        );
        killed_b_run1_logs = Some(stopped_b_run1);

        let degraded_membership_snapshot = {
            let a = spawned_a
                .as_mut()
                .expect("node-a process missing after peer loss");
            let mut watched = [a];
            wait_for_membership(
                &config_a,
                &mut watched,
                &expected_a,
                std::slice::from_ref(&expected_a),
                &[],
                "degraded membership on node-a",
                &degraded_membership,
            )
        };
        assert_eq!(
            degraded_membership_snapshot.membership,
            vec![expected_a.clone()],
            "survivor membership must shrink to self only after peer loss; response={}",
            degraded_membership_snapshot.raw_body
        );
        assert!(
            degraded_membership_snapshot.peers.is_empty(),
            "survivor peers must be empty after peer loss; response={}",
            degraded_membership_snapshot.raw_body
        );

        let degraded_response = {
            let a = spawned_a
                .as_mut()
                .expect("node-a process missing during degraded /work");
            let mut watched = [a];
            wait_for_work_response(
                config_a.http_port,
                &mut watched,
                "degraded /work on node-a",
                &degraded_work,
            )
        };
        assert_local_work_snapshot(&degraded_response, &expected_a, "degraded");
        assert_eq!(request_token_from_id(&degraded_response.request_id), 1);
        assert_ne!(pre_loss_response.request_id, degraded_response.request_id);

        spawned_b_run2 = Some(spawn_cluster_proof(config_b.clone(), &artifact_dir, 2));

        {
            let a = spawned_a
                .as_mut()
                .expect("node-a process missing before rejoin convergence");
            let b = spawned_b_run2
                .as_mut()
                .expect("node-b run2 missing before rejoin convergence");
            let mut watched = [a, b];
            let snapshot = wait_for_membership(
                &config_a,
                &mut watched,
                &expected_a,
                &[expected_a.clone(), expected_b.clone()],
                std::slice::from_ref(&expected_b),
                "post-rejoin membership on node-a",
                &post_rejoin_membership_a,
            );
            assert!(
                snapshot.membership.contains(&expected_b),
                "post-rejoin node-a membership must include node-b again: {}",
                snapshot.raw_body
            );
        }
        {
            let a = spawned_a
                .as_mut()
                .expect("node-a process missing before rejoin convergence");
            let b = spawned_b_run2
                .as_mut()
                .expect("node-b run2 missing before rejoin convergence");
            let mut watched = [a, b];
            let snapshot = wait_for_membership(
                &config_b,
                &mut watched,
                &expected_b,
                &[expected_a.clone(), expected_b.clone()],
                std::slice::from_ref(&expected_a),
                "post-rejoin membership on node-b",
                &post_rejoin_membership_b,
            );
            assert!(
                snapshot.membership.contains(&expected_a),
                "post-rejoin node-b membership must include node-a again: {}",
                snapshot.raw_body
            );
        }

        let post_rejoin_response = {
            let a = spawned_a
                .as_mut()
                .expect("node-a process missing before post-rejoin /work");
            let b = spawned_b_run2
                .as_mut()
                .expect("node-b run2 missing before post-rejoin /work");
            let mut watched = [a, b];
            wait_for_work_response(
                config_a.http_port,
                &mut watched,
                "post-rejoin /work on node-a",
                &post_rejoin_work,
            )
        };
        assert_remote_work_snapshot(
            &post_rejoin_response,
            &expected_a,
            &expected_b,
            "post-rejoin",
        );
        assert_eq!(request_token_from_id(&post_rejoin_response.request_id), 2);
        assert_ne!(
            pre_loss_response.request_id,
            post_rejoin_response.request_id
        );
        assert_ne!(
            degraded_response.request_id,
            post_rejoin_response.request_id
        );

        (pre_loss_response, degraded_response, post_rejoin_response)
    }));

    let logs_a = stop_cluster_proof(
        spawned_a
            .take()
            .expect("node-a process missing during cleanup"),
    );
    let logs_b_run2 = spawned_b_run2.take().map(stop_cluster_proof);
    let logs_b_run1 = match killed_b_run1_logs {
        Some(logs) => logs,
        None => stop_cluster_proof(
            spawned_b_run1
                .take()
                .expect("node-b run1 missing during cleanup"),
        ),
    };

    match run_result {
        Ok((pre_loss_response, degraded_response, post_rejoin_response)) => {
            let logs_b_run2 = logs_b_run2
                .as_ref()
                .expect("node-b run2 logs missing after successful rejoin proof");
            assert_dispatch_log_contains(
                &logs_a,
                &pre_loss_response.request_id,
                &expected_a,
                &expected_b,
                true,
            );
            assert_dispatch_log_contains(
                &logs_a,
                &degraded_response.request_id,
                &expected_a,
                &expected_a,
                false,
            );
            assert_dispatch_log_contains(
                &logs_a,
                &post_rejoin_response.request_id,
                &expected_a,
                &expected_b,
                true,
            );
            assert_execution_log_contains(&logs_b_run1, &expected_b);
            assert_execution_log_contains(&logs_a, &expected_a);
            assert_execution_log_contains(logs_b_run2, &expected_b);
        }
        Err(payload) => {
            let run2_stdout = logs_b_run2
                .as_ref()
                .map(|logs| logs.stdout_path.display().to_string())
                .unwrap_or_else(|| "<node-b run2 not started>".to_string());
            let run2_stderr = logs_b_run2
                .as_ref()
                .map(|logs| logs.stderr_path.display().to_string())
                .unwrap_or_else(|| "<node-b run2 not started>".to_string());
            let run2_stdout_body = logs_b_run2
                .as_ref()
                .map(|logs| logs.stdout.as_str())
                .unwrap_or("");
            let run2_stderr_body = logs_b_run2
                .as_ref()
                .map(|logs| logs.stderr.as_str())
                .unwrap_or("");

            panic!(
                "S03 rejoin assertions failed: {}\nartifact_dir: {}\npre_loss_membership_a: {}\npre_loss_membership_b: {}\npre_loss_work: {}\ndegraded_membership: {}\ndegraded_work: {}\npost_rejoin_membership_a: {}\npost_rejoin_membership_b: {}\npost_rejoin_work: {}\nnode-a stdout ({}):\n{}\nnode-a stderr ({}):\n{}\nnode-b run1 stdout ({}):\n{}\nnode-b run1 stderr ({}):\n{}\nnode-b run2 stdout ({}):\n{}\nnode-b run2 stderr ({}):\n{}",
                panic_payload_to_string(payload),
                artifact_dir.display(),
                pre_loss_membership_a.display(),
                pre_loss_membership_b.display(),
                pre_loss_work.display(),
                degraded_membership.display(),
                degraded_work.display(),
                post_rejoin_membership_a.display(),
                post_rejoin_membership_b.display(),
                post_rejoin_work.display(),
                logs_a.stdout_path.display(),
                logs_a.stdout,
                logs_a.stderr_path.display(),
                logs_a.stderr,
                logs_b_run1.stdout_path.display(),
                logs_b_run1.stdout,
                logs_b_run1.stderr_path.display(),
                logs_b_run1.stderr,
                run2_stdout,
                run2_stdout_body,
                run2_stderr,
                run2_stderr_body
            );
        }
    }
}
