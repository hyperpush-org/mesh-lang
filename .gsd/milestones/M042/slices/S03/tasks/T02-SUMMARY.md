---
id: T02
parent: S03
milestone: M042
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-rt/src/dist/continuity.rs", "compiler/mesh-rt/src/dist/node.rs", ".gsd/milestones/M042/slices/S03/tasks/T02-SUMMARY.md"]
key_decisions: ["Mark pending mirrored continuity records with explicit `replica_status=owner_lost` during `handle_node_disconnect(...)`, then let ordinary `Continuity.submit(...)` consume that runtime-owned state to roll a new attempt.", "Prefer `owner_lost` over stale `mirrored` data in merge precedence so same-identity rejoin cannot clear the surviving node's pre-retry recovery signal before a rollover happens."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the task verification command from the plan and the unchanged thin-consumer rail. `cargo test -p mesh-rt continuity -- --nocapture` passed with the new owner-loss coverage and showed the new `[mesh-rt continuity] transition=owner_lost ...` observability surface. `cargo run -q -p meshc -- test cluster-proof/tests` also passed unchanged, confirming the Mesh consumer still parses and maps the runtime-owned continuity JSON contract without app-authored repair logic."
completed_at: 2026-03-29T00:14:56.380Z
blocker_discovered: false
---

# T02: Added explicit runtime owner-loss continuity state and ordinary-submit recovery rollover.

> Added explicit runtime owner-loss continuity state and ordinary-submit recovery rollover.

## What Happened
---
id: T02
parent: S03
milestone: M042
key_files:
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-rt/src/dist/node.rs
  - .gsd/milestones/M042/slices/S03/tasks/T02-SUMMARY.md
key_decisions:
  - Mark pending mirrored continuity records with explicit `replica_status=owner_lost` during `handle_node_disconnect(...)`, then let ordinary `Continuity.submit(...)` consume that runtime-owned state to roll a new attempt.
  - Prefer `owner_lost` over stale `mirrored` data in merge precedence so same-identity rejoin cannot clear the surviving node's pre-retry recovery signal before a rollover happens.
duration: ""
verification_result: passed
completed_at: 2026-03-29T00:14:56.381Z
blocker_discovered: false
---

# T02: Added explicit runtime owner-loss continuity state and ordinary-submit recovery rollover.

**Added explicit runtime owner-loss continuity state and ordinary-submit recovery rollover.**

## What Happened

Extended `compiler/mesh-rt/src/dist/continuity.rs` so pending replicated records can move into an explicit `owner_lost` state when the active owner disappears, without widening the Mesh-facing continuity schema. `ContinuityRegistry.submit(...)` now uses that runtime-owned owner-loss state on the ordinary submit path, so same-key retry rolls a new `attempt_id` without a separate repair API. In `compiler/mesh-rt/src/dist/node.rs`, `handle_node_disconnect(...)` now marks owner-loss records before the existing replica-loss downgrade path, emits a structured `transition=owner_lost` log, and leaves stale reconnect sync beneath the newer local truth until retry rollover. Added runtime continuity tests for explicit owner-loss marking, ordinary-submit recovery rollover, unrelated/terminal disconnect no-ops, repeated disconnect idempotence, stale mirrored rejoin data losing to owner-lost state, and stale completion rejection after recovery rollover. `cluster-proof` code stayed unchanged because its existing runtime-owned JSON parsing and status mapping already accept the new status string without app-authored repair logic.

## Verification

Ran the task verification command from the plan and the unchanged thin-consumer rail. `cargo test -p mesh-rt continuity -- --nocapture` passed with the new owner-loss coverage and showed the new `[mesh-rt continuity] transition=owner_lost ...` observability surface. `cargo run -q -p meshc -- test cluster-proof/tests` also passed unchanged, confirming the Mesh consumer still parses and maps the runtime-owned continuity JSON contract without app-authored repair logic.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-rt continuity -- --nocapture` | 0 | ✅ pass | 10256ms |
| 2 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 18920ms |


## Deviations

None.

## Known Issues

The slice-level destructive owner-loss harness and fail-closed verifier for live two-node restart/rejoin proof are still pending in T03. This task only landed the runtime hook and unit-level authority rules.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `.gsd/milestones/M042/slices/S03/tasks/T02-SUMMARY.md`


## Deviations
None.

## Known Issues
The slice-level destructive owner-loss harness and fail-closed verifier for live two-node restart/rejoin proof are still pending in T03. This task only landed the runtime hook and unit-level authority rules.
