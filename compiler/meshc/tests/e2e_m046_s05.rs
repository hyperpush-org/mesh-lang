mod support;

use std::fs;
use std::path::{Path, PathBuf};
use support::m046_route_free as route_free;

const SHARED_COOKIE: &str = "mesh-m046-s05-cli-cookie";
const DISCOVERY_SEED: &str = "localhost";
const STARTUP_RUNTIME_NAME: &str = route_free::STARTUP_RUNTIME_NAME;
const STARTUP_SOURCE_DECLARATION: &str = route_free::STARTUP_SOURCE_DECLARATION;
const STARTUP_RUNTIME_NAME_GUIDANCE: &str = route_free::STARTUP_RUNTIME_NAME_GUIDANCE;
const STARTUP_AUTOSTART_GUIDANCE: &str = route_free::STARTUP_AUTOSTART_GUIDANCE;

struct ScaffoldSources {
    manifest: String,
    main: String,
    work: String,
    readme: String,
    tiny_cluster_work: String,
    cluster_proof_work: String,
}

fn repo_root() -> PathBuf {
    route_free::repo_root()
}

fn artifact_dir(test_name: &str) -> PathBuf {
    route_free::artifact_dir("m046-s05", test_name)
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

fn load_scaffold_sources(project_dir: &Path, artifacts: &Path) -> ScaffoldSources {
    let package_artifacts = artifacts.join("package");
    let reference_artifacts = artifacts.join("references");
    ScaffoldSources {
        manifest: route_free::read_and_archive(
            &project_dir.join("mesh.toml"),
            &package_artifacts.join("mesh.toml"),
        ),
        main: route_free::read_and_archive(
            &project_dir.join("main.mpl"),
            &package_artifacts.join("main.mpl"),
        ),
        work: route_free::read_and_archive(
            &project_dir.join("work.mpl"),
            &package_artifacts.join("work.mpl"),
        ),
        readme: route_free::read_and_archive(
            &project_dir.join("README.md"),
            &package_artifacts.join("README.md"),
        ),
        tiny_cluster_work: route_free::read_and_archive(
            &route_free::tiny_cluster_fixture_root().join("work.mpl"),
            &reference_artifacts.join("tiny-cluster.work.mpl"),
        ),
        cluster_proof_work: route_free::read_and_archive(
            &route_free::cluster_proof_fixture_root().join("work.mpl"),
            &reference_artifacts.join("cluster-proof.work.mpl"),
        ),
    }
}

fn assert_scaffold_matches_route_free_contract(sources: &ScaffoldSources) {
    assert_contains("generated mesh.toml", &sources.manifest, "[package]");
    assert_omits("generated mesh.toml", &sources.manifest, "[cluster]");
    assert_omits("generated mesh.toml", &sources.manifest, "declarations");

    assert_contains("generated main.mpl", &sources.main, "Node.start_from_env()");
    assert_eq!(
        sources.main.matches("Node.start_from_env()").count(),
        1,
        "generated main.mpl must keep exactly one Node.start_from_env() call"
    );
    assert_contains("generated main.mpl", &sources.main, "BootstrapStatus");
    assert_contains("generated main.mpl", &sources.main, "runtime bootstrap");
    assert_contains(
        "generated main.mpl",
        &sources.main,
        "runtime bootstrap failed",
    );
    for needle in [
        "HTTP.serve",
        "/work",
        "/status",
        "/health",
        "Continuity.",
        "Timer.sleep",
        "Env.get_int",
        "Node.start(",
    ] {
        assert_omits("generated main.mpl", &sources.main, needle);
    }

    assert_eq!(
        sources.work, sources.tiny_cluster_work,
        "generated work.mpl must stay byte-for-byte aligned with tiny-cluster/work.mpl"
    );
    assert_eq!(
        sources.work, sources.cluster_proof_work,
        "generated work.mpl must stay byte-for-byte aligned with cluster-proof/work.mpl"
    );
    assert_contains(
        "generated work.mpl",
        &sources.work,
        STARTUP_SOURCE_DECLARATION,
    );
    assert_contains("generated work.mpl", &sources.work, "1 + 1");
    assert_omits(
        "generated work.mpl",
        &sources.work,
        "declared_work_runtime_name",
    );
    assert_omits("generated work.mpl", &sources.work, "clustered(work)");
    for needle in [
        "Continuity.submit_declared_work",
        "Continuity.mark_completed",
        "Timer.sleep",
        "Env.get_int",
        "/work",
        "/health",
        "owner_node",
        "replica_node",
    ] {
        assert_omits("generated work.mpl", &sources.work, needle);
    }

    assert_contains(
        "generated README.md",
        &sources.readme,
        "Node.start_from_env()",
    );
    assert_contains("generated README.md", &sources.readme, "`@cluster`");
    assert_contains(
        "generated README.md",
        &sources.readme,
        STARTUP_RUNTIME_NAME_GUIDANCE,
    );
    assert_omits(
        "generated README.md",
        &sources.readme,
        "declared_work_runtime_name()",
    );
    assert_omits("generated README.md", &sources.readme, "clustered(work)");
    assert_contains(
        "generated README.md",
        &sources.readme,
        "meshc cluster status",
    );
    assert_contains(
        "generated README.md",
        &sources.readme,
        "meshc cluster continuity <node-name@host:port> --json",
    );
    assert_contains(
        "generated README.md",
        &sources.readme,
        "meshc cluster continuity <node-name@host:port> <request-key> --json",
    );
    assert_contains(
        "generated README.md",
        &sources.readme,
        "meshc cluster diagnostics",
    );
    assert_contains(
        "generated README.md",
        &sources.readme,
        STARTUP_AUTOSTART_GUIDANCE,
    );
    assert_contains(
        "generated README.md",
        &sources.readme,
        "Use the list form first to discover request keys",
    );
    assert_contains(
        "generated README.md",
        &sources.readme,
        "MESH_CONTINUITY_ROLE",
    );
    assert_contains(
        "generated README.md",
        &sources.readme,
        "MESH_CONTINUITY_PROMOTION_EPOCH",
    );
    for needle in [
        "Continuity.submit_declared_work",
        "Continuity.mark_completed",
        "HTTP.serve",
        "/health",
        "/work",
        "Timer.sleep",
        "CLUSTER_PROOF_",
    ] {
        assert_omits("generated README.md", &sources.readme, needle);
    }
}

fn read_source_file(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn assert_source_contains(path: &Path, needle: &str) {
    let source = read_source_file(path);
    assert!(
        source.contains(needle),
        "expected {} to contain `{}` but it was missing",
        path.display(),
        needle
    );
}

fn assert_source_omits(path: &Path, needle: &str) {
    let source = read_source_file(path);
    assert!(
        !source.contains(needle),
        "expected {} to omit `{}` but it was still present",
        path.display(),
        needle
    );
}

fn assert_source_contains_all(path: &Path, needles: &[&str]) {
    for needle in needles {
        assert_source_contains(path, needle);
    }
}

fn assert_source_omits_all(path: &Path, needles: &[&str]) {
    for needle in needles {
        assert_source_omits(path, needle);
    }
}

#[test]
fn m046_s05_historical_wrapper_alias_replays_the_m047_cutover_rail() {
    let verifier_path = repo_root().join("scripts").join("verify-m046-s05.sh");

    assert_source_contains_all(
        &verifier_path,
        &[
            "bash scripts/verify-m047-s04.sh",
            "retained-m047-s04-verify",
            "latest-proof-bundle.txt",
            "m047-s04-replay",
            "retain-m047-s04-verify",
        ],
    );

    assert_source_omits_all(
        &verifier_path,
        &[
            "bash scripts/verify-m046-s03.sh",
            "bash scripts/verify-m046-s04.sh",
            "cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture",
            "cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture",
            "cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture",
            "npm --prefix website run build",
            "retained-m046-s03-verify",
            "retained-m046-s04-verify",
            "retained-m046-s05-artifacts",
            "retained-proof-bundle",
        ],
    );
}

#[test]
fn m046_s05_docs_demote_the_m046_wrapper_below_the_m047_cutover_rail() {
    let readme_path = repo_root().join("README.md");
    let proof_path = repo_root()
        .join("website")
        .join("docs")
        .join("docs")
        .join("distributed-proof")
        .join("index.md");

    for path in [&readme_path, &proof_path] {
        assert_source_contains_all(
            path,
            &[
                "`bash scripts/verify-m047-s04.sh` — the authoritative cutover rail for the source-first route-free clustered contract",
                "`bash scripts/verify-m046-s06.sh` — the historical M046 closeout wrapper retained as a compatibility alias into the M047 cutover rail",
                "`bash scripts/verify-m046-s05.sh` — the historical M046 equal-surface wrapper retained as a compatibility alias into the M047 cutover rail",
                "`bash scripts/verify-m045-s05.sh` — the historical M045 closeout wrapper retained as a compatibility alias into the M047 cutover rail",
            ],
        );

        assert_source_omits_all(
            path,
            &[
                "`bash scripts/verify-m046-s06.sh` — the authoritative assembled closeout rail",
                "`bash scripts/verify-m046-s05.sh` — the lower-level equal-surface subrail",
                "`bash scripts/verify-m045-s05.sh` — the historical wrapper name retained for replay and transition into the S06 closeout rail",
            ],
        );
    }
}

#[test]
fn m046_s05_scaffold_equal_surface_contract_matches_route_free_packages() {
    let artifacts = artifact_dir("scaffold-equal-surface-contract");
    let temp = tempfile::tempdir().expect("create scaffold tempdir");
    let project_dir =
        route_free::init_clustered_project(temp.path(), "equal-surface-scaffold", &artifacts);
    let sources = load_scaffold_sources(&project_dir, &artifacts);

    assert_scaffold_matches_route_free_contract(&sources);
    assert!(
        artifacts
            .join("generated-project")
            .join("mesh.toml")
            .is_file(),
        "expected retained generated mesh.toml in {}",
        artifacts.display()
    );
    assert!(
        artifacts
            .join("generated-project")
            .join("main.mpl")
            .is_file(),
        "expected retained generated main.mpl in {}",
        artifacts.display()
    );
    assert!(
        artifacts
            .join("generated-project")
            .join("work.mpl")
            .is_file(),
        "expected retained generated work.mpl in {}",
        artifacts.display()
    );
    assert!(
        artifacts
            .join("generated-project")
            .join("README.md")
            .is_file(),
        "expected retained generated README.md in {}",
        artifacts.display()
    );
}

#[test]
fn m046_s05_scaffold_equal_surface_runtime_truth_uses_cli_only_surfaces() {
    let artifacts = artifact_dir("scaffold-equal-surface-runtime-truth");
    let temp = tempfile::tempdir().expect("create scaffold tempdir");
    let project_dir =
        route_free::init_clustered_project(temp.path(), "equal-surface-runtime", &artifacts);
    let sources = load_scaffold_sources(&project_dir, &artifacts);
    assert_scaffold_matches_route_free_contract(&sources);

    let binary_dir = artifacts.join("bin");
    fs::create_dir_all(&binary_dir).expect("failed to create scaffold binary dir");
    let output_path = binary_dir.join("equal-surface-runtime");
    let build_metadata =
        route_free::build_package_binary_to_output(&project_dir, &output_path, &artifacts);
    let persisted_metadata = route_free::read_required_build_metadata(&artifacts)
        .unwrap_or_else(|error| panic!("scaffold temp build metadata should be readable: {error}"));
    assert_eq!(build_metadata, persisted_metadata);
    assert_eq!(persisted_metadata.binary_path, output_path);

    let cluster_port = route_free::dual_stack_cluster_port();
    let primary_node = format!(
        "equal-surface-primary@{}:{cluster_port}",
        route_free::LOOPBACK_V4
    );
    let standby_node = format!(
        "equal-surface-standby@[{}]:{}",
        route_free::LOOPBACK_V6,
        cluster_port
    );
    let expected_nodes = vec![primary_node.clone(), standby_node.clone()];

    route_free::write_json_artifact(
        &artifacts.join("scenario-meta.json"),
        &serde_json::json!({
            "project_dir": project_dir.display().to_string(),
            "binary_path": build_metadata.binary_path.display().to_string(),
            "cluster_port": cluster_port,
            "primary_node": primary_node,
            "standby_node": standby_node,
            "startup_runtime_name": STARTUP_RUNTIME_NAME,
        }),
    );

    let primary_proc = route_free::spawn_route_free_runtime(
        &build_metadata.binary_path,
        &project_dir,
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
        &build_metadata.binary_path,
        &project_dir,
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

        let primary_list = route_free::wait_for_runtime_name_discovered_with_label(
            &artifacts,
            "cluster-continuity-list-primary",
            &primary_node,
            STARTUP_RUNTIME_NAME,
            SHARED_COOKIE,
        );
        let standby_list = route_free::wait_for_runtime_name_discovered_with_label(
            &artifacts,
            "cluster-continuity-list-standby",
            &standby_node,
            STARTUP_RUNTIME_NAME,
            SHARED_COOKIE,
        );
        assert_eq!(route_free::required_u64(&primary_list, "total_records"), 1);
        assert_eq!(route_free::required_u64(&standby_list, "total_records"), 1);
        assert!(!route_free::required_bool(&primary_list, "truncated"));
        assert!(!route_free::required_bool(&standby_list, "truncated"));
        assert_eq!(
            route_free::count_records_for_runtime_name(&primary_list, STARTUP_RUNTIME_NAME),
            1
        );
        assert_eq!(
            route_free::count_records_for_runtime_name(&standby_list, STARTUP_RUNTIME_NAME),
            1
        );

        let request_key = route_free::required_str(
            route_free::record_for_runtime_name(&primary_list, STARTUP_RUNTIME_NAME),
            "request_key",
        );
        let standby_request_key = route_free::required_str(
            route_free::record_for_runtime_name(&standby_list, STARTUP_RUNTIME_NAME),
            "request_key",
        );
        assert_eq!(request_key, standby_request_key);

        let primary_record = route_free::wait_for_continuity_record_completed(
            &artifacts,
            "cluster-continuity-primary-completed",
            &primary_node,
            &request_key,
            STARTUP_RUNTIME_NAME,
            SHARED_COOKIE,
        );
        let standby_record = route_free::wait_for_continuity_record_completed(
            &artifacts,
            "cluster-continuity-standby-completed",
            &standby_node,
            &request_key,
            STARTUP_RUNTIME_NAME,
            SHARED_COOKIE,
        );

        let primary_record = &primary_record["record"];
        let standby_record = &standby_record["record"];
        let owner_node = route_free::required_str(primary_record, "owner_node");
        let replica_node = route_free::required_str(primary_record, "replica_node");
        assert_eq!(
            owner_node,
            route_free::required_str(standby_record, "owner_node")
        );
        assert_eq!(
            replica_node,
            route_free::required_str(standby_record, "replica_node")
        );
        assert!(expected_nodes.contains(&owner_node));
        assert!(expected_nodes.contains(&replica_node));
        assert_ne!(owner_node, replica_node);
        assert_eq!(
            route_free::required_str(primary_record, "declared_handler_runtime_name"),
            STARTUP_RUNTIME_NAME
        );
        assert_eq!(
            route_free::required_str(standby_record, "declared_handler_runtime_name"),
            STARTUP_RUNTIME_NAME
        );
        assert_eq!(
            route_free::required_str(primary_record, "request_key"),
            request_key
        );
        assert_eq!(
            route_free::required_str(standby_record, "request_key"),
            request_key
        );
        assert_eq!(
            route_free::required_str(primary_record, "phase"),
            "completed"
        );
        assert_eq!(
            route_free::required_str(standby_record, "phase"),
            "completed"
        );
        assert_eq!(
            route_free::required_str(primary_record, "result"),
            "succeeded"
        );
        assert_eq!(
            route_free::required_str(standby_record, "result"),
            "succeeded"
        );
        assert_eq!(
            route_free::required_str(primary_record, "execution_node"),
            owner_node
        );
        assert_eq!(
            route_free::required_str(standby_record, "execution_node"),
            owner_node
        );
        assert_eq!(
            route_free::required_str(primary_record, "replica_status"),
            "mirrored"
        );
        assert_eq!(
            route_free::required_str(standby_record, "replica_status"),
            "mirrored"
        );
        assert_eq!(route_free::required_str(primary_record, "error"), "");
        assert_eq!(route_free::required_str(standby_record, "error"), "");

        let (primary_diagnostics, standby_diagnostics) = route_free::wait_for_startup_diagnostics(
            &artifacts,
            &primary_node,
            &standby_node,
            &request_key,
            SHARED_COOKIE,
        );
        let primary_entries =
            route_free::diagnostic_entries_for_request(&primary_diagnostics, &request_key);
        let standby_entries =
            route_free::diagnostic_entries_for_request(&standby_diagnostics, &request_key);
        let combined_transitions: Vec<_> = primary_entries
            .iter()
            .chain(standby_entries.iter())
            .filter_map(|entry| entry["transition"].as_str())
            .collect();
        assert!(combined_transitions.contains(&"startup_trigger"));
        assert!(combined_transitions.contains(&"startup_dispatch_window"));
        assert!(combined_transitions.contains(&"startup_completed"));
        assert!(!combined_transitions.contains(&"startup_rejected"));
        assert!(!combined_transitions.contains(&"startup_convergence_timeout"));
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
        "generated-project/work.mpl",
        "generated-project/README.md",
        "build.log",
        "build-meta.json",
        "scenario-meta.json",
        "cluster-status-primary.json",
        "cluster-status-standby.json",
        "cluster-continuity-list-primary.json",
        "cluster-continuity-list-standby.json",
        "cluster-continuity-primary-completed.json",
        "cluster-continuity-standby-completed.json",
        "cluster-diagnostics-primary.json",
        "cluster-diagnostics-standby.json",
        "primary.stdout.log",
        "primary.stderr.log",
        "standby.stdout.log",
        "standby.stderr.log",
        "primary.combined.log",
        "standby.combined.log",
    ] {
        assert!(
            artifacts.join(required).exists(),
            "missing retained scaffold equal-surface artifact {} in {}",
            required,
            artifacts.display()
        );
    }

    route_free::assert_log_absent(&primary_logs, SHARED_COOKIE);
    route_free::assert_log_absent(&standby_logs, SHARED_COOKIE);
    route_free::assert_log_contains(
        &primary_logs,
        &format!("[clustered-app] runtime bootstrap mode=cluster node={primary_node}"),
    );
    route_free::assert_log_contains(
        &standby_logs,
        &format!("[clustered-app] runtime bootstrap mode=cluster node={standby_node}"),
    );
    route_free::assert_log_contains(
        &primary_logs,
        &format!(
            "[mesh-rt startup] transition=startup_dispatch_window runtime_name={STARTUP_RUNTIME_NAME}"
        ),
    );
}
