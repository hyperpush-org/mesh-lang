use super::m046_route_free as route_free;
use super::m049_todo_postgres_scaffold as postgres;
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub const PACKAGE_NAME: &str = "todo-postgres";
pub const STARTUP_RUNTIME_NAME: &str = "Work.sync_todos";
pub const LIST_ROUTE_RUNTIME_NAME: &str = "Api.Todos.handle_list_todos";
pub const STAGED_BUNDLE_FILES: &[&str] = &[
    "todo-postgres",
    "todo-postgres.up.sql",
    "apply-deploy-migrations.sh",
    "deploy-smoke.sh",
];

#[derive(Debug, Clone)]
pub struct StagedBundle {
    pub bundle_dir: PathBuf,
    pub pointer_path: PathBuf,
    pub manifest_path: PathBuf,
    pub binary_path: PathBuf,
    pub sql_path: PathBuf,
    pub apply_script_path: PathBuf,
    pub smoke_script_path: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
pub struct StagedBundleEntry {
    pub relative_path: String,
    pub absolute_path: PathBuf,
    pub size_bytes: u64,
    pub executable: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct StagedBundleManifest {
    pub bundle_dir: PathBuf,
    pub entries: Vec<StagedBundleEntry>,
}

#[derive(Debug, Clone)]
pub struct StagedClusterRuntimePair {
    pub primary: postgres::TodoRuntimeConfig,
    pub standby: postgres::TodoRuntimeConfig,
}

pub struct SpawnedStagedTodoCluster {
    pub primary: postgres::SpawnedTodoApp,
    pub standby: postgres::SpawnedTodoApp,
}

pub struct StoppedStagedTodoCluster {
    pub primary: postgres::StoppedTodoApp,
    pub standby: postgres::StoppedTodoApp,
}

#[derive(Debug, Clone, Serialize)]
pub struct MirroredContinuitySelection {
    pub request_key: String,
    pub attempt_id: String,
    pub owner_node: String,
    pub replica_node: String,
}

pub fn repo_root() -> PathBuf {
    postgres::repo_root()
}

pub fn meshc_bin() -> PathBuf {
    postgres::meshc_bin()
}

pub fn artifact_dir(test_name: &str) -> PathBuf {
    route_free::artifact_dir("m053-s01", test_name)
}

pub fn artifact_dir_s02(test_name: &str) -> PathBuf {
    route_free::artifact_dir("m053-s02", test_name)
}

pub fn write_artifact(path: &Path, contents: impl AsRef<str>) {
    postgres::write_artifact(path, contents.as_ref());
}

pub fn write_json_artifact(path: &Path, value: &impl Serialize) {
    postgres::write_json_artifact(path, value);
}

pub fn init_postgres_todo_project(
    workspace_dir: &Path,
    project_name: &str,
    artifacts: &Path,
) -> PathBuf {
    postgres::init_postgres_todo_project(workspace_dir, project_name, artifacts)
}

pub fn create_isolated_database(
    base_database_url: &str,
    artifacts: &Path,
    label: &str,
) -> postgres::IsolatedPostgresDatabase {
    postgres::create_isolated_database(base_database_url, artifacts, label)
}

pub fn default_runtime_config(
    project_name: &str,
    database_url: &str,
) -> postgres::TodoRuntimeConfig {
    postgres::default_runtime_config(project_name, database_url)
}

pub fn default_cluster_runtime_pair_for_primary_owned_startup(
    project_name: &str,
    database_url: &str,
    startup_work_delay_ms: Option<u64>,
) -> StagedClusterRuntimePair {
    for _ in 0..64 {
        let cluster_port = route_free::dual_stack_cluster_port();
        let shared_cookie = format!("m053-s02-cookie-{}", unique_stamp());
        let primary = postgres::TodoRuntimeConfig {
            http_port: postgres::unused_port(),
            database_url: database_url.to_string(),
            rate_limit_window_seconds: postgres::DEFAULT_RATE_LIMIT_WINDOW_SECONDS,
            rate_limit_max_requests: postgres::DEFAULT_RATE_LIMIT_MAX_REQUESTS,
            cluster_cookie: shared_cookie.clone(),
            node_name: format!(
                "{project_name}-primary@{}:{cluster_port}",
                route_free::LOOPBACK_V4
            ),
            discovery_seed: "localhost".to_string(),
            cluster_port,
            cluster_role: "primary".to_string(),
            promotion_epoch: 0,
            startup_work_delay_ms,
        };
        let standby = postgres::TodoRuntimeConfig {
            http_port: postgres::unused_port(),
            database_url: database_url.to_string(),
            rate_limit_window_seconds: postgres::DEFAULT_RATE_LIMIT_WINDOW_SECONDS,
            rate_limit_max_requests: postgres::DEFAULT_RATE_LIMIT_MAX_REQUESTS,
            cluster_cookie: shared_cookie,
            node_name: format!(
                "{project_name}-standby@[{}]:{cluster_port}",
                route_free::LOOPBACK_V6
            ),
            discovery_seed: "localhost".to_string(),
            cluster_port,
            cluster_role: "standby".to_string(),
            promotion_epoch: 0,
            startup_work_delay_ms,
        };
        postgres::assert_valid_runtime_config(&primary);
        postgres::assert_valid_runtime_config(&standby);
        if startup_request_owns_primary(&primary.node_name, &standby.node_name) {
            return StagedClusterRuntimePair { primary, standby };
        }
    }

    panic!(
        "failed to find a dual-stack cluster port whose startup record is primary-owned for {project_name}"
    );
}

pub fn startup_request_key() -> String {
    format!("startup::{STARTUP_RUNTIME_NAME}")
}

pub fn send_http_request(
    port: u16,
    method: &str,
    path: &str,
    body: Option<&str>,
) -> std::io::Result<postgres::HttpResponse> {
    postgres::send_http_request(port, method, path, body)
}

pub fn json_response_snapshot(
    artifacts: &Path,
    name: &str,
    response: &postgres::HttpResponse,
    expected_status: u16,
    context: &str,
    secret_values: &[&str],
) -> Value {
    postgres::json_response_snapshot(
        artifacts,
        name,
        response,
        expected_status,
        context,
        secret_values,
    )
}

pub fn wait_for_health(
    config: &postgres::TodoRuntimeConfig,
    artifacts: &Path,
    label: &str,
    secret_values: &[&str],
) -> Value {
    postgres::wait_for_health(config, artifacts, label, secret_values)
}

pub fn wait_for_cluster_health(
    runtime: &StagedClusterRuntimePair,
    artifacts: &Path,
    label_prefix: &str,
    secret_values: &[&str],
) -> (Value, Value) {
    let primary = wait_for_health(
        &runtime.primary,
        artifacts,
        &format!("{label_prefix}-primary-health"),
        secret_values,
    );
    let standby = wait_for_health(
        &runtime.standby,
        artifacts,
        &format!("{label_prefix}-standby-health"),
        secret_values,
    );
    (primary, standby)
}

pub fn json_request_snapshot_for_node(
    artifacts: &Path,
    name: &str,
    config: &postgres::TodoRuntimeConfig,
    method: &str,
    path: &str,
    body: Option<&str>,
    expected_status: u16,
    context: &str,
    secret_values: &[&str],
) -> Value {
    let response = send_http_request(config.http_port, method, path, body)
        .unwrap_or_else(|error| panic!("{method} {path} failed on {}: {error}", config.http_port));
    json_response_snapshot(
        artifacts,
        name,
        &response,
        expected_status,
        context,
        secret_values,
    )
}

pub fn wait_for_dual_node_cluster_status(
    artifacts: &Path,
    label_prefix: &str,
    runtime: &StagedClusterRuntimePair,
) -> (Value, Value) {
    let membership = vec![
        runtime.primary.node_name.clone(),
        runtime.standby.node_name.clone(),
    ];
    let primary = route_free::wait_for_cluster_status_membership(
        artifacts,
        &format!("{label_prefix}-primary-status"),
        &runtime.primary.node_name,
        std::slice::from_ref(&runtime.standby.node_name),
        &membership,
        "primary",
        0,
        &["healthy"],
        &runtime.primary.cluster_cookie,
    );
    let standby = route_free::wait_for_cluster_status_membership(
        artifacts,
        &format!("{label_prefix}-standby-status"),
        &runtime.standby.node_name,
        std::slice::from_ref(&runtime.primary.node_name),
        &membership,
        "standby",
        0,
        &["healthy"],
        &runtime.standby.cluster_cookie,
    );
    (primary, standby)
}

pub fn continuity_list_snapshot_pair(
    artifacts: &Path,
    label_prefix: &str,
    runtime: &StagedClusterRuntimePair,
) -> (Value, Value) {
    let primary = continuity_list_snapshot(
        artifacts,
        &format!("{label_prefix}-primary-continuity"),
        &runtime.primary.node_name,
        &runtime.primary.cluster_cookie,
    );
    let standby = continuity_list_snapshot(
        artifacts,
        &format!("{label_prefix}-standby-continuity"),
        &runtime.standby.node_name,
        &runtime.standby.cluster_cookie,
    );
    (primary, standby)
}

pub fn wait_for_continuity_record_completed_pair(
    artifacts: &Path,
    label_prefix: &str,
    runtime: &StagedClusterRuntimePair,
    request_key: &str,
    runtime_name: &str,
) -> (Value, Value) {
    let primary = wait_for_continuity_record_completed(
        artifacts,
        &format!("{label_prefix}-primary-record"),
        &runtime.primary.node_name,
        request_key,
        runtime_name,
        &runtime.primary.cluster_cookie,
    );
    let standby = wait_for_continuity_record_completed(
        artifacts,
        &format!("{label_prefix}-standby-record"),
        &runtime.standby.node_name,
        request_key,
        runtime_name,
        &runtime.standby.cluster_cookie,
    );
    (primary, standby)
}

pub fn wait_for_request_diagnostics_pair(
    artifacts: &Path,
    label_prefix: &str,
    runtime: &StagedClusterRuntimePair,
    request_key: &str,
) -> (Value, Value) {
    let primary = route_free::wait_for_diagnostics_matching(
        artifacts,
        &format!("{label_prefix}-primary-diagnostics"),
        &runtime.primary.node_name,
        &format!("diagnostics for request {request_key} on primary"),
        &runtime.primary.cluster_cookie,
        |snapshot| !route_free::diagnostic_entries_for_request(snapshot, request_key).is_empty(),
    );
    let standby = route_free::wait_for_diagnostics_matching(
        artifacts,
        &format!("{label_prefix}-standby-diagnostics"),
        &runtime.standby.node_name,
        &format!("diagnostics for request {request_key} on standby"),
        &runtime.standby.cluster_cookie,
        |snapshot| !route_free::diagnostic_entries_for_request(snapshot, request_key).is_empty(),
    );
    (primary, standby)
}

pub fn wait_for_primary_owned_startup_selection(
    artifacts: &Path,
    label_prefix: &str,
    runtime: &StagedClusterRuntimePair,
) -> MirroredContinuitySelection {
    let primary_list = route_free::wait_for_continuity_list_matching(
        artifacts,
        &format!("{label_prefix}-primary-startup-list"),
        &runtime.primary.node_name,
        "mirrored primary-owned startup record on primary",
        &runtime.primary.cluster_cookie,
        |json| {
            mirrored_startup_record_matches(
                route_free::find_record_for_runtime_name(json, STARTUP_RUNTIME_NAME),
                &runtime.primary.node_name,
                &runtime.standby.node_name,
            )
        },
    );
    let standby_list = route_free::wait_for_continuity_list_matching(
        artifacts,
        &format!("{label_prefix}-standby-startup-list"),
        &runtime.standby.node_name,
        "mirrored primary-owned startup record on standby",
        &runtime.standby.cluster_cookie,
        |json| {
            mirrored_startup_record_matches(
                route_free::find_record_for_runtime_name(json, STARTUP_RUNTIME_NAME),
                &runtime.primary.node_name,
                &runtime.standby.node_name,
            )
        },
    );

    let primary_record = route_free::record_for_runtime_name(&primary_list, STARTUP_RUNTIME_NAME);
    let standby_record = route_free::record_for_runtime_name(&standby_list, STARTUP_RUNTIME_NAME);
    let request_key = route_free::required_str(primary_record, "request_key");
    let attempt_id = route_free::required_str(primary_record, "attempt_id");
    assert_eq!(
        request_key,
        route_free::required_str(standby_record, "request_key"),
        "startup request key drifted across primary/standby continuity lists"
    );
    assert_eq!(
        attempt_id,
        route_free::required_str(standby_record, "attempt_id"),
        "startup attempt id drifted across primary/standby continuity lists"
    );
    assert_eq!(request_key, startup_request_key());

    MirroredContinuitySelection {
        request_key,
        attempt_id,
        owner_node: runtime.primary.node_name.clone(),
        replica_node: runtime.standby.node_name.clone(),
    }
}

pub fn wait_for_startup_diagnostics_pair(
    artifacts: &Path,
    runtime: &StagedClusterRuntimePair,
    request_key: &str,
) -> (Value, Value) {
    route_free::wait_for_startup_diagnostics(
        artifacts,
        &runtime.primary.node_name,
        &runtime.standby.node_name,
        request_key,
        &runtime.primary.cluster_cookie,
    )
}

pub fn spawn_staged_todo_app(
    bundle: &StagedBundle,
    artifacts: &Path,
    label: &str,
    config: &postgres::TodoRuntimeConfig,
) -> postgres::SpawnedTodoApp {
    postgres::spawn_todo_app(
        &bundle.binary_path,
        &bundle.bundle_dir,
        artifacts,
        label,
        config,
    )
}

pub fn spawn_staged_todo_cluster(
    bundle: &StagedBundle,
    artifacts: &Path,
    label_prefix: &str,
    runtime: &StagedClusterRuntimePair,
) -> SpawnedStagedTodoCluster {
    write_json_artifact(
        &artifacts.join(format!("{label_prefix}.runtime-config.json")),
        &serde_json::json!({
            "primary": {
                "http_port": runtime.primary.http_port,
                "cluster_cookie": "<redacted:cluster-cookie>",
                "node_name": runtime.primary.node_name,
                "discovery_seed": runtime.primary.discovery_seed,
                "cluster_port": runtime.primary.cluster_port,
                "cluster_role": runtime.primary.cluster_role,
                "promotion_epoch": runtime.primary.promotion_epoch,
                "startup_work_delay_ms": runtime.primary.startup_work_delay_ms,
                "database_url": "<redacted:DATABASE_URL>",
            },
            "standby": {
                "http_port": runtime.standby.http_port,
                "cluster_cookie": "<redacted:cluster-cookie>",
                "node_name": runtime.standby.node_name,
                "discovery_seed": runtime.standby.discovery_seed,
                "cluster_port": runtime.standby.cluster_port,
                "cluster_role": runtime.standby.cluster_role,
                "promotion_epoch": runtime.standby.promotion_epoch,
                "startup_work_delay_ms": runtime.standby.startup_work_delay_ms,
                "database_url": "<redacted:DATABASE_URL>",
            }
        }),
    );

    let primary = spawn_staged_todo_app(
        bundle,
        artifacts,
        &format!("{label_prefix}-primary"),
        &runtime.primary,
    );
    let standby = spawn_staged_todo_app(
        bundle,
        artifacts,
        &format!("{label_prefix}-standby"),
        &runtime.standby,
    );

    SpawnedStagedTodoCluster { primary, standby }
}

pub fn stop_todo_app(
    spawned: postgres::SpawnedTodoApp,
    secret_values: &[&str],
) -> postgres::StoppedTodoApp {
    postgres::stop_todo_app(spawned, secret_values)
}

pub fn stop_staged_todo_cluster(
    spawned: SpawnedStagedTodoCluster,
    secret_values: &[&str],
) -> StoppedStagedTodoCluster {
    StoppedStagedTodoCluster {
        primary: stop_todo_app(spawned.primary, secret_values),
        standby: stop_todo_app(spawned.standby, secret_values),
    }
}

pub fn assert_runtime_logs(logs: &postgres::StoppedTodoApp, config: &postgres::TodoRuntimeConfig) {
    postgres::assert_runtime_logs(logs, config)
}

pub fn assert_valid_runtime_config(config: &postgres::TodoRuntimeConfig) {
    postgres::assert_valid_runtime_config(config)
}

pub fn assert_artifacts_redacted(artifacts: &Path, secret_values: &[&str]) {
    postgres::assert_artifacts_redacted(artifacts, secret_values)
}

pub fn assert_phase_success(run: &postgres::CompletedCommand, description: &str) {
    postgres::assert_phase_success(run, description)
}

pub fn create_retained_bundle_dir(label: &str) -> PathBuf {
    let bundle_root = env::temp_dir().join("mesh-m053-s01");
    fs::create_dir_all(&bundle_root)
        .unwrap_or_else(|error| panic!("failed to create {}: {error}", bundle_root.display()));
    let bundle_dir = bundle_root.join(format!("{}-{}", sanitize_label(label), unique_stamp()));
    fs::create_dir_all(&bundle_dir)
        .unwrap_or_else(|error| panic!("failed to create {}: {error}", bundle_dir.display()));
    assert_staged_bundle_dir_outside_repo_root(&bundle_dir);
    bundle_dir
}

pub fn run_stage_deploy_script(
    project_dir: &Path,
    bundle_dir: &Path,
    artifacts: &Path,
    label: &str,
) -> postgres::CompletedCommand {
    postgres::ensure_mesh_rt_staticlib();

    let stage_script = project_dir.join("scripts/stage-deploy.sh");
    assert!(
        stage_script.is_file(),
        "generated Postgres starter is missing stage script at {}",
        stage_script.display()
    );

    let mut command = Command::new("bash");
    command
        .current_dir(project_dir)
        .arg(&stage_script)
        .arg(bundle_dir);
    apply_meshc_path_env(&mut command);
    postgres::run_command_capture(
        &mut command,
        artifacts,
        label,
        "generated Postgres stage-deploy.sh",
        postgres::PHASE_TIMEOUT,
        &[],
    )
}

pub fn inspect_staged_bundle(bundle_dir: &Path, artifacts: &Path) -> StagedBundle {
    assert_staged_bundle_dir_outside_repo_root(bundle_dir);

    let bundle = StagedBundle {
        bundle_dir: bundle_dir.to_path_buf(),
        pointer_path: artifacts.join("staged-bundle.path.txt"),
        manifest_path: artifacts.join("staged-bundle.manifest.json"),
        binary_path: bundle_dir.join(PACKAGE_NAME),
        sql_path: bundle_dir.join("todo-postgres.up.sql"),
        apply_script_path: bundle_dir.join("apply-deploy-migrations.sh"),
        smoke_script_path: bundle_dir.join("deploy-smoke.sh"),
    };

    assert_is_executable(&bundle.binary_path);
    assert!(
        bundle.sql_path.is_file(),
        "expected staged deploy SQL artifact at {}",
        bundle.sql_path.display()
    );
    assert_is_executable(&bundle.apply_script_path);
    assert_is_executable(&bundle.smoke_script_path);
    assert!(
        !bundle.bundle_dir.join("meshc").exists(),
        "staged deploy bundle should not include meshc"
    );
    assert!(
        !bundle.bundle_dir.join("main.mpl").exists(),
        "staged deploy bundle should not require repo source files"
    );
    assert!(
        !bundle.bundle_dir.join("mesh.toml").exists(),
        "staged deploy bundle should not require the source manifest"
    );

    write_artifact(
        &bundle.pointer_path,
        format!("{}\n", bundle.bundle_dir.display()),
    );
    write_json_artifact(&bundle.manifest_path, &staged_bundle_manifest(&bundle));

    bundle
}

pub fn load_staged_bundle_from_pointer(pointer_path: &Path, artifacts: &Path) -> StagedBundle {
    let pointer_contents = fs::read_to_string(pointer_path).unwrap_or_else(|error| {
        panic!(
            "failed to read staged bundle pointer {}: {error}",
            pointer_path.display()
        )
    });
    let bundle_dir_text = pointer_contents.trim();
    assert!(
        !bundle_dir_text.is_empty(),
        "staged bundle pointer {} is empty",
        pointer_path.display()
    );
    let bundle_dir = PathBuf::from(bundle_dir_text);
    assert!(
        bundle_dir.is_dir(),
        "staged bundle pointer {} does not reference a directory: {}",
        pointer_path.display(),
        bundle_dir.display()
    );
    inspect_staged_bundle(&bundle_dir, artifacts)
}

pub fn run_staged_apply_deploy_migrations_script(
    bundle: &StagedBundle,
    artifacts: &Path,
    label: &str,
    database_url: Option<&str>,
) -> postgres::CompletedCommand {
    let mut command = Command::new("bash");
    command
        .current_dir(&bundle.bundle_dir)
        .arg(&bundle.apply_script_path)
        .arg(&bundle.sql_path);
    if let Some(database_url) = database_url {
        command.env("DATABASE_URL", database_url);
    } else {
        command.env_remove("DATABASE_URL");
    }
    postgres::run_command_capture(
        &mut command,
        artifacts,
        label,
        "staged apply-deploy-migrations.sh",
        postgres::PHASE_TIMEOUT,
        &database_url.into_iter().collect::<Vec<_>>(),
    )
}

pub fn run_staged_deploy_smoke_script(
    bundle: &StagedBundle,
    artifacts: &Path,
    label: &str,
    base_url: Option<&str>,
    port: u16,
) -> postgres::CompletedCommand {
    let mut command = Command::new("bash");
    command
        .current_dir(&bundle.bundle_dir)
        .arg(&bundle.smoke_script_path);
    command.env("PORT", port.to_string());
    if let Some(base_url) = base_url {
        command.env("BASE_URL", base_url);
    } else {
        command.env_remove("BASE_URL");
    }
    postgres::run_command_capture(
        &mut command,
        artifacts,
        label,
        "staged deploy-smoke.sh",
        postgres::PHASE_TIMEOUT,
        &[],
    )
}

pub fn wait_for_single_node_cluster_status(
    artifacts: &Path,
    label: &str,
    node_name: &str,
    cookie: &str,
) -> Value {
    route_free::wait_for_cluster_status_matching(
        artifacts,
        label,
        node_name,
        "single-node clustered status truth",
        cookie,
        |json| {
            route_free::required_str(&json["membership"], "local_node") == node_name
                && route_free::required_string_list(&json["membership"], "peer_nodes").is_empty()
                && route_free::sorted(&route_free::required_string_list(
                    &json["membership"],
                    "nodes",
                )) == vec![node_name.to_string()]
                && route_free::required_str(&json["authority"], "cluster_role") == "primary"
                && route_free::required_u64(&json["authority"], "promotion_epoch") == 0
                && ["local_only", "healthy"].contains(
                    &route_free::required_str(&json["authority"], "replication_health").as_str(),
                )
        },
    )
}

pub fn wait_for_startup_runtime_discovered(
    artifacts: &Path,
    label: &str,
    node_name: &str,
    cookie: &str,
) -> Value {
    route_free::wait_for_runtime_name_discovered_with_label(
        artifacts,
        label,
        node_name,
        STARTUP_RUNTIME_NAME,
        cookie,
    )
}

pub fn wait_for_continuity_record_completed(
    artifacts: &Path,
    label: &str,
    node_name: &str,
    request_key: &str,
    runtime_name: &str,
    cookie: &str,
) -> Value {
    route_free::wait_for_continuity_record_completed(
        artifacts,
        label,
        node_name,
        request_key,
        runtime_name,
        cookie,
    )
}

pub fn continuity_list_snapshot(
    artifacts: &Path,
    label: &str,
    node_name: &str,
    cookie: &str,
) -> Value {
    let output = route_free::run_meshc_cluster(
        artifacts,
        label,
        &["cluster", "continuity", node_name, "--json"],
        cookie,
    );
    assert!(
        output.status.success(),
        "cluster continuity list should succeed for {node_name}:\n{}",
        route_free::command_output_text(&output)
    );
    route_free::parse_json_output(artifacts, label, &output, "cluster continuity list")
}

pub fn wait_for_new_route_request_key(
    artifacts: &Path,
    label: &str,
    node_name: &str,
    before_list_json: &Value,
    cookie: &str,
) -> (Value, String) {
    route_free::wait_for_new_request_key_for_runtime_name_and_replication_count(
        artifacts,
        label,
        node_name,
        before_list_json,
        LIST_ROUTE_RUNTIME_NAME,
        1,
        cookie,
    )
}

pub fn wait_for_startup_diagnostics(
    artifacts: &Path,
    label: &str,
    node_name: &str,
    request_key: &str,
    cookie: &str,
) -> Value {
    route_free::wait_for_diagnostics_matching(
        artifacts,
        label,
        node_name,
        "single-node startup diagnostics truth",
        cookie,
        |snapshot| {
            let entries = route_free::diagnostic_entries_for_request(snapshot, request_key);
            let transitions: Vec<_> = entries
                .iter()
                .filter_map(|entry| entry["transition"].as_str())
                .collect();
            transitions.contains(&"startup_trigger")
                && transitions.contains(&"startup_completed")
                && !transitions.contains(&"startup_rejected")
                && !transitions.contains(&"startup_convergence_timeout")
        },
    )
}

fn mirrored_startup_record_matches(
    record: Option<&Value>,
    expected_owner: &str,
    expected_replica: &str,
) -> bool {
    let Some(record) = record else {
        return false;
    };
    !route_free::required_str(record, "request_key").is_empty()
        && !route_free::required_str(record, "attempt_id").is_empty()
        && route_free::required_u64(record, "replication_count") == 2
        && route_free::required_str(record, "owner_node") == expected_owner
        && route_free::required_str(record, "replica_node") == expected_replica
        && matches!(
            route_free::required_str(record, "replica_status").as_str(),
            "preparing" | "mirrored"
        )
}

fn stable_hash_u64(value: &str) -> u64 {
    let digest = Sha256::digest(value.as_bytes());
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&digest[..8]);
    u64::from_be_bytes(bytes)
}

fn startup_request_owns_primary(primary_node: &str, standby_node: &str) -> bool {
    let mut membership = vec![primary_node.to_string(), standby_node.to_string()];
    membership.sort_by_key(|value| (stable_hash_u64(value), value.clone()));
    let owner_index = (stable_hash_u64(&format!("request::{}", startup_request_key())) as usize)
        % membership.len();
    membership[owner_index] == primary_node
}

fn staged_bundle_manifest(bundle: &StagedBundle) -> StagedBundleManifest {
    let entries = STAGED_BUNDLE_FILES
        .iter()
        .map(|relative_path| {
            let absolute_path = bundle.bundle_dir.join(relative_path);
            let metadata = fs::metadata(&absolute_path).unwrap_or_else(|error| {
                panic!(
                    "failed to stat staged bundle entry {}: {error}",
                    absolute_path.display()
                )
            });
            StagedBundleEntry {
                relative_path: (*relative_path).to_string(),
                absolute_path,
                size_bytes: metadata.len(),
                executable: metadata.permissions().mode() & 0o111 != 0,
            }
        })
        .collect();
    StagedBundleManifest {
        bundle_dir: bundle.bundle_dir.clone(),
        entries,
    }
}

fn apply_meshc_path_env(command: &mut Command) {
    let meshc_dir = meshc_bin()
        .parent()
        .unwrap_or_else(|| panic!("meshc binary is missing a parent directory"))
        .to_path_buf();
    let existing = env::var_os("PATH").unwrap_or_default();
    let mut paths = vec![meshc_dir];
    paths.extend(env::split_paths(&existing));
    let joined = env::join_paths(paths)
        .unwrap_or_else(|error| panic!("failed to join meshc PATH override: {error}"));
    command.env("PATH", joined);
}

fn assert_is_executable(path: &Path) {
    let metadata = fs::metadata(path)
        .unwrap_or_else(|error| panic!("failed to stat {}: {error}", path.display()));
    assert!(metadata.is_file(), "expected file at {}", path.display());
    assert!(
        metadata.permissions().mode() & 0o111 != 0,
        "expected executable permissions at {}",
        path.display()
    );
}

fn assert_staged_bundle_dir_outside_repo_root(bundle_dir: &Path) {
    let bundle_dir = fs::canonicalize(bundle_dir)
        .unwrap_or_else(|error| panic!("failed to canonicalize {}: {error}", bundle_dir.display()));
    let repo_root = fs::canonicalize(repo_root()).expect("failed to canonicalize repo root");
    assert!(
        !bundle_dir.starts_with(&repo_root),
        "staged bundle dir must live outside the repo root; bundle_dir={} repo_root={}",
        bundle_dir.display(),
        repo_root.display()
    );
}

fn sanitize_label(label: &str) -> String {
    let sanitized: String = label
        .chars()
        .map(|ch| {
            if ch.is_ascii_lowercase() || ch.is_ascii_digit() {
                ch
            } else {
                '_'
            }
        })
        .collect();
    let trimmed = sanitized.trim_matches('_');
    if trimmed.is_empty() {
        "bundle".to_string()
    } else {
        trimmed.to_string()
    }
}

fn unique_stamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_nanos()
}
