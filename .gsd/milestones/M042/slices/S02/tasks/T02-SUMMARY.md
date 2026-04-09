---
id: T02
parent: S02
milestone: M042
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-typeck/src/infer.rs", "compiler/mesh-codegen/src/codegen/intrinsics.rs", "compiler/mesh-codegen/src/mir/lower.rs", "cluster-proof/work.mpl", "cluster-proof/tests/work.test.mpl", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Lower compiler-emitted Continuity.submit calls to mesh_continuity_submit_with_durability while leaving mesh_continuity_submit as the runtime compatibility wrapper.", "Keep /work submit outcome mapping in pure Work helpers so rejected/duplicate/conflict truth is unit-testable without the live cluster harness."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran `cargo run -q -p meshc -- test cluster-proof/tests` and got a full green pass: 8 config tests plus 11 work tests, including the new submit-response helper coverage for rejected, duplicate, and conflict truth."
completed_at: 2026-03-28T23:10:24.067Z
blocker_discovered: false
---

# T02: Plumbed durability-aware Continuity.submit through the compiler and made cluster-proof replay runtime-owned rejected and duplicate truth without app-authored replica acknowledgements.

> Plumbed durability-aware Continuity.submit through the compiler and made cluster-proof replay runtime-owned rejected and duplicate truth without app-authored replica acknowledgements.

## What Happened
---
id: T02
parent: S02
milestone: M042
key_files:
  - compiler/mesh-typeck/src/infer.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/mesh-codegen/src/mir/lower.rs
  - cluster-proof/work.mpl
  - cluster-proof/tests/work.test.mpl
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Lower compiler-emitted Continuity.submit calls to mesh_continuity_submit_with_durability while leaving mesh_continuity_submit as the runtime compatibility wrapper.
  - Keep /work submit outcome mapping in pure Work helpers so rejected/duplicate/conflict truth is unit-testable without the live cluster harness.
duration: ""
verification_result: passed
completed_at: 2026-03-28T23:10:24.069Z
blocker_discovered: false
---

# T02: Plumbed durability-aware Continuity.submit through the compiler and made cluster-proof replay runtime-owned rejected and duplicate truth without app-authored replica acknowledgements.

**Plumbed durability-aware Continuity.submit through the compiler and made cluster-proof replay runtime-owned rejected and duplicate truth without app-authored replica acknowledgements.**

## What Happened

Updated the compiler/runtime seam so module-qualified `Continuity.submit(...)` now carries the required replica count and lowers to `mesh_continuity_submit_with_durability(...)`. In `cluster-proof/work.mpl`, the keyed `/work` path now passes `current_required_replica_count()` into runtime submit, removes the old `acknowledge_replica` mirrored-truth shim, returns stored rejected admissions as `503`, replays duplicate rejected records as `503`, preserves accepted duplicate replay as `200`, and keeps the existing `409` conflict contract. Added helper-level tests in `cluster-proof/tests/work.test.mpl` for accepted dispatch, rejected admission, duplicate rejected replay, duplicate accepted replay, and conflict mapping, and recorded the helper-local `&&` LLVM verifier gotcha in `.gsd/KNOWLEDGE.md`.

## Verification

Ran `cargo run -q -p meshc -- test cluster-proof/tests` and got a full green pass: 8 config tests plus 11 work tests, including the new submit-response helper coverage for rejected, duplicate, and conflict truth.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 8949ms |


## Deviations

None.

## Known Issues

None in this task’s scope. The unstable remote-owner completion path remains outside T02 and is still handled by the later e2e/verifier task.

## Files Created/Modified

- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `cluster-proof/work.mpl`
- `cluster-proof/tests/work.test.mpl`
- `.gsd/KNOWLEDGE.md`


## Deviations
None.

## Known Issues
None in this task’s scope. The unstable remote-owner completion path remains outside T02 and is still handled by the later e2e/verifier task.
