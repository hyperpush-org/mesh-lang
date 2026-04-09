---
estimated_steps: 6
estimated_files: 6
skills_used: []
---

# T04: Repair reroll-safe hosted release and extension blockers locally

Update the reroll-sensitive release surfaces before touching remote refs again.
- Make `.github/workflows/publish-extension.yml` idempotent for reruns on the same `ext-v*` version so Open VSX/Marketplace duplicate publishes do not fail the caller workflow after the verified VSIX handoff has already passed.
- Extend `scripts/verify-m034-s04-workflows.sh` so the workflow contract enforces the duplicate-safe publish semantics and still guarantees the exact proof artifact handoff.
- Fix `scripts/verify-m034-s03.ps1` so `Invoke-LoggedCommand` does not throw under `Set-StrictMode -Version Latest` when `$LASTEXITCODE` has not been initialized, and add a focused PowerShell regression test that proves the helper treats the unset case as success.
- Harden `scripts/verify-m034-s01.sh` around the package metadata/version/search fetches with a fail-closed retry budget so a single transient curl SSL timeout does not sink `release.yml`, and add a small local regression harness that proves the fetch path retries transport failure before failing.
- Preserve durable diagnostics in the existing verifier artifact trees rather than inventing a new ad-hoc path.

## Inputs

- `.github/workflows/publish-extension.yml`
- `scripts/verify-m034-s04-workflows.sh`
- `scripts/verify-m034-s03.ps1`
- `scripts/verify-m034-s01.sh`
- `.tmp/m034-s09/rollout/workflow-status.json`
- `gh run view 23661341870 --repo snowdamiz/mesh-lang --job 68933430479 --log-failed`
- `gh run view 23661341416 --repo snowdamiz/mesh-lang --job 68932118207 --log-failed`
- `gh run view 23661341416 --repo snowdamiz/mesh-lang --job 68933643822 --log-failed`

## Expected Output

- `.github/workflows/publish-extension.yml`
- `scripts/verify-m034-s04-workflows.sh`
- `scripts/verify-m034-s03.ps1`
- `scripts/verify-m034-s01.sh`
- `scripts/tests/verify-m034-s03-last-exitcode.ps1`
- `scripts/tests/verify-m034-s01-fetch-retry.sh`

## Verification

bash scripts/verify-m034-s04-workflows.sh all
pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1
bash scripts/tests/verify-m034-s01-fetch-retry.sh
