---
id: T01
parent: S11
milestone: M034
provides: []
requires: []
affects: []
key_files: [".tmp/m034-s11/t01/release-workflow-contract.log", ".tmp/m034-s11/t01/verify-m034-s03-last-exitcode.log", ".tmp/m034-s11/t01/verify-m034-s03-shell-syntax.log", ".tmp/m034-s11/t01/slice-local-contract-tests.log", ".tmp/m034-s11/t01/verification-summary.json", ".gsd/milestones/M034/slices/S11/tasks/T01-SUMMARY.md"]
key_decisions: ["Left .github/workflows/release.yml, scripts/verify-m034-s02-workflows.sh, and scripts/verify-m034-s03.ps1 unchanged because replayed local proofs showed the Windows smoke patch was already fully present."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the repo-owned seam with bash scripts/verify-m034-s02-workflows.sh, pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1, and bash -n scripts/verify-m034-s03.sh; all passed. Also ran node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs as a local slice-level contract check; it passed. Saved fresh outputs and durations to .tmp/m034-s11/t01/ plus .tmp/m034-s11/t01/verification-summary.json."
completed_at: 2026-03-27T21:50:27.749Z
blocker_discovered: false
---

# T01: Proved the Windows release-smoke workflow patch is already aligned locally and captured fresh T01 verifier logs for the rollout handoff.

> Proved the Windows release-smoke workflow patch is already aligned locally and captured fresh T01 verifier logs for the rollout handoff.

## What Happened
---
id: T01
parent: S11
milestone: M034
key_files:
  - .tmp/m034-s11/t01/release-workflow-contract.log
  - .tmp/m034-s11/t01/verify-m034-s03-last-exitcode.log
  - .tmp/m034-s11/t01/verify-m034-s03-shell-syntax.log
  - .tmp/m034-s11/t01/slice-local-contract-tests.log
  - .tmp/m034-s11/t01/verification-summary.json
  - .gsd/milestones/M034/slices/S11/tasks/T01-SUMMARY.md
key_decisions:
  - Left .github/workflows/release.yml, scripts/verify-m034-s02-workflows.sh, and scripts/verify-m034-s03.ps1 unchanged because replayed local proofs showed the Windows smoke patch was already fully present.
duration: ""
verification_result: passed
completed_at: 2026-03-27T21:50:27.750Z
blocker_discovered: false
---

# T01: Proved the Windows release-smoke workflow patch is already aligned locally and captured fresh T01 verifier logs for the rollout handoff.

**Proved the Windows release-smoke workflow patch is already aligned locally and captured fresh T01 verifier logs for the rollout handoff.**

## What Happened

I compared the current local workflow state against the existing hosted evidence instead of forcing a speculative patch. The current tree already contains the intended Windows release-smoke additions in .github/workflows/release.yml, and scripts/verify-m034-s02-workflows.sh already asserts that exact contract. Replaying the repo-local workflow contract, the PowerShell LASTEXITCODE regression guard, the Unix verifier syntax check, and the slice-local contract tests all passed cleanly, so there was no evidence that scripts/verify-m034-s03.ps1 or the workflow still needed code changes. I left the source files unchanged and captured fresh logs under .tmp/m034-s11/t01/ so T02 can treat the remaining blocker as hosted rollout freshness rather than repo-local drift.

## Verification

Verified the repo-owned seam with bash scripts/verify-m034-s02-workflows.sh, pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1, and bash -n scripts/verify-m034-s03.sh; all passed. Also ran node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs as a local slice-level contract check; it passed. Saved fresh outputs and durations to .tmp/m034-s11/t01/ plus .tmp/m034-s11/t01/verification-summary.json.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash scripts/verify-m034-s02-workflows.sh` | 0 | ✅ pass | 1457ms |
| 2 | `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1` | 0 | ✅ pass | 3225ms |
| 3 | `bash -n scripts/verify-m034-s03.sh` | 0 | ✅ pass | 28ms |
| 4 | `node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs` | 0 | ✅ pass | 1997ms |


## Deviations

No source edits were required. The plan assumed the Windows workflow patch might still need finalization, but the current local tree already matched the intended workflow, verifier, and PowerShell helper contract, so the task became proof capture and handoff evidence.

## Known Issues

Hosted release.yml evidence is still red in .tmp/m034-s05/verify/remote-runs.json until T02 rolls the approved refs and refreshes remote evidence. This task only proved that the remaining blocker is not repo-local drift.

## Files Created/Modified

- `.tmp/m034-s11/t01/release-workflow-contract.log`
- `.tmp/m034-s11/t01/verify-m034-s03-last-exitcode.log`
- `.tmp/m034-s11/t01/verify-m034-s03-shell-syntax.log`
- `.tmp/m034-s11/t01/slice-local-contract-tests.log`
- `.tmp/m034-s11/t01/verification-summary.json`
- `.gsd/milestones/M034/slices/S11/tasks/T01-SUMMARY.md`


## Deviations
No source edits were required. The plan assumed the Windows workflow patch might still need finalization, but the current local tree already matched the intended workflow, verifier, and PowerShell helper contract, so the task became proof capture and handoff evidence.

## Known Issues
Hosted release.yml evidence is still red in .tmp/m034-s05/verify/remote-runs.json until T02 rolls the approved refs and refreshes remote evidence. This task only proved that the remaining blocker is not repo-local drift.
