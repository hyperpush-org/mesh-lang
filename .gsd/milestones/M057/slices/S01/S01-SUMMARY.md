---
id: S01
parent: M057
milestone: M057
provides:
  - A canonical 68-row reconciliation ledger keyed by canonical issue URL for all mesh-lang and Hyperpush issue rows.
  - Durable raw GitHub snapshots for both repos and org project #1, including `project_item_id` capture and field snapshots.
  - A reusable evidence/naming index that distinguishes workspace-path truth from public repo truth and normalizes tracker destinations.
  - A fail-closed retained verifier and audit summary that downstream reconciliation slices can replay before mutating GitHub.
requires:
  []
affects:
  - S02
  - S03
key_files:
  - scripts/lib/m057_tracker_inventory.py
  - scripts/lib/m057_project_items.graphql
  - scripts/lib/m057_evidence_index.py
  - scripts/lib/m057_reconciliation_ledger.py
  - scripts/tests/verify-m057-s01-inventory.test.mjs
  - scripts/tests/verify-m057-s01-evidence.test.mjs
  - scripts/tests/verify-m057-s01-ledger.test.mjs
  - scripts/verify-m057-s01.sh
  - .gsd/milestones/M057/slices/S01/mesh-lang-issues.snapshot.json
  - .gsd/milestones/M057/slices/S01/hyperpush-issues.snapshot.json
  - .gsd/milestones/M057/slices/S01/project-items.snapshot.json
  - .gsd/milestones/M057/slices/S01/project-fields.snapshot.json
  - .gsd/milestones/M057/slices/S01/reconciliation-evidence.json
  - .gsd/milestones/M057/slices/S01/reconciliation-evidence.md
  - .gsd/milestones/M057/slices/S01/naming-ownership-map.json
  - .gsd/milestones/M057/slices/S01/reconciliation-ledger.json
  - .gsd/milestones/M057/slices/S01/reconciliation-audit.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D455: persist normalized raw tracker snapshots keyed by canonical issue URL with schema-seeded field maps and explicit hyperpush alias canonicalization.
  - D456: preserve `workspace_path_truth`, `public_repo_truth`, and `normalized_canonical_destination` as separate fields instead of collapsing product naming/ownership into one string.
  - D457: keep the canonical ledger at 68 issue-backed rows and surface uncovered shipped work like `/pitch` through `derived_gaps` rather than synthetic issue rows.
patterns_established:
  - Use `canonical_issue_url` as the only cross-repo join key from raw snapshots through the final ledger.
  - Treat GitHub Projects V2 field values as sparse input: seed tracked field keys from the schema first and backfill missing values to `null`.
  - Keep workspace naming truth separate from public repo truth so `hyperpush-mono` local paths and `hyperpush` tracker destinations can both remain explicit.
  - Represent missing tracker coverage in a `derived_gaps` section rather than inventing synthetic issue rows that would corrupt row-count invariants.
  - Use a retained wrapper verifier with named phase logs and status markers so reconciliation failures stay diagnosable.
observability_surfaces:
  - `.gsd/milestones/M057/slices/S01/*snapshot.json` exposes `captured_at`, canonical repo identity, project field schema, and normalized project item data.
  - `.gsd/milestones/M057/slices/S01/reconciliation-evidence.md` and `reconciliation-audit.md` give human-readable inspection surfaces for ownership, delivery, and proposed action truth.
  - `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json` publishes the canonical 68-row joined action ledger plus `derived_gaps`.
  - `.tmp/m057-s01/verify/phase-report.txt`, `current-phase.txt`, `status.txt`, and the numbered phase logs expose retained verification state and failure location.
drill_down_paths:
  - .gsd/milestones/M057/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M057/slices/S01/tasks/T02-SUMMARY.md
  - .gsd/milestones/M057/slices/S01/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-10T06:26:24.509Z
blocker_discovered: false
---

# S01: Audit code reality and build the reconciliation ledger

**Built and verified the canonical M057/S01 tracker reconciliation artifacts: live GitHub snapshots, a reusable evidence/naming index, and a 68-row joined ledger with fail-closed verification and retained audit logs.**

