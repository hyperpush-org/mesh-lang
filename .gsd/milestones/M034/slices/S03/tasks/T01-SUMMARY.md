---
id: T01
parent: S03
milestone: M034
provides: []
requires: []
affects: []
key_files: ["website/docs/public/install.sh", "website/docs/public/install.ps1", "tools/install/install.sh", "tools/install/install.ps1", "scripts/verify-m034-s03.sh", "scripts/verify-m034-s03.ps1", "scripts/fixtures/m034-s03-installer-smoke/mesh.toml", "scripts/fixtures/m034-s03-installer-smoke/main.mpl", ".gsd/DECISIONS.md", ".gsd/KNOWLEDGE.md"]
key_decisions: ["D083: Canonicalized on website/docs/public/install.{sh,ps1} as the installer source of truth and used test-only MESH_INSTALL_* override hooks for staged proof instead of a separate proof-only installer path.", "Recorded the staged-version alignment gotcha in .gsd/KNOWLEDGE.md so future installer proofs keep staged tags and embedded binary versions in sync."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-level verification passed: bash -n tools/install/install.sh, diff -u for both installer copy pairs, and bash scripts/verify-m034-s03.sh all succeeded. Slice-level checks also passed for bash scripts/verify-m034-s02-workflows.sh release, ruby YAML parsing of .github/workflows/release.yml, and the current installer-doc URL grep. The staged proof confirmed that the public Unix installer can install meshc and meshpkg from staged assets, that strict proof mode fails on malformed metadata/checksum/archive scenarios, and that the installed meshc can build and run the checked-in hello fixture. Local PowerShell runtime verification was not possible on this host because pwsh is unavailable."
completed_at: 2026-03-26T23:30:53.739Z
blocker_discovered: false
---

# T01: Canonicalized public installers, added staged release proof hooks, and landed staged installer smoke verifiers.

> Canonicalized public installers, added staged release proof hooks, and landed staged installer smoke verifiers.

## What Happened
---
id: T01
parent: S03
milestone: M034
key_files:
  - website/docs/public/install.sh
  - website/docs/public/install.ps1
  - tools/install/install.sh
  - tools/install/install.ps1
  - scripts/verify-m034-s03.sh
  - scripts/verify-m034-s03.ps1
  - scripts/fixtures/m034-s03-installer-smoke/mesh.toml
  - scripts/fixtures/m034-s03-installer-smoke/main.mpl
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D083: Canonicalized on website/docs/public/install.{sh,ps1} as the installer source of truth and used test-only MESH_INSTALL_* override hooks for staged proof instead of a separate proof-only installer path.
  - Recorded the staged-version alignment gotcha in .gsd/KNOWLEDGE.md so future installer proofs keep staged tags and embedded binary versions in sync.
duration: ""
verification_result: passed
completed_at: 2026-03-26T23:30:53.740Z
blocker_discovered: false
---

# T01: Canonicalized public installers, added staged release proof hooks, and landed staged installer smoke verifiers.

**Canonicalized public installers, added staged release proof hooks, and landed staged installer smoke verifiers.**

## What Happened

Canonicalized the public installer scripts under website/docs/public/ and synced the repo-local tools/install mirrors to them byte-for-byte. Added staged-release override hooks (MESH_INSTALL_RELEASE_API_URL, MESH_INSTALL_RELEASE_BASE_URL, MESH_INSTALL_STRICT_PROOF) so the documented installers can be pointed at locally staged release assets without changing default GitHub behavior, tightened checksum/timeout/error handling for proof mode, and extended the Windows installer to use the real snowdamiz/mesh-lang repo while installing both meshc.exe and meshpkg.exe. Added bash and PowerShell staged installer verifiers plus a checked-in hello fixture; the Unix verifier now stages release-style archives and SHA256SUMS under .tmp/m034-s03/, serves them from a local HTTP mirror, proves happy-path install/version/build/run behavior, and exercises the planned negative cases for missing tag_name, malformed SHA256SUMS, missing meshpkg asset, and missing extracted binary.

## Verification

Task-level verification passed: bash -n tools/install/install.sh, diff -u for both installer copy pairs, and bash scripts/verify-m034-s03.sh all succeeded. Slice-level checks also passed for bash scripts/verify-m034-s02-workflows.sh release, ruby YAML parsing of .github/workflows/release.yml, and the current installer-doc URL grep. The staged proof confirmed that the public Unix installer can install meshc and meshpkg from staged assets, that strict proof mode fails on malformed metadata/checksum/archive scenarios, and that the installed meshc can build and run the checked-in hello fixture. Local PowerShell runtime verification was not possible on this host because pwsh is unavailable.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -n tools/install/install.sh` | 0 | ✅ pass | 19ms |
| 2 | `diff -u tools/install/install.sh website/docs/public/install.sh` | 0 | ✅ pass | 40ms |
| 3 | `diff -u tools/install/install.ps1 website/docs/public/install.ps1` | 0 | ✅ pass | 24ms |
| 4 | `bash scripts/verify-m034-s03.sh` | 0 | ✅ pass | 23425ms |
| 5 | `bash scripts/verify-m034-s02-workflows.sh release` | 0 | ✅ pass | 249ms |
| 6 | `ruby -e 'require "yaml"; YAML.load_file(".github/workflows/release.yml")'` | 0 | ✅ pass | 162ms |
| 7 | `rg -n 'meshlang.dev/install.sh|meshlang.dev/install.ps1' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md tools/editors/vscode-mesh/README.md` | 0 | ✅ pass | 33ms |


## Deviations

None.

## Known Issues

pwsh is not installed on this host, so the new PowerShell installer/verifier path was not runtime-exercised locally. Windows runtime proof is deferred to a Windows-capable environment or CI.

## Files Created/Modified

- `website/docs/public/install.sh`
- `website/docs/public/install.ps1`
- `tools/install/install.sh`
- `tools/install/install.ps1`
- `scripts/verify-m034-s03.sh`
- `scripts/verify-m034-s03.ps1`
- `scripts/fixtures/m034-s03-installer-smoke/mesh.toml`
- `scripts/fixtures/m034-s03-installer-smoke/main.mpl`
- `.gsd/DECISIONS.md`
- `.gsd/KNOWLEDGE.md`


## Deviations
None.

## Known Issues
pwsh is not installed on this host, so the new PowerShell installer/verifier path was not runtime-exercised locally. Windows runtime proof is deferred to a Windows-capable environment or CI.
