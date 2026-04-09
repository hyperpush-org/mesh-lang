---
estimated_steps: 4
estimated_files: 5
skills_used:
  - debug-like-expert
  - test
  - review
  - lint
---

# T01: Move recovery onto the real supervisor restart boundary

**Slice:** S07 — Recovery Proof Closure
**Milestone:** M028

## Description

The current `reference-backend/` recovery story is failing for the right reason: the crashing worker is pretending to be the restarted worker. This task fixes the root cause first. It must move boot/reclaim work behind a real child restart, replace blanket `processing`-row reclaim with a stale/lease-style contract that is safe on a shared database, and keep the canonical Mesh migration plus staged deploy SQL artifact aligned with whatever recovery query/index shape that contract needs.

## Steps

1. In `reference-backend/jobs/worker.mpl`, remove the fake in-process reboot path from `crash_after_claim(...)`, keep only exit-intent recording on the crashing worker, and make `supervised_job_worker()` / `handle_worker_pool_open(...)` the only place that performs boot bookkeeping and abandoned-job recovery.
2. In `reference-backend/storage/jobs.mpl`, replace blanket `status='processing'` reclaim with a stale/lease-style recovery query that only requeues provably abandoned rows; derive or thread the threshold explicitly so the contract is deterministic and testable.
3. If the new reclaim contract needs SQL/index support, update both `reference-backend/migrations/20260323010000_create_jobs.mpl` and `reference-backend/deploy/reference-backend.up.sql` so S04’s artifact-first runtime path stays truthful.
4. Tighten `compiler/meshc/tests/e2e_reference_backend.rs` around the worker-crash proof so it asserts the new restart-owned reclaim behavior, then rerun build/fmt plus the worker-crash and migration/deploy regression gates until they pass.

## Must-Haves

- [ ] `crash_after_claim(...)` no longer calls boot/reclaim helpers on behalf of the restarted worker.
- [ ] `reclaim_processing_jobs(...)` only recovers stale or otherwise abandoned `processing` rows instead of stealing live work from healthy instances.
- [ ] `reference-backend/migrations/20260323010000_create_jobs.mpl` and `reference-backend/deploy/reference-backend.up.sql` stay in sync with the recovery/storage contract.
- [ ] `e2e_reference_backend_worker_crash_recovers_job` passes with the recovered job finishing after a real restart boundary, not a simulated one.

## Verification

- `cargo run -p meshc -- build reference-backend`
- `cargo run -p meshc -- fmt --check reference-backend`
- `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_crash_recovers_job -- --ignored --nocapture`
- `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_migration_status_and_apply -- --ignored --nocapture`
- `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_deploy_artifact_smoke -- --ignored --nocapture`

## Observability Impact

- Signals added/changed: worker boot/restart and recovery are emitted only from the restarted child path, and abandoned-job recovery becomes visible as a real reclaim event instead of a fake in-process mutation.
- How a future agent inspects this: rerun `e2e_reference_backend_worker_crash_recovers_job`, inspect `GET /health`, and query `jobs` plus `_mesh_migrations` directly when migration/deploy regressions fire.
- Failure state exposed: restart-owned reclaim failures show up as a stuck `processing`/`pending` row, bad restart metadata, or broken staged deploy verification instead of a vague recovery claim.

## Inputs

- `reference-backend/jobs/worker.mpl` — current crash path that still simulates boot/recovery inside the crashing worker
- `reference-backend/storage/jobs.mpl` — current blanket abandoned-job reclaim logic
- `reference-backend/migrations/20260323010000_create_jobs.mpl` — canonical Mesh migration that may need recovery-query/index support
- `reference-backend/deploy/reference-backend.up.sql` — staged SQL artifact that must stay aligned with the migration contract
- `compiler/meshc/tests/e2e_reference_backend.rs` — authoritative backend recovery harness to retarget at the real restart boundary

## Expected Output

- `reference-backend/jobs/worker.mpl` — real supervisor-owned crash/restart recovery flow
- `reference-backend/storage/jobs.mpl` — stale/lease-style abandoned-job reclaim helpers
- `reference-backend/migrations/20260323010000_create_jobs.mpl` — recovery-aware canonical migration/index contract
- `reference-backend/deploy/reference-backend.up.sql` — recovery-aware staged deploy SQL artifact
- `compiler/meshc/tests/e2e_reference_backend.rs` — passing worker-crash proof aligned with the new restart/reclaim semantics
