---
estimated_steps: 3
estimated_files: 5
skills_used:
  - gh
  - bash-scripting
  - test
---

# T03: Assemble the reconciliation ledger and publish the fail-closed audit proof

**Slice:** S01 — Audit code reality and build the reconciliation ledger
**Milestone:** M057

## Description

Close the slice by joining live inventory and code evidence into one canonical ledger. This task is the actual slice contract: every repo issue and every org project item must land in one row set keyed by canonical issue URL, with non-empty evidence refs, ownership truth, delivery truth, and proposed repo/project actions, plus a readable audit summary that S02 and S03 can execute from directly.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Raw snapshot bundle from T01 | Refuse to build the ledger and report which snapshot is missing or stale instead of guessing counts. | If live refresh is part of the wrapper, fail the refresh phase and keep the last good snapshot bundle intact for inspection. | Reject rows whose schema no longer matches the join contract. |
| Evidence index and naming map from T02 | Fail any row without evidence refs or canonical naming normalization; do not emit partially classified rows. | N/A — local file reads should complete immediately. | Reject ambiguous evidence entries until they map to one canonical issue URL or explicit issue family. |
| End-to-end verifier wrapper | Stop at the first failing phase, retain phase logs under `.tmp/m057-s01/verify/`, and surface which invariant broke. | Mark timeout as a failed phase with the offending command preserved in the log. | Treat malformed JSON, missing rollup sections, or orphan project rows as contract failure, not a warning. |

## Load Profile

- **Shared resources**: local JSON joins, live `gh` refresh commands inside the wrapper, and retained verifier artifacts under `.tmp/m057-s01/verify/`.
- **Per-operation cost**: rebuild snapshots/evidence/ledger once and run two Node contract test files.
- **10x breakpoint**: pagination/rate limits on refresh and large row counts in invariant checks would break first, so the verifier must assert stable counts and join keys explicitly.

## Negative Tests

- **Malformed inputs**: duplicate canonical issue URLs, blank `project_item_id` on a project-backed row, empty `evidence_refs`, and unknown proposed action values.
- **Error paths**: project item with no matching repo issue, issue with no evidence row, or a raw snapshot missing one repo.
- **Boundary conditions**: `68` total rows, `63` joined project rows, `5` blank project rows by design, and `0` orphan project items.

## Steps

1. Add `scripts/lib/m057_reconciliation_ledger.py` that joins the raw issue/project snapshots to the evidence index by canonical issue URL and fails on blanks, orphans, or duplicates.
2. Emit `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json` plus `.gsd/milestones/M057/slices/S01/reconciliation-audit.md`, with grouped rollups for shipped-but-open, rewrite/split, keep-open, misfiled, missing-coverage, and naming-drift buckets.
3. Add `scripts/tests/verify-m057-s01-ledger.test.mjs` and `scripts/verify-m057-s01.sh` so the full refresh/build/assert chain can be replayed and debugged from retained phase logs.

## Must-Haves

- [ ] The final ledger covers all `68` repo issues and all `63` project-backed items using canonical issue URLs as the join key.
- [ ] Every row has non-empty `evidence_refs`, `ownership_truth`, `delivery_truth`, `proposed_repo_action`, and `proposed_project_action`; only the five known non-project rows may omit `project_item_id`.
- [ ] The audit summary clearly groups shipped-but-open, rewrite/split, keep-open, misfiled, missing-coverage, and naming-drift buckets for S02/S03 execution.

## Verification

- `node --test scripts/tests/verify-m057-s01-ledger.test.mjs`
- `bash scripts/verify-m057-s01.sh`

## Observability Impact

- Signals added/changed: verifier phase/status files, ledger rollup counts, and explicit invariant failures for orphan project rows or unclassified issues.
- How a future agent inspects this: run `bash scripts/verify-m057-s01.sh`, then read `.tmp/m057-s01/verify/phase-report.txt`, `current-phase.txt`, and the generated ledger/audit files.
- Failure state exposed: duplicate canonical URLs, orphan project items, empty evidence/action columns, stale snapshots, and canonical naming drift.

## Inputs

- `scripts/lib/m057_tracker_inventory.py` — refresh helper from T01.
- `scripts/lib/m057_evidence_index.py` — code/milestone evidence builder from T02.
- `scripts/tests/verify-m057-s01-inventory.test.mjs` — baseline inventory contract test.
- `.gsd/milestones/M057/slices/S01/mesh-lang-issues.snapshot.json` — normalized language-repo issue inventory.
- `.gsd/milestones/M057/slices/S01/hyperpush-issues.snapshot.json` — normalized product-repo issue inventory.
- `.gsd/milestones/M057/slices/S01/project-items.snapshot.json` — normalized project item inventory with `project_item_id`.
- `.gsd/milestones/M057/slices/S01/project-fields.snapshot.json` — project field schema snapshot.
- `.gsd/milestones/M057/slices/S01/reconciliation-evidence.json` — machine-readable evidence rows from T02.
- `.gsd/milestones/M057/slices/S01/naming-ownership-map.json` — normalized naming and ownership fields from T02.

## Expected Output

- `scripts/lib/m057_reconciliation_ledger.py` — joined ledger builder that enforces row invariants.
- `scripts/tests/verify-m057-s01-ledger.test.mjs` — final ledger invariant test file.
- `scripts/verify-m057-s01.sh` — retained wrapper that refreshes snapshots, rebuilds evidence/ledger outputs, and records phase logs.
- `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json` — final canonical machine-readable ledger.
- `.gsd/milestones/M057/slices/S01/reconciliation-audit.md` — maintainer-readable audit summary and rollups.
