use mesh_codegen::mir::{MirExpr, MirFunction, MirModule, MirType};
use mesh_codegen::{compile_mir_to_llvm_ir, StartupWorkRegistration};
use serde_json::Value;
use std::any::Any;
use std::fs::{self, File};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::OnceLock;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

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

fn ensure_mesh_rt_staticlib() {
    static BUILD_ONCE: OnceLock<()> = OnceLock::new();
    BUILD_ONCE.get_or_init(|| {
        let output = Command::new("cargo")
            .current_dir(repo_root())
            .args(["build", "-p", "mesh-rt"])
            .output()
            .expect("failed to invoke cargo build -p mesh-rt");
        assert!(
            output.status.success(),
            "cargo build -p mesh-rt failed:\n{}",
            command_output_text(&output)
        );
    });
}

fn command_output_text(output: &Output) -> String {
    format!(
        "status: {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn package_manifest(name: &str) -> String {
    format!("[package]\nname = \"{name}\"\nversion = \"1.0.0\"\n")
}

fn work_cluster_manifest(name: &str) -> String {
    format!(
        "{}\n[cluster]\nenabled = true\ndeclarations = [\n  {{ kind = \"work\", target = \"Work.handle_submit\" }},\n]\n",
        package_manifest(name)
    )
}

fn service_cluster_manifest(name: &str) -> String {
    format!(
        "{}\n[cluster]\nenabled = true\ndeclarations = [\n  {{ kind = \"service_call\", target = \"Services.Jobs.submit\" }},\n  {{ kind = \"service_cast\", target = \"Services.Jobs.reset\" }},\n]\n",
        package_manifest(name)
    )
}

fn declared_work_project_main_source() -> &'static str {
    "from Work import handle_submit, local_only\n\nfn main() do\n  let _ = handle_submit(\"req-1\", \"attempt-1\")\n  let _ = local_only(\"demo\")\nend\n"
}

fn declared_work_project_work_source() -> &'static str {
    "pub fn handle_submit(request_key :: String, attempt_id :: String) -> Int do\n  if String.length(request_key) > 0 do\n    String.length(attempt_id)\n  else\n    0\n  end\nend\n\npub fn local_only(payload :: String) -> String do\n  payload\nend\n\nfn hidden_submit(payload :: String) -> String do\n  payload\nend\n"
}

fn source_declared_work_source() -> &'static str {
    "clustered(work) pub fn handle_submit(request_key :: String, attempt_id :: String) -> Int do\n  if String.length(request_key) > 0 do\n    String.length(attempt_id)\n  else\n    0\n  end\nend\n\npub fn local_only(payload :: String) -> String do\n  payload\nend\n\nfn hidden_submit(payload :: String) -> String do\n  payload\nend\n"
}

fn service_project_main_source() -> &'static str {
    "from Services import Jobs\n\nfn main() do\n  let pid = Jobs.start(0)\n  let _ = Jobs.submit(pid, \"demo\")\n  let _ = Jobs.reset(pid)\nend\n"
}

fn service_project_services_source() -> &'static str {
    "service Jobs do\n  fn init(start :: Int) -> Int do\n    start\n  end\n\n  call Submit(payload :: String) :: String do |state|\n    (state, payload)\n  end\n\n  cast Reset() do |_state|\n    0\n  end\nend\n"
}

fn write_project_sources(project_dir: &Path, manifest: Option<&str>, sources: &[(&str, &str)]) {
    fs::create_dir_all(project_dir).expect("failed to create temp project dir");
    if let Some(manifest) = manifest {
        fs::write(project_dir.join("mesh.toml"), manifest).expect("failed to write mesh.toml");
    }
    for (path, content) in sources {
        fs::write(project_dir.join(path), content)
            .unwrap_or_else(|err| panic!("failed to write {path}: {err}"));
    }
}

