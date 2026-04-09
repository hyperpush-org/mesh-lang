---
estimated_steps: 4
estimated_files: 6
skills_used:
  - best-practices
  - debug-like-expert
  - test
  - review
---

# T01: Stage a deploy bundle and boring migration path

**Slice:** S04 — Boring Native Deployment
**Milestone:** M028

## Description

Replace the current compile-at-apply deployment story with an artifact-first operator path. This task should stage a runnable `reference-backend` bundle, add a boring deploy-time SQL apply artifact that preserves `_mesh_migrations`, and split deploy probing from the existing local rebuild smoke so later proof can run against a staged artifact instead of a dev-only loop.

## Steps

1. Derive a checked-in deploy SQL artifact at `reference-backend/deploy/reference-backend.up.sql` from `reference-backend/migrations/20260323010000_create_jobs.mpl`, including `_mesh_migrations` tracking so the deploy path stays honest without making the `.mpl` migration non-canonical.
2. Add `reference-backend/scripts/stage-deploy.sh` to build `reference-backend`, stage the native binary plus deploy assets into a caller-provided directory, and print the staged layout in a way an operator or test harness can inspect quickly.
3. Add `reference-backend/scripts/apply-deploy-migrations.sh` and `reference-backend/scripts/deploy-smoke.sh` so a release runner can apply the staged SQL through `psql` and probe a running/staged instance via `/health`, `POST /jobs`, and `GET /jobs/:id` without rebuilding locally.
4. Make every new script fail loudly on missing args, missing files, unset `DATABASE_URL`, or bad HTTP/job states while keeping `DATABASE_URL` redacted from logs and stderr.

## Must-Haves

- [ ] `reference-backend/deploy/reference-backend.up.sql` creates the `jobs` schema and records version `20260323010000` in `_mesh_migrations` idempotently.
- [ ] `reference-backend/scripts/stage-deploy.sh` stages a temp-dir bundle that contains the runnable binary plus the deploy SQL and probe script.
- [ ] `reference-backend/scripts/apply-deploy-migrations.sh` applies the deploy artifact through `psql` without invoking `meshc`.
- [ ] `reference-backend/scripts/deploy-smoke.sh` probes a running/staged binary without rebuilding the app locally.

## Verification

- `tmp_dir="$(mktemp -d)" && bash reference-backend/scripts/stage-deploy.sh "$tmp_dir" && test -x "$tmp_dir/reference-backend" && test -f "$tmp_dir/reference-backend.up.sql" && test -x "$tmp_dir/deploy-smoke.sh"`
- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash reference-backend/scripts/apply-deploy-migrations.sh reference-backend/deploy/reference-backend.up.sql && psql "$DATABASE_URL" -Atqc "select version::text from _mesh_migrations where version = 20260323010000" | rg "20260323010000"`

## Observability Impact

- Signals added/changed: bundle staging, SQL apply, and deploy probe phases become named script outputs instead of implicit manual steps.
- How a future agent inspects this: run the stage/apply scripts directly and inspect `_mesh_migrations`, `jobs`, or the printed staged layout.
- Failure state exposed: missing artifact files, missing env, failed SQL apply, and broken deploy probes fail at the exact stage with actionable stderr.

## Inputs

- `reference-backend/migrations/20260323010000_create_jobs.mpl` — canonical schema and migration version to mirror for boring deploy apply
- `reference-backend/scripts/smoke.sh` — existing local rebuild smoke path to keep distinct from the new deploy probe
- `reference-backend/README.md` — current backend operator surface that later tasks must sync to the new scripts
- `reference-backend/.env.example` — current runtime env contract that the staged bundle must preserve

## Expected Output

- `reference-backend/deploy/reference-backend.up.sql` — deploy-time SQL artifact for the boring migration path
- `reference-backend/scripts/stage-deploy.sh` — bundle staging script for native deployment artifacts
- `reference-backend/scripts/apply-deploy-migrations.sh` — deploy-time SQL apply helper using `psql`
- `reference-backend/scripts/deploy-smoke.sh` — probe-only smoke script for a running or staged deployment
