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
const SHARED_COOKIE: &str = "mesh-m039-s02-cookie";

#[derive(Clone, Debug)]
struct ClusterProofConfig {
    node_basename: String,
    advertise_host: String,
    cluster_port: u16,
    http_port: u16,
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
        .join("m039-s02")
        .join(format!("{test_name}-{stamp}"));
    fs::create_dir_all(&dir)
        .unwrap_or_else(|e| panic!("failed to create {}: {}", dir.display(), e));
    dir
}

fn write_artifact(path: &Path, contents: &str) {
    fs::write(path, contents)
        .unwrap_or_else(|e| panic!("failed to write {}: {}", path.display(), e));
}

fn node_log_paths(log_dir: &Path, node_basename: &str) -> (PathBuf, PathBuf) {
    let stdout_path = log_dir.join(format!("{node_basename}.stdout.log"));
    let stderr_path = log_dir.join(format!("{node_basename}.stderr.log"));
    (stdout_path, stderr_path)
}

fn spawn_cluster_proof(config: ClusterProofConfig, log_dir: &Path) -> SpawnedClusterProof {
    let binary = cluster_proof_binary();
    assert!(
        binary.exists(),
        "cluster-proof binary not found at {}. Run `meshc build scripts/fixtures/clustered/cluster-proof` first.",
        binary.display()
    );

    let (stdout_path, stderr_path) = node_log_paths(log_dir, &config.node_basename);
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
        child,
        stdout_path,
        stderr_path,
    }
}

