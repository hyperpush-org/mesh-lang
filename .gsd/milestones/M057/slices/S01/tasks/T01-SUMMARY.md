---
id: T01
parent: S01
milestone: M057
key_files:
  - scripts/lib/m057_tracker_inventory.py
  - scripts/lib/m057_project_items.graphql
  - scripts/tests/verify-m057-s01-inventory.test.mjs
  - scripts/tests/verify-m057-s01-ledger.test.mjs
  - .gsd/milestones/M057/slices/S01/mesh-lang-issues.snapshot.json
  - .gsd/milestones/M057/slices/S01/hyperpush-issues.snapshot.json
  - .gsd/milestones/M057/slices/S01/project-fields.snapshot.json
  - .gsd/milestones/M057/slices/S01/project-items.snapshot.json
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D455: persist normalized tracker snapshots keyed by canonical issue URL with schema-seeded tracked project field maps instead of joining later work against ad hoc gh output.
duration: 
verification_result: mixed
completed_at: 2026-04-10T04:54:56.268Z
blocker_discovered: false
---

# T01: Added live GitHub tracker snapshot capture, normalized inventory snapshots, and fail-closed inventory contract tests.

**Added live GitHub tracker snapshot capture, normalized inventory snapshots, and fail-closed inventory contract tests.**

## What Happened

Built scripts/lib/m057_tracker_inventory.py plus scripts/lib/m057_project_items.graphql to capture live mesh-lang issues, hyperpush issues, canonical repo identity, org project field schema, and paginated project item rows into four normalized snapshots under .gsd/milestones/M057/slices/S01/. The snapshots now carry canonical issue URLs, explicit hyperpush-mono -> hyperpush canonicalization proof, project_item_id values, and schema-seeded tracked project field maps with null backfill for missing board fields. Added scripts/tests/verify-m057-s01-inventory.test.mjs to prove the committed counts and negative cases, and added scripts/tests/verify-m057-s01-ledger.test.mjs as the fail-closed downstream slice rail referenced by the slice verification list. Recorded the sparse ProjectV2 field-value gotcha in .gsd/KNOWLEDGE.md and saved decision D455 for the canonical-URL snapshot shape.

## Verification

`python3 scripts/lib/m057_tracker_inventory.py --output-dir .gsd/milestones/M057/slices/S01 --refresh --check` passed and rewrote the live snapshots with the expected 68 repo issues, 63 project items, and non-project hyperpush issues 2/3/4/5/8. `node --test scripts/tests/verify-m057-s01-inventory.test.mjs` passed all eight current and negative contract cases. As expected for an intermediate task, `node --test scripts/tests/verify-m057-s01-ledger.test.mjs` still fails because T03 has not published reconciliation-ledger.json yet, and `bash scripts/verify-m057-s01.sh` still exits 127 because the assembled slice wrapper does not exist yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `python3 scripts/lib/m057_tracker_inventory.py --output-dir .gsd/milestones/M057/slices/S01 --refresh --check` | 0 | ✅ pass | 10778ms |
| 2 | `node --test scripts/tests/verify-m057-s01-inventory.test.mjs` | 0 | ✅ pass | 21174ms |
| 3 | `node --test scripts/tests/verify-m057-s01-ledger.test.mjs` | 1 | ❌ fail | 3542ms |
| 4 | `bash scripts/verify-m057-s01.sh` | 127 | ❌ fail | 44ms |

## Deviations

Added scripts/tests/verify-m057-s01-ledger.test.mjs one task earlier than the task plan itself called for, because this was the first task in the slice and the slice verification section already referenced that future test file. The placeholder intentionally fails closed until T03 publishes the final ledger outputs.

## Known Issues

`.gsd/milestones/M057/slices/S01/reconciliation-ledger.json` and `.gsd/milestones/M057/slices/S01/reconciliation-audit.md` do not exist yet, so `node --test scripts/tests/verify-m057-s01-ledger.test.mjs` currently fails by design. `scripts/verify-m057-s01.sh` also does not exist yet, so the assembled slice wrapper command exits 127 until T03 lands.

## Files Created/Modified

- `scripts/lib/m057_tracker_inventory.py`
- `scripts/lib/m057_project_items.graphql`
- `scripts/tests/verify-m057-s01-inventory.test.mjs`
- `scripts/tests/verify-m057-s01-ledger.test.mjs`
- `.gsd/milestones/M057/slices/S01/mesh-lang-issues.snapshot.json`
- `.gsd/milestones/M057/slices/S01/hyperpush-issues.snapshot.json`
- `.gsd/milestones/M057/slices/S01/project-fields.snapshot.json`
- `.gsd/milestones/M057/slices/S01/project-items.snapshot.json`
- `.gsd/KNOWLEDGE.md`
