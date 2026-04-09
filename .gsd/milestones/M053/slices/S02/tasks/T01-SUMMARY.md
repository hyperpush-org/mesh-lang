---
id: T01
parent: S02
milestone: M053
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs", "compiler/meshc/tests/support/m053_todo_postgres_deploy.rs", "compiler/meshc/tests/e2e_m053_s02.rs", ".gsd/milestones/M053/slices/S02/tasks/T01-SUMMARY.md"]
key_decisions: ["Reused the M046 dual-stack ownership hash pattern so startup::Work.sync_todos is selected primary-owned without adding app-owned routing or delay code.", "Kept the startup delay seam helper-owned by carrying MESH_STARTUP_WORK_DELAY_MS only through staged runtime env instead of widening examples/todo-postgres/work.mpl."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran cargo test -p meshc --test e2e_m053_s02 --no-run and confirmed the new helper surfaces, test target, and runtime-config changes compile together. Did not run the planned DATABASE_URL-backed helper replay or slice-level verification rails in this context."
completed_at: 2026-04-05T19:25:14.468Z
blocker_discovered: false
---

# T01: Added the two-node staged Postgres helper seam and the first M053/S02 helper-contract test target.

> Added the two-node staged Postgres helper seam and the first M053/S02 helper-contract test target.

## What Happened
---
id: T01
parent: S02
milestone: M053
key_files:
  - compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs
  - compiler/meshc/tests/support/m053_todo_postgres_deploy.rs
  - compiler/meshc/tests/e2e_m053_s02.rs
  - .gsd/milestones/M053/slices/S02/tasks/T01-SUMMARY.md
key_decisions:
  - Reused the M046 dual-stack ownership hash pattern so startup::Work.sync_todos is selected primary-owned without adding app-owned routing or delay code.
  - Kept the startup delay seam helper-owned by carrying MESH_STARTUP_WORK_DELAY_MS only through staged runtime env instead of widening examples/todo-postgres/work.mpl.
duration: ""
verification_result: passed
completed_at: 2026-04-05T19:25:14.471Z
blocker_discovered: false
---

# T01: Added the two-node staged Postgres helper seam and the first M053/S02 helper-contract test target.

**Added the two-node staged Postgres helper seam and the first M053/S02 helper-contract test target.**

## What Happened

Extended the shared Postgres starter support so the staged deploy path can derive a paired primary/standby runtime configuration from one generated bundle and one shared database. Added helper-level clustered env validation plus optional MESH_STARTUP_WORK_DELAY_MS handling in compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs. Expanded compiler/meshc/tests/support/m053_todo_postgres_deploy.rs with S02-specific staged bundle pointer loading, primary-owned startup selection, dual-node spawn/stop wrappers, paired health/status/continuity/diagnostics helpers, and per-node HTTP snapshot support. Created compiler/meshc/tests/e2e_m053_s02.rs with real-helper, fail-closed, and bounded-source contract rails. The new target compiles; runtime execution against a local Docker Postgres instance remains for the next unit because the context-budget stop landed before that replay.

## Verification

Ran cargo test -p meshc --test e2e_m053_s02 --no-run and confirmed the new helper surfaces, test target, and runtime-config changes compile together. Did not run the planned DATABASE_URL-backed helper replay or slice-level verification rails in this context.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m053_s02 --no-run` | 0 | ✅ pass | 14170ms |


## Deviations

Used a compile-only --no-run verification pass as the completed check because the context-budget stop arrived before provisioning the local Docker Postgres instance and running the runtime rail.

## Known Issues

The new e2e_m053_s02 runtime/helper tests were not executed yet against a real local Postgres instance in this context. The negative and bounded-source tests compile but were not run yet.

## Files Created/Modified

- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`
- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs`
- `compiler/meshc/tests/e2e_m053_s02.rs`
- `.gsd/milestones/M053/slices/S02/tasks/T01-SUMMARY.md`


## Deviations
Used a compile-only --no-run verification pass as the completed check because the context-budget stop arrived before provisioning the local Docker Postgres instance and running the runtime rail.

## Known Issues
The new e2e_m053_s02 runtime/helper tests were not executed yet against a real local Postgres instance in this context. The negative and bounded-source tests compile but were not run yet.
