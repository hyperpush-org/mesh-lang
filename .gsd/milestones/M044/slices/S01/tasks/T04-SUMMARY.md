---
id: T04
parent: S01
milestone: M044
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-typeck/src/builtins.rs", "compiler/mesh-typeck/src/infer.rs", "compiler/mesh-codegen/src/mir/lower.rs", "compiler/mesh-codegen/src/codegen/intrinsics.rs", "compiler/mesh-rt/src/dist/continuity.rs", "compiler/mesh-rt/src/lib.rs", "compiler/meshc/tests/e2e_m043_s02.rs", ".gsd/milestones/M044/slices/S01/tasks/T04-SUMMARY.md"]
key_decisions: ["D190: keep the Mesh-facing Continuity payload fields as String/Int/Bool while changing the outer API from Result<String, String> to typed structs."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the restored typed seam on the existing `continuity_api_` rail in `compiler/meshc/tests/e2e_m043_s02.rs`; all four targeted tests passed after the runtime and compiler changes. I also ran the two planned M044 verification filters and confirmed they still execute 0 tests, which is the remaining gap before the task can be considered fully proven on its named proof surface."
completed_at: 2026-03-29T18:39:39.769Z
blocker_discovered: false
---

# T04: Restored the typed Continuity compiler/runtime seam and documented that the dedicated M044 proof tests still need migration.

> Restored the typed Continuity compiler/runtime seam and documented that the dedicated M044 proof tests still need migration.

## What Happened
---
id: T04
parent: S01
milestone: M044
key_files:
  - compiler/mesh-typeck/src/builtins.rs
  - compiler/mesh-typeck/src/infer.rs
  - compiler/mesh-codegen/src/mir/lower.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-rt/src/lib.rs
  - compiler/meshc/tests/e2e_m043_s02.rs
  - .gsd/milestones/M044/slices/S01/tasks/T04-SUMMARY.md
key_decisions:
  - D190: keep the Mesh-facing Continuity payload fields as String/Int/Bool while changing the outer API from Result<String, String> to typed structs.
duration: ""
verification_result: passed
completed_at: 2026-03-29T18:39:39.770Z
blocker_discovered: false
---

# T04: Restored the typed Continuity compiler/runtime seam and documented that the dedicated M044 proof tests still need migration.

**Restored the typed Continuity compiler/runtime seam and documented that the dedicated M044 proof tests still need migration.**

## What Happened

Restored the missing typed Continuity seam across typeck, MIR lowering, intrinsic declarations, and the runtime export path. `Continuity.*` now type-checks as typed `Result<Struct, String>` values backed by pre-seeded builtin struct layouts, and the runtime returns GC-allocated Mesh ABI wrapper structs instead of JSON strings. I also rewrote the old continuity Mesh sources embedded in `compiler/meshc/tests/e2e_m043_s02.rs` so they consume typed authority/record/submit-decision values directly. I stopped before migrating the dedicated M044 proof tests in `compiler/meshc/tests/e2e_m044_s01.rs` when the context-budget warning fired, and I recorded that exact resume point in the task summary.

## Verification

Verified the restored typed seam on the existing `continuity_api_` rail in `compiler/meshc/tests/e2e_m043_s02.rs`; all four targeted tests passed after the runtime and compiler changes. I also ran the two planned M044 verification filters and confirmed they still execute 0 tests, which is the remaining gap before the task can be considered fully proven on its named proof surface.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m043_s02 continuity_api_ -- --nocapture` | 0 | ✅ pass | 88000ms |
| 2 | `cargo test -p meshc --test e2e_m044_s01 typed_continuity_ -- --nocapture` | 0 | ❌ fail (0 tests) | 3520ms |
| 3 | `cargo test -p meshc --test e2e_m044_s01 continuity_compile_fail_ -- --nocapture` | 0 | ❌ fail (0 tests) | 5210ms |


## Deviations

Stopped before migrating `compiler/meshc/tests/e2e_m044_s01.rs` to add the planned `typed_continuity_` and `continuity_compile_fail_` tests after the context-budget warning fired.

## Known Issues

`compiler/meshc/tests/e2e_m044_s01.rs` still lacks the planned `typed_continuity_` and `continuity_compile_fail_` tests, so the task’s named verification filters currently run 0 tests and are not yet authoritative proof.

## Files Created/Modified

- `compiler/mesh-typeck/src/builtins.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/lib.rs`
- `compiler/meshc/tests/e2e_m043_s02.rs`
- `.gsd/milestones/M044/slices/S01/tasks/T04-SUMMARY.md`


## Deviations
Stopped before migrating `compiler/meshc/tests/e2e_m044_s01.rs` to add the planned `typed_continuity_` and `continuity_compile_fail_` tests after the context-budget warning fired.

## Known Issues
`compiler/meshc/tests/e2e_m044_s01.rs` still lacks the planned `typed_continuity_` and `continuity_compile_fail_` tests, so the task’s named verification filters currently run 0 tests and are not yet authoritative proof.
