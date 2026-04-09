mod support;

use serde_json::json;
use std::path::{Path, PathBuf};

use support::m046_route_free as route_free;

const VERIFIER_PATH: &str = "scripts/verify-m054-s03.sh";
const SOURCE_CONTRACT_PATH: &str = "scripts/tests/verify-m054-s03-contract.test.mjs";
const HOMEPAGE_PATH: &str = "website/docs/index.md";
const CONFIG_PATH: &str = "website/docs/.vitepress/config.mts";
const PROOF_PATH: &str = "website/docs/docs/distributed-proof/index.md";
const OG_SOURCE_PATH: &str = "website/scripts/generate-og-image.py";

const HOMEPAGE_DESCRIPTION: &str = "One public app URL fronts multiple Mesh nodes. Runtime placement stays server-side, and operator truth stays on meshc cluster.";
const CONFIG_ALT_MARKER: &str =
    "one public app URL, server-side runtime placement, and operator truth on meshc cluster.";
const PROOF_BOUNDARY_MARKER: &str = "A proxy/platform ingress may expose one public app URL in front of multiple nodes, but that is where the public routing story ends.";
const PROOF_HEADER_MARKER: &str = "clustered `GET /todos` and `GET /todos/:id` responses include `X-Mesh-Continuity-Request-Key`; when you have that header, jump straight to the same request with `meshc cluster continuity <node-name@host:port> <request-key> --json`";
const PROOF_HEADER_HTML_MARKER: &str =
    "meshc cluster continuity &lt;node-name@host:port&gt; &lt;request-key&gt; --json";
const PROOF_LIST_MARKER: &str =
    "continuity-list discovery stays for startup records and manual inspection when you do not already have a request key";
const PROOF_LIST_HTML_MARKER: &str =
    "If you are inspecting startup work or doing manual discovery without a request key yet";
const PROOF_WORKFLOW_MARKER: &str = "If a clustered HTTP response returned `X-Mesh-Continuity-Request-Key`, run `meshc cluster continuity <node-name@host:port> <request-key> --json` directly for that same public request.";
const PROOF_NON_GOAL_MARKER: &str =
    "sticky sessions, frontend-aware routing, or client-visible topology claims";
const FLY_EVIDENCE_MARKER: &str = "keep Fly as a retained read-only reference/proof lane for already-deployed environments instead of treating it as a coequal public starter surface";
const OG_SUBTITLE_MARKER: &str = "subtitle = 'One public app URL. Server-side runtime placement. Operator truth stays on meshc cluster.'";
const OG_BADGE_MARKER: &str =
    "for badge in ['@cluster', 'LLVM native', 'Type-safe', 'One public URL']";
const OLD_GENERIC_TAGLINE: &str = "Built-in failover, load balancing, and exactly-once semantics";

const SOURCE_CONTRACT_COMMAND: &str = "node --test scripts/tests/verify-m054-s03-contract.test.mjs";
const RUST_CONTRACT_COMMAND: &str = "cargo test -p meshc --test e2e_m054_s03 -- --nocapture";
const S02_REPLAY_COMMAND: &str = "bash scripts/verify-m054-s02.sh";
const GENERATE_OG_COMMAND: &str = "npm --prefix website run generate:og";
const DOCS_BUILD_COMMAND: &str = "npm --prefix website run build";

struct ContractSources {
    verifier: String,
    source_contract: String,
    index: String,
    config: String,
    proof: String,
    og: String,
}

fn repo_root() -> PathBuf {
    route_free::repo_root()
}

fn artifact_dir(test_name: &str) -> PathBuf {
    route_free::artifact_dir("m054-s03", test_name)
}

fn require_includes(errors: &mut Vec<String>, path_label: &str, source: &str, needles: &[&str]) {
    for needle in needles {
        if !source.contains(needle) {
            errors.push(format!("{path_label} missing {needle:?}"));
        }
    }
}

fn require_omits(errors: &mut Vec<String>, path_label: &str, source: &str, needles: &[&str]) {
    for needle in needles {
        if source.contains(needle) {
            errors.push(format!("{path_label} still contains stale text {needle:?}"));
        }
    }
}

fn require_order(errors: &mut Vec<String>, path_label: &str, source: &str, needles: &[&str]) {
    let mut previous_index = None;
    for needle in needles {
        let Some(index) = source.find(needle) else {
            errors.push(format!("{path_label} missing ordered marker {needle:?}"));
            return;
        };
        if let Some(previous_index) = previous_index {
            if index <= previous_index {
                errors.push(format!("{path_label} drifted order around {needle:?}"));
                return;
            }
        }
        previous_index = Some(index);
    }
}

