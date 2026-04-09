mod support;

use serde_json::json;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant};
use support::m046_route_free as route_free;

const RUN_TIMEOUT: Duration = Duration::from_secs(10);
const SQLITE_BUILT_PACKAGE_SOURCE: &str = r#"
fn ensure_schema(db_path :: String) -> Int!String do
  let db = Sqlite.open(db_path)?
  let applied = Sqlite.execute(
    db,
    "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, age TEXT NOT NULL)",
    []
  )?
  Sqlite.close(db)
  Ok(applied)
end

fn insert_user(db_path :: String, name :: String, age :: String) -> Int!String do
  let db = Sqlite.open(db_path)?
  let inserted = Sqlite.execute(db, "INSERT INTO users (name, age) VALUES (?, ?)", [name, age])?
  Sqlite.close(db)
  Ok(inserted)
end

fn read_back_count(db_path :: String, age :: String) -> Int!String do
  let db = Sqlite.open(db_path)?
  let rows = Sqlite.query(db, "SELECT name FROM users WHERE age = ?", [age])?
  Sqlite.close(db)
  Ok(List.length(rows))
end

fn mismatch_message(db_path :: String) -> String!String do
  let db = Sqlite.open(db_path)?
  let result = Sqlite.execute(db, "INSERT INTO users (name, age) VALUES ('literal', '99')", ["extra"])
  Sqlite.close(db)
  Ok(case result do
    Ok(v) -> "unexpected-ok:" <> Int.to_string(v)
    Err(msg) -> msg
  end)
end

fn main() do
  let db_path = "test.sqlite3"

  case ensure_schema(db_path) do
    Ok(_) -> println("schema=ok")
    Err(msg) -> println("schema=err:" <> msg)
  end

  case insert_user(db_path, "Alice", "30") do
    Ok(inserted) -> println("insert=" <> Int.to_string(inserted))
    Err(msg) -> println("insert=err:" <> msg)
  end

  case read_back_count(db_path, "30") do
    Ok(count) -> println("count=" <> Int.to_string(count))
    Err(msg) -> println("count=err:" <> msg)
  end

  case mismatch_message(db_path) do
    Ok(msg) -> println("mismatch_err=" <> msg)
    Err(msg) -> println("mismatch_err=" <> msg)
  end
  println("done")
end
"#;

fn artifact_dir(test_name: &str) -> PathBuf {
    route_free::artifact_dir("m047-s06", test_name)
}

fn write_package_fixture(artifacts: &Path) -> PathBuf {
    let project_dir = artifacts.join("package");
    fs::create_dir_all(&project_dir)
        .unwrap_or_else(|error| panic!("failed to create {}: {error}", project_dir.display()));

    let manifest = "[package]\nname = \"sqlite-built-package\"\nversion = \"0.1.0\"\n";
    fs::write(project_dir.join("mesh.toml"), manifest)
        .unwrap_or_else(|error| panic!("failed to write mesh.toml: {error}"));
    fs::write(project_dir.join("main.mpl"), SQLITE_BUILT_PACKAGE_SOURCE)
        .unwrap_or_else(|error| panic!("failed to write main.mpl: {error}"));

    route_free::write_json_artifact(
        &artifacts.join("scenario-meta.json"),
        &json!({
            "package_dir": project_dir.display().to_string(),
            "binary_name": "output",
            "checks": [
                "Sqlite.execute(..., []) succeeds in helper",
                "Sqlite.execute(..., [name, age]) succeeds in helper",
                "Sqlite.query(..., [age]) read-back succeeds in helper",
                "placeholder mismatch returns an error message instead of hanging or crashing"
            ]
        }),
    );

    project_dir
}

fn build_package_binary(project_dir: &Path, artifacts: &Path) -> PathBuf {
    route_free::ensure_mesh_rt_staticlib();

    let output = Command::new(route_free::meshc_bin())
        .current_dir(project_dir)
        .args(["build", "."])
        .output()
        .unwrap_or_else(|error| {
            panic!(
                "failed to invoke meshc build . in {}: {error}",
                project_dir.display()
            )
        });

    route_free::write_artifact(
        &artifacts.join("build.log"),
        route_free::command_output_text(&output),
    );
    assert!(
        output.status.success(),
        "meshc build . should succeed in {}:\n{}\nartifacts: {}",
        project_dir.display(),
        route_free::command_output_text(&output),
        artifacts.display()
    );

    let binary_path = project_dir.join("output");
    assert!(
        binary_path.exists(),
        "meshc build . should emit {}\nartifacts: {}",
        binary_path.display(),
        artifacts.display()
    );

    route_free::write_json_artifact(
        &artifacts.join("build-meta.json"),
        &route_free::BuildOutputMetadata {
            source_package_dir: project_dir.to_path_buf(),
            binary_path: binary_path.clone(),
        },
    );

    binary_path
}

