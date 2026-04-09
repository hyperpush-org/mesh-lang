use super::m046_route_free as route_free;
use super::m049_todo_postgres_scaffold as postgres;
use super::m049_todo_sqlite_scaffold as sqlite;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const SQLITE_EXAMPLE_NAME: &str = "todo-sqlite";
pub const POSTGRES_EXAMPLE_NAME: &str = "todo-postgres";
pub const SQLITE_STORAGE_TEST_RELATIVE_PATH: &str = "tests/storage.test.mpl";
pub const POSTGRES_MIGRATION_RELATIVE_PATH: &str = "migrations/20260402120000_create_todos.mpl";
pub const POSTGRES_DEPLOY_SQL_RELATIVE_PATH: &str = "deploy/todo-postgres.up.sql";
pub const POSTGRES_STAGE_DEPLOY_SCRIPT_RELATIVE_PATH: &str = "scripts/stage-deploy.sh";
pub const POSTGRES_APPLY_DEPLOY_SCRIPT_RELATIVE_PATH: &str = "scripts/apply-deploy-migrations.sh";
pub const POSTGRES_DEPLOY_SMOKE_SCRIPT_RELATIVE_PATH: &str = "scripts/deploy-smoke.sh";

const MATERIALIZER_RUNNER_SCRIPT: &str = r#"
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const moduleUrl = pathToFileURL(
  path.resolve(process.cwd(), 'scripts/tests/verify-m049-s03-materialize-examples.mjs')
);
const { materializeExamples } = await import(moduleUrl.href);

