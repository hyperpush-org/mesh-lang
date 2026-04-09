---
estimated_steps: 23
estimated_files: 8
skills_used: []
---

# T01: Captured a fresh hosted blocker baseline and built a current-origin/main M053-only rollout commit for T02.

Reconfirm the local workflow/verifier contract, capture one fresh hosted-evidence baseline under `.tmp/m053-s03/verify/`, and assemble a minimal rollout commit from `origin/main` that carries the M053 S01-S04 tree without unrelated local-ahead work. Later tasks should push a known commit SHA, not local `HEAD`.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Local contract rails (`bash scripts/verify-m034-s02-workflows.sh` and `node --test scripts/tests/verify-m053-s03-contract.test.mjs`) | Stop before any remote mutation and fix local workflow/contract drift first. | Treat slow local rails as real regression signal; do not skip them to unblock rollout. | Fail closed if the local workflow or hosted-contract assertions stop producing the expected markers. |
| Hosted baseline capture (`GH_TOKEN` + `bash scripts/verify-m053-s03.sh`) | Preserve the retained `.tmp/m053-s03/verify/` bundle and stop on auth/repo-slug failures instead of guessing. | Treat `gh` or network timeouts as incomplete hosted evidence and retain the failing logs. | Fail closed if `remote-runs.json`, `candidate-refs.json`, or phase markers are missing or unparsable. |
| Local git history / rollout assembly | Stop if the candidate still contains M056 `/pitch`, omits required M053 files, or cannot be rebased cleanly onto `origin/main`. | Treat fetch/cherry-pick conflicts as rollout-assembly failures that must be resolved before pushing. | Fail closed if the retained rollout manifest cannot prove which SHA/files will ship. |

## Load Profile

- **Shared resources**: local git refs/branches, GitHub API rate budget for baseline capture, and retained artifacts under `.tmp/m053-s03/verify/` plus `.tmp/m053-s05/rollout/`.
- **Per-operation cost**: one local workflow sweep, one hosted verifier baseline replay, one `git fetch`, and one minimal rollout-branch/commit assembly.
- **10x breakpoint**: GitHub API waiting and git conflict resolution, not application throughput.

## Negative Tests

- **Malformed inputs**: missing `GH_TOKEN`, missing/renamed origin remote, absent required M053 files, or a commit selection that still includes unrelated M056 surfaces.
- **Error paths**: local workflow contract turns red, hosted baseline artifacts are missing, cherry-pick/conflict resolution fails, or the rollout tree drops S04 docs/runtime evidence.
- **Boundary conditions**: `origin/main` has advanced since research, the hosted baseline is already greener than expected, or the current binary tag is still lightweight-only.

## Steps

1. Re-run `bash scripts/verify-m034-s02-workflows.sh` and `node --test scripts/tests/verify-m053-s03-contract.test.mjs`, then capture a fresh hosted baseline with `GH_TOKEN` parsed from `.env` (do **not** `source .env`) so `.tmp/m053-s03/verify/` reflects current live blockers before any push.
2. Starting from `origin/main`, assemble a minimal local rollout branch/commit that carries the full M053 S01-S04 tree needed by starter proof, packages verification, and public docs while excluding unrelated M056 `/pitch` work.
3. Record the candidate commit SHA, included files/commits, and explicit exclusion of unrelated work in `.tmp/m053-s05/rollout/main-rollout-plan.md` plus `.tmp/m053-s05/rollout/main-rollout-commit.txt` for downstream tasks.

## Must-Haves

- [ ] A fresh hosted baseline bundle exists under `.tmp/m053-s03/verify/` before any remote mutation.
- [ ] The local rollout candidate carries the required M053 S01-S04 tree and explicitly excludes unrelated M056 work.
- [ ] A retained manifest names the exact rollout commit SHA and the proof-critical files it will ship.

## Inputs

- ``.gsd/milestones/M053/slices/S05/S05-RESEARCH.md` — hosted-rollout constraints, blockers, and recommended seam order`
- ``.gsd/milestones/M053/slices/S03/S03-SUMMARY.md` — retained hosted verifier contract and forward-intelligence notes`
- ``scripts/verify-m034-s02-workflows.sh` — local workflow-topology contract rail`
- ``scripts/tests/verify-m053-s03-contract.test.mjs` — local hosted-contract test suite`
- ``scripts/verify-m053-s03.sh` — hosted evidence verifier and artifact contract`
- ``.github/workflows/authoritative-verification.yml` — mainline starter-proof caller workflow`
- ``.github/workflows/deploy-services.yml` — packages/public-surface hosted contract on `main``
- ``.github/workflows/release.yml` — release/tag workflow that must rerun on the shipped binary tag`

## Expected Output

- ``.tmp/m053-s03/verify/status.txt` — fresh hosted baseline status marker captured before rollout`
- ``.tmp/m053-s03/verify/remote-runs.json` — current live workflow evidence used to confirm the red baseline`
- ``.tmp/m053-s05/rollout/main-rollout-plan.md` — retained summary of the exact M053-only rollout contents and excluded work`
- ``.tmp/m053-s05/rollout/main-rollout-commit.txt` — exact local commit SHA to push in T02`

## Verification

bash scripts/verify-m034-s02-workflows.sh && node --test scripts/tests/verify-m053-s03-contract.test.mjs && test -s .tmp/m053-s03/verify/remote-runs.json && test -s .tmp/m053-s05/rollout/main-rollout-commit.txt

## Observability Impact

- Signals added/changed: fresh baseline `status.txt`, `current-phase.txt`, `phase-report.txt`, and rollout manifest files under `.tmp/m053-s05/rollout/`.
- How a future agent inspects this: read `.tmp/m053-s03/verify/remote-runs.json` for live blockers, then read `.tmp/m053-s05/rollout/main-rollout-plan.md` for the exact ship set.
- Failure state exposed: local contract drift, missing GH auth/repo slug, cherry-pick conflicts, or a rollout candidate that still carries unrelated work.
