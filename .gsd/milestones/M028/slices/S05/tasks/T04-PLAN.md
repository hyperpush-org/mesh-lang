---
estimated_steps: 3
estimated_files: 2
skills_used:
  - test
  - best-practices
  - review
---

# T04: Prove whole-process recovery and document the supervision contract

**Slice:** S05 — Supervision, Recovery, and Failure Visibility
**Milestone:** M028

## Description

Close the slice with the highest-value operational proof: the whole backend process can die and come back without leaving durable work stranded, and the resulting supervision/recovery contract is written down in the canonical package docs. This task should keep proof in the existing backend harness and make the new health fields and verification commands discoverable for S06 instead of burying them in test-only knowledge.

## Steps

1. Extend `compiler/meshc/tests/e2e_reference_backend.rs` with a whole-process restart scenario that kills the backend while work is inflight, restarts it, and then proves `/health`, `/jobs/:id`, and direct `jobs` table reads agree on the recovered outcome.
2. Reuse the health/recovery signals produced by T03 so the harness asserts restart counts, exit reasons, or recovery markers instead of only final `processed` state.
3. Update `reference-backend/README.md` with a package-local “Supervision and recovery” section that names the exact ignored test commands and explains how to inspect the new `/health` fields during operator/debug flows.

## Must-Haves

- [ ] The backend harness proves whole-process restart recovery on the real `reference-backend/` entrypoint.
- [ ] The restart proof asserts health visibility and durable DB truth together, not just one final job status.
- [ ] `reference-backend/README.md` documents the verified supervision/recovery commands and health fields.

## Verification

- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_process_restart_recovers_inflight_job -- --ignored --nocapture`
- `rg -n "Supervision and recovery|restart_count|last_exit_reason|recovered_jobs|process restart" reference-backend/README.md`

## Observability Impact

- Signals added/changed: whole-process restart proof validates that the health/recovery contract remains meaningful after backend relaunch, not only worker-internal crashes.
- How a future agent inspects this: run the ignored restart test or read the documented `/health` fields and commands in `reference-backend/README.md`.
- Failure state exposed: process-level restart bugs become visible as disagreement between health JSON, job API state, and direct DB truth.

## Inputs

- `reference-backend/api/health.mpl` — recovery/liveness health contract produced by T03
- `reference-backend/jobs/worker.mpl` — supervised worker + recovery logic the whole-process proof depends on
- `reference-backend/storage/jobs.mpl` — durable abandoned-job recovery semantics produced by T03
- `compiler/meshc/tests/e2e_reference_backend.rs` — backend harness already extended with crash-recovery proofs in T03
- `reference-backend/README.md` — canonical package-local doc surface that needs the verified supervision/recovery contract

## Expected Output

- `compiler/meshc/tests/e2e_reference_backend.rs` — whole-process restart recovery proof tied to `/health`, `/jobs/:id`, and DB truth
- `reference-backend/README.md` — documented supervision/recovery commands and health-field interpretation
