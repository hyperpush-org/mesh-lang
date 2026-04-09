---
estimated_steps: 5
estimated_files: 7
skills_used:
  - bash-scripting
  - debug-like-expert
  - github-workflows
---

# T04: Re-close the S04 attribution bundle and refresh milestone validation

Finish the actual remediation closeout only after S03 is green. `scripts/verify-m055-s04.sh` must run serially against the staged `hyperpush-mono` workspace, publish the language/product repo metadata and pointer files promised by S04, and retain copied language/product proof bundles. Once that chain is green, refresh M055 milestone validation from the fresh evidence so the milestone no longer records the round-0 remediation failure as current truth.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/materialize-hyperpush-mono.mjs` + `scripts/verify-m055-s04.sh` | Run serially and fail closed on the first staged-workspace or product-root issue. | Do not start a parallel materializer/check run; a timeout or concurrent refresh means the S04 bundle is untrusted. | Treat missing `language-repo.meta.json`, `product-repo.meta.json`, or proof-bundle pointer files as S04 contract failure even if `status.txt` says `ok`. |
| staged product-root wrappers and repo metadata capture | Fix the first product-root or metadata/pointer seam the fresh run exposes; do not fall back to the in-repo `mesher/` tree. | Inspect staged product logs and copied child bundle pointers before rerunning. | Treat wrong slug/ref attribution, empty pointer files, or malformed retained bundle manifests as fail-closed evidence drift. |
| milestone validation refresh | Do not declare the slice closed while `M055-VALIDATION.md` still reflects remediation round 0 as current truth. | N/A for local validation render. | Treat a non-pass verdict after all wrappers are green as a real remaining blocker. |

## Load Profile

- **Shared resources**: `.tmp/m055-s04/workspace/hyperpush-mono`, `.tmp/m055-s04/verify/`, staged product `.tmp/m051-s01/verify/`, staged landing verify output, and milestone validation artifacts.
- **Per-operation cost**: materializer refresh, product-root wrapper replay, language-side wrapper replay, bundle copy/metadata capture, and validation render.
- **10x breakpoint**: staged workspace churn and stale attribution files, not CPU or memory.

## Negative Tests

- **Malformed inputs**: concurrent materializer use, missing staged manifest fingerprint, empty `language-proof-bundle.txt` / `product-proof-bundle.txt`, or wrong repo slug/ref metadata.
- **Error paths**: S04 fails after S03 is green because the copy/metadata layer or product-root wrapper is still incomplete.
- **Boundary conditions**: the final bundle is only authoritative when `status.txt=ok`, `current-phase.txt=complete`, all repo meta/pointer files exist, and the refreshed milestone validation records a pass.

## Steps

1. Run `bash scripts/verify-m055-s04.sh` in isolation with no concurrent `materialize-hyperpush-mono` check or other S04 replay.
2. If the wrapper fails after the staged product and language child phases are green, debug the first missing copy/metadata/pointer seam inside `.tmp/m055-s04/verify/`.
3. Rerun until S04 publishes the promised meta/pointer files and retained bundle, then refresh `M055-VALIDATION.md` from the fresh green S01/S03/S04 evidence.
4. Treat any non-pass revalidation verdict as a blocker instead of papering over it.

## Must-Haves

- [ ] `bash scripts/verify-m055-s04.sh` passes serially from repo root.
- [ ] `.tmp/m055-s04/verify/latest-proof-bundle.txt`, `language-repo.meta.json`, `product-repo.meta.json`, `language-proof-bundle.txt`, and `product-proof-bundle.txt` all exist and point at fresh retained content.
- [ ] `M055-VALIDATION.md` is refreshed from the green evidence chain and no longer reports remediation round 0 as current state.

## Inputs

- `.tmp/m055-s03/verify/latest-proof-bundle.txt`
- `scripts/verify-m055-s04.sh`
- `scripts/materialize-hyperpush-mono.mjs`
- `scripts/lib/m055-workspace.sh`
- `.gsd/milestones/M055/M055-VALIDATION.md`

## Expected Output

- `scripts/verify-m055-s04.sh`
- `.tmp/m055-s04/verify/status.txt`
- `.tmp/m055-s04/verify/current-phase.txt`
- `.tmp/m055-s04/verify/latest-proof-bundle.txt`
- `.tmp/m055-s04/verify/language-repo.meta.json`
- `.tmp/m055-s04/verify/product-repo.meta.json`
- `.tmp/m055-s04/verify/language-proof-bundle.txt`
- `.tmp/m055-s04/verify/product-proof-bundle.txt`
- `.gsd/milestones/M055/M055-VALIDATION.md`

## Verification

bash scripts/verify-m055-s04.sh && rg -n "^verdict: pass$" .gsd/milestones/M055/M055-VALIDATION.md

## Observability Impact

- Signals added/changed: S04 must republish `status.txt`, `current-phase.txt`, `phase-report.txt`, `full-contract.log`, repo metadata JSON, and product/language proof-bundle pointer files.
- How a future agent inspects this: start with `.tmp/m055-s04/verify/phase-report.txt`, then inspect the copied language/product bundle pointers and the staged product verify logs.
- Failure state exposed: staged-workspace churn, missing repo attribution, malformed child bundle copies, and stale validation verdicts remain explicit instead of being inferred.
