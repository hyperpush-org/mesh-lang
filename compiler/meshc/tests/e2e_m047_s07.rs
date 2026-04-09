mod support;

use serde_json::{json, Value};
use std::fs;
use std::io::{Read as _, Write as _};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::{Duration, Instant};
use support::m046_route_free as route_free;
use support::m047_todo_scaffold as todo;
use support::m049_todo_postgres_scaffold as postgres_scaffold;

const SHARED_COOKIE: &str = "mesh-m047-s07-http-cookie";
const DISCOVERY_SEED: &str = "localhost";
const SUCCESS_RUNTIME_NAME: &str = "Api.Todos.handle_list_todos";
const UNSUPPORTED_RUNTIME_NAME: &str = "Api.Todos.handle_retry_todos";
const UNSUPPORTED_REPLICATION_REASON: &str = "unsupported_replication_count:3";
const HTTP_READY_TIMEOUT: Duration = Duration::from_secs(15);

struct ClusteredHttpProject {
    _tempdir: tempfile::TempDir,
    project_dir: PathBuf,
    binary_path: PathBuf,
}

fn artifact_dir(test_name: &str) -> PathBuf {
    route_free::artifact_dir("m047-s07", test_name)
}

fn package_manifest(name: &str) -> String {
    format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\n")
}

fn main_source(primary_http_port: u16, standby_http_port: u16) -> String {
    format!(
        r#"from Api.Router import build_router

fn log_bootstrap(status :: BootstrapStatus) do
  println(
    "[m047-s07] runtime bootstrap mode=#{{status.mode}} node=#{{status.node_name}} cluster_port=#{{status.cluster_port}} discovery_seed=#{{status.discovery_seed}}"
  )
end

fn log_bootstrap_failure(reason :: String) do
  println("[m047-s07] runtime bootstrap failed reason=#{{reason}}")
end

fn runtime_role() -> String do
  Env.get("MESH_CONTINUITY_ROLE", "primary")
end

fn http_port() -> Int do
  if runtime_role() == "standby" do
    {standby_http_port}
  else
    {primary_http_port}
  end
end

fn start_http_runtime() do
  let role = runtime_role()
  let port = http_port()
  println("[m047-s07] HTTP server starting role=#{{role}} port=#{{port}}")
  let router = build_router()
  HTTP.serve(router, port)
end

fn on_bootstrap_success(status :: BootstrapStatus) do
  log_bootstrap(status)
  start_http_runtime()
end

fn main() do
  case Node.start_from_env() do
    Ok(status) -> on_bootstrap_success(status)
    Err(reason) -> log_bootstrap_failure(reason)
  end
end
"#
    )
}

fn health_source() -> &'static str {
    r#"pub fn handle_health(_request) do
  HTTP.response(200, json { status : "ok" })
end
"#
}

fn todos_source() -> &'static str {
    r#"pub fn handle_list_todos(request :: Request) -> Response do
  HTTP.response(200,
    json {
      status : "ok",
      handler : "Api.Todos.handle_list_todos",
      method : Request.method(request),
      path : Request.path(request)
    }
  )
end

pub fn handle_retry_todos(_request :: Request) -> Response do
  HTTP.response(200,
    json {
      status : "unexpected_success",
      handler : "Api.Todos.handle_retry_todos"
    }
  )
end
"#
}

fn router_source() -> &'static str {
    r#"import Api.Todos
from Api.Health import handle_health
from Api.Todos import handle_list_todos

pub fn build_router() do
  let router = HTTP.router()
    |> HTTP.on_get("/health", handle_health)
    |> HTTP.on_get("/todos", HTTP.clustered(handle_list_todos))
  let router = HTTP.on_get(router, "/todos/retry", HTTP.clustered(3, Todos.handle_retry_todos))
  router
end
"#
}

