---
estimated_steps: 6
estimated_files: 4
skills_used: []
---

# T04: Apply project deletes, adds, and field edits from the refreshed checked manifest

Why: After the planner is green, org project #1 still has to be brought into line with the reconciled repo truth through a deterministic, resumable apply step.

Do:
1. Build or finish the plan-driven applicator with dry-run default and explicit --apply.
2. Use only captured project and field ids from the S01 schema snapshot, preserve explicit live non-null values unless the checked plan changes them, and execute deletes/adds/field edits in deterministic order.
3. Persist per-command outcomes, final item ids, canonical issue URLs, and representative done/active/todo row state in results artifacts, then prove rerun safety through already_satisfied outcomes.

Done when: the live board changes are applied from the checked manifest, rerunning the applicator does not duplicate writes, and the results artifacts capture the final canonical row state for the touched set.

## Inputs

- `scripts/lib/m057_project_mutation_apply.py`
- `.gsd/milestones/M057/slices/S03/project-mutation-plan.json`
- `.gsd/milestones/M057/slices/S01/project-fields.snapshot.json`
- `.gsd/milestones/M057/slices/S02/repo-mutation-results.json`

## Expected Output

- `.gsd/milestones/M057/slices/S03/project-mutation-results.json`
- `.gsd/milestones/M057/slices/S03/project-mutation-results.md`
- `.tmp/m057-s03/apply/last-operation.txt`

## Verification

python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --check && python3 scripts/lib/m057_project_mutation_apply.py --output-dir .gsd/milestones/M057/slices/S03 --apply
