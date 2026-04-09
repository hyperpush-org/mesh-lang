---
id: T03
parent: S08
milestone: M034
provides: []
requires: []
affects: []
key_files: ["packages-website/Dockerfile", ".tmp/m034-s08/deploy-services-local-build.log", ".tmp/m034-s08/deploy-services-local-build-prechange.log", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M034/slices/S08/tasks/T03-SUMMARY.md"]
key_decisions: ["Keep the packages-website runtime image on the builder stage's pruned dependency tree instead of running a second `npm install --omit=dev --ignore-scripts` in the runtime stage.", "Leave `.github/workflows/deploy-services.yml` unchanged because the hosted contract was already truthful; the break was inside the image build, not the workflow ownership or health-check wiring."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Confirmed the old failure path by checking that `.tmp/m034-s08/deploy-services-local-build-prechange.log` contains the runtime-stage `npm install --omit=dev --ignore-scripts` and `ERESOLVE` markers while the post-fix `.tmp/m034-s08/deploy-services-local-build.log` does not. Ran the task-plan Docker build command and it completed successfully. Ran `bash scripts/verify-m034-s05-workflows.sh` and it passed, confirming the deploy-services workflow contract stayed truthful after the image change."
completed_at: 2026-03-27T16:43:47.307Z
blocker_discovered: false
---

# T03: Reworked the packages-website Docker image to prune builder dependencies instead of reinstalling runtime packages, eliminating the hosted ERESOLVE failure path.

> Reworked the packages-website Docker image to prune builder dependencies instead of reinstalling runtime packages, eliminating the hosted ERESOLVE failure path.

## What Happened
---
id: T03
parent: S08
milestone: M034
key_files:
  - packages-website/Dockerfile
  - .tmp/m034-s08/deploy-services-local-build.log
  - .tmp/m034-s08/deploy-services-local-build-prechange.log
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M034/slices/S08/tasks/T03-SUMMARY.md
key_decisions:
  - Keep the packages-website runtime image on the builder stage's pruned dependency tree instead of running a second `npm install --omit=dev --ignore-scripts` in the runtime stage.
  - Leave `.github/workflows/deploy-services.yml` unchanged because the hosted contract was already truthful; the break was inside the image build, not the workflow ownership or health-check wiring.
duration: ""
verification_result: passed
completed_at: 2026-03-27T16:43:47.308Z
blocker_discovered: false
---

# T03: Reworked the packages-website Docker image to prune builder dependencies instead of reinstalling runtime packages, eliminating the hosted ERESOLVE failure path.

**Reworked the packages-website Docker image to prune builder dependencies instead of reinstalling runtime packages, eliminating the hosted ERESOLVE failure path.**

## What Happened

Used the saved hosted `deploy-services.yml` failure log and the existing `packages-website/Dockerfile` to reproduce the break locally: the runtime-stage `npm install --omit=dev --ignore-scripts` re-resolved the Svelte/Vite dependency tree and failed with `ERESOLVE`. Reworked the image so the builder stage runs `npm ci`, builds the site, and prunes dev dependencies in place with `npm prune --omit=dev`, then the runtime stage copies the pruned `node_modules`, `package.json`, and `build/` output forward. That removed the second dependency-resolution path without changing the Fly deploy workflow contract. Left a before/after local build log, recorded the durable Docker pattern in `.gsd/KNOWLEDGE.md`, and saved the deployment choice to `.gsd/DECISIONS.md` via `gsd_decision_save`.

## Verification

Confirmed the old failure path by checking that `.tmp/m034-s08/deploy-services-local-build-prechange.log` contains the runtime-stage `npm install --omit=dev --ignore-scripts` and `ERESOLVE` markers while the post-fix `.tmp/m034-s08/deploy-services-local-build.log` does not. Ran the task-plan Docker build command and it completed successfully. Ran `bash scripts/verify-m034-s05-workflows.sh` and it passed, confirming the deploy-services workflow contract stayed truthful after the image change.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -c 'set -euo pipefail; rg -n "ERESOLVE|npm install --omit=dev --ignore-scripts" .tmp/m034-s08/deploy-services-local-build-prechange.log >/dev/null; if rg -n "ERESOLVE|npm install --omit=dev --ignore-scripts" .tmp/m034-s08/deploy-services-local-build.log >/dev/null; then echo "post-change build log still contains runtime reinstall failure" >&2; exit 1; fi'` | 0 | ✅ pass | 100ms |
| 2 | `bash -c 'set -euo pipefail; mkdir -p .tmp/m034-s08; docker build -f packages-website/Dockerfile packages-website | tee .tmp/m034-s08/deploy-services-local-build.log'` | 0 | ✅ pass | 4110ms |
| 3 | `bash scripts/verify-m034-s05-workflows.sh` | 0 | ✅ pass | 1470ms |


## Deviations

None.

## Known Issues

`release.yml` is still red on `v0.1.0` for the separate T04 blockers, so the slice cannot truthfully capture `.tmp/m034-s06/evidence/first-green/` yet. This task cleared the `deploy-services.yml` image blocker only.

## Files Created/Modified

- `packages-website/Dockerfile`
- `.tmp/m034-s08/deploy-services-local-build.log`
- `.tmp/m034-s08/deploy-services-local-build-prechange.log`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M034/slices/S08/tasks/T03-SUMMARY.md`


## Deviations
None.

## Known Issues
`release.yml` is still red on `v0.1.0` for the separate T04 blockers, so the slice cannot truthfully capture `.tmp/m034-s06/evidence/first-green/` yet. This task cleared the `deploy-services.yml` image blocker only.
