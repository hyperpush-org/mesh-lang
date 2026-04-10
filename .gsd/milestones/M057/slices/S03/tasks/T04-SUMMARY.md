---
id: T04
parent: S03
milestone: M057
key_files:
  - scripts/lib/m057_project_mutation_apply.py
  - scripts/tests/verify-m057-s03-apply.test.mjs
  - .gsd/milestones/M057/slices/S03/project-mutation-results.json
  - .gsd/milestones/M057/slices/S03/project-mutation-results.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Retry transient GitHub Projects V2 capture drift and added-row visibility lag inside the S03 applicator instead of weakening the shared S01 inventory contract.
  - Make the applicator's last-operation checkpoint path derive from the requested source root so isolated contract runs stay self-contained.
duration: 
verification_result: passed
completed_at: 2026-04-10T19:48:39.673Z
blocker_discovered: false
---

# T04: Applied the checked S03 org-project manifest live, hardened the applicator against transient GitHub Projects V2 lag, and proved rerun-safe already_satisfied steady state.

**Applied the checked S03 org-project manifest live, hardened the applicator against transient GitHub Projects V2 lag, and proved rerun-safe already_satisfied steady state.**

## What Happened

Added a new S03 apply contract test around a fake GitHub Projects backend, then updated `scripts/lib/m057_project_mutation_apply.py` so the last-operation checkpoint follows the requested source root and live post-mutation refreshes retry transient GraphQL pagination drift plus delayed added-row visibility. The first real apply exposed exactly those two GitHub consistency seams after `gh project item-add`, leaving the board in a truthful partial state; after patching the applicator I resumed from that live board state, finished the remaining add/update work, rendered the final results artifacts, and confirmed the steady-state rerun collapses every touched operation to `already_satisfied`.

## Verification

Verified the applicator with `python3 -m py_compile scripts/lib/m057_project_mutation_apply.py`, ran the local S03 contract suite with `node --test scripts/tests/verify-m057-s03-plan.test.mjs scripts/tests/verify-m057-s03-apply.test.mjs`, then ran the live task-plan commands against org project #1: `python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --check`, `python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --apply`, and a second `--apply` rerun. Final artifacts show 55 total board rows, canonical replacements present for `hyperpush#58` and `mesh-lang#19`, representative done/in-progress/todo rows rendered, and rerun rollup `applied=0` / `already_satisfied=35`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --check` | 0 | ✅ pass | 2804ms |
| 2 | `python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --apply` | 0 | ✅ pass | 143447ms |
| 3 | `python3 -m py_compile scripts/lib/m057_project_mutation_apply.py` | 0 | ✅ pass | 135ms |
| 4 | `node --test scripts/tests/verify-m057-s03-plan.test.mjs scripts/tests/verify-m057-s03-apply.test.mjs` | 0 | ✅ pass | 61224ms |

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `scripts/lib/m057_project_mutation_apply.py`
- `scripts/tests/verify-m057-s03-apply.test.mjs`
- `.gsd/milestones/M057/slices/S03/project-mutation-results.json`
- `.gsd/milestones/M057/slices/S03/project-mutation-results.md`
- `.gsd/KNOWLEDGE.md`