fn write_project_sources(project_dir: &Path, manifest: &str, sources: &[(&str, &str)]) {
    fs::create_dir_all(project_dir)
        .unwrap_or_else(|error| panic!("failed to create {}: {error}", project_dir.display()));
    fs::write(project_dir.join("mesh.toml"), manifest).expect("failed to write temp mesh.toml");
    for (path, source) in sources {
        let full_path = project_dir.join(path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .unwrap_or_else(|error| panic!("failed to create {}: {error}", parent.display()));
        }
        fs::write(&full_path, source)
            .unwrap_or_else(|error| panic!("failed to write {}: {error}", full_path.display()));
    }
}

fn build_clustered_http_project(
    name: &str,
    primary_http_port: u16,
    standby_http_port: u16,
    artifacts: &Path,
) -> ClusteredHttpProject {
    let tempdir = tempfile::tempdir().expect("failed to create clustered-http tempdir");
    let project_dir = tempdir.path().join("project");
    let output_dir = tempdir.path().join("out");
    fs::create_dir_all(&output_dir)
        .unwrap_or_else(|error| panic!("failed to create {}: {error}", output_dir.display()));

    let manifest = package_manifest(name);
    let main = main_source(primary_http_port, standby_http_port);
    let health = health_source();
    let todos = todos_source();
    let router = router_source();

    write_project_sources(
        &project_dir,
        &manifest,
        &[
            ("main.mpl", &main),
            ("api/health.mpl", health),
            ("api/todos.mpl", todos),
            ("api/router.mpl", router),
        ],
    );
    route_free::archive_directory_tree(&project_dir, &artifacts.join("generated-project"));

    let binary_path = output_dir.join(name);
    let build_metadata =
        route_free::build_package_binary_to_output(&project_dir, &binary_path, artifacts);
    let persisted_metadata = route_free::read_required_build_metadata(artifacts)
        .unwrap_or_else(|error| panic!("build metadata should be readable: {error}"));
    assert_eq!(build_metadata, persisted_metadata);
    assert_eq!(persisted_metadata.binary_path, binary_path);

    ClusteredHttpProject {
        _tempdir: tempdir,
        project_dir,
        binary_path,
    }
}

fn unique_http_ports() -> (u16, u16) {
    let first = todo::unused_port();
    let mut second = todo::unused_port();
    while second == first {
        second = todo::unused_port();
    }
    (first, second)
}

fn send_http_request(
    port: u16,
    method: &str,
    path: &str,
    body: Option<&str>,
) -> std::io::Result<todo::HttpResponse> {
    let mut stream = TcpStream::connect((route_free::LOOPBACK_V4, port))?;
    stream.set_read_timeout(Some(Duration::from_secs(10)))?;

    let request = match body {
        Some(body) => format!(
            "{method} {path} HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.as_bytes().len(),
            body
        ),
        None => format!(
            "{method} {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"
        ),
    };

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

    Ok(todo::HttpResponse {
        status_code,
        body,
        raw,
    })
}

fn wait_for_health(port: u16, artifacts: &Path, label: &str) -> Value {
    let start = Instant::now();
    let mut last_observation = String::new();

    while start.elapsed() < HTTP_READY_TIMEOUT {
        match send_http_request(port, "GET", "/health", None) {
            Ok(response) if response.status_code == 200 => {
                let json = todo::json_response_snapshot(
                    artifacts,
                    label,
                    &response,
                    200,
                    &format!("/health on :{port}"),
                );
                if json["status"].as_str() == Some("ok") {
                    return json;
                }
                last_observation = serde_json::to_string_pretty(&json)
                    .expect("health response should render as json");
            }
            Ok(response) => last_observation = response.raw,
            Err(error) => last_observation = format!("connect error: {error}"),
        }

        sleep(Duration::from_millis(200));
    }

    let timeout_path = artifacts.join(format!("{label}.timeout.txt"));
    route_free::write_artifact(&timeout_path, &last_observation);
    panic!(
        "HTTP health on :{port} never reached ready state within {:?}; last observation archived at {}",
        HTTP_READY_TIMEOUT,
        timeout_path.display()
    );
}

