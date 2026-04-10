---
estimated_steps: 3
estimated_files: 4
skills_used:
  - gh
  - test
---

# T01: Generate a dry-run repo mutation manifest from the S01 ledger

Build the mutation manifest first so live repo edits happen from one explicit checked artifact instead of ad hoc `gh` commands. This task turns the S01 ledger into one deterministic touched set, generates truthful replacement titles/bodies/comments using the existing repo issue shapes, and proves the plan excludes rows that should remain untouched.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json` + issue snapshots | Fail closed and refuse to emit a plan if any required row, bucket, or snapshot field is missing. | N/A — local file reads should fail immediately instead of retrying silently. | Reject duplicate/unknown action kinds, missing `derived_gaps`, or row counts that do not match the expected S01 buckets. |
| `.github/ISSUE_TEMPLATE/feature_request.yml` + `../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml` | Fall back to a simple heading-based body renderer only if the checked-in templates are unreadable, and record that fallback in the plan artifact. | N/A — local template reads should fail fast. | Reject template shapes that cannot provide stable section headings for rewritten issues. |

## Load Profile

- **Shared resources**: local JSON artifacts under `.gsd/milestones/M057/slices/S02/` and body generation for 43 planned repo mutations.
- **Per-operation cost**: one ledger row classification plus title/body/comment rendering for each touched issue, plus one derived-gap expansion for `/pitch`.
- **10x breakpoint**: artifact size and template/body generation would grow first; the planner must stream or build rows deterministically instead of relying on hand-curated one-offs.

## Negative Tests

- **Malformed inputs**: missing `project_item_id` on project-backed rows, unknown `proposed_repo_action_kind`, duplicate canonical issue handles, and absent `/pitch` `derived_gaps` entries.
- **Error paths**: missing snapshot files, stale repo alias assumptions, or an attempt to plan operations for already-closed `hyperpush#3/#4/#5`.
- **Boundary conditions**: exactly 10 closeouts, exactly 31 open-issue rewrites/normalizations, exactly 1 transfer, and exactly 1 retrospective issue creation.

## Steps

1. Add a plan builder that consumes the S01 ledger and before-state snapshots, expands only the live repo mutations, and emits a stable machine-readable manifest plus a human-readable summary under `.gsd/milestones/M057/slices/S02/`.
2. Generate replacement titles/bodies/comments for each mutation using the existing repo issue headings, including close comments that call out residual child follow-up where a shipped parent closes while a child issue remains open.
3. Add a plan contract test that proves the manifest covers the right buckets, preserves `hyperpush#8` as a transfer, expands `/pitch` into one created issue, and leaves already-correct closed rows out of the apply set.

## Must-Haves

- [ ] The plan artifact is the only allowed input to live mutations; later tasks must not execute directly from the ledger.
- [ ] Planned operations include before/after repo identity, canonical issue handle, old/new title/body intent, and comment text for close/create cases.
- [ ] The manifest explicitly records that `hyperpush#8` transfers to `mesh-lang` and that `/pitch` becomes one dedicated retrospective `hyperpush` issue.

## Inputs

- `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json` — canonical row-by-row repo action source from S01.
- `.gsd/milestones/M057/slices/S01/reconciliation-audit.md` — grouped bucket summary used for spot-checking touched issue sets.
- `.gsd/milestones/M057/slices/S01/mesh-lang-issues.snapshot.json` — before-state titles, bodies, labels, and issue URLs for `mesh-lang` rows.
- `.gsd/milestones/M057/slices/S01/hyperpush-issues.snapshot.json` — before-state titles, bodies, labels, and issue URLs for `hyperpush` rows.
- `scripts/lib/m057_tracker_inventory.py` — reusable `gh` JSON helpers and repo constants.
- `scripts/lib/m057_reconciliation_ledger.py` — canonical action enum definitions.
- `.github/ISSUE_TEMPLATE/feature_request.yml` — current `mesh-lang` issue body shape.
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml` — current product issue body shape.

## Expected Output

- `scripts/lib/m057_repo_mutation_plan.py` — dry-run planner that expands S01 ledger rows into explicit repo mutation operations.
- `scripts/tests/verify-m057-s02-plan.test.mjs` — contract test for planned mutation counts, identity changes, and excluded untouched rows.
- `.gsd/milestones/M057/slices/S02/repo-mutation-plan.json` — machine-readable manifest with one row per live repo mutation plus generated body/comment payloads.
- `.gsd/milestones/M057/slices/S02/repo-mutation-plan.md` — human-readable plan summary grouped by close, rewrite, transfer, and create actions.

## Verification

- `python3 scripts/lib/m057_repo_mutation_plan.py --output-dir .gsd/milestones/M057/slices/S02 --check`
- `node --test scripts/tests/verify-m057-s02-plan.test.mjs`

## Observability Impact

- Signals added/changed: the plan manifest records mutation bucket, destination repo, generated title/body/comment payloads, and planned canonical URL changes.
- How a future agent inspects this: open `repo-mutation-plan.json`/`.md` or rerun `node --test scripts/tests/verify-m057-s02-plan.test.mjs`.
- Failure state exposed: missing bucket coverage, accidental extra mutations, or loss of the `/pitch` / `hyperpush#8` identity-changing operations.