fn build_temp_project_with_sources(
    manifest: Option<&str>,
    sources: &[(&str, &str)],
    emit_llvm: bool,
) -> (Output, Option<String>) {
    ensure_mesh_rt_staticlib();
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let project_dir = tmp.path().join("project");
    write_project_sources(&project_dir, manifest, sources);

    let mut command = Command::new(meshc_bin());
    command.current_dir(repo_root());
    command.arg("build").arg(project_dir.to_str().unwrap());
    if emit_llvm {
        command.arg("--emit-llvm");
    }

    let output = command.output().expect("failed to invoke meshc build");
    let llvm = if emit_llvm && output.status.success() {
        Some(
            fs::read_to_string(project_dir.join("project.ll"))
                .expect("expected emitted llvm alongside built temp project"),
        )
    } else {
        None
    };
    (output, llvm)
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

fn assert_order(haystack: &str, earlier: &str, later: &str, context: &str) {
    let earlier_idx = haystack
        .find(earlier)
        .unwrap_or_else(|| panic!("missing earlier marker {earlier:?} in:\n{context}"));
    let later_idx = haystack
        .find(later)
        .unwrap_or_else(|| panic!("missing later marker {later:?} in:\n{context}"));
    assert!(
        earlier_idx < later_idx,
        "expected {earlier:?} before {later:?} in:\n{context}"
    );
}

fn minimal_entry_mir() -> MirModule {
    MirModule {
        functions: vec![MirFunction {
            name: "mesh_main".to_string(),
            params: vec![],
            return_type: MirType::Unit,
            body: MirExpr::Unit,
            is_closure_fn: false,
            captures: vec![],
            has_tail_calls: false,
        }],
        structs: vec![],
        sum_types: vec![],
        entry_function: Some("mesh_main".to_string()),
        service_dispatch: std::collections::HashMap::new(),
    }
}

#[test]
fn m046_s02_codegen_work_build_emits_startup_registration_and_trigger() {
    let manifest = work_cluster_manifest("startup-work-proof");
    let (output, llvm) = build_temp_project_with_sources(
        Some(manifest.as_str()),
        &[
            ("main.mpl", declared_work_project_main_source()),
            ("work.mpl", declared_work_project_work_source()),
        ],
        true,
    );
    assert!(
        output.status.success(),
        "declared-work build should succeed:\n{}",
        command_output_text(&output)
    );
    let llvm = llvm.expect("declared-work build should emit llvm");
    let main_ir = main_wrapper_ir(&llvm);

    assert!(
        llvm.contains("Work.handle_submit")
            && llvm.contains("@startup_work_reg_Work_handle_submit")
            && llvm.contains("@declared_runtime_reg_Work_handle_submit"),
        "expected emitted startup and declared runtime names in llvm:\n{llvm}"
    );
    assert!(
        main_ir.contains("call void @mesh_register_declared_handler")
            && main_ir.contains("call void @mesh_register_startup_work")
            && main_ir.contains("call void @mesh_trigger_startup_work"),
        "expected startup registration + trigger calls in main wrapper:\n{main_ir}"
    );

    assert_order(
        main_ir,
        "@mesh_register_declared_handler",
        "@mesh_register_startup_work",
        main_ir,
    );
    assert_order(
        main_ir,
        "@mesh_register_startup_work",
        "@mesh_main(",
        main_ir,
    );
    assert_order(
        main_ir,
        "@mesh_main(",
        "@mesh_trigger_startup_work",
        main_ir,
    );
    assert_order(
        main_ir,
        "@mesh_trigger_startup_work",
        "@mesh_rt_run_scheduler",
        main_ir,
    );
}

#[test]
fn m046_s02_codegen_service_build_omits_startup_runtime_hooks() {
    let manifest = service_cluster_manifest("startup-service-proof");
    let (output, llvm) = build_temp_project_with_sources(
        Some(manifest.as_str()),
        &[
            ("main.mpl", service_project_main_source()),
            ("services.mpl", service_project_services_source()),
        ],
        true,
    );
    assert!(
        output.status.success(),
        "declared-service build should succeed:\n{}",
        command_output_text(&output)
    );
    let llvm = llvm.expect("declared-service build should emit llvm");
    let main_ir = main_wrapper_ir(&llvm);

    assert!(
        main_ir.contains("call void @mesh_register_declared_handler"),
        "expected declared-handler registration to stay present:\n{main_ir}"
    );
    assert!(
        !main_ir.contains("call void @mesh_register_startup_work")
            && !main_ir.contains("call void @mesh_trigger_startup_work"),
        "service-only builds must not auto-trigger startup work:\n{main_ir}"
    );
}

#[test]
fn m046_s02_codegen_source_and_manifest_work_share_startup_identity() {
    let manifest_build = build_temp_project_with_sources(
        Some(work_cluster_manifest("startup-manifest-proof").as_str()),
        &[
            ("main.mpl", declared_work_project_main_source()),
            ("work.mpl", declared_work_project_work_source()),
        ],
        true,
    );
    assert!(
        manifest_build.0.status.success(),
        "manifest-declared work build should succeed:\n{}",
        command_output_text(&manifest_build.0)
    );
    let manifest_llvm = manifest_build
        .1
        .expect("manifest-declared work build should emit llvm");

    let source_build = build_temp_project_with_sources(
        Some(package_manifest("startup-source-proof").as_str()),
        &[
            ("main.mpl", declared_work_project_main_source()),
            ("work.mpl", source_declared_work_source()),
        ],
        true,
    );
    assert!(
        source_build.0.status.success(),
        "source-declared work build should succeed:\n{}",
        command_output_text(&source_build.0)
    );
    let source_llvm = source_build
        .1
        .expect("source-declared work build should emit llvm");

    for llvm in [&manifest_llvm, &source_llvm] {
        let main_ir = main_wrapper_ir(llvm);
        assert!(
            llvm.contains("Work.handle_submit")
                && llvm.contains("@startup_work_reg_Work_handle_submit"),
            "expected shared startup runtime name in llvm:\n{llvm}"
        );
        assert!(
            main_ir.contains("call void @mesh_register_startup_work"),
            "expected startup registration call in main wrapper:\n{main_ir}"
        );
    }
}

#[test]
fn m046_s02_codegen_missing_declared_handler_metadata_fails_explicitly() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let output_path = tmp.path().join("missing-startup.ll");
    let error = compile_mir_to_llvm_ir(
        &minimal_entry_mir(),
        &[],
        &[StartupWorkRegistration {
            runtime_registration_name: "Work.handle_submit".to_string(),
        }],
        &output_path,
        None,
    )
    .expect_err("startup work without declared handler metadata should fail");

    assert!(
        error.contains("Startup work `Work.handle_submit` is missing declared-handler metadata"),
        "expected explicit startup metadata failure, got: {error}"
    );
}

const LOOPBACK_V4: &str = "127.0.0.1";
const LOOPBACK_V6: &str = "::1";
const SHARED_COOKIE: &str = "mesh-m046-s02-cli-cookie";
const STATUS_TIMEOUT: Duration = Duration::from_secs(20);
const CONTINUITY_TIMEOUT: Duration = Duration::from_secs(20);
const DIAGNOSTIC_TIMEOUT: Duration = Duration::from_secs(20);

struct RouteFreeRuntimeProject {
    _tempdir: tempfile::TempDir,
    project_dir: PathBuf,
    binary_path: PathBuf,
}

