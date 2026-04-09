mod support;

use serde_json::json;
use std::any::Any;
use std::fs;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::thread::{self, JoinHandle};

use support::m046_route_free as route_free;
use support::m053_todo_postgres_deploy as deploy;
use support::m054_public_ingress as public_ingress;

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
    route_free::artifact_dir("m054-s01", test_name)
}

struct OneShotBackend {
    port: u16,
    join: Option<JoinHandle<()>>,
}

impl OneShotBackend {
    fn start(raw_response: Vec<u8>) -> Self {
        let listener = TcpListener::bind((route_free::LOOPBACK_V4, 0))
            .expect("failed to bind one-shot backend listener");
        let port = listener
            .local_addr()
            .expect("one-shot backend listener missing local addr")
            .port();
        let join = thread::spawn(move || {
            let (mut stream, _) = listener
                .accept()
                .expect("one-shot backend listener failed to accept a connection");
            stream
                .set_read_timeout(Some(std::time::Duration::from_secs(2)))
                .expect("failed to set one-shot backend read timeout");
            let mut buffer = [0_u8; 4096];
            let _ = stream.read(&mut buffer);
            let _ = stream.write_all(&raw_response);
            let _ = stream.flush();
            let _ = stream.shutdown(Shutdown::Both);
        });
        Self {
            port,
            join: Some(join),
        }
    }
}

