#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

source scripts/lib/clustered_fixture_paths.sh
clustered_fixture_require_cluster_proof_root

cargo test -p mesh-rt continuity -- --nocapture
cargo run -q -p meshc -- test "$CLUSTER_PROOF_FIXTURE_TESTS"
cargo test -p meshc --test e2e_m042_s01 continuity_api -- --nocapture