#[derive(Clone, Debug)]
struct RouteFreeNodeConfig {
    node_basename: String,
    advertise_host: String,
    cluster_port: u16,
    cluster_role: String,
    promotion_epoch: u64,
}

struct SpawnedProcess {
    child: Child,
    stdout_path: PathBuf,
    stderr_path: PathBuf,
}

struct StoppedProcess {
    stdout: String,
    stderr: String,
    combined: String,
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

fn unused_port() -> u16 {
    TcpListener::bind((LOOPBACK_V4, 0))
        .expect("failed to bind ephemeral loopback port")
        .local_addr()
        .expect("failed to read ephemeral loopback port")
        .port()
}

fn dual_stack_cluster_port() -> u16 {
    for _ in 0..64 {
        let listener = TcpListener::bind((LOOPBACK_V4, 0))
            .expect("failed to bind IPv4 loopback for ephemeral cluster port");
        let port = listener
            .local_addr()
            .expect("failed to read IPv4 ephemeral cluster port")
            .port();
        drop(listener);

        if TcpListener::bind((LOOPBACK_V4, port)).is_ok()
            && TcpListener::bind((LOOPBACK_V6, port)).is_ok()
        {
            return port;
        }
    }

    panic!("failed to find a dual-stack cluster port");
}

fn sorted(values: &[String]) -> Vec<String> {
    let mut copy = values.to_vec();
    copy.sort();
    copy
}

fn artifact_dir(test_name: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = repo_root()
        .join(".tmp")
        .join("m046-s02")
        .join(format!("{test_name}-{stamp}"));
    fs::create_dir_all(&dir).expect("failed to create e2e artifact dir");
    dir
}

fn write_artifact(path: &Path, contents: impl AsRef<str>) {
    fs::write(path, contents.as_ref())
        .unwrap_or_else(|error| panic!("failed to write artifact {}: {error}", path.display()));
}

fn route_free_startup_manifest(name: &str) -> String {
    format!(
        "{}\n[cluster]\nenabled = true\ndeclarations = [\n  {{ kind = \"work\", target = \"Work.execute_declared_work\" }},\n  {{ kind = \"work\", target = \"Work.execute_aux_work\" }},\n]\n",
        package_manifest(name)
    )
}

fn route_free_startup_main_source() -> &'static str {
    "fn log_bootstrap(status :: BootstrapStatus) do\n  println(\"[startup-cli-proof] runtime bootstrap mode=#{status.mode} node=#{status.node_name} cluster_port=#{status.cluster_port} discovery_seed=#{status.discovery_seed}\")\nend\n\nfn main() do\n  case Node.start_from_env() do\n    Ok(status) -> log_bootstrap(status)\n    Err(reason) -> println(\"[startup-cli-proof] runtime bootstrap failed reason=#{reason}\")\n  end\nend\n"
}

fn route_free_startup_work_source() -> &'static str {
    "pub fn execute_declared_work(request_key :: String, attempt_id :: String) -> Int do\n  Timer.sleep(50)\n  String.length(request_key)\nend\n\npub fn execute_aux_work(request_key :: String, attempt_id :: String) -> Int do\n  Timer.sleep(50)\n  String.length(attempt_id)\nend\n"
}

fn tiny_route_free_startup_main_source() -> &'static str {
    "fn log_bootstrap(status :: BootstrapStatus) do\n  println(\"[startup-tiny-proof] runtime bootstrap mode=#{status.mode} node=#{status.node_name} cluster_port=#{status.cluster_port} discovery_seed=#{status.discovery_seed}\")\nend\n\nfn main() do\n  case Node.start_from_env() do\n    Ok(status) -> log_bootstrap(status)\n    Err(reason) -> println(\"[startup-tiny-proof] runtime bootstrap failed reason=#{reason}\")\n  end\nend\n"
}

fn tiny_route_free_startup_work_source() -> &'static str {
    "clustered(work) pub fn execute_declared_work(_request_key :: String, _attempt_id :: String) -> Int do\n  1 + 1\nend\n"
}

fn build_route_free_runtime_project(name: &str) -> RouteFreeRuntimeProject {
    ensure_mesh_rt_staticlib();
    let tmpdir = tempfile::tempdir().expect("failed to create temp dir");
    let project_dir = tmpdir.path().join("project");
    write_project_sources(
        &project_dir,
        Some(route_free_startup_manifest(name).as_str()),
        &[
            ("main.mpl", route_free_startup_main_source()),
            ("work.mpl", route_free_startup_work_source()),
        ],
    );

    let binary_path = project_dir.join(name);
    let output = Command::new(meshc_bin())
        .current_dir(repo_root())
        .arg("build")
        .arg(&project_dir)
        .arg("--output")
        .arg(&binary_path)
        .output()
        .expect("failed to invoke meshc build for route-free runtime project");
    assert!(
        output.status.success(),
        "route-free runtime project build should succeed:\n{}",
        command_output_text(&output)
    );

    RouteFreeRuntimeProject {
        _tempdir: tmpdir,
        project_dir,
        binary_path,
    }
}

