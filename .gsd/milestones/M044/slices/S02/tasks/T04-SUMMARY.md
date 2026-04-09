---
id: T04
parent: S02
milestone: M044
provides: []
requires: []
affects: []
key_files: [".gsd/KNOWLEDGE.md", ".gsd/milestones/M044/slices/S02/tasks/T04-SUMMARY.md"]
key_decisions: ["Did not rewrite `cluster-proof` onto a fake local shim once the declared-handler runtime path proved absent in the working tree.", "Recorded the T04 stop as a plan-invalidating blocker because the remaining rewrite depends on T02/T03 execution plumbing that is still missing locally."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the task-plan build and package-test commands, then ran the named T04 rail and confirmed it still has no tests behind the requested filter. Also captured a compact source-level proof snapshot showing the dropped clustered execution plan in `meshc`, the generic `mesh_register_function` registry in `mesh-rt`, and the still-local placement/dispatch seams in `cluster-proof/work_continuity.mpl`."
completed_at: 2026-03-29T20:11:01.371Z
blocker_discovered: true
---

# T04: Stopped before a dishonest `cluster-proof` rewrite and recorded the missing S02 declared-execution seams.

> Stopped before a dishonest `cluster-proof` rewrite and recorded the missing S02 declared-execution seams.

## What Happened
---
id: T04
parent: S02
milestone: M044
key_files:
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M044/slices/S02/tasks/T04-SUMMARY.md
key_decisions:
  - Did not rewrite `cluster-proof` onto a fake local shim once the declared-handler runtime path proved absent in the working tree.
  - Recorded the T04 stop as a plan-invalidating blocker because the remaining rewrite depends on T02/T03 execution plumbing that is still missing locally.
duration: ""
verification_result: passed
completed_at: 2026-03-29T20:11:01.377Z
blocker_discovered: true
---

# T04: Stopped before a dishonest `cluster-proof` rewrite and recorded the missing S02 declared-execution seams.

**Stopped before a dishonest `cluster-proof` rewrite and recorded the missing S02 declared-execution seams.**

## What Happened

Activated the task contract, checked the live `cluster-proof` surfaces, and reproduced the current verification state before changing code. The package still builds and its package tests still pass, but the named T04 rail is absent: `cargo test -p meshc --test e2e_m044_s02 m044_s02_cluster_proof_ -- --nocapture` exits 0 while running zero tests. I then verified that the working tree still lacks the runtime/compiler prerequisites T04 depends on: `compiler/meshc/src/main.rs::build(...)` still drops `PreparedBuild.clustered_execution_plan`, `compiler/mesh-rt/src/dist/node.rs` still only exposes the generic `mesh_register_function` registry, and `cluster-proof/work_continuity.mpl` still owns both `current_target_selection(...)` and direct `Node.spawn(...)` dispatch on the keyed submit path. Rather than landing an app-only rewrite that would overclaim S02, I recorded the blocker seam in `.gsd/KNOWLEDGE.md` and wrote the task summary for replanning.

## Verification

Ran the task-plan build and package-test commands, then ran the named T04 rail and confirmed it still has no tests behind the requested filter. Also captured a compact source-level proof snapshot showing the dropped clustered execution plan in `meshc`, the generic `mesh_register_function` registry in `mesh-rt`, and the still-local placement/dispatch seams in `cluster-proof/work_continuity.mpl`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 10100ms |
| 2 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 12700ms |
| 3 | `cargo test -p meshc --test e2e_m044_s02 m044_s02_cluster_proof_ -- --nocapture` | 0 | ❌ fail | 12600ms |
| 4 | `rg -n "let _clustered_execution_plan|mesh_register_function|current_target_selection\(|Node\.spawn\(|m044_s02_cluster_proof_" compiler/meshc/src/main.rs compiler/mesh-rt/src/dist/node.rs cluster-proof/work_continuity.mpl compiler/meshc/tests/e2e_m044_s02.rs` | 0 | ✅ pass | 39ms |


## Deviations

Did not perform the planned `cluster-proof` manifest/handler rewrite. Local execution showed that the required T02/T03 runtime-owned declared-handler path is still missing, so proceeding with an app-only rewrite would have violated the slice contract rather than adapting a minor mismatch.

## Known Issues

The remaining S02 work is still blocked on the missing declared-handler execution substrate: there are no `m044_s02_declared_work_`, `m044_s02_service_`, or `m044_s02_cluster_proof_` tests in `compiler/meshc/tests/e2e_m044_s02.rs`, `meshc build` still ignores `PreparedBuild.clustered_execution_plan`, and `cluster-proof/work_continuity.mpl` still computes keyed target selection plus direct `Node.spawn(...)` dispatch on the new hot path.

## Files Created/Modified

- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M044/slices/S02/tasks/T04-SUMMARY.md`


## Deviations
Did not perform the planned `cluster-proof` manifest/handler rewrite. Local execution showed that the required T02/T03 runtime-owned declared-handler path is still missing, so proceeding with an app-only rewrite would have violated the slice contract rather than adapting a minor mismatch.

## Known Issues
The remaining S02 work is still blocked on the missing declared-handler execution substrate: there are no `m044_s02_declared_work_`, `m044_s02_service_`, or `m044_s02_cluster_proof_` tests in `compiler/meshc/tests/e2e_m044_s02.rs`, `meshc build` still ignores `PreparedBuild.clustered_execution_plan`, and `cluster-proof/work_continuity.mpl` still computes keyed target selection plus direct `Node.spawn(...)` dispatch on the new hot path.