fn get_json_snapshot(
    artifacts: &Path,
    label: &str,
    port: u16,
    path: &str,
    expected_status: u16,
) -> (todo::HttpResponse, Value) {
    let response = send_http_request(port, "GET", path, None)
        .unwrap_or_else(|error| panic!("GET {path} failed on :{port}: {error}"));
    let json = todo::json_response_snapshot(
        artifacts,
        label,
        &response,
        expected_status,
        &format!("GET {path} on :{port}"),
    );
    (response, json)
}

fn required_correlation_request_key(
    artifacts: &Path,
    label: &str,
    response: &todo::HttpResponse,
) -> String {
    let request_key =
        postgres_scaffold::required_response_header(&response.raw, "X-Mesh-Continuity-Request-Key");
    route_free::write_artifact(
        &artifacts.join(format!("{label}.request-key.txt")),
        &request_key,
    );
    request_key
}

fn wait_for_rejected_route_record(
    artifacts: &Path,
    label: &str,
    node_name: &str,
    request_key: &str,
    runtime_name: &str,
    cookie: &str,
) -> Value {
    route_free::wait_for_continuity_record_matching(
        artifacts,
        label,
        node_name,
        request_key,
        &format!("rejected continuity record for {runtime_name}"),
        cookie,
        |json| {
            let record = &json["record"];
            route_free::required_str(record, "request_key") == request_key
                && route_free::required_str(record, "declared_handler_runtime_name") == runtime_name
                && route_free::required_u64(record, "replication_count") == 3
                && route_free::required_str(record, "phase") == "rejected"
                && route_free::required_str(record, "result") == "rejected"
                && route_free::required_str(record, "error") == UNSUPPORTED_REPLICATION_REASON
        },
    )
}

fn assert_success_route_record(record: &Value, request_key: &str, expected_nodes: &[String]) {
    let owner_node = route_free::required_str(record, "owner_node");
    let replica_node = route_free::required_str(record, "replica_node");

    assert_eq!(route_free::required_str(record, "request_key"), request_key);
    assert_eq!(
        route_free::required_str(record, "declared_handler_runtime_name"),
        SUCCESS_RUNTIME_NAME
    );
    assert_eq!(route_free::required_u64(record, "replication_count"), 2);
    assert_eq!(route_free::required_str(record, "phase"), "completed");
    assert_eq!(route_free::required_str(record, "result"), "succeeded");
    assert_eq!(route_free::required_str(record, "error"), "");
    assert!(expected_nodes.contains(&owner_node));
    assert!(expected_nodes.contains(&replica_node));
    assert_ne!(owner_node, replica_node);
    assert_eq!(
        route_free::required_str(record, "execution_node"),
        owner_node
    );
    assert_eq!(
        route_free::required_str(record, "replica_status"),
        "mirrored"
    );
}

fn assert_rejected_route_record(record: &Value, request_key: &str) {
    assert_eq!(route_free::required_str(record, "request_key"), request_key);
    assert_eq!(
        route_free::required_str(record, "declared_handler_runtime_name"),
        UNSUPPORTED_RUNTIME_NAME
    );
    assert_eq!(route_free::required_u64(record, "replication_count"), 3);
    assert_eq!(route_free::required_str(record, "phase"), "rejected");
    assert_eq!(route_free::required_str(record, "result"), "rejected");
    assert_eq!(
        route_free::required_str(record, "error"),
        UNSUPPORTED_REPLICATION_REASON
    );
}

