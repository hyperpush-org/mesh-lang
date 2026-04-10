---
estimated_steps: 4
estimated_files: 4
skills_used:
  - gh
---

# T02: Apply project deletes, adds, and field edits from the checked manifest

**Slice:** S03 — Realign org project #1 to the reconciled issue truth
**Milestone:** M057

## Description

Make org project #1 truthful by applying only the operations published by the checked S03 plan. This task adds the live applicator, executes deletes/adds/field edits in deterministic order with the captured project/field schema, preserves explicit live non-null field values unless the checked plan changes them, and persists canonical final row state in results artifacts. The apply path must be safe to rerun and must never improvise field ids, option ids, or issue mappings outside the checked manifest.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `.gsd/milestones/M057/slices/S03/project-mutation-plan.json` | Refuse to mutate the board if the plan artifact is missing, red, or inconsistent with the live project id/field schema. | N/A — local plan reads should fail immediately. | Reject unknown operation kinds, missing item ids, or field edits that do not name a captured field/option id. |
| Live `gh project item-delete` / `item-add` / `item-edit` calls | Stop the batch, persist the last attempted operation plus command evidence, and leave results marked incomplete instead of guessing success. | Abort the current phase, record timeout details, and rely on rerun-safe `already_satisfied` detection for recovery. | Treat unexpected response shapes or missing returned item ids as failures and do not continue with downstream edits. |
| `.gsd/milestones/M057/slices/S01/project-fields.snapshot.json` | Fail closed if the project id, field ids, or single-select option ids do not match what the plan expects. | N/A — local snapshot reads should fail immediately. | Reject any write that would require a hardcoded id or an option not present in the captured schema. |

## Load Profile

- **Shared resources**: GitHub ProjectV2 write APIs, the captured field schema, and the touched board rows from the S03 plan.
- **Per-operation cost**: one delete/add per membership change plus one `gh project item-edit` call per field update because GitHub edits only one field per invocation.
- **10x breakpoint**: rate limits and item-edit fanout are the first bottlenecks; the applicator must keep deterministic ordering and idempotence instead of batching speculative writes.

## Negative Tests

- **Malformed inputs**: missing project id, stale item id, unknown field option id, and results artifact rows that omit final item or issue identity.
- **Error paths**: delete already-removed rows, add already-present canonical rows, edit a row whose live state already matches the plan, and recover cleanly on partial-batch interruption.
- **Boundary conditions**: touching only the planned rows, preserving already-correct non-null metadata, and proving rerun safety through `already_satisfied` outcomes.

## Steps

1. Add `scripts/lib/m057_project_mutation_apply.py` to read the checked plan artifact, validate schema ids, and expose `--check` plus explicit `--apply` modes.
2. Apply deletes/adds/field edits in deterministic order, capturing command logs, canonical issue identity, final item ids, and final tracked field values for every touched row.
3. Render `.gsd/milestones/M057/slices/S03/project-mutation-results.json` plus `.md` with rollups for removed rows, canonical additions/repoints, field edits, and representative done/active/todo coverage.
4. Re-run the apply path to prove idempotence so later automation can distinguish `already_satisfied` state from a failed partial batch.

## Must-Haves

- [ ] The applicator mutates org project #1 only from the checked S03 plan artifact and the captured field schema.
- [ ] The results artifact records final item ids, canonical issue URLs, per-operation status, and final tracked field values for touched rows.
- [ ] Re-running the applicator proves resume safety by recording `already_satisfied` outcomes instead of duplicating board edits.

## Verification

- `python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --check`
- `python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --apply`

## Observability Impact

- Signals added/changed: per-command status, last attempted operation, final board row snapshots, and idempotence rollups.
- How a future agent inspects this: open `.gsd/milestones/M057/slices/S03/project-mutation-results.json` / `.md` and the command evidence persisted by the applicator.
- Failure state exposed: whether a project row failed during delete, add, field edit, or final-state replay.

## Inputs

- `scripts/lib/m057_project_mutation_plan.py` — checked planner implementation consumed by the applicator precheck.
- `.gsd/milestones/M057/slices/S03/project-mutation-plan.json` — canonical delete/add/update manifest for S03.
- `.gsd/milestones/M057/slices/S01/project-fields.snapshot.json` — stable project id, field ids, and option ids used for all writes.
- `.gsd/milestones/M057/slices/S02/repo-mutation-results.json` — canonical old→new issue mappings and post-S02 repo truth used by the plan.

## Expected Output

- `scripts/lib/m057_project_mutation_apply.py` — plan-driven org project applicator with rerun-safe behavior.
- `.gsd/milestones/M057/slices/S03/project-mutation-results.json` — machine-readable live mutation results with final row snapshots.
- `.gsd/milestones/M057/slices/S03/project-mutation-results.md` — compact human-readable board summary and S03 handoff.
- `.tmp/m057-s03/apply/last-operation.txt` — retained apply-phase pointer to the last attempted project mutation for recovery/debugging.
