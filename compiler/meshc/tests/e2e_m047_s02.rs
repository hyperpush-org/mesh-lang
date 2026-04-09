mod support;

use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use support::m046_route_free as route_free;

const SHARED_COOKIE: &str = "mesh-m047-s02-cli-cookie";
const DISCOVERY_SEED: &str = "localhost";
const DEFAULT_RUNTIME_NAME: &str = "Work.handle_submit";
const EXPLICIT_RUNTIME_NAME: &str = "Work.handle_retry";
const LEGACY_RUNTIME_NAME: &str = "Work.execute_declared_work";

struct SourceClusterRuntimeProject {
    _tempdir: tempfile::TempDir,
    project_dir: PathBuf,
    binary_path: PathBuf,
}

fn repo_root() -> PathBuf {
    route_free::repo_root()
}

fn artifact_dir(test_name: &str) -> PathBuf {
    route_free::artifact_dir("m047-s02", test_name)
}

fn package_manifest(name: &str) -> String {
    format!("[package]\nname = \"{name}\"\nversion = \"1.0.0\"\n")
}

fn ordinary_cluster_main_source() -> &'static str {
    "fn log_bootstrap(status :: BootstrapStatus) do\n  println(\"[m047-s02] runtime bootstrap mode=#{status.mode} node=#{status.node_name} cluster_port=#{status.cluster_port} discovery_seed=#{status.discovery_seed}\")\nend\n\nfn main() do\n  case Node.start_from_env() do\n    Ok(status) -> log_bootstrap(status)\n    Err(reason) -> println(\"[m047-s02] runtime bootstrap failed reason=#{reason}\")\n  end\nend\n"
}

fn ordinary_cluster_work_source() -> &'static str {
    "@cluster pub fn handle_submit() -> Int do\n  Timer.sleep(50)\n  1 + 1\nend\n\n@cluster(3) pub fn handle_retry() -> Int do\n  Timer.sleep(50)\n  2 + 1\nend\n\npub fn local_only(payload :: String) -> String do\n  payload\nend\n"
}

fn command_output_text(output: &Output) -> String {
    format!(
        "status: {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn write_project_sources(project_dir: &Path, manifest: &str, sources: &[(&str, &str)]) {
    fs::create_dir_all(project_dir).expect("failed to create temp project dir");
    fs::write(project_dir.join("mesh.toml"), manifest).expect("failed to write mesh.toml");
    for (path, content) in sources {
        fs::write(project_dir.join(path), content)
            .unwrap_or_else(|err| panic!("failed to write {path}: {err}"));
    }
}

fn build_source_cluster_runtime_project(
    name: &str,
    artifacts: &Path,
) -> (SourceClusterRuntimeProject, String) {
    route_free::ensure_mesh_rt_staticlib();
    let tempdir = tempfile::tempdir().expect("failed to create temp dir");
    let project_dir = tempdir.path().join("project");
    let output_dir = tempdir.path().join("out");
    fs::create_dir_all(&output_dir).expect("failed to create temp output dir");

    let manifest = package_manifest(name);
    let main = ordinary_cluster_main_source();
    let work = ordinary_cluster_work_source();
    write_project_sources(
        &project_dir,
        &manifest,
        &[("main.mpl", main), ("work.mpl", work)],
    );

    let binary_path = output_dir.join(name);
    let output = Command::new(route_free::meshc_bin())
        .current_dir(repo_root())
        .arg("build")
        .arg(&project_dir)
        .arg("--output")
        .arg(&binary_path)
        .arg("--emit-llvm")
        .output()
        .expect("failed to invoke meshc build for M047 runtime project");
    route_free::write_artifact(&artifacts.join("build.log"), command_output_text(&output));
    route_free::archive_directory_tree(&project_dir, &artifacts.join("project"));
    assert!(
        output.status.success(),
        "M047 ordinary source-cluster build should succeed:\n{}",
        command_output_text(&output)
    );
    assert!(
        binary_path.exists(),
        "meshc build reported success but temp binary is missing at {}",
        binary_path.display()
    );

    let llvm = route_free::read_and_archive(
        &output_dir.join(format!("{name}.ll")),
        &artifacts.join("project/output.ll"),
    );

    (
        SourceClusterRuntimeProject {
            _tempdir: tempdir,
            project_dir,
            binary_path,
        },
        llvm,
    )
}

fn main_wrapper_ir(llvm: &str) -> &str {
    let start = llvm
        .find("define i32 @main(")
        .expect("expected c-level main wrapper in emitted llvm");
    let rest = &llvm[start..];
    let end = rest
        .find("\n}")
        .map(|idx| idx + 2)
        .expect("expected end of main wrapper in emitted llvm");
    &rest[..end]
}

fn node_name(cluster_port: u16) -> String {
    format!("m047-s02@{}:{cluster_port}", route_free::LOOPBACK_V4)
}

fn assert_human_record_contains(
    human_output: &str,
    runtime_name: &str,
    replication_count: u64,
    phase: &str,
    result: &str,
) {
    assert!(
        human_output.lines().any(|line| {
            line.contains(&format!("declared_handler_runtime_name={runtime_name}"))
                && line.contains(&format!("replication_count={replication_count}"))
                && line.contains(&format!("phase={phase}"))
                && line.contains(&format!("result={result}"))
        }),
        "expected human continuity list to include runtime={runtime_name} replication_count={replication_count} phase={phase} result={result}, got:\n{human_output}"
    );
}

fn metadata_value<'a>(entry: &'a Value, key: &str) -> Option<&'a str> {
    entry["metadata"]
        .as_array()?
        .iter()
        .find(|item| item["key"].as_str() == Some(key))?
        .get("value")?
        .as_str()
}