fn build_tiny_route_free_runtime_project(name: &str, artifacts: &Path) -> RouteFreeRuntimeProject {
    ensure_mesh_rt_staticlib();
    let tmpdir = tempfile::tempdir().expect("failed to create temp dir");
    let project_dir = tmpdir.path().join("project");
    let manifest = package_manifest(name);
    let main_source = tiny_route_free_startup_main_source();
    let work_source = tiny_route_free_startup_work_source();
    write_project_sources(
        &project_dir,
        Some(manifest.as_str()),
        &[("main.mpl", main_source), ("work.mpl", work_source)],
    );
    write_artifact(&artifacts.join("mesh.toml"), &manifest);
    write_artifact(&artifacts.join("main.mpl"), main_source);
    write_artifact(&artifacts.join("work.mpl"), work_source);

    let binary_path = project_dir.join(name);
    let output = Command::new(meshc_bin())
        .current_dir(repo_root())
        .arg("build")
        .arg(&project_dir)
        .arg("--output")
        .arg(&binary_path)
        .output()
        .expect("failed to invoke meshc build for tiny route-free runtime project");
    write_artifact(&artifacts.join("build.log"), command_output_text(&output));
    assert!(
        output.status.success(),
        "tiny route-free runtime project build should succeed:\n{}",
        command_output_text(&output)
    );

    RouteFreeRuntimeProject {
        _tempdir: tmpdir,
        project_dir,
        binary_path,
    }
}

fn route_free_expected_node_name(config: &RouteFreeNodeConfig) -> String {
    let host = if config.advertise_host.contains(':') {
        format!("[{}]", config.advertise_host)
    } else {
        config.advertise_host.clone()
    };
    format!("{}@{}:{}", config.node_basename, host, config.cluster_port)
}

fn route_free_node_name(cluster_port: u16) -> String {
    route_free_expected_node_name(&RouteFreeNodeConfig {
        node_basename: "startup-cli-proof".to_string(),
        advertise_host: LOOPBACK_V4.to_string(),
        cluster_port,
        cluster_role: "primary".to_string(),
        promotion_epoch: 0,
    })
}

fn spawn_route_free_runtime(
    project: &RouteFreeRuntimeProject,
    artifacts: &Path,
    log_label: &str,
    node_name: &str,
    cluster_port: u16,
    cluster_role: &str,
    promotion_epoch: u64,
) -> SpawnedProcess {
    let stdout_path = artifacts.join(format!("{log_label}.stdout.log"));
    let stderr_path = artifacts.join(format!("{log_label}.stderr.log"));
    let stdout = File::create(&stdout_path).expect("failed to create runtime stdout log");
    let stderr = File::create(&stderr_path).expect("failed to create runtime stderr log");

    let child = Command::new(&project.binary_path)
        .current_dir(&project.project_dir)
        .env("MESH_CLUSTER_COOKIE", SHARED_COOKIE)
        .env("MESH_NODE_NAME", node_name)
        .env("MESH_DISCOVERY_SEED", "localhost")
        .env("MESH_CLUSTER_PORT", cluster_port.to_string())
        .env("MESH_CONTINUITY_ROLE", cluster_role)
        .env(
            "MESH_CONTINUITY_PROMOTION_EPOCH",
            promotion_epoch.to_string(),
        )
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .spawn()
        .expect("failed to start route-free runtime project");

    SpawnedProcess {
        child,
        stdout_path,
        stderr_path,
    }
}

