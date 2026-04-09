---
estimated_steps: 4
estimated_files: 8
skills_used:
  - test
---

# T01: Canonicalize the installer scripts and add staged release proof hooks

**Slice:** S03 — Release assets and installer truth
**Milestone:** M034

## Description

Make the docs-served installer copies the real source of truth before touching CI. The public Unix script already works while the repo-local copy drifted, and the Windows script still points at the wrong repo and only installs `meshc`. Canonicalize on `website/docs/public/install.{sh,ps1}`, keep `tools/install/*` byte-identical, add test-only release URL override hooks that default to GitHub, and land staged-release verifiers plus a known-good hello fixture so S03 can prove installer behavior without waiting for a live tag.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Local staged release server / GitHub release endpoints | Fail the verifier at the exact download or checksum phase and leave the staged root plus requested URL in logs. | Stop without retry loops and report which asset URL stalled. | Treat missing `tag_name`, missing archive entries, or missing asset files as proof failure. |
| Installer checksum contract | Abort install and keep the mismatched archive plus expected hash visible; never continue with an unchecked binary. | N/A | Treat malformed `SHA256SUMS` or missing archive lines as verifier failures instead of warnings in staged-proof mode. |
| Public vs repo installer copies | Refuse proof if `tools/install/*` drifts from `website/docs/public/*` so the docs-served contract stays canonical. | N/A | Treat the wrong repo slug or a Windows installer that still omits `meshpkg` as contract drift. |

## Load Profile

- **Shared resources**: temp staging directories under `.tmp/m034-s03/`, downloaded archives, checksum manifests, and isolated HOME/PATH rewrites for smoke runs.
- **Per-operation cost**: release-style archive staging, two installer downloads/extracts per platform, and one hello build/run with the installed `meshc`.
- **10x breakpoint**: repeated archive staging and checksum scans would dominate first, so the verifier should prepare the staged release root once per run and reuse it across checks.

## Negative Tests

- **Malformed inputs**: missing `meshpkg` archive, wrong repo metadata, malformed `SHA256SUMS`, or drift between public and repo-local installer copies.
- **Error paths**: checksum mismatch, missing binary inside an archive, or installer failure after PATH/HOME setup must stop the proof and keep the failing phase log.
- **Boundary conditions**: Unix and Windows both install `meshc` plus `meshpkg`, and the hello smoke uses a known-good `println("hello")` fixture instead of the currently broken `meshc init` scaffold.

## Steps

1. Make `website/docs/public/install.sh` and `website/docs/public/install.ps1` the canonical installer sources, sync `tools/install/install.sh` and `tools/install/install.ps1` to them, and keep both pairs byte-identical.
2. Add test-only override hooks for the release API and asset base URLs that default to the current public GitHub endpoints so ordinary user installs stay unchanged while verifiers can point at staged local assets.
3. Extend the PowerShell installer to use the real `snowdamiz/mesh-lang` repo and install/checksum-verify both `meshc.exe` and `meshpkg.exe`; preserve Unix installation of both binaries and fix any remaining cleanup or drift bugs only in the canonical public source.
4. Add `scripts/verify-m034-s03.sh`, `scripts/verify-m034-s03.ps1`, and a checked-in hello fixture under `scripts/fixtures/m034-s03-installer-smoke/` that stage release-style archives plus `SHA256SUMS`, run the documented installer scripts against that staged source, verify `meshc --version` plus `meshpkg --version`, and build/run the known-good hello program.

## Must-Haves

- [ ] The public installer copies are canonical and the repo-local copies are byte-identical.
- [ ] Both installers accept test-only staged-release overrides without changing default public behavior.
- [ ] The Windows installer points at `snowdamiz/mesh-lang` and installs both `meshc` and `meshpkg`.
- [ ] Repo-local staged verifiers prove checksum, install, runnable binaries, and hello build/run truth.

## Verification

- `bash -n tools/install/install.sh`
- `diff -u tools/install/install.sh website/docs/public/install.sh`
- `diff -u tools/install/install.ps1 website/docs/public/install.ps1`
- `bash scripts/verify-m034-s03.sh`

## Observability Impact

- Signals added/changed: phase-scoped verifier logs and staged asset manifests under `.tmp/m034-s03/verify/`.
- How a future agent inspects this: rerun `bash scripts/verify-m034-s03.sh` locally or the PowerShell verifier inside the release workflow.
- Failure state exposed: the first failing asset, checksum, install, or hello-build phase and the URL/archive involved.

## Inputs

- `website/docs/public/install.sh` — real docs-served Unix installer contract.
- `tools/install/install.sh` — drifted repo-local Unix copy that must stop diverging.
- `website/docs/public/install.ps1` — public Windows installer contract with repo and binary gaps.
- `tools/install/install.ps1` — repo-local Windows copy that must stay identical to the public script.
- `scripts/fixtures/m034-s01-consumer/main.mpl` — known-good minimal Mesh hello shape to mirror for installer smoke.

## Expected Output

- `website/docs/public/install.sh` — canonical Unix installer with staged-release override hooks.
- `tools/install/install.sh` — repo-local Unix copy kept byte-identical to the public script.
- `website/docs/public/install.ps1` — canonical Windows installer with correct repo, dual-binary install, and staged-release hooks.
- `tools/install/install.ps1` — repo-local Windows copy kept byte-identical to the public script.
- `scripts/verify-m034-s03.sh` — repo-local staged release verifier for the documented Unix installer path.
- `scripts/verify-m034-s03.ps1` — repo-local staged release verifier for the documented Windows installer path.
- `scripts/fixtures/m034-s03-installer-smoke/mesh.toml` — known-good hello fixture manifest for installer smoke.
- `scripts/fixtures/m034-s03-installer-smoke/main.mpl` — known-good hello fixture entrypoint used by the verifiers.