#[test]
fn m047_s02_llvm_registration_keeps_generic_runtime_names_and_replication_count_markers() {
    let artifacts = artifact_dir("llvm-registration-markers");
    let (_project, llvm) =
        build_source_cluster_runtime_project("m047-source-cluster-ir", &artifacts);
    let main_ir = main_wrapper_ir(&llvm);

    assert!(
        llvm.contains("@mesh_register_declared_handler"),
        "expected declared-handler registration call in IR:\n{llvm}"
    );
    assert!(
        llvm.contains("@mesh_register_startup_work"),
        "expected startup registration call in IR:\n{llvm}"
    );
    assert!(
        llvm.contains("i64 2, ptr @__declared_work_work_handle_submit"),
        "expected default @cluster replication marker in IR:\n{llvm}"
    );
    assert!(
        llvm.contains("i64 3, ptr @__declared_work_work_handle_retry"),
        "expected explicit @cluster(3) replication marker in IR:\n{llvm}"
    );
    for (runtime_name, declared_marker, startup_marker) in [
        (
            DEFAULT_RUNTIME_NAME,
            "@declared_runtime_reg_Work_handle_submit",
            "@startup_work_reg_Work_handle_submit",
        ),
        (
            EXPLICIT_RUNTIME_NAME,
            "@declared_runtime_reg_Work_handle_retry",
            "@startup_work_reg_Work_handle_retry",
        ),
    ] {
        assert!(
            llvm.contains(runtime_name),
            "expected IR to preserve generic runtime name {runtime_name}:\n{llvm}"
        );
        assert!(
            main_ir.contains(declared_marker) && main_ir.contains(startup_marker),
            "expected main wrapper to register generic runtime name {runtime_name} through {declared_marker} / {startup_marker}:\n{main_ir}"
        );
    }
    assert!(
        !llvm.contains(LEGACY_RUNTIME_NAME),
        "ordinary @cluster IR must not fall back to the legacy runtime name {LEGACY_RUNTIME_NAME}:\n{llvm}"
    );
}

