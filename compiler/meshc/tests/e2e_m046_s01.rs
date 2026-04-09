use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::OnceLock;

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

fn source_declared_duplicate_manifest(name: &str) -> String {
    format!(
        "{}\n[cluster]\nenabled = true\ndeclarations = [\n  {{ kind = \"work\", target = \"Work.handle_submit\" }},\n]\n",
        package_manifest(name)
    )
}

fn source_declared_success_project_main_source() -> &'static str {
    "from Work import handle_submit, local_only\n\nfn main() do\n  let _ = handle_submit(\"req-1\", \"attempt-1\")\n  let _ = local_only(\"demo\")\nend\n"
}

fn source_declared_validation_project_main_source() -> &'static str {
    "fn main() do\n  nil\nend\n"
}

fn source_declared_public_work_source() -> &'static str {
    "clustered(work) pub fn handle_submit(request_key :: String, attempt_id :: String) -> Int do\n  if String.length(request_key) > 0 do\n    String.length(attempt_id)\n  else\n    0\n  end\nend\n\npub fn local_only(payload :: String) -> String do\n  payload\nend\n\nfn hidden_submit(payload :: String) -> String do\n  payload\nend\n"
}

fn source_declared_private_work_source() -> &'static str {
    "clustered(work) fn hidden_submit(payload :: String) -> String do\n  payload\nend\n"
}

fn write_project_sources(project_dir: &Path, sources: &[(&str, &str)]) {
    fs::create_dir_all(project_dir).expect("failed to create temp project dir");
    for (path, content) in sources {
        fs::write(project_dir.join(path), content)
            .unwrap_or_else(|err| panic!("failed to write {path}: {err}"));
    }
}

fn build_temp_project_with_sources(
    sources: &[(&str, &str)],
    emit_llvm: bool,
) -> (Output, Option<String>) {
    ensure_mesh_rt_staticlib();
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let project_dir = tmp.path().join("project");
    write_project_sources(&project_dir, sources);

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

#[test]
fn m046_s01_source_declared_work_llvm_registers_decorated_handler() {
    let manifest = package_manifest("clustered-source-proof");
    let (output, llvm) = build_temp_project_with_sources(
        &[
            ("mesh.toml", manifest.as_str()),
            ("main.mpl", source_declared_success_project_main_source()),
            ("work.mpl", source_declared_public_work_source()),
        ],
        true,
    );
    assert!(
        output.status.success(),
        "source-declared work build should succeed:\n{}",
        command_output_text(&output)
    );
    let llvm = llvm.expect("source-declared work build should emit llvm");

    assert!(
        llvm.contains("mesh_register_declared_handler"),
        "expected source-declared work to reach declared-handler registration:\n{llvm}"
    );
    assert!(
        llvm.contains("Work.handle_submit")
            && llvm.contains("declared_exec_reg___declared_work_work_handle_submit"),
        "expected source-declared work runtime name and wrapper registration:\n{llvm}"
    );
}

#[test]
fn m046_s01_source_declared_private_work_fails_before_codegen() {
    let manifest = package_manifest("clustered-source-proof");
    let (output, llvm) = build_temp_project_with_sources(
        &[
            ("mesh.toml", manifest.as_str()),
            ("main.mpl", source_declared_validation_project_main_source()),
            ("work.mpl", source_declared_private_work_source()),
        ],
        true,
    );
    assert!(
        !output.status.success(),
        "private source-declared work should fail validation:\n{}",
        command_output_text(&output)
    );
    assert!(
        llvm.is_none(),
        "validation failure should stop before llvm output"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("source `clustered(work)` marker")
            && stderr.contains("Work.hidden_submit")
            && stderr.contains("private function"),
        "expected explicit private source declaration diagnostic, got:\n{stderr}"
    );
}

#[test]
fn m046_s01_source_and_manifest_duplicate_fails_with_explicit_diagnostic() {
    let manifest = source_declared_duplicate_manifest("clustered-source-proof");
    let (output, llvm) = build_temp_project_with_sources(
        &[
            ("mesh.toml", manifest.as_str()),
            ("main.mpl", source_declared_validation_project_main_source()),
            ("work.mpl", source_declared_public_work_source()),
        ],
        true,
    );
    assert!(
        !output.status.success(),
        "duplicate source/manifest declarations should fail validation:\n{}",
        command_output_text(&output)
    );
    assert!(
        llvm.is_none(),
        "duplicate validation failure should stop before llvm output"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("mesh.toml")
            && stderr.contains("source `clustered(work)` marker")
            && stderr.contains("Work.handle_submit"),
        "expected explicit duplicate diagnostic, got:\n{stderr}"
    );
}
