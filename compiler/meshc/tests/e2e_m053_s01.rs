mod support;

use serde_json::Value;
use std::any::Any;
use std::fs;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::Path;
use tempfile::NamedTempFile;

use support::m046_route_free as route_free;
use support::m053_todo_postgres_deploy as deploy;

fn panic_payload_to_string(payload: Box<dyn Any + Send>) -> String {
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

fn read_source_file(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn assert_source_contains_all(path: &Path, needles: &[&str]) {
    let source = read_source_file(path);
    for needle in needles {
        assert!(
            source.contains(needle),
            "expected {} to contain `{}` but it was missing",
            path.display(),
            needle
        );
    }
}

fn assert_source_omits_all(path: &Path, needles: &[&str]) {
    let source = read_source_file(path);
    for needle in needles {
        assert!(
            !source.contains(needle),
            "expected {} to omit `{}` but it was still present",
            path.display(),
            needle
        );
    }
}

fn assert_source_order(path: &Path, needles: &[&str]) {
    let source = read_source_file(path);
    let mut current_index = 0usize;
    for needle in needles {
        let local_index = source[current_index..].find(needle).unwrap_or_else(|| {
            panic!(
                "expected {} to contain `{}` after byte {}",
                path.display(),
                needle,
                current_index
            )
        });
        current_index += local_index + needle.len();
    }
}

fn assert_startup_record(record: &Value, node_name: &str, request_key: &str) {
    assert_eq!(route_free::required_str(record, "request_key"), request_key);
    assert_eq!(
        route_free::required_str(record, "declared_handler_runtime_name"),
        deploy::STARTUP_RUNTIME_NAME
    );
    assert_eq!(route_free::required_str(record, "owner_node"), node_name);
    assert_eq!(
        route_free::required_str(record, "execution_node"),
        node_name
    );
    assert_eq!(route_free::required_str(record, "replica_node"), "");
    assert_eq!(
        route_free::required_str(record, "replica_status"),
        "unassigned"
    );
    assert_eq!(route_free::required_u64(record, "replication_count"), 2);
    assert_eq!(route_free::required_str(record, "phase"), "completed");
    assert_eq!(route_free::required_str(record, "result"), "succeeded");
    assert_eq!(route_free::required_str(record, "error"), "");
}

fn assert_clustered_route_record(record: &Value, node_name: &str, request_key: &str) {
    assert_eq!(route_free::required_str(record, "request_key"), request_key);
    assert_eq!(
        route_free::required_str(record, "declared_handler_runtime_name"),
        deploy::LIST_ROUTE_RUNTIME_NAME
    );
    assert_eq!(route_free::required_u64(record, "replication_count"), 1);
    assert_eq!(route_free::required_str(record, "phase"), "completed");
    assert_eq!(route_free::required_str(record, "result"), "succeeded");
    assert_eq!(route_free::required_str(record, "cluster_role"), "primary");
    assert_eq!(route_free::required_u64(record, "promotion_epoch"), 0);
    assert_eq!(
        route_free::required_str(record, "replication_health"),
        "local_only"
    );
    assert_eq!(route_free::required_str(record, "ingress_node"), node_name);
    assert_eq!(route_free::required_str(record, "owner_node"), node_name);
    assert_eq!(
        route_free::required_str(record, "execution_node"),
        node_name
    );
    assert_eq!(route_free::required_str(record, "replica_node"), "");
    assert_eq!(
        route_free::required_str(record, "replica_status"),
        "unassigned"
    );
    assert_eq!(route_free::required_str(record, "error"), "");
    assert!(!route_free::required_str(record, "attempt_id").is_empty());
    assert!(!route_free::required_str(record, "payload_hash").is_empty());
    assert!(!route_free::required_bool(record, "routed_remotely"));
    assert!(route_free::required_bool(record, "fell_back_locally"));
}

#[test]
fn m053_s01_postgres_staged_deploy_bundle_boots_serves_crud_and_answers_cluster_inspection() {
    let test_name =
        "m053_s01_postgres_staged_deploy_bundle_boots_serves_crud_and_answers_cluster_inspection";
    let base_database_url = required_database_url(test_name);
    let artifacts = deploy::artifact_dir("todo-postgres-staged-deploy-truth");
    let workspace_dir = artifacts.join("workspace");
    fs::create_dir_all(&workspace_dir)
        .unwrap_or_else(|error| panic!("failed to create {}: {error}", workspace_dir.display()));

    let project_dir =
        deploy::init_postgres_todo_project(&workspace_dir, deploy::PACKAGE_NAME, &artifacts);
    let database = deploy::create_isolated_database(&base_database_url, &artifacts, "staged");
    let secret_values = [base_database_url.as_str(), database.database_url.as_str()];

    let bundle_dir = deploy::create_retained_bundle_dir("todo-postgres-staged-bundle");
    let stage =
        deploy::run_stage_deploy_script(&project_dir, &bundle_dir, &artifacts, "stage-deploy");
    deploy::assert_phase_success(&stage, "generated Postgres stage-deploy.sh should succeed");
    assert!(
        stage.combined.contains("[stage-deploy] staged layout"),
        "expected staged layout output, got:\n{}",
        stage.combined
    );
    assert!(
        stage.combined.contains("[stage-deploy] bundle ready dir="),
        "expected staged bundle ready output, got:\n{}",
        stage.combined
    );
    assert!(
        !project_dir.join(deploy::PACKAGE_NAME).exists(),
        "stage-deploy should keep source tree free of in-place binaries"
    );
    assert!(
        !project_dir.join("output").exists(),
        "stage-deploy should not create a default output binary inside the source tree"
    );

    let bundle = deploy::inspect_staged_bundle(&bundle_dir, &artifacts);
    assert_eq!(
        fs::read_to_string(&bundle.pointer_path)
            .unwrap_or_else(|error| panic!(
                "failed to read {}: {error}",
                bundle.pointer_path.display()
            ))
            .trim(),
        bundle.bundle_dir.display().to_string()
    );

    let apply = deploy::run_staged_apply_deploy_migrations_script(
        &bundle,
        &artifacts,
        "deploy-apply",
        Some(&database.database_url),
    );
    deploy::assert_phase_success(&apply, "staged apply-deploy-migrations.sh should succeed");
    assert!(
        apply.combined.contains("[deploy-apply] sql artifact="),
        "expected staged apply artifact output, got:\n{}",
        apply.combined
    );
    assert!(
        apply
            .combined
            .contains("[deploy-apply] schema ready table=todos index=idx_todos_created_at"),
        "expected staged apply ready output, got:\n{}",
        apply.combined
    );
    assert!(
        !apply.combined.contains(&database.database_url),
        "staged apply output must not echo DATABASE_URL\n{}",
        apply.combined
    );

    let runtime_config =
        deploy::default_runtime_config(deploy::PACKAGE_NAME, &database.database_url);
    let spawned = deploy::spawn_staged_todo_app(&bundle, &artifacts, "runtime", &runtime_config);

    let run_result = catch_unwind(AssertUnwindSafe(|| {
        let health = deploy::wait_for_health(&runtime_config, &artifacts, "health", &secret_values);
        assert_eq!(health["status"].as_str(), Some("ok"));
        assert_eq!(health["db_backend"].as_str(), Some("postgres"));
        assert_eq!(health["migration_strategy"].as_str(), Some("meshc migrate"));
        assert_eq!(
            health["clustered_handler"].as_str(),
            Some(deploy::STARTUP_RUNTIME_NAME)
        );
        assert_eq!(
            health["rate_limit_window_seconds"].as_i64(),
            Some(runtime_config.rate_limit_window_seconds as i64)
        );
        assert_eq!(
            health["rate_limit_max_requests"].as_i64(),
            Some(runtime_config.rate_limit_max_requests as i64)
        );
        assert!(health.get("database_url").is_none());
        assert!(health.get("db_path").is_none());

        let status = deploy::wait_for_single_node_cluster_status(
            &artifacts,
            "cluster-status",
            &runtime_config.node_name,
            &runtime_config.cluster_cookie,
        );
        assert_eq!(
            route_free::required_str(&status["authority"], "cluster_role"),
            "primary"
        );
        assert_eq!(
            route_free::required_u64(&status["authority"], "promotion_epoch"),
            0
        );
        assert_eq!(
            route_free::required_str(&status["authority"], "replication_health"),
            "local_only"
        );

        let startup_list = deploy::wait_for_startup_runtime_discovered(
            &artifacts,
            "cluster-continuity-startup-list",
            &runtime_config.node_name,
            &runtime_config.cluster_cookie,
        );
        assert_eq!(route_free::required_u64(&startup_list, "total_records"), 1);
        assert!(!route_free::required_bool(&startup_list, "truncated"));
        assert_eq!(
            route_free::count_records_for_runtime_name(
                &startup_list,
                deploy::LIST_ROUTE_RUNTIME_NAME
            ),
            0,
            "continuity should start with only the runtime-owned startup record"
        );
        let startup_request_key = route_free::required_str(
            route_free::record_for_runtime_name(&startup_list, deploy::STARTUP_RUNTIME_NAME),
            "request_key",
        );
        assert_eq!(
            startup_request_key,
            format!("startup::{}", deploy::STARTUP_RUNTIME_NAME)
        );

        let startup_record = deploy::wait_for_continuity_record_completed(
            &artifacts,
            "cluster-continuity-startup-record",
            &runtime_config.node_name,
            &startup_request_key,
            deploy::STARTUP_RUNTIME_NAME,
            &runtime_config.cluster_cookie,
        );
        assert_startup_record(
            &startup_record["record"],
            &runtime_config.node_name,
            &startup_request_key,
        );

        let diagnostics = deploy::wait_for_startup_diagnostics(
            &artifacts,
            "cluster-diagnostics",
            &runtime_config.node_name,
            &startup_request_key,
            &runtime_config.cluster_cookie,
        );
        let startup_entries =
            route_free::diagnostic_entries_for_request(&diagnostics, &startup_request_key);
        let startup_transitions: Vec<_> = startup_entries
            .iter()
            .filter_map(|entry| entry["transition"].as_str())
            .collect();
        assert!(startup_transitions.contains(&"startup_trigger"));
        assert!(startup_transitions.contains(&"startup_completed"));
        assert!(
            startup_transitions.contains(&"startup_dispatch_window")
                || route_free::required_str(&startup_record["record"], "replica_status")
                    == "unassigned"
        );

        let empty_list = deploy::json_response_snapshot(
            &artifacts,
            "todos-empty",
            &deploy::send_http_request(runtime_config.http_port, "GET", "/todos", None)
                .unwrap_or_else(|error| {
                    panic!("GET /todos failed on {}: {error}", runtime_config.http_port)
                }),
            200,
            "GET /todos before staged smoke",
            &secret_values,
        );
        assert!(
            empty_list.as_array().is_some_and(|items| items.is_empty()),
            "expected an empty todo list before staged smoke, got: {empty_list}"
        );

        let (_route_list, route_request_key) = deploy::wait_for_new_route_request_key(
            &artifacts,
            "cluster-continuity-route-list",
            &runtime_config.node_name,
            &startup_list,
            &runtime_config.cluster_cookie,
        );
        let route_record = deploy::wait_for_continuity_record_completed(
            &artifacts,
            "cluster-continuity-route-record",
            &runtime_config.node_name,
            &route_request_key,
            deploy::LIST_ROUTE_RUNTIME_NAME,
            &runtime_config.cluster_cookie,
        );
        assert_clustered_route_record(
            &route_record["record"],
            &runtime_config.node_name,
            &route_request_key,
        );

        let base_url = format!("http://127.0.0.1:{}", runtime_config.http_port);
        let smoke = deploy::run_staged_deploy_smoke_script(
            &bundle,
            &artifacts,
            "deploy-smoke",
            Some(&base_url),
            runtime_config.http_port,
        );
        deploy::assert_phase_success(&smoke, "staged deploy-smoke.sh should succeed");
        assert!(
            smoke.combined.contains("[deploy-smoke] health ready body="),
            "expected staged deploy smoke health output, got:\n{}",
            smoke.combined
        );
        assert!(
            smoke
                .combined
                .contains("[deploy-smoke] creating todo via POST"),
            "expected staged deploy smoke create step, got:\n{}",
            smoke.combined
        );
        assert!(
            smoke.combined.contains("[deploy-smoke] toggling todo id="),
            "expected staged deploy smoke toggle step, got:\n{}",
            smoke.combined
        );
        assert!(
            smoke
                .combined
                .contains("[deploy-smoke] CRUD smoke passed id="),
            "expected staged deploy smoke success output, got:\n{}",
            smoke.combined
        );
        assert!(
            !smoke.combined.contains(&database.database_url),
            "staged deploy smoke output must not echo DATABASE_URL\n{}",
            smoke.combined
        );

        let final_list = deploy::json_response_snapshot(
            &artifacts,
            "todos-after-smoke",
            &deploy::send_http_request(runtime_config.http_port, "GET", "/todos", None)
                .unwrap_or_else(|error| {
                    panic!(
                        "GET /todos after staged smoke failed on {}: {error}",
                        runtime_config.http_port
                    )
                }),
            200,
            "GET /todos after staged smoke",
            &secret_values,
        );
        assert!(
            final_list.as_array().is_some_and(|items| items.is_empty()),
            "expected staged CRUD smoke to leave an empty todo list, got: {final_list}"
        );

        let continuity_after_smoke = deploy::continuity_list_snapshot(
            &artifacts,
            "cluster-continuity-after-smoke",
            &runtime_config.node_name,
            &runtime_config.cluster_cookie,
        );
        assert!(
            route_free::count_records_for_runtime_name(&continuity_after_smoke, deploy::LIST_ROUTE_RUNTIME_NAME) >= 1,
            "expected staged deploy replay to retain at least one clustered route continuity record, got: {continuity_after_smoke}"
        );
    }));

    let logs = deploy::stop_todo_app(spawned, &secret_values);
    deploy::write_artifact(&artifacts.join("runtime.combined.log"), &logs.combined);

    match run_result {
        Ok(()) => {
            deploy::assert_runtime_logs(&logs, &runtime_config);
            deploy::assert_artifacts_redacted(&artifacts, &secret_values);
        }
        Err(payload) => panic!(
            "staged Postgres deploy assertions failed: {}\nartifacts: {}\nstdout: {}\nstderr: {}\nstdout_log: {}\nstderr_log: {}",
            panic_payload_to_string(payload),
            artifacts.display(),
            logs.stdout,
            logs.stderr,
            logs.stdout_path.display(),
            logs.stderr_path.display()
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
        "health.http",
        "health.json",
        "cluster-status.log",
        "cluster-status.json",
        "cluster-continuity-startup-list.log",
        "cluster-continuity-startup-list.json",
        "cluster-continuity-startup-record.log",
        "cluster-continuity-startup-record.json",
        "cluster-continuity-route-list.log",
        "cluster-continuity-route-list.json",
        "cluster-continuity-route-record.log",
        "cluster-continuity-route-record.json",
        "cluster-diagnostics.log",
        "cluster-diagnostics.json",
        "todos-empty.http",
        "todos-empty.json",
        "deploy-smoke.stdout.log",
        "deploy-smoke.stderr.log",
        "deploy-smoke.meta.txt",
        "todos-after-smoke.http",
        "todos-after-smoke.json",
        "cluster-continuity-after-smoke.log",
        "cluster-continuity-after-smoke.json",
        "runtime.stdout.log",
        "runtime.stderr.log",
        "runtime.combined.log",
    ] {
        assert!(
            artifacts.join(required).exists(),
            "missing retained staged deploy artifact {} in {}",
            required,
            artifacts.display()
        );
    }
}

#[test]
fn m053_s01_postgres_stage_deploy_rejects_invalid_bundle_path() {
    let artifacts = deploy::artifact_dir("todo-postgres-invalid-bundle-path");
    let workspace_dir = artifacts.join("workspace");
    fs::create_dir_all(&workspace_dir)
        .unwrap_or_else(|error| panic!("failed to create {}: {error}", workspace_dir.display()));

    let project_dir =
        deploy::init_postgres_todo_project(&workspace_dir, deploy::PACKAGE_NAME, &artifacts);
    let bundle_file = NamedTempFile::new().expect("failed to create temp bundle file");
    let run = deploy::run_stage_deploy_script(
        &project_dir,
        bundle_file.path(),
        &artifacts,
        "stage-deploy-invalid-path",
    );

    assert!(
        !run.status.success(),
        "stage-deploy should fail closed when the bundle path is a file:\n{}",
        run.combined
    );
    assert!(
        run.combined
            .contains("[stage-deploy] bundle path exists but is not a directory:"),
        "expected invalid bundle-path error, got:\n{}",
        run.combined
    );
    deploy::assert_artifacts_redacted(&artifacts, &[]);
}

#[test]
fn m053_s01_postgres_staged_scripts_and_cluster_cli_fail_closed_on_bad_inputs() {
    let artifacts = deploy::artifact_dir("todo-postgres-staged-fail-closed");
    let workspace_dir = artifacts.join("workspace");
    fs::create_dir_all(&workspace_dir)
        .unwrap_or_else(|error| panic!("failed to create {}: {error}", workspace_dir.display()));

    let project_dir =
        deploy::init_postgres_todo_project(&workspace_dir, deploy::PACKAGE_NAME, &artifacts);
    let bundle_dir = deploy::create_retained_bundle_dir("todo-postgres-staged-fail-closed");
    let stage =
        deploy::run_stage_deploy_script(&project_dir, &bundle_dir, &artifacts, "stage-deploy");
    deploy::assert_phase_success(
        &stage,
        "generated Postgres stage-deploy.sh should succeed before fail-closed script checks",
    );
    let bundle = deploy::inspect_staged_bundle(&bundle_dir, &artifacts);

    let apply_missing_env = deploy::run_staged_apply_deploy_migrations_script(
        &bundle,
        &artifacts,
        "deploy-apply-missing-database-url",
        None,
    );
    assert!(
        !apply_missing_env.status.success(),
        "apply-deploy-migrations.sh should fail closed without DATABASE_URL:\n{}",
        apply_missing_env.combined
    );
    assert!(
        apply_missing_env
            .combined
            .contains("[deploy-apply] DATABASE_URL must be set"),
        "expected explicit missing-DATABASE_URL error, got:\n{}",
        apply_missing_env.combined
    );

    let smoke_bad_base_url = deploy::run_staged_deploy_smoke_script(
        &bundle,
        &artifacts,
        "deploy-smoke-malformed-base-url",
        Some("127.0.0.1:8080"),
        8080,
    );
    assert!(
        !smoke_bad_base_url.status.success(),
        "deploy-smoke.sh should fail closed for malformed BASE_URL:\n{}",
        smoke_bad_base_url.combined
    );
    assert!(
        smoke_bad_base_url
            .combined
            .contains("[deploy-smoke] BASE_URL must start with http:// or https://"),
        "expected explicit malformed-BASE_URL error, got:\n{}",
        smoke_bad_base_url.combined
    );

    let node_name = format!(
        "todo-postgres@{}:{}",
        route_free::LOOPBACK_V4,
        postgres_unused_port()
    );
    let cluster_output = route_free::run_meshc_cluster(
        &artifacts,
        "cluster-status-not-ready",
        &["cluster", "status", &node_name, "--json"],
        "m053-s01-bad-inputs-cookie",
    );
    assert!(
        !cluster_output.status.success(),
        "meshc cluster status should fail closed against a non-ready node"
    );
    deploy::assert_artifacts_redacted(&artifacts, &[]);
}

#[test]
fn m053_s01_retained_verifier_replays_starter_rails_and_publishes_bundle_markers() {
    let verifier_path = deploy::repo_root().join("scripts/verify-m053-s01.sh");

    assert_source_contains_all(
        &verifier_path,
        &[
            "m053-s01-db-env-preflight",
            "m053-s01-scaffold-rail",
            "m053-s01-mesh-rt-staticlib",
            "m053-s01-example-e2e",
            "m053-s01-example-parity",
            "m053-s01-staged-deploy-e2e",
            "m053-s01-retain-artifacts",
            "m053-s01-retain-staged-bundle",
            "m053-s01-redaction-drift",
            "m053-s01-bundle-shape",
            "DATABASE_URL must be set for scripts/verify-m053-s01.sh",
            "DATABASE_URL must start with postgres:// or postgresql://",
            "cargo test -p mesh-pkg m049_s01_postgres_scaffold_ -- --nocapture",
            "cargo build -q -p mesh-rt",
            "cargo test -p meshc --test e2e_m049_s03 -- --nocapture",
            "node scripts/tests/verify-m049-s03-materialize-examples.mjs --check",
            "cargo test -p meshc --test e2e_m053_s01 -- --nocapture",
            "retained-m053-s01-artifacts",
            "retained-staged-bundle",
            "retained-m053-s01-artifacts.manifest.txt",
            "retained-staged-bundle.manifest.json",
            "latest-proof-bundle.txt",
            "status.txt",
            "current-phase.txt",
            "phase-report.txt",
            "full-contract.log",
            "verify-m053-s01.sh",
            "todo-postgres.README.md",
            "verify-m053-s01: ok",
        ],
    );

    assert_source_order(
        &verifier_path,
        &[
            "begin_phase m053-s01-db-env-preflight",
            "run_expect_success m053-s01-scaffold-rail",
            "run_expect_success m053-s01-mesh-rt-staticlib",
            "run_expect_success m053-s01-example-e2e",
            "run_expect_success m053-s01-example-parity",
            "run_expect_success_with_database_url m053-s01-staged-deploy-e2e",
            "copy_new_prefixed_artifacts_or_fail",
            "copy_staged_bundle_or_fail",
            "assert_no_secret_leaks m053-s01-redaction-drift",
            "assert_retained_bundle_shape",
            "echo \"verify-m053-s01: ok\"",
        ],
    );
}

#[test]
fn m053_s01_retained_verifier_avoids_env_sourcing_and_later_slice_scope() {
    let verifier_path = deploy::repo_root().join("scripts/verify-m053-s01.sh");

    assert_source_omits_all(
        &verifier_path,
        &[
            "source \"$ROOT_DIR/.env\"",
            "cat .env",
            "echo \"$DATABASE_URL\"",
            "printf '%s\n' \"$DATABASE_URL\"",
            "packages-site",
            "verify-production-proof-surface",
            "fly.dev",
            "npm --prefix website run build",
            "bash scripts/verify-m051-s02.sh",
            "expected success within ${timeout_secs}s",
        ],
    );
}

fn postgres_unused_port() -> u16 {
    support::m049_todo_postgres_scaffold::unused_port()
}
