use std::any::Any;
use std::fs::{self, File};
use std::io::{Read as _, Write as _};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Output, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const LOOPBACK_V4: &str = "127.0.0.1";
const CONTAINER_CLUSTER_PORT: u16 = 4370;
const DISCOVERY_SEED: &str = "cluster-proof-seed";
const SHARED_COOKIE: &str = "mesh-m043-s03-cookie";
const IMAGE_TAG: &str = "mesh-cluster-proof:m043-s03-local";
const DOCKER_BUILD_TIMEOUT: Duration = Duration::from_secs(1_800);
const DOCKER_PHASE_TIMEOUT: Duration = Duration::from_secs(45);
const MEMBERSHIP_TIMEOUT: Duration = Duration::from_secs(45);
const STATUS_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Clone, Debug)]
struct DockerNodeConfig {
    artifact_label: String,
    container_name: String,
    hostname: String,
    cluster_role: String,
    promotion_epoch: u64,
    work_delay_ms: u64,
}

struct DockerStartedNode {
    config: DockerNodeConfig,
    attach_child: Child,
    host_port: u16,
    stdout_path: PathBuf,
    stderr_path: PathBuf,
}

struct StoppedDockerNode {
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

fn artifact_dir(test_name: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = repo_root()
        .join(".tmp")
        .join("m043-s03")
        .join(format!("{test_name}-{stamp}"));
    fs::create_dir_all(&dir).expect("failed to create e2e artifact dir");
    dir
}

fn write_artifact(path: &Path, contents: impl AsRef<str>) {
    fs::write(path, contents.as_ref())
        .unwrap_or_else(|error| panic!("failed to write artifact {}: {error}", path.display()));
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

fn run_command_with_timeout(
    command: &mut Command,
    log_path: &Path,
    timeout: Duration,
    description: &str,
) -> ExitStatus {
    let mut log = File::create(log_path)
        .unwrap_or_else(|error| panic!("failed to create {}: {error}", log_path.display()));
    writeln!(log, "description: {description}")
        .unwrap_or_else(|error| panic!("failed to write {}: {error}", log_path.display()));
    let stderr_log = log
        .try_clone()
        .unwrap_or_else(|error| panic!("failed to clone {}: {error}", log_path.display()));

    command.stdout(Stdio::from(log));
    command.stderr(Stdio::from(stderr_log));

    let mut child = command
        .spawn()
        .unwrap_or_else(|error| panic!("failed to spawn {description}: {error}"));

    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return status,
            Ok(None) => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    panic!(
                        "{description} timed out after {:?}; partial log: {}",
                        timeout,
                        log_path.display()
                    );
                }
                sleep(Duration::from_millis(250));
            }
            Err(error) => panic!("failed to wait on {description}: {error}"),
        }
    }
}

fn docker_output(args: &[&str], description: &str) -> Output {
    Command::new("docker")
        .current_dir(repo_root())
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("failed to run docker for {description}: {error}"))
}

fn assert_command_success(output: &Output, description: &str) {
    assert!(
        output.status.success(),
        "{description} failed:\n{}",
        command_output_text(output)
    );
}

fn redact_string(value: &str) -> String {
    value
        .replace(
            &format!("CLUSTER_PROOF_COOKIE={SHARED_COOKIE}"),
            "CLUSTER_PROOF_COOKIE=[REDACTED]",
        )
        .replace(SHARED_COOKIE, "[REDACTED]")
}

fn redact_json(value: &mut Value) {
    match value {
        Value::String(text) => *text = redact_string(text),
        Value::Array(items) => {
            for item in items.iter_mut() {
                redact_json(item);
            }
        }
        Value::Object(map) => {
            for item in map.values_mut() {
                redact_json(item);
            }
        }
        _ => {}
    }
}

fn write_json_artifact(path: &Path, value: &Value) {
    write_artifact(
        path,
        serde_json::to_string_pretty(value).expect("failed to pretty print JSON artifact"),
    );
}

fn docker_image_build(artifacts: &Path) {
    let build_log = artifacts.join("docker-build.log");
    let status = run_command_with_timeout(
        Command::new("docker").current_dir(repo_root()).args([
            "build",
            "--progress=plain",
            "-f",
            "cluster-proof/Dockerfile",
            "-t",
            IMAGE_TAG,
            ".",
        ]),
        &build_log,
        DOCKER_BUILD_TIMEOUT,
        "docker image build for same-image continuity harness",
    );
    assert!(
        status.success(),
        "docker build failed; inspect {}",
        build_log.display()
    );
}

fn docker_image_inspect(artifacts: &Path) -> Value {
    let output = docker_output(&["image", "inspect", IMAGE_TAG], "docker image inspect");
    assert_command_success(&output, "docker image inspect");
    let mut json: Value = serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("failed to parse docker image inspect JSON: {error}"));
    redact_json(&mut json);
    write_json_artifact(&artifacts.join("image.inspect.json"), &json);
    json.as_array()
        .and_then(|entries| entries.first())
        .cloned()
        .expect("docker image inspect returned no entries")
}

fn docker_network_create(network_name: &str, artifacts: &Path) {
    let log_path = artifacts.join("network.create.log");
    let status = run_command_with_timeout(
        Command::new("docker")
            .current_dir(repo_root())
            .args(["network", "create", network_name]),
        &log_path,
        DOCKER_PHASE_TIMEOUT,
        "docker network create for same-image continuity harness",
    );
    assert!(
        status.success(),
        "docker network create failed; inspect {}",
        log_path.display()
    );
}

fn docker_network_remove(network_name: &str, artifacts: &Path) {
    let output = docker_output(
        &["network", "rm", network_name],
        "docker network rm during cleanup",
    );
    write_artifact(
        &artifacts.join("network.rm.log"),
        command_output_text(&output),
    );
}

