---
id: T03
parent: S02
milestone: M028
provides:
  - A named multi-instance exact-once regression that proves shared-DB reference-backend processing through DB truth, cross-instance job HTTP reads, healthy /health failure signals, and per-instance worker participation logs
key_files:
  - compiler/meshc/tests/e2e_reference_backend.rs
  - reference-backend/jobs/worker.mpl
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M028/slices/S02/S02-PLAN.md
key_decisions:
  - Kept the shared-DB proof in the existing Rust harness and treated `/health.failed_jobs` plus `/health.last_error` as the stable contention signal, while proving exact-once and cross-instance participation through DB rows, `/jobs/:id`, and per-instance processed-job logs
patterns_established:
  - Multi-instance reference-backend exact-once proof can alternate job creation across both instances, wait for direct `jobs` table truth, cross-check each row through the opposite instance's `/jobs/:id`, and confirm each worker participated via local processed-job logs
observability_surfaces:
  - GET /health for `failed_jobs`, `last_error`, and healthy worker status; GET /jobs/:id; direct `jobs` table reads; per-instance `Job worker processed id=` logs
duration: 2h 40m
verification_result: passed
completed_at: 2026-03-23T18:05:04Z
blocker_discovered: false
---

# T03: Prove multi-instance exact-once processing on the shared database

**Added a named two-instance exact-once reference-backend regression with DB, HTTP, health, and per-instance worker proof.**

## What Happened

I extended `compiler/meshc/tests/e2e_reference_backend.rs` with a reusable two-process helper, stable shutdown/log capture, and a shared `assert_reference_backend_multi_instance_exact_once(...)` path that now powers both the existing contention regression and the new named slice gate, `e2e_reference_backend_multi_instance_claims_once`.

The new proof starts two real `reference-backend` processes on unique ports against the same Postgres database, alternates job creation across both HTTP APIs, waits for the `jobs` table to prove every row is `processed` exactly once with `attempts = 1` and no `failed` state, then cross-checks each row through `/jobs/:id` on the opposite backend instance so the HTTP view is forced to agree with DB truth.

I also tightened process cleanup and log handling so the multi-instance assertions survive failures without leaving stale listeners behind. The harness now captures per-instance logs as structured outputs instead of raw tuples, which made it straightforward to assert that both backend processes actually processed work and that neither emitted the old `update_where: no rows matched` claim-race failure signal.

While stabilizing the new proof, I found that the original high-chatter harness shape could turn the multi-instance gate into a false negative by over-trusting `/health.processed_jobs` and `/health.last_job_id` as exact totals under heavier shared-DB polling. I kept `/health` in the proof for the signal the slice contract actually needs there — `failed_jobs == 0`, `last_error == null`, and healthy worker status under contention — and moved exact-once/worker-participation proof to the more reliable surfaces: direct DB reads, `/jobs/:id`, and per-instance `Job worker processed id=` logs.

To reduce unnecessary worker-state churn on the backend side, I removed the redundant per-iteration tick log/state update from `reference-backend/jobs/worker.mpl`. Idle/claimed/processed/failed transitions still update state and keep `/health` useful, but the worker no longer emits an extra state write before every real transition.

## Verification

I ran the full slice verification matrix against the real Postgres database from the repo-root `.env`, and every required command passed.

In addition to the slice gate, I reran the older `e2e_reference_backend_claim_contention_is_not_failure` alias after refactoring the shared helper; it also passed, confirming the T02 regression still works on top of the new exact-once helper.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_builds -- --nocapture` | 0 | ✅ pass | 7.70s |
| 2 | `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_runtime_starts -- --ignored --nocapture` | 0 | ✅ pass | 8.17s |
| 3 | `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_migration_status_and_apply -- --ignored --nocapture` | 0 | ✅ pass | 9.90s |
| 4 | `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_job_flow_updates_health_and_db -- --ignored --nocapture` | 0 | ✅ pass | 10.32s |
| 5 | `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_multi_instance_claims_once -- --ignored --nocapture` | 0 | ✅ pass | 11.57s |

## Diagnostics

Future agents can inspect this proof by rerunning `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_multi_instance_claims_once -- --ignored --nocapture` with the repo-root `.env` loaded, then checking four surfaces together:

- direct `jobs` table reads for `status`, `attempts`, `last_error`, `processed_at`, and payload truth
- alternating cross-instance `GET /jobs/:id` reads to confirm HTTP matches the shared DB state
- per-instance `GET /health` for `failed_jobs`, `last_error`, and healthy worker status under contention
- per-instance backend logs for `Job worker processed id=` participation and absence of `update_where: no rows matched`

The older `e2e_reference_backend_claim_contention_is_not_failure` alias still passes and exercises the same helper path if a future agent wants a narrower failure-counter regression.

## Deviations

I did not keep `processed_jobs` total and `last_job_id` as the exact multi-instance proof source inside `/health`. During execution, those fields proved unstable under the noisier shared-DB two-instance harness even when DB truth and `/jobs/:id` already showed the correct exact-once outcome. I narrowed the `/health` assertion to the stable false-failure signal (`failed_jobs == 0`, `last_error == null`, healthy status) and used DB + `/jobs/:id` + per-instance processed-job logs for the exact-once and worker-participation proof.

## Known Issues

- Under heavier multi-instance polling, `/health.processed_jobs` and `/health.last_job_id` are not yet strong enough to use as the sole exact-once proof signal. The harness now avoids over-claiming on that surface and documents the safer inspection pattern in `.gsd/KNOWLEDGE.md`.

## Files Created/Modified

- `compiler/meshc/tests/e2e_reference_backend.rs` — added reusable two-process helpers, the named `e2e_reference_backend_multi_instance_claims_once` regression, direct DB payload checks, opposite-instance `/jobs/:id` cross-checks, and log-backed worker participation assertions.
- `reference-backend/jobs/worker.mpl` — removed the redundant per-iteration tick state/log update so the worker only records meaningful idle/claim/process/failure transitions.
- `.gsd/KNOWLEDGE.md` — recorded the stable multi-instance inspection pattern for `/health`, DB, `/jobs/:id`, and per-instance processed-job logs.
- `.gsd/milestones/M028/slices/S02/S02-PLAN.md` — marked T03 complete.
