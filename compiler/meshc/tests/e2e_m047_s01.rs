use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::OnceLock;

use serde_json::Value;
use tempfile::TempDir;

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
    format!("[package]\nname = \"{}\"\nversion = \"1.0.0\"\n", name)
}

fn legacy_manifest(name: &str) -> String {
    format!(
        "{}\n[cluster]\nenabled = true\ndeclarations = [\n  {{ kind = \"work\", target = \"Work.handle_submit\" }},\n]\n",
        package_manifest(name)
    )
}

fn source_cluster_success_main() -> &'static str {
    "from Work import handle_submit, handle_retry, local_only\n\nfn main() do\n  let _ = handle_submit()\n  let _ = handle_retry()\n  let _ = local_only(\"demo\")\nend\n"
}

fn validation_main() -> &'static str {
    "fn main() do\n  nil\nend\n"
}

fn source_cluster_success_work() -> &'static str {
    "@cluster pub fn handle_submit() -> Int do\n  1 + 1\nend\n\n@cluster(3) pub fn handle_retry() -> Int do\n  2 + 1\nend\n\npub fn local_only(payload :: String) -> String do\n  payload\nend\n"
}

fn private_cluster_work_with_explicit_count() -> &'static str {
    "@cluster(3) fn hidden_submit(payload :: String) -> String do\n  payload\nend\n"
}

fn invalid_cluster_count_work() -> &'static str {
    "@cluster(1, 2) pub fn broken_submit(payload :: String) -> String do\n  payload\nend\n"
}

fn legacy_clustered_work_source() -> &'static str {
    "clustered(work) pub fn handle_submit(request_key :: String, attempt_id :: String) -> Int do\n  if String.length(request_key) > 0 do\n    String.length(attempt_id)\n  else\n    0\n  end\nend\n"
}

fn write_project_sources(project_dir: &Path, manifest: &str, sources: &[(&str, &str)]) {
    fs::create_dir_all(project_dir).expect("failed to create temp project dir");
    fs::write(project_dir.join("mesh.toml"), manifest).expect("failed to write temp mesh.toml");
    for (path, content) in sources {
        fs::write(project_dir.join(path), content)
            .unwrap_or_else(|err| panic!("failed to write {path}: {err}"));
    }
}

fn build_temp_project_with_sources(
    manifest: &str,
    sources: &[(&str, &str)],
    extra_args: &[&str],
) -> (TempDir, PathBuf, Output) {
    ensure_mesh_rt_staticlib();
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let project_dir = tmp.path().join("project");
    write_project_sources(&project_dir, manifest, sources);

    let mut command = Command::new(meshc_bin());
    command.current_dir(repo_root());
    command.arg("build").arg(project_dir.to_str().unwrap());
    command.args(extra_args);

    let output = command.output().expect("failed to invoke meshc build");
    (tmp, project_dir, output)
}

fn parse_json_stderr(output: &Output) -> Vec<Value> {
    String::from_utf8_lossy(&output.stderr)
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .collect()
}

#[test]
fn m047_s01_source_only_cluster_decorators_build_without_cluster_manifest_section() {
    let (_tmp, project_dir, output) = build_temp_project_with_sources(
        package_manifest("clustered-source-proof").as_str(),
        &[
            ("main.mpl", source_cluster_success_main()),
            ("work.mpl", source_cluster_success_work()),
        ],
        &["--emit-llvm"],
    );
    assert!(
        output.status.success(),
        "source-only @cluster build should succeed:\n{}",
        command_output_text(&output)
    );

    let llvm = fs::read_to_string(project_dir.join("project.ll"))
        .expect("expected llvm output for successful source-only @cluster build");
    assert!(
        llvm.contains("mesh_register_declared_handler"),
        "expected source-only build to register declared handlers:\n{llvm}"
    );
    for marker in [
        "Work.handle_submit",
        "Work.handle_retry",
        "declared_exec_reg___declared_work_work_handle_submit",
        "declared_exec_reg___declared_work_work_handle_retry",
    ] {
        assert!(
            llvm.contains(marker),
            "expected llvm output to keep stable registration naming marker {marker}:\n{llvm}"
        );
    }
}

