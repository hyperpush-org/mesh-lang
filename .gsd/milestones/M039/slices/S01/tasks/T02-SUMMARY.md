---
id: T02
parent: S01
milestone: M039
provides: []
requires: []
affects: []
key_files: ["cluster-proof/main.mpl", "cluster-proof/config.mpl", "cluster-proof/cluster.mpl", "cluster-proof/tests/config.test.mpl", "compiler/meshc/tests/e2e_m039_s01.rs", ".gsd/milestones/M039/slices/S01/tasks/T02-SUMMARY.md"]
key_decisions: ["Kept the proof surface as a new `cluster-proof/` app and derived endpoint membership from `Node.self()` plus `Node.list()` instead of discovery candidates.", "Added Mesh-side config tests so malformed identity and missing-cookie cases fail on pure helpers before the live harness runs."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Confirmed that `cluster-proof/` can build on an earlier revision once `mesh-rt` is present and that the pure config tests pass with `cargo run -q -p meshc -- test cluster-proof/tests`. The live convergence proof is still red: `cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_converges_without_manual_peers -- --nocapture` failed because the proof app never served `/membership`, and direct launches of `./cluster-proof/cluster-proof` crashed with exit 139 before any logs. After the startup-path refactor, the next build hit an LLVM verifier failure in the config helpers; the final rewrite after that failure has not been rerun yet."
completed_at: 2026-03-28T09:39:56.717Z
blocker_discovered: false
---

# T02: Built the `cluster-proof/` app skeleton, config tests, and a live convergence harness, but the proof app still needs one more pass because startup/compiler failures are not fully retired.

> Built the `cluster-proof/` app skeleton, config tests, and a live convergence harness, but the proof app still needs one more pass because startup/compiler failures are not fully retired.

## What Happened
---
id: T02
parent: S01
milestone: M039
key_files:
  - cluster-proof/main.mpl
  - cluster-proof/config.mpl
  - cluster-proof/cluster.mpl
  - cluster-proof/tests/config.test.mpl
  - compiler/meshc/tests/e2e_m039_s01.rs
  - .gsd/milestones/M039/slices/S01/tasks/T02-SUMMARY.md
key_decisions:
  - Kept the proof surface as a new `cluster-proof/` app and derived endpoint membership from `Node.self()` plus `Node.list()` instead of discovery candidates.
  - Added Mesh-side config tests so malformed identity and missing-cookie cases fail on pure helpers before the live harness runs.
duration: ""
verification_result: mixed
completed_at: 2026-03-28T09:39:56.718Z
blocker_discovered: false
---

# T02: Built the `cluster-proof/` app skeleton, config tests, and a live convergence harness, but the proof app still needs one more pass because startup/compiler failures are not fully retired.

**Built the `cluster-proof/` app skeleton, config tests, and a live convergence harness, but the proof app still needs one more pass because startup/compiler failures are not fully retired.**

## What Happened

Created the new narrow proof surface under `cluster-proof/` rather than extending Mesher again. `cluster-proof/config.mpl` now owns the env contract, explicit/Fly identity composition, and config-error messages; `cluster-proof/cluster.mpl` shapes the membership payload from `Node.self()` plus `Node.list()` and explicitly includes `self`; `cluster-proof/main.mpl` wires the single read-only `/membership` endpoint and intended startup logging; `cluster-proof/tests/config.test.mpl` covers malformed config cases; and `compiler/meshc/tests/e2e_m039_s01.rs` now contains a real two-node convergence harness with durable per-node logs under `.tmp/m039-s01/`. The live proof is not green yet. The first convergence run showed the spawned proof app never bound HTTP and produced empty logs. Direct binary repro showed an immediate segmentation fault even in standalone mode. I refactored the startup path away from `Result<Int, String>` matching because this repo already has boxed-int/result brittleness on compiled Mesh paths, then hit an LLVM verifier failure from boolean lowering in the config helpers. I flattened that logic and finished the pending `cluster-proof/cluster.mpl` write, but I did not get to rerun the build after the final rewrite before the context-budget stop.

## Verification

Confirmed that `cluster-proof/` can build on an earlier revision once `mesh-rt` is present and that the pure config tests pass with `cargo run -q -p meshc -- test cluster-proof/tests`. The live convergence proof is still red: `cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_converges_without_manual_peers -- --nocapture` failed because the proof app never served `/membership`, and direct launches of `./cluster-proof/cluster-proof` crashed with exit 139 before any logs. After the startup-path refactor, the next build hit an LLVM verifier failure in the config helpers; the final rewrite after that failure has not been rerun yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- build cluster-proof` | 1 | ❌ fail | 0ms |
| 2 | `cargo build -p mesh-rt` | 0 | ✅ pass | 7740ms |
| 3 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 0ms |
| 4 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 0ms |
| 5 | `cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_converges_without_manual_peers -- --nocapture` | 101 | ❌ fail | 17980ms |
| 6 | `PORT=18081 MESH_CLUSTER_PORT=43711 CLUSTER_PROOF_COOKIE=testcookie MESH_DISCOVERY_SEED=localhost CLUSTER_PROOF_NODE_BASENAME=node-a CLUSTER_PROOF_ADVERTISE_HOST=127.0.0.1 ./cluster-proof/cluster-proof` | 139 | ❌ fail | 0ms |
| 7 | `./cluster-proof/cluster-proof` | 139 | ❌ fail | 0ms |
| 8 | `cargo run -q -p meshc -- build cluster-proof` | 1 | ❌ fail | 0ms |


## Deviations

Added `cluster-proof/tests/config.test.mpl` even though the Expected Output list only named the app files and Rust harness, because the task plan explicitly required malformed-input and config-error coverage and the pure helper tests were the smallest honest way to get that proof.

## Known Issues

The current `cluster-proof/` source tree is not fully reverified after the last context-budget rewrite. The last known live runtime failure was an immediate segfault before startup logs, and the last known compiler failure after the startup-path refactor was an LLVM verifier error in the config helpers. `e2e_m039_s01_membership_updates_after_node_loss` is still the deliberate T03 placeholder.

## Files Created/Modified

- `cluster-proof/main.mpl`
- `cluster-proof/config.mpl`
- `cluster-proof/cluster.mpl`
- `cluster-proof/tests/config.test.mpl`
- `compiler/meshc/tests/e2e_m039_s01.rs`
- `.gsd/milestones/M039/slices/S01/tasks/T02-SUMMARY.md`


## Deviations
Added `cluster-proof/tests/config.test.mpl` even though the Expected Output list only named the app files and Rust harness, because the task plan explicitly required malformed-input and config-error coverage and the pure helper tests were the smallest honest way to get that proof.

## Known Issues
The current `cluster-proof/` source tree is not fully reverified after the last context-budget rewrite. The last known live runtime failure was an immediate segfault before startup logs, and the last known compiler failure after the startup-path refactor was an LLVM verifier error in the config helpers. `e2e_m039_s01_membership_updates_after_node_loss` is still the deliberate T03 placeholder.
