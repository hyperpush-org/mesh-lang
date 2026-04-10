---
estimated_steps: 4
estimated_files: 4
skills_used:
  - gh
  - test
---

# T01: Generate a live-truth board mutation manifest with repo-drift precheck and field inheritance

**Slice:** S03 — Realign org project #1 to the reconciled issue truth
**Milestone:** M057

## Description

Build the S03 planner before any board writes so org project edits happen from one checked manifest instead of ad hoc `gh project` commands. This task must record the current repo-truth preflight, normalize the live board with the existing inventory helpers, join that live state to the S01 ledger plus S02 canonical mapping/results artifacts, and emit one deterministic delete/add/update plan. The planner also needs to fill missing tracked project metadata through explicit parent-chain inheritance so S03 does not depend on manual board cleanup.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m057-s02.sh` and current live repo issue state | Fail closed and record the red preflight in the S03 plan artifact; do not allow silent board planning from stale repo assumptions. | Stop planning, persist the timed-out preflight result, and require a fresh rerun before any board write path. | Reject contradictory repo totals or issue-state replay output instead of guessing the intended board truth. |
| `scripts/lib/m057_tracker_inventory.py` + `scripts/lib/m057_project_items.graphql` live project capture | Abort and emit no plan if the live board cannot be fetched or normalized with stable tracked-field keys. | Fail the planner and leave no checked manifest so later tasks cannot mutate from partial board data. | Reject missing item ids, duplicate canonical issue URLs, or tracked-field schema drift. |
| `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json` + `.gsd/milestones/M057/slices/S02/repo-mutation-results.json` + `.gsd/milestones/M057/slices/S01/project-fields.snapshot.json` | Refuse to generate delete/add/update operations when canonical mappings, field ids, or ledger coverage are missing. | N/A — local artifact reads should fail immediately. | Reject missing `mesh-lang#19` / `hyperpush#58` mappings, unknown field option ids, or incomplete inheritance inputs. |

## Load Profile

- **Shared resources**: live GitHub issue/project APIs, the 63-row org project dataset, and local S01/S02 JSON artifacts.
- **Per-operation cost**: one retained S02 verifier run, one normalized live board fetch, one deterministic join/classification pass over all project rows, and 23 inheritance lookups for incomplete tracked metadata.
- **10x breakpoint**: GitHub pagination/rate limits and inheritance-join drift would break first; the planner must stay deterministic and fail closed instead of papering over missing rows.

## Negative Tests

- **Malformed inputs**: missing canonical mapping for `mesh-lang#19` or `hyperpush#58`, unknown project field ids/options, duplicate canonical issue URLs, and broken inheritance source definitions.
- **Error paths**: red or timed-out S02 preflight, missing project items that the ledger says are board-backed, and live board rows whose titles/URLs would regress the S02 naming normalization.
- **Boundary conditions**: all 10 stale cleanup rows still present, canonical replacement rows missing, 23 rows needing inheritance, and representative already-correct rows that should remain untouched.

## Steps

1. Add `scripts/lib/m057_project_mutation_plan.py` to run the retained S02 verifier, capture/normalize the live board, and join live project rows to S01/S02 truth.
2. Encode deterministic parent-chain inheritance for the incomplete tracked fields and make the inheritance source explicit in the plan artifact.
3. Render `.gsd/milestones/M057/slices/S03/project-mutation-plan.json` plus `.md` with delete/add/update rollups, preflight evidence, canonical mapping handling, and representative field-edit details.
4. Add `scripts/tests/verify-m057-s03-plan.test.mjs` to lock the plan shape, cleanup-row handling, canonical mappings, and inheritance coverage/fail-closed cases.

## Must-Haves

- [ ] The planner records the retained S02 verifier outcome and derives board mutations from current live repo truth plus the persisted S02 canonical mappings.
- [ ] The plan artifact explicitly accounts for stale cleanup rows, canonical `mesh-lang#19` / `hyperpush#58` handling, and public naming normalization on `hyperpush#54/#55/#56`.
- [ ] The plan resolves the missing tracked project metadata through deterministic parent-chain inheritance instead of manual board edits.

## Verification

- `python3 scripts/lib/m057_project_mutation_plan.py --output-dir .gsd/milestones/M057/slices/S03 --check`
- `node --test scripts/tests/verify-m057-s03-plan.test.mjs`

## Observability Impact

- Signals added/changed: repo-precheck verdict, delete/add/update rollups, inheritance-source records, and planned final row state for touched items.
- How a future agent inspects this: open `.gsd/milestones/M057/slices/S03/project-mutation-plan.json` / `.md` or rerun the plan test.
- Failure state exposed: whether drift came from repo truth, missing board rows, canonical mapping loss, naming regression, or inherited-field resolution.

## Inputs

- `scripts/verify-m057-s02.sh` — retained repo-truth verifier that must be run and recorded before board planning.
- `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json` — canonical source for which historical rows should be removed, kept, or updated on the board.
- `.gsd/milestones/M057/slices/S01/project-fields.snapshot.json` — stable project id, field ids, and single-select option ids.
- `.gsd/milestones/M057/slices/S01/project-items.snapshot.json` — historical normalized board shape for regression checks.
- `.gsd/milestones/M057/slices/S02/repo-mutation-results.json` — canonical old→new issue mappings and normalized repo truth from S02.
- `scripts/lib/m057_tracker_inventory.py` — reusable field/item normalization helpers and project constants.
- `scripts/lib/m057_project_items.graphql` — live ProjectV2 query used to capture board state.

## Expected Output

- `scripts/lib/m057_project_mutation_plan.py` — checked planner that emits the S03 live-truth board manifest.
- `scripts/tests/verify-m057-s03-plan.test.mjs` — contract test for plan shape, canonical mapping handling, and inheritance coverage.
- `.gsd/milestones/M057/slices/S03/project-mutation-plan.json` — machine-readable manifest with preflight evidence and delete/add/update operations.
- `.gsd/milestones/M057/slices/S03/project-mutation-plan.md` — human-readable summary of the live board realignment plan.
