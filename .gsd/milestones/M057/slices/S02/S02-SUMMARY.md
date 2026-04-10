---
id: S02
parent: M057
milestone: M057
provides:
  - Truthful repo issue state in both repos: 10 shipped `mesh-lang` issues closed, 21 `hyperpush` `rewrite_scope` rows rewritten, 7 mock-backed follow-through rows kept open with accurate wording, and `hyperpush#54/#55/#56` normalized to public `hyperpush` naming.
  - Canonical identity mappings for downstream work: `hyperpush#8 -> mesh-lang#19` and `/pitch -> hyperpush#58`.
  - A deterministic dry-run plan artifact, a live results artifact, and a retained live-state verifier that future maintainers can replay without reopening S01 archaeology.
  - A compact S03 handoff markdown surface that gives the next slice exact canonical URLs/numbers and repo totals before project-board realignment.
requires:
  - slice: S01
    provides: Canonical issue snapshots, evidence/naming truth, and the joined reconciliation ledger with derived `/pitch` gap.
affects:
  - S03
key_files:
  - scripts/lib/m057_repo_mutation_plan.py
  - scripts/lib/m057_repo_mutation_apply.py
  - scripts/tests/verify-m057-s02-plan.test.mjs
  - scripts/tests/verify-m057-s02-results.test.mjs
  - scripts/verify-m057-s02.sh
  - .gsd/milestones/M057/slices/S02/repo-mutation-plan.json
  - .gsd/milestones/M057/slices/S02/repo-mutation-plan.md
  - .gsd/milestones/M057/slices/S02/repo-mutation-results.json
  - .gsd/milestones/M057/slices/S02/repo-mutation-results.md
  - .tmp/m057-s02/verify/phase-report.txt
  - .tmp/m057-s02/verify/verification-summary.json
key_decisions:
  - D458: drive all live repo corrections from generated plan/results artifacts, transfer `hyperpush#8` instead of recreating it, and create one retrospective `/pitch` issue before closing it as shipped.
  - D459: verify post-mutation live state by replaying `gh issue view` against the persisted `final_state` snapshots in `repo-mutation-results.json`, and treat `hyperpush#8` source-repo lookup failure as expected absence while canonical identity comes from the results artifact.
  - Treat redirected `gh api` reads of transferred issues as success when GitHub returns the destination-repo payload, and resolve only destination-repo-assignable labels rather than mutating repo label catalogs mid-batch.
  - Prove rerun safety with a second live `--apply` pass so the final artifact demonstrates `already_satisfied` idempotence instead of assuming it.
patterns_established:
  - Expand risky external mutations into checked plan artifacts first, then drive the live batch only from that manifest.
  - Capture canonical old→new issue mappings for transfers and retrospective creates up front and persist them in results artifacts for downstream consumers.
  - Prove rerun safety with a second live apply pass so results artifacts record idempotence instead of relying on best-effort assumptions.
  - Verify live tracker state by replaying `gh issue view` against persisted `final_state` snapshots, and treat transferred-source absence as explicit evidence rather than as an error to paper over.
observability_surfaces:
  - .gsd/milestones/M057/slices/S02/repo-mutation-plan.json and `.md` publish the checked touched set, replacement payloads, and identity-changing operations before any live mutation occurs.
  - .gsd/milestones/M057/slices/S02/repo-mutation-results.json records per-operation status, timestamps, canonical old→new mappings, label resolution, and persisted `final_state` snapshots for all touched issues.
  - .gsd/milestones/M057/slices/S02/repo-mutation-results.md is the human-readable S03 handoff for canonical issue URLs/numbers, repo totals, and bucket rollups.
  - .tmp/m057-s02/verify/phase-report.txt, `verification-summary.json`, `last-target.txt`, and the `commands/` logs expose the retained verifier's phase health, last attempted issue, and underlying `gh` command evidence.
drill_down_paths:
  - .gsd/milestones/M057/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M057/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M057/slices/S02/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-10T17:05:31.087Z
blocker_discovered: false
---

# S02: Reconcile repo issues across `mesh-lang` and `hyperpush`

**Applied the S01 reconciliation ledger to the live `mesh-lang` and `hyperpush` issue sets, preserved canonical transfer/create history, and verified the final repo issue truth with a replayable retained GitHub verifier.**