fn load_contract_sources(artifacts: &Path) -> ContractSources {
    let contract_artifacts = artifacts.join("contract");
    route_free::write_json_artifact(
        &artifacts.join("scenario-meta.json"),
        &json!({
            "verifier": VERIFIER_PATH,
            "source_contract": SOURCE_CONTRACT_PATH,
            "commands": [
                SOURCE_CONTRACT_COMMAND,
                RUST_CONTRACT_COMMAND,
                S02_REPLAY_COMMAND,
                GENERATE_OG_COMMAND,
                DOCS_BUILD_COMMAND,
            ],
            "expected_phase_markers": [
                "m054-s03-db-env-preflight",
                "m054-s03-source-contract",
                "m054-s03-rust-contract",
                "m054-s03-s02-replay",
                "m054-s03-generate-og",
                "m054-s03-build-docs",
                "m054-s03-built-html-assertions",
                "m054-s03-retain-s02-verify",
                "m054-s03-retain-source-and-logs",
                "m054-s03-retain-site-evidence",
                "m054-s03-retain-og-evidence",
                "m054-s03-redaction-drift",
                "m054-s03-bundle-shape",
            ],
            "expected_bundle_entries": [
                ".tmp/m054-s03/verify/status.txt",
                ".tmp/m054-s03/verify/current-phase.txt",
                ".tmp/m054-s03/verify/phase-report.txt",
                ".tmp/m054-s03/verify/full-contract.log",
                ".tmp/m054-s03/verify/latest-proof-bundle.txt",
                ".tmp/m054-s03/verify/built-html-summary.json",
                "retained-m054-s02-verify/status.txt",
                "retained-m054-s02-verify/current-phase.txt",
                "retained-m054-s02-verify/phase-report.txt",
                "retained-m054-s02-verify/full-contract.log",
                "retained-m054-s02-verify/latest-proof-bundle.txt",
                "retained-site/index.html",
                "retained-site/docs/distributed-proof/index.html",
                "retained-og-image-v2.png",
                "built-html-summary.json",
            ],
        }),
    );

    let verifier = route_free::read_and_archive(
        &repo_root().join(VERIFIER_PATH),
        &contract_artifacts.join("verify-m054-s03.sh"),
    );
    let source_contract = route_free::read_and_archive(
        &repo_root().join(SOURCE_CONTRACT_PATH),
        &contract_artifacts.join("verify-m054-s03-contract.test.mjs"),
    );
    let index = route_free::read_and_archive(
        &repo_root().join(HOMEPAGE_PATH),
        &contract_artifacts.join("website.docs.index.md"),
    );
    let config = route_free::read_and_archive(
        &repo_root().join(CONFIG_PATH),
        &contract_artifacts.join("website.docs._vitepress.config.mts"),
    );
    let proof = route_free::read_and_archive(
        &repo_root().join(PROOF_PATH),
        &contract_artifacts.join("website.docs.distributed-proof.index.md"),
    );
    let og = route_free::read_and_archive(
        &repo_root().join(OG_SOURCE_PATH),
        &contract_artifacts.join("website.scripts.generate-og-image.py"),
    );
    let _ = route_free::read_and_archive(
        &repo_root().join("compiler/meshc/tests/e2e_m054_s03.rs"),
        &contract_artifacts.join("e2e_m054_s03.rs"),
    );

    ContractSources {
        verifier,
        source_contract,
        index,
        config,
        proof,
        og,
    }
}

