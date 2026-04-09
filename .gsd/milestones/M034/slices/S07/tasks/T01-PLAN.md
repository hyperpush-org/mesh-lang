---
estimated_steps: 4
estimated_files: 7
skills_used:
  - github-workflows
  - test
  - debug-like-expert
---

# T01: Align hosted deploy checks with the exact S05 public-surface contract

The current hosted deploy lanes validate a weaker public contract than `run_public_http_truth()`, which is how the repo can have green deploy runs while `meshlang.dev` still serves stale installers and docs. This task closes that drift first by making the exact installer/docs/packages markers and the bounded freshness-wait behavior shared and testable across the repo-local S05 verifier and the hosted deploy workflows.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Shared public-contract helper / marker set | Fail closed in tests and workflow verifiers if any caller drops a required marker or step. | Treat exhausted freshness wait budget as a proof failure, not a soft warning. | Treat missing marker tables, missing retry budget, or drifted step names as contract breakage. |
| GitHub Pages / Fly deploy workflows | Keep workflow sweeps red until both workflows call the exact stronger contract instead of the current weaker grep set. | Fail with the last observed public mismatch and attempt count so later tasks can see whether the problem is propagation or permanent drift. | Treat presence-only checks, homepage-only curls, or missing runbook markers as malformed proof. |
| Exact public HTTP marker set | Stop the slice if installers/docs/packages markers diverge between S05, deploy.yml, deploy-services.yml, and the workflow verifier. | N/A | Treat wrong content-type expectations, missing diff paths, or missing tooling runbook markers as malformed contract data. |

## Load Profile

- **Shared resources**: live `meshlang.dev` / `packages.meshlang.dev` endpoints, CDN propagation window, workflow YAML, and `.tmp/m034-s05/verify/` diagnostics.
- **Per-operation cost**: repeated HTTP GETs plus exact diff/marker checks for four public `meshlang.dev` surfaces and three packages surfaces.
- **10x breakpoint**: stale-content polling and repeated endpoint fetches dominate first, so the helper must use one bounded retry budget with actionable diagnostics instead of many ad hoc curls.

## Negative Tests

- **Malformed inputs**: missing installer/docs markers, wrong content types, missing workflow steps, or mismatched retry-budget constants.
- **Error paths**: stale public bytes after deploy, missing shared helper call-sites, or workflow verifiers still accepting the weaker pre-S07 checks.
- **Boundary conditions**: exact installer body diffs, tooling page runbook markers, package search/detail/API markers, and the full GitHub Pages cache window are all pinned.

## Steps

1. Extract the exact installer/docs/packages marker set and bounded freshness-wait behavior into a shared helper or equivalent single-source contract that `scripts/verify-m034-s05.sh` can call for local/built/live checks without duplicating marker lists.
2. Rewire `.github/workflows/deploy.yml` and `.github/workflows/deploy-services.yml` to enforce that same stronger contract, including the full tooling runbook markers and propagation-aware failure output instead of the current weaker grep subset.
3. Update `scripts/verify-m034-s05-workflows.sh` so the workflow contract sweeps pin the stronger step bodies and fail if the workflows drift back to shallow checks.
4. Add or extend Node contract coverage so the shared markers, workflow call-sites, and freshness retry budget are mechanically pinned before any hosted rollout attempt.

## Must-Haves

- [ ] One exact public-surface contract owns the required installers/docs/packages markers and bounded freshness wait semantics.
- [ ] `scripts/verify-m034-s05.sh`, `deploy.yml`, and `deploy-services.yml` all consume that stronger contract instead of weaker subsets.
- [ ] `scripts/verify-m034-s05-workflows.sh` and Node contract tests fail if required markers, steps, or retry boundaries drift.
- [ ] The task leaves high-signal diagnostics for stale bytes vs missing markers vs wrong workflow wiring.

## Inputs

- `scripts/verify-m034-s05.sh`
- `scripts/verify-m034-s05-workflows.sh`
- `.github/workflows/deploy.yml`
- `.github/workflows/deploy-services.yml`
- `scripts/tests/verify-m034-s05-contract.test.mjs`
- `.tmp/m034-s06/evidence/closeout-20260326-1525/remote-runs.json`

## Expected Output

- `scripts/lib/m034_public_surface_contract.py`
- `scripts/verify-m034-s05.sh`
- `scripts/verify-m034-s05-workflows.sh`
- `.github/workflows/deploy.yml`
- `.github/workflows/deploy-services.yml`
- `scripts/tests/verify-m034-s05-contract.test.mjs`
- `scripts/tests/verify-m034-s07-public-contract.test.mjs`

## Verification

bash -n scripts/verify-m034-s05.sh
node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s07-public-contract.test.mjs
bash scripts/verify-m034-s05-workflows.sh

## Observability Impact

- Signals added/changed: shared public-contract diagnostics expose retry attempt count, last mismatch reason, and exact missing marker/diff paths.
- How a future agent inspects this: run `bash scripts/verify-m034-s05-workflows.sh` or `bash scripts/verify-m034-s05.sh` and inspect `.tmp/m034-s05/verify/public-http.log`, `*.diff`, and `*-check.log` artifacts.
- Failure state exposed: stale CDN bytes, missing workflow steps, wrong content-type expectations, and marker drift are distinguishable without rerunning the entire slice.