fn stop_process(mut spawned: SpawnedProcess) -> StoppedProcess {
    let _ = spawned.child.kill();
    let _ = spawned.child.wait();

    let stdout = fs::read_to_string(&spawned.stdout_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", spawned.stdout_path.display()));
    let stderr = fs::read_to_string(&spawned.stderr_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", spawned.stderr_path.display()));
    let combined = format!("{stdout}{stderr}");

    StoppedProcess {
        stdout,
        stderr,
        combined,
    }
}

fn run_meshc_cluster(
    artifacts: &Path,
    name: &str,
    args: &[&str],
    cookie_env: Option<&str>,
) -> Output {
    let mut command = Command::new(meshc_bin());
    command.current_dir(repo_root()).args(args);
    if let Some(cookie) = cookie_env {
        command.env("MESH_CLUSTER_COOKIE", cookie);
    }
    let output = command
        .output()
        .unwrap_or_else(|error| panic!("failed to run meshc {:?}: {error}", args));
    write_artifact(
        &artifacts.join(format!("{name}.log")),
        command_output_text(&output),
    );
    output
}

fn parse_json_output(artifacts: &Path, name: &str, output: &Output, context: &str) -> Value {
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let json = serde_json::from_str::<Value>(&stdout)
        .unwrap_or_else(|error| panic!("{context} returned invalid JSON: {error}\n{stdout}"));
    write_artifact(
        &artifacts.join(format!("{name}.json")),
        serde_json::to_string_pretty(&json).expect("json pretty print failed"),
    );
    json
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

fn diagnostic_entries_for_request<'a>(snapshot: &'a Value, request_key: &str) -> Vec<&'a Value> {
    snapshot["entries"]
        .as_array()
        .unwrap_or_else(|| panic!("missing diagnostics entries array in {snapshot}"))
        .iter()
        .filter(|entry| entry["request_key"].as_str() == Some(request_key))
        .collect()
}

fn wait_for_cluster_status_membership(
    artifacts: &Path,
    name: &str,
    node_name: &str,
    expected_peer_nodes: &[String],
    expected_nodes: &[String],
    expected_role: &str,
    expected_epoch: u64,
    allowed_health: &[&str],
) -> Value {
    let start = Instant::now();
    let mut last_json = None;

    while start.elapsed() < STATUS_TIMEOUT {
        let output = run_meshc_cluster(
            artifacts,
            name,
            &["cluster", "status", node_name, "--json"],
            Some(SHARED_COOKIE),
        );
        if output.status.success() {
            let json = parse_json_output(artifacts, name, &output, "cluster status");
            last_json = Some(json.clone());
            let replication_health = required_str(&json["authority"], "replication_health");
            if required_str(&json["membership"], "local_node") == node_name
                && sorted(&required_string_list(&json["membership"], "peer_nodes"))
                    == sorted(expected_peer_nodes)
                && sorted(&required_string_list(&json["membership"], "nodes"))
                    == sorted(expected_nodes)
                && required_str(&json["authority"], "cluster_role") == expected_role
                && required_u64(&json["authority"], "promotion_epoch") == expected_epoch
                && allowed_health.contains(&replication_health.as_str())
            {
                return json;
            }
        }
        sleep(Duration::from_millis(200));
    }

    panic!(
        "meshc cluster status {} did not converge within {:?}; last response: {:?}",
        node_name, STATUS_TIMEOUT, last_json
    );
}

fn wait_for_cluster_status_ready(artifacts: &Path, node_name: &str) -> Value {
    let start = Instant::now();
    let mut last_json = None;

    while start.elapsed() < STATUS_TIMEOUT {
        let output = run_meshc_cluster(
            artifacts,
            "cluster-status-ready",
            &["cluster", "status", node_name, "--json"],
            Some(SHARED_COOKIE),
        );
        if output.status.success() {
            let json =
                parse_json_output(artifacts, "cluster-status-ready", &output, "cluster status");
            last_json = Some(json.clone());
            if required_str(&json["membership"], "local_node") == node_name
                && required_string_list(&json["membership"], "peer_nodes").is_empty()
                && required_string_list(&json["membership"], "nodes") == vec![node_name.to_string()]
                && required_str(&json["authority"], "cluster_role") == "primary"
                && required_u64(&json["authority"], "promotion_epoch") == 0
                && required_str(&json["authority"], "replication_health") == "local_only"
            {
                return json;
            }
        }
        sleep(Duration::from_millis(200));
    }

    panic!(
        "meshc cluster status {} did not converge within {:?}; last response: {:?}",
        node_name, STATUS_TIMEOUT, last_json
    );
}

fn wait_for_runtime_names_discovered_with_label(
    artifacts: &Path,
    name: &str,
    node_name: &str,
    expected_runtime_names: &[&str],
) -> Value {
    let start = Instant::now();
    let mut last_json = None;

    while start.elapsed() < CONTINUITY_TIMEOUT {
        let output = run_meshc_cluster(
            artifacts,
            name,
            &["cluster", "continuity", node_name, "--json"],
            Some(SHARED_COOKIE),
        );
        if output.status.success() {
            let json = parse_json_output(artifacts, name, &output, "cluster continuity list");
            last_json = Some(json.clone());
            let Some(records) = json["records"].as_array() else {
                panic!("cluster continuity list JSON missing records array: {json}");
            };
            let all_present = expected_runtime_names.iter().all(|runtime_name| {
                records.iter().any(|record| {
                    record["declared_handler_runtime_name"].as_str() == Some(*runtime_name)
                        && !record["request_key"].as_str().unwrap_or("").is_empty()
                })
            });
            if all_present {
                return json;
            }
        }
        sleep(Duration::from_millis(200));
    }

    panic!(
        "meshc cluster continuity {} did not surface runtime names {:?} within {:?}; last response: {:?}",
        node_name, expected_runtime_names, CONTINUITY_TIMEOUT, last_json
    );
}

fn wait_for_runtime_names_discovered(
    artifacts: &Path,
    node_name: &str,
    expected_runtime_names: &[&str],
) -> Value {
    wait_for_runtime_names_discovered_with_label(
        artifacts,
        "cluster-continuity-list-json",
        node_name,
        expected_runtime_names,
    )
}

fn count_records_for_runtime_name(list_json: &Value, runtime_name: &str) -> usize {
    list_json["records"]
        .as_array()
        .unwrap_or_else(|| panic!("missing continuity records array in {list_json}"))
        .iter()
        .filter(|record| record["declared_handler_runtime_name"].as_str() == Some(runtime_name))
        .count()
}

fn wait_for_continuity_record_completed(
    artifacts: &Path,
    name: &str,
    node_name: &str,
    request_key: &str,
    expected_runtime_name: &str,
) -> Value {
    let start = Instant::now();
    let mut last_json = None;

    while start.elapsed() < CONTINUITY_TIMEOUT {
        let output = run_meshc_cluster(
            artifacts,
            name,
            &["cluster", "continuity", node_name, request_key, "--json"],
            Some(SHARED_COOKIE),
        );
        if output.status.success() {
            let json =
                parse_json_output(artifacts, name, &output, "cluster continuity single record");
            last_json = Some(json.clone());
            let record = &json["record"];
            if required_str(record, "request_key") == request_key
                && required_str(record, "declared_handler_runtime_name") == expected_runtime_name
                && required_str(record, "phase") == "completed"
                && required_str(record, "result") == "succeeded"
            {
                return json;
            }
        }
        sleep(Duration::from_millis(200));
    }

    panic!(
        "meshc cluster continuity {} {} did not reach completed within {:?}; last response: {:?}",
        node_name, request_key, CONTINUITY_TIMEOUT, last_json
    );
}

fn wait_for_startup_diagnostics(
    artifacts: &Path,
    primary_node: &str,
    standby_node: &str,
    request_key: &str,
) -> (Value, Value) {
    let start = Instant::now();
    let mut last_primary = None;
    let mut last_standby = None;

    while start.elapsed() < DIAGNOSTIC_TIMEOUT {
        let primary_output = run_meshc_cluster(
            artifacts,
            "cluster-diagnostics-primary",
            &["cluster", "diagnostics", primary_node, "--json"],
            Some(SHARED_COOKIE),
        );
        let standby_output = run_meshc_cluster(
            artifacts,
            "cluster-diagnostics-standby",
            &["cluster", "diagnostics", standby_node, "--json"],
            Some(SHARED_COOKIE),
        );
        if primary_output.status.success() && standby_output.status.success() {
            let primary_json = parse_json_output(
                artifacts,
                "cluster-diagnostics-primary",
                &primary_output,
                "cluster diagnostics",
            );
            let standby_json = parse_json_output(
                artifacts,
                "cluster-diagnostics-standby",
                &standby_output,
                "cluster diagnostics",
            );
            last_primary = Some(primary_json.clone());
            last_standby = Some(standby_json.clone());

            let primary_entries = diagnostic_entries_for_request(&primary_json, request_key);
            let standby_entries = diagnostic_entries_for_request(&standby_json, request_key);
            let combined_transitions: Vec<_> = primary_entries
                .iter()
                .chain(standby_entries.iter())
                .filter_map(|entry| entry["transition"].as_str())
                .collect();
            let has_trigger = combined_transitions.contains(&"startup_trigger");
            let has_completed = combined_transitions.contains(&"startup_completed");
            let has_failure = combined_transitions.contains(&"startup_rejected")
                || combined_transitions.contains(&"startup_convergence_timeout");
            if has_trigger && has_completed && !has_failure {
                return (primary_json, standby_json);
            }
        }
        sleep(Duration::from_millis(200));
    }

    panic!(
        "meshc cluster diagnostics did not surface startup truth for {} within {:?}; last primary: {:?}; last standby: {:?}",
        request_key, DIAGNOSTIC_TIMEOUT, last_primary, last_standby
    );
}

fn record_for_runtime_name<'a>(list_json: &'a Value, runtime_name: &str) -> &'a Value {
    list_json["records"]
        .as_array()
        .and_then(|records| {
            records.iter().find(|record| {
                record["declared_handler_runtime_name"].as_str() == Some(runtime_name)
            })
        })
        .unwrap_or_else(|| panic!("missing runtime name {runtime_name} in {list_json}"))
}

