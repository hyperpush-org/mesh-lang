---
id: T02
parent: S02
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-rt/src/dist/continuity.rs", "compiler/mesh-rt/src/dist/node.rs", "compiler/meshc/src/cluster.rs", "compiler/mesh-typeck/src/infer.rs", "compiler/mesh-codegen/src/mir/lower.rs", ".gsd/milestones/M047/slices/S02/tasks/T02-SUMMARY.md"]
key_decisions: ["D275: continuity records store public replication_count, derive required_replica_count from declared-handler metadata, and reject unsupported fanout durably inside the continuity state machine while keeping the single-node startup carveout explicit."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "`cargo test -p mesh-rt m047_s02 -- --nocapture` passed with six M047 runtime tests covering continuity record preservation, durable unsupported-count rejection, and node-side required-replica derivation. `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture` also passed, confirming the updated continuity record surface still compiles through the existing route-free meshc/cluster CLI rail."
completed_at: 2026-04-01T07:11:06.739Z
blocker_discovered: false
---

# T02: Continuity records now preserve replication counts and declared-work runtime truth derives required replicas from registered handler metadata.

> Continuity records now preserve replication counts and declared-work runtime truth derives required replicas from registered handler metadata.

## What Happened
---
id: T02
parent: S02
milestone: M047
key_files:
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/meshc/src/cluster.rs
  - compiler/mesh-typeck/src/infer.rs
  - compiler/mesh-codegen/src/mir/lower.rs
  - .gsd/milestones/M047/slices/S02/tasks/T02-SUMMARY.md
key_decisions:
  - D275: continuity records store public replication_count, derive required_replica_count from declared-handler metadata, and reject unsupported fanout durably inside the continuity state machine while keeping the single-node startup carveout explicit.
duration: ""
verification_result: passed
completed_at: 2026-04-01T07:11:06.739Z
blocker_discovered: false
---

# T02: Continuity records now preserve replication counts and declared-work runtime truth derives required replicas from registered handler metadata.

**Continuity records now preserve replication counts and declared-work runtime truth derives required replicas from registered handler metadata.**

## What Happened

Added replication_count to runtime continuity records and Mesh-facing continuity payloads, preserved it through wire encode/decode and CLI rendering, and changed declared-work startup/direct submit/automatic recovery to derive required_replica_count from registered handler metadata instead of hardcoded defaults. The continuity state machine now keeps unsupported fanout requests queryable as rejected records with explicit unsupported_replication_count reasons, while startup preserves the existing single-node route-free carveout by relaxing the default two-copy request only when no peer was observed. Compiler/runtime type surfaces were kept aligned by updating the built-in ContinuityRecord shape in mesh-typeck inference and MIR lowering, and the existing route-free meshc rail stayed green after the runtime/CLI changes.

## Verification

`cargo test -p mesh-rt m047_s02 -- --nocapture` passed with six M047 runtime tests covering continuity record preservation, durable unsupported-count rejection, and node-side required-replica derivation. `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture` also passed, confirming the updated continuity record surface still compiles through the existing route-free meshc/cluster CLI rail.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-rt m047_s02 -- --nocapture` | 0 | ✅ pass | 14000ms |
| 2 | `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture` | 0 | ✅ pass | 44000ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/meshc/src/cluster.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `.gsd/milestones/M047/slices/S02/tasks/T02-SUMMARY.md`


## Deviations
None.

## Known Issues
None.
