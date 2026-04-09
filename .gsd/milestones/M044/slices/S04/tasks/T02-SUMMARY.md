---
id: T02
parent: S04
milestone: M044
provides: []
requires: []
affects: []
key_files: [".gsd/milestones/M044/slices/S04/tasks/T02-SUMMARY.md"]
key_decisions: []
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Baseline verification only. cargo test -p mesh-rt automatic_recovery_ -- --nocapture exited 0 but ran 0 tests, confirming the runtime recovery proof surface does not exist yet. cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_resume_ -- --nocapture failed because the e2e_m044_s04 target does not exist yet. cargo run -q -p meshc -- test cluster-proof/tests passed, confirming the current package-level continuity contract is still green before any T02 edits."
completed_at: 2026-03-30T03:15:49.083Z
blocker_discovered: false
---

# T02: Replayed the missing auto-resume proof surface and documented the exact runtime and harness seams; no recovery code landed in this unit.

> Replayed the missing auto-resume proof surface and documented the exact runtime and harness seams; no recovery code landed in this unit.

## What Happened
---
id: T02
parent: S04
milestone: M044
key_files:
  - .gsd/milestones/M044/slices/S04/tasks/T02-SUMMARY.md
key_decisions:
  - (none)
duration: ""
verification_result: mixed
completed_at: 2026-03-30T03:15:49.085Z
blocker_discovered: false
---

# T02: Replayed the missing auto-resume proof surface and documented the exact runtime and harness seams; no recovery code landed in this unit.

**Replayed the missing auto-resume proof surface and documented the exact runtime and harness seams; no recovery code landed in this unit.**

## What Happened

I read the declared-handler submit/dispatch path in compiler/mesh-rt/src/dist/node.rs, the continuity rollover/promotion record model in compiler/mesh-rt/src/dist/continuity.rs, and the existing failover proof harnesses in compiler/meshc/tests/e2e_m042_s03.rs, compiler/meshc/tests/e2e_m043_s03.rs, and compiler/meshc/tests/e2e_m044_s03.rs. I confirmed that the runtime can already mint a new attempt through the owner-loss retry-rollover path, but it does not persist declared-handler recovery metadata in ContinuityRecord/SubmitRequest, handle_node_disconnect(...) never auto-promotes or auto-resumes declared work, and there is currently no compiler/meshc/tests/e2e_m044_s04.rs target. I stopped before code edits because the context-budget warning arrived, then wrote T02-SUMMARY.md with the exact seams and next implementation steps: persist runtime handler metadata in continuity records, add a non-terminal recovery-error surface, wire bounded auto-promotion plus runtime-owned redispatch from the disconnect path, and build the new e2e proof from the existing local-process failover harnesses.

## Verification

Baseline verification only. cargo test -p mesh-rt automatic_recovery_ -- --nocapture exited 0 but ran 0 tests, confirming the runtime recovery proof surface does not exist yet. cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_resume_ -- --nocapture failed because the e2e_m044_s04 target does not exist yet. cargo run -q -p meshc -- test cluster-proof/tests passed, confirming the current package-level continuity contract is still green before any T02 edits.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-rt automatic_recovery_ -- --nocapture` | 0 | ❌ fail | 1000ms |
| 2 | `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_resume_ -- --nocapture` | 101 | ❌ fail | 1000ms |
| 3 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 8000ms |


## Deviations

None. I stopped before implementation because of the context-budget warning.

## Known Issues

The task is not implemented yet. There is no automatic_recovery_ unit-test surface in mesh-rt, no compiler/meshc/tests/e2e_m044_s04.rs target, and the runtime still lacks persisted declared-handler recovery metadata plus an auto-resume redispatch path.

## Files Created/Modified

- `.gsd/milestones/M044/slices/S04/tasks/T02-SUMMARY.md`


## Deviations
None. I stopped before implementation because of the context-budget warning.

## Known Issues
The task is not implemented yet. There is no automatic_recovery_ unit-test surface in mesh-rt, no compiler/meshc/tests/e2e_m044_s04.rs target, and the runtime still lacks persisted declared-handler recovery metadata plus an auto-resume redispatch path.
