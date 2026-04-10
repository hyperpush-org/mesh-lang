---
estimated_steps: 3
estimated_files: 7
skills_used:
  - gh
  - test
---

# T01: Capture live issue and project snapshots with baseline inventory assertions

**Slice:** S01 — Audit code reality and build the reconciliation ledger
**Milestone:** M057

## Description

Establish the live-data truth surface first. Later reconciliation work must mutate GitHub from durable raw snapshots with explicit `project_item_id` and field values, not from ad hoc terminal output or issue prose. This task creates the refresh path and the first contract test so count drift and canonical repo drift fail immediately.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `gh issue list` / `gh repo view` | Stop the refresh and keep partial output out of the slice directory; surface which repo command failed. | Fail the refresh and leave the previous snapshot untouched; report which command stalled. | Reject the refresh if the JSON shape changes or canonical repo identity cannot be resolved explicitly. |
| `gh project field-list` / GraphQL item query | Fail closed instead of guessing `project_item_id` or missing board fields. | Abort project capture and report the missing board phase separately from repo issue capture. | Reject rows whose field schema cannot be normalized to stable names/ids. |

## Load Profile

- **Shared resources**: GitHub API rate limits, the local `gh` CLI session, and JSON writes under `.gsd/milestones/M057/slices/S01/`.
- **Per-operation cost**: two repo issue queries, one canonical repo lookup, one project field schema query, and one project items query.
- **10x breakpoint**: project pagination and GitHub rate limits would break first, so the helper must preserve cursors/limits instead of assuming one small page forever.

## Negative Tests

- **Malformed inputs**: missing labels arrays, issues without `closedAt`, and project items with blank field values.
- **Error paths**: `gh` unavailable, GraphQL returns permission/rate-limit errors, or `hyperpush-org/hyperpush-mono` resolves unexpectedly.
- **Boundary conditions**: open non-project issue `hyperpush#8`, closed non-project issues `hyperpush#2`–`#5`, and exactly `63` project-backed rows.

## Steps

1. Add `scripts/lib/m057_tracker_inventory.py` plus a checked-in GraphQL query file that capture both repo issue inventories, canonical product repo identity, project field schema, and project item rows with stable keys.
2. Write normalized snapshot files under `.gsd/milestones/M057/slices/S01/` with capture metadata and canonical issue URLs so later tasks can join them deterministically.
3. Add `scripts/tests/verify-m057-s01-inventory.test.mjs` to assert the research-hand-off counts and canonical repo assumptions.

## Must-Haves

- [ ] Snapshot JSON files carry stable keys, source metadata, and canonical issue URLs for both repos plus project data.
- [ ] `project_item_id` and project field snapshots are captured now, not deferred to S03.
- [ ] The inventory contract test proves `68` total repo issues, `63` project-backed rows, and the `5` explicit non-project hyperpush issues.

## Verification

- `python3 scripts/lib/m057_tracker_inventory.py --output-dir .gsd/milestones/M057/slices/S01 --refresh --check`
- `node --test scripts/tests/verify-m057-s01-inventory.test.mjs`

## Observability Impact

- Signals added/changed: raw snapshot metadata capturing source command, canonical repo identity, and capture time for each inventory file.
- How a future agent inspects this: open the snapshot JSON files or rerun `node --test scripts/tests/verify-m057-s01-inventory.test.mjs` to localize which inventory drifted.
- Failure state exposed: missing project items, count drift, malformed field snapshots, and canonical repo mismatches.

## Inputs

- `.gsd/PROJECT.md` — current repo-boundary truth and the M057 framing.
- `.gsd/REQUIREMENTS.md` — requirement coverage for `R128`–`R134` and the `R133` naming constraint.
- `.gsd/milestones/M057/slices/S01/S01-RESEARCH.md` — expected counts, gap cases, and the preferred issue-centric ledger shape.
- `scripts/lib/repo-identity.json` — current language/product repo identity values, including stale `hyperpush-mono` public naming.
- `scripts/lib/m055-workspace.sh` — sibling-product resolution helper and canonical repo slug helpers.

## Expected Output

- `scripts/lib/m057_tracker_inventory.py` — refresh helper for repo issues, project fields, and project item inventory.
- `scripts/lib/m057_project_items.graphql` — explicit project item query with field snapshots and `project_item_id` capture.
- `scripts/tests/verify-m057-s01-inventory.test.mjs` — fail-closed raw inventory count assertions.
- `.gsd/milestones/M057/slices/S01/mesh-lang-issues.snapshot.json` — normalized `mesh-lang` issue inventory.
- `.gsd/milestones/M057/slices/S01/hyperpush-issues.snapshot.json` — normalized `hyperpush` issue inventory.
- `.gsd/milestones/M057/slices/S01/project-items.snapshot.json` — normalized org project item inventory keyed to issue URLs.
- `.gsd/milestones/M057/slices/S01/project-fields.snapshot.json` — current org project field schema snapshot.
