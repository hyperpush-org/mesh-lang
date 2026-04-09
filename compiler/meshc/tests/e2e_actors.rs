//! End-to-end integration tests for the Mesh actor runtime.
//!
//! Each test compiles a .mpl program that exercises actor features,
//! runs the resulting binary, and asserts the expected stdout output.
//!
//! Actor tests use generous timeouts since they involve concurrency.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// Helper: compile a Mesh source and run the binary with a timeout.
/// Returns stdout on success. Panics on compilation failure or timeout.
fn compile_and_run_with_timeout(source: &str, timeout_secs: u64) -> String {
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

    // Run the compiled binary with a timeout
    let binary = project_dir.join("project");
    let child = Command::new(&binary)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed to spawn binary at {}: {}", binary.display(), e));

    let output = wait_with_timeout(child, Duration::from_secs(timeout_secs));

    match output {
        Ok(out) => {
            assert!(
                out.status.success(),
                "binary execution failed with exit code {:?}:\nstdout: {}\nstderr: {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            );
            String::from_utf8_lossy(&out.stdout).to_string()
        }
        Err(msg) => panic!("{}", msg),
    }
}

/// Helper: compile a Mesh source and expect compilation to fail.
/// Returns the stderr output.
fn compile_expect_error(source: &str) -> String {
    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
    let project_dir = temp_dir.path().join("project");
    std::fs::create_dir_all(&project_dir).expect("failed to create project dir");

    let main_mesh = project_dir.join("main.mpl");
    std::fs::write(&main_mesh, source).expect("failed to write main.mpl");

    let meshc = find_meshc();
    let output = Command::new(&meshc)
        .args(["build", project_dir.to_str().unwrap()])
        .output()
        .expect("failed to invoke meshc");

    assert!(
        !output.status.success(),
        "expected compilation to fail but it succeeded"
    );

    String::from_utf8_lossy(&output.stderr).to_string()
}

/// Wait for a child process with a timeout. Kill it if it exceeds the timeout.
fn wait_with_timeout(
    mut child: std::process::Child,
    timeout: Duration,
) -> Result<std::process::Output, String> {
    let start = std::time::Instant::now();
    let poll_interval = Duration::from_millis(50);

    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                // Process exited. Collect stdout/stderr.
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
                return Ok(std::process::Output {
                    status,
                    stdout,
                    stderr,
                });
            }
            Ok(None) => {
                // Still running
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(format!(
                        "Binary timed out after {} seconds",
                        timeout.as_secs()
                    ));
                }
                std::thread::sleep(poll_interval);
            }
            Err(e) => return Err(format!("Error waiting for process: {}", e)),
        }
    }
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

// ── Actor E2E Tests ─────────────────────────────────────────────────────

/// Test 1: Basic actor spawning and messaging.
/// An actor receives a message and prints a response.
#[test]
fn actors_basic() {
    let source = read_fixture("actors_basic.mpl");
    let output = compile_and_run_with_timeout(&source, 10);
    assert!(
        output.contains("actor received"),
        "Expected 'actor received' in output, got: {}",
        output
    );
    assert!(
        output.contains("main done"),
        "Expected 'main done' in output, got: {}",
        output
    );
}

/// Test 2: Receive with message processing.
/// Multiple actors receive messages and process them.
#[test]
fn actors_messaging() {
    let source = read_fixture("actors_messaging.mpl");
    let output = compile_and_run_with_timeout(&source, 10);
    // All three workers should print their done message
    let count = output.matches("worker done").count();
    assert!(
        count >= 3,
        "Expected at least 3 'worker done' messages, got {} in: {}",
        count,
        output
    );
}

/// Test 3: Preemptive scheduling -- a tight-loop actor does not starve others.
/// One actor does a lot of work while another waits for a message.
/// Both should complete.
#[test]
fn actors_preemption() {
    let source = read_fixture("actors_preemption.mpl");
    let output = compile_and_run_with_timeout(&source, 10);
    assert!(
        output.contains("fast done"),
        "Expected 'fast done' in output (fast actor was not starved), got: {}",
        output
    );
    assert!(
        output.contains("slow done"),
        "Expected 'slow done' in output (slow actor completed), got: {}",
        output
    );
}

/// Test 4: Process linking -- when one actor exits, linked actor is notified.
/// This tests the exit signal propagation through link().
#[test]
fn actors_linking() {
    let source = read_fixture("actors_linking.mpl");
    let output = compile_and_run_with_timeout(&source, 10);
    assert!(
        output.contains("link test done"),
        "Expected 'link test done' in output, got: {}",
        output
    );
}

/// Test 5: Typed Pid prevents wrong-type sends at compile time.
/// A program that tries to send the wrong type to a typed Pid should fail.
#[test]
fn actors_typed_pid() {
    let source = read_fixture("actors_typed_pid.mpl");
    let output = compile_and_run_with_timeout(&source, 10);
    assert!(
        output.contains("typed pid ok"),
        "Expected 'typed pid ok' in output, got: {}",
        output
    );
}

/// Test 6: 100K actor benchmark -- spawn 100K actors and verify they all respond.
#[test]
fn actors_100k() {
    let source = read_fixture("actors_100k.mpl");
    let output = compile_and_run_with_timeout(&source, 30);
    assert!(
        output.contains("100000 actors done"),
        "Expected '100000 actors done' in output, got: {}",
        output
    );
}

/// Test 7: Terminate callback -- cleanup logic runs before actor exit.
#[test]
fn actors_terminate() {
    let source = read_fixture("actors_terminate.mpl");
    let output = compile_and_run_with_timeout(&source, 10);
    assert!(
        output.contains("terminate test done"),
        "Expected 'terminate test done' in output, got: {}",
        output
    );
}

/// Test 8: GC bounded memory -- a long-running actor allocates and discards
/// strings in a tight loop. With mark-sweep GC, unreachable allocations are
/// reclaimed at yield points so memory stays bounded.
#[test]
fn gc_bounded_memory() {
    let source = read_fixture("gc_bounded_memory.mpl");
    let output = compile_and_run_with_timeout(&source, 30);
    assert!(
        output.contains("gc bounded memory test done"),
        "Expected 'gc bounded memory test done' in output, got: {}",
        output
    );
}

/// Test 9: Actors with arguments -- spawn passes initial state correctly.
/// Actors receive typed arguments, not raw buffer pointers. Tests the
/// wrapper function that deserializes args from the spawn buffer.
#[test]
fn actors_with_args() {
    let source = read_fixture("actors_with_args.mpl");
    let output = compile_and_run_with_timeout(&source, 10);
    assert!(
        output.contains("15"),
        "Expected '15' (adder initial=10 + msg=5) in output, got: {}",
        output
    );
    assert!(
        output.contains("42"),
        "Expected '42' (calculator 30 + 12) in output, got: {}",
        output
    );
    assert!(
        output.contains("args test done"),
        "Expected 'args test done' in output, got: {}",
        output
    );
}
