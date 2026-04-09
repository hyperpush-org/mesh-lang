use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

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

fn clustered_project_main_source() -> &'static str {
    "from Services import Jobs\nfrom Work import handle_submit\n\nfn main() do\n  let pid = Jobs.start(0)\n  let _ = Jobs.submit(pid, \"demo\")\n  let _ = handle_submit(\"req-1\", \"attempt-1\")\nend\n"
}

fn clustered_project_services_source() -> &'static str {
    "service Jobs do\n  fn init(start :: Int) -> Int do\n    start\n  end\n\n  call Submit(payload :: String) :: String do |state|\n    (state, payload)\n  end\n\n  cast Reset() do |_state|\n    0\n  end\nend\n"
}

fn clustered_project_work_source() -> &'static str {
    "pub fn handle_submit(request_key :: String, attempt_id :: String) -> String do\n  if String.length(request_key) > 0 do\n    attempt_id\n  else\n    request_key\n  end\nend\n\nfn hidden_submit(payload :: String) -> String do\n  payload\nend\n"
}

fn write_project(project_dir: &Path, manifest: Option<&str>) {
    fs::create_dir_all(project_dir).expect("failed to create temp project dir");
    if let Some(manifest) = manifest {
        fs::write(project_dir.join("mesh.toml"), manifest).expect("failed to write mesh.toml");
    }
    fs::write(
        project_dir.join("main.mpl"),
        clustered_project_main_source(),
    )
    .expect("failed to write main.mpl");
    fs::write(
        project_dir.join("services.mpl"),
        clustered_project_services_source(),
    )
    .expect("failed to write services.mpl");
    fs::write(
        project_dir.join("work.mpl"),
        clustered_project_work_source(),
    )
    .expect("failed to write work.mpl");
}

fn build_temp_project(manifest: Option<&str>) -> Output {
    ensure_mesh_rt_staticlib();
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let project_dir = tmp.path().join("project");
    write_project(&project_dir, manifest);

    Command::new(meshc_bin())
        .current_dir(repo_root())
        .args(["build", project_dir.to_str().unwrap()])
        .output()
        .expect("failed to invoke meshc build")
}

fn run_repo_project(args: &[&str]) -> Output {
    ensure_mesh_rt_staticlib();
    Command::new(meshc_bin())
        .current_dir(repo_root())
        .args(args)
        .output()
        .expect("failed to invoke repo-root meshc command")
}

