---
estimated_steps: 4
estimated_files: 10
skills_used:
  - test
---

# T06: Update wrapper/closeout rails, delete the root proof-package dirs, and close the slice

**Slice:** S04 — Retire top-level proof-app onboarding surfaces
**Milestone:** M049

## Description

Finish the retirement by updating the wrapper/closeout rails to the fixture-backed/public-story contract, deleting the top-level `tiny-cluster/` and `cluster-proof/` directories, and rerunning the authoritative verifier plus the retained historical clustered Todo subrail.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Wrapper and closeout scripts (`m045`/`m046` aliases plus `m047` rails) | Fail before the root directories are removed or before the slice claims final closeout. | Preserve the wrapper/closeout logs and retained bundle pointers for diagnosis. | Treat stale root-path literals, stale public-story regexes, or broken retained-bundle checks as contract drift. |
| Root-directory deletion | Do not remove the top-level proof packages until wrapper/closeout rails point at the moved fixtures. | N/A — local file removal only. | Treat any remaining repo-root proof-package dependency as a blocker, not a warning. |
| Authoritative cutover and retained Todo rails | Stop the slice closeout if the fixture-backed/public-story contract is not really assembled. | Preserve `.tmp/m047-s04/verify` and `.tmp/m047-s05/verify` for diagnosis. | Reject missing `running N test` markers, stale public-story assertions, or wrong fixture commands. |

## Load Profile

- **Shared resources**: wrapper/closeout verifier scripts, shared `.tmp/m047-s04` and `.tmp/m047-s05` bundles, and the moved internal fixtures.
- **Per-operation cost**: scripted grep/build/test phases with retained logs rather than live services.
- **10x breakpoint**: execution time dominates; the task should keep one authoritative cutover rail and one retained Todo subrail instead of introducing another wrapper layer.

## Negative Tests

- **Malformed inputs**: stale root literals in wrapper/closeout scripts, missing retained-bundle pointers, or deletion of the repo-root dirs before the wrappers move.
- **Error paths**: wrapper aliases no longer delegating to the correct fixture-backed rail, or `scripts/verify-m047-s04.sh` still teaching root-package onboarding text.
- **Boundary conditions**: the repo root must lose the proof-package directories entirely, but the retained rails still need meaningful artifact pointers and fixture-backed commands after deletion.

## Steps

1. Retarget `scripts/verify-m047-s04.sh`, `scripts/verify-m047-s05.sh`, and `scripts/verify-m047-s06.sh` to the moved fixtures and new public onboarding contract.
2. Retarget the thin historical wrapper aliases (`scripts/verify-m045-s04.sh`, `scripts/verify-m045-s05.sh`, `scripts/verify-m046-s04.sh`, `scripts/verify-m046-s05.sh`, and `scripts/verify-m046-s06.sh`) so they still delegate and validate retained bundle state after the path move.
3. Delete the repo-root `tiny-cluster/` and `cluster-proof/` directories once the wrapper/closeout layer no longer depends on them.
4. Rerun the authoritative cutover rail, the retained historical clustered Todo subrail, and the new onboarding contract test as the final proof that the root proof-package surfaces are gone.

## Must-Haves

- [ ] Wrapper/closeout rails stop depending on the repo-root proof-package directories.
- [ ] The repo root no longer contains `tiny-cluster/` or `cluster-proof/` as onboarding project directories.
- [ ] `bash scripts/verify-m047-s04.sh`, `bash scripts/verify-m047-s05.sh`, and the onboarding contract test all pass after deletion.

## Verification

- `node --test scripts/tests/verify-m049-s04-onboarding-contract.test.mjs`
- `bash scripts/verify-m047-s04.sh`
- `bash scripts/verify-m047-s05.sh`

## Observability Impact

- Signals added/changed: wrapper/closeout phase logs and bundle pointers should resolve the moved fixtures explicitly and still name the authoritative cutover vs retained Todo rails clearly.
- How a future agent inspects this: rerun the verification commands above and inspect `.tmp/m047-s04/verify` or `.tmp/m047-s05/verify` when a wrapper/closeout expectation regresses.
- Failure state exposed: stale wrapper literals, broken retained-bundle checks, or root-directory deletion happening too early fail in named wrapper phases instead of as ambiguous missing-path errors.

## Inputs

- `scripts/verify-m047-s04.sh` — authoritative cutover verifier that must become fixture-backed and example-first.
- `scripts/verify-m047-s05.sh` — retained historical clustered Todo subrail that should follow the moved fixtures.
- `scripts/verify-m047-s06.sh` — docs and retained-proof wrapper that must keep working after the path move.
- `scripts/verify-m045-s04.sh` — historical wrapper alias currently delegating through the older layout.
- `scripts/verify-m045-s05.sh` — historical wrapper alias currently delegating through the older layout.
- `scripts/verify-m046-s04.sh` — historical wrapper alias currently delegating through the older layout.
- `scripts/verify-m046-s05.sh` — historical wrapper alias currently delegating through the older layout.
- `scripts/verify-m046-s06.sh` — historical wrapper alias currently delegating through the older layout.
- `tiny-cluster/README.md` — repo-root proof-package file to remove once all consumers are migrated.
- `cluster-proof/README.md` — repo-root proof-package file to remove once all consumers are migrated.

## Expected Output

- `scripts/verify-m047-s04.sh` — authoritative cutover verifier updated for the fixture-backed/public-story contract.
- `scripts/verify-m047-s05.sh` — retained historical clustered Todo verifier aligned with the moved fixtures.
- `scripts/verify-m047-s06.sh` — docs and retained-proof wrapper aligned with the moved fixtures.
- `scripts/verify-m045-s04.sh` — historical wrapper alias updated for the moved fixtures.
- `scripts/verify-m045-s05.sh` — historical wrapper alias updated for the moved fixtures.
- `scripts/verify-m046-s04.sh` — historical wrapper alias updated for the moved fixtures.
- `scripts/verify-m046-s05.sh` — historical wrapper alias updated for the moved fixtures.
- `scripts/verify-m046-s06.sh` — historical wrapper alias updated for the moved fixtures.
- `tiny-cluster/README.md` — removed from the repo root after all consumers move.
- `cluster-proof/README.md` — removed from the repo root after all consumers move.
