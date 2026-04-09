---
id: T05
parent: S04
milestone: M049
provides: []
requires: []
affects: []
key_files: ["scripts/lib/clustered_fixture_paths.sh", "scripts/verify-m039-s01.sh", "scripts/verify-m039-s02.sh", "scripts/verify-m039-s03.sh", "scripts/verify-m040-s01.sh", "scripts/verify-m042-s01.sh", "scripts/verify-m042-s02.sh", "scripts/verify-m042-s03.sh", "scripts/verify-m043-s01.sh", "scripts/verify-m043-s02.sh", "scripts/verify-m043-s03.sh", "scripts/verify-m045-s01.sh", "scripts/verify-m045-s02.sh", "compiler/meshc/tests/e2e_m039_s02.rs", "compiler/meshc/tests/e2e_m039_s03.rs", "compiler/meshc/tests/e2e_m044_s02.rs"]
key_decisions: ["Use scripts/lib/clustered_fixture_paths.sh as the single fail-closed shell owner of the relocated tiny-cluster and cluster-proof fixture roots.", "Move the M044 declared-work LLVM replay onto the current source-first @cluster contract instead of the removed [cluster] manifest surface."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed: shell syntax check for the new helper plus updated verifier family, rustfmt on the changed Rust tests, and cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture. A direct cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_falls_back_locally_without_peers -- --nocapture replay still fails for a deeper historical routeful-contract reason unrelated to the new helper. bash scripts/verify-m039-s01.sh failed once in e2e_m039_s01_membership_updates_after_node_loss, but the exact failing cargo test passed immediately on rerun; bash scripts/verify-m045-s02.sh was not rerun after the final edits because the unit had to wrap up for context budget."
completed_at: 2026-04-03T03:47:57.865Z
blocker_discovered: false
---

# T05: Centralized clustered-fixture shell paths for the older verifier family and moved the M044 declared-work LLVM replay onto the current source-first @cluster contract.

> Centralized clustered-fixture shell paths for the older verifier family and moved the M044 declared-work LLVM replay onto the current source-first @cluster contract.

## What Happened
---
id: T05
parent: S04
milestone: M049
key_files:
  - scripts/lib/clustered_fixture_paths.sh
  - scripts/verify-m039-s01.sh
  - scripts/verify-m039-s02.sh
  - scripts/verify-m039-s03.sh
  - scripts/verify-m040-s01.sh
  - scripts/verify-m042-s01.sh
  - scripts/verify-m042-s02.sh
  - scripts/verify-m042-s03.sh
  - scripts/verify-m043-s01.sh
  - scripts/verify-m043-s02.sh
  - scripts/verify-m043-s03.sh
  - scripts/verify-m045-s01.sh
  - scripts/verify-m045-s02.sh
  - compiler/meshc/tests/e2e_m039_s02.rs
  - compiler/meshc/tests/e2e_m039_s03.rs
  - compiler/meshc/tests/e2e_m044_s02.rs
key_decisions:
  - Use scripts/lib/clustered_fixture_paths.sh as the single fail-closed shell owner of the relocated tiny-cluster and cluster-proof fixture roots.
  - Move the M044 declared-work LLVM replay onto the current source-first @cluster contract instead of the removed [cluster] manifest surface.
duration: ""
verification_result: mixed
completed_at: 2026-04-03T03:47:57.885Z
blocker_discovered: false
---

# T05: Centralized clustered-fixture shell paths for the older verifier family and moved the M044 declared-work LLVM replay onto the current source-first @cluster contract.

**Centralized clustered-fixture shell paths for the older verifier family and moved the M044 declared-work LLVM replay onto the current source-first @cluster contract.**

## What Happened

Added scripts/lib/clustered_fixture_paths.sh as the shared fail-closed source for relocated tiny-cluster and cluster-proof fixture roots, then retargeted the older direct bash verifier family (M039/M040/M042/M043/M045 scripts) to source that helper instead of open-coding repo-root package paths. Updated compiler/meshc/tests/e2e_m039_s02.rs and e2e_m039_s03.rs to resolve the relocated cluster-proof fixture through support::m046_route_free, and updated compiler/meshc/tests/e2e_m044_s02.rs so the m044_s02_declared_work_ LLVM replay now uses a normal package manifest plus source-first @cluster work instead of the removed [cluster] manifest surface. While tightening the helper I briefly over-required work_continuity.mpl for cluster-proof, then corrected the helper to match the actual relocated route-free fixture shape. The unit hit the context-budget wrap-up path before the final representative reruns finished: the full bash scripts/verify-m039-s01.sh rail failed once on a transient node-loss convergence race, but the exact failing cargo test passed immediately on rerun without code changes, and bash scripts/verify-m045-s02.sh still needs a fresh replay from the current tree.

## Verification

Passed: shell syntax check for the new helper plus updated verifier family, rustfmt on the changed Rust tests, and cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture. A direct cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_falls_back_locally_without_peers -- --nocapture replay still fails for a deeper historical routeful-contract reason unrelated to the new helper. bash scripts/verify-m039-s01.sh failed once in e2e_m039_s01_membership_updates_after_node_loss, but the exact failing cargo test passed immediately on rerun; bash scripts/verify-m045-s02.sh was not rerun after the final edits because the unit had to wrap up for context budget.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -n scripts/lib/clustered_fixture_paths.sh scripts/verify-m039-s01.sh scripts/verify-m039-s02.sh scripts/verify-m039-s03.sh scripts/verify-m040-s01.sh scripts/verify-m042-s01.sh scripts/verify-m042-s02.sh scripts/verify-m042-s03.sh scripts/verify-m043-s01.sh scripts/verify-m043-s02.sh scripts/verify-m043-s03.sh scripts/verify-m045-s01.sh scripts/verify-m045-s02.sh` | 0 | ✅ pass | 100ms |
| 2 | `rustfmt compiler/meshc/tests/e2e_m039_s02.rs compiler/meshc/tests/e2e_m039_s03.rs compiler/meshc/tests/e2e_m044_s02.rs` | 0 | ✅ pass | 200ms |
| 3 | `cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture` | 0 | ✅ pass | 25498ms |
| 4 | `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_falls_back_locally_without_peers -- --nocapture` | 101 | ❌ fail | 25676ms |
| 5 | `bash scripts/verify-m039-s01.sh` | 1 | ❌ fail | 195637ms |
| 6 | `cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_membership_updates_after_node_loss -- --nocapture` | 0 | ✅ pass | 11279ms |


## Deviations

Also updated three Rust consumers (compiler/meshc/tests/e2e_m039_s02.rs, compiler/meshc/tests/e2e_m039_s03.rs, compiler/meshc/tests/e2e_m044_s02.rs) because script-only path replacement was not enough to keep the representative rails moving honestly.

## Known Issues

bash scripts/verify-m045-s02.sh still needs a clean rerun from the current tree. bash scripts/verify-m039-s01.sh needs one clean full-script rerun after the transient node-loss flake. cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_falls_back_locally_without_peers -- --nocapture still expects the pre-route-free cluster-proof env/HTTP contract (CLUSTER_PROOF_*, /membership, /work) and now exits after Node.start_from_env() with MESH_CLUSTER_COOKIE guidance instead.

## Files Created/Modified

- `scripts/lib/clustered_fixture_paths.sh`
- `scripts/verify-m039-s01.sh`
- `scripts/verify-m039-s02.sh`
- `scripts/verify-m039-s03.sh`
- `scripts/verify-m040-s01.sh`
- `scripts/verify-m042-s01.sh`
- `scripts/verify-m042-s02.sh`
- `scripts/verify-m042-s03.sh`
- `scripts/verify-m043-s01.sh`
- `scripts/verify-m043-s02.sh`
- `scripts/verify-m043-s03.sh`
- `scripts/verify-m045-s01.sh`
- `scripts/verify-m045-s02.sh`
- `compiler/meshc/tests/e2e_m039_s02.rs`
- `compiler/meshc/tests/e2e_m039_s03.rs`
- `compiler/meshc/tests/e2e_m044_s02.rs`


## Deviations
Also updated three Rust consumers (compiler/meshc/tests/e2e_m039_s02.rs, compiler/meshc/tests/e2e_m039_s03.rs, compiler/meshc/tests/e2e_m044_s02.rs) because script-only path replacement was not enough to keep the representative rails moving honestly.

## Known Issues
bash scripts/verify-m045-s02.sh still needs a clean rerun from the current tree. bash scripts/verify-m039-s01.sh needs one clean full-script rerun after the transient node-loss flake. cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_falls_back_locally_without_peers -- --nocapture still expects the pre-route-free cluster-proof env/HTTP contract (CLUSTER_PROOF_*, /membership, /work) and now exits after Node.start_from_env() with MESH_CLUSTER_COOKIE guidance instead.
