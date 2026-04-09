mod support;

use serde_json::{json, Value};
use std::any::Any;
use std::fs;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;

use support::m046_route_free as route_free;
use support::m049_todo_postgres_scaffold as postgres;
use support::m053_todo_postgres_deploy as deploy;
use support::m054_public_ingress::{self as public_ingress, PublicIngressRequestRecord};

const CORRELATION_HEADER: &str = "X-Mesh-Continuity-Request-Key";

fn panic_payload_to_string(payload: &(dyn Any + Send)) -> String {
    if let Some(message) = payload.downcast_ref::<&str>() {
        (*message).to_string()
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else {
        "non-string panic payload".to_string()
    }
}

fn required_database_url(test_name: &str) -> String {
    std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| panic!("DATABASE_URL must be set for {test_name}"))
}

fn artifact_dir(test_name: &str) -> PathBuf {
    route_free::artifact_dir("m054-s02", test_name)
}

fn sample_public_request() -> PublicIngressRequestRecord {
    PublicIngressRequestRecord {
        request_id: 1,
        method: "GET".to_string(),
        path: "/todos".to_string(),
        request_line: "GET /todos HTTP/1.1".to_string(),
        target_label: "standby".to_string(),
        target_node: "todo-postgres-standby@[::1]:4370".to_string(),
        target_host: route_free::LOOPBACK_V4.to_string(),
        target_port: 8081,
        status_code: 200,
        response_status_line: "HTTP/1.1 200 OK".to_string(),
        request_raw: "GET /todos HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"
            .to_string(),
        response_raw: format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n{CORRELATION_HEADER}: http-route::Api.Todos.handle_list_todos::1\r\nConnection: close\r\n\r\n[]"
        ),
        error: String::new(),
    }
}

fn sample_route_record(request_key: &str) -> Value {
    json!({
        "request_key": request_key,
        "attempt_id": "attempt-0",
        "declared_handler_runtime_name": deploy::LIST_ROUTE_RUNTIME_NAME,
        "replication_count": 1,
        "ingress_node": "todo-postgres-standby@[::1]:4370",
        "owner_node": "todo-postgres-primary@127.0.0.1:4370",
        "replica_node": "todo-postgres-standby@[::1]:4370",
        "execution_node": "todo-postgres-primary@127.0.0.1:4370",
        "phase": "completed",
        "result": "succeeded",
        "error": ""
    })
}

fn assert_header_failure(raw_response: &str, expected_message_fragment: &str) {
    let panic = catch_unwind(AssertUnwindSafe(|| {
        postgres::required_response_header(raw_response, CORRELATION_HEADER)
    }))
    .expect_err("malformed response headers should fail closed");
    let message = panic_payload_to_string(panic.as_ref());
    assert!(
        message.contains(expected_message_fragment),
        "expected `{expected_message_fragment}` in panic message, got: {message}"
    );
}