#[test]
fn m047_s02_cli_ordinary_cluster_continuity_surfaces_default_and_explicit_counts() {
    let artifacts = artifact_dir("cli-source-cluster-counts");
    let (project, _llvm) =
        build_source_cluster_runtime_project("m047-source-cluster-runtime", &artifacts);
    let cluster_port = route_free::dual_stack_cluster_port();
    let node_name = node_name(cluster_port);
    let spawned = route_free::spawn_route_free_runtime(
        &project.binary_path,
        &project.project_dir,
        &artifacts,
        "runtime",
        &node_name,
        cluster_port,
        "primary",
        0,
        SHARED_COOKIE,
        DISCOVERY_SEED,
    );

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        route_free::wait_for_cluster_status_matching(
            &artifacts,
            "cluster-status-ready",
            &node_name,
            "single-node startup readiness",
            SHARED_COOKIE,
            |json| {
                route_free::required_str(&json["membership"], "local_node") == node_name
                    && route_free::required_string_list(&json["membership"], "peer_nodes")
                        .is_empty()
                    && route_free::required_string_list(&json["membership"], "nodes")
                        == vec![node_name.clone()]
                    && route_free::required_str(&json["authority"], "cluster_role") == "primary"
                    && route_free::required_u64(&json["authority"], "promotion_epoch") == 0
                    && route_free::required_str(&json["authority"], "replication_health")
                        == "local_only"
            },
        );

        let list_json = route_free::wait_for_continuity_list_matching(
            &artifacts,
            "cluster-continuity-list-json",
            &node_name,
            "default and explicit @cluster runtime/count discovery",
            SHARED_COOKIE,
            |json| {
                let Some(default_record) =
                    route_free::find_record_for_runtime_name(json, DEFAULT_RUNTIME_NAME)
                else {
                    return false;
                };
                let Some(explicit_record) =
                    route_free::find_record_for_runtime_name(json, EXPLICIT_RUNTIME_NAME)
                else {
                    return false;
                };
                route_free::required_u64(default_record, "replication_count") == 2
                    && route_free::required_u64(explicit_record, "replication_count") == 3
                    && !route_free::required_str(default_record, "request_key").is_empty()
                    && !route_free::required_str(explicit_record, "request_key").is_empty()
            },
        );
        assert_eq!(route_free::required_u64(&list_json, "total_records"), 2);
        assert!(!route_free::required_bool(&list_json, "truncated"));
        assert_eq!(
            route_free::count_records_for_runtime_name(&list_json, DEFAULT_RUNTIME_NAME),
            1
        );
        assert_eq!(
            route_free::count_records_for_runtime_name(&list_json, EXPLICIT_RUNTIME_NAME),
            1
        );

        let default_list_record =
            route_free::record_for_runtime_name(&list_json, DEFAULT_RUNTIME_NAME);
        let explicit_list_record =
            route_free::record_for_runtime_name(&list_json, EXPLICIT_RUNTIME_NAME);
        let default_request_key = route_free::required_str(default_list_record, "request_key");
        let explicit_request_key = route_free::required_str(explicit_list_record, "request_key");
        assert_ne!(default_request_key, explicit_request_key);
        assert_eq!(
            route_free::required_str(default_list_record, "declared_handler_runtime_name"),
            DEFAULT_RUNTIME_NAME
        );
        assert_eq!(
            route_free::required_u64(default_list_record, "replication_count"),
            2
        );
        assert_eq!(
            route_free::required_str(explicit_list_record, "declared_handler_runtime_name"),
            EXPLICIT_RUNTIME_NAME
        );
        assert_eq!(
            route_free::required_u64(explicit_list_record, "replication_count"),
            3
        );
        assert!(
            !route_free::command_output_text(&route_free::run_meshc_cluster(
                &artifacts,
                "cluster-continuity-list-json-replay",
                &["cluster", "continuity", &node_name, "--json"],
                SHARED_COOKIE,
            ))
            .contains(LEGACY_RUNTIME_NAME),
            "JSON continuity replay should not mention the legacy runtime name"
        );

        let human_list = route_free::run_meshc_cluster(
            &artifacts,
            "cluster-continuity-list-human",
            &["cluster", "continuity", &node_name],
            SHARED_COOKIE,
        );
        assert!(
            human_list.status.success(),
            "human continuity list should succeed:\n{}",
            route_free::command_output_text(&human_list)
        );
        let human_list_stdout = route_free::command_output_text(&human_list);
        assert_human_record_contains(
            &human_list_stdout,
            DEFAULT_RUNTIME_NAME,
            2,
            "completed",
            "succeeded",
        );
        assert_human_record_contains(
            &human_list_stdout,
            EXPLICIT_RUNTIME_NAME,
            3,
            "rejected",
            "rejected",
        );
        assert!(
            !human_list_stdout.contains(LEGACY_RUNTIME_NAME),
            "human continuity list must not mention the legacy runtime name:\n{human_list_stdout}"
        );

        let default_json = route_free::wait_for_continuity_record_completed(
            &artifacts,
            "cluster-continuity-default-single-json",
            &node_name,
            &default_request_key,
            DEFAULT_RUNTIME_NAME,
            SHARED_COOKIE,
        );
        let default_record = &default_json["record"];
        assert_eq!(
            route_free::required_u64(default_record, "replication_count"),
            2
        );
        assert_eq!(
            route_free::required_str(default_record, "owner_node"),
            node_name
        );
        assert_eq!(route_free::required_str(default_record, "replica_node"), "");
        assert_eq!(
            route_free::required_str(default_record, "execution_node"),
            node_name
        );
        assert_eq!(
            route_free::required_str(default_record, "replica_status"),
            "unassigned"
        );
        assert_eq!(
            route_free::required_str(default_record, "phase"),
            "completed"
        );
        assert_eq!(
            route_free::required_str(default_record, "result"),
            "succeeded"
        );
        assert_eq!(route_free::required_str(default_record, "error"), "");

        let default_human = route_free::run_meshc_cluster(
            &artifacts,
            "cluster-continuity-default-single-human",
            &["cluster", "continuity", &node_name, &default_request_key],
            SHARED_COOKIE,
        );
        assert!(
            default_human.status.success(),
            "default single-record continuity human output should succeed:\n{}",
            route_free::command_output_text(&default_human)
        );
        let default_human_stdout = String::from_utf8_lossy(&default_human.stdout);
        assert!(
            default_human_stdout.contains(&format!(
                "declared_handler_runtime_name: {DEFAULT_RUNTIME_NAME}"
            )) && default_human_stdout.contains("replication_count: 2"),
            "default single-record continuity output should surface runtime name/count truth:\n{default_human_stdout}"
        );
        assert!(
            !default_human_stdout.contains(LEGACY_RUNTIME_NAME),
            "default single-record continuity output must not mention the legacy runtime name:\n{default_human_stdout}"
        );

        let explicit_json = route_free::wait_for_continuity_record_matching(
            &artifacts,
            "cluster-continuity-explicit-single-json",
            &node_name,
            &explicit_request_key,
            "rejected explicit @cluster(3) continuity record",
            SHARED_COOKIE,
            |json| {
                let record = &json["record"];
                route_free::required_str(record, "request_key") == explicit_request_key
                    && route_free::required_str(record, "declared_handler_runtime_name")
                        == EXPLICIT_RUNTIME_NAME
                    && route_free::required_u64(record, "replication_count") == 3
                    && route_free::required_str(record, "phase") == "rejected"
                    && route_free::required_str(record, "result") == "rejected"
                    && route_free::required_str(record, "error")
                        == "unsupported_replication_count:3"
            },
        );
        let explicit_record = &explicit_json["record"];
        assert_eq!(
            route_free::required_u64(explicit_record, "replication_count"),
            3
        );
        assert_eq!(
            route_free::required_str(explicit_record, "declared_handler_runtime_name"),
            EXPLICIT_RUNTIME_NAME
        );
        assert_eq!(
            route_free::required_str(explicit_record, "phase"),
            "rejected"
        );
        assert_eq!(
            route_free::required_str(explicit_record, "result"),
            "rejected"
        );
        assert_eq!(
            route_free::required_str(explicit_record, "error"),
            "unsupported_replication_count:3"
        );

        let explicit_human = route_free::run_meshc_cluster(
            &artifacts,
            "cluster-continuity-explicit-single-human",
            &["cluster", "continuity", &node_name, &explicit_request_key],
            SHARED_COOKIE,
        );
        assert!(
            explicit_human.status.success(),
            "explicit single-record continuity human output should succeed:\n{}",
            route_free::command_output_text(&explicit_human)
        );
        let explicit_human_stdout = String::from_utf8_lossy(&explicit_human.stdout);
        assert!(
            explicit_human_stdout.contains(&format!(
                "declared_handler_runtime_name: {EXPLICIT_RUNTIME_NAME}"
            )) && explicit_human_stdout.contains("replication_count: 3")
                && explicit_human_stdout.contains("phase: rejected")
                && explicit_human_stdout.contains("result: rejected")
                && explicit_human_stdout.contains("error: unsupported_replication_count:3"),
            "explicit single-record continuity output should surface rejected runtime/count truth:\n{explicit_human_stdout}"
        );
        assert!(
            !explicit_human_stdout.contains(LEGACY_RUNTIME_NAME),
            "explicit single-record continuity output must not mention the legacy runtime name:\n{explicit_human_stdout}"
        );

        let diagnostics = route_free::wait_for_diagnostics_matching(
            &artifacts,
            "cluster-diagnostics-json",
            &node_name,
            "startup completed + rejected diagnostics for ordinary @cluster functions",
            SHARED_COOKIE,
            |snapshot| {
                let Some(entries) = snapshot["entries"].as_array() else {
                    return false;
                };
                let default_completed = entries.iter().any(|entry| {
                    entry["request_key"].as_str() == Some(default_request_key.as_str())
                        && entry["transition"].as_str() == Some("startup_completed")
                        && metadata_value(entry, "runtime_name") == Some(DEFAULT_RUNTIME_NAME)
                });
                let explicit_rejected = entries.iter().any(|entry| {
                    entry["request_key"].as_str() == Some(explicit_request_key.as_str())
                        && entry["transition"].as_str() == Some("startup_rejected")
                        && entry["reason"].as_str() == Some("unsupported_replication_count:3")
                        && metadata_value(entry, "runtime_name") == Some(EXPLICIT_RUNTIME_NAME)
                });
                default_completed && explicit_rejected
            },
        );
        let entries = diagnostics["entries"]
            .as_array()
            .expect("diagnostics entries array");
        assert!(entries.iter().any(|entry| {
            entry["request_key"].as_str() == Some(default_request_key.as_str())
                && entry["transition"].as_str() == Some("startup_completed")
                && metadata_value(entry, "runtime_name") == Some(DEFAULT_RUNTIME_NAME)
        }));
        assert!(entries.iter().any(|entry| {
            entry["request_key"].as_str() == Some(explicit_request_key.as_str())
                && entry["transition"].as_str() == Some("startup_rejected")
                && entry["reason"].as_str() == Some("unsupported_replication_count:3")
                && metadata_value(entry, "runtime_name") == Some(EXPLICIT_RUNTIME_NAME)
        }));
    }));

    let logs = route_free::stop_process(spawned);
    route_free::write_artifact(&artifacts.join("runtime.combined.log"), &logs.combined);
    if let Err(payload) = result {
        panic!(
            "{}\nartifacts: {}\nstdout:\n{}\nstderr:\n{}",
            route_free::panic_payload_to_string(payload),
            artifacts.display(),
            logs.stdout,
            logs.stderr
        );
    }

    route_free::assert_log_absent(&logs, SHARED_COOKIE);
    route_free::assert_log_contains(&logs, "[m047-s02] runtime bootstrap mode=cluster");
    route_free::assert_log_contains(&logs, &format!("node={node_name}"));
    route_free::assert_log_contains(
        &logs,
        &format!(
            "[mesh-rt startup] transition=startup_completed runtime_name={DEFAULT_RUNTIME_NAME}"
        ),
    );
    route_free::assert_log_contains(
        &logs,
        &format!(
            "[mesh-rt startup] transition=startup_rejected runtime_name={EXPLICIT_RUNTIME_NAME}"
        ),
    );
    route_free::assert_log_contains(&logs, "unsupported_replication_count:3");
}
