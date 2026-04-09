---
id: T02
parent: S01
milestone: M043
provides: []
requires: []
affects: []
key_files: ["cluster-proof/config.mpl", "cluster-proof/main.mpl", "cluster-proof/cluster.mpl", "cluster-proof/work.mpl", "cluster-proof/work_continuity.mpl", "cluster-proof/tests/config.test.mpl", "cluster-proof/tests/work.test.mpl", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M043/slices/S01/tasks/T02-SUMMARY.md"]
key_decisions: ["Validate `MESH_CONTINUITY_ROLE` and `MESH_CONTINUITY_PROMOTION_EPOCH` directly in `cluster-proof/config.mpl` so cluster mode requires explicit role truth instead of inheriting the runtime's permissive primary default.", "Keep topology parsing on an `error string + plain getter` seam after reproducing a boxed-primitive crash on a `Result<Int, String>` promotion-epoch helper; the operator contract stayed the same while the Mesh path became stable."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the task-plan verification commands exactly: `cargo run -q -p meshc -- test cluster-proof/tests` and `cargo run -q -p meshc -- build cluster-proof`. The package tests passed all config and keyed-work cases, and the full `cluster-proof` package built successfully."
completed_at: 2026-03-29T06:51:22.606Z
blocker_discovered: false
---

# T02: Surfaced runtime-owned primary/standby role truth through cluster-proof membership and keyed status surfaces, with fail-closed topology validation and package coverage.

> Surfaced runtime-owned primary/standby role truth through cluster-proof membership and keyed status surfaces, with fail-closed topology validation and package coverage.

## What Happened
---
id: T02
parent: S01
milestone: M043
key_files:
  - cluster-proof/config.mpl
  - cluster-proof/main.mpl
  - cluster-proof/cluster.mpl
  - cluster-proof/work.mpl
  - cluster-proof/work_continuity.mpl
  - cluster-proof/tests/config.test.mpl
  - cluster-proof/tests/work.test.mpl
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M043/slices/S01/tasks/T02-SUMMARY.md
key_decisions:
  - Validate `MESH_CONTINUITY_ROLE` and `MESH_CONTINUITY_PROMOTION_EPOCH` directly in `cluster-proof/config.mpl` so cluster mode requires explicit role truth instead of inheriting the runtime's permissive primary default.
  - Keep topology parsing on an `error string + plain getter` seam after reproducing a boxed-primitive crash on a `Result<Int, String>` promotion-epoch helper; the operator contract stayed the same while the Mesh path became stable.
duration: ""
verification_result: passed
completed_at: 2026-03-29T06:51:22.607Z
blocker_discovered: false
---

# T02: Surfaced runtime-owned primary/standby role truth through cluster-proof membership and keyed status surfaces, with fail-closed topology validation and package coverage.

**Surfaced runtime-owned primary/standby role truth through cluster-proof membership and keyed status surfaces, with fail-closed topology validation and package coverage.**

## What Happened

Extended `cluster-proof/config.mpl` with a small continuity-topology contract over `MESH_CONTINUITY_ROLE` and `MESH_CONTINUITY_PROMOTION_EPOCH`, keeping standalone truthful while making cluster mode fail closed unless role truth is explicit and consistent. Updated `cluster-proof/main.mpl` and `cluster-proof/cluster.mpl` so `/membership` now includes `cluster_role`, `promotion_epoch`, and `replication_health`. Expanded keyed work/status payloads in `cluster-proof/work.mpl` and threaded the new runtime record fields through `cluster-proof/work_continuity.mpl` for keyed status payloads, failure payloads, and request-scoped logs. Added package tests for topology validation, mirrored standby status visibility, malformed runtime JSON rejection, and no app-authored promotion logic. During execution, an initial `Result<Int, String>` promotion-epoch helper reproduced the known boxed-primitive crash on the cluster path, so the implementation was corrected to use fail-closed error-string validation plus plain getters without changing the surfaced contract.

## Verification

Ran the task-plan verification commands exactly: `cargo run -q -p meshc -- test cluster-proof/tests` and `cargo run -q -p meshc -- build cluster-proof`. The package tests passed all config and keyed-work cases, and the full `cluster-proof` package built successfully.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 13392ms |
| 2 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 10250ms |


## Deviations

Replaced an initial `Result<Int, String>` promotion-epoch helper with `error-string + plain getter` helpers after reproducing a cluster-path segmentation fault. This changed the implementation shape, not the shipped topology contract.

## Known Issues

None in the T02 scope.

## Files Created/Modified

- `cluster-proof/config.mpl`
- `cluster-proof/main.mpl`
- `cluster-proof/cluster.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/tests/config.test.mpl`
- `cluster-proof/tests/work.test.mpl`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M043/slices/S01/tasks/T02-SUMMARY.md`


## Deviations
Replaced an initial `Result<Int, String>` promotion-epoch helper with `error-string + plain getter` helpers after reproducing a cluster-path segmentation fault. This changed the implementation shape, not the shipped topology contract.

## Known Issues
None in the T02 scope.
