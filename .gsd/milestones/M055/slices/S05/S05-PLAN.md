# S05: Validation Remediation: Contract Truth & Two-Repo Evidence Closure

**Goal:** Re-close the M055 split-contract evidence chain by restoring truthful current-state wording, rerunning the language-side wrappers serially, and republishing the final two-repo bundle with explicit language/product attribution.
**Demo:** After this: After this, `bash scripts/verify-m055-s01.sh`, `bash scripts/verify-m055-s03.sh`, and `bash scripts/verify-m055-s04.sh` all pass from a clean repo and publish the retained bundle plus per-repo attribution files promised by M055.

## Tasks
- [x] **T01: Restore truthful current-state M055 wording and fast contract truth** — Fix the only currently reproduced source drift before any assembled replay. The S01 node contract is already telling the truth: `.gsd/PROJECT.md` still says M055 is complete while S05 is open and the milestone validation is still in remediation. Treat the contract test as the guard rail, keep the fix current-state only, and widen the touched surface only if the fast three-test preflight exposes another real source drift after the wording repair.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `.gsd/PROJECT.md` current-state wording | Keep the milestone state truthful and stop if the wording would claim completion early. | N/A for local file edits. | Treat mixed `active split-contract milestone` and `now complete` language as contract drift. |
| `scripts/tests/verify-m055-s01-contract.test.mjs` preflight | Fix the smallest source surface the test points at before touching wrapper code. | N/A for local node:test runs. | Treat a new failing assertion after the wording repair as the next real source drift, not as permission to relax the test. |

## Load Profile

- **Shared resources**: repo-local current-state docs and fast node:test contract files only.
- **Per-operation cost**: one `.gsd` doc edit plus a small node:test preflight.
- **10x breakpoint**: stale state language across multiple docs, not compute or IO.

## Negative Tests

- **Malformed inputs**: `.gsd/PROJECT.md` still says `M055 is now complete.` or mixes completed/open milestone language.
- **Error paths**: the fast three-file contract preflight stays red after the wording fix and points at a second source surface.
- **Boundary conditions**: the M055 section stays current-state while older milestone-complete sections below it remain untouched.

## Steps

1. Update `.gsd/PROJECT.md` so M055 is described as the active split-contract milestone and the current repo state stays truthful while S05 is pending.
2. Cross-check the wording against `scripts/tests/verify-m055-s01-contract.test.mjs` and the current M055 validation/remediation story before broadening the edit surface.
3. Run the fast M055 node preflight and only expand into the smallest additional source file if the fresh preflight exposes another real contract drift.

## Must-Haves

- [ ] `.gsd/PROJECT.md` no longer claims M055 is complete while S05 is still open.
- [ ] The M055 wording stays current-state only and preserves the repo-local `.gsd` authority / two-repo split contract already shipped in S01-S04.
- [ ] The fast M055 contract preflight is green or points at one concrete next drift surface for T02.
  - Estimate: 30m
  - Files: .gsd/PROJECT.md, scripts/tests/verify-m055-s01-contract.test.mjs, scripts/tests/verify-m055-s03-contract.test.mjs, scripts/tests/verify-m055-s04-contract.test.mjs, .gsd/milestones/M055/M055-VALIDATION.md
  - Verify: node --test scripts/tests/verify-m055-s01-contract.test.mjs scripts/tests/verify-m055-s03-contract.test.mjs scripts/tests/verify-m055-s04-contract.test.mjs