#[test]
fn m047_s07_continuity_diff_helpers_ignore_record_order_for_repeated_runtime_names() {
    let before = json!({
        "records": [
            {
                "request_key": "http-route::Api.Todos.handle_list_todos::1",
                "declared_handler_runtime_name": SUCCESS_RUNTIME_NAME
            },
            {
                "request_key": "other-runtime::1",
                "declared_handler_runtime_name": "Other.handle"
            }
        ]
    });
    let after = json!({
        "records": [
            {
                "request_key": "other-runtime::1",
                "declared_handler_runtime_name": "Other.handle"
            },
            {
                "request_key": "http-route::Api.Todos.handle_list_todos::2",
                "declared_handler_runtime_name": SUCCESS_RUNTIME_NAME
            },
            {
                "request_key": "http-route::Api.Todos.handle_list_todos::1",
                "declared_handler_runtime_name": SUCCESS_RUNTIME_NAME
            }
        ]
    });

    let new_keys =
        route_free::new_request_keys_for_runtime_name(&before, &after, SUCCESS_RUNTIME_NAME);
    assert_eq!(
        new_keys,
        vec!["http-route::Api.Todos.handle_list_todos::2".to_string()]
    );
    let record = route_free::record_for_request_key(&after, &new_keys[0]);
    assert_eq!(
        route_free::required_str(record, "declared_handler_runtime_name"),
        SUCCESS_RUNTIME_NAME
    );
}

#[test]
fn m047_s07_continuity_diff_helpers_fail_closed_on_missing_request_keys() {
    let before = json!({ "records": [] });
    let malformed_after = json!({
        "records": [
            {
                "declared_handler_runtime_name": SUCCESS_RUNTIME_NAME
            }
        ]
    });

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        route_free::new_request_keys_for_runtime_name(
            &before,
            &malformed_after,
            SUCCESS_RUNTIME_NAME,
        )
    }));
    assert!(result.is_err(), "missing request keys should fail closed");
}