fn collect_stopped_cluster_proof(
    mut child: Child,
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
        stdout,
        stderr,
        combined,
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
    std::thread::sleep(Duration::from_millis(250));
    if child
        .try_wait()
        .expect("failed to probe cluster-proof exit status")
        .is_none()
    {
        let _ = child.kill();
    }

    collect_stopped_cluster_proof(child, stdout_path, stderr_path)
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

fn parse_membership_snapshot(response: HttpResponse, description: &str) -> MembershipSnapshot {
    assert!(
        response.status_code == 200,
        "expected HTTP 200 for {description}, got raw response:\n{}",
        response.raw
    );

    let json: Value = serde_json::from_str(&response.body).unwrap_or_else(|e| {
        panic!(
            "expected JSON body for {description}, got parse error {e}: {}",
            response.body
        )
    });

    MembershipSnapshot {
        self_name: required_str_field(&json, &response.body, "self"),
        peers: required_string_list(&json, &response.body, "peers"),
        membership: required_string_list(&json, &response.body, "membership"),
        raw_body: response.body,
    }
}

fn parse_work_snapshot(response: HttpResponse, description: &str) -> WorkSnapshot {
    assert!(
        response.status_code == 200,
        "expected HTTP 200 for {description}, got raw response:\n{}",
        response.raw
    );

    let json: Value = serde_json::from_str(&response.body).unwrap_or_else(|e| {
        panic!(
            "expected JSON body for {description}, got parse error {e}: {}",
            response.body
        )
    });

    WorkSnapshot {
        ok: required_bool_field(&json, &response.body, "ok"),
        request_id: required_str_field(&json, &response.body, "request_id"),
        ingress_node: required_str_field(&json, &response.body, "ingress_node"),
        target_node: required_str_field(&json, &response.body, "target_node"),
        execution_node: required_str_field(&json, &response.body, "execution_node"),
        routed_remotely: required_bool_field(&json, &response.body, "routed_remotely"),
        fell_back_locally: required_bool_field(&json, &response.body, "fell_back_locally"),
        timed_out: required_bool_field(&json, &response.body, "timed_out"),
        error: required_str_field(&json, &response.body, "error"),
        raw_body: response.body,
    }
}

fn required_str_field(json: &Value, raw_body: &str, field: &str) -> String {
    json[field]
        .as_str()
        .unwrap_or_else(|| panic!("response missing string field `{field}`: {}", raw_body))
        .to_string()
}

fn required_bool_field(json: &Value, raw_body: &str, field: &str) -> bool {
    json[field]
        .as_bool()
        .unwrap_or_else(|| panic!("response missing bool field `{field}`: {}", raw_body))
}

fn required_string_list(json: &Value, raw_body: &str, field: &str) -> Vec<String> {
    let values = json[field]
        .as_array()
        .unwrap_or_else(|| panic!("response missing array field `{field}`: {}", raw_body));

    values
        .iter()
        .map(|value| {
            value
                .as_str()
                .unwrap_or_else(|| {
                    panic!(
                        "response field `{field}` must contain only strings: {}",
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
) -> MembershipSnapshot {
    let deadline = Instant::now() + Duration::from_secs(12);
    let mut last_snapshot: Option<MembershipSnapshot> = None;
    let mut last_issue = String::new();

    while Instant::now() < deadline {
        for spawned in watched_nodes.iter_mut() {
            assert_cluster_proof_running(
                spawned,
                &format!("waiting for /membership on :{}", config.http_port),
            );
        }

        match send_http_request(config.http_port, "/membership") {
            Ok(response) if response.status_code == 200 => {
                let snapshot = parse_membership_snapshot(
                    response,
                    &format!("/membership on :{}", config.http_port),
                );

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
                last_issue = format!(
                    "unexpected HTTP {} from /membership on :{}: {}",
                    response.status_code, config.http_port, response.raw
                );
            }
            Err(error) => {
                last_issue = format!("GET /membership failed on :{}: {}", config.http_port, error);
            }
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    panic!(
        "membership did not converge for {} on :{} within timeout; {} ; last_snapshot={:?}",
        config.node_basename, config.http_port, last_issue, last_snapshot
    );
}

fn wait_for_work_response(port: u16, description: &str, artifact_path: &Path) -> WorkSnapshot {
    let deadline = Instant::now() + Duration::from_secs(8);
    let mut last_issue = String::new();

    while Instant::now() < deadline {
        match send_http_request(port, "/work") {
            Ok(response) if response.status_code == 200 => {
                write_artifact(artifact_path, &response.body);
                return parse_work_snapshot(response, description);
            }
            Ok(response) => {
                write_artifact(artifact_path, &response.raw);
                last_issue = format!(
                    "unexpected HTTP {} from {description}: {}",
                    response.status_code, response.raw
                );
            }
            Err(error) => {
                last_issue = format!("GET /work failed for {description}: {error}");
            }
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    panic!("timed out waiting for {description}; last_issue={last_issue}");
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
        "expected dispatch log `{}` in {} / {}\nstdout:\n{}\nstderr:\n{}",
        expected,
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
        "expected execution log `{}` in {} / {}\nstdout:\n{}\nstderr:\n{}",
        expected,
        logs.stdout_path.display(),
        logs.stderr_path.display(),
        logs.stdout,
        logs.stderr
    );
}

fn request_token_from_id(request_id: &str) -> u64 {
    let token = request_id
        .strip_prefix("work-")
        .unwrap_or_else(|| panic!("request_id must start with `work-`, got {request_id:?}"));

    token.parse::<u64>().unwrap_or_else(|error| {
        panic!("request_id must end with an integer token, got {request_id:?}: {error}")
    })
}

fn run_remote_route_proof(
    test_name: &str,
    ingress_config: ClusterProofConfig,
    peer_config: ClusterProofConfig,
) -> WorkSnapshot {
    let expected_ingress = expected_node_name(&ingress_config);
    let expected_peer = expected_node_name(&peer_config);
    let log_dir = proof_logs_dir(test_name);
    let response_artifact = log_dir.join(format!("{}-work.json", ingress_config.node_basename));

    let mut spawned_ingress = spawn_cluster_proof(ingress_config.clone(), &log_dir);
    let mut spawned_peer = spawn_cluster_proof(peer_config.clone(), &log_dir);

    let run_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        {
            let mut watched = [&mut spawned_ingress, &mut spawned_peer];
            let snapshot = wait_for_membership(
                &ingress_config,
                &mut watched,
                &expected_ingress,
                &[expected_ingress.clone(), expected_peer.clone()],
                std::slice::from_ref(&expected_peer),
            );
            assert!(
                snapshot.membership.contains(&expected_peer),
                "peer missing from ingress membership: {}",
                snapshot.raw_body
            );
        }
        {
            let mut watched = [&mut spawned_ingress, &mut spawned_peer];
            let snapshot = wait_for_membership(
                &peer_config,
                &mut watched,
                &expected_peer,
                &[expected_ingress.clone(), expected_peer.clone()],
                std::slice::from_ref(&expected_ingress),
            );
            assert!(
                snapshot.membership.contains(&expected_ingress),
                "ingress missing from peer membership: {}",
                snapshot.raw_body
            );
        }

        let response = wait_for_work_response(
            ingress_config.http_port,
            &format!("/work on :{}", ingress_config.http_port),
            &response_artifact,
        );

        assert!(
            response.ok,
            "route response must be ok: {}",
            response.raw_body
        );
        assert_eq!(
            response.error, "",
            "route response must not report an error"
        );
        assert!(
            !response.timed_out,
            "route response must not report timeout"
        );
        assert!(
            response.routed_remotely,
            "route response must report remote routing"
        );
        assert!(
            !response.fell_back_locally,
            "remote route must not claim local fallback: {}",
            response.raw_body
        );
        assert_eq!(
            response.ingress_node, expected_ingress,
            "ingress node must match the contacted port"
        );
        assert_eq!(
            response.target_node, expected_peer,
            "target node must be the peer node"
        );
        assert_eq!(
            response.execution_node, expected_peer,
            "execution node must match the peer node"
        );
        assert_eq!(
            request_token_from_id(&response.request_id),
            0,
            "fresh remote-route proof should emit the first request token"
        );

        response
    }));

    let peer_logs = stop_cluster_proof(spawned_peer);
    let ingress_logs = stop_cluster_proof(spawned_ingress);

    match run_result {
        Ok(response) => {
            assert_dispatch_log_contains(
                &ingress_logs,
                &response.request_id,
                &expected_ingress,
                &expected_peer,
                true,
            );
            assert_execution_log_contains(&peer_logs, &expected_peer);
            response
        }
        Err(payload) => {
            panic!(
                "remote route assertions failed: {}\ningress stdout ({}):\n{}\ningress stderr ({}):\n{}\npeer stdout ({}):\n{}\npeer stderr ({}):\n{}\nresponse_artifact: {}",
                panic_payload_to_string(payload),
                ingress_logs.stdout_path.display(),
                ingress_logs.stdout,
                ingress_logs.stderr_path.display(),
                ingress_logs.stderr,
                peer_logs.stdout_path.display(),
                peer_logs.stdout,
                peer_logs.stderr_path.display(),
                peer_logs.stderr,
                response_artifact.display()
            );
        }
    }
}

#[test]
fn e2e_m039_s02_routes_work_to_peer_and_logs_execution() {
    assert_cluster_proof_tests_pass();
    assert_cluster_proof_build_succeeds();

    let cluster_port_a = dual_stack_cluster_port();
    let ingress_a = ClusterProofConfig {
        node_basename: "node-a".to_string(),
        advertise_host: LOOPBACK_V4.to_string(),
        cluster_port: cluster_port_a,
        http_port: unused_http_port(),
    };
    let peer_b = ClusterProofConfig {
        node_basename: "node-b".to_string(),
        advertise_host: LOOPBACK_V6.to_string(),
        cluster_port: cluster_port_a,
        http_port: unused_http_port(),
    };

    let response_a =
        run_remote_route_proof("e2e-m039-s02-ingress-a", ingress_a.clone(), peer_b.clone());
    assert_eq!(response_a.ingress_node, expected_node_name(&ingress_a));
    assert_eq!(response_a.execution_node, expected_node_name(&peer_b));
}

#[test]
fn e2e_m039_s02_falls_back_locally_without_peers() {
    assert_cluster_proof_tests_pass();
    assert_cluster_proof_build_succeeds();

    let config = ClusterProofConfig {
        node_basename: "node-solo".to_string(),
        advertise_host: LOOPBACK_V4.to_string(),
        cluster_port: unused_http_port(),
        http_port: unused_http_port(),
    };
    let expected_self = expected_node_name(&config);
    let log_dir = proof_logs_dir("e2e-m039-s02-solo");
    let response_artifact = log_dir.join("node-solo-work.json");
    let mut spawned = spawn_cluster_proof(config.clone(), &log_dir);

    let run_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        {
            let mut watched = [&mut spawned];
            let snapshot = wait_for_membership(
                &config,
                &mut watched,
                &expected_self,
                std::slice::from_ref(&expected_self),
                &[],
            );
            assert!(
                snapshot.peers.is_empty(),
                "single-node cluster must report zero peers: {}",
                snapshot.raw_body
            );
        }

        let response = wait_for_work_response(
            config.http_port,
            &format!("/work on :{}", config.http_port),
            &response_artifact,
        );

        assert!(
            response.ok,
            "fallback response must be ok: {}",
            response.raw_body
        );
        assert_eq!(
            response.error, "",
            "fallback response must not report an error"
        );
        assert!(
            !response.timed_out,
            "fallback response must not report timeout"
        );
        assert!(
            !response.routed_remotely,
            "single-node route must not claim remote routing"
        );
        assert!(
            response.fell_back_locally,
            "single-node route must report local fallback"
        );
        assert_eq!(response.ingress_node, expected_self);
        assert_eq!(response.target_node, expected_self);
        assert_eq!(response.execution_node, expected_self);
        assert_eq!(
            request_token_from_id(&response.request_id),
            0,
            "fresh single-node proof should emit the first request token"
        );

        response
    }));

    let logs = stop_cluster_proof(spawned);

    match run_result {
        Ok(response) => {
            assert_dispatch_log_contains(
                &logs,
                &response.request_id,
                &expected_self,
                &expected_self,
                false,
            );
            assert_execution_log_contains(&logs, &expected_self);
        }
        Err(payload) => {
            panic!(
                "single-node fallback assertions failed: {}\nstdout ({}):\n{}\nstderr ({}):\n{}\nresponse_artifact: {}",
                panic_payload_to_string(payload),
                logs.stdout_path.display(),
                logs.stdout,
                logs.stderr_path.display(),
                logs.stderr,
                response_artifact.display()
            );
        }
    }
}