#[test]
fn m047_s01_private_cluster_decorator_with_explicit_count_fails_before_codegen() {
    let (_tmp, project_dir, output) = build_temp_project_with_sources(
        package_manifest("clustered-source-proof").as_str(),
        &[
            ("main.mpl", validation_main()),
            ("work.mpl", private_cluster_work_with_explicit_count()),
        ],
        &["--emit-llvm"],
    );
    assert!(
        !output.status.success(),
        "private @cluster(3) function should fail validation:\n{}",
        command_output_text(&output)
    );
    assert!(
        !project_dir.join("project.ll").exists(),
        "private @cluster(3) validation failure should stop before llvm emission"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("source `@cluster` decorator")
            && stderr.contains("Work.hidden_submit")
            && stderr.contains("private function")
            && stderr.contains("replication count 3")
            && stderr.contains("work.mpl:1:1"),
        "expected explicit private @cluster(3) diagnostic with source range, got:\n{stderr}"
    );
}

#[test]
fn m047_s04_legacy_manifest_cluster_section_fails_with_migration_guidance() {
    let (_tmp, project_dir, output) = build_temp_project_with_sources(
        legacy_manifest("clustered-source-proof").as_str(),
        &[
            ("main.mpl", validation_main()),
            ("work.mpl", source_cluster_success_work()),
        ],
        &["--emit-llvm"],
    );
    assert!(
        !output.status.success(),
        "legacy [cluster] manifest should fail closed:\n{}",
        command_output_text(&output)
    );
    assert!(
        !project_dir.join("project.ll").exists(),
        "legacy manifest rejection should stop before llvm emission"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("`[cluster]` manifest sections are no longer supported")
            && stderr.contains("`@cluster`")
            && stderr.contains("`@cluster(N)`")
            && stderr.contains("mesh.toml"),
        "expected explicit legacy manifest migration guidance, got:\n{stderr}"
    );
}

#[test]
fn m047_s04_legacy_clustered_work_json_diagnostic_uses_source_file_and_span() {
    let (_tmp, project_dir, output) = build_temp_project_with_sources(
        package_manifest("clustered-source-proof").as_str(),
        &[
            ("main.mpl", validation_main()),
            ("work.mpl", legacy_clustered_work_source()),
        ],
        &["--json", "--emit-llvm"],
    );
    assert!(
        !output.status.success(),
        "legacy clustered(work) source should fail closed:\n{}",
        command_output_text(&output)
    );
    assert!(
        !project_dir.join("project.ll").exists(),
        "legacy clustered(work) parse failure should stop before llvm emission"
    );

    let diagnostics = parse_json_stderr(&output);
    let diag = diagnostics
        .iter()
        .find(|diag| {
            diag["message"]
                .as_str()
                .map(|message| {
                    message
                        .contains("legacy `clustered(work)` declarations are no longer supported")
                })
                .unwrap_or(false)
        })
        .unwrap_or_else(|| {
            panic!(
                "expected legacy clustered(work) JSON diagnostic in stderr, got:\n{}",
                String::from_utf8_lossy(&output.stderr)
            )
        });

    let file = diag["file"]
        .as_str()
        .expect("legacy clustered(work) JSON diagnostic should include a file path");
    assert!(
        file.ends_with("/work.mpl") || file.ends_with("\\work.mpl"),
        "expected legacy clustered(work) diagnostic to point at work.mpl, got {file}"
    );
    let spans = diag["spans"]
        .as_array()
        .expect("legacy clustered(work) JSON diagnostic should include spans");
    assert_eq!(
        spans.len(),
        1,
        "expected one source span on legacy diagnostic"
    );
    let start = spans[0]["start"]
        .as_u64()
        .expect("span start should be numeric") as usize;
    let end = spans[0]["end"]
        .as_u64()
        .expect("span end should be numeric") as usize;
    assert!(
        end > start,
        "expected non-empty legacy source span, got {start}..{end}"
    );
}

#[test]
fn m047_s01_invalid_cluster_decorator_count_fails_before_codegen() {
    let (_tmp, project_dir, output) = build_temp_project_with_sources(
        package_manifest("clustered-source-proof").as_str(),
        &[
            ("main.mpl", validation_main()),
            ("work.mpl", invalid_cluster_count_work()),
        ],
        &["--emit-llvm"],
    );
    assert!(
        !output.status.success(),
        "malformed @cluster count should fail before codegen:\n{}",
        command_output_text(&output)
    );
    assert!(
        !project_dir.join("project.ll").exists(),
        "malformed @cluster count should stop before llvm emission"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Parse error"),
        "expected malformed decorator count to surface as a parse error, got:\n{stderr}"
    );
}
