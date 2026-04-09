---
id: S02
parent: M028
milestone: M028
provides:
  - an authoritative Postgres-backed runtime-correctness harness in `compiler/meshc/tests/e2e_reference_backend.rs` covering startup, migration truth, single-job lifecycle truth, contention-safe claiming, and two-instance exact-once processing
  - an atomic `reference-backend` job-claim path that removes the read/update race on the shared `jobs` table without changing runtime internals
  - a stable proof split for shared-DB correctness: `/health.failed_jobs` + `/health.last_error` for failure visibility, DB rows + `/jobs/:id` + worker logs for exact-once truth
requires:
  - S01
affects:
  - R003
key_files:
  - compiler/meshc/tests/e2e_reference_backend.rs
  - reference-backend/storage/jobs.mpl
  - reference-backend/jobs/worker.mpl
  - reference-backend/api/health.mpl
  - reference-backend/api/jobs.mpl
  - .gsd/KNOWLEDGE.md
  - .gsd/DECISIONS.md
key_decisions:
  - Keep the runtime-correctness proof in the existing compiler-facing `e2e_reference_backend` harness instead of introducing a second backend test surface.
  - Fix shared-DB claim races entirely in application code with one atomic `Repo.query_raw(...)` claim-and-return statement using `FOR UPDATE SKIP LOCKED`, leaving `compiler/mesh-rt/src/db/repo.rs` unchanged.
  - Treat `/health.failed_jobs` and `/health.last_error` as the stable contention signal, and prove exact-once plus cross-instance participation through direct `jobs` reads, opposite-instance `/jobs/:id`, and per-instance `Job worker processed id=` logs.
patterns_established:
  - Reference-backend runtime proofs should reset DB state, drive the real binary over HTTP, and cross-check the same truth through direct Postgres reads rather than trusting command success or shell smoke alone.
  - Shared-DB worker correctness should be proved with durable row state (`status`, `attempts`, `processed_at`, `last_error`) and participation logs, not by summing noisy in-memory counters under contention.
  - Worker observability is most trustworthy when `/health` focuses on liveness and false-failure visibility while exact job truth stays anchored in the database and job API.
observability_surfaces:
  - `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_runtime_starts -- --ignored --nocapture`
  - `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_migration_status_and_apply -- --ignored --nocapture`
  - `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_job_flow_updates_health_and_db -- --ignored --nocapture`
  - `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_claim_contention_is_not_failure -- --ignored --nocapture`
  - `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_multi_instance_claims_once -- --ignored --nocapture`
  - `meshc migrate reference-backend status`
  - `meshc migrate reference-backend up`
  - `GET /health`
  - `GET /jobs/:id`
  - direct Postgres reads from `_mesh_migrations` and `jobs`
drill_down_paths:
  - .gsd/KNOWLEDGE.md
  - .gsd/DECISIONS.md
  - .gsd/milestones/M028/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M028/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M028/slices/S02/tasks/T03-SUMMARY.md
duration: slice closure verification + summary
verification_result: passed
completed_at: 2026-03-23
blocker_discovered: false
---
# S02: Runtime Correctness on the Golden Path

## Outcome

S02 turns `reference-backend/` from a believable smoke target into a mechanically trusted golden path. The same compiler-facing Rust harness that S01 introduced now proves the real runtime path end to end: startup against a real `DATABASE_URL`, migration pending→applied truth, one job’s HTTP/DB/health lifecycle, contention-safe shared-DB claiming, and two-instance exact-once processing.

This slice also fixed the most credibility-damaging runtime issue on that path. Ordinary claim contention on the shared `jobs` table no longer shows up as worker failure accounting. The backend now treats benign claim misses as idle/no-work behavior, while real processing failures still surface through `failed_jobs` and `last_error`.

## What this slice actually delivered

### 1. The authoritative runtime-correctness harness stayed in one place

S02 did not create a second proof surface. It deepened `compiler/meshc/tests/e2e_reference_backend.rs` so later slices can keep extending one authoritative harness instead of splitting backend truth across ad hoc scripts.

The harness now has:

- configurable backend process ports for single- and multi-instance scenarios
- reusable HTTP JSON helpers for `/health`, `POST /jobs`, and `GET /jobs/:id`
- native Postgres query helpers aligned with the same low-level path `meshc migrate` uses
- reusable multi-process startup, shutdown, and log-capture helpers

That matters for downstream slices because S03-S05 can keep adding proof to the same compiler-facing gate instead of inventing new verification surfaces.

### 2. Migration truth is now proved, not inferred

The new migration proof does more than check command exit codes.

`e2e_reference_backend_migration_status_and_apply` now proves all of these together:

- `meshc migrate reference-backend status` reports the expected migration as pending before apply
- `meshc migrate reference-backend up` applies it
- a follow-up `status` reports the migration as applied
- `_mesh_migrations` in Postgres contains the same version directly

So the milestone now has a reliable answer to “did the schema actually land?” instead of trusting CLI output alone.

### 3. Single-job runtime truth now agrees across HTTP, health, and DB

`e2e_reference_backend_job_flow_updates_health_and_db` proves one real job lifecycle across all relevant surfaces:

- `POST /jobs` creates the durable row
- the worker moves that row from `pending` to `processed`
- `GET /jobs/:id` reflects the terminal state
- direct `jobs` reads agree on `status`, `attempts`, `processed_at`, payload, and `last_error`
- `GET /health` agrees on worker counters and failure state

This is the slice’s core golden-path trust gain: the backend’s user-facing API, its health surface, and the underlying Postgres row now have executable agreement checks instead of a shell smoke approximation.

