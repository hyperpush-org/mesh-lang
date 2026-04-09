---
estimated_steps: 4
estimated_files: 5
skills_used:
  - best-practices
  - review
  - test
---

# T03: Document the operator-facing boring deployment workflow

**Slice:** S04 — Boring Native Deployment
**Milestone:** M028

## Description

Publish the exact boring deployment story where an evaluator will look first: the canonical backend package docs. This task should describe the staged bundle layout, deploy-time SQL apply step, runtime-host contract, and probe-only smoke command using the same names and paths proven by the new scripts and ignored e2e.

## Steps

1. Add a `Boring native deployment` section to `reference-backend/README.md` that spells out build-host prerequisites, staged bundle contents, deploy SQL apply, runtime start, and deploy smoke commands in the verified order.
2. Update `reference-backend/.env.example` if needed so it still matches the staged runtime contract and any optional deploy-smoke overrides introduced by T01.
3. Cross-check the docs against `reference-backend/scripts/stage-deploy.sh`, `reference-backend/scripts/apply-deploy-migrations.sh`, `reference-backend/scripts/deploy-smoke.sh`, and the ignored deploy-artifact e2e so no command or claim is aspirational.
4. Keep the docs package-local and operator-focused; broader website/README promotion remains for S06.

## Must-Haves

- [ ] `reference-backend/README.md` tells the exact verified build/apply/run/smoke story for the boring deployment path.
- [ ] The docs clearly separate build-host requirements from runtime-host requirements and state that the runtime side does not need `meshc` after staging.
- [ ] `reference-backend/.env.example` stays aligned with the runtime env contract used by the staged binary.
- [ ] Any “easier deployment” language is grounded in the concrete artifact and commands shipped in this slice, not vague single-binary marketing.

## Verification

- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_deploy_artifact_smoke -- --ignored --nocapture`
- `rg -n "Boring native deployment|stage-deploy\.sh|apply-deploy-migrations\.sh|deploy-smoke\.sh|runtime host|meshc" reference-backend/README.md`
- `rg -n "^DATABASE_URL=|^PORT=|^JOB_POLL_MS=" reference-backend/.env.example`

## Inputs

- `reference-backend/README.md` — current backend docs to update with the verified operator story
- `reference-backend/.env.example` — runtime env contract to keep synchronized with the staged binary
- `reference-backend/scripts/stage-deploy.sh` — verified bundle staging command to document
- `reference-backend/scripts/apply-deploy-migrations.sh` — verified deploy-time migration command to document
- `reference-backend/scripts/deploy-smoke.sh` — verified deploy probe command to document
- `compiler/meshc/tests/e2e_reference_backend.rs` — ignored deploy-artifact proof that the docs must describe truthfully

## Expected Output

- `reference-backend/README.md` — operator-facing boring deployment runbook tied to the verified scripts and e2e
- `reference-backend/.env.example` — env example aligned with the staged runtime contract

## Observability Impact

- Signals exposed to operators stay package-local and named: `stage-deploy.sh` prints bundle phases and staged paths, `apply-deploy-migrations.sh` prints the SQL artifact path plus migration version recording, and `deploy-smoke.sh` prints health/create/poll/processed phases against the running staged binary.
- Future agents should inspect this task through `reference-backend/README.md`, `reference-backend/.env.example`, the three deploy scripts, and `compiler/meshc/tests/e2e_reference_backend.rs` so docs and deploy proof stay on the same contract.
- Failure state becomes more visible because the README now points directly at the exact stage where deploys fail: bundle creation, missing SQL artifact, `psql` apply, runtime startup, `/health`, or job polling, without requiring `meshc` on the runtime host.