fn docker_network_inspect(network_name: &str, path: &Path) {
    let output = docker_output(
        &["network", "inspect", network_name],
        "docker network inspect",
    );
    assert_command_success(&output, "docker network inspect");
    let mut json: Value = serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("failed to parse docker network inspect JSON: {error}"));
    redact_json(&mut json);
    write_json_artifact(path, &json);
}

fn container_log_paths(artifacts: &Path, artifact_label: &str) -> (PathBuf, PathBuf) {
    (
        artifacts.join(format!("{artifact_label}.stdout.log")),
        artifacts.join(format!("{artifact_label}.stderr.log")),
    )
}

fn docker_create_container(config: &DockerNodeConfig, network_name: &str, artifacts: &Path) {
    let log_path = artifacts.join(format!("{}.create.log", config.artifact_label));
    let status = run_command_with_timeout(
        Command::new("docker").current_dir(repo_root()).args([
            "create",
            "--name",
            &config.container_name,
            "--hostname",
            &config.hostname,
            "--network",
            network_name,
            "--network-alias",
            &config.hostname,
            "--network-alias",
            DISCOVERY_SEED,
            "-p",
            "127.0.0.1::8080",
            "-e",
            &format!("CLUSTER_PROOF_COOKIE={SHARED_COOKIE}"),
            "-e",
            &format!("MESH_DISCOVERY_SEED={DISCOVERY_SEED}"),
            "-e",
            "CLUSTER_PROOF_DURABILITY=replica-backed",
            "-e",
            &format!("CLUSTER_PROOF_WORK_DELAY_MS={}", config.work_delay_ms),
            "-e",
            &format!("MESH_CONTINUITY_ROLE={}", config.cluster_role),
            "-e",
            &format!("MESH_CONTINUITY_PROMOTION_EPOCH={}", config.promotion_epoch),
            IMAGE_TAG,
        ]),
        &log_path,
        DOCKER_PHASE_TIMEOUT,
        &format!("docker create {}", config.container_name),
    );
    assert!(
        status.success(),
        "docker create failed for {}; inspect {}",
        config.container_name,
        log_path.display()
    );
}

fn spawn_attached_container(config: DockerNodeConfig, artifacts: &Path) -> DockerStartedNode {
    let (stdout_path, stderr_path) = container_log_paths(artifacts, &config.artifact_label);
    let stdout = File::create(&stdout_path)
        .unwrap_or_else(|error| panic!("failed to create {}: {error}", stdout_path.display()));
    let stderr = File::create(&stderr_path)
        .unwrap_or_else(|error| panic!("failed to create {}: {error}", stderr_path.display()));

    let attach_child = Command::new("docker")
        .current_dir(repo_root())
        .args(["start", "-a", &config.container_name])
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .spawn()
        .unwrap_or_else(|error| {
            panic!(
                "failed to start attached {}: {error}",
                config.container_name
            )
        });

    let host_port = wait_for_published_http_port(&config.container_name);

    DockerStartedNode {
        config,
        attach_child,
        host_port,
        stdout_path,
        stderr_path,
    }
}

fn docker_container_running(container_name: &str) -> bool {
    let output = docker_output(
        &["inspect", "-f", "{{.State.Running}}", container_name],
        "docker inspect running state",
    );
    if !output.status.success() {
        return false;
    }
    String::from_utf8_lossy(&output.stdout).trim() == "true"
}

fn wait_for_published_http_port(container_name: &str) -> u16 {
    let start = Instant::now();
    while start.elapsed() < DOCKER_PHASE_TIMEOUT {
        let output = docker_output(
            &[
                "inspect",
                "-f",
                "{{(index (index .NetworkSettings.Ports \"8080/tcp\") 0).HostPort}}",
                container_name,
            ],
            "docker inspect host port",
        );
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Ok(port) = text.parse::<u16>() {
                return port;
            }
        }
        sleep(Duration::from_millis(250));
    }

    panic!(
        "container {container_name} never published an HTTP port within {:?}",
        DOCKER_PHASE_TIMEOUT
    );
}

fn docker_container_inspect(container_name: &str, path: &Path) -> Value {
    let output = docker_output(&["inspect", container_name], "docker inspect container");
    assert_command_success(&output, "docker inspect container");
    let mut json: Value = serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("failed to parse docker inspect JSON: {error}"));
    redact_json(&mut json);
    write_json_artifact(path, &json);
    json.as_array()
        .and_then(|entries| entries.first())
        .cloned()
        .expect("docker inspect returned no entries")
}

