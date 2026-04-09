---
estimated_steps: 4
estimated_files: 6
skills_used:
  - debug-like-expert
  - test
  - review
---

# T02: Prove staged-artifact deployment in the backend e2e harness

**Slice:** S04 — Boring Native Deployment
**Milestone:** M028

## Description

Turn the new deployment path into the slice’s authoritative operational proof. This task should extend the existing Rust backend harness so it stages the deploy bundle into a temp directory outside the repo root, applies the boring migration artifact, starts the staged binary from that staged location, and proves the same health/job flow that earlier slices used on the dev path.

## Steps

1. Add temp-dir helpers in `compiler/meshc/tests/e2e_reference_backend.rs` that call `reference-backend/scripts/stage-deploy.sh`, run the staged `apply-deploy-migrations.sh`, and start the staged binary without relying on repo-root runtime assets after staging completes.
2. Add an ignored Postgres-backed test named `e2e_reference_backend_deploy_artifact_smoke` that resets the DB, stages the bundle, applies the deploy SQL, starts the staged binary, and runs the deploy smoke flow against the staged instance.
3. Assert `_mesh_migrations` truth, durable `jobs` truth, `/health`, `/jobs/:id`, and log redaction together so deployment proof does not depend on exit codes alone.
4. Keep `compiler/meshc/tests/e2e.rs`’s `e2e_self_contained_binary` as companion proof, but make the new reference-backend test the operational gate for S04.

## Must-Haves

- [ ] `compiler/meshc/tests/e2e_reference_backend.rs` contains one ignored deploy-artifact test that stages the artifact outside the repo root and proves the migrated startup path end to end.
- [ ] The new test verifies health success, job creation/processing success, and `_mesh_migrations` state after the deploy SQL apply step.
- [ ] The staged runtime path proves it does not require `meshc`, source files, or repo-root cwd once the bundle is staged.
- [ ] Startup/test logs stay useful while still redacting `DATABASE_URL`.

## Verification

- `cargo test -p meshc e2e_self_contained_binary -- --nocapture`
- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_deploy_artifact_smoke -- --ignored --nocapture`

## Observability Impact

- Signals added/changed: the backend harness now exposes a named staged-artifact deploy proof instead of only dev-path smoke.
- How a future agent inspects this: rerun `e2e_reference_backend_deploy_artifact_smoke`, then inspect the temp bundle paths, staged binary logs, `_mesh_migrations`, and `jobs` rows the test reports on failure.
- Failure state exposed: deploy regressions localize to staging, SQL apply, startup, HTTP health, or job-processing phases rather than collapsing into a generic failed deploy claim.

## Inputs

- `compiler/meshc/tests/e2e_reference_backend.rs` — canonical backend proof harness to extend
- `compiler/meshc/tests/e2e.rs` — existing self-contained binary proof to preserve as companion evidence
- `reference-backend/scripts/stage-deploy.sh` — staged bundle entrypoint created in T01
- `reference-backend/scripts/apply-deploy-migrations.sh` — deploy-time SQL apply helper created in T01
- `reference-backend/scripts/deploy-smoke.sh` — probe-only deploy smoke helper created in T01
- `reference-backend/deploy/reference-backend.up.sql` — boring deploy migration artifact created in T01

## Expected Output

- `compiler/meshc/tests/e2e_reference_backend.rs` — ignored staged-artifact deployment regression proving the boring deployment path
