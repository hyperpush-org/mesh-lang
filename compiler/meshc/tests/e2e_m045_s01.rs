mod support;

use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};
use support::m046_route_free as route_free;

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

fn run_meshc(args: &[&str]) -> Output {
    Command::new(meshc_bin())
        .current_dir(repo_root())
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("failed to invoke meshc {:?}: {error}", args))
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

fn ensure_mesh_rt_staticlib() {
    static BUILD_ONCE: OnceLock<()> = OnceLock::new();
    BUILD_ONCE.get_or_init(|| {
        let output = Command::new("cargo")
            .current_dir(repo_root())
            .args(["build", "-q", "-p", "mesh-rt"])
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

fn artifact_dir(test_name: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = repo_root()
        .join(".tmp")
        .join("m045-s01")
        .join(format!("{test_name}-{stamp}"));
    fs::create_dir_all(&dir).expect("failed to create e2e artifact dir");
    dir
}

fn write_artifact(path: &Path, contents: impl AsRef<str>) {
    fs::write(path, contents.as_ref())
        .unwrap_or_else(|error| panic!("failed to write artifact {}: {error}", path.display()));
}

fn write_mesh_program(project_dir: &Path, source: &str) {
    fs::create_dir_all(project_dir).expect("failed to create project dir");
    fs::write(project_dir.join("main.mpl"), source).expect("failed to write main.mpl");
}

fn build_mesh_project(project_dir: &Path) -> Output {
    Command::new(meshc_bin())
        .current_dir(repo_root())
        .args(["build", project_dir.to_str().unwrap()])
        .output()
        .expect("failed to invoke meshc build")
}

fn build_only_mesh(source: &str, artifacts: &Path) -> Output {
    ensure_mesh_rt_staticlib();

    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
    let project_dir = temp_dir.path().join("project");
    write_mesh_program(&project_dir, source);
    write_artifact(&artifacts.join("main.mpl"), source);

    let output = build_mesh_project(&project_dir);
    write_artifact(&artifacts.join("build.log"), command_output_text(&output));
    output
}

fn build_and_run_mesh(source: &str, envs: &[(&str, String)], artifacts: &Path) -> Output {
    ensure_mesh_rt_staticlib();

    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
    let project_dir = temp_dir.path().join("project");
    write_mesh_program(&project_dir, source);
    write_artifact(&artifacts.join("main.mpl"), source);

    let build_output = build_mesh_project(&project_dir);
    write_artifact(
        &artifacts.join("build.log"),
        command_output_text(&build_output),
    );
    assert!(
        build_output.status.success(),
        "meshc build failed:\n{}\nartifacts: {}",
        command_output_text(&build_output),
        artifacts.display()
    );

    let binary = project_dir.join("project");
    let mut command = Command::new(&binary);
    command.current_dir(&project_dir);
    for (key, value) in envs {
        command.env(key, value);
    }
    let run_output = command
        .output()
        .unwrap_or_else(|error| panic!("failed to run {}: {error}", binary.display()));

    write_artifact(&artifacts.join("run.log"), command_output_text(&run_output));
    run_output
}

fn stdout_lines(output: &Output) -> Vec<String> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn unused_port() -> u16 {
    TcpListener::bind(("127.0.0.1", 0))
        .expect("failed to bind ephemeral port")
        .local_addr()
        .expect("failed to read ephemeral port")
        .port()
}

const BOOTSTRAP_STATUS_SOURCE: &str = r##"
fn print_status(label :: String, status :: BootstrapStatus) do
  println(
    "#{label}|#{status.mode}|#{status.node_name}|#{status.cluster_port}|#{status.discovery_seed}"
  )
end

fn main() do
  case Node.start_from_env() do
    Ok( status) -> print_status("ok", status)
    Err( reason) -> println("err|#{reason}")
  end
end
"##;

#[test]
fn m045_s01_bootstrap_api_standalone_reports_typed_status() {
    let artifacts = artifact_dir("bootstrap-api-standalone");
    let output = build_and_run_mesh(BOOTSTRAP_STATUS_SOURCE, &[], &artifacts);
    assert!(
        output.status.success(),
        "bootstrap standalone runtime should exit 0:\n{}",
        command_output_text(&output)
    );
    assert_eq!(
        stdout_lines(&output),
        vec!["ok|standalone||4370|".to_string()]
    );
}

#[test]
fn m045_s01_bootstrap_api_explicit_node_starts_cluster_mode() {
    let artifacts = artifact_dir("bootstrap-api-explicit-node");
    let cluster_port = unused_port();
    let node_name = format!("app@127.0.0.1:{cluster_port}");
    let output = build_and_run_mesh(
        BOOTSTRAP_STATUS_SOURCE,
        &[
            ("MESH_CLUSTER_COOKIE", "dev-cookie".to_string()),
            ("MESH_NODE_NAME", node_name.clone()),
            ("MESH_DISCOVERY_SEED", "localhost".to_string()),
            ("MESH_CLUSTER_PORT", cluster_port.to_string()),
        ],
        &artifacts,
    );
    assert!(
        output.status.success(),
        "bootstrap explicit-node runtime should exit 0:\n{}",
        command_output_text(&output)
    );
    assert_eq!(
        stdout_lines(&output),
        vec![format!("ok|cluster|{node_name}|{cluster_port}|localhost")]
    );
}

#[test]
fn m045_s01_bootstrap_api_fly_identity_falls_back_when_node_name_absent() {
    let artifacts = artifact_dir("bootstrap-api-fly-identity");
    let cluster_port = unused_port();
    let expected_node = format!("demo-dfw-machine-1@127.0.0.1:{cluster_port}");
    let output = build_and_run_mesh(
        BOOTSTRAP_STATUS_SOURCE,
        &[
            ("MESH_CLUSTER_COOKIE", "dev-cookie".to_string()),
            ("MESH_DISCOVERY_SEED", "localhost".to_string()),
            ("MESH_CLUSTER_PORT", cluster_port.to_string()),
            ("FLY_APP_NAME", "demo".to_string()),
            ("FLY_REGION", "dfw".to_string()),
            ("FLY_MACHINE_ID", "machine-1".to_string()),
            ("FLY_PRIVATE_IP", "127.0.0.1".to_string()),
        ],
        &artifacts,
    );
    assert!(
        output.status.success(),
        "bootstrap fly-identity runtime should exit 0:\n{}",
        command_output_text(&output)
    );
    assert_eq!(
        stdout_lines(&output),
        vec![format!(
            "ok|cluster|{expected_node}|{cluster_port}|localhost"
        )]
    );
}

#[test]
fn m045_s01_bootstrap_api_fail_closed_without_cookie() {
    let artifacts = artifact_dir("bootstrap-api-fail-closed-no-cookie");
    let output = build_and_run_mesh(
        BOOTSTRAP_STATUS_SOURCE,
        &[
            ("MESH_NODE_NAME", "app@127.0.0.1:4370".to_string()),
            ("MESH_DISCOVERY_SEED", "localhost".to_string()),
        ],
        &artifacts,
    );
    assert!(
        output.status.success(),
        "bootstrap no-cookie runtime should exit 0:\n{}",
        command_output_text(&output)
    );
    let lines = stdout_lines(&output);
    assert_eq!(
        lines.len(),
        1,
        "expected one bootstrap output line: {lines:?}"
    );
    assert_eq!(
        lines[0],
        "err|MESH_CLUSTER_COOKIE is required when discovery or identity env is set"
    );
}

#[test]
fn m045_s01_bootstrap_api_bind_failure_surfaces_runtime_error() {
    let artifacts = artifact_dir("bootstrap-api-bind-failure");
    let listener = TcpListener::bind(("127.0.0.1", 0)).expect("failed to hold cluster port");
    let cluster_port = listener
        .local_addr()
        .expect("failed to read held port")
        .port();
    let node_name = format!("app@127.0.0.1:{cluster_port}");
    let output = build_and_run_mesh(
        BOOTSTRAP_STATUS_SOURCE,
        &[
            ("MESH_CLUSTER_COOKIE", "dev-cookie".to_string()),
            ("MESH_NODE_NAME", node_name.clone()),
            ("MESH_DISCOVERY_SEED", "localhost".to_string()),
            ("MESH_CLUSTER_PORT", cluster_port.to_string()),
        ],
        &artifacts,
    );
    drop(listener);
    assert!(
        output.status.success(),
        "bootstrap bind-failure runtime should exit 0:\n{}",
        command_output_text(&output)
    );
    assert_eq!(
        stdout_lines(&output),
        vec![format!(
            "err|mesh bootstrap start failed node={node_name}: listener bind failed"
        )]
    );
}

#[test]
fn m045_s01_bootstrap_api_wrong_arity_fails_at_compile_time() {
    let artifacts = artifact_dir("bootstrap-api-wrong-arity");
    let output = build_only_mesh(
        r##"
fn main() do
  let _ = Node.start_from_env("extra")
end
"##,
        &artifacts,
    );
    let combined = command_output_text(&output);
    assert!(
        !output.status.success(),
        "wrong-arity bootstrap call should fail compilation; artifacts: {}",
        artifacts.display()
    );
    assert!(
        combined.contains("expected 0 argument(s), found 1"),
        "expected wrong-arity bootstrap diagnostic:\n{}",
        combined
    );
}

#[test]
fn m045_s01_bootstrap_api_missing_field_fails_at_compile_time() {
    let artifacts = artifact_dir("bootstrap-api-missing-field");
    let output = build_only_mesh(
        r##"
fn main() do
  case Node.start_from_env() do
    Ok( status) -> println(status.cookie)
    Err( reason) -> println(reason)
  end
end
"##,
        &artifacts,
    );
    let combined = command_output_text(&output);
    assert!(
        !output.status.success(),
        "missing bootstrap field access should fail compilation; artifacts: {}",
        artifacts.display()
    );
    assert!(
        combined.contains("type BootstrapStatus has no field cookie")
            || combined.contains("no field `cookie`"),
        "expected bootstrap missing-field diagnostic:\n{}",
        combined
    );
}

#[test]
fn m045_s01_bootstrap_api_invalid_int_use_fails_at_compile_time() {
    let artifacts = artifact_dir("bootstrap-api-invalid-int-use");
    let output = build_only_mesh(
        r##"
fn main() do
  let started :: Int = Node.start_from_env()
  println("#{started}")
end
"##,
        &artifacts,
    );
    let combined = command_output_text(&output);
    assert!(
        !output.status.success(),
        "invalid Int bootstrap use should fail compilation; artifacts: {}",
        artifacts.display()
    );
    assert!(
        combined.contains("expected Result<BootstrapStatus, String>, found Int"),
        "expected bootstrap type-mismatch diagnostic:\n{}",
        combined
    );
}

#[test]
fn m045_s01_bootstrap_api_scaffold_contract_uses_runtime_owned_bootstrap() {
    let artifacts = artifact_dir("bootstrap-api-scaffold-contract");
    let temp = tempfile::tempdir().expect("create scaffold tempdir");
    let project_name = "clustered-scaffold";

    let init = Command::new(meshc_bin())
        .current_dir(temp.path())
        .args(["init", "--clustered", project_name])
        .output()
        .expect("failed to run meshc init --clustered");
    write_artifact(&artifacts.join("init.log"), command_output_text(&init));
    assert!(
        init.status.success(),
        "meshc init --clustered should succeed:\n{}",
        command_output_text(&init)
    );

    let project_dir = temp.path().join(project_name);
    let main = fs::read_to_string(project_dir.join("main.mpl")).expect("read scaffolded main.mpl");
    let work = fs::read_to_string(project_dir.join("work.mpl")).expect("read scaffolded work.mpl");
    let readme =
        fs::read_to_string(project_dir.join("README.md")).expect("read scaffolded README.md");
    write_artifact(&artifacts.join("main.mpl"), &main);
    write_artifact(&artifacts.join("work.mpl"), &work);
    write_artifact(&artifacts.join("README.md"), &readme);

    assert!(main.contains("Node.start_from_env()"));
    assert!(main.contains("BootstrapStatus"));
    assert!(main.contains("runtime bootstrap"));
    assert!(!main.contains("Continuity.submit_declared_work"));
    assert!(!main.contains("Continuity.mark_completed"));
    assert!(!main.contains("handle_status"));
    assert!(!main.contains("HTTP.serve"));
    assert!(!main.contains("/health"));
    assert!(!main.contains("/work"));
    assert!(!main.contains("MESH_CLUSTER_COOKIE"));
    assert!(!main.contains("MESH_NODE_NAME"));
    assert!(!main.contains("MESH_DISCOVERY_SEED"));
    assert!(!main.contains("Node.start("));
    assert!(!main.contains("CLUSTER_PROOF_"));

    assert!(work.contains(route_free::STARTUP_SOURCE_DECLARATION));
    assert!(work.contains("1 + 1"));
    assert!(!work.contains("declared_work_runtime_name"));
    assert!(!work.contains("clustered(work)"));
    assert!(!work.contains("declared_work_target"));
    assert!(!work.contains("Continuity.submit_declared_work"));
    assert!(!work.contains("Continuity.mark_completed"));
    assert!(!work.contains("Timer.sleep"));
    assert!(!work.contains("owner_node"));
    assert!(!work.contains("replica_node"));

    assert!(readme.contains("Node.start_from_env()"));
    assert!(readme.contains("meshc cluster status"));
    assert!(readme.contains("meshc cluster continuity"));
    assert!(readme.contains("meshc cluster diagnostics"));
    assert!(readme.contains("MESH_CLUSTER_COOKIE"));
    assert!(readme.contains("MESH_NODE_NAME"));
    assert!(readme.contains("MESH_DISCOVERY_SEED"));
    assert!(readme.contains("MESH_CONTINUITY_ROLE"));
    assert!(readme.contains("MESH_CONTINUITY_PROMOTION_EPOCH"));
    assert!(readme.contains("`@cluster`"));
    assert!(readme.contains(route_free::STARTUP_RUNTIME_NAME_GUIDANCE));
    assert!(readme.contains(route_free::STARTUP_AUTOSTART_GUIDANCE));
    assert!(!readme.contains("declared_work_runtime_name()"));
    assert!(!readme.contains("clustered(work)"));
    assert!(!readme.contains("Continuity.submit_declared_work"));
    assert!(!readme.contains("Continuity.mark_completed"));
    assert!(!readme.contains("HTTP.serve"));
    assert!(!readme.contains("/health"));
    assert!(!readme.contains("/work"));
    assert!(!readme.contains("Timer.sleep"));
    assert!(!readme.contains("CLUSTER_PROOF_"));
}

#[test]
fn m045_s01_cluster_proof_source_contract_uses_runtime_owned_bootstrap() {
    let cluster_proof_dir = route_free::cluster_proof_fixture_root();
    let main_path = cluster_proof_dir.join("main.mpl");
    let work_path = cluster_proof_dir.join("work.mpl");
    let work_test_path = cluster_proof_dir.join("tests").join("work.test.mpl");
    let dockerfile_path = cluster_proof_dir.join("Dockerfile");
    let fly_toml_path = cluster_proof_dir.join("fly.toml");
    let readme_path = cluster_proof_dir.join("README.md");

    assert_source_contains(&main_path, "Node.start_from_env()");
    assert_source_contains(&main_path, "BootstrapStatus");
    assert_source_contains(&main_path, "[cluster-proof] runtime bootstrap");
    assert_source_omits(&main_path, "Node.start(");
    assert_source_omits(&main_path, "Continuity.submit_declared_work");
    assert_source_omits(&main_path, "HTTP.serve");
    assert_source_omits(&main_path, "/work");
    assert_source_omits(&main_path, "/membership");

    assert_source_contains(&work_path, route_free::STARTUP_SOURCE_DECLARATION);
    assert_source_contains(&work_path, "1 + 1");
    assert_source_omits(&work_path, "declared_work_runtime_name");
    assert_source_omits(&work_path, "clustered(work)");
    assert_source_omits(&work_path, "declared_work_target");
    assert_source_omits(&work_path, "Continuity.submit_declared_work");
    assert_source_omits(&work_path, "Continuity.mark_completed");
    assert_source_omits(&work_path, "Timer.sleep");
    assert_source_omits(&work_path, "CLUSTER_PROOF_WORK_DELAY_MS");

    assert_source_contains(
        &work_test_path,
        "manifest and source stay source-first and route-free",
    );
    assert_source_contains(
        &work_test_path,
        "assert_not_contains(main_source, \"/work\")",
    );
    assert_source_contains(
        &work_test_path,
        "assert_not_contains(work_source, \"Timer.sleep\")",
    );

    assert_source_contains(
        &dockerfile_path,
        "ENTRYPOINT [\"/usr/local/bin/cluster-proof\"]",
    );
    assert_source_contains(&dockerfile_path, "EXPOSE 4370");
    assert_source_omits(&dockerfile_path, "docker-entrypoint.sh");
    assert_source_omits(&dockerfile_path, "EXPOSE 8080");

    assert_source_contains(
        &fly_toml_path,
        "dockerfile = 'scripts/fixtures/clustered/cluster-proof/Dockerfile'",
    );
    assert_source_contains(&fly_toml_path, "MESH_CLUSTER_PORT = '4370'");
    assert_source_contains(
        &fly_toml_path,
        "MESH_DISCOVERY_SEED = 'mesh-cluster-proof.internal'",
    );
    assert_source_omits(&fly_toml_path, "http_service");

    assert_source_contains(&readme_path, "Node.start_from_env()");
    assert_source_contains(&readme_path, "meshc cluster status");
    assert_source_contains(&readme_path, "meshc cluster continuity");
    assert_source_contains(&readme_path, "meshc cluster diagnostics");
    assert_source_contains(&readme_path, "route-free");
    assert_source_contains(&readme_path, route_free::STARTUP_RUNTIME_NAME_GUIDANCE);
    assert_source_contains(&readme_path, route_free::STARTUP_AUTOSTART_GUIDANCE);
    assert_source_omits(&readme_path, "declared_work_runtime_name()");
    assert_source_omits(&readme_path, "clustered(work)");
    assert_source_omits(&readme_path, "/work");
    assert_source_omits(&readme_path, "/membership");
    assert_source_omits(&readme_path, "mesh-cluster-proof.fly.dev");
    assert_source_omits(&readme_path, "docker-entrypoint.sh");

    assert!(
        !cluster_proof_dir.join("config.mpl").exists(),
        "cluster-proof/config.mpl should stay deleted"
    );
    assert!(
        !cluster_proof_dir
            .join("tests")
            .join("config.test.mpl")
            .exists(),
        "cluster-proof/tests/config.test.mpl should stay deleted"
    );
    assert!(
        !cluster_proof_dir.join("docker-entrypoint.sh").exists(),
        "cluster-proof/docker-entrypoint.sh should stay deleted"
    );
    assert!(
        !cluster_proof_dir.join("cluster.mpl").exists(),
        "cluster-proof/cluster.mpl should stay deleted"
    );
    assert!(
        !cluster_proof_dir.join("work_continuity.mpl").exists(),
        "cluster-proof/work_continuity.mpl should stay deleted"
    );
}

#[test]
fn m045_s01_cluster_proof_package_contract_builds_and_tests() {
    ensure_mesh_rt_staticlib();

    let artifacts = artifact_dir("cluster-proof-package-contract");
    let cluster_proof_dir = route_free::cluster_proof_fixture_root();
    let cluster_proof_tests_dir = cluster_proof_dir.join("tests");
    let build = Command::new(meshc_bin())
        .current_dir(repo_root())
        .arg("build")
        .arg(&cluster_proof_dir)
        .output()
        .expect("failed to invoke meshc build on relocated cluster-proof fixture");
    write_artifact(&artifacts.join("build.log"), command_output_text(&build));
    assert!(
        build.status.success(),
        "meshc build {} should succeed:\n{}",
        cluster_proof_dir.display(),
        command_output_text(&build)
    );

    let tests = Command::new(meshc_bin())
        .current_dir(repo_root())
        .arg("test")
        .arg(&cluster_proof_tests_dir)
        .output()
        .expect("failed to invoke meshc test on relocated cluster-proof fixture");
    write_artifact(&artifacts.join("tests.log"), command_output_text(&tests));
    assert!(
        tests.status.success(),
        "meshc test {} should succeed:\n{}",
        cluster_proof_tests_dir.display(),
        command_output_text(&tests)
    );
}
