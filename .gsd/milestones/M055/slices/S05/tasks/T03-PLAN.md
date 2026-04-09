---
estimated_steps: 4
estimated_files: 8
skills_used:
  - bash-scripting
  - debug-like-expert
  - github-workflows
---

# T03: Re-close the language-side S03 chain and republish the retained proof bundle

With S01 green, rebuild the language-side retained bundle instead of trusting the stale one already on disk. `scripts/verify-m055-s03.sh` replays the S01 wrapper, retained docs/public-surface wrappers, the workflow/public-surface contract, and the packages build before copying its own retained proof bundle. Run it only after T02 is clean, debug from `phase-report.txt` and `latest-proof-bundle.txt`, and keep the repair localized to the first failing child surface.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m055-s03.sh` wrapper chain | Fix the first failing child phase before rerunning the assembled wrapper. | Treat the timed-out child log as authoritative and do not hand-wave long-running retained rails. | Treat missing `latest-proof-bundle.txt`, bad retained copy shape, or stale `status.txt` / `current-phase.txt` files as real contract failures. |
| retained child wrappers (`m050-s02`, `m050-s03`, `m051-s04`, `m034-s05-workflows`) | Keep the repair inside the first failing retained surface and re-close S03 with fresh copied artifacts. | Read the delegated wrapper logs first; do not guess from top-level `status.txt` alone. | Treat missing child bundle pointers or mismatched phase markers as wrapper drift, not as acceptable legacy noise. |

## Load Profile

- **Shared resources**: `.tmp/m055-s03/verify/`, delegated `.tmp/m050-s02/verify/`, `.tmp/m050-s03/verify/`, `.tmp/m051-s04/verify/`, and `.tmp/m034-s05/workflows/`.
- **Per-operation cost**: one heavy wrapper plus nested docs/workflow/packages replays and retained bundle copies.
- **10x breakpoint**: repeated full wrapper replays and stale retained bundle pointers, not raw compute.

## Negative Tests

- **Malformed inputs**: missing child `latest-proof-bundle.txt`, bundle pointer that resolves to a non-directory, or stale landing job markers in the workflow/public-surface rails.
- **Error paths**: the wrapper goes red after S01 is green and points at one retained child surface that must be repaired in place.
- **Boundary conditions**: the top-level S03 bundle is only valid when `status.txt=ok`, `current-phase.txt=complete`, and the copied retained bundle snapshot exists under the fresh `.tmp/m055-s03/verify/` tree.

## Steps

1. Run `bash scripts/verify-m055-s03.sh` only after T02 leaves a fresh green S01 bundle.
2. If it fails, inspect `.tmp/m055-s03/verify/phase-report.txt`, `full-contract.log`, and the named failing child log before changing source.
3. Repair the first failing S03-owned source/wrapper/test surface, then rerun until the wrapper republishes a fresh retained proof bundle and passed phase markers.

## Must-Haves

- [ ] `bash scripts/verify-m055-s03.sh` passes from repo root after T02.
- [ ] `.tmp/m055-s03/verify/latest-proof-bundle.txt` points at a real retained bundle with the copied S01 / M050 / M051 / M034 artifacts promised by the wrapper.
- [ ] The language-side bundle is fresh and no longer depends on stale pre-S05 `.tmp` state.

## Inputs

- `.tmp/m055-s01/verify/status.txt`
- `scripts/verify-m055-s03.sh`
- `scripts/tests/verify-m055-s03-contract.test.mjs`
- `.github/workflows/deploy-services.yml`
- `scripts/verify-m050-s02.sh`
- `scripts/verify-m050-s03.sh`
- `scripts/verify-m051-s04.sh`
- `scripts/verify-m034-s05-workflows.sh`

## Expected Output

- `scripts/verify-m055-s03.sh`
- `scripts/tests/verify-m055-s03-contract.test.mjs`
- `.tmp/m055-s03/verify/status.txt`
- `.tmp/m055-s03/verify/current-phase.txt`
- `.tmp/m055-s03/verify/phase-report.txt`
- `.tmp/m055-s03/verify/latest-proof-bundle.txt`
- `.tmp/m055-s03/verify/retained-proof-bundle/verify-m055-s03.sh`

## Verification

bash scripts/verify-m055-s03.sh

## Observability Impact

- Signals added/changed: S03 must republish `status.txt`, `current-phase.txt`, `phase-report.txt`, `full-contract.log`, and a fresh `latest-proof-bundle.txt` pointer.
- How a future agent inspects this: start with `.tmp/m055-s03/verify/phase-report.txt`, then inspect the named child wrapper log and the resolved retained bundle pointer.
- Failure state exposed: stale child bundle pointers, missing retained copies, and workflow/public-surface drift remain explicit in the wrapper logs.