#[test]
fn m046_s02_cli_tiny_route_free_startup_dedupes_on_two_nodes() {
    let manifest = package_manifest("startup-tiny-proof");
    let main_source = tiny_route_free_startup_main_source();
    let work_source = tiny_route_free_startup_work_source();
    assert!(!manifest.contains("[cluster]"));
    assert!(!manifest.contains("declarations"));
    assert!(!main_source.contains("HTTP.serve"));
    assert!(!main_source.contains("/work"));
    assert!(!main_source.contains("/status"));
    assert!(!main_source.contains("Continuity.submit_declared_work"));
    assert_eq!(main_source.matches("Node.start_from_env()").count(), 1);
    assert!(work_source.contains("clustered(work)"));
    assert!(work_source.contains("1 + 1"));
    assert!(!work_source.contains("HTTP.serve"));
    assert!(!work_source.contains("/work"));
    assert!(!work_source.contains("/status"));
    assert!(!work_source.contains("Continuity.submit_declared_work"));
    assert!(!work_source.contains("Continuity.mark_completed"));
    assert!(!work_source.contains("owner_node"));
    assert!(!work_source.contains("replica_node"));
    assert!(!work_source.contains("Timer.sleep"));

    let artifacts = artifact_dir("cli-tiny-route-free-startup-two-node");
    let project = build_tiny_route_free_runtime_project("startup-tiny-proof", &artifacts);
    let cluster_port = dual_stack_cluster_port();
    let primary_config = RouteFreeNodeConfig {
        node_basename: "startup-tiny-primary".to_string(),
        advertise_host: LOOPBACK_V4.to_string(),
        cluster_port,
        cluster_role: "primary".to_string(),
        promotion_epoch: 0,
    };
    let standby_config = RouteFreeNodeConfig {
        node_basename: "startup-tiny-standby".to_string(),
        advertise_host: LOOPBACK_V6.to_string(),
        cluster_port,
        cluster_role: "standby".to_string(),
        promotion_epoch: 0,
    };
    let primary_node = route_free_expected_node_name(&primary_config);
    let standby_node = route_free_expected_node_name(&standby_config);
    let expected_nodes = vec![primary_node.clone(), standby_node.clone()];

    let primary_proc = spawn_route_free_runtime(
        &project,
        &artifacts,
        "primary",
        &primary_node,
        cluster_port,
        &primary_config.cluster_role,
        primary_config.promotion_epoch,
    );
    let standby_proc = spawn_route_free_runtime(
        &project,
        &artifacts,
        "standby",
        &standby_node,
        cluster_port,
        &standby_config.cluster_role,
        standby_config.promotion_epoch,
    );

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        wait_for_cluster_status_membership(
            &artifacts,
            "cluster-status-primary",
            &primary_node,
            std::slice::from_ref(&standby_node),
            &expected_nodes,
            "primary",
            0,
            &["local_only", "healthy"],
        );
        wait_for_cluster_status_membership(
            &artifacts,
            "cluster-status-standby",
            &standby_node,
            std::slice::from_ref(&primary_node),
            &expected_nodes,
            "standby",
            0,
            &["local_only", "healthy"],
        );

        let expected_runtime_names = ["Work.execute_declared_work"];
        let primary_list = wait_for_runtime_names_discovered_with_label(
            &artifacts,
            "cluster-continuity-list-primary",
            &primary_node,
            &expected_runtime_names,
        );
        let standby_list = wait_for_runtime_names_discovered_with_label(
            &artifacts,
            "cluster-continuity-list-standby",
            &standby_node,
            &expected_runtime_names,
        );
        assert_eq!(required_u64(&primary_list, "total_records"), 1);
        assert_eq!(required_u64(&standby_list, "total_records"), 1);
        assert!(!required_bool(&primary_list, "truncated"));
        assert!(!required_bool(&standby_list, "truncated"));
        assert_eq!(
            count_records_for_runtime_name(&primary_list, "Work.execute_declared_work"),
            1
        );
        assert_eq!(
            count_records_for_runtime_name(&standby_list, "Work.execute_declared_work"),
            1
        );

        let request_key = required_str(
            record_for_runtime_name(&primary_list, "Work.execute_declared_work"),
            "request_key",
        );
        assert_eq!(
            request_key,
            required_str(
                record_for_runtime_name(&standby_list, "Work.execute_declared_work"),
                "request_key",
            )
        );

        let primary_continuity = wait_for_continuity_record_completed(
            &artifacts,
            "cluster-continuity-primary-completed",
            &primary_node,
            &request_key,
            "Work.execute_declared_work",
        );
        let standby_continuity = wait_for_continuity_record_completed(
            &artifacts,
            "cluster-continuity-standby-completed",
            &standby_node,
            &request_key,
            "Work.execute_declared_work",
        );

        for record in [&primary_continuity["record"], &standby_continuity["record"]] {
            let owner_node = required_str(record, "owner_node");
            let replica_node = required_str(record, "replica_node");
            assert_eq!(required_str(record, "request_key"), request_key);
            assert_eq!(
                required_str(record, "declared_handler_runtime_name"),
                "Work.execute_declared_work"
            );
            assert_eq!(required_str(record, "phase"), "completed");
            assert_eq!(required_str(record, "result"), "succeeded");
            assert!(expected_nodes.contains(&owner_node));
            assert!(expected_nodes.contains(&replica_node));
            assert_ne!(owner_node, replica_node);
            assert_eq!(required_str(record, "execution_node"), owner_node);
            assert_eq!(required_str(record, "replica_status"), "mirrored");
            assert_eq!(required_str(record, "error"), "");
        }

        let (primary_diagnostics, standby_diagnostics) =
            wait_for_startup_diagnostics(&artifacts, &primary_node, &standby_node, &request_key);
        let primary_entries = diagnostic_entries_for_request(&primary_diagnostics, &request_key);
        let standby_entries = diagnostic_entries_for_request(&standby_diagnostics, &request_key);
        let combined_transitions: Vec<_> = primary_entries
            .iter()
            .chain(standby_entries.iter())
            .filter_map(|entry| entry["transition"].as_str())
            .collect();
        assert!(combined_transitions.contains(&"startup_trigger"));
        assert!(combined_transitions.contains(&"startup_completed"));
        assert!(!combined_transitions.contains(&"startup_rejected"));
        assert!(!combined_transitions.contains(&"startup_convergence_timeout"));
    }));

    let primary_logs = stop_process(primary_proc);
    let standby_logs = stop_process(standby_proc);
    write_artifact(
        &artifacts.join("primary.combined.log"),
        &primary_logs.combined,
    );
    write_artifact(
        &artifacts.join("standby.combined.log"),
        &standby_logs.combined,
    );
    if let Err(payload) = result {
        panic!(
            "{}\nartifacts: {}\nprimary stdout:\n{}\nprimary stderr:\n{}\nstandby stdout:\n{}\nstandby stderr:\n{}",
            panic_payload_to_string(payload),
            artifacts.display(),
            primary_logs.stdout,
            primary_logs.stderr,
            standby_logs.stdout,
            standby_logs.stderr
        );
    }

    assert!(
        primary_logs
            .combined
            .contains("[startup-tiny-proof] runtime bootstrap mode=cluster"),
        "primary runtime bootstrap log missing:\n{}",
        primary_logs.combined
    );
    assert!(
        primary_logs
            .combined
            .contains(&format!("node={primary_node}")),
        "primary runtime bootstrap log missing node name {}:\n{}",
        primary_node,
        primary_logs.combined
    );
    assert!(
        standby_logs
            .combined
            .contains("[startup-tiny-proof] runtime bootstrap mode=cluster"),
        "standby runtime bootstrap log missing:\n{}",
        standby_logs.combined
    );
    assert!(
        standby_logs
            .combined
            .contains(&format!("node={standby_node}")),
        "standby runtime bootstrap log missing node name {}:\n{}",
        standby_node,
        standby_logs.combined
    );
}

