---
id: T01
parent: S02
milestone: M028
provides:
  - Stronger reference-backend Rust e2e coverage that proves migration truth and one full HTTP/health/DB job lifecycle against a real Postgres runtime
key_files:
  - compiler/meshc/tests/e2e_reference_backend.rs
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M028/slices/S02/S02-PLAN.md
key_decisions:
  - Kept the golden-path proof inside compiler/meshc/tests/e2e_reference_backend.rs and deepened it with dynamic ports plus native-PG helpers instead of creating a second harness surface
patterns_established:
  - Reference-backend e2e tests can reset Postgres state, run `meshc migrate reference-backend <command>`, assert structured HTTP JSON, and cross-check the same truth through native_pg_query
observability_surfaces:
  - compiler/meshc/tests/e2e_reference_backend.rs ignored tests, GET /health, GET /jobs/:id, _mesh_migrations, jobs
duration: 1h 10m
verification_result: passed
completed_at: 2026-03-23T17:20:25Z
blocker_discovered: false
---

# T01: Deepen the Rust harness with migration, health, and DB truth assertions

**Deepened the reference-backend Rust harness with configurable ports, native-PG migration truth checks, and a real job-flow health/DB proof.**

## What Happened

I replaced the hardcoded `:18080` harness assumptions in `compiler/meshc/tests/e2e_reference_backend.rs` with configurable backend test config, dynamic port allocation, reusable HTTP helpers, structured JSON parsing, and native Postgres query/execute helpers using the same runtime path `meshc migrate` already trusts.

I kept the existing build/runtime/smoke proofs, updated the helper layer so future multi-instance work can spawn multiple backend processes cleanly, and added two new ignored proofs:

- `e2e_reference_backend_migration_status_and_apply` resets backend state, proves `status` shows the expected migration as pending before `up`, proves it as applied after `up`, and verifies `_mesh_migrations` directly through native PG reads.
- `e2e_reference_backend_job_flow_updates_health_and_db` resets backend state, applies migrations, starts the real backend, creates a job over HTTP, waits for the worker to finish, and then cross-checks `/jobs/:id`, `/health`, and the `jobs` table for `status`, `attempts`, `processed_at`, `processed_jobs`, `failed_jobs`, `last_job_id`, and `last_error` truth.

Because this session had no live `DATABASE_URL`, I reused the existing project-local `.env` target and the already-running local Docker Postgres container on `127.0.0.1:55432` to run the real runtime proofs without asking for manual setup.

## Verification

I ran the task-level verification commands plus the full slice verification matrix against the local Docker-backed Postgres from `.env`.

Task-level required proofs all passed:

- `e2e_reference_backend_builds`
- `e2e_reference_backend_migration_status_and_apply`
- `e2e_reference_backend_job_flow_updates_health_and_db`

Slice-level status at this intermediate task:

- `e2e_reference_backend_runtime_starts` passed.
- The future `e2e_reference_backend_multi_instance_claims_once` command exited successfully but matched **0 tests**, so it is a vacuous pass, not real multi-instance proof yet. I documented that gotcha in `.gsd/KNOWLEDGE.md` for T03.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_builds -- --nocapture` | 0 | ✅ pass | 9.17s |
| 2 | `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_runtime_starts -- --ignored --nocapture` | 0 | ✅ pass | 7.70s |
| 3 | `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_migration_status_and_apply -- --ignored --nocapture` | 0 | ✅ pass | 8.51s |
| 4 | `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_job_flow_updates_health_and_db -- --ignored --nocapture` | 0 | ✅ pass | 9.48s |
| 5 | `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_multi_instance_claims_once -- --ignored --nocapture` *(matched 0 tests; vacuous until T03)* | 0 | ✅ pass | 1.25s |

## Diagnostics

Future agents can inspect the new proof surface by running the ignored tests in `compiler/meshc/tests/e2e_reference_backend.rs` and reading the assertion failures they emit for:

- migration pending/applied drift from `meshc migrate reference-backend status`
- `_mesh_migrations` row/version mismatches through native PG reads
- `/jobs/:id` vs `jobs` row disagreements on `status`, `attempts`, `processed_at`, `payload`, or `last_error`
- `/health` worker-counter drift on `processed_jobs`, `failed_jobs`, `last_job_id`, and `last_error`

The local verification runtime used the Docker-backed database already pointed to by `.env`. The active container was `mesh_reference_backend_m028` on port `55432` during execution.

## Deviations

- Instead of collecting a remote `DATABASE_URL`, I reused the existing project-local `.env` target and local Docker Postgres path after the user asked for a local database. This preserved the slice contract while avoiding manual secret setup.

## Known Issues

- The slice-level `e2e_reference_backend_multi_instance_claims_once` command currently passes vacuously because no matching test exists yet. T03 still needs to add that named exact-once proof before the slice can claim multi-instance coverage.

## Files Created/Modified

- `compiler/meshc/tests/e2e_reference_backend.rs` — refactored the harness around configurable ports/processes, added native PG helpers, and added ignored migration/job-flow truth tests with structured JSON assertions.
- `.gsd/KNOWLEDGE.md` — recorded the zero-tests Cargo filter gotcha for the future multi-instance proof command.
- `.gsd/milestones/M028/slices/S02/S02-PLAN.md` — marked T01 complete.
