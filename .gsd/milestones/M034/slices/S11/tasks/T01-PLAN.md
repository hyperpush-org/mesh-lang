---
estimated_steps: 4
estimated_files: 5
skills_used:
  - github-workflows
  - powershell-windows
---

# T01: Finalize the Windows release-smoke workflow patch and keep the workflow contract honest

**Slice:** S11 — First-green archive and final assembly closeout
**Milestone:** M034

## Description

Finish the only remaining repo-local blocker surface before any rollout mutation: keep `.github/workflows/release.yml` and `scripts/verify-m034-s02-workflows.sh` aligned on the Windows smoke-toolchain steps, and only touch `scripts/verify-m034-s03.ps1` if replayed diagnostics prove the workflow patch alone is insufficient.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `.github/workflows/release.yml` Windows smoke verifier steps | Keep the task red locally and do not proceed to hosted mutation until the workflow and verifier agree on the expected steps. | Treat the local replay as inconclusive, preserve the partial log under `.tmp/m034-s11/t01/`, and stop before rollout. | Treat missing or renamed workflow steps as contract drift rather than silently weakening the release lane. |
| `scripts/verify-m034-s02-workflows.sh` | Fail closed and update the workflow and verifier together instead of hand-waving a mismatch. | Preserve the verifier log and stop the slice at the repo-owned seam. | Treat YAML parsing or step-discovery failures as proof-surface regressions that must be repaired before T02. |
| `scripts/tests/verify-m034-s03-last-exitcode.ps1` / `scripts/verify-m034-s03.ps1` | Stop red and inspect whether the Windows smoke path needs a deeper PowerShell fix than the current workflow patch. | Capture the last successful phase log path under `.tmp/m034-s11/t01/` and keep the task incomplete. | Treat missing phase logs or malformed exit-code handling as verifier drift, not as a hosted-only blocker. |

## Load Profile

- **Shared resources**: Windows staged-smoke workflow semantics, release-asset verifier contract, and local `.tmp/m034-s11/t01/` logs.
- **Per-operation cost**: one workflow-contract replay, one PowerShell helper regression replay, and one shell syntax pass.
- **10x breakpoint**: repeated hosted reruns on a stale local workflow patch would consume release-lane evidence without changing the real blocker.

## Negative Tests

- **Malformed inputs**: renamed workflow steps, missing Windows LLVM prefix/export, and absent staged-smoke helper paths.
- **Error paths**: workflow contract failure, PowerShell helper regression failure, and shell syntax failure in `scripts/verify-m034-s03.sh`.
- **Boundary conditions**: workflow patch present with no `scripts/verify-m034-s03.ps1` changes, workflow patch plus a required PowerShell adjustment, and the unchanged green path where the blocker is purely rollout.

## Steps

1. Reproduce the current local release-workflow diff and compare it to the existing S10 blocker evidence so the task stays focused on the real Windows smoke seam.
2. Keep `.github/workflows/release.yml` and `scripts/verify-m034-s02-workflows.sh` aligned on the Windows smoke-toolchain steps, touching `scripts/verify-m034-s03.ps1` only if replayed diagnostics prove the workflow patch alone is insufficient.
3. Re-run the repo-local workflow-contract and PowerShell helper proofs, then preserve fresh logs under `.tmp/m034-s11/t01/`.
4. Stop red if local proof is still failing; do not hand the task to T02 until the remaining blocker is clearly hosted rollout rather than repo-local drift.

## Must-Haves

- [ ] The Windows smoke-toolchain steps in `.github/workflows/release.yml` and `scripts/verify-m034-s02-workflows.sh` match exactly.
- [ ] `scripts/verify-m034-s03.ps1` changes only if the replayed local diagnostics require them.
- [ ] `bash scripts/verify-m034-s02-workflows.sh` passes on the repaired local tree.
- [ ] Fresh local logs exist under `.tmp/m034-s11/t01/` so T02 can trust that hosted evidence is the next seam.

## Verification

- `bash scripts/verify-m034-s02-workflows.sh`
- `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1`
- `bash -n scripts/verify-m034-s03.sh`

## Observability Impact

- Signals added/changed: refreshed repo-local workflow-contract and PowerShell helper logs under `.tmp/m034-s11/t01/`.
- How a future agent inspects this: rerun the three verification commands above and inspect `.tmp/m034-s11/t01/` plus the exact workflow/verifier diff.
- Failure state exposed: stale workflow drift vs deeper Windows staged-smoke breakage becomes explicit before any GitHub mutation.

## Inputs

- `.github/workflows/release.yml` — current release-lane workflow with the uncommitted Windows smoke-toolchain patch.
- `scripts/verify-m034-s02-workflows.sh` — repo-local contract gate that must match the workflow exactly.
- `scripts/verify-m034-s03.ps1` — Windows staged-smoke verifier to touch only if the local replay proves it is still wrong.
- `scripts/tests/verify-m034-s03-last-exitcode.ps1` — strict-mode regression guard for the PowerShell verifier.
- `.tmp/m034-s05/verify/remote-runs.json` — current canonical hosted evidence showing `release.yml` as the only failing lane.

## Expected Output

- `.github/workflows/release.yml` — finalized Windows smoke-toolchain workflow patch.
- `scripts/verify-m034-s02-workflows.sh` — aligned repo-local workflow contract for the repaired release lane.
- `scripts/verify-m034-s03.ps1` — only updated if the local replay proves a deeper Windows staged-smoke fix is required.
- `.tmp/m034-s11/t01/release-workflow-contract.log` — fresh local proof summary for the repaired workflow seam.