#[test]
fn m047_s07_clustered_http_routes_two_node_end_to_end() {
    let artifacts = artifact_dir("clustered-http-routes-two-node");
    let (primary_http_port, standby_http_port) = unique_http_ports();
    let project = build_clustered_http_project(
        "m047-s07-clustered-http",
        primary_http_port,
        standby_http_port,
        &artifacts,
    );

    let cluster_port = route_free::dual_stack_cluster_port();
    let primary_node = format!(
        "m047-s07-primary@{}:{cluster_port}",
        route_free::LOOPBACK_V4
    );
    let standby_node = format!(
        "m047-s07-standby@[{}]:{}",
        route_free::LOOPBACK_V6,
        cluster_port
    );
    let expected_nodes = vec![primary_node.clone(), standby_node.clone()];

    route_free::write_json_artifact(
        &artifacts.join("scenario-meta.json"),
        &json!({
            "project_dir": project.project_dir.display().to_string(),
            "binary_path": project.binary_path.display().to_string(),
            "cluster_port": cluster_port,
            "primary_node": primary_node,
            "standby_node": standby_node,
            "primary_http_port": primary_http_port,
            "standby_http_port": standby_http_port,
            "success_runtime_name": SUCCESS_RUNTIME_NAME,
            "unsupported_runtime_name": UNSUPPORTED_RUNTIME_NAME,
        }),
    );

    let primary_proc = route_free::spawn_route_free_runtime(
        &project.binary_path,
        &project.project_dir,
        &artifacts,
        "primary",
        &primary_node,
        cluster_port,
        "primary",
        0,
        SHARED_COOKIE,
        DISCOVERY_SEED,
    );
    let standby_proc = route_free::spawn_route_free_runtime(
        &project.binary_path,
        &project.project_dir,
        &artifacts,
        "standby",
        &standby_node,
        cluster_port,
        "standby",
        0,
        SHARED_COOKIE,
        DISCOVERY_SEED,
    );

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        route_free::wait_for_cluster_status_membership(
            &artifacts,
            "cluster-status-primary",
            &primary_node,
            std::slice::from_ref(&standby_node),
            &expected_nodes,
            "primary",
            0,
            &["local_only", "healthy"],
            SHARED_COOKIE,
        );
        route_free::wait_for_cluster_status_membership(
            &artifacts,
            "cluster-status-standby",
            &standby_node,
            std::slice::from_ref(&primary_node),
            &expected_nodes,
            "standby",
            0,
            &["local_only", "healthy"],
            SHARED_COOKIE,
        );

        let primary_health = wait_for_health(primary_http_port, &artifacts, "health-primary");
        let standby_health = wait_for_health(standby_http_port, &artifacts, "health-standby");
        assert_eq!(primary_health["status"].as_str(), Some("ok"));
        assert_eq!(standby_health["status"].as_str(), Some("ok"));

        let (first_success_response, first_success) = get_json_snapshot(
            &artifacts,
            "route-success-first",
            primary_http_port,
            "/todos",
            200,
        );
        assert_eq!(first_success["status"].as_str(), Some("ok"));
        assert_eq!(
            first_success["handler"].as_str(),
            Some(SUCCESS_RUNTIME_NAME)
        );
        assert_eq!(first_success["method"].as_str(), Some("GET"));
        assert_eq!(first_success["path"].as_str(), Some("/todos"));
        let first_request_key_primary = required_correlation_request_key(
            &artifacts,
            "route-success-first",
            &first_success_response,
        );
        assert!(
            first_request_key_primary.starts_with("http-route::Api.Todos.handle_list_todos::"),
            "unexpected first request key: {first_request_key_primary}"
        );

        let first_primary_record = route_free::wait_for_continuity_record_completed(
            &artifacts,
            "continuity-first-completed-primary",
            &primary_node,
            &first_request_key_primary,
            SUCCESS_RUNTIME_NAME,
            SHARED_COOKIE,
        );
        let first_standby_record = route_free::wait_for_continuity_record_completed(
            &artifacts,
            "continuity-first-completed-standby",
            &standby_node,
            &first_request_key_primary,
            SUCCESS_RUNTIME_NAME,
            SHARED_COOKIE,
        );
        assert_success_route_record(
            &first_primary_record["record"],
            &first_request_key_primary,
            &expected_nodes,
        );
        assert_success_route_record(
            &first_standby_record["record"],
            &first_request_key_primary,
            &expected_nodes,
        );

        let (second_success_response, second_success) = get_json_snapshot(
            &artifacts,
            "route-success-second",
            primary_http_port,
            "/todos",
            200,
        );
        assert_eq!(second_success["status"].as_str(), Some("ok"));
        assert_eq!(
            second_success["handler"].as_str(),
            Some(SUCCESS_RUNTIME_NAME)
        );
        assert_eq!(second_success["method"].as_str(), Some("GET"));
        assert_eq!(second_success["path"].as_str(), Some("/todos"));
        let second_request_key_primary = required_correlation_request_key(
            &artifacts,
            "route-success-second",
            &second_success_response,
        );
        assert!(
            second_request_key_primary.starts_with("http-route::Api.Todos.handle_list_todos::"),
            "unexpected second request key: {second_request_key_primary}"
        );
        assert_ne!(first_request_key_primary, second_request_key_primary);

        let second_primary_record = route_free::wait_for_continuity_record_completed(
            &artifacts,
            "continuity-second-completed-primary",
            &primary_node,
            &second_request_key_primary,
            SUCCESS_RUNTIME_NAME,
            SHARED_COOKIE,
        );
        let second_standby_record = route_free::wait_for_continuity_record_completed(
            &artifacts,
            "continuity-second-completed-standby",
            &standby_node,
            &second_request_key_primary,
            SUCCESS_RUNTIME_NAME,
            SHARED_COOKIE,
        );
        assert_success_route_record(
            &second_primary_record["record"],
            &second_request_key_primary,
            &expected_nodes,
        );
        assert_success_route_record(
            &second_standby_record["record"],
            &second_request_key_primary,
            &expected_nodes,
        );

        let (rejected_response_raw, rejected_response) = get_json_snapshot(
            &artifacts,
            "route-unsupported-count",
            standby_http_port,
            "/todos/retry",
            503,
        );
        assert_eq!(
            rejected_response["error"].as_str(),
            Some(UNSUPPORTED_REPLICATION_REASON)
        );

        let rejected_request_key_primary = required_correlation_request_key(
            &artifacts,
            "route-unsupported-count",
            &rejected_response_raw,
        );
        assert!(
            rejected_request_key_primary.starts_with("http-route::Api.Todos.handle_retry_todos::"),
            "unexpected rejected request key: {rejected_request_key_primary}"
        );

        let rejected_primary_record = wait_for_rejected_route_record(
            &artifacts,
            "continuity-rejected-primary",
            &primary_node,
            &rejected_request_key_primary,
            UNSUPPORTED_RUNTIME_NAME,
            SHARED_COOKIE,
        );
        let rejected_standby_record = wait_for_rejected_route_record(
            &artifacts,
            "continuity-rejected-standby",
            &standby_node,
            &rejected_request_key_primary,
            UNSUPPORTED_RUNTIME_NAME,
            SHARED_COOKIE,
        );
        assert_rejected_route_record(
            &rejected_primary_record["record"],
            &rejected_request_key_primary,
        );
        assert_rejected_route_record(
            &rejected_standby_record["record"],
            &rejected_request_key_primary,
        );
    }));

    let primary_logs = route_free::stop_process(primary_proc);
    let standby_logs = route_free::stop_process(standby_proc);
    route_free::write_artifact(
        &artifacts.join("primary.combined.log"),
        &primary_logs.combined,
    );
    route_free::write_artifact(
        &artifacts.join("standby.combined.log"),
        &standby_logs.combined,
    );

    if let Err(payload) = result {
        panic!(
            "{}\nartifacts: {}\nprimary stdout:\n{}\nprimary stderr:\n{}\nstandby stdout:\n{}\nstandby stderr:\n{}",
            route_free::panic_payload_to_string(payload),
            artifacts.display(),
            primary_logs.stdout,
            primary_logs.stderr,
            standby_logs.stdout,
            standby_logs.stderr,
        );
    }

    for required in [
        "generated-project/mesh.toml",
        "generated-project/main.mpl",
        "generated-project/api/health.mpl",
        "generated-project/api/router.mpl",
        "generated-project/api/todos.mpl",
        "build.log",
        "build-meta.json",
        "scenario-meta.json",
        "cluster-status-primary.json",
        "cluster-status-standby.json",
        "health-primary.http",
        "health-primary.json",
        "health-standby.http",
        "health-standby.json",
        "route-success-first.http",
        "route-success-first.json",
        "route-success-first.request-key.txt",
        "route-success-second.http",
        "route-success-second.json",
        "route-success-second.request-key.txt",
        "route-unsupported-count.http",
        "route-unsupported-count.json",
        "route-unsupported-count.request-key.txt",
        "continuity-first-completed-primary.json",
        "continuity-first-completed-standby.json",
        "continuity-second-completed-primary.json",
        "continuity-second-completed-standby.json",
        "continuity-rejected-primary.json",
        "continuity-rejected-standby.json",
        "primary.stdout.log",
        "primary.stderr.log",
        "standby.stdout.log",
        "standby.stderr.log",
        "primary.combined.log",
        "standby.combined.log",
    ] {
        assert!(
            artifacts.join(required).exists(),
            "missing retained clustered-http artifact {} in {}",
            required,
            artifacts.display()
        );
    }

    route_free::assert_log_absent(&primary_logs, SHARED_COOKIE);
    route_free::assert_log_absent(&standby_logs, SHARED_COOKIE);
    route_free::assert_log_contains(
        &primary_logs,
        &format!("[m047-s07] runtime bootstrap mode=cluster node={primary_node}"),
    );
    route_free::assert_log_contains(
        &standby_logs,
        &format!("[m047-s07] runtime bootstrap mode=cluster node={standby_node}"),
    );
    route_free::assert_log_contains(
        &primary_logs,
        &format!("[m047-s07] HTTP server starting role=primary port={primary_http_port}"),
    );
    route_free::assert_log_contains(
        &standby_logs,
        &format!("[m047-s07] HTTP server starting role=standby port={standby_http_port}"),
    );
    assert!(
        primary_logs.combined.contains(UNSUPPORTED_REPLICATION_REASON)
            || standby_logs.combined.contains(UNSUPPORTED_REPLICATION_REASON),
        "expected unsupported-count rejection to appear in retained runtime logs\nprimary:\n{}\nstandby:\n{}",
        primary_logs.combined,
        standby_logs.combined,
    );
}
