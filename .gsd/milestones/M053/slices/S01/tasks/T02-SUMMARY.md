---
id: T02
parent: S01
milestone: M053
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/support/m053_todo_postgres_deploy.rs", "compiler/meshc/tests/e2e_m053_s01.rs", "compiler/meshc/tests/support/mod.rs", ".gsd/milestones/M053/slices/S01/tasks/T02-SUMMARY.md"]
key_decisions: ["Kept staged-bundle creation and staged-script execution in a new M053 support module while reusing the existing M049 Postgres runtime/redaction helpers and M046 cluster-inspection helpers.", "Preserve an external staged bundle via a retained pointer plus manifest instead of copying the built binary back under the repo tree."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Compile verification passed for the new target with `cargo test -p meshc --test e2e_m053_s01 --no-run`. Attempts to run the authoritative replay failed before Cargo started because `.env` is not shell-sourceable and does not contain `DATABASE_URL`; a Python dotenv loader confirmed the key was still missing, and `secure_env_collect` returned `DATABASE_URL: skipped`. The next intended step is the user-approved temporary Fly MPG create/proxy/test/destroy replay."
completed_at: 2026-04-05T18:12:56.540Z
blocker_discovered: false
---

# T02: Added a staged Postgres starter deploy harness and e2e rail for external bundle boot, CRUD, and cluster inspection.

> Added a staged Postgres starter deploy harness and e2e rail for external bundle boot, CRUD, and cluster inspection.

## What Happened
---
id: T02
parent: S01
milestone: M053
key_files:
  - compiler/meshc/tests/support/m053_todo_postgres_deploy.rs
  - compiler/meshc/tests/e2e_m053_s01.rs
  - compiler/meshc/tests/support/mod.rs
  - .gsd/milestones/M053/slices/S01/tasks/T02-SUMMARY.md
key_decisions:
  - Kept staged-bundle creation and staged-script execution in a new M053 support module while reusing the existing M049 Postgres runtime/redaction helpers and M046 cluster-inspection helpers.
  - Preserve an external staged bundle via a retained pointer plus manifest instead of copying the built binary back under the repo tree.
duration: ""
verification_result: mixed
completed_at: 2026-04-05T18:12:56.542Z
blocker_discovered: false
---

# T02: Added a staged Postgres starter deploy harness and e2e rail for external bundle boot, CRUD, and cluster inspection.

**Added a staged Postgres starter deploy harness and e2e rail for external bundle boot, CRUD, and cluster inspection.**

## What Happened

Added `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs` as the staged-bundle harness for M053. It creates and validates a retained bundle outside the repo tree, prepends the local `meshc` binary into PATH for `stage-deploy.sh`, runs the staged apply/smoke scripts with captured and redacted stdout/stderr/meta logs, spawns the staged binary from the external bundle directory, and wraps the existing M046 cluster CLI helpers for single-node status, continuity, and diagnostics checks. Updated `compiler/meshc/tests/support/mod.rs` to export that harness. Added `compiler/meshc/tests/e2e_m053_s01.rs` with a real staged deploy replay plus fail-closed invalid-bundle-path and staged-script/cluster-CLI bad-input rails. The new target compiles with `cargo test -p meshc --test e2e_m053_s01 --no-run`, but the authoritative PostgreSQL-backed replay is still pending because this repo `.env` is not shell-sourceable, does not contain `DATABASE_URL`, `secure_env_collect` returned skipped, and the temporary Fly MPG verification path had to be deferred at the context-budget warning.

## Verification

Compile verification passed for the new target with `cargo test -p meshc --test e2e_m053_s01 --no-run`. Attempts to run the authoritative replay failed before Cargo started because `.env` is not shell-sourceable and does not contain `DATABASE_URL`; a Python dotenv loader confirmed the key was still missing, and `secure_env_collect` returned `DATABASE_URL: skipped`. The next intended step is the user-approved temporary Fly MPG create/proxy/test/destroy replay.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m053_s01 --no-run` | 0 | ✅ pass | 16110ms |
| 2 | `set -a && source .env && set +a && cargo test -p meshc --test e2e_m053_s01 -- --nocapture` | 127 | ❌ fail | 6900ms |
| 3 | `python3 dotenv loader -> cargo test -p meshc --test e2e_m053_s01 -- --nocapture` | 1 | ❌ fail | 100ms |


## Deviations

Did not complete the planned DATABASE_URL-backed end-to-end replay in this unit. `secure_env_collect` returned `DATABASE_URL: skipped`, and the user-approved temporary Fly MPG create/proxy/test/destroy path was not started before the context-budget warning forced wrap-up.

## Known Issues

The authoritative verification command `cargo test -p meshc --test e2e_m053_s01 -- --nocapture` has not been executed successfully yet, so the new staged deploy rail is implemented and compile-checked but not fully proven against a real PostgreSQL instance in this unit.

## Files Created/Modified

- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs`
- `compiler/meshc/tests/e2e_m053_s01.rs`
- `compiler/meshc/tests/support/mod.rs`
- `.gsd/milestones/M053/slices/S01/tasks/T02-SUMMARY.md`


## Deviations
Did not complete the planned DATABASE_URL-backed end-to-end replay in this unit. `secure_env_collect` returned `DATABASE_URL: skipped`, and the user-approved temporary Fly MPG create/proxy/test/destroy path was not started before the context-budget warning forced wrap-up.

## Known Issues
The authoritative verification command `cargo test -p meshc --test e2e_m053_s01 -- --nocapture` has not been executed successfully yet, so the new staged deploy rail is implemented and compile-checked but not fully proven against a real PostgreSQL instance in this unit.
