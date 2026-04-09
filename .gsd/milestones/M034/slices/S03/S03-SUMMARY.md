---
id: S03
parent: M034
milestone: M034
provides:
  - Canonical public installer scripts for Unix and Windows, with repo-local mirrored copies kept in lockstep.
  - Repo-local staged release verifiers plus a known-good hello fixture that prove checksum/install/version/build/run truth for installer-driven `meshc` and `meshpkg` installs.
  - A release workflow that ships every installer-facing asset, including Windows `meshpkg`, and blocks publication on installer smoke.
  - Docs and editor guidance that now match the verified installer contract instead of directing users to source builds as the only trustworthy path.
requires:
  - slice: S01
    provides: The live registry publish/install proof pattern and real release-path mindset that S03 extends from package-manager truth to installer truth.
  - slice: S02
    provides: The authoritative workflow-contract pattern and reusable release-proof gating model that S03 extends with installer smoke and release-asset coverage.
affects:
  - S04
  - S05
key_files:
  - website/docs/public/install.sh
  - website/docs/public/install.ps1
  - tools/install/install.sh
  - tools/install/install.ps1
  - scripts/verify-m034-s03.sh
  - scripts/verify-m034-s03.ps1
  - scripts/fixtures/m034-s03-installer-smoke/mesh.toml
  - scripts/fixtures/m034-s03-installer-smoke/main.mpl
  - .github/workflows/release.yml
  - scripts/verify-m034-s02-workflows.sh
  - README.md
  - website/docs/docs/getting-started/index.md
  - website/docs/docs/tooling/index.md
  - tools/editors/vscode-mesh/README.md
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - Treat `website/docs/public/install.{sh,ps1}` as the canonical installer sources and keep `tools/install/*` byte-identical mirrors.
  - Own installer truth in repo-local verifier scripts (`scripts/verify-m034-s03.sh` / `.ps1`) and have CI shell out to those scripts instead of duplicating installer assertions in workflow YAML.
  - Derive staged release archive versions from the Cargo versions so archive names, installer-written version files, and `meshc --version` / `meshpkg --version` all stay aligned during smoke verification.
patterns_established:
  - Public delivery surfaces that users actually execute should be canonicalized first, with repo-local mirrors treated as generated copies that must stay byte-identical.
  - Release verification should stage real release-style assets locally and in CI, then reuse the same checked-in proof scripts across environments instead of rewriting assertions in workflow YAML.
  - Installer smoke should prove both the happy path and exact failure phases (metadata, checksum, missing archive, missing extracted binary) so future regressions localize quickly instead of surfacing as generic release failures.
observability_surfaces:
  - `.tmp/m034-s03/verify/run/*.log` phase logs for installer metadata/checksum/install/build/runtime failures.
  - `.tmp/m034-s03/verify/run/staged-layout.txt` and `server-urls.log` to show exactly which staged assets and URLs the verifier exercised.
  - `.tmp/m034-s02/verify/release.log` for local workflow-contract drift failures.
  - GitHub Actions failure artifact uploads named `release-smoke-${{ matrix.target }}-diagnostics` from the `verify-release-assets` job.
drill_down_paths:
  - .gsd/milestones/M034/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M034/slices/S03/tasks/T02-SUMMARY.md
  - .gsd/milestones/M034/slices/S03/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-27T00:08:04.965Z
blocker_discovered: false
---

# S03: Release assets and installer truth

**S03 turned Mesh release uploads into a verified installer contract: the public install scripts are now canonical, staged verifier scripts prove checksum/install/runtime truth for `meshc` and `meshpkg`, `release.yml` publishes the full asset matrix and blocks publication on installer smoke, and the docs now point users at the verified installer path.**

## What Happened

