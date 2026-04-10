---
estimated_steps: 6
estimated_files: 4
skills_used: []
---

# T03: Re-run the S03 planner on green upstream truth and add the plan contract test

Why: Once the retained S02 preflight is truthful again, S03 still needs its missing plan contract test and a verified ready-path planner output before any board mutation is allowed.

Do:
1. Finish the S03 planner ready path so it records the retained S02 preflight verdict, derives mutations from live repo/project truth plus canonical mappings, and persists deterministic parent-chain inheritance sources.
2. Add scripts/tests/verify-m057-s03-plan.test.mjs to lock the checked plan artifact shape, canonical replacement handling, stale-row deletions, and representative inherited metadata coverage.
3. Regenerate the S03 plan artifacts from the now-green S02 preflight and confirm the checked manifest is no longer blocked.

Done when: the checked S03 plan artifacts exist, the retained S02 verifier outcome is recorded as green, the stale cleanup and canonical replacement rows are explicitly accounted for, and the plan contract test passes.

## Inputs

- `scripts/lib/m057_project_mutation_plan.py`
- `.gsd/milestones/M057/slices/S01/reconciliation-ledger.json`
- `.gsd/milestones/M057/slices/S01/project-fields.snapshot.json`
- `.gsd/milestones/M057/slices/S01/project-items.snapshot.json`
- `.gsd/milestones/M057/slices/S02/repo-mutation-results.json`
- `scripts/verify-m057-s02.sh`

## Expected Output

- `scripts/tests/verify-m057-s03-plan.test.mjs`
- `.gsd/milestones/M057/slices/S03/project-mutation-plan.json`
- `.gsd/milestones/M057/slices/S03/project-mutation-plan.md`

## Verification

python3 scripts/lib/m057_project_mutation_plan.py --output-dir .gsd/milestones/M057/slices/S03 --check && node --test scripts/tests/verify-m057-s03-plan.test.mjs