fn wait_with_timeout(mut child: Child, timeout: Duration) -> Result<Output, String> {
    let start = Instant::now();
    let poll_interval = Duration::from_millis(50);

    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let mut stdout = Vec::new();
                let mut stderr = Vec::new();
                if let Some(mut out) = child.stdout.take() {
                    out.read_to_end(&mut stdout).ok();
                }
                if let Some(mut err) = child.stderr.take() {
                    err.read_to_end(&mut stderr).ok();
                }
                return Ok(Output {
                    status,
                    stdout,
                    stderr,
                });
            }
            Ok(None) => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(format!(
                        "binary timed out after {} seconds",
                        timeout.as_secs()
                    ));
                }
                sleep(poll_interval);
            }
            Err(error) => return Err(format!("error waiting for process: {error}")),
        }
    }
}

fn run_binary(binary_path: &Path, current_dir: &Path, artifacts: &Path, label: &str) -> Output {
    let child = Command::new(binary_path)
        .current_dir(current_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| {
            panic!(
                "failed to spawn {} from {}: {error}",
                binary_path.display(),
                current_dir.display()
            )
        });

    let output = wait_with_timeout(child, RUN_TIMEOUT).unwrap_or_else(|error| {
        route_free::write_artifact(&artifacts.join(format!("{label}.timeout.txt")), &error);
        panic!(
            "sqlite built-package regression timed out for {}\nartifacts: {}\n{}",
            binary_path.display(),
            artifacts.display(),
            error
        );
    });

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let combined = format!("stdout:\n{}\nstderr:\n{}", stdout, stderr);

    route_free::write_artifact(&artifacts.join(format!("{label}.stdout.log")), &stdout);
    route_free::write_artifact(&artifacts.join(format!("{label}.stderr.log")), &stderr);
    route_free::write_artifact(&artifacts.join(format!("{label}.combined.log")), &combined);
    route_free::write_artifact(
        &artifacts.join(format!("{label}.status.txt")),
        format!("{:?}", output.status.code()),
    );

    output
}

fn non_empty_lines(output: &Output) -> Vec<String> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

#[test]
fn m047_s06_built_package_sqlite_execute_handles_helper_rewraps_and_mismatch_errors() {
    let artifacts = artifact_dir("sqlite-built-package-execute");
    let project_dir = write_package_fixture(&artifacts);
    let binary_path = build_package_binary(&project_dir, &artifacts);
    let run_output = run_binary(&binary_path, &project_dir, &artifacts, "run");
    let stdout = String::from_utf8_lossy(&run_output.stdout);
    let stderr = String::from_utf8_lossy(&run_output.stderr);
    let lines = non_empty_lines(&run_output);

    assert!(
        run_output.status.success(),
        "built package should exit successfully after SQLite helper regression check\nstdout:\n{}\nstderr:\n{}\nartifacts: {}",
        stdout,
        stderr,
        artifacts.display()
    );
    assert!(
        !stdout.contains("schema=err:")
            && !stdout.contains("insert=err:")
            && !stdout.contains("count=err:"),
        "SQLite helper stages should succeed before the negative mismatch check\nstdout:\n{}\nartifacts: {}",
        stdout,
        artifacts.display()
    );
    assert!(
        lines.iter().any(|line| line == "schema=ok"),
        "expected schema helper success marker\nstdout:\n{}\nartifacts: {}",
        stdout,
        artifacts.display()
    );
    assert!(
        lines.iter().any(|line| line == "insert=1"),
        "expected parameterized insert success marker\nstdout:\n{}\nartifacts: {}",
        stdout,
        artifacts.display()
    );
    assert!(
        lines.iter().any(|line| line == "count=1"),
        "expected read-back count success marker\nstdout:\n{}\nartifacts: {}",
        stdout,
        artifacts.display()
    );

    let mismatch_line = lines
        .iter()
        .find(|line| line.starts_with("mismatch_err="))
        .unwrap_or_else(|| {
            panic!(
                "expected placeholder mismatch marker\nstdout:\n{}\nartifacts: {}",
                stdout,
                artifacts.display()
            )
        });
    assert_ne!(
        mismatch_line,
        "mismatch_err=unexpected-ok:1",
        "placeholder mismatch must fail closed, not succeed\nstdout:\n{}\nartifacts: {}",
        stdout,
        artifacts.display()
    );
    assert_ne!(
        mismatch_line,
        "mismatch_err=",
        "placeholder mismatch should preserve SQLite error text\nstdout:\n{}\nartifacts: {}",
        stdout,
        artifacts.display()
    );
    assert!(
        lines.iter().any(|line| line == "done"),
        "expected terminal success marker\nstdout:\n{}\nartifacts: {}",
        stdout,
        artifacts.display()
    );
}