#[test]
fn m046_s02_cli_route_free_startup_work_is_discoverable_from_list_and_single_record_output() {
    let main_source = route_free_startup_main_source();
    let work_source = route_free_startup_work_source();
    assert!(!main_source.contains("HTTP.serve"));
    assert!(!main_source.contains("/work"));
    assert!(!main_source.contains("/status"));
    assert!(!main_source.contains("Continuity.submit_declared_work"));
    assert!(!work_source.contains("Continuity.mark_completed"));

    let artifacts = artifact_dir("cli-route-free-startup-discovery");
    let project = build_route_free_runtime_project("startup-cli-proof");
    let cluster_port = unused_port();
    let node_name = route_free_node_name(cluster_port);
    let spawned = spawn_route_free_runtime(
        &project,
        &artifacts,
        "runtime",
        &node_name,
        cluster_port,
        "primary",
        0,
    );

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let status = wait_for_cluster_status_ready(&artifacts, &node_name);
        assert_eq!(required_str(&status["membership"], "local_node"), node_name);
        assert!(required_string_list(&status["membership"], "peer_nodes").is_empty());

        let expected_runtime_names = ["Work.execute_declared_work", "Work.execute_aux_work"];
        let list_json =
            wait_for_runtime_names_discovered(&artifacts, &node_name, &expected_runtime_names);
        assert!(
            required_u64(&list_json, "total_records") >= expected_runtime_names.len() as u64,
            "expected at least {} continuity records, got {}",
            expected_runtime_names.len(),
            required_u64(&list_json, "total_records")
        );
        assert!(!required_bool(&list_json, "truncated"));

        for runtime_name in expected_runtime_names {
            let record = record_for_runtime_name(&list_json, runtime_name);
            assert_eq!(
                required_str(record, "declared_handler_runtime_name"),
                runtime_name
            );
            assert!(!required_str(record, "request_key").is_empty());
        }

        let human_list = run_meshc_cluster(
            &artifacts,
            "cluster-continuity-list-human",
            &["cluster", "continuity", &node_name],
            Some(SHARED_COOKIE),
        );
        assert!(
            human_list.status.success(),
            "human continuity list should succeed:\n{}",
            command_output_text(&human_list)
        );
        let human_list_stdout = String::from_utf8_lossy(&human_list.stdout);
        assert!(
            human_list_stdout.contains("declared_handler_runtime_name=Work.execute_declared_work"),
            "human continuity list should surface the first runtime name:\n{human_list_stdout}"
        );
        assert!(
            human_list_stdout.contains("declared_handler_runtime_name=Work.execute_aux_work"),
            "human continuity list should surface the second runtime name:\n{human_list_stdout}"
        );

        let request_key = required_str(
            record_for_runtime_name(&list_json, "Work.execute_declared_work"),
            "request_key",
        );
        let single_json_output = run_meshc_cluster(
            &artifacts,
            "cluster-continuity-single-json",
            &["cluster", "continuity", &node_name, &request_key, "--json"],
            Some(SHARED_COOKIE),
        );
        assert!(
            single_json_output.status.success(),
            "single-record continuity JSON should succeed:\n{}",
            command_output_text(&single_json_output)
        );
        let single_json = parse_json_output(
            &artifacts,
            "cluster-continuity-single-json",
            &single_json_output,
            "cluster continuity single record",
        );
        assert_eq!(
            required_str(&single_json["record"], "request_key"),
            request_key
        );
        assert_eq!(
            required_str(&single_json["record"], "declared_handler_runtime_name"),
            "Work.execute_declared_work"
        );

        let single_human = run_meshc_cluster(
            &artifacts,
            "cluster-continuity-single-human",
            &["cluster", "continuity", &node_name, &request_key],
            Some(SHARED_COOKIE),
        );
        assert!(
            single_human.status.success(),
            "single-record continuity human output should succeed:\n{}",
            command_output_text(&single_human)
        );
        let single_human_stdout = String::from_utf8_lossy(&single_human.stdout);
        assert!(
            single_human_stdout
                .contains("declared_handler_runtime_name: Work.execute_declared_work"),
            "single-record continuity output should surface the runtime name:\n{single_human_stdout}"
        );
    }));

    let logs = stop_process(spawned);
    write_artifact(&artifacts.join("runtime.combined.log"), &logs.combined);
    if let Err(payload) = result {
        panic!(
            "{}\nartifacts: {}\nstdout:\n{}\nstderr:\n{}",
            panic_payload_to_string(payload),
            artifacts.display(),
            logs.stdout,
            logs.stderr
        );
    }
}

