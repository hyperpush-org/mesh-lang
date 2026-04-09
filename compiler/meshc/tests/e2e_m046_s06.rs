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

fn assert_clustered_surface_omits_routeful_drift(path: &Path) {
    assert_source_omits_all(
        path,
        &[
            "Continuity.submit_declared_work",
            "/work/:request_key",
            "Timer.sleep(5000)",
            "tiny-cluster/README.md",
            "cluster-proof/README.md",
        ],
    );
}

#[test]
fn m046_s06_historical_wrapper_alias_contract() {
    let verifier_path = repo_root().join("scripts").join("verify-m046-s06.sh");

    assert_source_contains_all(
        &verifier_path,
        &[
            "bash scripts/verify-m047-s04.sh",
            "retained-m047-s04-verify",
            "latest-proof-bundle.txt",
            "phase-report.txt",
            "status.txt",
            "current-phase.txt",
            "full-contract.log",
            "m047-s04-replay",
            "retain-m047-s04-verify",
        ],
    );

    assert_source_omits_all(
        &verifier_path,
        &[
            "bash scripts/verify-m046-s05.sh",
            "bash scripts/verify-m046-s04.sh",
            "bash scripts/verify-m046-s03.sh",
            "cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_startup_dedupes_and_surfaces_runtime_truth_on_two_nodes -- --nocapture",
            "cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_failover_proves_promotion_recovery_completion_and_fenced_rejoin_from_cli_surfaces -- --nocapture",
            "cargo test -p meshc --test e2e_m046_s04 m046_s04_cluster_proof_startup_dedupes_and_surfaces_runtime_truth_on_two_nodes -- --nocapture",
            "retained-m046-s05-verify",
            "retained-m046-s06-artifacts",
        ],
    );
}

#[test]
fn m046_s06_authoritative_docs_contract() {
    let authoritative_surfaces = [
        repo_root().join("README.md"),
        repo_root()
            .join("website")
            .join("docs")
            .join("docs")
            .join("distributed-proof")
            .join("index.md"),
    ];

    for path in authoritative_surfaces {
        assert_source_contains_all(
            &path,
            &[
                "`bash scripts/verify-m047-s04.sh` — the authoritative cutover rail for the source-first route-free clustered contract",
                "`bash scripts/verify-m046-s06.sh` — the historical M046 closeout wrapper retained as a compatibility alias into the M047 cutover rail",
                "`bash scripts/verify-m046-s05.sh` — the historical M046 equal-surface wrapper retained as a compatibility alias into the M047 cutover rail",
                "`bash scripts/verify-m045-s05.sh` — the historical M045 closeout wrapper retained as a compatibility alias into the M047 cutover rail",
            ],
        );

        assert_source_omits_all(
            &path,
            &[
                "`bash scripts/verify-m046-s06.sh` — the authoritative assembled closeout rail",
                "`bash scripts/verify-m046-s05.sh` — the lower-level equal-surface subrail",
                "`bash scripts/verify-m045-s05.sh` — the historical wrapper name retained for replay and transition into the S06 closeout rail",
            ],
        );

        assert_clustered_surface_omits_routeful_drift(&path);
    }
}

#[test]
fn m046_s06_clustered_docs_and_runbooks_repoint_to_the_m047_cutover_rail() {
    let repo_root = repo_root();
    let surfaces = [
        repo_root.join("website/docs/docs/distributed/index.md"),
        repo_root.join("website/docs/docs/tooling/index.md"),
        repo_root.join("website/docs/docs/getting-started/clustered-example/index.md"),
        repo_root.join("scripts/fixtures/clustered/tiny-cluster/README.md"),
        repo_root.join("scripts/fixtures/clustered/cluster-proof/README.md"),
    ];

    for path in surfaces {
        assert_source_contains(&path, "bash scripts/verify-m047-s04.sh");
        assert_source_contains(&path, "bash scripts/verify-m046-s06.sh");
        assert_source_contains(&path, "bash scripts/verify-m046-s05.sh");
        assert_source_contains(&path, "bash scripts/verify-m045-s05.sh");
        assert_source_omits_all(
            &path,
            &[
                "The authoritative assembled closeout rail is `bash scripts/verify-m046-s06.sh`",
                "the authoritative repo-wide closeout rail is `bash scripts/verify-m046-s06.sh`",
                "For the repo-wide closeout story, `bash scripts/verify-m046-s06.sh` is the authoritative assembled closeout rail",
            ],
        );
        assert_clustered_surface_omits_routeful_drift(&path);
    }
}

#[test]
fn m046_s06_package_runbooks_keep_the_runtime_owned_operator_flow() {
    let package_surfaces = [
        repo_root().join("scripts/fixtures/clustered/tiny-cluster/README.md"),
        repo_root().join("scripts/fixtures/clustered/cluster-proof/README.md"),
    ];

    for path in package_surfaces {
        assert_source_contains_all(
            &path,
            &[
                "meshc cluster status <node-name@host:port> --json",
                "meshc cluster continuity <node-name@host:port> --json",
                "meshc cluster continuity <node-name@host:port> <request-key> --json",
                "meshc cluster diagnostics <node-name@host:port> --json",
            ],
        );
        assert_clustered_surface_omits_routeful_drift(&path);
    }
}
