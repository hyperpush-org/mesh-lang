---
id: T01
parent: S01
milestone: M053
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-pkg/src/scaffold.rs", "scripts/tests/verify-m049-s03-materialize-examples.mjs", "compiler/meshc/tests/e2e_m049_s03.rs", "compiler/meshc/tests/support/m049_todo_examples.rs", "examples/todo-postgres/README.md", "examples/todo-postgres/scripts/stage-deploy.sh", "examples/todo-postgres/scripts/apply-deploy-migrations.sh", "examples/todo-postgres/scripts/deploy-smoke.sh", "examples/todo-postgres/deploy/todo-postgres.up.sql", ".gsd/milestones/M053/slices/S01/tasks/T01-SUMMARY.md"]
key_decisions: ["D400: the Postgres starter’s deployable handoff is a starter-owned staged bundle (`scripts/` + `deploy/<package>.up.sql`), with hosted/Fly concerns explicitly out of contract.", "Refresh committed starter examples from the rebuilt public `meshc` CLI instead of hand-editing them, even when stricter required-path checks force a clean regenerate."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the task-level rails: `cargo test -p mesh-pkg m049_s01_postgres_scaffold_ -- --nocapture`, `cargo test -p meshc --test e2e_m049_s03 -- --nocapture`, and `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`. Also verified the new observability/readme markers directly with `rg -n "\[stage-deploy\]|\[deploy-apply\]|\[deploy-smoke\]|staged bundle is the public deploy contract" examples/todo-postgres/README.md examples/todo-postgres/scripts/*.sh`. Slice-level `e2e_m053_s01` and `scripts/verify-m053-s01.sh` were not run yet because they belong to T02/T03."
completed_at: 2026-04-05T17:55:33.164Z
blocker_discovered: false
---

# T01: Generated Postgres starter-owned staged deploy assets and regenerated examples/todo-postgres from the updated scaffold.

> Generated Postgres starter-owned staged deploy assets and regenerated examples/todo-postgres from the updated scaffold.

## What Happened
---
id: T01
parent: S01
milestone: M053
key_files:
  - compiler/mesh-pkg/src/scaffold.rs
  - scripts/tests/verify-m049-s03-materialize-examples.mjs
  - compiler/meshc/tests/e2e_m049_s03.rs
  - compiler/meshc/tests/support/m049_todo_examples.rs
  - examples/todo-postgres/README.md
  - examples/todo-postgres/scripts/stage-deploy.sh
  - examples/todo-postgres/scripts/apply-deploy-migrations.sh
  - examples/todo-postgres/scripts/deploy-smoke.sh
  - examples/todo-postgres/deploy/todo-postgres.up.sql
  - .gsd/milestones/M053/slices/S01/tasks/T01-SUMMARY.md
key_decisions:
  - D400: the Postgres starter’s deployable handoff is a starter-owned staged bundle (`scripts/` + `deploy/<package>.up.sql`), with hosted/Fly concerns explicitly out of contract.
  - Refresh committed starter examples from the rebuilt public `meshc` CLI instead of hand-editing them, even when stricter required-path checks force a clean regenerate.
duration: ""
verification_result: passed
completed_at: 2026-04-05T17:55:33.168Z
blocker_discovered: false
---

# T01: Generated Postgres starter-owned staged deploy assets and regenerated examples/todo-postgres from the updated scaffold.

**Generated Postgres starter-owned staged deploy assets and regenerated examples/todo-postgres from the updated scaffold.**

## What Happened

Extended the Postgres todo scaffold in `compiler/mesh-pkg/src/scaffold.rs` so generated starters now emit a starter-owned staged deploy bundle: `scripts/stage-deploy.sh`, `scripts/apply-deploy-migrations.sh`, `scripts/deploy-smoke.sh`, and `deploy/<package>.up.sql`, alongside updated README guidance that keeps the bundle as the public deploy contract and keeps hosted/Fly concerns out of the starter contract. Tightened the scaffold unit tests to assert the new file set, script markers, deploy SQL content, and SQLite-local-only absence. Updated `scripts/tests/verify-m049-s03-materialize-examples.mjs`, `compiler/meshc/tests/e2e_m049_s03.rs`, and `compiler/meshc/tests/support/m049_todo_examples.rs` so example parity now requires the deploy-kit files and fails closed when staged deploy files drift. Rebuilt `target/debug/meshc` and regenerated `examples/todo-postgres/` from the updated public CLI; the first write attempt failed closed because the tracked example root was now malformed under the stricter required-path contract, so I removed that stale example root and regenerated it cleanly instead of hand-editing the example tree.

## Verification

Passed the task-level rails: `cargo test -p mesh-pkg m049_s01_postgres_scaffold_ -- --nocapture`, `cargo test -p meshc --test e2e_m049_s03 -- --nocapture`, and `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`. Also verified the new observability/readme markers directly with `rg -n "\[stage-deploy\]|\[deploy-apply\]|\[deploy-smoke\]|staged bundle is the public deploy contract" examples/todo-postgres/README.md examples/todo-postgres/scripts/*.sh`. Slice-level `e2e_m053_s01` and `scripts/verify-m053-s01.sh` were not run yet because they belong to T02/T03.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-pkg m049_s01_postgres_scaffold_ -- --nocapture` | 0 | ✅ pass | 34000ms |
| 2 | `cargo test -p meshc --test e2e_m049_s03 -- --nocapture` | 0 | ✅ pass | 9000ms |
| 3 | `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check` | 0 | ✅ pass | 1000ms |
| 4 | `rg -n "\[stage-deploy\]|\[deploy-apply\]|\[deploy-smoke\]|staged bundle is the public deploy contract" examples/todo-postgres/README.md examples/todo-postgres/scripts/*.sh` | 0 | ✅ pass | 500ms |


## Deviations

`verify-m049-s03-materialize-examples.mjs --write` failed closed against the existing partial `examples/todo-postgres/` tree once the required deploy-kit paths were added. I adapted by deleting that stale example root and re-materializing it from the rebuilt public CLI so the committed example stayed generator-owned. Otherwise none.

## Known Issues

None.

## Files Created/Modified

- `compiler/mesh-pkg/src/scaffold.rs`
- `scripts/tests/verify-m049-s03-materialize-examples.mjs`
- `compiler/meshc/tests/e2e_m049_s03.rs`
- `compiler/meshc/tests/support/m049_todo_examples.rs`
- `examples/todo-postgres/README.md`
- `examples/todo-postgres/scripts/stage-deploy.sh`
- `examples/todo-postgres/scripts/apply-deploy-migrations.sh`
- `examples/todo-postgres/scripts/deploy-smoke.sh`
- `examples/todo-postgres/deploy/todo-postgres.up.sql`
- `.gsd/milestones/M053/slices/S01/tasks/T01-SUMMARY.md`


## Deviations
`verify-m049-s03-materialize-examples.mjs --write` failed closed against the existing partial `examples/todo-postgres/` tree once the required deploy-kit paths were added. I adapted by deleting that stale example root and re-materializing it from the rebuilt public CLI so the committed example stayed generator-owned. Otherwise none.

## Known Issues
None.
