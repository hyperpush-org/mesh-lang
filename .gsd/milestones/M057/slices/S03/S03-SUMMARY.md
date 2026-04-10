---
id: S03
parent: M057
milestone: M057
provides:
  - A truthful org project #1 board state with 55 verified live rows across `mesh-lang` and `hyperpush`.
  - Canonical board presence for `mesh-lang#19` and `hyperpush#58`, plus removal of the stale Mesh cleanup rows.
  - Deterministic inheritance-backed tracked metadata on previously sparse representative rows.
  - A plan artifact, results artifact, and retained verifier/handoff that future maintainers can replay without reopening earlier `.gsd` archaeology.
requires:
  - slice: S01
    provides: Canonical issue/project snapshots, naming truth, field schema, and the joined reconciliation ledger.
  - slice: S02
    provides: Canonical repo issue mappings, refreshed live repo truth, and the retained repo precheck verifier.
affects:
  []
key_files:
  - scripts/lib/m057_project_mutation_plan.py
  - scripts/lib/m057_project_mutation_apply.py
  - scripts/tests/verify-m057-s02-results.test.mjs
  - scripts/tests/verify-m057-s03-plan.test.mjs
  - scripts/tests/verify-m057-s03-results.test.mjs
  - scripts/verify-m057-s02.sh
  - scripts/verify-m057-s03.sh
  - .gsd/milestones/M057/slices/S02/repo-mutation-results.json
  - .gsd/milestones/M057/slices/S02/repo-mutation-results.md
  - .gsd/milestones/M057/slices/S03/project-mutation-plan.json
  - .gsd/milestones/M057/slices/S03/project-mutation-plan.md
  - .gsd/milestones/M057/slices/S03/project-mutation-results.json
  - .gsd/milestones/M057/slices/S03/project-mutation-results.md
  - .tmp/m057-s02/verify/phase-report.txt
  - .tmp/m057-s03/verify/phase-report.txt
  - .tmp/m057-s03/verify/verification-summary.json
key_decisions:
  - D460: run the retained S02 verifier as the S03 preflight and derive board mutations from current live repo truth plus persisted canonical mappings instead of stale board text.
  - D461: treat S01 `leave_untracked` rows as explicit repo-only no-ops, but fail closed if any of them are marked project-backed or appear on the live board.
  - D462: retry transient GitHub Projects V2 capture drift and added-row visibility lag inside the S03 applicator instead of weakening the shared inventory contract.
  - D463: normalize blank-string GitHub CLI `stateReason` values to null while preserving real reopen metadata so the retained S02 verifier only fails on real drift.
  - D464: delegate the retained S02 verifier first in the S03 verifier so repo-truth drift is diagnosed before project-membership or field-coherence drift.
patterns_established:
  - Drive risky GitHub Projects changes from a checked manifest first, then execute the live apply step only from that manifest.
  - Replay upstream repo truth before board truth so dependency drift is classified at the correct layer.
  - Preserve explicit live board values and backfill only missing tracked metadata through deterministic parent-chain inheritance.
  - Treat GitHub Projects V2 eventual consistency as a slice-local retry concern in the applicator, not as a reason to weaken canonical inventory checks.
  - Use `--check`/`--apply` steady-state results plus a retained read-only verifier to prove rerun safety instead of assuming the board is stable.
observability_surfaces:
  - .gsd/milestones/M057/slices/S03/project-mutation-plan.json and `.md` publish the checked pre-apply touched set, canonical replacement handling, stale deletions, and inheritance coverage.
  -  .gsd/milestones/M057/slices/S03/project-mutation-results.json records per-operation outcomes, final live capture rollups, canonical board presence, representative rows, and rerun-safe `already_satisfied` status.
  -  .gsd/milestones/M057/slices/S03/project-mutation-results.md is the maintainer-readable done/active/next handoff for the live board.
  -  .tmp/m057-s03/verify/phase-report.txt, `verification-summary.json`, and `commands/` expose retained verifier phase health, drift classification, and last-target evidence.
  -  .tmp/m057-s02/verify/phase-report.txt and `verification-summary.json` remain the delegated upstream repo-truth precheck surfaces.
drill_down_paths:
  - .gsd/milestones/M057/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M057/slices/S03/tasks/T02-SUMMARY.md
  - .gsd/milestones/M057/slices/S03/tasks/T03-SUMMARY.md
  - .gsd/milestones/M057/slices/S03/tasks/T04-SUMMARY.md
  - .gsd/milestones/M057/slices/S03/tasks/T05-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-10T20:12:49.503Z
blocker_discovered: false
---

# S03: S03