fn validate_public_docs_contract(sources: &ContractSources) -> Vec<String> {
    let mut errors = Vec::new();

    require_includes(
        &mut errors,
        HOMEPAGE_PATH,
        &sources.index,
        &[&format!("description: {HOMEPAGE_DESCRIPTION}")],
    );
    require_omits(
        &mut errors,
        HOMEPAGE_PATH,
        &sources.index,
        &[OLD_GENERIC_TAGLINE],
    );

    require_includes(
        &mut errors,
        CONFIG_PATH,
        &sources.config,
        &[HOMEPAGE_DESCRIPTION, CONFIG_ALT_MARKER],
    );
    require_omits(
        &mut errors,
        CONFIG_PATH,
        &sources.config,
        &[OLD_GENERIC_TAGLINE],
    );

    require_includes(
        &mut errors,
        PROOF_PATH,
        &sources.proof,
        &[
            PROOF_BOUNDARY_MARKER,
            PROOF_HEADER_MARKER,
            PROOF_LIST_MARKER,
            PROOF_WORKFLOW_MARKER,
            PROOF_NON_GOAL_MARKER,
            FLY_EVIDENCE_MARKER,
        ],
    );
    require_order(
        &mut errors,
        PROOF_PATH,
        &sources.proof,
        &[
            PROOF_BOUNDARY_MARKER,
            PROOF_HEADER_MARKER,
            PROOF_WORKFLOW_MARKER,
            PROOF_NON_GOAL_MARKER,
        ],
    );

    require_includes(
        &mut errors,
        OG_SOURCE_PATH,
        &sources.og,
        &[OG_SUBTITLE_MARKER, OG_BADGE_MARKER],
    );
    require_omits(
        &mut errors,
        OG_SOURCE_PATH,
        &sources.og,
        &[OLD_GENERIC_TAGLINE],
    );

    require_includes(
        &mut errors,
        SOURCE_CONTRACT_PATH,
        &sources.source_contract,
        &[
            HOMEPAGE_DESCRIPTION,
            CONFIG_ALT_MARKER,
            PROOF_BOUNDARY_MARKER,
            PROOF_HEADER_MARKER,
            PROOF_LIST_MARKER,
            PROOF_WORKFLOW_MARKER,
            PROOF_NON_GOAL_MARKER,
            FLY_EVIDENCE_MARKER,
            OG_SUBTITLE_MARKER,
            OG_BADGE_MARKER,
            OLD_GENERIC_TAGLINE,
        ],
    );

    errors
}

fn validate_verifier_contract(sources: &ContractSources) -> Vec<String> {
    let mut errors = Vec::new();
    let verifier = &sources.verifier;

    require_includes(
        &mut errors,
        VERIFIER_PATH,
        verifier,
        &[
            "ARTIFACT_ROOT=\".tmp/m054-s03\"",
            "PHASE_REPORT_PATH=\"$ARTIFACT_DIR/phase-report.txt\"",
            "STATUS_PATH=\"$ARTIFACT_DIR/status.txt\"",
            "CURRENT_PHASE_PATH=\"$ARTIFACT_DIR/current-phase.txt\"",
            "LATEST_PROOF_BUNDLE_PATH=\"$ARTIFACT_DIR/latest-proof-bundle.txt\"",
            "BUILT_HTML_SUMMARY_PATH=\"$ARTIFACT_DIR/built-html-summary.json\"",
            "DATABASE_URL must be set for scripts/verify-m054-s03.sh",
            "assert_test_filter_ran",
            SOURCE_CONTRACT_COMMAND,
            RUST_CONTRACT_COMMAND,
            S02_REPLAY_COMMAND,
            GENERATE_OG_COMMAND,
            DOCS_BUILD_COMMAND,
            "printf '%s\\n' \"$RETAINED_PROOF_BUNDLE_DIR\" >\"$LATEST_PROOF_BUNDLE_PATH\"",
            "retained-m054-s02-verify",
            "retained-site/index.html",
            "retained-site/docs/distributed-proof/index.html",
            "retained-og-image-v2.png",
            "built-html-summary.json",
            "source-contract.log",
            "rust-contract.log",
            "generate-og.log",
            "build-docs.log",
            "built-html-assertions.log",
            "assert_no_secret_leaks",
            "assert_retained_bundle_shape",
            "m054-s03-db-env-preflight",
            "m054-s03-source-contract",
            "m054-s03-rust-contract",
            "m054-s03-s02-replay",
            "m054-s03-generate-og",
            "m054-s03-build-docs",
            "m054-s03-built-html-assertions",
            "m054-s03-retain-s02-verify",
            "m054-s03-retain-source-and-logs",
            "m054-s03-retain-site-evidence",
            "m054-s03-retain-og-evidence",
            "m054-s03-redaction-drift",
            "m054-s03-bundle-shape",
            HOMEPAGE_DESCRIPTION,
            PROOF_BOUNDARY_MARKER,
            "X-Mesh-Continuity-Request-Key",
            PROOF_HEADER_HTML_MARKER,
            PROOF_LIST_HTML_MARKER,
            PROOF_NON_GOAL_MARKER,
            OLD_GENERIC_TAGLINE,
        ],
    );

    require_omits(
        &mut errors,
        VERIFIER_PATH,
        verifier,
        &[
            "source \"$ROOT_DIR/.env\"",
            "echo \"$DATABASE_URL\"",
            "printf '%s\\n' \"$DATABASE_URL\"",
        ],
    );

    require_order(
        &mut errors,
        VERIFIER_PATH,
        verifier,
        &[
            SOURCE_CONTRACT_COMMAND,
            RUST_CONTRACT_COMMAND,
            S02_REPLAY_COMMAND,
            GENERATE_OG_COMMAND,
            DOCS_BUILD_COMMAND,
        ],
    );

    require_order(
        &mut errors,
        VERIFIER_PATH,
        verifier,
        &[
            "run_expect_success m054-s03-source-contract",
            "run_expect_success m054-s03-rust-contract",
            "run_expect_success_with_database_url m054-s03-s02-replay",
            "run_expect_success m054-s03-generate-og",
            "run_expect_success m054-s03-build-docs",
            "record_phase m054-s03-built-html-assertions started",
            "record_phase m054-s03-retain-s02-verify started",
            "record_phase m054-s03-retain-source-and-logs started",
            "record_phase m054-s03-retain-site-evidence started",
            "record_phase m054-s03-retain-og-evidence started",
            "record_phase m054-s03-redaction-drift started",
            "record_phase m054-s03-bundle-shape started",
        ],
    );

    require_includes(
        &mut errors,
        SOURCE_CONTRACT_PATH,
        &sources.source_contract,
        &[
            HOMEPAGE_DESCRIPTION,
            PROOF_BOUNDARY_MARKER,
            PROOF_HEADER_MARKER,
            PROOF_WORKFLOW_MARKER,
            OG_SUBTITLE_MARKER,
        ],
    );

    errors
}