Closed the slice by proving the exact public installer surface instead of treating uploaded archives as sufficient. `website/docs/public/install.sh` and `website/docs/public/install.ps1` are now the canonical installer sources, with `tools/install/install.sh` and `tools/install/install.ps1` kept byte-identical as repo-local mirrors. Both installers gained test-only staged-release hooks (`MESH_INSTALL_RELEASE_API_URL`, `MESH_INSTALL_RELEASE_BASE_URL`) plus strict-proof behavior so the unmodified public scripts can be pointed at staged assets and must fail hard on missing or malformed release metadata/checksum state. The Windows installer now targets `snowdamiz/mesh-lang` and installs/checksum-verifies both `meshc.exe` and `meshpkg.exe` instead of only `meshc`.

The repo now has installer truth verifiers that exercise release-style assets directly. `scripts/verify-m034-s03.sh` stages `meshc` and `meshpkg` archives plus `SHA256SUMS`, runs the documented Unix installer into an isolated HOME, checks `meshc --version` and `meshpkg --version`, builds a known-good hello fixture, runs the built binary, and also proves exact failure behavior for missing `tag_name`, malformed `SHA256SUMS`, missing `meshpkg` archives, and missing extracted binaries. `scripts/verify-m034-s03.ps1` mirrors that contract for Windows zipped assets and isolated `%USERPROFILE%` installs. The checked-in fixture under `scripts/fixtures/m034-s03-installer-smoke/` keeps the runnable smoke independent from the currently broken `meshc init` scaffold path.

On the release side, `release.yml` now ships the full installer-facing asset set, including `meshpkg-v<version>-x86_64-pc-windows-msvc.zip`, and inserts a `verify-release-assets` matrix job that downloads the staged artifacts, generates platform-native `SHA256SUMS`, and shells out to the repo-local S03 verifiers instead of embedding installer assertions inline in YAML. `Create Release` now depends on `verify-release-assets` as well as `build`, `build-meshpkg`, and `authoritative-live-proof`, and `scripts/verify-m034-s02-workflows.sh release` was extended so future drift in the smoke job, the release needs graph, Windows `meshpkg` coverage, or verifier reuse fails locally before CI does.

Docs were rewritten last so the public story matches the now-proven contract. `README.md`, `website/docs/docs/getting-started/index.md`, `website/docs/docs/tooling/index.md`, and `tools/editors/vscode-mesh/README.md` now tell users to install from `https://meshlang.dev/install.sh` or `https://meshlang.dev/install.ps1`, verify both `meshc --version` and `meshpkg --version`, and treat source builds as an explicit alternative for contributors or unsupported targets rather than the only trustworthy install path.

Operational readiness: health signal = green `bash scripts/verify-m034-s03.sh` plus green `bash scripts/verify-m034-s02-workflows.sh release`, and after push, green `verify-release-assets` matrix runs on GitHub Actions; failure signal = phase-local logs under `.tmp/m034-s03/verify/run`, workflow-contract drift in `.tmp/m034-s02/verify/release.log`, or CI artifacts named `release-smoke-<target>-diagnostics`; recovery procedure = rerun the repo-local verifier, inspect the exact failing phase (metadata/checksum/download/extract/build/runtime), repair the installer/workflow/asset contract, then rerun the smoke gate before allowing publication; monitoring gap = first live remote runner evidence for the new release-smoke matrix is still pending the next push to the remote default branch.

## Verification

Passed all slice-plan verification checks and the assembled staged installer proof surface:
- `bash -n tools/install/install.sh`
- `diff -u tools/install/install.sh website/docs/public/install.sh`
- `diff -u tools/install/install.ps1 website/docs/public/install.ps1`
- `bash scripts/verify-m034-s03.sh`
- `bash scripts/verify-m034-s02-workflows.sh release`
- `ruby -e 'require "yaml"; YAML.load_file(".github/workflows/release.yml")'`
- `rg -n 'verify-release-assets|x86_64-pc-windows-msvc' .github/workflows/release.yml`
- `rg -n 'meshlang.dev/install.sh|meshlang.dev/install.ps1' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md tools/editors/vscode-mesh/README.md`
- `! rg -n 'Today the verified install path is building \`meshc\` from source' website/docs/docs/getting-started/index.md`
- `rg -n 'meshc --version|meshpkg --version' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md`

