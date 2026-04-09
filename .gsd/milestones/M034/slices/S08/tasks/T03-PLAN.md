---
estimated_steps: 9
estimated_files: 7
skills_used: []
---

# T03: Repair the packages-website deploy image so `deploy-services.yml` can go green on `v0.1.0`.

Reproduce the hosted `deploy-services.yml` failure from the saved tag-run logs and the current `packages-website` container build, then remove the runtime-stage dependency installation path that triggers the Vite/Svelte peer-resolution `ERESOLVE`. Prefer a container layout that carries a production-safe dependency set forward from build time instead of re-resolving peers during the runtime image build. Keep the Fly deploy workflow contract truthful and leave a repo-local reproduction log.

Steps:
1. Use `.tmp/m034-s08/tag-rollout/deploy-services-v0.1.0-log-failed.txt` plus the current `packages-website/Dockerfile` to reproduce the failing runtime install path locally.
2. Update the packages website image build so the runtime stage no longer reruns the failing `npm install --omit=dev --ignore-scripts` resolution step.
3. Touch `.github/workflows/deploy-services.yml` or its verifier only if the deploy contract itself must change to stay truthful.

Must-haves:
- The old peer-resolution failure is reproduced or otherwise explained by a deterministic local check.
- A local Docker build for `packages-website` completes without the runtime-stage `ERESOLVE` failure.
- Any workflow/verifier edits preserve the current deploy-services ownership and health-check contract.

## Inputs

- `.tmp/m034-s08/tag-rollout/deploy-services-v0.1.0-log-failed.txt`
- `packages-website/Dockerfile`
- `packages-website/package.json`
- `packages-website/package-lock.json`
- `.github/workflows/deploy-services.yml`
- `scripts/verify-m034-s05-workflows.sh`

## Expected Output

- `packages-website/Dockerfile`
- `.tmp/m034-s08/deploy-services-local-build.log`

## Verification

bash -c 'set -euo pipefail; mkdir -p .tmp/m034-s08; docker build -f packages-website/Dockerfile packages-website | tee .tmp/m034-s08/deploy-services-local-build.log'
bash scripts/verify-m034-s05-workflows.sh

## Observability Impact

- Signals added/changed: the final `first-green` manifest and copied verifier artifacts become the milestone-closeout inspection surface.
- How a future agent inspects this: read `.tmp/m034-s06/evidence/first-green/{manifest.json,remote-runs.json,phase-report.txt,status.txt,current-phase.txt}`.
- Failure state exposed: any remaining red workflow, stop-after drift, or wrapper regression is visible in durable bundle files instead of implied by exit code alone.
