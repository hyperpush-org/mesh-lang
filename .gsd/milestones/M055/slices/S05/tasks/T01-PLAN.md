---
estimated_steps: 4
estimated_files: 5
skills_used:
  - debug-like-expert
  - test
---

# T01: Restore truthful current-state M055 wording and fast contract truth

Fix the only currently reproduced source drift before any assembled replay. The S01 node contract is already telling the truth: `.gsd/PROJECT.md` still says M055 is complete while S05 is open and the milestone validation is still in remediation. Treat the contract test as the guard rail, keep the fix current-state only, and widen the touched surface only if the fast three-test preflight exposes another real source drift after the wording repair.

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

## Inputs

- `.gsd/PROJECT.md`
- `scripts/tests/verify-m055-s01-contract.test.mjs`
- `scripts/tests/verify-m055-s03-contract.test.mjs`
- `scripts/tests/verify-m055-s04-contract.test.mjs`
- `.gsd/milestones/M055/M055-VALIDATION.md`

## Expected Output

- `.gsd/PROJECT.md`
- `scripts/tests/verify-m055-s01-contract.test.mjs`

## Verification

node --test scripts/tests/verify-m055-s01-contract.test.mjs scripts/tests/verify-m055-s03-contract.test.mjs scripts/tests/verify-m055-s04-contract.test.mjs
