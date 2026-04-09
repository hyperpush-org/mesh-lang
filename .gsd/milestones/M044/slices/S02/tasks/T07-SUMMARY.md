---
id: T07
parent: S02
milestone: M044
provides: []
requires: []
affects: []
key_files: [".gsd/milestones/M044/slices/S02/tasks/T07-SUMMARY.md"]
key_decisions: ["Did not rewrite `cluster-proof` onto a fake declared-runtime path while `meshc` still drops `PreparedBuild.clustered_execution_plan` and the proof app still owns keyed placement/dispatch.", "Reconfirmed that T07 is still blocked on the missing T05/T06 substrate: declared service exports still point at raw `__service_*` helpers, and the named `m044_s02_cluster_proof_` rail still has no tests behind it."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Reran the task-plan verification commands against the unmodified tree and captured one source-level seam check. `cluster-proof` still builds and its package tests still pass. The named T07 e2e filter still runs zero tests, which is the failure signal. S02-PLAN does not define an additional slice-level verification command beyond this task rail."
completed_at: 2026-03-29T20:39:16.335Z
blocker_discovered: true
---

# T07: Recorded that T07 remains blocked by the missing declared-handler execution substrate; `cluster-proof` still cannot be rewritten honestly.

> Recorded that T07 remains blocked by the missing declared-handler execution substrate; `cluster-proof` still cannot be rewritten honestly.

## What Happened
---
id: T07
parent: S02
milestone: M044
key_files:
  - .gsd/milestones/M044/slices/S02/tasks/T07-SUMMARY.md
key_decisions:
  - Did not rewrite `cluster-proof` onto a fake declared-runtime path while `meshc` still drops `PreparedBuild.clustered_execution_plan` and the proof app still owns keyed placement/dispatch.
  - Reconfirmed that T07 is still blocked on the missing T05/T06 substrate: declared service exports still point at raw `__service_*` helpers, and the named `m044_s02_cluster_proof_` rail still has no tests behind it.
duration: ""
verification_result: passed
completed_at: 2026-03-29T20:39:16.336Z
blocker_discovered: true
---

# T07: Recorded that T07 remains blocked by the missing declared-handler execution substrate; `cluster-proof` still cannot be rewritten honestly.

**Recorded that T07 remains blocked by the missing declared-handler execution substrate; `cluster-proof` still cannot be rewritten honestly.**

## What Happened

Loaded the T07 contract, the earlier T04/T06 handoff notes, and the live `cluster-proof` / compiler seams before changing code. The current tree still does not have the runtime-owned declared execution path this rewrite depends on. `compiler/meshc/src/main.rs::build(...)` still reads `PreparedBuild.clustered_execution_plan` into a throwaway `_clustered_execution_plan` binding, so the manifest-approved execution metadata never reaches lowering or codegen. `compiler/mesh-typeck/src/infer.rs` still exports declared service targets as raw `__service_*_call/*_cast` helper symbols rather than distinct clustered wrapper symbols, and `cluster-proof/work_continuity.mpl` still owns the keyed submit hot path through `current_target_selection(...)` plus direct `Node.spawn(...)` dispatch.

I also reproduced the verification state the slice contract depends on. `cargo run -q -p meshc -- build cluster-proof` still succeeds, and `cargo run -q -p meshc -- test cluster-proof/tests` still passes, so there is no local regression in the existing proof app. But the named T07 rail is still absent: `cargo test -p meshc --test e2e_m044_s02 m044_s02_cluster_proof_ -- --nocapture` exits 0 while running 0 tests, which means there is still no executable proof for the rewrite this task is supposed to land. Because the compiler/runtime substrate is still missing, any app-only manifest or handler rewrite here would overclaim S02 instead of dogfooding a real declared-runtime path. I stopped at the honest blocker boundary and recorded the current evidence instead of shipping a fake green change.

## Verification

Reran the task-plan verification commands against the unmodified tree and captured one source-level seam check. `cluster-proof` still builds and its package tests still pass. The named T07 e2e filter still runs zero tests, which is the failure signal. S02-PLAN does not define an additional slice-level verification command beyond this task rail.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m044_s02 m044_s02_cluster_proof_ -- --nocapture` | 0 | ❌ fail | 3245ms |
| 2 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 13523ms |
| 3 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 17517ms |
| 4 | `rg -n 'let _clustered_execution_plan|current_target_selection\(|Node\.spawn\(|m044_s02_(declared_work|service|cluster_proof)_|__service_.*(call|cast)' compiler/meshc/src/main.rs cluster-proof/work_continuity.mpl compiler/meshc/tests/e2e_m044_s02.rs compiler/mesh-typeck/src/infer.rs` | 0 | ✅ pass | 30ms |


## Deviations

Did not perform the planned `cluster-proof` manifest/handler rewrite. Local execution showed that the remaining slice plan is still blocked on missing compiler/runtime support, so an app-only rewrite would have violated the task contract rather than adapting a minor mismatch.

## Known Issues

The missing declared-handler execution substrate is still the blocker for the remainder of S02. Until `meshc` actually consumes `clustered_execution_plan`, declared service targets lower to real clustered wrappers, and `cluster-proof` can call runtime-owned submit/status surfaces, T07 cannot land honestly.

## Files Created/Modified

- `.gsd/milestones/M044/slices/S02/tasks/T07-SUMMARY.md`


## Deviations
Did not perform the planned `cluster-proof` manifest/handler rewrite. Local execution showed that the remaining slice plan is still blocked on missing compiler/runtime support, so an app-only rewrite would have violated the task contract rather than adapting a minor mismatch.

## Known Issues
The missing declared-handler execution substrate is still the blocker for the remainder of S02. Until `meshc` actually consumes `clustered_execution_plan`, declared service targets lower to real clustered wrappers, and `cluster-proof` can call runtime-owned submit/status surfaces, T07 cannot land honestly.