- [x] **T02: Re-close the S01 wrapper and republish a fresh split-boundary bundle** — Once the current-state doc is fixed, close the first real gate instead of skipping ahead. `scripts/verify-m055-s01.sh` is the authoritative narrow stop/go rail for the split boundary, and S03/S04 are downstream of it. Rerun the wrapper from a clean repo, inspect `.tmp/m055-s01/verify/phase-report.txt` first on failure, and only patch the exact source/helper/verifier surface the fresh failure localizes.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m055-s01.sh` phase chain | Stop at the first red phase and repair that exact source/verifier surface before rerunning. | Treat the timed-out child log as authoritative and do not skip the phase. | Treat missing `phase-report.txt`, empty logs, or a 0-test cargo filter as verifier drift. |
| packages / landing / local-docs child phases | Preserve the current narrow S01 scope; fix only the failing helper/build surface. | Read the per-phase log in `.tmp/m055-s01/verify/` before changing commands. | Treat stale copied artifacts or missing passed markers as real failures, not as acceptable partial success. |

## Load Profile

- **Shared resources**: `.tmp/m055-s01/verify/`, packages build output, landing build output, and the retained M046 cargo rail.
- **Per-operation cost**: one assembled shell verifier plus nested node/python/npm/cargo phases.
- **10x breakpoint**: repeated full wrapper reruns and stale artifact interpretation, not memory or CPU.

## Negative Tests

- **Malformed inputs**: missing or stale `.tmp/m055-s01/verify/phase-report.txt`, empty child log, or 0-test cargo output.
- **Error paths**: a fresh child phase fails after T01 and requires a minimal source/helper fix instead of a broader wrapper rewrite.
- **Boundary conditions**: the wrapper must succeed from repo root without relying on previous `.tmp/m055-s01/verify/` state.

## Steps

1. Rerun `bash scripts/verify-m055-s01.sh` from a clean repo and inspect `.tmp/m055-s01/verify/phase-report.txt` plus the failing child log first if it goes red.
2. Repair only the smallest truthful source/helper/verifier surface the fresh S01 failure identifies; do not broaden into S03/S04 work here.
3. Rerun until `.tmp/m055-s01/verify/status.txt` is `ok`, `current-phase.txt` is `complete`, and every S01 phase marker is `passed`.

## Must-Haves

- [ ] `bash scripts/verify-m055-s01.sh` passes from repo root with fresh `.tmp/m055-s01/verify/` artifacts.
- [ ] The passing bundle includes `status.txt`, `current-phase.txt`, `phase-report.txt`, and `full-contract.log`.
- [ ] Any new fix stays inside the narrow S01 boundary contract instead of pulling S03/S04 concerns earlier.
  - Estimate: 1h
  - Files: .gsd/PROJECT.md, scripts/verify-m055-s01.sh, scripts/lib/m034_public_surface_contract.py, packages-website/package.json, mesher/landing/package.json, .tmp/m055-s01/verify/status.txt, .tmp/m055-s01/verify/phase-report.txt
  - Verify: bash scripts/verify-m055-s01.sh
- [x] **T03: Replayed `verify-m055-s03.sh` to republish a fresh language-side retained bundle with green phase markers and a valid proof-bundle pointer.** — With S01 green, rebuild the language-side retained bundle instead of trusting the stale one already on disk. `scripts/verify-m055-s03.sh` replays the S01 wrapper, retained docs/public-surface wrappers, the workflow/public-surface contract, and the packages build before copying its own retained proof bundle. Run it only after T02 is clean, debug from `phase-report.txt` and `latest-proof-bundle.txt`, and keep the repair localized to the first failing child surface.

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
  - Estimate: 90m
  - Files: .tmp/m055-s01/verify/status.txt, scripts/verify-m055-s03.sh, scripts/tests/verify-m055-s03-contract.test.mjs, .github/workflows/deploy-services.yml, scripts/verify-m050-s02.sh, scripts/verify-m050-s03.sh, scripts/verify-m051-s04.sh, scripts/verify-m034-s05-workflows.sh
  - Verify: bash scripts/verify-m055-s03.sh
- [x] **T04: Replayed the S04 two-repo verifier, published fresh language/product attribution pointers, and replaced the stale remediation-round-0 milestone validation with a pass verdict.** — Finish the actual remediation closeout only after S03 is green. `scripts/verify-m055-s04.sh` must run serially against the staged `hyperpush-mono` workspace, publish the language/product repo metadata and pointer files promised by S04, and retain copied language/product proof bundles. Once that chain is green, refresh M055 milestone validation from the fresh evidence so the milestone no longer records the round-0 remediation failure as current truth.

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
  - Estimate: 2h
  - Files: .tmp/m055-s03/verify/latest-proof-bundle.txt, scripts/verify-m055-s04.sh, scripts/materialize-hyperpush-mono.mjs, scripts/lib/m055-workspace.sh, .gsd/milestones/M055/M055-VALIDATION.md, .tmp/m055-s04/verify/language-repo.meta.json, .tmp/m055-s04/verify/product-repo.meta.json
  - Verify: bash scripts/verify-m055-s04.sh && rg -n "^verdict: pass$" .gsd/milestones/M055/M055-VALIDATION.md
