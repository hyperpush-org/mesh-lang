---
id: T03
parent: S03
milestone: M057
key_files:
  - scripts/lib/m057_project_mutation_plan.py
  - scripts/tests/verify-m057-s03-plan.test.mjs
  - .gsd/milestones/M057/slices/S03/project-mutation-plan.json
  - .gsd/milestones/M057/slices/S03/project-mutation-plan.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Treat S01 `leave_untracked` rows as explicit repo-only no-ops for S03 planning, but fail closed if any of them are marked project-backed or appear on the live board.
duration: 
verification_result: passed
completed_at: 2026-04-10T18:52:45.151Z
blocker_discovered: false
---

# T03: Unblocked the green S03 project-mutation planner, regenerated the ready manifest, and locked it with a plan contract test.

**Unblocked the green S03 project-mutation planner, regenerated the ready manifest, and locked it with a plan contract test.**

## What Happened

Updated the S03 planner so the ready-path manifest can be generated from the now-green retained S02 truth without tripping on S01’s intentional repo-only `leave_untracked` rows, while still failing closed if those rows drift back onto the live board. Fixed the planner’s own `--check` contract to validate the emitted `desired_project_items` rollup field instead of a stale `desired_total` key. Regenerated the checked S03 plan artifacts, which now record a green retained S02 preflight, the ten stale `mesh-lang` cleanup deletions, the canonical replacement adds for `mesh-lang#19` and `hyperpush#58`, the naming-preserved no-op rows for `hyperpush#54/#55/#56`, and deterministic inheritance sources including the deep `hyperpush#57 -> hyperpush#34 -> hyperpush#15` chain. Added `scripts/tests/verify-m057-s03-plan.test.mjs` to lock that artifact contract with both happy-path assertions and fail-closed mutation checks.

## Verification

`python3 scripts/lib/m057_project_mutation_plan.py --output-dir .gsd/milestones/M057/slices/S03 --check` passed and regenerated the checked S03 plan artifacts from live repo/project truth while recording a green retained S02 preflight. `node --test scripts/tests/verify-m057-s03-plan.test.mjs` passed and proved the plan contract for canonical replacement rows, stale cleanup deletes, naming-preserved no-ops, leave-untracked exclusion, and representative inherited metadata coverage.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `python3 scripts/lib/m057_project_mutation_plan.py --output-dir .gsd/milestones/M057/slices/S03 --check` | 0 | ✅ pass | 52020ms |
| 2 | `node --test scripts/tests/verify-m057-s03-plan.test.mjs` | 0 | ✅ pass | 505ms |

## Deviations

The live S01 ledger still contains four explicit `leave_untracked` repo-only rows (`hyperpush#2/#3/#4/#5`), so the planner had to skip them instead of treating them as unexpected action kinds; and the planner’s own `--check` validator still referenced a stale `desired_total` key instead of the emitted `desired_project_items` rollup field.

## Known Issues

None.

## Files Created/Modified

- `scripts/lib/m057_project_mutation_plan.py`
- `scripts/tests/verify-m057-s03-plan.test.mjs`
- `.gsd/milestones/M057/slices/S03/project-mutation-plan.json`
- `.gsd/milestones/M057/slices/S03/project-mutation-plan.md`
- `.gsd/KNOWLEDGE.md`