## What Happened

S01 converted the tracker-reconciliation problem from ad hoc GitHub archaeology into one deterministic, code-backed row set. The slice now persists four live snapshot artifacts under `.gsd/milestones/M057/slices/S01/`: `mesh-lang-issues.snapshot.json`, `hyperpush-issues.snapshot.json`, `project-items.snapshot.json`, and `project-fields.snapshot.json`. Those snapshots capture canonical repo identity, the `hyperpush-mono` → `hyperpush` alias proof, `project_item_id` values, and schema-seeded project field maps so later work does not have to trust sparse GraphQL payloads or stale issue prose.

On top of the raw capture layer, S01 added a reusable evidence/naming layer. `reconciliation-evidence.json`/`.md` and `naming-ownership-map.json` classify the shipped launch foundations, active product work, misfiled docs work (`hyperpush#8`), missing `/pitch` tracker coverage, and the split-boundary naming drift between local `hyperpush-mono` workspace paths and the public `hyperpush` repo identity. The key pattern established here is that ownership truth is not one field: `workspace_path_truth`, `public_repo_truth`, and `normalized_canonical_destination` must all be preserved separately so downstream mutation work can normalize tracker wording without losing the concrete file-path evidence that proved the classification.

The final join layer now lives in `reconciliation-ledger.json` and `reconciliation-audit.md`. The ledger carries exact issue-backed coverage for all 68 repo rows, joins 63 project-backed items through `project_item_id`, preserves the 5 explicit non-project rows, and keeps missing tracker coverage in a separate `derived_gaps` section instead of inventing synthetic issue rows. The current rollup is 13 `shipped-but-open`, 21 `rewrite-split`, 33 `keep-open`, and 1 `misfiled`, with 14 rows flagged for naming drift and one explicit missing-coverage gap for the shipped `/pitch` route. That gives S02 one canonical mutation target per issue and gives S03 one canonical project row set to realign, without forcing either slice to re-query or reinterpret GitHub data.

The retained verifier under `.tmp/m057-s01/verify/` now makes the slice diagnosable instead of opaque. The wrapper replays inventory refresh, evidence build, ledger build, both Node contract suites, and the ledger-surface checks, then leaves named logs plus `current-phase.txt=complete` and `status.txt=ok`. The raw snapshots, audit markdown, and retained verify tree are now the inspection surfaces future slices should use first when a tracker classification looks suspicious.

## Operational Readiness

- **Health signal:** `.tmp/m057-s01/verify/status.txt` must read `ok`, `.tmp/m057-s01/verify/current-phase.txt` must read `complete`, and `reconciliation-ledger.json` rollup must stay at 68 rows / 63 project-backed rows / 5 non-project rows / 0 orphan project rows.
- **Failure signal:** any missing `project_item_id` on a project-backed row, duplicate canonical issue URL, unknown action enum, orphan project item, broken `hyperpush-mono` alias canonicalization, or empty evidence/action field fails the Node tests or the wrapper with a named phase.
- **Recovery procedure:** rerun `bash scripts/verify-m057-s01.sh`; inspect `.tmp/m057-s01/verify/phase-report.txt` and the numbered `*.stdout` / `*.stderr` logs for the failed phase; repair the raw snapshot/evidence/join invariant at the source script rather than hand-editing the generated JSON; rerun the wrapper until the phase markers return to `complete` / `ok`.
- **Monitoring gaps:** freshness is capture-time based, not push-driven. The ledger is truthful for the recorded `captured_at` snapshot, but S02/S03 should rerun the verifier immediately before mutating GitHub if the tracker may have changed since the last capture.

This slice does not yet change GitHub itself. It intentionally stops at the canonical audit ledger so later reconciliation work can close, rewrite, move, create, or reproject items from one evidence-backed source of truth instead of from memory.

## Verification

Passed all slice-level verification rails:

- `node --test scripts/tests/verify-m057-s01-inventory.test.mjs`
- `node --test scripts/tests/verify-m057-s01-ledger.test.mjs`
- `bash scripts/verify-m057-s01.sh`

Also confirmed the observability surfaces required by the slice plan: the live snapshots carry `captured_at` metadata, `reconciliation-audit.md` renders the grouped bucket rollup and sample actions, and `.tmp/m057-s01/verify/phase-report.txt`, `current-phase.txt`, and `status.txt` all show a completed green retained replay.

## Requirements Advanced

- R128 — Published canonical issue-backed language-repo classifications with code-backed evidence, ownership truth, and repo/project action proposals for every relevant `mesh-lang` issue row.
- R129 — Published canonical issue-backed product-repo classifications against the sibling Hyperpush code state, including naming normalization from local `hyperpush-mono` paths to the public `hyperpush` repo.
- R130 — Captured 63 project-backed org-project items with `project_item_id` and field snapshots, then joined them into one zero-orphan reconciliation ledger.
- R131 — Surfaced the shipped `/pitch` route as explicit missing tracker coverage with concrete create-issue and create-project-item actions instead of hiding the gap.
- R132 — Separated rows into shipped-but-open, rewrite/split, keep-open, and misfiled buckets so later cleanup can preserve history rather than silently repurposing tracker entries.
- R133 — Encoded `workspace_path_truth`, `public_repo_truth`, and normalized destination fields throughout the evidence map and ledger so tracker ownership/naming can be corrected deterministically.
- R134 — Published `reconciliation-audit.md` and `reconciliation-ledger.json` so a later maintainer can inspect shipped, active, misfiled, and missing work from one canonical artifact set.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None beyond the intentional slice shape captured in D457: missing tracker coverage such as `/pitch` is represented in `derived_gaps` instead of synthetic issue rows so the 68-row issue-backed ledger contract stays exact.

## Known Limitations

GitHub itself has not been mutated yet; the ledger is the canonical action plan for S02 and S03, not the final external tracker state. Snapshot freshness is point-in-time (`captured_at` metadata is recorded in the raw artifacts), so downstream mutation slices should rerun the retained wrapper immediately before applying changes if remote tracker state may have drifted.

## Follow-ups

S02 should apply the repo-side mutations directly from `reconciliation-ledger.json`: close shipped-but-open rows, rewrite/split drifted rows, move the misfiled `hyperpush#8` docs work, and create the missing `/pitch` issue. S03 should then realign org project #1 from the same canonical row set so project status derives from reconciled issue truth instead of stale portfolio state.

## Files Created/Modified

- `scripts/lib/m057_tracker_inventory.py` — Captures live repo issue inventories, canonical repo identity, and org-project snapshots into normalized JSON artifacts.
- `scripts/lib/m057_project_items.graphql` — Defines the paginated GitHub GraphQL query used to snapshot project items and their sparse field values.
- `scripts/lib/m057_evidence_index.py` — Builds the code-backed evidence bundle and naming/ownership map used by the final join layer.
- `scripts/lib/m057_reconciliation_ledger.py` — Joins snapshots and evidence into the final fail-closed reconciliation ledger and audit outputs.
- `scripts/tests/verify-m057-s01-inventory.test.mjs` — Verifies current inventory counts and negative cases around snapshot shape and repo canonicalization.
- `scripts/tests/verify-m057-s01-evidence.test.mjs` — Verifies evidence-index coverage and fail-closed naming/ownership invariants.
- `scripts/tests/verify-m057-s01-ledger.test.mjs` — Verifies joined ledger counts, actions, and negative cases like orphan project rows and blank project ids.
- `scripts/verify-m057-s01.sh` — Runs the full retained slice replay and leaves per-phase diagnostics under `.tmp/m057-s01/verify/`.
- `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json` — Publishes the canonical 68-row joined reconciliation ledger plus the explicit `derived_gaps` section.
- `.gsd/milestones/M057/slices/S01/reconciliation-audit.md` — Publishes the grouped human-readable audit summary for shipped, rewrite/split, keep-open, misfiled, and missing-coverage rows.