### 4. Shared-DB claiming is now atomic and contention-safe

Before S02, the claim path had a read-then-update race. Under two backend instances sharing one `jobs` table, ordinary claim contention could surface as `update_where: no rows matched` and inflate `failed_jobs` even though nothing actually broke.

S02 fixed that by replacing the app-level claim flow in `reference-backend/storage/jobs.mpl` with one atomic SQL claim-and-return statement using:

- `Repo.query_raw(...)`
- `FOR UPDATE SKIP LOCKED`
- `RETURNING`

That removed the race window without changing `compiler/mesh-rt/src/db/repo.rs`.

The worker side in `reference-backend/jobs/worker.mpl` was updated to classify benign claim misses as idle behavior. Real processing failures still drive `NoteFailed`, `failed_jobs`, and `last_error`; benign shared-DB contention does not.

### 5. The slice now has a named two-instance exact-once proof

`e2e_reference_backend_multi_instance_claims_once` is no longer a vacuous filter. It now starts two real `reference-backend` processes on unique ports against the same database, alternates job creation across both instances, and proves all of this together:

- every durable `jobs` row ends in `processed`
- every row has `attempts = 1`
- no row lands in `failed`
- opposite-instance `GET /jobs/:id` reads agree with DB truth
- both workers actually participated, confirmed through `Job worker processed id=` logs
- neither instance reports benign contention as failure through `/health.failed_jobs` or `/health.last_error`

This is the slice’s strongest deliverable because it directly retires the specific “concurrency exists but isn’t trustworthy” risk on the golden path’s normal shared-DB case.

### 6. Worker state/log noise was reduced to make the proof more reliable

While stabilizing the two-instance gate, S02 removed the redundant per-iteration tick state/log update from `reference-backend/jobs/worker.mpl`. The worker still records meaningful idle/claim/process/failure transitions, but it no longer writes extra churn on every poll iteration.

That makes `/health` and logs easier to trust without over-claiming that they are perfect exact-once counters under heavier contention.

## Verification that passed during slice closure

All slice-level verification commands were rerun from the repo root. For the Postgres-backed commands, the repo-root `.env` was loaded into the subprocess environment first so the tests exercised the real runtime path instead of failing early on an unset `DATABASE_URL` guard.

### Passed slice-gate commands

```bash
cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_builds -- --nocapture
DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_runtime_starts -- --ignored --nocapture
DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_migration_status_and_apply -- --ignored --nocapture
DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_job_flow_updates_health_and_db -- --ignored --nocapture
DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_multi_instance_claims_once -- --ignored --nocapture
```

### Additional regression check that also passed

```bash
DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_claim_contention_is_not_failure -- --ignored --nocapture
```

## Observability spot checks confirmed

Closure verification confirmed the slice’s intended inspection surfaces are usable and aligned:

- `meshc migrate reference-backend status` / `up` and direct `_mesh_migrations` reads agree on migration version truth
- `/jobs/:id` and direct `jobs` reads agree on `status`, `attempts`, `processed_at`, payload, and empty/null error state for processed jobs
- `/health` remains useful for worker liveness plus false-failure visibility via `failed_jobs`, `last_error`, and worker status
- per-instance worker logs show real participation through `Job worker processed id=` and no longer show the old `update_where: no rows matched` contention failure shape during normal shared-DB operation

## Forward Intelligence

### What later slices can rely on

- The compiler-facing `e2e_reference_backend` harness is now the canonical runtime-correctness surface for the reference backend.
- The migration, job API, worker, and health path are proved against a real Postgres database, not just via shell smoke.
- The ordinary two-instance shared-DB case has an executable regression for exact-once processing and benign-contention classification.
- `reference-backend/storage/jobs.mpl` now contains a proven atomic claim seam that later slices should preserve unless they intentionally redesign storage semantics.

### Authoritative diagnostics

Use these first when the golden path looks wrong:

1. `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_migration_status_and_apply -- --ignored --nocapture`
   - Best first check for migration truth drift.
2. `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_job_flow_updates_health_and_db -- --ignored --nocapture`
   - Best first check when HTTP, health, and DB disagree for a single job lifecycle.
3. `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_multi_instance_claims_once -- --ignored --nocapture`
   - Best first check when shared-DB correctness or duplicate processing is in doubt.
4. `GET /health`
   - Best live signal for worker liveness and false-failure visibility.
5. direct Postgres reads from `jobs` and `_mesh_migrations`
   - Best durable truth when HTTP responses or in-memory counters look suspicious.

### Gotchas worth preserving

- In this worktree, non-interactive shell commands do not inherit the repo-root `.env`; load it before running Postgres-backed verification.
- Keep the Cargo test ordering as `cargo test -p meshc --test e2e_reference_backend <test_name> -- ...`.
- For two-instance proof, do not over-trust `/health.processed_jobs` or `/health.last_job_id` as exact totals under heavier polling.
- Reverting `claim_next_pending_job()` to a read-then-update flow will reintroduce benign claim races as false worker failures.

### What S02 does not prove yet

S02 proves the golden path under normal runtime and shared-DB contention conditions. It does **not** yet prove:

- crash recovery or restart semantics after worker/process failure
- supervision guarantees beyond the normal running path
- deployment/binary smoke outside the development proof workflow
- daily-driver tooling trust for fmt/diagnostics/LSP/tests/coverage
- final docs/examples promotion for external evaluators

Those remain the jobs of S03-S06, but they now inherit a much stronger runtime baseline instead of a smoke-only backend story.
