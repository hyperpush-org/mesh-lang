mod support;

use serde_json::{json, Value};
use std::path::PathBuf;
use support::m046_route_free as route_free;

const STARTUP_SOURCE_DECLARATION: &str = route_free::STARTUP_SOURCE_DECLARATION;
const STARTUP_RUNTIME_NAME_GUIDANCE: &str = route_free::STARTUP_RUNTIME_NAME_GUIDANCE;
const STARTUP_AUTOSTART_GUIDANCE: &str = route_free::STARTUP_AUTOSTART_GUIDANCE;

fn repo_root() -> PathBuf {
    route_free::repo_root()
}

fn artifact_dir(test_name: &str) -> PathBuf {
    route_free::artifact_dir("m045-s03", test_name)
}

fn assert_contains(path_label: &str, source: &str, needle: &str) {
    assert!(
        source.contains(needle),
        "expected {path_label} to contain {needle:?}, got:\n{source}"
    );
}

fn assert_omits(path_label: &str, source: &str, needle: &str) {
    assert!(
        !source.contains(needle),
        "expected {path_label} to omit {needle:?}, got:\n{source}"
    );
}

fn pending_record_matches(record: &Value) -> bool {
    record["request_key"].as_str() == Some("req-1")
        && record["attempt_id"].as_str() == Some("attempt-1")
        && record["phase"].as_str() == Some("submitted")
        && record["result"].as_str() == Some("pending")
        && record["owner_node"].as_str() == Some("primary@127.0.0.1:4370")
        && record["replica_node"].as_str() == Some("standby@[::1]:4370")
        && matches!(
            record["replica_status"].as_str(),
            Some("preparing" | "mirrored")
        )
        && record["cluster_role"].as_str() == Some("standby")
        && record["promotion_epoch"].as_u64() == Some(0)
        && record["execution_node"].as_str() == Some("")
}

#[test]
fn m045_s03_failover_helpers_accept_preparing_and_mirrored_pending_truth() {
    let preparing = json!({
        "request_key": "req-1",
        "attempt_id": "attempt-1",
        "phase": "submitted",
        "result": "pending",
        "owner_node": "primary@127.0.0.1:4370",
        "replica_node": "standby@[::1]:4370",
        "replica_status": "preparing",
        "cluster_role": "standby",
        "promotion_epoch": 0,
        "execution_node": ""
    });
    let mirrored = json!({
        "request_key": "req-1",
        "attempt_id": "attempt-1",
        "phase": "submitted",
        "result": "pending",
        "owner_node": "primary@127.0.0.1:4370",
        "replica_node": "standby@[::1]:4370",
        "replica_status": "mirrored",
        "cluster_role": "standby",
        "promotion_epoch": 0,
        "execution_node": ""
    });
    let completed = json!({
        "request_key": "req-1",
        "attempt_id": "attempt-1",
        "phase": "completed",
        "result": "succeeded",
        "owner_node": "primary@127.0.0.1:4370",
        "replica_node": "standby@[::1]:4370",
        "replica_status": "mirrored",
        "cluster_role": "standby",
        "promotion_epoch": 0,
        "execution_node": "primary@127.0.0.1:4370"
    });

    assert!(pending_record_matches(&preparing));
    assert!(pending_record_matches(&mirrored));
    assert!(!pending_record_matches(&completed));
}

#[test]
fn m045_s03_failover_helpers_reject_malformed_cluster_json() {
    let error = route_free::parse_json_stdout(b"{not-json}", "continuity response").unwrap_err();
    assert!(error.contains("continuity response returned invalid JSON"));
}

