---
estimated_steps: 4
estimated_files: 6
skills_used:
  - bash-scripting
  - test
  - github-workflows
---

# T04: Assemble one retained two-repo evidence verifier

Close the loop with one assembled two-repo proof rail. This task should materialize or validate the staged sibling workspace, run the language-owned and product-owned proof entrypoints from their own repo roots, and retain a single S04 bundle that records which repo/ref and which proof bundle belonged to each side.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| staged workspace materializer | Stop on the first staging failure and preserve the workspace root plus manifest. | Fail within the staging budget instead of leaving partial repo trees behind. | Treat missing product-root files or leaked local state as assembly drift. |
| language-owned `bash scripts/verify-m055-s03.sh` and product-owned verifier entrypoints | Stop on the first failing delegated phase and retain each delegated `.tmp/.../verify/` pointer. | Use the delegated timeout budgets and surface the exact failing phase. | Treat missing `latest-proof-bundle.txt` pointers or malformed delegated markers as contract breaks. |
| assembled `.tmp/m055-s04/verify/` bundle | Fail if phase markers, repo/ref metadata, or retained bundle pointers are missing or contradictory. | Stop on the first bundle-shape mismatch and keep the failing assembly log. | Treat mixed language/product repo attribution as false evidence. |

## Load Profile

- **Shared resources**: `.tmp/m055-s04/workspace/`, `.tmp/m055-s04/verify/`, delegated `.tmp/m055-s03/verify/`, and delegated product verifier artifacts.
- **Per-operation cost**: one staged workspace refresh, one product-owned verifier replay, one language-owned verifier replay, and one retained bundle copy.
- **10x breakpoint**: verifier runtime and retained artifact churn, not CPU.

## Negative Tests

- **Malformed inputs**: missing sibling repo, missing delegated bundle pointer, or repo/ref metadata captured from the wrong repo root.
- **Error paths**: both delegated verifiers can pass independently, but the S04 wrapper fails because it cannot attribute repo/ref or copy the retained bundles truthfully.
- **Boundary conditions**: the wrapper may use env overrides for debugging, but the published S04 bundle must still record both repo identities and both proof-bundle pointers explicitly.

## Steps

1. Add `scripts/verify-m055-s04.sh` to refresh the staged sibling workspace, run the product-owned verifier entrypoints from `hyperpush-mono`, run the language-owned `bash scripts/verify-m055-s03.sh` with the canonical language repo slug, and stop on the first failing phase.
2. Capture product and language repo/ref metadata, delegated bundle pointers, and copied retained verifier trees into `.tmp/m055-s04/verify/`.
3. Extend the S04 contract test to pin the assembled wrapper’s phase order, repo/ref metadata fields, and retained bundle shape.
4. Replay the full S04 wrapper and inspect the retained bundle rather than raw delegated `.tmp` trees.

## Must-Haves

- [ ] One S04 bundle shows both repo identities/refs and the proof bundle pointer for each repo.
- [ ] The assembled wrapper only succeeds when both the language-owned and product-owned proof entrypoints pass from their own repo roots.
- [ ] `.tmp/m055-s04/verify/` publishes the standard `status.txt`, `current-phase.txt`, `phase-report.txt`, `full-contract.log`, and `latest-proof-bundle.txt` surfaces.

## Inputs

- `scripts/materialize-hyperpush-mono.mjs`
- `scripts/lib/m055-workspace.sh`
- `scripts/verify-m051-s01.sh`
- `scripts/verify-m055-s03.sh`
- `scripts/verify-m053-s03.sh`
- `scripts/tests/verify-m055-s04-contract.test.mjs`
- `scripts/tests/verify-m055-s04-materialize.test.mjs`

## Expected Output

- `scripts/verify-m055-s04.sh`
- `scripts/tests/verify-m055-s04-contract.test.mjs`

## Verification

node --test scripts/tests/verify-m055-s04-contract.test.mjs
bash scripts/verify-m055-s04.sh

## Observability Impact

- Signals added/changed: `.tmp/m055-s04/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, `full-contract.log`, repo/ref metadata files, and the final `latest-proof-bundle.txt` pointer.
- How a future agent inspects this: run `node --test scripts/tests/verify-m055-s04-contract.test.mjs` first, then `bash scripts/verify-m055-s04.sh`, then open `.tmp/m055-s04/verify/phase-report.txt` and the copied delegated bundle metadata.
- Failure state exposed: first failing delegated phase, missing sibling repo, malformed delegated bundle pointer, repo/ref mismatch, and retained-bundle drift.
