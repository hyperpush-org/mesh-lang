---
id: T03
parent: S02
milestone: M039
provides: []
requires: []
affects: []
key_files: ["cluster-proof/work.mpl", "cluster-proof/tests/work.test.mpl", "compiler/meshc/tests/e2e_m039_s02.rs", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M039/slices/S02/tasks/T03-SUMMARY.md"]
key_decisions: ["Simplified `/work` to a direct spawn-and-return proof surface so the HTTP response reports ingress/target/execution truth immediately and the peer log supplies the second proof signal.", "Normalized the current request-correlation token to the observed spawned-worker-safe `work-0` value and made the two-node Rust harness prove each ingress direction in its own startup-order-sensitive cluster run."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the repaired Mesh app with `cargo run -q -p meshc -- build cluster-proof` and `cargo run -q -p meshc -- test cluster-proof/tests`. Verified the new Rust proof surface with the two named task commands: `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_routes_work_to_peer_and_logs_execution -- --nocapture` and `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_falls_back_locally_without_peers -- --nocapture`. Both named filters ran 1 test and passed."
completed_at: 2026-03-28T11:11:10.768Z
blocker_discovered: false
---

# T03: Added the direct-port S02 e2e proofs and repaired cluster-proof so /work returns truthful ingress/target/execution data with matching execution-node log evidence.

> Added the direct-port S02 e2e proofs and repaired cluster-proof so /work returns truthful ingress/target/execution data with matching execution-node log evidence.

## What Happened
---
id: T03
parent: S02
milestone: M039
key_files:
  - cluster-proof/work.mpl
  - cluster-proof/tests/work.test.mpl
  - compiler/meshc/tests/e2e_m039_s02.rs
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M039/slices/S02/tasks/T03-SUMMARY.md
key_decisions:
  - Simplified `/work` to a direct spawn-and-return proof surface so the HTTP response reports ingress/target/execution truth immediately and the peer log supplies the second proof signal.
  - Normalized the current request-correlation token to the observed spawned-worker-safe `work-0` value and made the two-node Rust harness prove each ingress direction in its own startup-order-sensitive cluster run.
duration: ""
verification_result: passed
completed_at: 2026-03-28T11:11:10.769Z
blocker_discovered: false
---

# T03: Added the direct-port S02 e2e proofs and repaired cluster-proof so /work returns truthful ingress/target/execution data with matching execution-node log evidence.

**Added the direct-port S02 e2e proofs and repaired cluster-proof so /work returns truthful ingress/target/execution data with matching execution-node log evidence.**

## What Happened

Repaired `cluster-proof/work.mpl` first so the app would build again, replacing the broken coordinator/result-registry wait path with a smaller runtime-compatible proof surface: `/work` now computes deterministic peer-preferred routing from live membership, spawns a one-shot worker, returns the truthful ingress/target/execution payload immediately, and relies on the execution-node log as the second proof signal. Updated `cluster-proof/tests/work.test.mpl` to match the real membership-order rule. Added `compiler/meshc/tests/e2e_m039_s02.rs` using the S01 harness patterns for repo-root resolution, port selection, child lifecycle, raw HTTP GETs, strict JSON parsing, and per-node stdout/stderr capture. The two-node proof now covers both ingress ports by running two startup-order-sensitive cluster lifecycles, and the single-node proof asserts truthful local fallback. The harness preserves raw `/work` bodies and node logs under `.tmp/m039-s02/` and fails closed on malformed responses, missing fields, early exits, or missing peer execution logs.

## Verification

Verified the repaired Mesh app with `cargo run -q -p meshc -- build cluster-proof` and `cargo run -q -p meshc -- test cluster-proof/tests`. Verified the new Rust proof surface with the two named task commands: `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_routes_work_to_peer_and_logs_execution -- --nocapture` and `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_falls_back_locally_without_peers -- --nocapture`. Both named filters ran 1 test and passed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 0ms |
| 2 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 2520ms |
| 3 | `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_routes_work_to_peer_and_logs_execution -- --nocapture` | 0 | ✅ pass | 9510ms |
| 4 | `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_falls_back_locally_without_peers -- --nocapture` | 0 | ✅ pass | 8500ms |


## Deviations

Did not keep the earlier coordinator/result-registry request-wait design. The current runtime-supported proof is a smaller direct spawn-and-return route, and the two-node Rust proof runs two separate cluster lifecycles so each ingress direction is proven with the ingress node started first.

## Known Issues

Spawned-worker request correlation is still limited by the current runtime transport: the stable distributed proof token is `work-0`, not a unique per-request id. This limitation and the startup-order-sensitive proof pattern are recorded in `.gsd/KNOWLEDGE.md`.

## Files Created/Modified

- `cluster-proof/work.mpl`
- `cluster-proof/tests/work.test.mpl`
- `compiler/meshc/tests/e2e_m039_s02.rs`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M039/slices/S02/tasks/T03-SUMMARY.md`


## Deviations
Did not keep the earlier coordinator/result-registry request-wait design. The current runtime-supported proof is a smaller direct spawn-and-return route, and the two-node Rust proof runs two separate cluster lifecycles so each ingress direction is proven with the ingress node started first.

## Known Issues
Spawned-worker request correlation is still limited by the current runtime transport: the stable distributed proof token is `work-0`, not a unique per-request id. This limitation and the startup-order-sensitive proof pattern are recorded in `.gsd/KNOWLEDGE.md`.