impl Drop for OneShotBackend {
    fn drop(&mut self) {
        let _ = TcpStream::connect((route_free::LOOPBACK_V4, self.port));
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

#[test]
fn m054_s01_staged_postgres_starter_serves_crud_through_one_public_ingress_url_and_retains_route_truth(
) {
    let test_name = "m054_s01_staged_postgres_starter_serves_crud_through_one_public_ingress_url_and_retains_route_truth";
    let base_database_url = required_database_url(test_name);
    let artifacts = artifact_dir("staged-postgres-public-ingress-truth");
    let workspace_dir = artifacts.join("workspace");
    fs::create_dir_all(&workspace_dir)
        .unwrap_or_else(|error| panic!("failed to create {}: {error}", workspace_dir.display()));

    let project_dir =
        deploy::init_postgres_todo_project(&workspace_dir, deploy::PACKAGE_NAME, &artifacts);
    let database = deploy::create_isolated_database(&base_database_url, &artifacts, "public");
    let bundle_dir = deploy::create_retained_bundle_dir("todo-postgres-public-ingress");
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

    deploy::write_json_artifact(
        &artifacts.join("scenario-meta.json"),
        &json!({
            "project_dir": project_dir.display().to_string(),
            "bundle_dir": bundle.bundle_dir.display().to_string(),
            "public_base_url": ingress.base_url(),
            "public_port": ingress.port(),
            "public_first_target": "standby",
            "startup_runtime_name": deploy::STARTUP_RUNTIME_NAME,
            "list_route_runtime_name": deploy::LIST_ROUTE_RUNTIME_NAME,
            "primary_node": runtime.primary.node_name.clone(),
            "standby_node": runtime.standby.node_name.clone(),
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

        let expected_membership = vec![
            runtime.primary.node_name.clone(),
            runtime.standby.node_name.clone(),
        ];
        let primary_status = route_free::wait_for_cluster_status_membership(
            &artifacts,
            "cluster-status-primary-status",
            &runtime.primary.node_name,
            std::slice::from_ref(&runtime.standby.node_name),
            &expected_membership,
            "primary",
            0,
            &["healthy"],
            &runtime.primary.cluster_cookie,
        );
        assert_eq!(
            route_free::required_str(&primary_status["authority"], "cluster_role"),
            "primary"
        );
        assert_eq!(
            route_free::required_str(&primary_status["authority"], "replication_health"),
            "healthy"
        );

        let startup_primary_list = deploy::wait_for_startup_runtime_discovered(
            &artifacts,
            "startup-selection-primary-startup-list",
            &runtime.primary.node_name,
            &runtime.primary.cluster_cookie,
        );
        let startup_primary_selection = route_free::record_for_runtime_name(
            &startup_primary_list,
            deploy::STARTUP_RUNTIME_NAME,
        );
        let startup_request_key =
            route_free::required_str(startup_primary_selection, "request_key").to_string();
        assert_eq!(
            route_free::required_str(startup_primary_selection, "owner_node"),
            runtime.primary.node_name
        );
        assert_eq!(
            route_free::required_str(startup_primary_selection, "replica_node"),
            runtime.standby.node_name
        );
        let startup_primary_record = deploy::wait_for_continuity_record_completed(
            &artifacts,
            "startup-completed-primary-record",
            &runtime.primary.node_name,
            &startup_request_key,
            deploy::STARTUP_RUNTIME_NAME,
            &runtime.primary.cluster_cookie,
        );
        assert_eq!(
            route_free::required_str(&startup_primary_record["record"], "request_key"),
            startup_request_key
        );
        let (startup_primary_diagnostics, startup_standby_diagnostics) =
            deploy::wait_for_startup_diagnostics_pair(&artifacts, &runtime, &startup_request_key);
        let mut startup_transitions = route_free::diagnostic_entries_for_request(
            &startup_primary_diagnostics,
            &startup_request_key,
        )
        .into_iter()
        .chain(route_free::diagnostic_entries_for_request(
            &startup_standby_diagnostics,
            &startup_request_key,
        ))
        .filter_map(|entry| entry["transition"].as_str())
        .collect::<Vec<_>>();
        startup_transitions.sort_unstable();
        assert!(startup_transitions.contains(&"startup_trigger"));
        assert!(startup_transitions.contains(&"startup_dispatch_window"));
        assert!(startup_transitions.contains(&"startup_completed"));

        let (before_primary, before_standby) = deploy::continuity_list_snapshot_pair(
            &artifacts,
            "continuity-before-selected-route",
            &runtime,
        );
        for (label, json) in [("primary", &before_primary), ("standby", &before_standby)] {
            assert_eq!(
                route_free::required_u64(json, "total_records"),
                1,
                "{label} continuity should only expose the runtime-owned startup record before the first public GET /todos"
            );
            assert!(!route_free::required_bool(json, "truncated"));
            assert_eq!(
                route_free::count_records_for_runtime_name(json, deploy::LIST_ROUTE_RUNTIME_NAME),
                0,
                "{label} continuity must not expose route traffic before the first public GET /todos"
            );
        }

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
        let selected_public_request =
            public_ingress::single_new_request(&public_before, &public_after);
        assert_eq!(selected_public_request.request_id, 1);
        assert_eq!(selected_public_request.method, "GET");
        assert_eq!(selected_public_request.path, "/todos");
        assert_eq!(selected_public_request.target_label, "standby");
        assert_eq!(selected_public_request.status_code, 200);
        assert!(selected_public_request.error.is_empty());
        deploy::write_json_artifact(
            &artifacts.join("public-selected-list.request-summary.json"),
            &selected_public_request,
        );

        let (_after_primary, primary_request_key) = deploy::wait_for_new_route_request_key(
            &artifacts,
            "selected-route-key-primary",
            &runtime.primary.node_name,
            &before_primary,
            &runtime.primary.cluster_cookie,
        );
        let (_after_standby, standby_request_key) = deploy::wait_for_new_route_request_key(
            &artifacts,
            "selected-route-key-standby",
            &runtime.standby.node_name,
            &before_standby,
            &runtime.standby.cluster_cookie,
        );
        assert_eq!(
            primary_request_key, standby_request_key,
            "public GET /todos continuity diff drifted across primary and standby"
        );
        deploy::write_json_artifact(
            &artifacts.join("selected-route.diff.json"),
            &json!({
                "public_request_id": selected_public_request.request_id,
                "public_target_label": selected_public_request.target_label,
                "public_target_node": selected_public_request.target_node,
                "request_key": primary_request_key,
            }),
        );

        let (primary_record_json, standby_record_json) =
            deploy::wait_for_continuity_record_completed_pair(
                &artifacts,
                "selected-route",
                &runtime,
                &primary_request_key,
                deploy::LIST_ROUTE_RUNTIME_NAME,
            );
        let summary = public_ingress::build_route_continuity_summary(
            &selected_public_request,
            &primary_record_json["record"],
            &standby_record_json["record"],
            deploy::LIST_ROUTE_RUNTIME_NAME,
        );
        assert_eq!(summary.public_target_label, "standby");
        assert_eq!(summary.ingress_node, selected_public_request.target_node);
        assert!(!summary.owner_node.is_empty());
        assert!(!summary.execution_node.is_empty());
        deploy::write_json_artifact(&artifacts.join("selected-route.summary.json"), &summary);

        let (primary_diagnostics, standby_diagnostics) = deploy::wait_for_request_diagnostics_pair(
            &artifacts,
            "selected-route",
            &runtime,
            &primary_request_key,
        );
        let primary_entries =
            route_free::diagnostic_entries_for_request(&primary_diagnostics, &primary_request_key);
        let standby_entries =
            route_free::diagnostic_entries_for_request(&standby_diagnostics, &primary_request_key);
        assert!(
            !primary_entries.is_empty(),
            "primary diagnostics should retain entries for the selected route request"
        );
        assert!(
            !standby_entries.is_empty(),
            "standby diagnostics should retain entries for the selected route request"
        );
        deploy::write_json_artifact(
            &artifacts.join("selected-route.primary-diagnostics.entries.json"),
            &primary_entries,
        );
        deploy::write_json_artifact(
            &artifacts.join("selected-route.standby-diagnostics.entries.json"),
            &standby_entries,
        );

        let smoke = deploy::run_staged_deploy_smoke_script(
            &bundle,
            &artifacts,
            "public-deploy-smoke",
            Some(ingress.base_url()),
            ingress.port(),
        );
        deploy::assert_phase_success(
            &smoke,
            "staged deploy-smoke.sh should succeed against one public ingress URL",
        );
        assert!(
            smoke.combined.contains("[deploy-smoke] health ready body="),
            "expected public deploy smoke to report health readiness, got:\n{}",
            smoke.combined
        );
        assert!(
            smoke
                .combined
                .contains("[deploy-smoke] CRUD smoke passed id="),
            "expected public deploy smoke success output, got:\n{}",
            smoke.combined
        );
        assert!(
            !smoke.combined.contains(&database.database_url),
            "public deploy smoke output must not echo DATABASE_URL\n{}",
            smoke.combined
        );

        let public_after_smoke = ingress.snapshot();
        assert!(
            public_after_smoke.request_count > public_after.request_count,
            "public deploy smoke should issue additional ingress requests"
        );
        assert_eq!(
            public_after_smoke.requests[0].target_label, "standby",
            "the first public request must hit standby-first ingress"
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
            assert!(
                ingress_snapshot.request_count >= 2,
                "expected the public ingress harness to retain the selected GET /todos plus the CRUD smoke flow"
            );
            deploy::assert_runtime_logs(&stopped.primary, &runtime.primary);
            deploy::assert_runtime_logs(&stopped.standby, &runtime.standby);
            deploy::assert_artifacts_redacted(&artifacts, &secret_values);
        }
        Err(payload) => panic!(
            "public ingress staged Postgres assertions failed: {}\nartifacts: {}\npublic_requests: {}\nprimary stdout:\n{}\nprimary stderr:\n{}\nstandby stdout:\n{}\nstandby stderr:\n{}",
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
        "startup-selection-primary-startup-list.log",
        "startup-selection-primary-startup-list.json",
        "startup-completed-primary-record.log",
        "startup-completed-primary-record.json",
        "cluster-diagnostics-primary.log",
        "cluster-diagnostics-primary.json",
        "cluster-diagnostics-standby.log",
        "cluster-diagnostics-standby.json",
        "continuity-before-selected-route-primary-continuity.log",
        "continuity-before-selected-route-primary-continuity.json",
        "continuity-before-selected-route-standby-continuity.log",
        "continuity-before-selected-route-standby-continuity.json",
        "public-ingress.meta.json",
        "public-ingress.log",
        "public-ingress.snapshot.json",
        "public-ingress.requests.json",
        "public-selected-list.http",
        "public-selected-list.json",
        "public-selected-list.request-summary.json",
        "selected-route-key-primary.log",
        "selected-route-key-primary.json",
        "selected-route-key-standby.log",
        "selected-route-key-standby.json",
        "selected-route-primary-record.log",
        "selected-route-primary-record.json",
        "selected-route-standby-record.log",
        "selected-route-standby-record.json",
        "selected-route-primary-diagnostics.log",
        "selected-route-primary-diagnostics.json",
        "selected-route-standby-diagnostics.log",
        "selected-route-standby-diagnostics.json",
        "selected-route.primary-diagnostics.entries.json",
        "selected-route.standby-diagnostics.entries.json",
        "selected-route.summary.json",
        "selected-route.diff.json",
        "public-deploy-smoke.stdout.log",
        "public-deploy-smoke.stderr.log",
        "public-deploy-smoke.meta.txt",
        "runtime-primary.stdout.log",
        "runtime-primary.stderr.log",
        "runtime-primary.combined.log",
        "runtime-standby.stdout.log",
        "runtime-standby.stderr.log",
        "runtime-standby.combined.log",
    ] {
        assert!(
            artifacts.join(required).exists(),
            "missing retained public-ingress artifact {} in {}",
            required,
            artifacts.display()
        );
    }
}

#[test]
fn m054_s01_postgres_readme_keeps_one_public_url_contract_and_sqlite_boundary() {
    let postgres_readme = fs::read_to_string(
        deploy::repo_root()
            .join("examples")
            .join("todo-postgres")
            .join("README.md"),
    )
    .expect("todo-postgres README should be readable");
    let sqlite_readme = fs::read_to_string(
        deploy::repo_root()
            .join("examples")
            .join("todo-sqlite")
            .join("README.md"),
    )
    .expect("todo-sqlite README should be readable");

    assert!(postgres_readme.contains("One public app URL may front multiple starter nodes"));
    assert!(postgres_readme.contains("proxy/platform ingress"));
    assert!(postgres_readme.contains("BASE_URL"));
    assert!(postgres_readme.contains("meshc cluster status"));
    assert!(postgres_readme.contains("meshc cluster continuity"));
    assert!(postgres_readme.contains("meshc cluster diagnostics"));
    assert!(postgres_readme.contains("frontend-aware node selection"));
    assert!(postgres_readme.contains("Fly-specific product contract"));
    assert!(postgres_readme.contains("meshc init --template todo-api --db sqlite"));
    assert!(!postgres_readme.contains("Fly.io"));

    assert!(sqlite_readme.contains(
        "there is no `work.mpl`, `HTTP.clustered(...)`, or `meshc cluster` story in this starter"
    ));
    assert!(sqlite_readme.contains("meshc init --template todo-api --db postgres"));
    assert!(!sqlite_readme.contains("One public app URL may front multiple starter nodes"));
    assert!(!sqlite_readme.contains("meshc cluster status"));
}

#[test]
fn m054_s01_public_ingress_fails_closed_on_invalid_target_config_and_truncated_backend_response() {
    let artifacts = artifact_dir("public-ingress-truncated-backend");

    let invalid_config_panic = catch_unwind(AssertUnwindSafe(|| {
        public_ingress::start_public_ingress(&artifacts, "public-ingress-invalid", Vec::new(), 0)
    }))
    .expect_err("empty target lists should fail closed");
    assert!(panic_payload_to_string(invalid_config_panic.as_ref())
        .contains("public ingress requires at least one backend target"));

    let backend = OneShotBackend::start(
        b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\nConnection: close\r\n\r\n[]".to_vec(),
    );
    let ingress = public_ingress::start_public_ingress(
        &artifacts,
        "public-ingress",
        vec![public_ingress::PublicIngressTarget {
            label: "truncated-backend".to_string(),
            node_name: "truncated-backend@127.0.0.1:4370".to_string(),
            host: route_free::LOOPBACK_V4.to_string(),
            port: backend.port,
        }],
        0,
    );

    let before = ingress.snapshot();
    let response = deploy::send_http_request(ingress.port(), "GET", "/todos", None)
        .unwrap_or_else(|error| panic!("GET /todos via truncated public ingress failed: {error}"));
    assert_eq!(response.status_code, 502);
    assert!(
        response.raw.contains("truncated response body"),
        "expected an explicit truncated-response error, got: {}",
        response.raw
    );

    let after = ingress.stop();
    let record = public_ingress::single_new_request(&before, &after);
    assert_eq!(record.target_label, "truncated-backend");
    assert_eq!(record.status_code, 502);
    assert!(
        record.error.contains("truncated response body"),
        "expected retained ingress error to mention the truncated backend response, got: {}",
        record.error
    );
}

#[test]
fn m054_s01_public_json_and_continuity_summary_fail_closed_on_non_json_and_missing_route_fields() {
    let artifacts = artifact_dir("public-ingress-non-json-and-missing-fields");
    let backend = OneShotBackend::start(
        b"HTTP/1.1 200 OK\r\nContent-Length: 8\r\nContent-Type: text/plain; charset=utf-8\r\nConnection: close\r\n\r\nnot-json"
            .to_vec(),
    );
    let ingress = public_ingress::start_public_ingress(
        &artifacts,
        "public-ingress",
        vec![public_ingress::PublicIngressTarget {
            label: "plain-text-backend".to_string(),
            node_name: "plain-text-backend@127.0.0.1:4370".to_string(),
            host: route_free::LOOPBACK_V4.to_string(),
            port: backend.port,
        }],
        0,
    );

    let before = ingress.snapshot();
    let response = deploy::send_http_request(ingress.port(), "GET", "/todos", None)
        .unwrap_or_else(|error| panic!("GET /todos via non-json public ingress failed: {error}"));
    let non_json_panic = catch_unwind(AssertUnwindSafe(|| {
        deploy::json_response_snapshot(
            &artifacts,
            "public-non-json",
            &response,
            200,
            "GET /todos via non-json public ingress",
            &[],
        )
    }))
    .expect_err("non-JSON public responses should fail closed");
    let non_json_message = panic_payload_to_string(non_json_panic.as_ref());
    assert!(
        non_json_message.contains("expected JSON body"),
        "expected explicit non-JSON failure, got: {}",
        non_json_message
    );

    let after = ingress.stop();
    let public_request = public_ingress::single_new_request(&before, &after);
    assert_eq!(public_request.method, "GET");
    assert_eq!(public_request.path, "/todos");
    assert_eq!(public_request.status_code, 200);
    assert!(public_request.error.is_empty());

    let malformed_primary = json!({
        "request_key": "http-route::Api.Todos.handle_list_todos::1",
        "attempt_id": "attempt-0",
        "declared_handler_runtime_name": deploy::LIST_ROUTE_RUNTIME_NAME,
        "replication_count": 1,
        "owner_node": "todo-postgres-primary@127.0.0.1:4370",
        "replica_node": "",
        "execution_node": "todo-postgres-primary@127.0.0.1:4370",
        "phase": "completed",
        "result": "succeeded",
        "error": ""
    });
    let malformed_standby = malformed_primary.clone();

    let missing_field_panic = catch_unwind(AssertUnwindSafe(|| {
        public_ingress::build_route_continuity_summary(
            &public_request,
            &malformed_primary,
            &malformed_standby,
            deploy::LIST_ROUTE_RUNTIME_NAME,
        )
    }))
    .expect_err("missing route fields should fail closed during continuity summary extraction");
    let missing_field_message = panic_payload_to_string(missing_field_panic.as_ref());
    assert!(
        missing_field_message.contains("missing string field `ingress_node`"),
        "expected explicit missing-field failure, got: {}",
        missing_field_message
    );
}
