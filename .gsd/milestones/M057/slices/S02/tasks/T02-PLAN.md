---
estimated_steps: 3
estimated_files: 4
skills_used:
  - gh
  - bash-scripting
---

# T02: Apply transfer, retrospective create, closeouts, and rewrites from the checked plan

Execute the live GitHub mutations only after the plan is explicit and tested. This task applies the identity-changing operations first, then bulk-closes and bulk-rewrites the remaining touched issues, and records every live mutation result in a durable JSON artifact that S03 can consume without rediscovery.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `gh issue transfer` / `gh issue create` | Stop the apply run before bulk edits, persist the failed operation and stderr, and leave later operations unapplied. | Abort the apply run and record the last attempted identity-changing operation so a retry does not guess whether transfer/create succeeded. | Reject output that does not contain a canonical destination URL/number for transfer/create. |
| `gh issue close` / `gh issue edit` | Record the failed issue handle, halt the batch, and keep prior successful mutations in the results artifact instead of continuing blindly. | Abort the run and surface which issue stalled so follow-up can resume from the checked plan. | Reject empty or non-canonical issue URLs, missing state transitions, or responses that do not match the target repo/issue. |
| `repo-mutation-plan.json` | Refuse `--apply` unless the plan file exists and passes the task-1 contract checks. | N/A — local file validation should fail immediately. | Reject rows whose operation kind, destination repo, or generated payload does not match the expected manifest schema. |

## Load Profile

- **Shared resources**: GitHub issue mutation rate limits, the authenticated `gh` session, and the S02 results artifact being appended as operations complete.
- **Per-operation cost**: 1 transfer, 1 create, 10 close operations, and 31 edit operations against live repos.
- **10x breakpoint**: GitHub rate limiting and partial-run recovery would break first; the applicator must be resumable and record already-satisfied operations instead of duplicating them.

## Negative Tests

- **Malformed inputs**: missing generated body/comment text, invalid repo slug, or a plan row that points at an issue no longer matching the expected current repo.
- **Error paths**: transfer drops repo-specific labels, create returns a closed issue URL unexpectedly, or a close/edit request fails mid-batch.
- **Boundary conditions**: rerunning after a partial success, applying a plan when `hyperpush#8` already moved, and applying after `/pitch` already exists and is closed.

## Steps

1. Add a plan-driven applicator with `--dry-run` default and explicit `--apply` mode, and make it refuse to mutate anything unless task-1 plan checks pass first.
2. Execute `hyperpush#8` transfer and `/pitch` issue creation/close first, capture the returned canonical URLs/numbers, then apply the remaining close and rewrite operations in deterministic order.
3. Persist one durable results artifact that records each operation’s requested action, live outcome, old/new canonical handles or URLs, skipped/already-satisfied reason, and final repo/issue state snapshot.

## Must-Haves

- [ ] `hyperpush#8` is transferred instead of recreated, and the results artifact captures both the old `hyperpush#8` identity and the new `mesh-lang#?` canonical destination.
- [ ] `/pitch` gets one explicit `hyperpush` issue row with an evidence-backed retrospective body and completed close comment.
- [ ] The batch is safe to rerun: already-satisfied closes/rewrites/transfers/creates are recorded as such instead of duplicating repo changes.

## Inputs

- `scripts/lib/m057_repo_mutation_plan.py` — checked planner and manifest schema from T01.
- `.gsd/milestones/M057/slices/S02/repo-mutation-plan.json` — approved live mutation manifest.
- `.gsd/milestones/M057/slices/S01/mesh-lang-issues.snapshot.json` — before-state reference for `mesh-lang` mutations.
- `.gsd/milestones/M057/slices/S01/hyperpush-issues.snapshot.json` — before-state reference for `hyperpush` mutations.
- `scripts/lib/m057_tracker_inventory.py` — reusable `gh` execution helpers and repo constants.

## Expected Output

- `scripts/lib/m057_repo_mutation_apply.py` — plan-driven live GitHub applicator with dry-run/apply and resume-safe behavior.
- `.gsd/milestones/M057/slices/S02/repo-mutation-results.json` — durable per-operation outcome log with old/new canonical issue mapping and final live state snapshots.

## Verification

- `python3 scripts/lib/m057_repo_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S02 --check`
- `python3 scripts/lib/m057_repo_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S02 --apply`

## Observability Impact

- Signals added/changed: per-operation apply status, timestamped mutation ordering, transfer/create canonical URL capture, and already-satisfied/skipped reasons.
- How a future agent inspects this: open `repo-mutation-results.json` or rerun the applicator in `--check` mode to validate the manifest before another apply.
- Failure state exposed: the last attempted issue handle, failed command family, raw `gh` stderr/stdout, and whether the partial batch mutated live state before stopping.
