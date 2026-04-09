---
id: T03
parent: S05
milestone: M028
provides:
  - Rewrote the reference-backend runtime toward supervised worker recovery, added local Docker Postgres wiring for this worktree, and captured the remaining Mesh compile blockers for the next unit.
key_files:
  - reference-backend/storage/jobs.mpl
  - reference-backend/jobs/worker.mpl
  - reference-backend/api/health.mpl
  - reference-backend/runtime/registry.mpl
  - reference-backend/main.mpl
  - compiler/meshc/tests/e2e_reference_backend.rs
  - .env
key_decisions:
  - Create a local Docker Postgres instance for this worktree and bind it through the repo-local `.env` instead of waiting on an external `DATABASE_URL`.
  - Move the worker toward a zero-arg supervised child shape with registry lookups and a persistent worker-state service so restart/recovery signals can survive worker crashes.
patterns_established:
  - Mesh parser/typechecker work in this area is sensitive to inline boolean-expression style; flatten control flow and re-run the real `meshc build reference-backend` path after each small change.
  - Crash-after-claim recovery can be proved deterministically by a payload-triggered one-shot worker crash plus health/DB assertions, but the Mesh build must be clean before the Rust harness can exercise it.
observability_surfaces:
  - GET /health
  - GET /jobs/:id
  - compiler/meshc/tests/e2e_reference_backend.rs
  - local Docker Postgres wired through `.env`
duration: 2h
verification_result: partial
completed_at: 2026-03-23 01:25:29 EDT
blocker_discovered: false
---

# T03: Recover abandoned `processing` jobs and classify failure state in health

**Reworked the backend toward supervised recovery and local reproducibility, but stopped at an honest handoff when the real Mesh build still failed on the new worker/storage contract.**

## What Happened

I activated the requested skills, read the slice/task plans plus the prior T01/T02 summaries, and then verified the local reality before editing. The planner snapshot was ahead of the checked-out code: `reference-backend` still booted the worker with the old detached path, `runtime/registry.mpl` only stored the pool, and the existing e2e harness did not yet contain the T03 crash-recovery proofs.

Because `DATABASE_URL` was unset locally, I created a dedicated Docker Postgres instance for this worktree and wrote the resulting local `DATABASE_URL` into the repo-local `.env` so the ignored backend tests could run without waiting on an external secret.

On the code side, I rewrote the following areas toward the intended T03 shape:

- `reference-backend/storage/jobs.mpl`
  - added a `RecoveryResult` shape and recovery SQL for reclaiming abandoned `processing` rows back to `pending`
- `reference-backend/runtime/registry.mpl`
  - expanded the runtime registry to carry both `pool` and `poll_ms`
- `reference-backend/main.mpl`
  - switched runtime boot to the new registry shape and the updated worker-start entrypoint
- `reference-backend/jobs/worker.mpl`
  - replaced the old simple detached loop with a persistent worker-state service, a zero-arg supervised child direction, restart/recovery bookkeeping, and payload-triggered crash injection intended for the new ignored tests
- `reference-backend/api/health.mpl`
  - expanded `/health` toward explicit restart/recovery fields instead of only the old flat counters
- `compiler/meshc/tests/e2e_reference_backend.rs`
  - added focused ignored tests for `e2e_reference_backend_worker_crash_recovers_job` and `e2e_reference_backend_worker_restart_is_visible_in_health`

I then iterated on the real verification command instead of guessing. That flushed out a sequence of real blockers:

1. the first test run failed before the code path because the reused local Docker Postgres credentials did not match the worktree `.env`; I reset that local container onto a clean known-good port and reran;
2. the subsequent runs moved past infra and into real Mesh compile errors in the new backend modules;
3. I fixed the early parser issues caused by the first pass at `/health` control flow, but I stopped when the remaining build failures became a tight compile/debug loop and the context-budget warning required wrap-up.

At stop time, the remaining blockers were no longer infra or planner drift; they were concrete compile-time issues in the edited Mesh code.

## Verification

