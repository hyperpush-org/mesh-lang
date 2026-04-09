use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
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

fn assert_source_contains_all(path: &Path, needles: &[&str]) {
    for needle in needles {
        assert_source_contains(path, needle);
    }
}

fn assert_source_omits_all(path: &Path, needles: &[&str]) {
    for needle in needles {
        assert_source_omits(path, needle);
    }
}

#[test]
fn m045_s05_historical_closeout_wrapper_alias_contract() {
    let verifier_path = repo_root().join("scripts").join("verify-m045-s05.sh");

    assert_source_contains_all(
        &verifier_path,
        &[
            "bash scripts/verify-m047-s04.sh",
            "cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture",
            "retained-m047-s04-verify",
            "latest-proof-bundle.txt",
            "phase-report.txt",
            "status.txt",
            "current-phase.txt",
            "full-contract.log",
            "m047-s04-replay",
            "retain-m047-s04-verify",
            "m045-s05-contract",
        ],
    );

    assert_source_omits_all(
        &verifier_path,
        &[
            "bash scripts/verify-m046-s06.sh",
            "bash scripts/verify-m046-s05.sh",
            "bash scripts/verify-m046-s04.sh",
            "cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture",
            "cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture",
            "cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture",
            "npm --prefix website run build",
        ],
    );
}

#[test]
fn m045_s05_authoritative_cutover_dependency_contract() {
    let delegated_verifier_path = repo_root().join("scripts").join("verify-m047-s04.sh");

    assert_source_contains_all(
        &delegated_verifier_path,
        &[
            "cargo test -p mesh-parser m047_s04 -- --nocapture",
            "cargo test -p mesh-pkg m047_s04 -- --nocapture",
            "cargo test -p meshc --test e2e_m047_s01 -- --nocapture",
            "cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture",
            "cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture",
            "cargo run -q -p meshc -- test scripts/fixtures/clustered/tiny-cluster/tests",
            "cargo run -q -p meshc -- build scripts/fixtures/clustered/tiny-cluster",
            "cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests",
            "cargo run -q -p meshc -- build scripts/fixtures/clustered/cluster-proof",
            "npm --prefix website run build",
            "cargo test -p meshc --test e2e_m047_s04 -- --nocapture",
            "retained-m047-s04-artifacts",
            "latest-proof-bundle.txt",
            "m047-s04-bundle-shape",
        ],
    );
}