fn artifact_dir(test_name: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = repo_root()
        .join(".tmp")
        .join("m044-s01")
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

fn build_and_run_mesh(source: &str, envs: &[(&str, &str)], artifacts: &Path) -> Output {
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

const AUTHORITY_RUNTIME_SOURCE: &str = r##"
fn print_authority(label :: String, value :: ContinuityAuthorityStatus) do
  println("#{label}|ok|#{value.cluster_role}|#{value.promotion_epoch}|#{value.replication_health}")
end

fn print_authority_status(label :: String) do
  case Continuity.authority_status() do
    Ok( value) -> print_authority(label, value)
    Err( reason) -> println("#{label}|err|#{reason}")
  end
end

fn seed_attempt_id() -> String ! String do
  case Continuity.submit(
    "req-1",
    "hash-1",
    "standby@node",
    "primary@node",
    "replica@node",
    0,
    false,
    true
  ) do
    Ok( decision) -> Ok(decision.record.attempt_id)
    Err( reason) -> Err(reason)
  end
end

fn acknowledge_replica(attempt_id :: String) do
  case Continuity.acknowledge_replica("req-1", attempt_id) do
    Ok( record) -> println("ack|ok|#{record.replica_status}|#{record.replication_health}")
    Err( reason) -> println("ack|err|#{reason}")
  end
end

fn main() do
  print_authority_status("before")
  case seed_attempt_id() do
    Ok( attempt_id) -> do
      acknowledge_replica(attempt_id)
      print_authority_status("after_ack")
    end
    Err( reason) -> println("submit|err|#{reason}")
  end
end
"##;

const PRIMARY_AUTHORITY_RUNTIME_SOURCE: &str = r##"
fn main() do
  case Continuity.authority_status() do
    Ok( value) -> println("status|ok|#{value.cluster_role}|#{value.promotion_epoch}|#{value.replication_health}")
    Err( reason) -> println("status|err|#{reason}")
  end
end
"##;

const MANUAL_PROMOTION_DISABLED_SOURCE: &str = r##"
fn main() do
  case Continuity.promote() do
    Ok( value) -> println("#{value.cluster_role}")
    Err( reason) -> println(reason)
  end
end
"##;

#[test]
fn m044_s01_manifest_absent_build_still_succeeds() {
    let output = build_temp_project(None);
    assert!(
        output.status.success(),
        "manifestless build should still succeed:\n{}",
        command_output_text(&output)
    );
}

#[test]
fn m044_s01_manifest_present_without_cluster_section_still_succeeds() {
    let output = build_temp_project(Some(
        r#"
[package]
name = "clustered-proof"
version = "1.0.0"
"#,
    ));

    assert!(
        output.status.success(),
        "manifest-present local build should still succeed:\n{}",
        command_output_text(&output)
    );
}

#[test]
fn m044_s01_manifest_clustered_valid_declarations_build_succeeds() {
    let output = build_temp_project(Some(
        r#"
[package]
name = "clustered-proof"
version = "1.0.0"

[cluster]
enabled = true
declarations = [
  { kind = "service_call", target = "Services.Jobs.submit" },
  { kind = "service_cast", target = "Services.Jobs.reset" },
  { kind = "work", target = "Work.handle_submit" },
]
"#,
    ));

    assert!(
        output.status.success(),
        "valid clustered declarations should build:\n{}",
        command_output_text(&output)
    );
}

#[test]
fn m044_s01_manifest_private_work_target_fails_before_codegen() {
    let output = build_temp_project(Some(
        r#"
[package]
name = "clustered-proof"
version = "1.0.0"

[cluster]
enabled = true
declarations = [
  { kind = "work", target = "Work.hidden_submit" },
]
"#,
    ));

    let combined = command_output_text(&output);
    assert!(
        !output.status.success(),
        "private work target should fail:\n{}",
        combined
    );
    assert!(
        combined.contains("Work.hidden_submit") && combined.contains("private function"),
        "expected private work target diagnostic:\n{}",
        combined
    );
}

#[test]
fn m044_s01_manifest_service_kind_mismatch_fails_before_codegen() {
    let output = build_temp_project(Some(
        r#"
[package]
name = "clustered-proof"
version = "1.0.0"

[cluster]
enabled = true
declarations = [
  { kind = "service_call", target = "Services.Jobs.reset" },
]
"#,
    ));

    let combined = command_output_text(&output);
    assert!(
        !output.status.success(),
        "service kind mismatch should fail:\n{}",
        combined
    );
    assert!(
        combined.contains("Services.Jobs.reset") && combined.contains("service cast handler"),
        "expected service kind mismatch diagnostic:\n{}",
        combined
    );
}

#[test]
fn m044_s01_manifest_service_target_bad_shape_fails_before_codegen() {
    let output = build_temp_project(Some(
        r#"
[package]
name = "clustered-proof"
version = "1.0.0"

[cluster]
enabled = true
declarations = [
  { kind = "service_call", target = "Jobs.submit" },
]
"#,
    ));

    let combined = command_output_text(&output);
    assert!(
        !output.status.success(),
        "bad service target shape should fail:\n{}",
        combined
    );
    assert!(
        combined.contains("<ModulePath>.<Service>.<method>"),
        "expected bad target shape diagnostic:\n{}",
        combined
    );
}

#[test]
fn m044_s01_typed_continuity_authority_status_round_trip_runtime_truth() {
    let artifacts = artifact_dir("continuity-api-authority-runtime");
    let output = build_and_run_mesh(
        AUTHORITY_RUNTIME_SOURCE,
        &[
            ("MESH_CONTINUITY_ROLE", "standby"),
            ("MESH_CONTINUITY_PROMOTION_EPOCH", "0"),
        ],
        &artifacts,
    );

    assert!(
        output.status.success(),
        "mesh program failed:\n{}\nartifacts: {}",
        command_output_text(&output),
        artifacts.display()
    );

    let lines = stdout_lines(&output);
    assert_eq!(
        lines,
        vec![
            "before|ok|standby|0|local_only",
            "ack|ok|mirrored|healthy",
            "after_ack|ok|standby|0|healthy",
        ],
        "unexpected runtime authority output; artifacts: {}",
        artifacts.display()
    );
}

#[test]
fn m044_s01_typed_continuity_primary_authority_status_preserves_runtime_truth() {
    let artifacts = artifact_dir("continuity-api-primary-authority-status");
    let output = build_and_run_mesh(PRIMARY_AUTHORITY_RUNTIME_SOURCE, &[], &artifacts);

    assert!(
        output.status.success(),
        "mesh program failed:\n{}\nartifacts: {}",
        command_output_text(&output),
        artifacts.display()
    );

    let lines = stdout_lines(&output);
    assert_eq!(
        lines,
        vec!["status|ok|primary|0|local_only"],
        "unexpected primary authority output; artifacts: {}",
        artifacts.display()
    );
}

#[test]
fn m044_s01_typed_continuity_manual_promotion_surface_is_disabled() {
    let artifacts = artifact_dir("continuity-api-manual-promotion-disabled");
    let output = build_only_mesh(MANUAL_PROMOTION_DISABLED_SOURCE, &artifacts);

    assert!(
        !output.status.success(),
        "manual continuity promotion should fail compilation; artifacts: {}",
        artifacts.display()
    );

    let combined = command_output_text(&output);
    assert!(
        combined.contains("Continuity.promote()")
            || combined.contains("automatic-only")
            || combined.contains("authority_status"),
        "compile failure should explain that manual promotion is disabled:\n{}\nartifacts: {}",
        combined,
        artifacts.display()
    );
}

#[test]
fn m044_s01_continuity_compile_fail_promote_wrong_arity() {
    let artifacts = artifact_dir("continuity-api-promote-wrong-arity");
    let output = build_only_mesh(
        r#"
fn main() do
  let _ = Continuity.promote("extra")
  println("unreachable")
end
"#,
        &artifacts,
    );

    assert!(
        !output.status.success(),
        "wrong-arity promote call should fail compilation; artifacts: {}",
        artifacts.display()
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = command_output_text(&output);
    assert!(
        stderr.contains("promote")
            || combined.contains("Continuity.promote()")
            || combined.contains("automatic-only")
            || combined.contains("argument"),
        "compile failure should mention that manual promotion is disabled:\n{}\nartifacts: {}",
        combined,
        artifacts.display()
    );
}

#[test]
fn m044_s01_continuity_compile_fail_authority_status_wrong_result_shape() {
    let artifacts = artifact_dir("continuity-api-authority-status-wrong-shape");
    let output = build_only_mesh(
        r##"
fn main() do
  let impossible = Continuity.authority_status() + 1
  println("#{impossible}")
end
"##,
        &artifacts,
    );

    assert!(
        !output.status.success(),
        "wrong-result-shape authority_status call should fail compilation; artifacts: {}",
        artifacts.display()
    );

    let combined = command_output_text(&output);
    assert!(
        combined.contains("authority_status")
            || combined.contains("type")
            || combined.contains("Result"),
        "compile failure should mention the bad authority_status result usage:\n{}\nartifacts: {}",
        combined,
        artifacts.display()
    );
}

#[test]
fn m044_s01_cluster_proof_manifest_declares_clustered_boundary() {
    let manifest = fs::read_to_string(repo_root().join("cluster-proof/mesh.toml"))
        .expect("failed to read cluster-proof/mesh.toml");
    assert!(
        manifest.contains("[cluster]"),
        "manifest should declare [cluster]:\n{manifest}"
    );
    assert!(
        manifest.contains("enabled = true"),
        "manifest should opt into clustered mode:\n{manifest}"
    );
    for target in ["Work.execute_declared_work"] {
        assert!(
            manifest.contains(target),
            "manifest should declare clustered boundary target `{target}`:\n{manifest}"
        );
    }
    assert!(
        !manifest.contains("handle_promote"),
        "cluster manifest should not declare removed manual promotion routes:\n{manifest}"
    );
}

#[test]
fn m044_s01_cluster_proof_build_succeeds_with_cluster_manifest() {
    let output = run_repo_project(&["build", "cluster-proof"]);
    assert!(
        output.status.success(),
        "cluster-proof build should succeed with its clustered manifest:\n{}",
        command_output_text(&output)
    );
}

#[test]
fn m044_s01_cluster_proof_package_tests_pass_on_typed_continuity_surface() {
    let output = run_repo_project(&["test", "cluster-proof/tests"]);
    assert!(
        output.status.success(),
        "cluster-proof package tests should pass on the typed continuity surface:\n{}",
        command_output_text(&output)
    );
}

#[test]
fn m044_s01_cluster_proof_work_continuity_omits_stringly_runtime_shims() {
    let source = fs::read_to_string(repo_root().join("cluster-proof/work_continuity.mpl"))
        .expect("failed to read cluster-proof/work_continuity.mpl");
    for needle in [
        "ContinuityAuthorityStatus.from_json",
        "ContinuitySubmitDecision.from_json",
        "WorkRequestRecord.from_json",
        "parse_authority_status_json",
        "parse_continuity_submit_response",
        "parse_continuity_record",
    ] {
        assert!(
            !source.contains(needle),
            "cluster-proof/work_continuity.mpl should not contain `{needle}`:\n{source}"
        );
    }
}
