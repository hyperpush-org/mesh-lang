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
fn m044_s05_historical_closeout_wrapper_contract() {
    let verifier_path = repo_root().join("scripts").join("verify-m044-s05.sh");

    assert_source_contains_all(
        &verifier_path,
        &[
            "bash scripts/verify-m046-s04.sh",
            "cargo test -p meshc --test e2e_m044_s05 m044_s05_ -- --nocapture",
            "retained-m046-s04-verify",
            "latest-proof-bundle.txt",
            "phase-report.txt",
            "status.txt",
            "current-phase.txt",
            "full-contract.log",
            "m046-s04-e2e",
            "m046-s04-bundle-shape",
        ],
    );

    assert_source_omits_all(
        &verifier_path,
        &[
            "bash scripts/verify-m044-s03.sh",
            "bash scripts/verify-m044-s04.sh",
            "bash scripts/verify-m045-s03.sh",
            "bash scripts/verify-m045-s04.sh",
            "bash scripts/verify-m045-s05.sh",
            "cargo run -q -p meshc -- build cluster-proof",
            "cargo run -q -p meshc -- test cluster-proof/tests",
            "npm --prefix website run build",
            "README.md",
            "website/docs",
            "/work",
            "/membership",
            "CLUSTER_PROOF_WORK_DELAY_MS",
            "http_service",
            "mesh-cluster-proof.fly.dev",
            "mesh-cluster-proof.fly.io",
        ],
    );
}

#[test]
fn m044_s05_route_free_package_story_contract() {
    let cluster_proof_readme = repo_root().join("cluster-proof").join("README.md");

    assert_source_contains_all(
        &cluster_proof_readme,
        &[
            "Node.start_from_env()",
            "meshc cluster status <node-name@host:port> --json",
            "meshc cluster continuity <node-name@host:port> <request-key> --json",
            "meshc cluster diagnostics <node-name@host:port> --json",
        ],
    );

    assert_source_omits_all(
        &cluster_proof_readme,
        &[
            "/work",
            "/membership",
            "CLUSTER_PROOF_WORK_DELAY_MS",
            "http_service",
            "mesh-cluster-proof.fly.dev",
            "mesh-cluster-proof.fly.io",
        ],
    );
}
