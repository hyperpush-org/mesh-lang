---
estimated_steps: 6
estimated_files: 4
skills_used: []
---

# T05: Replay live board truth and publish the retained S03 verifier

Why: S03 is only complete once a read-only verifier can replay the final live board state, prove it matches reconciled repo truth, and expose drift clearly for future maintainers.

Do:
1. Add the S03 results contract test to lock touched-set coverage, canonical mapping handling, representative row states, and inherited metadata expectations.
2. Add or finish the retained live verifier so it re-fetches org project #1, checks stale cleanup row removal, canonical mesh-lang#19 / hyperpush#58 handling, public naming normalization on hyperpush#54/#55/#56, and representative inherited rows.
3. Persist phase diagnostics, last-target evidence, and a maintainer-readable results markdown handoff that explains the final done/active/next board truth without reopening .gsd archaeology.

Done when: the results contract test and retained verifier pass, .tmp/m057-s03/verify contains clear diagnostics, and the published results markdown explains the final board truth from the verified live state.

## Inputs

- `scripts/lib/m057_project_mutation_apply.py`
- `.gsd/milestones/M057/slices/S03/project-mutation-plan.json`
- `.gsd/milestones/M057/slices/S03/project-mutation-results.json`
- `.gsd/milestones/M057/slices/S02/repo-mutation-results.json`

## Expected Output

- `scripts/tests/verify-m057-s03-results.test.mjs`
- `scripts/verify-m057-s03.sh`
- `.tmp/m057-s03/verify/phase-report.txt`
- `.tmp/m057-s03/verify/verification-summary.json`
- `.gsd/milestones/M057/slices/S03/project-mutation-results.md`

## Verification

node --test scripts/tests/verify-m057-s03-results.test.mjs && bash scripts/verify-m057-s03.sh