## What Happened

## What Happened

S02 turned the S01 reconciliation ledger into one deterministic live repo correction flow. T01 added `scripts/lib/m057_repo_mutation_plan.py` plus the plan contract rail, expanded the immutable S01 ledger and snapshots into a checked 43-operation apply set, and published `.gsd/milestones/M057/slices/S02/repo-mutation-plan.json` / `.md`. The planner intentionally excluded already-closed `hyperpush#3/#4/#5` from the live apply set, leaving exactly 10 `mesh-lang` closeouts, 31 open-issue rewrites/normalizations, 1 transfer, and 1 retrospective create.

T02 added `scripts/lib/m057_repo_mutation_apply.py` as the plan-driven applicator, executed the identity-changing operations first, and recorded every live outcome in `.gsd/milestones/M057/slices/S02/repo-mutation-results.json`. The batch preserved the misfiled docs history by transferring `hyperpush#8` into `mesh-lang#19`, created and then closed retrospective `/pitch` issue `hyperpush#58`, closed the 10 shipped `mesh-lang` issues, rewrote the 21 `rewrite_scope` product issues, kept the 7 mock-backed follow-through rows open with truthful wording, and normalized public naming on `hyperpush#54/#55/#56` away from stale public `hyperpush-mono` wording. A second live `--apply` pass returned `already_satisfied` for all 43 operations, proving resume safety instead of assuming it.

T03 completed the slice by adding the checked results contract and the retained read-only verifier. `scripts/verify-m057-s02.sh` now replays repo totals plus all 43 touched issue views into `.tmp/m057-s02/verify/`, leaving named phases, `last-target.txt`, `verification-summary.json`, and per-command stdout/stderr logs. The final verified state is: `mesh-lang` = 17 total issues (7 open / 10 closed), `hyperpush` = 52 total issues (47 open / 5 closed), combined = 69. The rendered handoff at `.gsd/milestones/M057/slices/S02/repo-mutation-results.md` gives S03 the canonical `hyperpush#8 -> mesh-lang#19` and `/pitch -> hyperpush#58` mappings plus the bucket rollups it must trust when updating org project #1.

## Operational Readiness

- **Health signal:** `bash scripts/verify-m057-s02.sh` exits 0; `.tmp/m057-s02/verify/phase-report.txt` reports `Status: ok`; `verification-summary.json` records `artifact-contract`, `repo-totals`, `issue-state-replay`, and `handoff-render` as green; repo totals remain `mesh-lang=17`, `hyperpush=52`, `combined=69`.
- **Failure signal:** the results contract loses a canonical transfer/create URL, bucket coverage drifts, a supposedly rewritten issue closes or regresses to stale wording, repo totals change unexpectedly, or the retained verifier fails any phase and names the last attempted issue in `.tmp/m057-s02/verify/last-target.txt`.
- **Recovery procedure:** rerun `node --test scripts/tests/verify-m057-s02-plan.test.mjs`, `node --test scripts/tests/verify-m057-s02-results.test.mjs`, and `bash scripts/verify-m057-s02.sh`; inspect `.tmp/m057-s02/verify/phase-report.txt`, `verification-summary.json`, `last-target.txt`, and the retained per-command logs to isolate the exact drifted issue or artifact; repair the plan/apply/results generation or the live repo state at the source instead of hand-editing generated JSON/markdown.
- **Monitoring gaps:** this is a live on-demand GitHub verification seam, not a continuously scheduled monitor. S03 or any future maintainer must rerun the verifier when they need fresh truth.

S02 now leaves the two repos truthful on their own. The only remaining milestone-wide drift is at the org project layer, which is intentionally deferred to S03 so the board realignment consumes the canonical issue truth produced here rather than re-deriving it from stale board text.

## Verification

Passed all slice-level verification rails defined in the slice plan:

- `node --test scripts/tests/verify-m057-s02-plan.test.mjs`
- `node --test scripts/tests/verify-m057-s02-results.test.mjs`
- `bash scripts/verify-m057-s02.sh`

