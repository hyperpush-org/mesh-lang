---
id: T01
parent: S02
milestone: M057
key_files:
  - scripts/lib/m057_repo_mutation_plan.py
  - scripts/tests/verify-m057-s02-plan.test.mjs
  - .gsd/milestones/M057/slices/S02/repo-mutation-plan.json
  - .gsd/milestones/M057/slices/S02/repo-mutation-plan.md
  - .gsd/milestones/M057/slices/S02/tasks/T01-SUMMARY.md
key_decisions:
  - Treat the S02 live apply set as 43 operations: 10 live closeouts, 31 rewrites/normalizations, 1 transfer, and 1 retrospective create.
  - Exclude already-closed `hyperpush#3/#4/#5` from the apply set even though the ledger marks them `close_as_shipped`.
  - Catch imported ledger/inventory validator failures as clean CLI errors so malformed-input verification stays readable.
duration: 
verification_result: mixed
completed_at: 2026-04-10T07:34:54.322Z
blocker_discovered: false
---

# T01: Built the M057/S02 dry-run repo mutation planner and contract test that expand the S01 ledger into a 43-operation manifest with explicit transfer and `/pitch` create handling.

**Built the M057/S02 dry-run repo mutation planner and contract test that expand the S01 ledger into a 43-operation manifest with explicit transfer and `/pitch` create handling.**

## What Happened

Added `scripts/lib/m057_repo_mutation_plan.py` to validate the immutable S01 ledger/snapshots, parse both repos’ feature-request templates with a documented fallback path, and emit deterministic plan artifacts under `.gsd/milestones/M057/slices/S02/`. The planner now distinguishes the true live touched set from the broader ledger by excluding already-closed `hyperpush#3/#4/#5`, expanding 10 live `mesh-lang` closeouts, 21 `rewrite_scope` rows, 10 open normalization rows, the `hyperpush#8` transfer, and one retrospective `/pitch` issue creation. Added `scripts/tests/verify-m057-s02-plan.test.mjs` to lock the apply-set counts, identity-changing rows, exclusions, fail-closed malformed-input behavior, and unreadable-template fallback. Generated `repo-mutation-plan.json` and `repo-mutation-plan.md` from the new planner.

## Verification

Task-level verification passed with `python3 scripts/lib/m057_repo_mutation_plan.py --output-dir .gsd/milestones/M057/slices/S02 --check` and `node --test scripts/tests/verify-m057-s02-plan.test.mjs`. I also ran the full slice verification list once as an intermediate-task check: the new plan verifier passed, while `node --test scripts/tests/verify-m057-s02-results.test.mjs` and `bash scripts/verify-m057-s02.sh` failed cleanly because those T03 artifacts do not exist yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `python3 scripts/lib/m057_repo_mutation_plan.py --output-dir .gsd/milestones/M057/slices/S02 --check` | 0 | ✅ pass | 780ms |
| 2 | `node --test scripts/tests/verify-m057-s02-plan.test.mjs` | 0 | ✅ pass | 16459ms |
| 3 | `node --test scripts/tests/verify-m057-s02-results.test.mjs` | 1 | ❌ fail | 1734ms |
| 4 | `bash scripts/verify-m057-s02.sh` | 127 | ❌ fail | 21ms |

## Deviations

None.

## Known Issues

`scripts/tests/verify-m057-s02-results.test.mjs` and `scripts/verify-m057-s02.sh` are not implemented yet, so the slice-level results verification remains red until T03 lands.

## Files Created/Modified

- `scripts/lib/m057_repo_mutation_plan.py`
- `scripts/tests/verify-m057-s02-plan.test.mjs`
- `.gsd/milestones/M057/slices/S02/repo-mutation-plan.json`
- `.gsd/milestones/M057/slices/S02/repo-mutation-plan.md`
- `.gsd/milestones/M057/slices/S02/tasks/T01-SUMMARY.md`
