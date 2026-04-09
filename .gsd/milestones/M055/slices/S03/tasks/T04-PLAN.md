---
estimated_steps: 4
estimated_files: 9
skills_used:
  - github-workflows
  - bash-scripting
  - test
---

# T04: Split the language-owned deploy/public-surface workflow from Hyperpush landing and add the assembled S03 verifier

Finish the slice by making the hosted/public proof graph match the language-owned boundary. This task should remove Hyperpush landing deployment and landing health checks from the mesh-lang deploy contract, update the `m034`/`m053` verifier stack to the language-only workflow graph, and add one slice-owned preflight plus assembled verifier for the full mesh-lang public-surface/starter contract.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `.github/workflows/deploy-services.yml` plus workflow verifiers | Fail closed on missing or extra jobs/steps so the language repo cannot silently keep deploying product surfaces. | Stop within the existing workflow-verifier timeout budgets. | Treat lingering landing jobs/checks or a malformed health-check graph as workflow drift. |
| `scripts/lib/m034_public_surface_contract.py` and hosted/public verifiers | Stop on the first helper/verifier mismatch and preserve the failing artifact dir. | Fail within the helper’s retry budget instead of hanging on stale public endpoints. | Treat a mismatch between helper, workflow tests, and assembled wrapper as contract drift. |
| `scripts/verify-m055-s03.sh` and `scripts/tests/verify-m055-s03-contract.test.mjs` | Fail if the wrapper omits a required phase, reuses stale bundles, or publishes malformed `.tmp/m055-s03/verify/` pointers. | Stop on the first failing phase and keep the exact phase marker. | Treat missing `status.txt`, `phase-report.txt`, or `latest-proof-bundle.txt` semantics as assembled-verifier drift. |

## Load Profile

Shared resources are `website/docs/.vitepress/dist`, `packages-website/` build output, `.tmp/m034-s05/workflows/`, and `.tmp/m055-s03/verify/`; per-operation cost is workflow source tests plus one docs helper replay, one packages build, and one assembled wrapper replay; the first 10x breakpoint is build time and repeated workflow/helper replays.

## Negative Tests

- **Malformed inputs**: landing jobs or landing health checks reappear in `deploy-services.yml`; workflow tests still require `deploy-hyperpush-landing` or `Verify hyperpush landing`; the assembled wrapper points at malformed retained bundle files.
- **Error paths**: workflow YAML changes but `m034`/`m053` verifiers still pin the old graph, or the verifiers change without the wrapper adopting the same contract.
- **Boundary conditions**: mesh-lang still owns registry + packages/public-site proof, uses the existing shared helper and retry budget, and keeps the public runbook/build surfaces truthful.

## Steps

1. Rewrite `.github/workflows/deploy-services.yml` so mesh-lang owns registry + packages-website deployment and public-surface checks only, with no landing deployment or Hyperpush endpoint checks.
2. Update `scripts/lib/m034_public_surface_contract.py`, `scripts/tests/verify-m034-s05-contract.test.mjs`, `scripts/verify-m034-s05-workflows.sh`, `scripts/verify-m034-s05.sh`, `scripts/tests/verify-m053-s03-contract.test.mjs`, and `scripts/verify-m053-s03.sh` so the workflow graph, helper contract, and hosted/public proof all match the language-only boundary.
3. Add `scripts/tests/verify-m055-s03-contract.test.mjs` and `scripts/verify-m055-s03.sh` as the slice-owned fast preflight and assembled replay, publishing the standard `.tmp/m055-s03/verify/` markers and retained bundle pointer.
4. Re-run the packages build, the public-surface helper, and the assembled wrapper.

## Must-Haves

- [ ] `mesh-lang` hosted deploy/public proof no longer requires `mesher/landing` deployment or landing health checks.
- [ ] The packages/public-site contract stays inside the normal mesh-lang hosted proof and uses the same shared helper instead of ad hoc checks.
- [ ] `scripts/verify-m055-s03.sh` proves the full mesh-lang-only public/starter contract end to end and retains standard verifier markers.

## Inputs

- `.github/workflows/deploy-services.yml`
- `scripts/lib/m034_public_surface_contract.py`
- `scripts/tests/verify-m034-s05-contract.test.mjs`
- `scripts/verify-m034-s05-workflows.sh`
- `scripts/verify-m034-s05.sh`
- `scripts/tests/verify-m053-s03-contract.test.mjs`
- `scripts/verify-m053-s03.sh`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `scripts/verify-production-proof-surface.sh`

## Expected Output

- `.github/workflows/deploy-services.yml`
- `scripts/lib/m034_public_surface_contract.py`
- `scripts/tests/verify-m034-s05-contract.test.mjs`
- `scripts/verify-m034-s05-workflows.sh`
- `scripts/verify-m034-s05.sh`
- `scripts/tests/verify-m053-s03-contract.test.mjs`
- `scripts/verify-m053-s03.sh`
- `scripts/tests/verify-m055-s03-contract.test.mjs`
- `scripts/verify-m055-s03.sh`

## Verification

node --test scripts/tests/verify-m034-s05-contract.test.mjs
bash scripts/verify-m034-s05-workflows.sh
node --test scripts/tests/verify-m053-s03-contract.test.mjs
node --test scripts/tests/verify-m055-s03-contract.test.mjs
python3 scripts/lib/m034_public_surface_contract.py local-docs --root .
npm --prefix packages-website run build
bash scripts/verify-m055-s03.sh

## Observability Impact

- Signals added/changed: `.tmp/m055-s03/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, `full-contract.log`, and `latest-proof-bundle.txt`, plus the existing `.tmp/m034-s05/workflows/` helper/workflow artifacts.
- How a future agent inspects this: run `node --test scripts/tests/verify-m055-s03-contract.test.mjs` first, then `bash scripts/verify-m055-s03.sh`, then open `.tmp/m055-s03/verify/phase-report.txt` or the retained bundle pointer.
- Failure state exposed: the exact failing workflow/helper phase, missing/extra workflow jobs, malformed wrapper artifact pointers, and build/helper drift.
