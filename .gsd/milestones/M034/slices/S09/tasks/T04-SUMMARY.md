---
id: T04
parent: S09
milestone: M034
provides: []
requires: []
affects: []
key_files: [".github/workflows/publish-extension.yml", "scripts/verify-m034-s04-workflows.sh", "scripts/verify-m034-s03.ps1", "scripts/verify-m034-s01.sh", "scripts/tests/verify-m034-s03-last-exitcode.ps1", "scripts/tests/verify-m034-s01-fetch-retry.sh", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Use HaaLeo/publish-vscode-extension's documented `skipDuplicate: true` mode on both publish steps and enforce it in the local workflow contract instead of masking duplicate reruns with `continue-on-error`.", "Treat an unset PowerShell `$LASTEXITCODE` under strict mode as a successful pure-PowerShell command and preserve the verifier's log artifacts rather than letting the helper throw before artifact capture."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the exact task-plan verification commands: `bash scripts/verify-m034-s04-workflows.sh all`, `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1`, and `bash scripts/tests/verify-m034-s01-fetch-retry.sh`. Also confirmed `.tmp/m034-s09/rollout/target-sha.txt`, `remote-refs.before.txt`, `remote-refs.after.txt`, `workflow-status.json`, and `workflow-urls.txt` still exist and that `workflow-status.json` still records the pre-T04 `publish-extension.yml` failure on the approved rollout SHA, so T04's local blocker fixes are cleanly separated from the later hosted reroll work."
completed_at: 2026-03-27T18:56:07.893Z
blocker_discovered: false
---

# T04: Made extension reruns duplicate-safe, fixed the PowerShell strict-mode verifier helper, and added retry-covered local guards for the S01 metadata fetch path.

> Made extension reruns duplicate-safe, fixed the PowerShell strict-mode verifier helper, and added retry-covered local guards for the S01 metadata fetch path.

## What Happened
---
id: T04
parent: S09
milestone: M034
key_files:
  - .github/workflows/publish-extension.yml
  - scripts/verify-m034-s04-workflows.sh
  - scripts/verify-m034-s03.ps1
  - scripts/verify-m034-s01.sh
  - scripts/tests/verify-m034-s03-last-exitcode.ps1
  - scripts/tests/verify-m034-s01-fetch-retry.sh
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Use HaaLeo/publish-vscode-extension's documented `skipDuplicate: true` mode on both publish steps and enforce it in the local workflow contract instead of masking duplicate reruns with `continue-on-error`.
  - Treat an unset PowerShell `$LASTEXITCODE` under strict mode as a successful pure-PowerShell command and preserve the verifier's log artifacts rather than letting the helper throw before artifact capture.
duration: ""
verification_result: passed
completed_at: 2026-03-27T18:56:07.895Z
blocker_discovered: false
---

# T04: Made extension reruns duplicate-safe, fixed the PowerShell strict-mode verifier helper, and added retry-covered local guards for the S01 metadata fetch path.

**Made extension reruns duplicate-safe, fixed the PowerShell strict-mode verifier helper, and added retry-covered local guards for the S01 metadata fetch path.**

## What Happened

Updated `.github/workflows/publish-extension.yml` so both registry publish steps pass `skipDuplicate: true` while still publishing the exact VSIX artifact handed off by the reusable proof workflow, then tightened `scripts/verify-m034-s04-workflows.sh` so the local workflow contract fails if either publish step drops that reroll-safe setting. Fixed `scripts/verify-m034-s03.ps1` by reading `$LASTEXITCODE` through `Get-Variable ... -ErrorAction SilentlyContinue` and treating the unset case as exit code `0`, which removes the hosted Windows strict-mode throw while preserving log capture. Hardened `scripts/verify-m034-s01.sh` so the metadata/version/search GETs retry curl transport failures inside the existing `.tmp/m034-s01/verify/...` tree and still fail closed on wrong HTTP statuses. Added focused local regressions for both helper surfaces (`scripts/tests/verify-m034-s03-last-exitcode.ps1` and `scripts/tests/verify-m034-s01-fetch-retry.sh`) and verified the carried-forward T03 rollout artifacts still capture the pre-T04 hosted blocker on the approved target SHA.

## Verification

Passed the exact task-plan verification commands: `bash scripts/verify-m034-s04-workflows.sh all`, `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1`, and `bash scripts/tests/verify-m034-s01-fetch-retry.sh`. Also confirmed `.tmp/m034-s09/rollout/target-sha.txt`, `remote-refs.before.txt`, `remote-refs.after.txt`, `workflow-status.json`, and `workflow-urls.txt` still exist and that `workflow-status.json` still records the pre-T04 `publish-extension.yml` failure on the approved rollout SHA, so T04's local blocker fixes are cleanly separated from the later hosted reroll work.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash scripts/verify-m034-s04-workflows.sh all` | 0 | ✅ pass | 579ms |
| 2 | `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1` | 0 | ✅ pass | 2033ms |
| 3 | `bash scripts/tests/verify-m034-s01-fetch-retry.sh` | 0 | ✅ pass | 451ms |
| 4 | `python3 - <<'PYCHECK' ... verify rollout artifacts still capture the pre-T04 hosted blocker on the approved target SHA ... PYCHECK` | 0 | ✅ pass | 157ms |


## Deviations

Installed Homebrew `powershell` locally so the planned `pwsh` regression command could be run on this host instead of being left unverified.

## Known Issues

Hosted refs and workflow artifacts still reflect the pre-T04 reroll state from T03. `publish-extension.yml` remains recorded as a failure and `release.yml` was not rerun by this task. T05 still needs to produce a repaired rollout SHA and rerun the hosted evidence set.

## Files Created/Modified

- `.github/workflows/publish-extension.yml`
- `scripts/verify-m034-s04-workflows.sh`
- `scripts/verify-m034-s03.ps1`
- `scripts/verify-m034-s01.sh`
- `scripts/tests/verify-m034-s03-last-exitcode.ps1`
- `scripts/tests/verify-m034-s01-fetch-retry.sh`
- `.gsd/KNOWLEDGE.md`


## Deviations
Installed Homebrew `powershell` locally so the planned `pwsh` regression command could be run on this host instead of being left unverified.

## Known Issues
Hosted refs and workflow artifacts still reflect the pre-T04 reroll state from T03. `publish-extension.yml` remains recorded as a failure and `release.yml` was not rerun by this task. T05 still needs to produce a repaired rollout SHA and rerun the hosted evidence set.
