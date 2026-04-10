---
estimated_steps: 4
estimated_files: 4
skills_used:
  - gh
  - test
  - bash-scripting
---

# T03: Replay live board truth and publish the retained S03 verifier

**Slice:** S03 — Realign org project #1 to the reconciled issue truth
**Milestone:** M057

## Description

Close the loop with a read-only verifier and results contract that prove org project #1 now matches the reconciled repo truth. This task must re-fetch the live board, replay it against the S03 plan/results artifacts, and leave durable diagnostics under `.tmp/m057-s03/verify/` so future maintainers can localize drift quickly. The verification contract must cover membership, canonical issue identity, status truth, naming normalization, and inherited tracked metadata on representative rows.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `.gsd/milestones/M057/slices/S03/project-mutation-results.json` + `.gsd/milestones/M057/slices/S03/project-mutation-plan.json` | Fail the verifier immediately if results do not cover the exact planned touched set or if canonical mappings/final row snapshots are missing. | N/A — local artifact reads should fail immediately. | Reject missing rollups, duplicate operation ids, or incomplete final-state records instead of inferring success. |
| Live `gh project item-list` plus representative `gh issue view` reads | Mark the phase red, persist the last target and command evidence, and stop rather than silently trusting stale artifacts. | Persist timeout diagnostics under `.tmp/m057-s03/verify/` and require a rerun before claiming verification passed. | Reject unexpected board row shapes, missing canonical issue URLs, or field values that cannot be normalized to the captured schema. |
| `.gsd/milestones/M057/slices/S02/repo-mutation-results.json` canonical mapping source | Fail closed when the verifier cannot prove `mesh-lang#19` / `hyperpush#58` handling against the persisted S02 identities. | N/A — local artifact reads should fail immediately. | Reject mapping drift that would let the board claim truth while pointing at the wrong issue identity. |

## Load Profile

- **Shared resources**: live GitHub ProjectV2 read API, live issue read API, and the S03/S02 artifacts under `.gsd/milestones/M057/slices/`.
- **Per-operation cost**: one full board replay plus representative issue lookups and inherited-field spot checks on sample descendants.
- **10x breakpoint**: repeated live reads and broad field-coherence checks would hit rate limits first; the verifier should target the touched set plus representative inherited rows instead of rescanning the world blindly.

## Negative Tests

- **Malformed inputs**: results artifact missing final item ids, canonical issue URLs, or representative field snapshots; verifier logs missing failed phase or last target.
- **Error paths**: stale cleanup rows still present, canonical replacement rows absent, `Done` assigned to open issues, `Todo` left on newly closed canonical rows, and `hyperpush-mono` naming reappearing on `hyperpush#54/#55/#56`.
- **Boundary conditions**: representative `Done`, `In Progress`, and `Todo` rows all pass; inherited rows such as `hyperpush#29`, `hyperpush#33`, `hyperpush#35`, `hyperpush#54`, `hyperpush#55`, and `hyperpush#57` resolve to coherent tracked metadata.

## Steps

1. Add `scripts/tests/verify-m057-s03-results.test.mjs` to lock the results artifact shape, touched-set coverage, canonical mapping handling, and representative row-state assertions.
2. Add `scripts/verify-m057-s03.sh` to re-fetch the live board, replay touched rows, and write phase diagnostics, `last-target.txt`, and command logs under `.tmp/m057-s03/verify/`.
3. Verify representative membership/status/naming/inheritance outcomes, including canonical `mesh-lang#19` / `hyperpush#58` handling and normalized naming on `hyperpush#54/#55/#56`.
4. Refresh `.gsd/milestones/M057/slices/S03/project-mutation-results.md` so a maintainer can understand done/active/next truth directly from the handoff artifact.

## Must-Haves

- [ ] The retained verifier proves stale cleanup rows are gone and canonical replacement rows are handled truthfully on the live board.
- [ ] Verification covers representative `Done`, `In Progress`, and `Todo` rows plus inherited tracked metadata on representative descendants.
- [ ] `.tmp/m057-s03/verify/` exposes failed phase, last target, and command evidence so future drift is diagnosable without re-deriving S03 logic.

## Verification

- `node --test scripts/tests/verify-m057-s03-results.test.mjs`
- `bash scripts/verify-m057-s03.sh`

## Observability Impact

- Signals added/changed: phase verdicts, representative row assertions, last-target tracking, and persisted command evidence for each live verification step.
- How a future agent inspects this: open `.tmp/m057-s03/verify/phase-report.txt`, `.tmp/m057-s03/verify/verification-summary.json`, and the per-command logs.
- Failure state exposed: whether drift came from repo-precheck mismatch, stale board membership, canonical mapping loss, naming regression, or inherited-field incoherence.

## Inputs

- `scripts/lib/m057_project_mutation_apply.py` — apply implementation whose results and idempotence behavior must be replayed.
- `.gsd/milestones/M057/slices/S03/project-mutation-plan.json` — planned touched set and representative row expectations.
- `.gsd/milestones/M057/slices/S03/project-mutation-results.json` — live mutation results and final row snapshots.
- `.gsd/milestones/M057/slices/S02/repo-mutation-results.json` — canonical S02 identity mappings used for final board truth checks.

## Expected Output

- `scripts/tests/verify-m057-s03-results.test.mjs` — results contract test for touched-set coverage and final-state truth.
- `scripts/verify-m057-s03.sh` — retained live verifier for org project #1.
- `.tmp/m057-s03/verify/phase-report.txt` — compact phase health summary for future reruns.
- `.tmp/m057-s03/verify/verification-summary.json` — machine-readable verification rollup with last target and phase details.
