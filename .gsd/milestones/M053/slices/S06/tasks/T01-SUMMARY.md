---
id: T01
parent: S06
milestone: M053
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-rt/src/dist/node.rs"]
key_decisions: ["Keep `MESH_STARTUP_WORK_DELAY_MS` parsing in `mesh-rt` so startup timing stays runtime-owned and invalid values fall back to the 2500ms safe default instead of pushing timing logic into starter code."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the task-owned runtime rail and confirmed the new startup-window tests pass in `mesh-rt`. Also ran the local workflow/contract guard from the slice plan; it stayed green, which means the hosted-proof wiring did not drift while touching the runtime seam. The `DATABASE_URL`- and `GH_TOKEN`-dependent slice rails were not run in this task because they remain owned by T02/T03."
completed_at: 2026-04-06T01:58:36.945Z
blocker_discovered: false
---

# T01: Restored the runtime-owned startup dispatch window env override and added fail-closed unit coverage.

> Restored the runtime-owned startup dispatch window env override and added fail-closed unit coverage.

## What Happened
---
id: T01
parent: S06
milestone: M053
key_files:
  - compiler/mesh-rt/src/dist/node.rs
key_decisions:
  - Keep `MESH_STARTUP_WORK_DELAY_MS` parsing in `mesh-rt` so startup timing stays runtime-owned and invalid values fall back to the 2500ms safe default instead of pushing timing logic into starter code.
duration: ""
verification_result: passed
completed_at: 2026-04-06T01:58:36.947Z
blocker_discovered: false
---

# T01: Restored the runtime-owned startup dispatch window env override and added fail-closed unit coverage.

**Restored the runtime-owned startup dispatch window env override and added fail-closed unit coverage.**

## What Happened

Updated `compiler/mesh-rt/src/dist/node.rs` so `startup_dispatch_window_ms(...)` now reads `MESH_STARTUP_WORK_DELAY_MS` through a small local helper, but only for runtime-owned startup request keys with `required_replica_count > 0`. Positive env values now override the pending window, while missing, zero, negative, malformed, or non-UTF8 values fall back to `STARTUP_CLUSTERED_PENDING_WINDOW_MS` (2500ms). Added a test-only env guard in the existing `#[cfg(test)]` module and replaced the old single dispatch-window assertion with focused `startup_work_dispatch_window_*` cases for missing env fallback, positive override boundaries (`1` and `20000`), invalid fallback (`0`, `-5`, and non-integer text), and zero-delay behavior for non-startup or replica-free requests even when the env is set.

## Verification

Ran the task-owned runtime rail and confirmed the new startup-window tests pass in `mesh-rt`. Also ran the local workflow/contract guard from the slice plan; it stayed green, which means the hosted-proof wiring did not drift while touching the runtime seam. The `DATABASE_URL`- and `GH_TOKEN`-dependent slice rails were not run in this task because they remain owned by T02/T03.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-rt startup_work_dispatch_window_ -- --nocapture` | 0 | ✅ pass | 46423ms |
| 2 | `bash scripts/verify-m034-s02-workflows.sh && node --test scripts/tests/verify-m053-s03-contract.test.mjs` | 0 | ✅ pass | 22344ms |


## Deviations

Added a small test-only env guard to restore process env between runtime unit cases. This stayed within the task plan and did not widen the shipped runtime contract.

## Known Issues

The `DATABASE_URL`-dependent staged Postgres failover rail and the `GH_TOKEN`-dependent hosted closeout rail were not exercised in T01. T02/T03 still own those slice-level checks.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/node.rs`


## Deviations
Added a small test-only env guard to restore process env between runtime unit cases. This stayed within the task plan and did not widen the shipped runtime contract.

## Known Issues
The `DATABASE_URL`-dependent staged Postgres failover rail and the `GH_TOKEN`-dependent hosted closeout rail were not exercised in T01. T02/T03 still own those slice-level checks.