#[test]
fn m054_s02_staged_postgres_public_ingress_directly_correlates_selected_get_todos_request() {
    let test_name =
        "m054_s02_staged_postgres_public_ingress_directly_correlates_selected_get_todos_request";
    let base_database_url = required_database_url(test_name);
    let artifacts = artifact_dir("staged-postgres-public-ingress-direct-correlation");
    let workspace_dir = artifacts.join("workspace");
    fs::create_dir_all(&workspace_dir)
        .unwrap_or_else(|error| panic!("failed to create {}: {error}", workspace_dir.display()));

    let project_dir =
        deploy::init_postgres_todo_project(&workspace_dir, deploy::PACKAGE_NAME, &artifacts);
    let database = deploy::create_isolated_database(&base_database_url, &artifacts, "public");
    let bundle_dir = deploy::create_retained_bundle_dir("todo-postgres-public-ingress-direct");
    let stage =
        deploy::run_stage_deploy_script(&project_dir, &bundle_dir, &artifacts, "stage-deploy");
    deploy::assert_phase_success(&stage, "generated Postgres stage-deploy.sh should succeed");

    let bundle = deploy::inspect_staged_bundle(&bundle_dir, &artifacts);
    let apply = deploy::run_staged_apply_deploy_migrations_script(
        &bundle,
        &artifacts,
        "deploy-apply",
        Some(&database.database_url),
    );
    deploy::assert_phase_success(&apply, "staged apply-deploy-migrations.sh should succeed");

    let runtime = deploy::default_cluster_runtime_pair_for_primary_owned_startup(
        deploy::PACKAGE_NAME,
        &database.database_url,
        None,
    );
    let secret_values = [
        base_database_url.as_str(),
        database.database_url.as_str(),
        runtime.primary.cluster_cookie.as_str(),
    ];
    let spawned = deploy::spawn_staged_todo_cluster(&bundle, &artifacts, "runtime", &runtime);

    let ingress = public_ingress::start_public_ingress(
        &artifacts,
        "public-ingress",
        vec![
            public_ingress::PublicIngressTarget::for_todo_runtime("primary", &runtime.primary),
            public_ingress::PublicIngressTarget::for_todo_runtime("standby", &runtime.standby),
        ],
        1,
    );

    let mut selected_request_key: Option<String> = None;
    deploy::write_json_artifact(
        &artifacts.join("scenario-meta.json"),
        &json!({
            "project_dir": project_dir.display().to_string(),
            "bundle_dir": bundle.bundle_dir.display().to_string(),
            "public_base_url": ingress.base_url(),
            "public_port": ingress.port(),
            "public_first_target": "standby",
            "list_route_runtime_name": deploy::LIST_ROUTE_RUNTIME_NAME,
            "primary_node": runtime.primary.node_name.clone(),
            "standby_node": runtime.standby.node_name.clone(),
            "request_key": Value::Null,
            "database_url": "<redacted:DATABASE_URL>",
        }),
    );

    let run_result = catch_unwind(AssertUnwindSafe(|| {
        let (primary_health, standby_health) =
            deploy::wait_for_cluster_health(&runtime, &artifacts, "health", &secret_values);
        for (label, json) in [("primary", &primary_health), ("standby", &standby_health)] {
            assert_eq!(
                json["status"].as_str(),
                Some("ok"),
                "{label} health should be ok"
            );
            assert_eq!(json["db_backend"].as_str(), Some("postgres"));
            assert_eq!(
                json["clustered_handler"].as_str(),
                Some(deploy::STARTUP_RUNTIME_NAME)
            );
            assert!(
                json.get("database_url").is_none(),
                "{label} /health must not leak DATABASE_URL"
            );
        }

        let (primary_status, standby_status) =
            deploy::wait_for_dual_node_cluster_status(&artifacts, "cluster-status", &runtime);
        assert_eq!(
            route_free::required_str(&primary_status["authority"], "cluster_role"),
            "primary"
        );
        assert_eq!(
            route_free::required_str(&primary_status["authority"], "replication_health"),
            "healthy"
        );
        assert_eq!(
            route_free::required_str(&standby_status["authority"], "cluster_role"),
            "standby"
        );
        assert_eq!(
            route_free::required_str(&standby_status["authority"], "replication_health"),
            "healthy"
        );

        let public_before = ingress.snapshot();
        ingress.force_next_target("standby");
        let selected_public_response =
            deploy::send_http_request(ingress.port(), "GET", "/todos", None).unwrap_or_else(
                |error| {
                    panic!(
                        "GET /todos via public ingress {} failed: {error}",
                        ingress.base_url()
                    )
                },
            );
        let selected_public_json = deploy::json_response_snapshot(
            &artifacts,
            "public-selected-list",
            &selected_public_response,
            200,
            "GET /todos via one public ingress URL before create",
            &secret_values,
        );
        assert!(
            selected_public_json
                .as_array()
                .is_some_and(|items| items.is_empty()),
            "expected the first public GET /todos after startup to return an empty list, got: {selected_public_json}"
        );

        let public_after = ingress.snapshot();
        let selected_public_request = public_ingress::single_new_request(&public_before, &public_after);
        assert_eq!(selected_public_request.request_id, 1);
        assert_eq!(selected_public_request.method, "GET");
        assert_eq!(selected_public_request.path, "/todos");
        assert_eq!(selected_public_request.target_label, "standby");
        assert_eq!(selected_public_request.status_code, 200);
        assert!(selected_public_request.error.is_empty());
        assert_eq!(
            selected_public_request.response_raw,
            selected_public_response.raw,
            "public ingress retained raw response drifted from the client-observed selected response"
        );
        deploy::write_json_artifact(
            &artifacts.join("public-selected-list.request-summary.json"),
            &selected_public_request,
        );

        let request_key_from_client =
            postgres::required_response_header(&selected_public_response.raw, CORRELATION_HEADER);
        let request_key_from_ingress = postgres::required_response_header(
            &selected_public_request.response_raw,
            CORRELATION_HEADER,
        );
        assert_eq!(
            request_key_from_client, request_key_from_ingress,
            "selected public response header drifted between the client capture and retained ingress transcript"
        );
        selected_request_key = Some(request_key_from_ingress.clone());
        route_free::write_artifact(
            &artifacts.join("public-selected-list.request-key.txt"),
            &request_key_from_ingress,
        );
        deploy::write_json_artifact(
            &artifacts.join("public-selected-list.request-key.json"),
            &json!({
                "header_name": CORRELATION_HEADER,
                "request_key": request_key_from_ingress,
                "public_request_id": selected_public_request.request_id,
                "public_target_label": selected_public_request.target_label.clone(),
                "public_target_node": selected_public_request.target_node.clone(),
                "public_status_code": selected_public_request.status_code,
                "public_response_artifact": "public-selected-list.http",
                "public_ingress_requests_artifact": "public-ingress.requests.json",
            }),
        );
        deploy::write_json_artifact(
            &artifacts.join("scenario-meta.json"),
            &json!({
                "project_dir": project_dir.display().to_string(),
                "bundle_dir": bundle.bundle_dir.display().to_string(),
                "public_base_url": ingress.base_url(),
                "public_port": ingress.port(),
                "public_first_target": "standby",
                "public_request_id": selected_public_request.request_id,
                "list_route_runtime_name": deploy::LIST_ROUTE_RUNTIME_NAME,
                "primary_node": runtime.primary.node_name.clone(),
                "standby_node": runtime.standby.node_name.clone(),
                "request_key": selected_request_key.as_deref().unwrap(),
                "database_url": "<redacted:DATABASE_URL>",
            }),
        );

        let request_key = selected_request_key.as_deref().unwrap();
        let (primary_record_json, standby_record_json) =
            deploy::wait_for_continuity_record_completed_pair(
                &artifacts,
                "selected-route-direct",
                &runtime,
                request_key,
                deploy::LIST_ROUTE_RUNTIME_NAME,
            );
        assert_eq!(
            route_free::required_str(&primary_record_json["record"], "request_key"),
            request_key
        );
        assert_eq!(
            route_free::required_str(&standby_record_json["record"], "request_key"),
            request_key
        );

        let summary = public_ingress::build_route_continuity_summary(
            &selected_public_request,
            &primary_record_json["record"],
            &standby_record_json["record"],
            deploy::LIST_ROUTE_RUNTIME_NAME,
        );
        assert_eq!(summary.request_key, request_key);
        assert_eq!(summary.public_request_id, selected_public_request.request_id);
        assert_eq!(summary.public_target_label, "standby");
        assert_eq!(summary.ingress_node, selected_public_request.target_node);
        deploy::write_json_artifact(&artifacts.join("selected-route.summary.json"), &summary);

        let (primary_diagnostics, standby_diagnostics) = deploy::wait_for_request_diagnostics_pair(
            &artifacts,
            "selected-route-direct",
            &runtime,
            request_key,
        );
        let primary_entries =
            route_free::diagnostic_entries_for_request(&primary_diagnostics, request_key);
        let standby_entries =
            route_free::diagnostic_entries_for_request(&standby_diagnostics, request_key);
        assert!(
            !primary_entries.is_empty(),
            "primary diagnostics should retain entries for the selected public request key"
        );
        assert!(
            !standby_entries.is_empty(),
            "standby diagnostics should retain entries for the selected public request key"
        );
        deploy::write_json_artifact(
            &artifacts.join("selected-route.primary-diagnostics.entries.json"),
            &primary_entries,
        );
        deploy::write_json_artifact(
            &artifacts.join("selected-route.standby-diagnostics.entries.json"),
            &standby_entries,
        );
    }));

    let ingress_snapshot = ingress.stop();
    let stopped = deploy::stop_staged_todo_cluster(spawned, &secret_values);
    deploy::write_artifact(
        &artifacts.join("runtime-primary.combined.log"),
        &stopped.primary.combined,
    );
    deploy::write_artifact(
        &artifacts.join("runtime-standby.combined.log"),
        &stopped.standby.combined,
    );

    match run_result {
        Ok(()) => {
            assert_eq!(
                ingress_snapshot.request_count, 1,
                "expected exactly one retained public ingress request for the selected GET /todos proof"
            );
            assert!(
                selected_request_key.is_some(),
                "selected public request correlation key should be captured before teardown"
            );
            deploy::assert_runtime_logs(&stopped.primary, &runtime.primary);
            deploy::assert_runtime_logs(&stopped.standby, &runtime.standby);
            deploy::assert_artifacts_redacted(&artifacts, &secret_values);
        }
        Err(payload) => panic!(
            "direct-correlation public ingress staged Postgres assertions failed: {}\nartifacts: {}\npublic_requests: {}\nprimary stdout:\n{}\nprimary stderr:\n{}\nstandby stdout:\n{}\nstandby stderr:\n{}",
            panic_payload_to_string(payload.as_ref()),
            artifacts.display(),
            ingress_snapshot.request_count,
            stopped.primary.stdout,
            stopped.primary.stderr,
            stopped.standby.stdout,
            stopped.standby.stderr,
        ),
    }

    for required in [
        "stage-deploy.stdout.log",
        "stage-deploy.stderr.log",
        "stage-deploy.meta.txt",
        "staged-bundle.path.txt",
        "staged-bundle.manifest.json",
        "database.json",
        "deploy-apply.stdout.log",
        "deploy-apply.stderr.log",
        "deploy-apply.meta.txt",
        "runtime.runtime-config.json",
        "scenario-meta.json",
        "health-primary-health.http",
        "health-primary-health.json",
        "health-standby-health.http",
        "health-standby-health.json",
        "cluster-status-primary-status.log",
        "cluster-status-primary-status.json",
        "cluster-status-standby-status.log",
        "cluster-status-standby-status.json",
        "public-ingress.meta.json",
        "public-ingress.log",
        "public-ingress.snapshot.json",
        "public-ingress.requests.json",
        "public-selected-list.http",
        "public-selected-list.json",
        "public-selected-list.request-summary.json",
        "public-selected-list.request-key.txt",
        "public-selected-list.request-key.json",
        "selected-route-direct-primary-record.log",
        "selected-route-direct-primary-record.json",
        "selected-route-direct-standby-record.log",
        "selected-route-direct-standby-record.json",
        "selected-route-direct-primary-diagnostics.log",
        "selected-route-direct-primary-diagnostics.json",
        "selected-route-direct-standby-diagnostics.log",
        "selected-route-direct-standby-diagnostics.json",
        "selected-route.primary-diagnostics.entries.json",
        "selected-route.standby-diagnostics.entries.json",
        "selected-route.summary.json",
        "runtime-primary.stdout.log",
        "runtime-primary.stderr.log",
        "runtime-primary.combined.log",
        "runtime-standby.stdout.log",
        "runtime-standby.stderr.log",
        "runtime-standby.combined.log",
    ] {
        assert!(
            artifacts.join(required).exists(),
            "missing retained direct-correlation artifact {} in {}",
            required,
            artifacts.display()
        );
    }
}

