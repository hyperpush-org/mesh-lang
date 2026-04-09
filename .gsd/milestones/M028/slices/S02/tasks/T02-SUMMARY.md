---
id: T02
parent: S02
milestone: M028
provides:
  - Atomic reference-backend job claiming plus a real two-instance regression that proves shared-DB contention no longer inflates worker failure counters
key_files:
  - compiler/meshc/tests/e2e_reference_backend.rs
  - reference-backend/storage/jobs.mpl
  - reference-backend/jobs/worker.mpl
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M028/slices/S02/S02-PLAN.md
key_decisions:
  - Kept the claim fix entirely in application code by using `Repo.query_raw(...)` for a single Postgres claim-and-return statement, so `compiler/mesh-rt/src/db/repo.rs` did not need a runtime seam change
patterns_established:
  - Multi-instance reference-backend contention can be proved mechanically by posting a burst of jobs to one instance, waiting for DB truth, then asserting combined `/health` counters stay at `failed_jobs=0` while both workers participate
observability_surfaces:
  - GET /health worker counters, GET /jobs/:id, direct `jobs` table reads, and worker logs for benign claim-miss vs real failure shapes
duration: 1h 45m
verification_result: passed
completed_at: 2026-03-23T17:36:19Z
blocker_discovered: false
---

# T02: Make job claiming atomic and contention-safe on the reference backend

**Made reference-backend job claims atomic and proved shared-DB contention no longer inflates worker failures.**

## What Happened

I first reproduced the real bug outside the harness by starting two `reference-backend` instances against the same Postgres database and posting a burst of jobs. That live repro showed the exact bad signal the slice called out: both instances processed real work, but `/health` still reported non-zero `failed_jobs`, and the worker logs showed `update_where: no rows matched` from ordinary claim races.

I then encoded that behavior into `compiler/meshc/tests/e2e_reference_backend.rs` with a new ignored proof, `e2e_reference_backend_claim_contention_is_not_failure`, plus two small helper layers at `compiler/meshc/tests/e2e_reference_backend.rs:522`, `:559`, and `:817`. The regression starts two backend instances on unique ports, posts 20 jobs through the real HTTP API, waits for DB truth on the shared `jobs` table, and then asserts both instances participated while combined `/health` counters still report `failed_jobs == 0` and `last_error == null`.

To remove the race window, I rewrote `reference-backend/storage/jobs.mpl:59-84` so `claim_next_pending_job()` no longer does `oldest_pending_job()` followed by `Repo.update_where(...)`. The new path uses one `Repo.query_raw(...)` SQL statement with `FOR UPDATE SKIP LOCKED` and `RETURNING` to select, claim, and return the row atomically in a single round-trip.

I also updated `reference-backend/jobs/worker.mpl:143-214` so benign claim misses map to idle behavior instead of failure accounting. Real processing failures still flow through `NoteFailed`, `failed_jobs`, and `last_error`, but ordinary claim contention now clears through the idle path and gets logged as a benign claim miss rather than as a worker failure.

I did not touch `compiler/mesh-rt/src/db/repo.rs`; the existing raw-query surface was sufficient.

## Verification

I reran the task-level proof plus the slice-level runtime matrix with the repo-root `.env` loaded into the subprocess environment, which closes the original gate failure where `${DATABASE_URL:?set DATABASE_URL}` aborted before the actual test ran.

Concrete results:

- `e2e_reference_backend_builds` passed.
- `e2e_reference_backend_runtime_starts` passed.
- `e2e_reference_backend_migration_status_and_apply` passed with the real Postgres runtime.
- `e2e_reference_backend_job_flow_updates_health_and_db` passed.
- `e2e_reference_backend_claim_contention_is_not_failure` passed and no longer logged `update_where: no rows matched` in either backend instance.
- The slice-level `e2e_reference_backend_multi_instance_claims_once` command still exits 0 while matching **0 tests**, so it remains a vacuous pass until T03 adds the named exact-once proof.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_builds -- --nocapture` | 0 | ✅ pass | 6.63s |
| 2 | `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_runtime_starts -- --ignored --nocapture` | 0 | ✅ pass | 7.28s |
| 3 | `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_migration_status_and_apply -- --ignored --nocapture` | 0 | ✅ pass | 8.41s |
| 4 | `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_job_flow_updates_health_and_db -- --ignored --nocapture` | 0 | ✅ pass | 9.52s |
| 5 | `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_claim_contention_is_not_failure -- --ignored --nocapture` | 0 | ✅ pass | 9.84s |
| 6 | `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_multi_instance_claims_once -- --ignored --nocapture` *(matched 0 tests; vacuous until T03)* | 0 | ✅ pass | 1.27s |

## Diagnostics

Future agents can inspect this fix by rerunning `e2e_reference_backend_claim_contention_is_not_failure` and checking four surfaces together:

- per-instance `GET /health` for `processed_jobs`, `failed_jobs`, `last_job_id`, and `last_error`
- `GET /jobs/:id` for terminal job state
- direct `jobs` table reads for `status`, `attempts`, `last_error`, and `processed_at`
- backend worker logs, which should no longer contain `update_where: no rows matched` during ordinary shared-DB contention

The key proof points are:

- both instances perform real work
- combined processed count matches the posted job count
- combined failed count stays at zero
- real processing errors would still surface through the existing `failed_jobs`/`last_error` path

## Deviations

- I loaded the repo-root `.env` into the verification subprocess environment because auto-mode shell commands in this worktree do not inherit it automatically. This did not change the slice contract; it only made the real runtime proofs execute instead of failing early on an unset `DATABASE_URL` guard.

## Known Issues

- `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_multi_instance_claims_once -- --ignored --nocapture` still matches 0 tests and therefore remains a vacuous pass. T03 still needs to add the named exact-once proof before the slice can claim full shared-DB multi-instance coverage.

## Files Created/Modified

- `compiler/meshc/tests/e2e_reference_backend.rs` — added the ignored two-instance contention regression, DB/health settling helpers, and a slightly more tolerant socket read timeout for the heavier runtime proof.
- `reference-backend/storage/jobs.mpl` — replaced the read-then-update claim flow with one atomic raw SQL claim-and-return statement using `FOR UPDATE SKIP LOCKED`.
- `reference-backend/jobs/worker.mpl` — reclassified benign claim misses as idle behavior and preserved real processing failures on the existing failure diagnostics path.
- `.gsd/KNOWLEDGE.md` — recorded the repo-root `.env` verification gotcha for future runtime-proof tasks.
- `.gsd/milestones/M028/slices/S02/S02-PLAN.md` — marked T02 complete.