try {
  const summary = materializeExamples({
    mode: process.env.M049_MATERIALIZE_MODE,
    examplesRoot: process.env.M049_EXAMPLES_ROOT || undefined,
    meshcBin: process.env.M049_MESHC_BIN || undefined,
    tempParent: process.env.M049_TEMP_PARENT || undefined,
    keepTemp: process.env.M049_KEEP_TEMP === '1',
  });
  process.stdout.write(JSON.stringify(summary));
} catch (error) {
  const message = error instanceof Error ? error.message : String(error);
  process.stderr.write(`${message}\n`);
  process.exit(1);
}
"#;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterializeSummary {
    pub mode: String,
    pub examples_root: PathBuf,
    pub meshc_bin: PathBuf,
    pub session_dir: PathBuf,
    pub examples: Vec<MaterializeExample>,
    #[serde(default)]
    pub lines: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterializeExample {
    pub example: ExampleDefinition,
    pub generated_dir: PathBuf,
    pub target_dir: PathBuf,
    pub generated_manifest: TreeManifest,
    #[serde(default)]
    pub target_exists: bool,
    #[serde(default)]
    pub prior_diff: ManifestDiff,
    #[serde(default)]
    pub target_manifest: Option<TreeManifest>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleDefinition {
    pub name: String,
    pub db: String,
    #[serde(default)]
    pub required_paths: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeManifest {
    pub root_dir: PathBuf,
    pub file_count: usize,
    pub dir_count: usize,
    pub fingerprint: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManifestDiff {
    pub missing: Vec<String>,
    pub extra: Vec<String>,
    pub changed: Vec<String>,
}

impl ManifestDiff {
    pub fn is_clean(&self) -> bool {
        self.missing.is_empty() && self.extra.is_empty() && self.changed.is_empty()
    }
}

pub fn repo_root() -> PathBuf {
    route_free::repo_root()
}

pub fn meshc_bin() -> PathBuf {
    route_free::meshc_bin()
}

pub fn examples_root() -> PathBuf {
    repo_root().join("examples")
}

pub fn sqlite_example_dir() -> PathBuf {
    examples_root().join(SQLITE_EXAMPLE_NAME)
}

pub fn postgres_example_dir() -> PathBuf {
    examples_root().join(POSTGRES_EXAMPLE_NAME)
}

pub fn artifact_dir(test_name: &str) -> PathBuf {
    route_free::artifact_dir("m049-s03", test_name)
}

pub fn write_json_artifact(path: &Path, value: &impl serde::Serialize) {
    route_free::write_json_artifact(path, value);
}

pub fn clone_examples_root(destination_root: &Path) {
    route_free::archive_directory_tree(&examples_root(), destination_root);
}

pub fn materialize_examples_check(examples_root: &Path, artifacts: &Path) -> MaterializeSummary {
    if examples_root.is_dir() {
        route_free::archive_directory_tree(examples_root, &artifacts.join("input-examples-root"));
    }

    let materializer_artifacts = artifacts.join("materializer");
    ensure_directory(&materializer_artifacts);
    let run = run_materializer_check_command(examples_root, &materializer_artifacts);
    sqlite::assert_phase_success(&run, "materializer check should succeed");

    let summary = serde_json::from_str::<MaterializeSummary>(&run.stdout).unwrap_or_else(|error| {
        panic!(
            "materializer check returned invalid JSON in {}: {error}\nstdout:\n{}\nstderr:\n{}",
            run.stdout_path.display(),
            run.stdout,
            run.stderr
        )
    });
    assert_eq!(
        summary.mode, "check",
        "materializer summary mode drifted: {summary:?}"
    );

    write_json_artifact(
        &materializer_artifacts.join("materializer-summary.json"),
        &summary,
    );
    route_free::write_artifact(
        &materializer_artifacts.join("retained-session.txt"),
        summary.session_dir.display().to_string(),
    );

    for entry in &summary.examples {
        route_free::archive_directory_tree(
            &entry.generated_dir,
            &materializer_artifacts
                .join("generated")
                .join(&entry.example.name),
        );
        if entry.target_dir.is_dir() {
            route_free::archive_directory_tree(
                &entry.target_dir,
                &materializer_artifacts
                    .join("target")
                    .join(&entry.example.name),
            );
        }
        write_json_artifact(
            &materializer_artifacts.join(format!("{}-generated-manifest.json", entry.example.name)),
            &entry.generated_manifest,
        );
        write_json_artifact(
            &materializer_artifacts.join(format!("{}-prior-diff.json", entry.example.name)),
            &entry.prior_diff,
        );
        if let Some(target_manifest) = &entry.target_manifest {
            write_json_artifact(
                &materializer_artifacts
                    .join(format!("{}-target-manifest.json", entry.example.name)),
                target_manifest,
            );
        }
    }

    summary
}

pub fn materialize_examples_check_expect_failure(
    examples_root: &Path,
    artifacts: &Path,
) -> sqlite::CompletedCommand {
    if examples_root.is_dir() {
        route_free::archive_directory_tree(examples_root, &artifacts.join("input-examples-root"));
    }

    let materializer_artifacts = artifacts.join("materializer");
    ensure_directory(&materializer_artifacts);
    let run = run_materializer_check_command(examples_root, &materializer_artifacts);
    assert!(
        !run.status.success(),
        "materializer check should fail closed for {}\nstdout={} stderr={} meta={}\n{}",
        examples_root.display(),
        run.stdout_path.display(),
        run.stderr_path.display(),
        run.meta_path.display(),
        run.combined
    );
    run
}

pub fn assert_expected_example_set(summary: &MaterializeSummary) {
    let sqlite = example_entry(summary, SQLITE_EXAMPLE_NAME);
    let postgres = example_entry(summary, POSTGRES_EXAMPLE_NAME);

    assert_eq!(sqlite.example.db, "sqlite");
    assert_eq!(sqlite.target_dir, sqlite_example_dir());
    assert!(sqlite.target_exists);
    assert!(
        sqlite.prior_diff.is_clean(),
        "SQLite example drifted even though check succeeded: {:?}",
        sqlite.prior_diff
    );
    assert_eq!(postgres.example.db, "postgres");
    assert_eq!(postgres.target_dir, postgres_example_dir());
    assert!(postgres.target_exists);
    assert!(
        postgres.prior_diff.is_clean(),
        "Postgres example drifted even though check succeeded: {:?}",
        postgres.prior_diff
    );
    assert_eq!(
        summary.examples.len(),
        2,
        "expected exactly two tracked examples in summary, got {:?}",
        summary
            .examples
            .iter()
            .map(|entry| entry.example.name.as_str())
            .collect::<Vec<_>>()
    );
}

pub fn example_entry<'a>(summary: &'a MaterializeSummary, name: &str) -> &'a MaterializeExample {
    let mut matches = summary
        .examples
        .iter()
        .filter(|entry| entry.example.name == name);
    let entry = matches
        .next()
        .unwrap_or_else(|| panic!("missing example {name} in materializer summary"));
    assert!(
        matches.next().is_none(),
        "duplicate example {name} in materializer summary"
    );
    entry
}

pub fn assert_sqlite_example_shape(project_dir: &Path) {
    assert_project_name(project_dir, SQLITE_EXAMPLE_NAME);
    assert_required_file(
        project_dir,
        SQLITE_STORAGE_TEST_RELATIVE_PATH,
        "SQLite example",
    );
    assert_absent_path(project_dir, "work.mpl", "SQLite example");
    assert_absent_path(project_dir, ".env.example", "SQLite example");
    assert_absent_path(project_dir, "migrations", "SQLite example");
    assert_absent_path(project_dir, "deploy", "SQLite example");
    assert_absent_path(
        project_dir,
        POSTGRES_STAGE_DEPLOY_SCRIPT_RELATIVE_PATH,
        "SQLite example",
    );
}

pub fn assert_postgres_example_shape(project_dir: &Path) {
    assert_project_name(project_dir, POSTGRES_EXAMPLE_NAME);
    assert_required_file(
        project_dir,
        POSTGRES_MIGRATION_RELATIVE_PATH,
        "Postgres example",
    );
    assert_required_file(
        project_dir,
        POSTGRES_DEPLOY_SQL_RELATIVE_PATH,
        "Postgres example",
    );
    assert_required_file(
        project_dir,
        POSTGRES_STAGE_DEPLOY_SCRIPT_RELATIVE_PATH,
        "Postgres example",
    );
    assert_required_file(
        project_dir,
        POSTGRES_APPLY_DEPLOY_SCRIPT_RELATIVE_PATH,
        "Postgres example",
    );
    assert_required_file(
        project_dir,
        POSTGRES_DEPLOY_SMOKE_SCRIPT_RELATIVE_PATH,
        "Postgres example",
    );
    assert_required_file(project_dir, "work.mpl", "Postgres example");
    assert_required_file(project_dir, ".env.example", "Postgres example");
    assert_absent_path(
        project_dir,
        SQLITE_STORAGE_TEST_RELATIVE_PATH,
        "Postgres example",
    );
}

pub fn verify_sqlite_example_meshc_test_and_build(project_dir: &Path, artifacts: &Path) {
    assert_sqlite_example_shape(project_dir);
    route_free::archive_directory_tree(project_dir, &artifacts.join("project"));

    let test_artifacts = artifacts.join("meshc-test");
    ensure_directory(&test_artifacts);
    let meshc_test = sqlite::run_meshc_tests(project_dir, &test_artifacts);
    sqlite::assert_phase_success(&meshc_test, "meshc test <project> should succeed");
    assert!(
        meshc_test
            .stdout
            .contains("SQLite todo-api config > exposes local environment keys and defaults"),
        "expected SQLite config test names in meshc test output, got:\n{}",
        meshc_test.combined
    );
    assert!(
        meshc_test.stdout.contains(
            "SQLite todo storage > local storage module compiles for the generated starter"
        ),
        "expected SQLite storage test names in meshc test output, got:\n{}",
        meshc_test.combined
    );
    let sqlite_two_pass_markers = meshc_test.stdout.matches("2 passed").count();
    assert!(
        sqlite_two_pass_markers >= 3,
        "expected SQLite meshc test output to report two passing generated tests for each file and the final package summary, got {} markers:\n{}",
        sqlite_two_pass_markers,
        meshc_test.combined
    );
    assert!(
        !meshc_test.combined.contains("COMPILE ERROR"),
        "SQLite example meshc test must compile cleanly:\n{}",
        meshc_test.combined
    );

    let build_artifacts = artifacts.join("build");
    ensure_directory(&build_artifacts);
    let (build, binary_path) = sqlite::run_meshc_build(project_dir, &build_artifacts);
    sqlite::assert_phase_success(&build, "meshc build <project> should succeed");
    assert_binary_output_contract(project_dir, &binary_path);
    let metadata = route_free::read_required_build_metadata(&build_artifacts)
        .unwrap_or_else(|error| panic!("missing SQLite build metadata: {error}"));
    assert_eq!(metadata.source_package_dir, project_dir);
    assert_eq!(metadata.binary_path, binary_path);
}

pub fn verify_postgres_example_meshc_test_and_build(project_dir: &Path, artifacts: &Path) {
    assert_postgres_example_shape(project_dir);
    route_free::archive_directory_tree(project_dir, &artifacts.join("project"));

    let test_artifacts = artifacts.join("meshc-test");
    ensure_directory(&test_artifacts);
    let meshc_test = postgres::run_meshc_tests(project_dir, &test_artifacts);
    postgres::assert_phase_success(&meshc_test, "meshc test <project> should succeed");
    assert!(
        meshc_test
            .stdout
            .contains("Config helpers > exposes the canonical environment variable keys"),
        "expected Postgres config test names in meshc test output, got:\n{}",
        meshc_test.combined
    );
    let postgres_two_pass_markers = meshc_test.stdout.matches("2 passed").count();
    assert!(
        postgres_two_pass_markers >= 1,
        "expected Postgres meshc test output to report the two generated config tests, got {} markers:\n{}",
        postgres_two_pass_markers,
        meshc_test.combined
    );
    let postgres_one_pass_markers = meshc_test.stdout.matches("1 passed").count();
    assert!(
        postgres_one_pass_markers >= 1,
        "expected Postgres meshc test output to report the final one-file package summary, got {} markers:\n{}",
        postgres_one_pass_markers,
        meshc_test.combined
    );
    assert!(
        meshc_test
            .combined
            .contains("startup::Work.sync_todos"),
        "expected Postgres meshc test output to retain the runtime-owned startup record marker, got:\n{}",
        meshc_test.combined
    );
    assert!(
        !meshc_test.combined.contains("COMPILE ERROR"),
        "Postgres example meshc test must compile cleanly:\n{}",
        meshc_test.combined
    );

    let build_artifacts = artifacts.join("build");
    ensure_directory(&build_artifacts);
    let (build, binary_path) = postgres::run_meshc_build(project_dir, &build_artifacts);
    postgres::assert_phase_success(&build, "meshc build <project> should succeed");
    assert_binary_output_contract(project_dir, &binary_path);
    let metadata = read_build_metadata(&build_artifacts.join("build-output.json"));
    assert_eq!(metadata.source_package_dir, project_dir);
    assert_eq!(metadata.binary_path, binary_path);
}

fn run_materializer_check_command(
    examples_root: &Path,
    artifacts: &Path,
) -> sqlite::CompletedCommand {
    ensure_directory(artifacts);
    let temp_parent = artifacts.join("temp");
    ensure_directory(&temp_parent);

    let mut command = Command::new("node");
    command
        .current_dir(repo_root())
        .env("M049_MATERIALIZE_MODE", "check")
        .env("M049_EXAMPLES_ROOT", examples_root)
        .env("M049_MESHC_BIN", meshc_bin())
        .env("M049_TEMP_PARENT", &temp_parent)
        .env("M049_KEEP_TEMP", "1")
        .arg("--input-type=module")
        .arg("-e")
        .arg(MATERIALIZER_RUNNER_SCRIPT);

    sqlite::run_command_capture(
        &mut command,
        artifacts,
        "materializer-check",
        "node materializer check",
        sqlite::PHASE_TIMEOUT,
    )
}

fn assert_project_name(project_dir: &Path, expected_name: &str) {
    let mesh_toml_path = project_dir.join("mesh.toml");
    let mesh_toml = fs::read_to_string(&mesh_toml_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", mesh_toml_path.display()));
    let expected_line = format!("name = \"{expected_name}\"");
    assert!(
        mesh_toml.contains(&expected_line),
        "expected {} to declare {}, got:\n{}",
        mesh_toml_path.display(),
        expected_line,
        mesh_toml
    );
}

fn assert_required_file(project_dir: &Path, relative_path: &str, label: &str) {
    let path = project_dir.join(relative_path);
    assert!(
        path.is_file(),
        "{label} should contain {}, but it is missing at {}",
        relative_path,
        path.display()
    );
}

fn assert_absent_path(project_dir: &Path, relative_path: &str, label: &str) {
    let path = project_dir.join(relative_path);
    assert!(
        !path.exists(),
        "{label} should omit {}, but it exists at {}",
        relative_path,
        path.display()
    );
}

fn assert_binary_output_contract(project_dir: &Path, binary_path: &Path) {
    let project_name = project_dir
        .file_name()
        .unwrap_or_else(|| panic!("missing project dir name for {}", project_dir.display()));
    assert!(
        binary_path.exists(),
        "meshc build reported success but binary is missing at {}",
        binary_path.display()
    );
    assert_eq!(binary_path.file_name(), Some(project_name));
    assert!(
        !binary_path.starts_with(project_dir),
        "meshc build --output should keep binaries out of the tracked project dir {}, but wrote {}",
        project_dir.display(),
        binary_path.display()
    );

    let default_binary_path = project_dir.join(project_name);
    assert!(
        !default_binary_path.exists(),
        "meshc build --output must not emit the default repo-tree binary at {}",
        default_binary_path.display()
    );
    let accidental_output_path = project_dir.join("output");
    assert!(
        !accidental_output_path.exists(),
        "meshc build --output must not create {}",
        accidental_output_path.display()
    );
}

fn read_build_metadata(path: &Path) -> route_free::BuildOutputMetadata {
    let raw = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()))
}

fn ensure_directory(path: &Path) {
    fs::create_dir_all(path)
        .unwrap_or_else(|error| panic!("failed to create {}: {error}", path.display()));
}
