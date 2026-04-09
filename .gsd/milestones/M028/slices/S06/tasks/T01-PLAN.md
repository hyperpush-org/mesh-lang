---
estimated_steps: 4
estimated_files: 6
skills_used:
  - debug-like-expert
  - test
  - review
  - lint
---

# T01: Re-green the `reference-backend` proof baseline and recovery gates

**Slice:** S06 — Honest Production Proof and Documentation
**Milestone:** M028

## Description

Before S06 can promote any public backend story, the real `reference-backend/` proof target has to become truthful again. This task is the carry-forward repair from S05: finish the supervised worker recovery path, make `/health` and the ignored e2e proofs agree on restart/recovery behavior, and restore the named build/fmt/test gates that the rest of the slice will cite publicly.

## Steps

1. Resume from the current S05 carry-forward state in `reference-backend/jobs/worker.mpl`, `reference-backend/storage/jobs.mpl`, `reference-backend/main.mpl`, `reference-backend/runtime/registry.mpl`, and `reference-backend/api/health.mpl`; finish the cooperative supervised-exit path so worker crash simulation triggers restart/recovery instead of killing the whole backend process.
2. Keep the durable recovery contract honest: abandoned `processing` rows must be reclaimed, `/health` must surface restart/recovery metadata (`status`, `restart_count`, `last_exit_reason`, `recovered_jobs`, liveness), and `GET /jobs/:id` / direct DB truth must continue to agree with the worker state.
3. Tighten or finish `compiler/meshc/tests/e2e_reference_backend.rs` so the named build-only and ignored recovery/restart tests are the authoritative regression surface for this repaired backend path.
4. Rerun the backend truth gates until they are green as written: build, fmt check, project tests, build-only e2e, worker-crash recovery, restart visibility, and whole-process restart recovery.

## Must-Haves

- [ ] `cargo run -p meshc -- build reference-backend`, `cargo run -p meshc -- fmt --check reference-backend`, and `cargo run -p meshc -- test reference-backend` all pass again from the repo root.
- [ ] `compiler/meshc/tests/e2e_reference_backend.rs` proves worker crash recovery, restart visibility in `/health`, and whole-process recovery on the real `reference-backend/` path.
- [ ] `/health`, `GET /jobs/:id`, and the `jobs` table agree on recovered/restarted job state without requiring log-only interpretation.
- [ ] The repaired backend path remains safe to document publicly because it no longer dies on the crash-after-claim proof path.

## Verification

- `cargo run -p meshc -- build reference-backend`
- `cargo run -p meshc -- fmt --check reference-backend`
- `cargo run -p meshc -- test reference-backend`
- `cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_builds -- --nocapture`
- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_crash_recovers_job -- --ignored --nocapture`
- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_restart_is_visible_in_health -- --ignored --nocapture`
- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_process_restart_recovers_inflight_job -- --ignored --nocapture`

## Observability Impact

- Signals added/changed: supervised worker `status`, `restart_count`, `last_exit_reason`, `recovered_jobs`, and recovery-phase liveness stay visible in `/health` and the ignored Rust harness.
- How a future agent inspects this: rerun the named `e2e_reference_backend` tests, inspect `GET /health`, fetch `GET /jobs/:id`, and query `jobs` directly.
- Failure state exposed: broken build/fmt/test gates, missing restart visibility, and stranded `processing` rows fail through one named command instead of a vague “docs feel dishonest” symptom.

## Inputs

- `reference-backend/jobs/worker.mpl` — current supervised worker implementation with the unfinished cooperative-exit path
- `reference-backend/storage/jobs.mpl` — current abandoned-job recovery and durable row mutation logic
- `reference-backend/api/health.mpl` — backend health contract that must expose restart/recovery truth
- `reference-backend/main.mpl` — backend startup wiring for registry, worker, and HTTP server
- `reference-backend/runtime/registry.mpl` — registry contract that carries runtime worker dependencies
- `compiler/meshc/tests/e2e_reference_backend.rs` — authoritative backend proof harness to repair and extend

## Expected Output

- `reference-backend/jobs/worker.mpl` — working supervised worker recovery path that exits cooperatively and restarts cleanly
- `reference-backend/storage/jobs.mpl` — finalized durable recovery helpers for abandoned `processing` jobs
- `reference-backend/api/health.mpl` — truthful restart/recovery health payload
- `reference-backend/main.mpl` — stable startup wiring for the repaired supervised backend path
- `reference-backend/runtime/registry.mpl` — runtime registry aligned with the repaired worker boot contract
- `compiler/meshc/tests/e2e_reference_backend.rs` — passing build and recovery proofs for the canonical backend path
