---
id: T02
parent: S02
milestone: M043
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-rt/src/dist/continuity.rs", "compiler/mesh-rt/src/lib.rs", "compiler/mesh-typeck/src/infer.rs", "compiler/mesh-codegen/src/mir/lower.rs", "compiler/mesh-codegen/src/codegen/intrinsics.rs", "compiler/meshc/tests/e2e_m043_s02.rs", ".gsd/milestones/M043/slices/S02/tasks/T02-SUMMARY.md"]
key_decisions: ["D177: Expose `Continuity.promote()` and `Continuity.authority_status()` as `Result<String, String>` built-ins with JSON payloads carrying `cluster_role`, `promotion_epoch`, and aggregate `replication_health`.", "Keep the Mesh-visible continuity failover surface on the existing JSON-over-Result seam instead of widening the compiler/runtime ABI for T02."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "The task gate `cargo test -p meshc --test e2e_m043_s02 continuity_api_ -- --nocapture` passed with 4/4 `continuity_api_` tests green after formatting, covering authority-status readback, standby promotion to epoch 1, repeated-promotion rejection, primary-side promotion rejection, and negative compile failures for wrong arity and wrong result shape. An extra runtime regression sweep with `cargo test -p mesh-rt continuity -- --nocapture` also passed with 33 continuity tests green."
completed_at: 2026-03-29T08:29:34.746Z
blocker_discovered: false
---

# T02: Added runtime-backed Continuity promotion and authority-status APIs with compiler e2e coverage.

> Added runtime-backed Continuity promotion and authority-status APIs with compiler e2e coverage.

## What Happened
---
id: T02
parent: S02
milestone: M043
key_files:
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-rt/src/lib.rs
  - compiler/mesh-typeck/src/infer.rs
  - compiler/mesh-codegen/src/mir/lower.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/meshc/tests/e2e_m043_s02.rs
  - .gsd/milestones/M043/slices/S02/tasks/T02-SUMMARY.md
key_decisions:
  - D177: Expose `Continuity.promote()` and `Continuity.authority_status()` as `Result<String, String>` built-ins with JSON payloads carrying `cluster_role`, `promotion_epoch`, and aggregate `replication_health`.
  - Keep the Mesh-visible continuity failover surface on the existing JSON-over-Result seam instead of widening the compiler/runtime ABI for T02.
duration: ""
verification_result: passed
completed_at: 2026-03-29T08:29:34.748Z
blocker_discovered: false
---

# T02: Added runtime-backed Continuity promotion and authority-status APIs with compiler e2e coverage.

**Added runtime-backed Continuity promotion and authority-status APIs with compiler e2e coverage.**

## What Happened

Added runtime-backed `Continuity.promote()` and `Continuity.authority_status()` entrypoints in `mesh-rt`, re-exported them through the runtime library, and wired the new built-ins through the typechecker and codegen intrinsic seam. The runtime now exposes a narrow `ContinuityAuthorityStatus` JSON contract that reports live `cluster_role`, `promotion_epoch`, and aggregate `replication_health` from the continuity registry instead of forcing Mesh consumers to infer authority state from env. I also created `compiler/meshc/tests/e2e_m043_s02.rs` with focused `continuity_api_` coverage that compiles standalone Mesh programs, exercises successful standby promotion and rejection paths, and proves wrong-arity and wrong result-shape calls fail closed at compile time.

## Verification

The task gate `cargo test -p meshc --test e2e_m043_s02 continuity_api_ -- --nocapture` passed with 4/4 `continuity_api_` tests green after formatting, covering authority-status readback, standby promotion to epoch 1, repeated-promotion rejection, primary-side promotion rejection, and negative compile failures for wrong arity and wrong result shape. An extra runtime regression sweep with `cargo test -p mesh-rt continuity -- --nocapture` also passed with 33 continuity tests green.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m043_s02 continuity_api_ -- --nocapture` | 0 | ✅ pass | 55325ms |
| 2 | `cargo test -p mesh-rt continuity -- --nocapture` | 0 | ✅ pass | 26424ms |


## Deviations

None.

## Known Issues

Pre-existing `mesh-rt` test warnings remain in `compiler/mesh-rt/src/actor/scheduler.rs`, `compiler/mesh-rt/src/actor/service.rs`, and `compiler/mesh-rt/src/iter.rs`. They did not affect the continuity task gate or the runtime regression sweep.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/lib.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/meshc/tests/e2e_m043_s02.rs`
- `.gsd/milestones/M043/slices/S02/tasks/T02-SUMMARY.md`


## Deviations
None.

## Known Issues
Pre-existing `mesh-rt` test warnings remain in `compiler/mesh-rt/src/actor/scheduler.rs`, `compiler/mesh-rt/src/actor/service.rs`, and `compiler/mesh-rt/src/iter.rs`. They did not affect the continuity task gate or the runtime regression sweep.
