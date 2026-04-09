---
estimated_steps: 4
estimated_files: 4
skills_used:
  - github-workflows
  - test
---

# T02: Expand release asset coverage and gate publication on installer smoke

**Slice:** S03 — Release assets and installer truth
**Milestone:** M034

## Description

Once the installer and staged verifier seam exists, make `release.yml` ship every asset those installers expect and prove them before publication. Extend `build-meshpkg` to cover Windows, add platform-native release-asset smoke that reuses the repo-local S03 verifier scripts instead of inline YAML assertions, and update the S02 workflow contract verifier so the release graph remains mechanically guarded after the new job and dependency edges land.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Release artifact build matrix | Fail the platform build job and block smoke/publication; do not allow partial asset sets to proceed. | Let the job fail visibly with the target name and missing artifact path in logs. | Treat a missing Windows `meshpkg` archive or wrong archive extension as contract drift. |
| Installer smoke jobs | Keep `Create Release` blocked if any platform verifier fails and surface the failing verifier phase plus artifact pair in job logs. | Stop the smoke job and leave the staged asset directory plus verifier output visible. | Treat missing verifier scripts or inlined installer assertions as drift; the workflow must call the repo-local proof scripts. |
| S02 release contract verifier | Reject job-name, needs-graph, permission, and asset-matrix drift locally before CI. | N/A | Treat missing `verify-release-assets` wiring or stale expected jobs as release-contract failure. |

## Load Profile

- **Shared resources**: GitHub runner minutes, cross-platform build matrices, artifact upload/download storage, and staged smoke temp directories.
- **Per-operation cost**: one extra Windows `meshpkg` build/package plus one installer smoke run per platform before release publication.
- **10x breakpoint**: artifact fan-out and queued release smoke would bottleneck first, so the workflow should reuse built artifacts and avoid recompiling inside the smoke jobs.

## Negative Tests

- **Malformed inputs**: missing Windows `meshpkg` artifact, smoke job missing one binary archive, or a release workflow that still publishes without the smoke gate.
- **Error paths**: checksum mismatch, installer-verifier failure, or a broken `needs` graph must keep `Create Release` blocked.
- **Boundary conditions**: the smoke jobs reuse `scripts/verify-m034-s03.sh` and `scripts/verify-m034-s03.ps1` instead of duplicating installer assertions inside YAML.

## Steps

1. Extend `.github/workflows/release.yml` so `build-meshpkg` emits a Windows `meshpkg-v<version>-x86_64-pc-windows-msvc.zip` artifact alongside the existing Unix archives, using platform-appropriate packaging for Windows.
2. Add a dedicated `verify-release-assets` job, or an equivalent platform-native smoke graph, that downloads the built artifacts, stages a local release directory plus `SHA256SUMS`, and runs `bash scripts/verify-m034-s03.sh` on Unix runners and `pwsh -File scripts/verify-m034-s03.ps1` on Windows.
3. Make `Create Release` depend on that installer-smoke job in addition to `build`, `build-meshpkg`, and `authoritative-live-proof`, so uploaded artifacts are already proven installable and runnable.
4. Extend `scripts/verify-m034-s02-workflows.sh` release-mode checks to require the new smoke job, the updated `release.needs` graph, Windows `meshpkg` asset production, and reuse of the repo-local S03 verifier scripts.

## Must-Haves

- [ ] The release workflow produces every asset the documented installers expect, including Windows `meshpkg`.
- [ ] Installer smoke runs from staged release assets before `Create Release` publishes anything.
- [ ] Repo-local S03 verifier scripts, not inline YAML logic, own installer truth in the workflow.
- [ ] The S02 workflow contract verifier stays authoritative after the new release job graph lands.

## Verification

- `bash scripts/verify-m034-s02-workflows.sh release`
- `ruby -e 'require "yaml"; YAML.load_file(".github/workflows/release.yml")'`
- `rg -n 'verify-release-assets|x86_64-pc-windows-msvc' .github/workflows/release.yml`

## Observability Impact

- Signals added/changed: `verify-release-assets` workflow logs, staged smoke artifacts, and stricter local release-contract diagnostics under `.tmp/m034-s02/verify/release.log`.
- How a future agent inspects this: rerun `bash scripts/verify-m034-s02-workflows.sh release` locally or inspect the release smoke job logs in GitHub Actions.
- Failure state exposed: whether drift came from asset coverage, smoke-job wiring, or blocked `Create Release` dependencies.

## Inputs

- `.github/workflows/release.yml` — current release graph with no installer smoke and no Windows `meshpkg` packaging.
- `scripts/verify-m034-s02-workflows.sh` — current S02 contract verifier that hard-codes the old release job set and needs graph.
- `scripts/verify-m034-s03.sh` — Unix staged installer verifier from T01 that the workflow must call.
- `scripts/verify-m034-s03.ps1` — Windows staged installer verifier from T01 that the workflow must call.

## Expected Output

- `.github/workflows/release.yml` — release workflow with Windows `meshpkg` assets and installer-smoke gating before publication.
- `scripts/verify-m034-s02-workflows.sh` — updated release-contract verifier that enforces the new job graph and asset coverage.
- `scripts/verify-m034-s03.sh` — verifier adjustments needed for workflow-driven staged asset smoke.
- `scripts/verify-m034-s03.ps1` — verifier adjustments needed for workflow-driven staged asset smoke on Windows.