#[test]
fn m054_s02_response_header_helper_fails_closed_on_malformed_missing_empty_and_duplicate_headers() {
    assert_header_failure(
        &format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n[]"
        ),
        "missing response header `X-Mesh-Continuity-Request-Key`",
    );
    assert_header_failure(
        &format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n{CORRELATION_HEADER}:   \r\nConnection: close\r\n\r\n[]"
        ),
        "response header `X-Mesh-Continuity-Request-Key` should not be empty",
    );
    assert_header_failure(
        &format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n{CORRELATION_HEADER}: request-1\r\n{CORRELATION_HEADER}: request-2\r\nConnection: close\r\n\r\n[]"
        ),
        "duplicate response header `X-Mesh-Continuity-Request-Key`",
    );
    assert_header_failure(
        &format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n{CORRELATION_HEADER}: request-1"
        ),
        "raw response is missing HTTP header terminator",
    );
}

#[test]
fn m054_s02_route_continuity_summary_fails_closed_on_primary_standby_drift() {
    let public_request = sample_public_request();
    let primary_record = sample_route_record("http-route::Api.Todos.handle_list_todos::1");
    let standby_record = sample_route_record("http-route::Api.Todos.handle_list_todos::2");

    let panic = catch_unwind(AssertUnwindSafe(|| {
        public_ingress::build_route_continuity_summary(
            &public_request,
            &primary_record,
            &standby_record,
            deploy::LIST_ROUTE_RUNTIME_NAME,
        )
    }))
    .expect_err("primary/standby direct lookup drift should fail closed");
    let message = panic_payload_to_string(panic.as_ref());
    assert!(
        message.contains("route continuity field `request_key` drifted between primary and standby records"),
        "expected explicit primary/standby drift failure, got: {message}"
    );
}
