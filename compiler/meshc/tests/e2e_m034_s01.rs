use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn meshc_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_meshc"))
}

fn package_manifest(name: &str) -> String {
    format!(
        "[package]\nname = \"{}\"\nversion = \"1.0.0\"\n\ndescription = \"Scoped install regression fixture\"\nlicense = \"MIT\"\n\n[dependencies]\n",
        name
    )
}

fn build_project(project_dir: &Path) -> Output {
    Command::new(meshc_bin())
        .args(["build", project_dir.to_str().unwrap()])
        .output()
        .expect("failed to invoke meshc build")
}

#[test]
fn scoped_installed_package_builds() {
    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
    let project_dir = temp_dir.path().join("consumer");
    let package_root = project_dir.join(".mesh/packages/acme/greeter@1.0.0");

    std::fs::create_dir_all(package_root.join("support"))
        .expect("failed to create scoped package dirs");
    std::fs::write(
        project_dir.join("main.mpl"),
        "from Support.Message import message\n\nfn main() do\n  println(message())\nend\n",
    )
    .expect("failed to write consumer main.mpl");
    std::fs::write(
        package_root.join("mesh.toml"),
        package_manifest("acme/greeter"),
    )
    .expect("failed to write package manifest");
    std::fs::write(package_root.join("main.mpl"), "fn main() do\n  0\nend\n")
        .expect("failed to write package main.mpl");
    std::fs::write(
        package_root.join("support/message.mpl"),
        "pub fn message() -> String do\n  \"hello from scoped package\"\nend\n",
    )
    .expect("failed to write package module");

    let output = build_project(&project_dir);
    assert!(
        output.status.success(),
        "meshc build should succeed for a scoped installed package without flattening:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let binary = project_dir.join("consumer");
    let run_output = Command::new(&binary)
        .output()
        .unwrap_or_else(|e| panic!("failed to run binary at {}: {}", binary.display(), e));

    assert!(
        run_output.status.success(),
        "consumer binary should run successfully:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&run_output.stdout),
        String::from_utf8_lossy(&run_output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&run_output.stdout).trim(),
        "hello from scoped package"
    );
}