The Unix staged verifier completed in local-build mode, installed both binaries into an isolated HOME, built and ran the hello fixture successfully, and confirmed hard-failure behavior for missing release `tag_name`, malformed checksums, missing `meshpkg` archives, and archives missing the expected binary.

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None to the slice contract. Closeout only refreshed `.gsd/KNOWLEDGE.md` and `.gsd/PROJECT.md` with the proved S03 state.

## Known Limitations

The first live GitHub Actions execution of the new `verify-release-assets` matrix is still pending the next push to the remote default branch. This macOS closeout exercised the Unix staged verifier locally; Windows coverage is present in `scripts/verify-m034-s03.ps1` and wired into `release.yml`, but its first runner execution must come from CI.

## Follow-ups

1. Push the workflow changes and capture the first successful remote `verify-release-assets` matrix run as execution evidence.
2. Reuse the repo-local smoke-script pattern in S04 so extension release verification is owned by checked-in verifiers rather than inline YAML checks.
3. In S05, compose S01 live registry proof, S02 authoritative workflow proof, S03 installer smoke, extension release checks, and public docs deployment into one release-candidate acceptance flow.

## Files Created/Modified

- `website/docs/public/install.sh` — Made the docs-served Unix installer the canonical source, added staged-release override hooks and strict-proof checksum/metadata failure behavior, and kept installation of both `meshc` and `meshpkg` on the public path.
- `website/docs/public/install.ps1` — Made the docs-served Windows installer canonical, pointed it at `snowdamiz/mesh-lang`, added staged-release override hooks and strict-proof behavior, and installed/checksum-verified both `meshc.exe` and `meshpkg.exe`.
- `tools/install/install.sh` — Synced the repo-local Unix installer copy to stay byte-identical with the canonical public script.
- `tools/install/install.ps1` — Synced the repo-local PowerShell installer copy to stay byte-identical with the canonical public script.
- `scripts/verify-m034-s03.sh` — Added the Unix staged-release verifier that stages release-style assets, runs the documented installer, checks both version commands, builds and runs the hello fixture, and exercises negative metadata/checksum/archive/binary failure cases.
- `scripts/verify-m034-s03.ps1` — Added the Windows staged-release verifier that stages zipped assets, installs through the documented PowerShell script, checks both binaries, and runs the hello fixture under an isolated `%USERPROFILE%`.
- `scripts/fixtures/m034-s03-installer-smoke/mesh.toml` — Added the known-good installer smoke manifest used by the staged verifiers.
- `scripts/fixtures/m034-s03-installer-smoke/main.mpl` — Added the hello fixture program that proves installed `meshc` can build a runnable binary.
- `.github/workflows/release.yml` — Extended the release graph to publish Windows `meshpkg`, generate staged `SHA256SUMS`, run `verify-release-assets` on each release target, and block `Create Release` on the new smoke gate.
- `scripts/verify-m034-s02-workflows.sh` — Extended the release contract verifier to require the new smoke job, the updated release needs graph, Windows `meshpkg` asset coverage, and reuse of the repo-local S03 verifier scripts.
- `README.md` — Rewrote the quick-start install story around the verified public installer path and kept source builds as an explicit alternative workflow.
- `website/docs/docs/getting-started/index.md` — Changed getting-started installation guidance to the verified installer path, with precise platform coverage and both version-check commands.
- `website/docs/docs/tooling/index.md` — Updated tooling docs to describe installer-based setup for `meshc` and `meshpkg` before formatter, test, registry, and editor usage.
- `tools/editors/vscode-mesh/README.md` — Updated the VS Code extension README so editor setup starts from the verified installer path and the documented `meshc`/`meshpkg` install contract.
- `.gsd/KNOWLEDGE.md` — Recorded the non-obvious S03 verifier pattern: reuse prebuilt release archives via `M034_S03_PREBUILT_RELEASE_DIR` and keep strict-proof mode enabled so installer regressions fail hard.
- `.gsd/PROJECT.md` — Refreshed current project state to reflect completed S03 installer/release-asset proof and the remaining M034 release-confidence gaps.