Also confirmed the slice observability surfaces are live and useful: the checked plan/results artifacts exist under `.gsd/milestones/M057/slices/S02/`, and the retained verifier left `.tmp/m057-s02/verify/phase-report.txt`, `verification-summary.json`, `last-target.txt`, and per-command logs with green phase state.

## Requirements Advanced

- R133 — Applied the naming normalization live on `hyperpush#54/#55/#56`, and carried canonical repo identity plus compatibility-path truth through the plan/results artifacts and S03 handoff.
- R134 — Made the repo-level planning surfaces intelligible from GitHub issue truth alone and published a compact handoff for the remaining board-only realignment step.

## Requirements Validated

- R128 — Validated by the green slice verification chain (`node --test scripts/tests/verify-m057-s02-plan.test.mjs`, `node --test scripts/tests/verify-m057-s02-results.test.mjs`, `bash scripts/verify-m057-s02.sh`) plus live repo totals showing `mesh-lang` at 17 issues with the 10 shipped rows closed and replayed against persisted final-state snapshots.
- R129 — Validated by the same green verification chain plus live repo totals showing `hyperpush` at 52 issues, 21 `rewrite_scope` rows rewritten in place, 7 mock-backed follow-through rows left open with truthful wording, and the naming-normalization rows verified live.
- R131 — Validated by retrospective creation and closeout of canonical issue `hyperpush#58` for the shipped `/pitch` route, with the canonical URL/number preserved in `repo-mutation-results.json` / `.md` and re-verified by `bash scripts/verify-m057-s02.sh`.
- R132 — Validated by preserving history through transfer and rewrite rather than recreation: `hyperpush#8` moved to `mesh-lang#19`, shipped work was closed with evidence, and drifted items were rewritten in place; the retained verifier replays those states live.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

`repo-mutation-results.json` intentionally reflects the second live `--apply` idempotence proof pass, so the final per-operation statuses are `already_satisfied` rather than the first successful pass's `applied` statuses. The canonical transfer/create mappings and final snapshots remain authoritative.

## Known Limitations

Org project #1 still reflects pre-reconciliation issue URLs/statuses until S03 consumes the S02 handoff and updates the board. Verification is truthful and replayable, but it is an on-demand GitHub read path rather than a continuously running monitor.

## Follow-ups

S03 should consume `.gsd/milestones/M057/slices/S02/repo-mutation-results.json` and `.gsd/milestones/M057/slices/S02/repo-mutation-results.md` as the canonical old→new issue mapping surface when realigning org project #1 item URLs and statuses. Re-run `bash scripts/verify-m057-s02.sh` immediately before any board mutation if the repo issue state may have drifted.

## Files Created/Modified

- `scripts/lib/m057_repo_mutation_plan.py` — Builds the deterministic S01-ledger-driven mutation manifest, replacement issue text, and checked plan artifacts for S02.
- `scripts/lib/m057_repo_mutation_apply.py` — Applies the checked mutation manifest to live GitHub, records per-operation outcomes, canonical mappings, label resolution, and final-state snapshots.
- `scripts/tests/verify-m057-s02-plan.test.mjs` — Locks the plan artifact shape, touched-set counts, exclusions, and negative cases.
- `scripts/tests/verify-m057-s02-results.test.mjs` — Locks the checked results artifact shape, canonical mapping persistence, bucket rollups, and fail-closed drift cases.
- `scripts/verify-m057-s02.sh` — Runs the retained live verifier, replays repo totals plus all 43 touched issues, and emits phase diagnostics under `.tmp/m057-s02/verify/`.
- `.gsd/milestones/M057/slices/S02/repo-mutation-plan.json` — Publishes the deterministic dry-run mutation manifest that S02 executed.
- `.gsd/milestones/M057/slices/S02/repo-mutation-results.json` — Publishes the live mutation outcomes, canonical old→new mappings, and final issue snapshots used by verification.
- `.gsd/milestones/M057/slices/S02/repo-mutation-results.md` — Publishes the compact S03 handoff with canonical issue mappings, repo totals, and bucket rollups.
- `.gsd/KNOWLEDGE.md` — Captures the transfer verification gotcha and the redirected `gh api` identity-change read behavior for future tracker-reconciliation work.
- `.gsd/PROJECT.md` — Refreshes current project state so M057/S02 is reflected in the repo-level planning summary.