fn docker_stop_node(node: DockerStartedNode, artifacts: &Path, verb: &str) -> StoppedDockerNode {
    let log_path = artifacts.join(format!("{}.{}.log", node.config.artifact_label, verb));
    let output = docker_output(
        &[verb, "-t", "2", &node.config.container_name],
        &format!("docker {verb} {}", node.config.container_name),
    );
    write_artifact(&log_path, command_output_text(&output));

    let mut attach_child = node.attach_child;
    let _ = attach_child.wait();

    let stdout = fs::read_to_string(&node.stdout_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", node.stdout_path.display()));
    let stderr = fs::read_to_string(&node.stderr_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", node.stderr_path.display()));
    let combined = format!("{stdout}{stderr}");

    StoppedDockerNode {
        stdout,
        stderr,
        combined,
        stdout_path: node.stdout_path,
        stderr_path: node.stderr_path,
    }
}

fn docker_kill_node(node: DockerStartedNode, artifacts: &Path) -> StoppedDockerNode {
    let log_path = artifacts.join(format!("{}.kill.log", node.config.artifact_label));
    let output = docker_output(
        &["kill", &node.config.container_name],
        &format!("docker kill {}", node.config.container_name),
    );
    write_artifact(&log_path, command_output_text(&output));

    let mut attach_child = node.attach_child;
    let _ = attach_child.wait();

    let stdout = fs::read_to_string(&node.stdout_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", node.stdout_path.display()));
    let stderr = fs::read_to_string(&node.stderr_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", node.stderr_path.display()));
    let combined = format!("{stdout}{stderr}");

    StoppedDockerNode {
        stdout,
        stderr,
        combined,
        stdout_path: node.stdout_path,
        stderr_path: node.stderr_path,
    }
}

fn docker_remove_container(container_name: &str, artifacts: &Path, label: &str) {
    let output = docker_output(&["rm", "-f", container_name], "docker rm during cleanup");
    write_artifact(
        &artifacts.join(format!("{label}.remove.log")),
        command_output_text(&output),
    );
}

fn assert_docker_node_running(node: &mut DockerStartedNode, context: &str) {
    if let Some(status) = node
        .attach_child
        .try_wait()
        .unwrap_or_else(|error| panic!("failed to probe attached docker process: {error}"))
    {
        panic!(
            "container {} exited early while {}; status={:?}; stdout_log={}; stderr_log={}",
            node.config.container_name,
            context,
            status,
            node.stdout_path.display(),
            node.stderr_path.display()
        );
    }
    assert!(
        docker_container_running(&node.config.container_name),
        "container {} is not running while {}; stdout_log={}; stderr_log={}",
        node.config.container_name,
        context,
        node.stdout_path.display(),
        node.stderr_path.display()
    );
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
            write_json_artifact(&json_path, &json);
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

fn expected_node_name(hostname: &str) -> String {
    format!("{hostname}@{hostname}:{CONTAINER_CLUSTER_PORT}")
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
    format_ok && expected.map(|value| value == actual).unwrap_or(true)
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

fn membership_truth_matches_with_allowed_healths(
    json: &Value,
    expected: &ExpectedMembershipTruth<'_>,
    allowed_healths: &[&str],
) -> bool {
    required_str(json, "mode") == "cluster"
        && required_str(json, "self") == expected.self_name
        && sorted(&required_string_list(json, "membership")) == sorted(expected.membership)
        && required_str(json, "cluster_role") == expected.cluster_role
        && required_u64(json, "promotion_epoch") == expected.promotion_epoch
        && allowed_healths.contains(&required_str(json, "replication_health").as_str())
        && required_str(json, "discovery_provider") == "dns"
        && required_str(json, "discovery_seed") == DISCOVERY_SEED
}

fn work_truth_matches_with_allowed_healths(
    json: &Value,
    expected: &ExpectedWorkTruth<'_>,
    allowed_healths: &[&str],
) -> bool {
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
        && allowed_healths.contains(&required_str(json, "replication_health").as_str())
        && required_str(json, "execution_node") == expected.execution_node
        && required_bool(json, "routed_remotely") == expected.routed_remotely
        && required_bool(json, "fell_back_locally") == expected.fell_back_locally
        && required_bool(json, "ok") == expected.ok
        && required_str(json, "error") == expected.error
        && required_str(json, "conflict_reason") == expected.conflict_reason
}

fn wait_for_membership_with_allowed_healths(
    artifacts: &Path,
    name: &str,
    port: u16,
    watched_nodes: &mut [&mut DockerStartedNode],
    expected: &ExpectedMembershipTruth<'_>,
    allowed_healths: &[&str],
) -> Value {
    const REQUIRED_STABLE_POLLS: usize = 5;

    let start = Instant::now();
    let mut last_body = String::new();
    let mut stable_polls = 0usize;

    while start.elapsed() < MEMBERSHIP_TIMEOUT {
        for node in watched_nodes.iter_mut() {
            assert_docker_node_running(
                node,
                &format!("waiting for membership convergence on :{port}"),
            );
        }

        match try_get_json(port, "/membership") {
            Ok(response) => {
                if response.status_code == 200 {
                    let json =
                        parse_json_response(artifacts, name, &response, "membership response");
                    last_body = response.body.clone();
                    if membership_truth_matches_with_allowed_healths(
                        &json,
                        expected,
                        allowed_healths,
                    ) {
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
        sleep(Duration::from_millis(250));
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

fn wait_for_work_truth_with_allowed_healths(
    artifacts: &Path,
    name: &str,
    port: u16,
    watched_nodes: &mut [&mut DockerStartedNode],
    expected: &ExpectedWorkTruth<'_>,
    allowed_healths: &[&str],
    predicate_description: &str,
) -> Value {
    let start = Instant::now();
    let path = format!("/work/{}", expected.request_key);
    let mut last_body = String::new();

    while start.elapsed() < STATUS_TIMEOUT {
        for node in watched_nodes.iter_mut() {
            assert_docker_node_running(
                node,
                &format!("waiting for {predicate_description} on :{port}"),
            );
        }

        match try_get_json(port, &path) {
            Ok(response) => {
                if response.status_code == 200 {
                    let json =
                        parse_json_response(artifacts, name, &response, predicate_description);
                    last_body = response.body.clone();
                    if work_truth_matches_with_allowed_healths(&json, expected, allowed_healths) {
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
        sleep(Duration::from_millis(250));
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

fn wait_for_membership(
    artifacts: &Path,
    name: &str,
    port: u16,
    watched_nodes: &mut [&mut DockerStartedNode],
    expected: &ExpectedMembershipTruth<'_>,
) -> Value {
    const REQUIRED_STABLE_POLLS: usize = 5;

    let start = Instant::now();
    let mut last_body = String::new();
    let mut stable_polls = 0usize;

    while start.elapsed() < MEMBERSHIP_TIMEOUT {
        for node in watched_nodes.iter_mut() {
            assert_docker_node_running(
                node,
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
        sleep(Duration::from_millis(250));
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
    watched_nodes: &mut [&mut DockerStartedNode],
    expected: &ExpectedWorkTruth<'_>,
    predicate_description: &str,
) -> Value {
    let start = Instant::now();
    let path = format!("/work/{}", expected.request_key);
    let mut last_body = String::new();

    while start.elapsed() < STATUS_TIMEOUT {
        for node in watched_nodes.iter_mut() {
            assert_docker_node_running(
                node,
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
        sleep(Duration::from_millis(250));
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

fn assert_request_key_places_owner_and_replica(
    request_key: &str,
    desired_owner: &str,
    desired_replica: &str,
) {
    assert!(
        request_key_matches_placement(request_key, desired_owner, desired_replica),
        "request key {} does not place owner={} replica={}",
        request_key,
        desired_owner,
        desired_replica
    );
}

fn find_request_key_candidate(
    prefix: &str,
    desired_owner: &str,
    desired_replica: &str,
    max_attempts: usize,
) -> (String, String) {
    for idx in 0..max_attempts {
        let request_key = format!("{prefix}-key-{idx}");
        if request_key_matches_placement(&request_key, desired_owner, desired_replica) {
            return (request_key, format!("payload-{idx}"));
        }
    }
    panic!(
        "failed to find request key candidate for owner={} replica={} within {} attempts",
        desired_owner, desired_replica, max_attempts
    );
}

fn find_submit_matching_placement(
    artifacts: &Path,
    prefix: &str,
    port: u16,
    watched_nodes: &mut [&mut DockerStartedNode],
    desired_owner: &str,
    desired_replica: &str,
    max_attempts: usize,
) -> SubmittedRequest {
    let search_root = artifacts.join(format!("{prefix}-search"));
    fs::create_dir_all(&search_root).expect("failed to create placement search artifact root");

    let mut rejected: Vec<String> = Vec::new();
    for idx in 0..max_attempts {
        for node in watched_nodes.iter_mut() {
            assert_docker_node_running(
                node,
                &format!("searching placement candidate {idx} for {prefix}"),
            );
        }

        let request_key = format!("{prefix}-key-{idx}");
        let payload = format!("payload-{idx}");
        if !request_key_matches_placement(&request_key, desired_owner, desired_replica) {
            rejected.push(request_key);
            continue;
        }

        write_json_artifact(
            &search_root.join("chosen.json"),
            &json!({
                "request_key": request_key,
                "payload": payload,
                "owner_node": desired_owner,
                "replica_node": desired_replica,
                "rejected_candidates": rejected,
            }),
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

fn assert_log_contains(logs: &StoppedDockerNode, needle: &str) {
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

fn assert_log_absent(logs: &StoppedDockerNode, needle: &str) {
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

fn inspect_env_values(inspect: &Value) -> Vec<String> {
    inspect["Config"]["Env"]
        .as_array()
        .unwrap_or_else(|| panic!("docker inspect missing Config.Env in {inspect}"))
        .iter()
        .map(|value| {
            value
                .as_str()
                .unwrap_or_else(|| panic!("non-string env entry in docker inspect: {inspect}"))
                .to_string()
        })
        .collect()
}

fn assert_inspect_env_contains(inspect: &Value, expected: &str) {
    let envs = inspect_env_values(inspect);
    assert!(
        envs.iter().any(|value| value == expected),
        "docker inspect env missing {expected:?}: {envs:?}"
    );
}

fn assert_inspect_env_absent(inspect: &Value, prefix: &str) {
    let envs = inspect_env_values(inspect);
    assert!(
        envs.iter().all(|value| !value.starts_with(prefix)),
        "docker inspect env unexpectedly contains prefix {prefix:?}: {envs:?}"
    );
}

fn inspect_image_id(inspect: &Value) -> String {
    required_str(inspect, "Image")
}

fn assert_container_identity_contract(
    inspect: &Value,
    expected_hostname: &str,
    expected_role: &str,
    expected_epoch: u64,
) {
    assert_eq!(
        required_str(&inspect["Config"], "Hostname"),
        expected_hostname,
        "docker hostname drifted"
    );
    assert_inspect_env_contains(inspect, &format!("MESH_CONTINUITY_ROLE={expected_role}"));
    assert_inspect_env_contains(
        inspect,
        &format!("MESH_CONTINUITY_PROMOTION_EPOCH={expected_epoch}"),
    );
    assert_inspect_env_contains(inspect, &format!("MESH_DISCOVERY_SEED={DISCOVERY_SEED}"));
    assert_inspect_env_contains(inspect, "CLUSTER_PROOF_DURABILITY=replica-backed");
    assert_inspect_env_contains(inspect, "CLUSTER_PROOF_COOKIE=[REDACTED]");
    assert_inspect_env_absent(inspect, "CLUSTER_PROOF_NODE_BASENAME=");
    assert_inspect_env_absent(inspect, "CLUSTER_PROOF_ADVERTISE_HOST=");
}

#[test]
fn e2e_m043_s03_request_key_without_primary_owner_and_standby_replica_is_rejected() {
    let primary_node = expected_node_name("primary");
    let standby_node = expected_node_name("standby");
    let rejected_key = (0..256)
        .map(|idx| format!("same-image-invalid-key-{idx}"))
        .find(|candidate| !request_key_matches_placement(candidate, &primary_node, &standby_node))
        .expect("expected at least one rejected placement candidate");

    let result = std::panic::catch_unwind(|| {
        assert_request_key_places_owner_and_replica(&rejected_key, &primary_node, &standby_node)
    });
    assert!(
        result.is_err(),
        "non-matching request key should be rejected by the harness"
    );

    let (chosen_key, payload) =
        find_request_key_candidate("same-image-valid", &primary_node, &standby_node, 256);
    assert_request_key_places_owner_and_replica(&chosen_key, &primary_node, &standby_node);
    assert!(payload.starts_with("payload-"));
}

#[test]
fn e2e_m043_s03_malformed_or_incomplete_http_responses_fail_closed() {
    let artifacts = artifact_dir("continuity-api-same-image-failover-malformed");

    let malformed = HttpResponse {
        status_code: 200,
        raw: "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\nnot-json".to_string(),
        body: "not-json".to_string(),
    };
    let malformed_result = std::panic::catch_unwind(|| {
        parse_json_response(
            &artifacts,
            "membership-malformed",
            &malformed,
            "malformed same-image membership response",
        )
    });
    assert!(
        malformed_result.is_err(),
        "malformed JSON should fail closed"
    );
    assert!(
        artifacts.join("membership-malformed.http").is_file(),
        "raw malformed response should be retained"
    );
    assert!(
        artifacts.join("membership-malformed.body.txt").is_file(),
        "malformed body should be retained"
    );

    let incomplete_membership = json!({
        "mode": "cluster",
        "self": expected_node_name("primary"),
        "membership": [expected_node_name("primary"), expected_node_name("standby")],
        "discovery_provider": "dns",
        "discovery_seed": DISCOVERY_SEED,
    });
    let membership_result = std::panic::catch_unwind(|| {
        assert_membership_truth(
            &incomplete_membership,
            &ExpectedMembershipTruth {
                self_name: &expected_node_name("primary"),
                membership: &[expected_node_name("primary"), expected_node_name("standby")],
                cluster_role: "primary",
                promotion_epoch: 0,
                replication_health: "healthy",
            },
        )
    });
    assert!(
        membership_result.is_err(),
        "missing authority fields should fail membership assertions"
    );

    let incomplete_work = json!({
        "ok": true,
        "request_key": "req-1",
        "attempt_id": "attempt-1",
        "phase": "submitted",
        "result": "pending",
        "ingress_node": expected_node_name("primary"),
        "owner_node": expected_node_name("primary"),
        "replica_node": expected_node_name("standby"),
        "replica_status": "mirrored",
        "execution_node": "",
        "routed_remotely": false,
        "fell_back_locally": true,
        "error": "",
        "conflict_reason": "",
    });
    let work_result = std::panic::catch_unwind(|| {
        assert_work_truth(
            &incomplete_work,
            &ExpectedWorkTruth {
                request_key: "req-1",
                attempt_id: Some("attempt-1"),
                phase: "submitted",
                result: "pending",
                ingress_node: &expected_node_name("primary"),
                owner_node: &expected_node_name("primary"),
                replica_node: &expected_node_name("standby"),
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
        )
    });
    assert!(
        work_result.is_err(),
        "missing authority fields should fail work assertions"
    );
}

#[test]
fn e2e_m043_s03_same_image_failover_fences_stale_primary() {
    let artifacts = artifact_dir("continuity-api-same-image-failover");
    let run_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let network_name = format!("m043-s03-net-{run_id}");

    let primary_run1_config = DockerNodeConfig {
        artifact_label: "primary-run1".to_string(),
        container_name: format!("m043-s03-primary-run1-{run_id}"),
        hostname: "primary".to_string(),
        cluster_role: "primary".to_string(),
        promotion_epoch: 0,
        work_delay_ms: 5_000,
    };
    let standby_run1_config = DockerNodeConfig {
        artifact_label: "standby-run1".to_string(),
        container_name: format!("m043-s03-standby-run1-{run_id}"),
        hostname: "standby".to_string(),
        cluster_role: "standby".to_string(),
        promotion_epoch: 0,
        work_delay_ms: 1_200,
    };
    let primary_run2_config = DockerNodeConfig {
        artifact_label: "primary-run2".to_string(),
        container_name: format!("m043-s03-primary-run2-{run_id}"),
        hostname: "primary".to_string(),
        cluster_role: "primary".to_string(),
        promotion_epoch: 0,
        work_delay_ms: 1_200,
    };

    let primary_node = expected_node_name(&primary_run1_config.hostname);
    let standby_node = expected_node_name(&standby_run1_config.hostname);
    let membership = vec![primary_node.clone(), standby_node.clone()];

    docker_image_build(&artifacts);
    let image_inspect = docker_image_inspect(&artifacts);
    docker_network_create(&network_name, &artifacts);
    docker_network_inspect(&network_name, &artifacts.join("network.inspect.json"));

    docker_create_container(&primary_run1_config, &network_name, &artifacts);
    docker_create_container(&standby_run1_config, &network_name, &artifacts);

    let mut primary_run1 = Some(spawn_attached_container(
        primary_run1_config.clone(),
        &artifacts,
    ));
    let mut standby_run1 = Some(spawn_attached_container(
        standby_run1_config.clone(),
        &artifacts,
    ));
    let mut primary_run2: Option<DockerStartedNode> = None;

    let primary_run1_inspect = docker_container_inspect(
        &primary_run1_config.container_name,
        &artifacts.join("primary-run1.inspect.json"),
    );
    let standby_run1_inspect = docker_container_inspect(
        &standby_run1_config.container_name,
        &artifacts.join("standby-run1.inspect.json"),
    );

    assert_container_identity_contract(
        &primary_run1_inspect,
        &primary_run1_config.hostname,
        "primary",
        0,
    );
    assert_container_identity_contract(
        &standby_run1_inspect,
        &standby_run1_config.hostname,
        "standby",
        0,
    );
    let image_id = required_str(&image_inspect, "Id");
    assert_eq!(inspect_image_id(&primary_run1_inspect), image_id);
    assert_eq!(inspect_image_id(&standby_run1_inspect), image_id);

    let mut selected_request_key: Option<String> = None;
    let mut owner_attempt_id: Option<String> = None;
    let mut failover_attempt_id: Option<String> = None;
    let mut killed_primary_run1_logs: Option<StoppedDockerNode> = None;

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        {
            let primary = primary_run1
                .as_mut()
                .expect("primary run1 missing before membership convergence");
            let standby = standby_run1
                .as_mut()
                .expect("standby run1 missing before membership convergence");
            let mut watched = [primary, standby];
            let membership_primary = wait_for_membership(
                &artifacts,
                "membership-primary-run1",
                watched[0].host_port,
                &mut watched,
                &ExpectedMembershipTruth {
                    self_name: &primary_node,
                    membership: &membership,
                    cluster_role: "primary",
                    promotion_epoch: 0,
                    replication_health: "local_only",
                },
            );
            assert_membership_truth(
                &membership_primary,
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
            let primary = primary_run1
                .as_mut()
                .expect("primary run1 missing before standby membership convergence");
            let standby = standby_run1
                .as_mut()
                .expect("standby run1 missing before standby membership convergence");
            let mut watched = [primary, standby];
            let membership_standby = wait_for_membership(
                &artifacts,
                "membership-standby-run1",
                watched[1].host_port,
                &mut watched,
                &ExpectedMembershipTruth {
                    self_name: &standby_node,
                    membership: &membership,
                    cluster_role: "standby",
                    promotion_epoch: 0,
                    replication_health: "local_only",
                },
            );
            assert_membership_truth(
                &membership_standby,
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
            let primary = primary_run1
                .as_mut()
                .expect("primary run1 missing before placement search");
            let standby = standby_run1
                .as_mut()
                .expect("standby run1 missing before placement search");
            let mut watched = [primary, standby];
            find_submit_matching_placement(
                &artifacts,
                "same-image-failover",
                watched[0].host_port,
                &mut watched,
                &primary_node,
                &standby_node,
                128,
            )
        };
        selected_request_key = Some(submitted.request_key.clone());
        assert_eq!(submitted.status_code, 200, "initial submit must succeed");
        let original_attempt_id = required_str(&submitted.response, "attempt_id");
        owner_attempt_id = Some(original_attempt_id.clone());

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
                .join("same-image-failover-search")
                .join("selected.http"),
            artifacts.join("submit-primary.http"),
        )
        .expect("failed to retain raw primary submit response");
        write_json_artifact(&artifacts.join("submit-primary.json"), &submitted.response);

        {
            let primary = primary_run1
                .as_mut()
                .expect("primary run1 missing before pending primary truth");
            let standby = standby_run1
                .as_mut()
                .expect("standby run1 missing before pending primary truth");
            let mut watched = [primary, standby];
            wait_for_work_truth(
                &artifacts,
                "pending-primary",
                watched[0].host_port,
                &mut watched,
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
                "pending primary truth before destructive failover",
            );
        }
        {
            let primary = primary_run1
                .as_mut()
                .expect("primary run1 missing before pending standby truth");
            let standby = standby_run1
                .as_mut()
                .expect("standby run1 missing before pending standby truth");
            let mut watched = [primary, standby];
            wait_for_work_truth(
                &artifacts,
                "pending-standby",
                watched[1].host_port,
                &mut watched,
                &ExpectedWorkTruth {
                    request_key: selected_request_key.as_deref().unwrap(),
                    attempt_id: owner_attempt_id.as_deref(),
                    phase: "submitted",
                    result: "pending",
                    ingress_node: &primary_node,
                    owner_node: &primary_node,
                    replica_node: &standby_node,
                    replica_status: "mirrored",
                    cluster_role: "standby",
                    promotion_epoch: 0,
                    replication_health: "healthy",
                    execution_node: "",
                    routed_remotely: false,
                    fell_back_locally: true,
                    ok: true,
                    error: "",
                    conflict_reason: "",
                },
                "pending standby mirrored truth before destructive failover",
            );
        }

        killed_primary_run1_logs = primary_run1
            .take()
            .map(|node| docker_kill_node(node, &artifacts));
        docker_container_inspect(
            &primary_run1_config.container_name,
            &artifacts.join("primary-run1.post-kill.inspect.json"),
        );

        {
            let standby = standby_run1
                .as_mut()
                .expect("standby run1 missing after primary kill");
            let mut watched = [standby];
            wait_for_membership(
                &artifacts,
                "degraded-membership-standby",
                watched[0].host_port,
                &mut watched,
                &ExpectedMembershipTruth {
                    self_name: &standby_node,
                    membership: std::slice::from_ref(&standby_node),
                    cluster_role: "standby",
                    promotion_epoch: 0,
                    replication_health: "degraded",
                },
            );
        }
        {
            let standby = standby_run1
                .as_mut()
                .expect("standby run1 missing before promotion");
            let mut watched = [standby];
            wait_for_work_truth(
                &artifacts,
                "pre-promote-standby-status",
                watched[0].host_port,
                &mut watched,
                &ExpectedWorkTruth {
                    request_key: selected_request_key.as_deref().unwrap(),
                    attempt_id: owner_attempt_id.as_deref(),
                    phase: "submitted",
                    result: "pending",
                    ingress_node: &primary_node,
                    owner_node: &primary_node,
                    replica_node: &standby_node,
                    replica_status: "mirrored",
                    cluster_role: "standby",
                    promotion_epoch: 0,
                    replication_health: "degraded",
                    execution_node: "",
                    routed_remotely: false,
                    fell_back_locally: true,
                    ok: true,
                    error: "",
                    conflict_reason: "",
                },
                "pre-promotion degraded standby truth",
            );
        }

        let promote_response = json_body(
            &artifacts,
            "promote-standby",
            &post_json(
                standby_run1
                    .as_ref()
                    .expect("standby run1 missing during promote")
                    .host_port,
                "/promote",
                "{}",
            ),
            200,
            "standby promote response",
        );
        assert!(required_bool(&promote_response, "ok"));
        assert_eq!(required_str(&promote_response, "cluster_role"), "primary");
        assert_eq!(required_u64(&promote_response, "promotion_epoch"), 1);
        assert_eq!(
            required_str(&promote_response, "replication_health"),
            "unavailable"
        );

        {
            let standby = standby_run1
                .as_mut()
                .expect("standby run1 missing after promote");
            let mut watched = [standby];
            wait_for_membership(
                &artifacts,
                "promoted-membership-standby",
                watched[0].host_port,
                &mut watched,
                &ExpectedMembershipTruth {
                    self_name: &standby_node,
                    membership: std::slice::from_ref(&standby_node),
                    cluster_role: "primary",
                    promotion_epoch: 1,
                    replication_health: "unavailable",
                },
            );
        }
        {
            let standby = standby_run1
                .as_mut()
                .expect("standby run1 missing during owner-lost truth");
            let mut watched = [standby];
            wait_for_work_truth(
                &artifacts,
                "promoted-owner-lost-status",
                watched[0].host_port,
                &mut watched,
                &ExpectedWorkTruth {
                    request_key: selected_request_key.as_deref().unwrap(),
                    attempt_id: owner_attempt_id.as_deref(),
                    phase: "submitted",
                    result: "pending",
                    ingress_node: &primary_node,
                    owner_node: &primary_node,
                    replica_node: &standby_node,
                    replica_status: "owner_lost",
                    cluster_role: "primary",
                    promotion_epoch: 1,
                    replication_health: "unavailable",
                    execution_node: "",
                    routed_remotely: false,
                    fell_back_locally: true,
                    ok: true,
                    error: "",
                    conflict_reason: "",
                },
                "promoted owner-lost truth on standby",
            );
        }

        let failover_retry = json_body(
            &artifacts,
            "failover-retry",
            &post_json(
                standby_run1
                    .as_ref()
                    .expect("standby run1 missing during failover retry")
                    .host_port,
                "/work",
                &format!(
                    r#"{{"request_key":"{}","payload":"{}"}}"#,
                    selected_request_key.as_deref().unwrap(),
                    submitted.payload
                ),
            ),
            200,
            "same-key retry on promoted standby",
        );
        let recovered_attempt = required_str(&failover_retry, "attempt_id");
        failover_attempt_id = Some(recovered_attempt.clone());
        assert_ne!(recovered_attempt, owner_attempt_id.as_deref().unwrap());
        assert_work_truth(
            &failover_retry,
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
            let standby = standby_run1
                .as_mut()
                .expect("standby run1 missing during failover pending truth");
            let mut watched = [standby];
            wait_for_work_truth(
                &artifacts,
                "failover-pending-status",
                watched[0].host_port,
                &mut watched,
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
                "promoted authority pending truth",
            );
        }
        {
            let standby = standby_run1
                .as_mut()
                .expect("standby run1 missing during promoted completion truth");
            let mut watched = [standby];
            wait_for_work_truth(
                &artifacts,
                "failover-completed-standby",
                watched[0].host_port,
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
                "promoted authority completion truth",
            );
        }

        docker_create_container(&primary_run2_config, &network_name, &artifacts);
        primary_run2 = Some(spawn_attached_container(
            primary_run2_config.clone(),
            &artifacts,
        ));
        let primary_run2_inspect = docker_container_inspect(
            &primary_run2_config.container_name,
            &artifacts.join("primary-run2.inspect.json"),
        );
        assert_container_identity_contract(
            &primary_run2_inspect,
            &primary_run2_config.hostname,
            "primary",
            0,
        );
        assert_eq!(inspect_image_id(&primary_run2_inspect), image_id);

        {
            let primary = primary_run2
                .as_mut()
                .expect("primary run2 missing during rejoin membership");
            let standby = standby_run1
                .as_mut()
                .expect("standby run1 missing during rejoin membership");
            let mut watched = [primary, standby];
            wait_for_membership(
                &artifacts,
                "membership-primary-run2",
                watched[0].host_port,
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
            let primary = primary_run2
                .as_mut()
                .expect("primary run2 missing during standby rejoin membership");
            let standby = standby_run1
                .as_mut()
                .expect("standby run1 missing during standby rejoin membership");
            let mut watched = [primary, standby];
            wait_for_membership_with_allowed_healths(
                &artifacts,
                "membership-standby-run2",
                watched[1].host_port,
                &mut watched,
                &ExpectedMembershipTruth {
                    self_name: &standby_node,
                    membership: &membership,
                    cluster_role: "primary",
                    promotion_epoch: 1,
                    replication_health: "local_only",
                },
                &["local_only", "healthy"],
            );
        }
        {
            let primary = primary_run2
                .as_mut()
                .expect("primary run2 missing during fenced status truth");
            let standby = standby_run1
                .as_mut()
                .expect("standby run1 missing during fenced status truth");
            let mut watched = [primary, standby];
            wait_for_work_truth(
                &artifacts,
                "post-rejoin-primary-status",
                watched[0].host_port,
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
            let primary = primary_run2
                .as_mut()
                .expect("primary run2 missing during promoted standby status truth");
            let standby = standby_run1
                .as_mut()
                .expect("standby run1 missing during promoted standby status truth");
            let mut watched = [primary, standby];
            wait_for_work_truth_with_allowed_healths(
                &artifacts,
                "post-rejoin-standby-status",
                watched[1].host_port,
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
                &["local_only", "healthy"],
                "promoted standby status after rejoin",
            );
        }

        let stale_guard = json_body(
            &artifacts,
            "stale-guard-primary",
            &post_json(
                primary_run2
                    .as_ref()
                    .expect("primary run2 missing during stale-guard request")
                    .host_port,
                "/work",
                &format!(
                    r#"{{"request_key":"{}","payload":"{}"}}"#,
                    selected_request_key.as_deref().unwrap(),
                    submitted.payload
                ),
            ),
            200,
            "same-key request against stale primary after rejoin",
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

        write_json_artifact(
            &artifacts.join("scenario-meta.json"),
            &json!({
                "image_tag": IMAGE_TAG,
                "image_id": image_id,
                "network_name": network_name,
                "request_key": selected_request_key.as_deref().unwrap(),
                "payload": submitted.payload,
                "original_attempt_id": owner_attempt_id.as_deref().unwrap(),
                "failover_attempt_id": failover_attempt_id.as_deref().unwrap(),
                "primary_node": primary_node,
                "standby_node": standby_node,
                "primary_run1_container": primary_run1_config.container_name,
                "standby_run1_container": standby_run1_config.container_name,
                "primary_run2_container": primary_run2_config.container_name,
                "primary_host_port_run1": primary_run1_inspect["NetworkSettings"]["Ports"]["8080/tcp"][0]["HostPort"],
                "standby_host_port_run1": standby_run1_inspect["NetworkSettings"]["Ports"]["8080/tcp"][0]["HostPort"],
                "primary_host_port_run2": primary_run2_inspect["NetworkSettings"]["Ports"]["8080/tcp"][0]["HostPort"],
            }),
        );
    }));

    let standby_logs = standby_run1
        .take()
        .map(|node| docker_stop_node(node, &artifacts, "stop"));
    let primary_run2_logs = primary_run2
        .take()
        .map(|node| docker_stop_node(node, &artifacts, "stop"));
    let primary_run1_logs = match killed_primary_run1_logs {
        Some(logs) => Some(logs),
        None => primary_run1
            .take()
            .map(|node| docker_stop_node(node, &artifacts, "stop")),
    };

    docker_remove_container(
        &primary_run1_config.container_name,
        &artifacts,
        &primary_run1_config.artifact_label,
    );
    docker_remove_container(
        &standby_run1_config.container_name,
        &artifacts,
        &standby_run1_config.artifact_label,
    );
    docker_remove_container(
        &primary_run2_config.container_name,
        &artifacts,
        &primary_run2_config.artifact_label,
    );
    docker_network_remove(&network_name, &artifacts);

    let standby_logs = standby_logs.expect("standby logs missing during cleanup");
    let primary_run1_logs = primary_run1_logs.expect("primary run1 logs missing during cleanup");
    let primary_run2_logs = primary_run2_logs.expect("primary run2 logs missing during cleanup");

    if let Err(payload) = result {
        panic!(
            "same-image failover assertions failed: {}\nartifacts: {}\nprimary run1 stdout:\n{}\nprimary run1 stderr:\n{}\nprimary run2 stdout:\n{}\nprimary run2 stderr:\n{}\nstandby stdout:\n{}\nstandby stderr:\n{}",
            panic_payload_to_string(payload),
            artifacts.display(),
            primary_run1_logs.stdout,
            primary_run1_logs.stderr,
            primary_run2_logs.stdout,
            primary_run2_logs.stderr,
            standby_logs.stdout,
            standby_logs.stderr
        );
    }

    let request_key = selected_request_key
        .as_deref()
        .expect("selected request key missing after successful run");
    let original_attempt = owner_attempt_id
        .as_deref()
        .expect("owner attempt id missing after successful run");
    let failover_attempt = failover_attempt_id
        .as_deref()
        .expect("failover attempt id missing after successful run");

    assert!(!fs::read_dir(&artifacts)
        .unwrap()
        .collect::<Vec<_>>()
        .is_empty());
    assert!(artifacts.join("scenario-meta.json").is_file());

    for logs in [&primary_run1_logs, &primary_run2_logs, &standby_logs] {
        assert_log_absent(logs, SHARED_COOKIE);
    }

    assert_log_contains(&standby_logs, "continuity=runtime-native");
    assert_log_contains(
        &standby_logs,
        "[cluster-proof] continuity promote cluster_role=primary promotion_epoch=1 replication_health=unavailable",
    );
    assert_log_contains(
        &standby_logs,
        &format!(
            "[mesh-rt continuity] transition=promote previous_role=standby previous_epoch=0 next_role=primary next_epoch=1"
        ),
    );
    assert_log_contains(
        &standby_logs,
        &format!(
            "[mesh-rt continuity] transition=recovery_rollover request_key={request_key} previous_attempt_id={original_attempt} next_attempt_id={failover_attempt}"
        ),
    );
    assert_log_contains(
        &standby_logs,
        &format!(
            "[cluster-proof] work executed request_key={request_key} attempt_id={failover_attempt} execution={standby_node}"
        ),
    );
    assert_log_absent(
        &primary_run1_logs,
        &format!(
            "[mesh-rt continuity] transition=completed request_key={request_key} attempt_id={original_attempt}"
        ),
    );
    assert_log_contains(
        &primary_run2_logs,
        &format!(
            "[mesh-rt continuity] transition=fenced_rejoin request_key={request_key} attempt_id={failover_attempt} previous_role=primary previous_epoch=0 next_role=standby next_epoch=1"
        ),
    );
    assert_log_contains(
        &primary_run2_logs,
        &format!(
            "[cluster-proof] keyed status request_key={request_key} attempt_id={failover_attempt} phase=completed result=succeeded owner={standby_node} replica= source={primary_node} replica_status=unassigned cluster_role=standby promotion_epoch=1"
        ),
    );
    assert_log_absent(
        &primary_run2_logs,
        &format!(
            "[cluster-proof] work executed request_key={request_key} attempt_id={failover_attempt} execution={primary_node}"
        ),
    );
}
