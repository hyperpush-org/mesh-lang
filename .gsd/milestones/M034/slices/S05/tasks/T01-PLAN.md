---
estimated_steps: 4
estimated_files: 3
skills_used:
  - github-workflows
  - test
---

# T01: Add deploy workflow contract verification for the public release surfaces

**Slice:** S05 — Full public release assembly proof
**Milestone:** M034

## Description

Close the remaining local workflow coverage gap before composing the final release proof. S02 and S04 already verify release and extension workflows, but `deploy.yml` and `deploy-services.yml` still only have basic YAML/runtime coverage and root curls. This task adds an S05-owned workflow verifier and tightens the deploy workflows so docs deployment, Fly service deployment, and post-deploy checks are mechanically provable rather than assumed.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| GitHub Actions YAML / contract parser | Fail the task immediately and keep the first drifting invariant in a named `.tmp/m034-s05/workflows/*.log` artifact. | N/A | Treat missing triggers, permissions, needs edges, or required URL probes as contract drift. |
| Fly deploy post-deploy health checks | Keep the deploy jobs fail-closed and surface which exact URL/content assertion drifted instead of falling back to root-level curls. | Treat retries/timeouts as deploy-health failure, not as a soft warning. | Treat HTML/JSON bodies missing the exact docs or package proof markers as failure. |
| Docs deploy workflow | Block the task if Pages deploy still proves only build/upload shape without wiring the exact public files that S05 claims. | Treat a stalled Pages deploy step as workflow drift that must be fixed locally first. | Treat missing build/upload steps or wrong artifact paths as contract failure. |

## Load Profile

- **Shared resources**: GitHub runner minutes, Fly deploy jobs, and the public docs/packages hosts checked after deploy.
- **Per-operation cost**: one local workflow-contract parse sweep plus a small number of exact URL/content checks in the deploy workflows.
- **10x breakpoint**: hosted runner time and slow public retries fail first, so the contract must stay exact and avoid duplicate curl probes.

## Negative Tests

- **Malformed inputs**: missing `workflow_dispatch`, wrong tag filters, missing `needs`, or post-deploy checks that only hit `/`.
- **Error paths**: stale installer/docs content, empty package search results, or wrong registry search query must each keep the workflow red.
- **Boundary conditions**: the deploy-services checks must use the full scoped package query and exact public docs/install URLs, not slug-only or homepage-only probes.

## Steps

1. Add `scripts/verify-m034-s05-workflows.sh` as the S05 local contract verifier for `.github/workflows/deploy.yml` and `.github/workflows/deploy-services.yml`, following the S02/S04 parser-backed pattern and writing phase logs under `.tmp/m034-s05/workflows/`.
2. Tighten `.github/workflows/deploy.yml` so the docs build/deploy lane preserves the exact VitePress/public contract S05 will later verify live.
3. Tighten `.github/workflows/deploy-services.yml` so post-deploy checks prove the exact docs/install/package surfaces instead of only root reachability.
4. Rerun the new workflow verifier and YAML parse checks until deploy-lane drift fails locally before any remote rollout.

## Must-Haves

- [ ] `scripts/verify-m034-s05-workflows.sh` mechanically rejects drift in `deploy.yml` and `deploy-services.yml`
- [ ] Deploy workflow checks prove exact public installer/docs/package surfaces, not just `200 OK`
- [ ] The deploy-services package search probe uses the full scoped package query from S01
- [ ] Workflow diagnostics land under `.tmp/m034-s05/workflows/` so future failures localize cleanly

## Verification

- `bash scripts/verify-m034-s05-workflows.sh`
- `ruby -e 'require "yaml"; %w[.github/workflows/deploy.yml .github/workflows/deploy-services.yml].each { |f| YAML.load_file(f) }'`
- `rg -n 'install\.sh|install\.ps1|packages/snowdamiz/mesh-registry-proof|api/v1/packages\?search=snowdamiz%2Fmesh-registry-proof' .github/workflows/deploy.yml .github/workflows/deploy-services.yml`

## Observability Impact

- Signals added/changed: deploy-workflow contract logs under `.tmp/m034-s05/workflows/` and named failing invariants for docs vs services deploy drift.
- How a future agent inspects this: rerun `bash scripts/verify-m034-s05-workflows.sh` and read `.tmp/m034-s05/workflows/*.log`.
- Failure state exposed: the first drifting workflow, step, trigger, or URL/content assertion.

## Inputs

- `.github/workflows/deploy.yml` — current docs deployment contract that still needs exact public-surface proof.
- `.github/workflows/deploy-services.yml` — current Fly deploy contract that still only proves coarse health checks.
- `scripts/verify-m034-s02-workflows.sh` — parser-backed workflow verification pattern to mirror for S05.
- `scripts/verify-m034-s04-workflows.sh` — extension workflow verifier pattern for reusable phase logging and drift checks.
- `website/docs/public/install.sh` — exact public installer path the deploy workflows must eventually prove live.
- `website/docs/public/install.ps1` — Windows installer surface that must be checked exactly rather than inferred from site health.

## Expected Output

- `scripts/verify-m034-s05-workflows.sh` — new deploy-workflow contract verifier for S05.
- `.github/workflows/deploy.yml` — docs deploy workflow updated to preserve the exact public contract S05 claims.
- `.github/workflows/deploy-services.yml` — services deploy workflow updated to probe exact package/docs/install surfaces.
