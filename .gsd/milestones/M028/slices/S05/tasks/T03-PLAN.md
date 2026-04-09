---
estimated_steps: 4
estimated_files: 4
skills_used:
  - debug-like-expert
  - test
  - best-practices
  - review
---

# T03: Recover abandoned `processing` jobs and classify failure state in health

**Slice:** S05 — Supervision, Recovery, and Failure Visibility
**Milestone:** M028

## Description

Close S05’s core trust gap: a worker crash after claiming a job currently leaves that durable row stuck in `processing`, while `/health` can still look green. This task should add storage/worker recovery logic that reclaims abandoned inflight jobs during restart or boot, preserve honest attempt/error semantics, and upgrade `/health` from a flat “ok + counters” surface into an explicit liveness/recovery/failure contract.

## Steps

1. Build on T02’s supervised worker bootstrap and add storage-level support in `reference-backend/storage/jobs.mpl` for finding and reclaiming abandoned `processing` rows without regressing the exact-once golden path already proved in S02.
2. Update `reference-backend/jobs/worker.mpl` so supervised boot/restart runs a recovery pass before normal polling, tracks recovery activity, and preserves honest attempts / last-error behavior for retried or failed jobs.
3. Extend `reference-backend/api/health.mpl` to report explicit liveness and recovery fields such as restart count, last exit reason, recovery activity, and a non-green worker state when the backend is not actually healthy.
4. Add focused ignored e2e coverage in `compiler/meshc/tests/e2e_reference_backend.rs` for worker crash recovery and health failure visibility, cross-checking `/health`, `/jobs/:id`, and direct `jobs` table truth.

## Must-Haves

- [ ] `reference-backend/storage/jobs.mpl` can reclaim abandoned `processing` work instead of leaving it stranded forever.
- [ ] `reference-backend/jobs/worker.mpl` runs recovery before normal polling and records recovery/failure metadata honestly.
- [ ] `reference-backend/api/health.mpl` reports explicit liveness/recovery/failure state instead of unconditional top-level green behavior.
- [ ] Backend e2e tests prove both crash recovery and visible health-state degradation/recovery on the real Postgres-backed path.

## Verification

- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_crash_recovers_job -- --ignored --nocapture`
- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_restart_is_visible_in_health -- --ignored --nocapture`

## Observability Impact

- Signals added/changed: abandoned-job recovery counts/markers, restart count, last exit reason, and explicit worker liveness classification become health-visible.
- How a future agent inspects this: query `GET /health`, `GET /jobs/:id`, and the `jobs` table, or rerun the focused ignored e2e tests.
- Failure state exposed: crash-after-claim, restart recovery, and hard-failed worker states become visible without depending on ad hoc log reading.

## Inputs

- `reference-backend/runtime/registry.mpl` — supervised worker dependency seam produced by T02
- `reference-backend/jobs/worker.mpl` — supervised worker implementation that now needs recovery logic
- `reference-backend/api/health.mpl` — health contract already carrying supervision metadata from T02
- `compiler/meshc/tests/e2e_reference_backend.rs` — backend harness T02 already extended with supervised startup proof
- `reference-backend/storage/jobs.mpl` — current claim/process storage contract with the abandoned-`processing` gap

## Expected Output

- `reference-backend/storage/jobs.mpl` — abandoned-`processing` recovery/reclaim helpers
- `reference-backend/jobs/worker.mpl` — recovery-before-polling worker logic and recovery bookkeeping
- `reference-backend/api/health.mpl` — explicit liveness/recovery/failure health fields
- `compiler/meshc/tests/e2e_reference_backend.rs` — ignored crash-recovery and health-visibility proofs
