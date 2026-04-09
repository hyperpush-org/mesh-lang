---
estimated_steps: 4
estimated_files: 3
skills_used:
  - debug-like-expert
  - test
  - review
  - lint
---

# T03: Add deterministic whole-process restart recovery proof

**Slice:** S07 — Recovery Proof Closure
**Milestone:** M028

## Description

S07 is only closed when the backend survives more than a child crash. This task adds a deterministic in-flight window that lets the canonical Rust harness kill the entire `reference-backend` process while a job is actively claimed, restart the process, and prove the reclaimed job finishes exactly once on the same recovery-aware runtime contract established in T01-T02.

## Steps

1. Add a small deterministic hold-after-claim seam in `reference-backend/jobs/worker.mpl` so the harness can create a job, wait until it is definitely `processing`, and keep it in flight long enough to kill the whole backend process without introducing a second test surface.
2. Update any recovery/storage hooks in `reference-backend/storage/jobs.mpl` that the new in-flight process-restart proof depends on, keeping the stale/lease reclaim semantics from T01 intact.
3. Implement `e2e_reference_backend_process_restart_recovers_inflight_job` in `compiler/meshc/tests/e2e_reference_backend.rs` using the existing canonical helpers rather than a new ad hoc script or harness file.
4. Rerun the full slice gate set so build, fmt, project tests, worker-crash recovery, restart visibility, process restart recovery, migration status/apply, and staged deploy smoke all agree on the final recovery contract.

## Must-Haves

- [ ] The harness can deterministically observe an in-flight `processing` row before killing the backend process.
- [ ] `e2e_reference_backend_process_restart_recovers_inflight_job` exists in `compiler/meshc/tests/e2e_reference_backend.rs` and passes on the real backend path.
- [ ] The final slice gate set is green, including migration and staged deploy regressions if storage/recovery SQL changed.

## Verification

- `cargo run -p meshc -- build reference-backend`
- `cargo run -p meshc -- fmt --check reference-backend`
- `cargo run -p meshc -- test reference-backend`
- `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_crash_recovers_job -- --ignored --nocapture`
- `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_restart_is_visible_in_health -- --ignored --nocapture`
- `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_process_restart_recovers_inflight_job -- --ignored --nocapture`
- `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_migration_status_and_apply -- --ignored --nocapture`
- `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_deploy_artifact_smoke -- --ignored --nocapture`

## Observability Impact

- Signals added/changed: the process-restart proof adds a deterministic in-flight window and final recovery assertions that tie worker restart metadata, job API state, and durable DB truth together.
- How a future agent inspects this: rerun `e2e_reference_backend_process_restart_recovers_inflight_job`, inspect `/health`, inspect `GET /jobs/:id`, and compare against the `jobs` table plus staged deploy smoke output.
- Failure state exposed: whole-process restart regressions fail as an explicit in-flight reclaim/progress mismatch instead of a broad “backend recovery feels untrusted” symptom.

## Inputs

- `reference-backend/jobs/worker.mpl` — recovery-aware worker loop from T01-T02 where the deterministic in-flight seam belongs
- `reference-backend/storage/jobs.mpl` — stale/lease recovery helpers that whole-process restart must reuse honestly
- `compiler/meshc/tests/e2e_reference_backend.rs` — canonical backend harness that must gain the missing process-restart proof

## Expected Output

- `reference-backend/jobs/worker.mpl` — deterministic in-flight hold seam for process-restart proof
- `reference-backend/storage/jobs.mpl` — finalized recovery helpers compatible with whole-process restart closure
- `compiler/meshc/tests/e2e_reference_backend.rs` — passing whole-process restart recovery proof and final slice gate coverage
