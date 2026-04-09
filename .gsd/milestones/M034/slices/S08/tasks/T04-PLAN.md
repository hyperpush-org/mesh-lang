---
estimated_steps: 10
estimated_files: 10
skills_used: []
---

# T04: Repair `release.yml` release-asset verification so the hosted candidate tag can pass on Unix, macOS, and Windows.

Use the saved `release.yml` tag-run failure logs to repair the repo-owned release verification path instead of hand-waving around hosted drift. The fixes must cover the real blockers recorded by T02: the staged smoke path must truthfully satisfy the `libmesh_rt.a` requirement where the verifier expects it, checksum generation must no longer assume `sha256sum` on macOS, and the Windows checksum archive selection must use valid PowerShell syntax. Update the workflow-contract verifiers in the same task so local proof encodes the repaired hosted contract.

Steps:
1. Reproduce the failing `Verify release assets (...)` expectations from `.tmp/m034-s08/tag-rollout/release-v0.1.0-log-failed.txt` and the current `.github/workflows/release.yml` plus staged installer proof scripts.
2. Repair the release workflow and any repo-owned helper scripts so the Unix/macOS/Windows verification jobs are truthful for the staged assets they consume.
3. Update `scripts/verify-m034-s02-workflows.sh` and `scripts/verify-m034-s05-workflows.sh` if their current assertions encode the broken hosted behavior.
4. Keep the documented installer mirrors in `website/docs/public/` aligned if the underlying helper logic changes.

Must-haves:
- No `Verify release assets (...)` step depends on a host-only checksum tool assumption.
- The Windows checksum selection path is valid PowerShell instead of the broken `Select-Object -First 1,` form.
- The repo-owned workflow verifiers pass only when the repaired release contract is present.

## Inputs

- `.tmp/m034-s08/tag-rollout/release-v0.1.0-log-failed.txt`
- `.github/workflows/release.yml`
- `scripts/verify-m034-s03.sh`
- `scripts/verify-m034-s02-workflows.sh`
- `scripts/verify-m034-s05-workflows.sh`
- `tools/install/install.sh`
- `tools/install/install.ps1`
- `website/docs/public/install.sh`
- `website/docs/public/install.ps1`

## Expected Output

- `.github/workflows/release.yml`
- `scripts/verify-m034-s02-workflows.sh`
- `scripts/verify-m034-s05-workflows.sh`
- `.tmp/m034-s08/release-workflow-proof.log`

## Verification

bash -c 'set -euo pipefail; mkdir -p .tmp/m034-s08; bash scripts/verify-m034-s02-workflows.sh | tee .tmp/m034-s08/release-workflow-proof.log'
bash scripts/verify-m034-s05-workflows.sh
bash scripts/verify-m034-s03.sh