**Reconciled org project #1 to live repo truth, removed stale Mesh cleanup rows, added the canonical `mesh-lang#19` / `hyperpush#58` board rows, backfilled missing tracked metadata through parent-chain inheritance, and published a replayable board-truth verifier plus maintainer handoff.**

## What Happened

# S03: Realign org project #1 to live repo truth

**Realigned org project #1 from reconciled live repo truth so the board now truthfully shows what is done, active, and next across `mesh-lang` and `hyperpush`, while preserving canonical issue mappings and leaving replayable verification surfaces for future maintainers.**

## What Happened

S03 closed the last public tracker gap in M057 by taking the already-reconciled repo issue truth from S02 and applying it to the org portfolio board instead of trusting stale board text. T01 introduced the project-mutation planner and correctly failed closed when the retained S02 verifier exposed upstream drift: `mesh-lang#19` had become closed live while the retained S02 artifact still claimed it was open. T02 repaired that upstream truth source by refreshing the retained S02 results/verifier contract to current GitHub reality, preserving canonical mappings (`hyperpush#8 -> mesh-lang#19`, `/pitch -> hyperpush#58`) while normalizing CLI `stateReason` noise so only real repo drift fails the replay.

With the upstream preflight green again, T03 completed the checked S03 planning layer. The planner now records the retained S02 preflight result, skips the explicit S01 `leave_untracked` repo-only rows (`hyperpush#2/#3/#4/#5`), renders the checked delete/add/update manifest, and locks it with a plan contract test. The checked plan captures a 63-row pre-apply board, 10 stale `mesh-lang` cleanup deletes, 2 canonical add rows (`mesh-lang#19`, `hyperpush#58`), 23 metadata updates, and 30 verified no-op rows including the naming-normalized `hyperpush#54/#55/#56` active deployment items.

T04 executed the live board mutations through a plan-driven applicator. The real GitHub Projects V2 write path exposed eventual-consistency seams after `gh project item-add`, so the applicator was hardened to retry page capture drift and added-row visibility lag without weakening the shared inventory contract. The final applied board truth is now 55 live rows: 2 `Done`, 3 `In Progress`, and 50 `Todo`. The stale cleanup rows (`mesh-lang#3/#4/#5/#6/#8/#9/#10/#11/#13/#14`) are absent, the canonical replacement rows for `mesh-lang#19` and `hyperpush#58` are present, and previously sparse product rows now inherit missing `Domain`, `Track`, `Delivery Mode`, and deeper tracked metadata deterministically from parent chains while preserving explicit live values.

T05 finished the slice by adding the retained S03 verifier and maintainer-readable handoff. `scripts/verify-m057-s03.sh` now delegates the retained S02 verifier first, re-fetches org project #1, classifies drift as repo-truth vs project-membership vs field-coherence, and leaves `.tmp/m057-s03/verify/phase-report.txt`, `verification-summary.json`, and command logs as the primary diagnostic bundle. The rendered handoff in `.gsd/milestones/M057/slices/S03/project-mutation-results.md` now makes the board readable without reopening `.gsd` archaeology: `mesh-lang#19` is the representative done row, `hyperpush#54` is the representative active row, and `hyperpush#29` is the representative next row.

## Operational Readiness

- **Health signal:** `bash scripts/verify-m057-s03.sh` exits 0; `.tmp/m057-s03/verify/phase-report.txt` reports `Status: ok`; the live capture in `.tmp/m057-s03/verify/verification-summary.json` shows `total_items=55`, repo counts `{hyperpush-org/hyperpush: 48, hyperpush-org/mesh-lang: 7}`, and status counts `{Done: 2, In Progress: 3, Todo: 50}`.
- **Failure signal:** the delegated repo precheck goes red, a canonical replacement row (`mesh-lang#19` or `hyperpush#58`) disappears, a stale cleanup row reappears, naming-normalized deployment titles drift back to stale wording, or inherited tracked metadata on representative rows no longer matches the checked results artifact.
- **Recovery procedure:** rerun `bash scripts/verify-m057-s02.sh`, `python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --check`, `node --test scripts/tests/verify-m057-s03-results.test.mjs`, and `bash scripts/verify-m057-s03.sh`; start with `.tmp/m057-s03/verify/phase-report.txt` to identify whether drift is upstream repo truth, board membership, or field coherence; then repair the planner/applicator/verifier or live project state at the source instead of hand-editing generated artifacts.
- **Monitoring gaps:** this remains an on-demand GitHub replay, not a continuously scheduled monitor. Future maintainers must rerun the retained verifier when they need fresh board truth.

S03 leaves the public planning layer truthful: the repos and the org portfolio board now agree on what has shipped, what is actively in progress, and what is next.

## Verification

Passed the slice closeout verification chain on live GitHub state:

- `node --test scripts/tests/verify-m057-s02-results.test.mjs`
- `bash scripts/verify-m057-s02.sh`
- `node --test scripts/tests/verify-m057-s03-plan.test.mjs`
- `python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --check`
- `python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --apply`
- `node --test scripts/tests/verify-m057-s03-results.test.mjs`
- `bash scripts/verify-m057-s03.sh`

Also confirmed the required observability surfaces are live and useful: `.gsd/milestones/M057/slices/S03/project-mutation-plan.json`, `.gsd/milestones/M057/slices/S03/project-mutation-results.json`, `.gsd/milestones/M057/slices/S03/project-mutation-results.md`, and `.tmp/m057-s03/verify/phase-report.txt` / `verification-summary.json` all reflect the final verified board truth.

Note: `python3 scripts/lib/m057_project_mutation_plan.py --output-dir .gsd/milestones/M057/slices/S03 --check` is a pre-apply baseline validator for the checked 63-row board snapshot, not the authoritative steady-state verifier after the live board has been reconciled to 55 rows. Final live verification therefore uses the locked plan artifact test plus the apply/results verifiers.

## Requirements Advanced

- R133 — Preserved normalized public `hyperpush` naming on the representative active deployment rows (`hyperpush#54/#55/#56`) while keeping board ownership aligned with repo truth.

## Requirements Validated

- R130 — `node --test scripts/tests/verify-m057-s03-results.test.mjs` and `bash scripts/verify-m057-s03.sh` now replay a live 55-row board with canonical done/active/next state, stale cleanup removals, and canonical presence for `mesh-lang#19` / `hyperpush#58`.
- R134 — `.gsd/milestones/M057/slices/S03/project-mutation-results.md` plus the retained `.tmp/m057-s03/verify/` bundle now explain final board truth and replay drift surfaces without reopening `.gsd` archaeology.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

`python3 scripts/lib/m057_project_mutation_plan.py --output-dir .gsd/milestones/M057/slices/S03 --check` is intentionally a pre-apply 63-row board-baseline validator. After T04 reconciled the live board to 55 rows, the honest steady-state closeout path became the locked plan artifact test plus the apply/results/verifier chain rather than rerunning that pre-apply live-capture check against the post-apply board.

## Known Limitations

Verification remains an on-demand GitHub replay rather than a continuously scheduled monitor. GitHub Projects V2 eventual consistency still exists after writes, but the applicator now retries capture and added-row visibility so the final live board state is truthful and rerun-safe. The checked planner artifact remains the historical pre-apply manifest; use the apply/results/verifier surfaces for current board truth.

## Follow-ups

Milestone closeout should now run M057 validation and completion against the reconciled repo + board truth. Future tracker maintenance should start from `bash scripts/verify-m057-s02.sh` and `bash scripts/verify-m057-s03.sh` rather than from hand inspection.

## Files Created/Modified

- `scripts/lib/m057_project_mutation_plan.py` — Built the preflight-gated planner, leave-untracked handling, and checked pre-apply project mutation manifest generation.
- `scripts/lib/m057_project_mutation_apply.py` — Applied the checked manifest live, retried transient GitHub Projects lag, and persisted rerun-safe results artifacts.
- `scripts/tests/verify-m057-s02-results.test.mjs` — Refreshed the retained S02 results contract to current live repo truth so S03 preflight consumes a truthful upstream source.
- `scripts/tests/verify-m057-s03-plan.test.mjs` — Locked the checked S03 plan artifact shape, canonical add rows, stale deletes, and inheritance coverage.
- `scripts/tests/verify-m057-s03-results.test.mjs` — Locked the final results artifact, canonical mapping summaries, naming-normalized rows, and inherited metadata expectations.
- `scripts/verify-m057-s02.sh` — Refreshed the retained upstream repo-truth verifier used as S03's delegated preflight.
- `scripts/verify-m057-s03.sh` — Added the retained board-truth verifier with phase diagnostics and drift classification.
- `.gsd/milestones/M057/slices/S03/project-mutation-plan.json` — Publishes the checked 63-row pre-apply board mutation manifest.
- `.gsd/milestones/M057/slices/S03/project-mutation-results.json` — Publishes the live board mutation outcomes, final rollups, canonical board presence, and representative rows.
- `.gsd/milestones/M057/slices/S03/project-mutation-results.md` — Publishes the maintainer-readable done/active/next handoff for the verified live board.
- `.tmp/m057-s03/verify/phase-report.txt` — Provides the retained phase-by-phase verifier diagnosis surface for future drift.
- `.gsd/KNOWLEDGE.md` — Captured the GitHub CLI `stateReason` normalization gotcha and the pre-apply planner-vs-steady-state verification distinction.
- `.gsd/PROJECT.md` — Refreshed project state to reflect completed repo and board reconciliation under M057.
