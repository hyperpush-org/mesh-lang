---
id: T01
parent: S02
milestone: M042
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-rt/src/dist/continuity.rs", "compiler/mesh-rt/src/dist/node.rs", "compiler/mesh-rt/src/lib.rs", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M042/slices/S02/tasks/T01-SUMMARY.md"]
key_decisions: ["D157: Add `mesh_continuity_submit_with_durability` now and keep `mesh_continuity_submit` as a compatibility wrapper until T02 updates the compiler/runtime seam."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran `cargo test -p mesh-rt continuity -- --nocapture` after the implementation and again after formatting. The final gate passed with 13 continuity-focused unit tests covering invalid durability input, durable rejection when replica safety is unavailable, rejected duplicate replay, conflict preservation, targeted prepare/ack mirrored admission, prepare failure rejection, disconnect-driven `degraded_continuing`, monotonic degraded-vs-stale-mirrored merging, completion guards, and wire-format roundtrips."
completed_at: 2026-03-28T23:00:07.925Z
blocker_discovered: false
---

# T01: Added runtime-owned durable continuity admission, replica prepare/ack rejection, and disconnect downgrade truth.

> Added runtime-owned durable continuity admission, replica prepare/ack rejection, and disconnect downgrade truth.

## What Happened
---
id: T01
parent: S02
milestone: M042
key_files:
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-rt/src/lib.rs
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M042/slices/S02/tasks/T01-SUMMARY.md
key_decisions:
  - D157: Add `mesh_continuity_submit_with_durability` now and keep `mesh_continuity_submit` as a compatibility wrapper until T02 updates the compiler/runtime seam.
duration: ""
verification_result: passed
completed_at: 2026-03-28T23:00:07.927Z
blocker_discovered: false
---

# T01: Added runtime-owned durable continuity admission, replica prepare/ack rejection, and disconnect downgrade truth.

**Added runtime-owned durable continuity admission, replica prepare/ack rejection, and disconnect downgrade truth.**

## What Happened

Extended `compiler/mesh-rt/src/dist/continuity.rs` so durable submit admission is explicit and monotonic: `SubmitRequest` now carries `required_replica_count`, `SubmitOutcome` can return `Rejected`, rejected admissions are persisted and replayed on same-key retries, and mirrored pending work can degrade to `degraded_continuing` after replica loss. Added targeted continuity prepare/ack transport in `compiler/mesh-rt/src/dist/node.rs` with pending request tracking so the owner-side runtime only accepts replica-required work after a real replica ack, otherwise it stores rejected truth with stable `phase`, `result`, `replica_status`, and `error` fields. Exported `mesh_continuity_submit_with_durability` from `compiler/mesh-rt/src/lib.rs` while keeping the legacy submit entrypoint as a compatibility wrapper for the next compiler/runtime seam task.

## Verification

Ran `cargo test -p mesh-rt continuity -- --nocapture` after the implementation and again after formatting. The final gate passed with 13 continuity-focused unit tests covering invalid durability input, durable rejection when replica safety is unavailable, rejected duplicate replay, conflict preservation, targeted prepare/ack mirrored admission, prepare failure rejection, disconnect-driven `degraded_continuing`, monotonic degraded-vs-stale-mirrored merging, completion guards, and wire-format roundtrips.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-rt continuity -- --nocapture` | 0 | ✅ pass | 270ms |


## Deviations

Added `mesh_continuity_submit_with_durability(...)` in T01 and kept `mesh_continuity_submit(...)` as a compatibility wrapper defaulting `required_replica_count=0` so the runtime behavior could land without breaking current compiler-generated call sites before T02 updates the seam.

## Known Issues

The Mesh-facing compiler/runtime seam still targets the legacy submit entrypoint until T02 updates the intrinsic/typechecker/lowering path, so live `cluster-proof` flows do not yet pass `required_replica_count` into the new runtime-owned admission path.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/lib.rs`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M042/slices/S02/tasks/T01-SUMMARY.md`


## Deviations
Added `mesh_continuity_submit_with_durability(...)` in T01 and kept `mesh_continuity_submit(...)` as a compatibility wrapper defaulting `required_replica_count=0` so the runtime behavior could land without breaking current compiler-generated call sites before T02 updates the seam.

## Known Issues
The Mesh-facing compiler/runtime seam still targets the legacy submit entrypoint until T02 updates the intrinsic/typechecker/lowering path, so live `cluster-proof` flows do not yet pass `required_replica_count` into the new runtime-owned admission path.