I verified the local runtime prerequisite by creating a dedicated Docker Postgres for this worktree and wiring `.env` so the ignored backend tests can run locally.

I then repeatedly reran the task’s minimal real proof command:

- `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_crash_recovers_job -- --ignored --nocapture`

That command progressed from local-database auth failure into real `meshc build reference-backend` errors. The final run still failed at build time, so I did **not** run the second ignored health-visibility command afterward.

The latest remaining build blockers surfaced by the final rerun were:

- `reference-backend/jobs/worker.mpl` imports `reclaim_processing_jobs`, but the compiler still reported it as not exported by `Storage.Jobs`; the next unit should verify the final saved shape of `reference-backend/storage/jobs.mpl` and the import/export contract together rather than assuming the write landed as intended;
- `reference-backend/jobs/worker.mpl` still has a type-check issue around the `JobWorkerState.note_processed(...)` / related note helpers where the checker reported `expected Int, found ()`;
- `do_crash(0) = 0` is intentionally one-shot crash injection, but the checker currently flags it as a non-exhaustive match because it is called with `42`; the next unit should replace that trick with a Mesh crash primitive/pattern the typechecker accepts, or make the function exhaustive while still crashing deterministically;
- the undefined-name errors for `reclaim_processing_jobs` / `recovery_result` in the later worker lines should be rechecked after the export/import fix because some of that downstream noise may be cascading.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_crash_recovers_job -- --ignored --nocapture` | 101 | ❌ fail | 6.50s |

The command above was rerun multiple times during implementation. The final rerun still failed at `meshc build reference-backend`, so the second ignored T03 verification command was not started.

## Diagnostics

Use this exact resume path next:

1. Keep the local Docker-backed `.env` in place for this worktree and rerun only the single focused command until it builds:
   - `set -a && source ./.env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_crash_recovers_job -- --ignored --nocapture`
2. Start with `reference-backend/storage/jobs.mpl` and `reference-backend/jobs/worker.mpl` together:
   - verify `pub fn reclaim_processing_jobs(...)` is really present on disk and that `worker.mpl` imports it exactly as exported;
   - inspect the `note_processed` / note helper calls around the type error site and align them with the actual return type expected by Mesh service casts;
   - replace the current `do_crash(0)=0` crash-injection trick with a typechecker-safe deterministic crash path.
3. Only after the crash-recovery test builds and passes, run the second proof:
   - `set -a && source ./.env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_restart_is_visible_in_health -- --ignored --nocapture`
4. If both pass, then update `reference-backend/README.md` later in T04 rather than spending more T03 context budget on docs now.

## Deviations

- I had to finish the missing T02-adjacent local prerequisites (`runtime/registry.mpl` and `main.mpl` worker boot wiring) because the checked-out worktree still used the detached worker path the T03 plan assumed was already gone.
- I created a local Docker Postgres instance and repo-local `.env` because the task’s real verification path needed `DATABASE_URL` and none was set in the shell.
- I stopped before the second ignored T03 verification command because the first proof still failed at Mesh build time; starting the second test would not have produced new signal.

## Known Issues

- T03 is not complete yet.
- The current edited backend files still do not compile cleanly under `meshc build reference-backend`.
- The task state should remain incomplete until the focused crash-recovery command passes and then the health-visibility proof passes afterward.

## Files Created/Modified

- `.env` — repo-local local-development `DATABASE_URL` pointing at the new Docker Postgres instance for this worktree.
- `reference-backend/storage/jobs.mpl` — attempted abandoned-`processing` recovery helper plus `RecoveryResult` shape.
- `reference-backend/runtime/registry.mpl` — expanded registry contract to carry `poll_ms` alongside the pool.
- `reference-backend/main.mpl` — rewired runtime startup to the new registry/worker shape.
- `reference-backend/jobs/worker.mpl` — substantial but not-yet-compiling supervised worker/recovery rewrite.
- `reference-backend/api/health.mpl` — expanded health payload toward restart/recovery visibility.
- `compiler/meshc/tests/e2e_reference_backend.rs` — added ignored crash-recovery and health-visibility test coverage.