#[test]
fn m045_s03_failover_scaffold_binary_proves_runtime_truth_and_retains_artifacts() {
    let artifacts = artifact_dir("scaffold-failover-delegation-contract");
    let temp = tempfile::tempdir().expect("create scaffold tempdir");
    let project_dir =
        route_free::init_clustered_project(temp.path(), "scaffold-failover", &artifacts);

    let manifest = route_free::read_and_archive(
        &project_dir.join("mesh.toml"),
        &artifacts.join("package").join("mesh.toml"),
    );
    let main = route_free::read_and_archive(
        &project_dir.join("main.mpl"),
        &artifacts.join("package").join("main.mpl"),
    );
    let work = route_free::read_and_archive(
        &project_dir.join("work.mpl"),
        &artifacts.join("package").join("work.mpl"),
    );
    let readme = route_free::read_and_archive(
        &project_dir.join("README.md"),
        &artifacts.join("package").join("README.md"),
    );
    let harness_source = route_free::read_and_archive(
        &repo_root().join("compiler/meshc/tests/support/m046_route_free.rs"),
        &artifacts.join("contract").join("m046_route_free.rs"),
    );
    let equal_surface_rail = route_free::read_and_archive(
        &repo_root().join("compiler/meshc/tests/e2e_m046_s05.rs"),
        &artifacts.join("contract").join("e2e_m046_s05.rs"),
    );

    assert_contains("generated mesh.toml", &manifest, "[package]");
    assert_omits("generated mesh.toml", &manifest, "[cluster]");

    assert_contains("generated main.mpl", &main, "Node.start_from_env()");
    assert_contains("generated main.mpl", &main, "runtime bootstrap");
    assert_omits(
        "generated main.mpl",
        &main,
        "Continuity.submit_declared_work",
    );
    assert_omits("generated main.mpl", &main, "HTTP.serve");
    assert_omits("generated main.mpl", &main, "/health");
    assert_omits("generated main.mpl", &main, "/work");

    assert_contains("generated work.mpl", &work, STARTUP_SOURCE_DECLARATION);
    assert_omits("generated work.mpl", &work, "declared_work_runtime_name");
    assert_omits("generated work.mpl", &work, "clustered(work)");
    assert_omits(
        "generated work.mpl",
        &work,
        "Continuity.submit_declared_work",
    );
    assert_omits("generated work.mpl", &work, "Timer.sleep");

    assert_contains("generated README.md", &readme, "meshc cluster status");
    assert_contains("generated README.md", &readme, "meshc cluster continuity");
    assert_contains("generated README.md", &readme, "meshc cluster diagnostics");
    assert_contains(
        "generated README.md",
        &readme,
        STARTUP_RUNTIME_NAME_GUIDANCE,
    );
    assert_contains("generated README.md", &readme, STARTUP_AUTOSTART_GUIDANCE);
    assert_omits(
        "generated README.md",
        &readme,
        "declared_work_runtime_name()",
    );
    assert_omits("generated README.md", &readme, "clustered(work)");
    assert_omits("generated README.md", &readme, "/health");
    assert_omits("generated README.md", &readme, "/work");

    assert_contains(
        "compiler/meshc/tests/support/m046_route_free.rs",
        &harness_source,
        "archive_directory_tree(",
    );
    assert_contains(
        "compiler/meshc/tests/support/m046_route_free.rs",
        &harness_source,
        "init_clustered_project(",
    );
    assert_contains(
        "compiler/meshc/tests/support/m046_route_free.rs",
        &harness_source,
        "build_package_binary_to_output(",
    );
    assert_contains(
        "compiler/meshc/tests/support/m046_route_free.rs",
        &harness_source,
        "wait_for_runtime_name_discovered_with_label(",
    );
    assert_contains(
        "compiler/meshc/tests/support/m046_route_free.rs",
        &harness_source,
        "wait_for_continuity_record_completed(",
    );
    assert_contains(
        "compiler/meshc/tests/support/m046_route_free.rs",
        &harness_source,
        "wait_for_startup_diagnostics(",
    );

    assert_contains(
        "compiler/meshc/tests/e2e_m046_s05.rs",
        &equal_surface_rail,
        "route_free::init_clustered_project(",
    );
    assert_contains(
        "compiler/meshc/tests/e2e_m046_s05.rs",
        &equal_surface_rail,
        "route_free::build_package_binary_to_output(",
    );
    assert_contains(
        "compiler/meshc/tests/e2e_m046_s05.rs",
        &equal_surface_rail,
        "cluster-continuity-list-primary",
    );
    assert_contains(
        "compiler/meshc/tests/e2e_m046_s05.rs",
        &equal_surface_rail,
        "cluster-continuity-primary-completed",
    );
    assert_contains(
        "compiler/meshc/tests/e2e_m046_s05.rs",
        &equal_surface_rail,
        "cluster-diagnostics-primary",
    );
    assert_contains(
        "compiler/meshc/tests/e2e_m046_s05.rs",
        &equal_surface_rail,
        "generated-project/mesh.toml",
    );
    assert_contains(
        "compiler/meshc/tests/e2e_m046_s05.rs",
        &equal_surface_rail,
        "primary.combined.log",
    );
}
