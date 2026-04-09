---
estimated_steps: 3
estimated_files: 3
skills_used:
  - debug-like-expert
  - test
  - review
  - lint
---

# T02: Stabilize recovery health and restart visibility

**Slice:** S07 — Recovery Proof Closure
**Milestone:** M028

## Description

Once restart ownership is real, S07 still needs a trustworthy inspection surface. This task hardens `/health` around real lifecycle evidence so the degraded/recovering window is observable, restart metadata stays internally consistent, and the slower 500ms recovery proof no longer relies on fragile timing or corrupted fields.

## Steps

1. In `reference-backend/api/health.mpl`, derive worker liveness from lifecycle evidence such as `last_status` plus `tick_age_ms`, so stale workers cannot look healthy forever and recovery-active windows settle deterministically.
2. In `reference-backend/jobs/worker.mpl`, keep `boot_id`, `started_at`, `last_exit_reason`, `last_recovery_at`, `last_recovery_job_id`, and `last_recovery_count` coherent across restart and post-recovery transitions.
3. Update `compiler/meshc/tests/e2e_reference_backend.rs` so `e2e_reference_backend_worker_restart_is_visible_in_health` proves the degraded recovery window, pending-row visibility during reclaim, and healthy settlement after resumed processing at the slower poll cadence.

## Must-Haves

- [ ] `/health` can distinguish healthy, recovering, failed, and stale worker states without relying only on raw status strings.
- [ ] Recovery metadata fields remain coherent across the slower restart-visibility proof; no null/corrupted `boot_id`, `started_at`, or mismatched recovery fields.
- [ ] `e2e_reference_backend_worker_restart_is_visible_in_health` passes reliably and ends with healthy worker state after recovery completes.

## Verification

- `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_restart_is_visible_in_health -- --ignored --nocapture`

## Observability Impact

- Signals added/changed: `/health` liveness, `tick_age_ms`, `recovery_active`, and restart/recovery metadata become authoritative for diagnosing recovery-state drift.
- How a future agent inspects this: rerun the ignored restart-visibility test and inspect the last `/health` payload plus the corresponding `jobs` row state.
- Failure state exposed: stale worker ticks, corrupted restart metadata, or missing degraded/recovering windows fail through one named health-oriented test instead of log archaeology.

## Inputs

- `reference-backend/api/health.mpl` — current health payload and liveness classification rules
- `reference-backend/jobs/worker.mpl` — worker state transitions and recovery metadata bookkeeping from T01
- `compiler/meshc/tests/e2e_reference_backend.rs` — restart-visibility proof that must become stable at slower poll cadence

## Expected Output

- `reference-backend/api/health.mpl` — tick-age-aware recovery/health classification
- `reference-backend/jobs/worker.mpl` — coherent worker lifecycle metadata across recovery
- `compiler/meshc/tests/e2e_reference_backend.rs` — stable restart-visibility proof with explicit degraded→healthy assertions