#[test]
fn m054_s03_public_docs_sources_keep_bounded_one_public_url_story() {
    let artifacts = artifact_dir("public-docs-source-contract");
    let sources = load_contract_sources(&artifacts);
    let errors = validate_public_docs_contract(&sources);
    assert!(errors.is_empty(), "{}", errors.join("\n"));
}

#[test]
fn m054_s03_verifier_replays_s02_and_retains_built_html_and_og_evidence() {
    let artifacts = artifact_dir("verifier-contract");
    let sources = load_contract_sources(&artifacts);
    let errors = validate_verifier_contract(&sources);
    assert!(errors.is_empty(), "{}", errors.join("\n"));
}

#[test]
fn m054_s03_verifier_contract_fails_closed_when_rust_phase_drifts() {
    let artifacts = artifact_dir("rust-phase-drift");
    let sources = load_contract_sources(&artifacts);

    let mutated = sources
        .verifier
        .replace(
            RUST_CONTRACT_COMMAND,
            "cargo test -p meshc --test e2e_m054_s02 -- --nocapture",
        )
        .replace("m054-s03-rust-contract", "m054-s03-script-contract");

    let mutated_sources = ContractSources {
        verifier: mutated,
        ..sources
    };
    let errors = validate_verifier_contract(&mutated_sources);
    assert!(
        errors.iter().any(|error| {
            error.contains(RUST_CONTRACT_COMMAND)
                || error.contains("m054-s03-rust-contract")
                || error.contains("drifted order")
        }),
        "{}",
        errors.join("\n")
    );
}

#[test]
fn m054_s03_verifier_contract_fails_closed_when_summary_bundle_or_redaction_markers_disappear() {
    let artifacts = artifact_dir("summary-bundle-drift");
    let sources = load_contract_sources(&artifacts);

    let mutated = sources
        .verifier
        .replace(
            "BUILT_HTML_SUMMARY_PATH=\"$ARTIFACT_DIR/built-html-summary.json\"",
            "BUILT_HTML_SUMMARY_PATH=\"$ARTIFACT_DIR/built-html.txt\"",
        )
        .replace("built-html-summary.json", "built-html.txt")
        .replace("retained-og-image-v2.png", "retained-og.png")
        .replace("latest-proof-bundle.txt", "latest-proof.json")
        .replace("assert_no_secret_leaks", "echo \"$DATABASE_URL\"");

    let mutated_sources = ContractSources {
        verifier: mutated,
        ..sources
    };
    let errors = validate_verifier_contract(&mutated_sources);
    assert!(
        errors.iter().any(|error| {
            error.contains("built-html-summary.json")
                || error.contains("retained-og-image-v2.png")
                || error.contains("latest-proof-bundle.txt")
                || error.contains("echo \"$DATABASE_URL\"")
        }),
        "{}",
        errors.join("\n")
    );
}
