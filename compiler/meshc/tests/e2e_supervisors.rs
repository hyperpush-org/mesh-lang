//! End-to-end integration tests for the Mesh supervisor compiler pipeline.
//!
//! Each test compiles a .mpl program that exercises supervisor features,
//! builds it into a native binary, and verifies expected behavior.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Helper: compile a Mesh source file and assert it compiles successfully.
/// Returns the path to the compiled binary.
fn compile_mesh(source: &str) -> (tempfile::TempDir, PathBuf) {
    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
    let project_dir = temp_dir.path().join("project");
    std::fs::create_dir_all(&project_dir).expect("failed to create project dir");

    let main_mesh = project_dir.join("main.mpl");
    std::fs::write(&main_mesh, source).expect("failed to write main.mpl");

    // Build with meshc
    let meshc = find_meshc();
    let output = Command::new(&meshc)
        .args(["build", project_dir.to_str().unwrap()])
        .output()
        .expect("failed to invoke meshc");

    assert!(
        output.status.success(),
        "meshc build failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let binary = project_dir.join("project");
    assert!(
        binary.exists(),
        "compiled binary not found at {}",
        binary.display()
    );

    (temp_dir, binary)
}

/// Read a test fixture from the tests/e2e/ directory.
fn read_fixture(name: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let fixture_path = Path::new(manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("tests")
        .join("e2e")
        .join(name);
    std::fs::read_to_string(&fixture_path)
        .unwrap_or_else(|e| panic!("failed to read fixture {}: {}", fixture_path.display(), e))
}

/// Find the meshc binary in the target directory.
fn find_meshc() -> PathBuf {
    let mut path = std::env::current_exe()
        .expect("cannot find current exe")
        .parent()
        .expect("cannot find parent dir")
        .to_path_buf();

    if path.file_name().map_or(false, |n| n == "deps") {
        path = path.parent().unwrap().to_path_buf();
    }

    let meshc = path.join("meshc");
    assert!(
        meshc.exists(),
        "meshc binary not found at {}. Run `cargo build -p meshc` first.",
        meshc.display()
    );
    meshc
}

/// Helper: compile a Mesh source file without asserting success.
/// Returns the raw compilation output (for negative tests).
fn compile_only(source: &str) -> Output {
    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
    let project_dir = temp_dir.path().join("project");
    std::fs::create_dir_all(&project_dir).expect("failed to create project dir");

    let main_mesh = project_dir.join("main.mpl");
    std::fs::write(&main_mesh, source).expect("failed to write main.mpl");

    let meshc = find_meshc();
    Command::new(&meshc)
        .args(["build", project_dir.to_str().unwrap()])
        .output()
        .expect("failed to invoke meshc")
}

/// Helper: run a compiled binary with a timeout and return stdout.
fn run_with_timeout(binary: &Path, timeout_secs: u64) -> String {
    let mut child = Command::new(binary)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed to spawn binary: {}", e));

    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(timeout_secs);
    let poll_interval = std::time::Duration::from_millis(50);

    let output = loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let mut stdout = Vec::new();
                let mut stderr = Vec::new();
                if let Some(mut out) = child.stdout.take() {
                    use std::io::Read;
                    out.read_to_end(&mut stdout).ok();
                }
                if let Some(mut err) = child.stderr.take() {
                    use std::io::Read;
                    err.read_to_end(&mut stderr).ok();
                }
                break Output {
                    status,
                    stdout,
                    stderr,
                };
            }
            Ok(None) => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    panic!("binary timed out after {} seconds", timeout_secs);
                }
                std::thread::sleep(poll_interval);
            }
            Err(e) => panic!("error waiting for process: {}", e),
        }
    };

    String::from_utf8_lossy(&output.stdout).to_string()
}

// ── Supervisor E2E Tests ─────────────────────────────────────────────────

/// Test: Supervisor block compiles to a native binary successfully.
///
/// This validates the full compiler pipeline: parsing supervisor blocks,
/// type checking, MIR lowering, and LLVM codegen with supervisor intrinsics.
#[test]
fn supervisor_basic() {
    let source = read_fixture("supervisor_basic.mpl");
    let (_temp_dir, binary) = compile_mesh(&source);

    // Verify the binary exists and is executable.
    assert!(binary.exists(), "compiled supervisor binary should exist");

    // Run the binary with a short timeout -- it should print and exit.
    let mut child = Command::new(&binary)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed to spawn binary: {}", e));

    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(10);
    let poll_interval = std::time::Duration::from_millis(50);

    let output = loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let mut stdout = Vec::new();
                let mut stderr = Vec::new();
                if let Some(mut out) = child.stdout.take() {
                    use std::io::Read;
                    out.read_to_end(&mut stdout).ok();
                }
                if let Some(mut err) = child.stderr.take() {
                    use std::io::Read;
                    err.read_to_end(&mut stderr).ok();
                }
                break std::process::Output {
                    status,
                    stdout,
                    stderr,
                };
            }
            Ok(None) => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    panic!(
                        "supervisor binary timed out after {} seconds",
                        timeout.as_secs()
                    );
                }
                std::thread::sleep(poll_interval);
            }
            Err(e) => panic!("error waiting for process: {}", e),
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("supervisor started"),
        "Expected 'supervisor started' in output, got: {}",
        stdout
    );
}

/// Test: one_for_all supervisor compiles and runs.
///
/// Success Criterion 2: one_for_all strategy is recognized by compiler
/// and runtime. The supervisor starts and prints expected output.
#[test]
fn supervisor_one_for_all() {
    let source = read_fixture("supervisor_one_for_all.mpl");
    let (_temp_dir, binary) = compile_mesh(&source);

    let stdout = run_with_timeout(&binary, 10);
    assert!(
        stdout.contains("one_for_all supervisor started"),
        "Expected 'one_for_all supervisor started' in output, got: {}",
        stdout
    );
}

/// Test: supervisor with restart limits compiles and runs.
///
/// Success Criterion 3: restart limits are part of the compiled supervisor
/// configuration. The supervisor starts and manages restart counting.
#[test]
fn supervisor_restart_limit() {
    let source = read_fixture("supervisor_restart_limit.mpl");
    let (_temp_dir, binary) = compile_mesh(&source);

    let stdout = run_with_timeout(&binary, 10);
    assert!(
        stdout.contains("restart limit test started"),
        "Expected 'restart limit test started' in output, got: {}",
        stdout
    );
}

/// Test: supervisor with invalid child spec is rejected at compile time.
///
/// Success Criterion 4: typed supervision validates at compile time.
/// A child spec whose start function returns Int (not Pid) produces
/// error E0018 (InvalidChildStart).
#[test]
fn supervisor_typed_error_rejected() {
    let source = read_fixture("supervisor_typed_error.mpl");
    let result = compile_only(&source);

    assert!(
        !result.status.success(),
        "Should fail to compile: start fn returns Int, not Pid"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    let stdout = String::from_utf8_lossy(&result.stdout);
    let all_output = format!("{}{}", stdout, stderr);

    assert!(
        all_output.contains("E0018")
            || all_output.contains("must return Pid")
            || all_output.contains("InvalidChildStart"),
        "Should report child start type error (E0018).\nstdout: {}\nstderr: {}",
        stdout,
        stderr
    );
}
