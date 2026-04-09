---
id: T02
parent: S05
milestone: M044
provides: []
requires: []
affects: []
key_files: ["cluster-proof/main.mpl", "cluster-proof/work.mpl", "cluster-proof/work_continuity.mpl", "cluster-proof/work_legacy.mpl", "cluster-proof/tests/work.test.mpl", "compiler/meshc/tests/e2e_m044_s05.rs", ".gsd/milestones/M044/slices/S05/tasks/T02-SUMMARY.md"]
key_decisions: ["Removed app-owned placement and dispatch from cluster-proof entirely; the proof app now relies on Continuity.submit_declared_work plus execute_declared_work as the only work path.", "Added fail-closed source-absence assertions to e2e_m044_s05 so WorkLegacy and the old helper seam cannot silently return."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the full task rail: `cargo run -q -p meshc -- build cluster-proof`, `cargo run -q -p meshc -- test cluster-proof/tests`, `cargo test -p meshc --test e2e_m044_s05 m044_s05_legacy_cleanup_ -- --nocapture`, and `test ! -e cluster-proof/work_legacy.mpl`. The live Rust rail also inspected combined logs and retained HTTP artifacts proving the missing legacy route and the absence of legacy dispatch logs."
completed_at: 2026-03-30T06:21:18.219Z
blocker_discovered: false
---

# T02: Removed cluster-proof’s legacy /work probe path and proved the keyed runtime-owned route is the only remaining work surface.

> Removed cluster-proof’s legacy /work probe path and proved the keyed runtime-owned route is the only remaining work surface.

## What Happened
---
id: T02
parent: S05
milestone: M044
key_files:
  - cluster-proof/main.mpl
  - cluster-proof/work.mpl
  - cluster-proof/work_continuity.mpl
  - cluster-proof/work_legacy.mpl
  - cluster-proof/tests/work.test.mpl
  - compiler/meshc/tests/e2e_m044_s05.rs
  - .gsd/milestones/M044/slices/S05/tasks/T02-SUMMARY.md
key_decisions:
  - Removed app-owned placement and dispatch from cluster-proof entirely; the proof app now relies on Continuity.submit_declared_work plus execute_declared_work as the only work path.
  - Added fail-closed source-absence assertions to e2e_m044_s05 so WorkLegacy and the old helper seam cannot silently return.
duration: ""
verification_result: passed
completed_at: 2026-03-30T06:21:18.222Z
blocker_discovered: false
---

# T02: Removed cluster-proof’s legacy /work probe path and proved the keyed runtime-owned route is the only remaining work surface.

**Removed cluster-proof’s legacy /work probe path and proved the keyed runtime-owned route is the only remaining work surface.**

## What Happened

Removed `WorkLegacy` from `cluster-proof/main.mpl`, deleted `cluster-proof/work_legacy.mpl`, shrank `cluster-proof/work.mpl` to keyed request/status models plus validation helpers, and cut the dead manual placement/dispatch/probe helpers out of `cluster-proof/work_continuity.mpl`. Rewrote `cluster-proof/tests/work.test.mpl` around keyed helper truth instead of legacy placement logic, and extended `compiler/meshc/tests/e2e_m044_s05.rs` with a fail-closed `m044_s05_legacy_cleanup_` rail that proves `GET /work` now returns 404, keyed submit/status/duplicate/conflict behavior still works, invalid inputs fail closed, and source absence checks catch any reintroduction of `WorkLegacy` or app-owned dispatch helpers.

## Verification

Passed the full task rail: `cargo run -q -p meshc -- build cluster-proof`, `cargo run -q -p meshc -- test cluster-proof/tests`, `cargo test -p meshc --test e2e_m044_s05 m044_s05_legacy_cleanup_ -- --nocapture`, and `test ! -e cluster-proof/work_legacy.mpl`. The live Rust rail also inspected combined logs and retained HTTP artifacts proving the missing legacy route and the absence of legacy dispatch logs.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 12342ms |
| 2 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 14994ms |
| 3 | `cargo test -p meshc --test e2e_m044_s05 m044_s05_legacy_cleanup_ -- --nocapture` | 0 | ✅ pass | 20899ms |
| 4 | `test ! -e cluster-proof/work_legacy.mpl` | 0 | ✅ pass | 77ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `cluster-proof/main.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/work_legacy.mpl`
- `cluster-proof/tests/work.test.mpl`
- `compiler/meshc/tests/e2e_m044_s05.rs`
- `.gsd/milestones/M044/slices/S05/tasks/T02-SUMMARY.md`


## Deviations
None.

## Known Issues
None.