#[test]
fn m046_s02_cli_continuity_failures_stay_explicit_for_invalid_request_and_auth() {
    let artifacts = artifact_dir("cli-continuity-failures");
    let project = build_route_free_runtime_project("startup-cli-proof-failures");
    let cluster_port = unused_port();
    let node_name = route_free_node_name(cluster_port);
    let spawned = spawn_route_free_runtime(
        &project,
        &artifacts,
        "runtime",
        &node_name,
        cluster_port,
        "primary",
        0,
    );

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        wait_for_cluster_status_ready(&artifacts, &node_name);

        let empty_request_key = run_meshc_cluster(
            &artifacts,
            "cluster-continuity-empty-request-key",
            &["cluster", "continuity", &node_name, "", "--json"],
            Some(SHARED_COOKIE),
        );
        assert!(
            !empty_request_key.status.success(),
            "empty request key should fail closed"
        );
        let empty_request_stderr = String::from_utf8_lossy(&empty_request_key.stderr);
        assert!(
            empty_request_stderr.contains("error:")
                && empty_request_stderr.contains("request_key_missing"),
            "empty request key failure should stay explicit, got:\n{empty_request_stderr}"
        );

        let wrong_cookie = run_meshc_cluster(
            &artifacts,
            "cluster-continuity-wrong-cookie",
            &[
                "cluster",
                "continuity",
                &node_name,
                "--json",
                "--cookie",
                "wrong-cookie",
            ],
            None,
        );
        assert!(
            !wrong_cookie.status.success(),
            "wrong cookie should fail closed"
        );
        let wrong_cookie_stderr = String::from_utf8_lossy(&wrong_cookie.stderr);
        assert!(
            wrong_cookie_stderr.contains("error:")
                && (wrong_cookie_stderr.contains("cookie mismatch")
                    || wrong_cookie_stderr.contains("authentication failed")
                    || wrong_cookie_stderr.contains("handshake")),
            "wrong cookie failure should mention the operator/auth seam, got:\n{wrong_cookie_stderr}"
        );
    }));

    let logs = stop_process(spawned);
    write_artifact(&artifacts.join("runtime.combined.log"), &logs.combined);
    if let Err(payload) = result {
        panic!(
            "{}\nartifacts: {}\nstdout:\n{}\nstderr:\n{}",
            panic_payload_to_string(payload),
            artifacts.display(),
            logs.stdout,
            logs.stderr
        );
    }
}
