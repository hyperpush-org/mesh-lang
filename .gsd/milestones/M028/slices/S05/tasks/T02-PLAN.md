---
estimated_steps: 4
estimated_files: 5
skills_used:
  - debug-like-expert
  - test
  - best-practices
  - review
---

# T02: Wire `reference-backend` worker startup under supervision and expose restart bookkeeping

**Slice:** S05 — Supervision, Recovery, and Failure Visibility
**Milestone:** M028

## Description

Refactor `reference-backend/` so the worker can be started as a real supervised child instead of a detached `spawn(...)` with captured startup args. The task should move runtime dependencies behind registry/service lookups, start the worker from a supervision-friendly path in `main.mpl`, and expose enough worker identity/restart bookkeeping in `/health` to prove the state service still reflects the actual worker process.

## Steps

1. Build on the repaired source-level supervisor contract from T01 and reshape worker startup so the supervised child can bootstrap from package-local runtime lookups instead of captured `pool` / `worker_state` / `poll_ms` args.
2. Update `reference-backend/runtime/registry.mpl` and `reference-backend/jobs/worker.mpl` so the worker can resolve its runtime dependencies and record supervised worker identity plus restart bookkeeping in durable in-memory state.
3. Change `reference-backend/main.mpl` to start the worker through the new supervision path rather than a plain `spawn(...)`, while preserving the existing startup env contract and health endpoint behavior.
4. Extend `compiler/meshc/tests/e2e_reference_backend.rs` with a focused ignored startup proof that asserts the backend boots with the supervised worker metadata exposed through `/health`.

## Must-Haves

- [ ] `reference-backend/main.mpl` no longer starts the worker with a plain detached `spawn(...)` path.
- [ ] `reference-backend/runtime/registry.mpl` exposes the runtime data the supervised child needs without reintroducing broad app-global state.
- [ ] `reference-backend/jobs/worker.mpl` records worker identity/restart bookkeeping that can distinguish a live worker from stale state.
- [ ] `/health` exposes the new supervision metadata, and the backend harness asserts it on a real Postgres-backed startup path.

## Verification

- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_supervision_starts -- --ignored --nocapture`

## Observability Impact

- Signals added/changed: worker PID/identity, restart count, and startup/restart metadata become part of the worker state/health contract.
- How a future agent inspects this: start `reference-backend`, call `GET /health`, and run the focused startup proof in `compiler/meshc/tests/e2e_reference_backend.rs`.
- Failure state exposed: a dead or never-started worker can no longer hide behind a still-running bookkeeping service with stale counters alone.

## Inputs

- `compiler/meshc/tests/e2e_supervisors.rs` — source-level supervisor contract T01 strengthens before backend wiring relies on it
- `reference-backend/main.mpl` — current unsupervised backend startup root
- `reference-backend/runtime/registry.mpl` — current runtime lookup seam for backend dependencies
- `reference-backend/jobs/worker.mpl` — current detached worker/state-service implementation
- `reference-backend/api/health.mpl` — current worker-health JSON surface that needs supervision metadata
- `compiler/meshc/tests/e2e_reference_backend.rs` — authoritative backend harness this task must extend instead of inventing a new proof path

## Expected Output

- `reference-backend/main.mpl` — backend startup wired through the worker supervision path
- `reference-backend/runtime/registry.mpl` — runtime lookup surface that a supervised child can bootstrap from
- `reference-backend/jobs/worker.mpl` — supervised worker bootstrap plus worker identity/restart bookkeeping
- `reference-backend/api/health.mpl` — health contract exposing supervised worker metadata
- `compiler/meshc/tests/e2e_reference_backend.rs` — ignored startup proof for supervised worker boot metadata
