---
id: T02
parent: S03
milestone: M034
provides: []
requires: []
affects: []
key_files: [".github/workflows/release.yml", "scripts/verify-m034-s02-workflows.sh", "scripts/verify-m034-s03.sh", "scripts/verify-m034-s03.ps1", ".gsd/DECISIONS.md"]
key_decisions: ["D084: Derive staged release archive versions from the repo Cargo version and fail tag/Cargo divergence so installer smoke stays honest on both tag and non-tag refs.", "Route workflow installer proof through the repo-local S03 verifier scripts with `M034_S03_PREBUILT_RELEASE_DIR` instead of duplicating installer assertions inside YAML."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the task’s required slice checks with `bash scripts/verify-m034-s02-workflows.sh release`, `ruby -e 'require "yaml"; YAML.load_file(".github/workflows/release.yml")'`, and `rg -n 'verify-release-assets|x86_64-pc-windows-msvc' .github/workflows/release.yml`. Also reran `bash scripts/verify-m034-s03.sh` to prove the updated Unix verifier still passes in full local mode, then staged local archives and reran the verifier in prebuilt-asset mode to mirror the new workflow smoke path. Confirmed the expected observability artifacts exist under `.tmp/m034-s02/verify/` and `.tmp/m034-s03/verify/run/`."
completed_at: 2026-03-26T23:54:16.266Z
blocker_discovered: false
---

# T02: Gated release publication on staged installer smoke and shipped Windows meshpkg asset coverage.

> Gated release publication on staged installer smoke and shipped Windows meshpkg asset coverage.

## What Happened
---
id: T02
parent: S03
milestone: M034
key_files:
  - .github/workflows/release.yml
  - scripts/verify-m034-s02-workflows.sh
  - scripts/verify-m034-s03.sh
  - scripts/verify-m034-s03.ps1
  - .gsd/DECISIONS.md
key_decisions:
  - D084: Derive staged release archive versions from the repo Cargo version and fail tag/Cargo divergence so installer smoke stays honest on both tag and non-tag refs.
  - Route workflow installer proof through the repo-local S03 verifier scripts with `M034_S03_PREBUILT_RELEASE_DIR` instead of duplicating installer assertions inside YAML.
duration: ""
verification_result: passed
completed_at: 2026-03-26T23:54:16.274Z
blocker_discovered: false
---

# T02: Gated release publication on staged installer smoke and shipped Windows meshpkg asset coverage.

**Gated release publication on staged installer smoke and shipped Windows meshpkg asset coverage.**

## What Happened

Extended `release.yml` so `build-meshpkg` now publishes the missing Windows `meshpkg` zip, release archives stay version-aligned with the repo Cargo version instead of a proof-breaking `dev` label, and a new `verify-release-assets` matrix job downloads staged artifacts, regenerates target-local `SHA256SUMS`, and runs the repo-local Unix/Windows S03 verifier scripts before release publication. Updated the Unix and PowerShell verifier scripts with a `M034_S03_PREBUILT_RELEASE_DIR` mode so CI smoke can reuse downloaded archives without recompiling while still proving installer, version, and hello-build/run behavior. Tightened `scripts/verify-m034-s02-workflows.sh` so the local release contract now rejects drift in the new smoke job wiring, Windows asset coverage, repo-local verifier reuse, release needs graph, and staged-version alignment.

## Verification

Passed the task’s required slice checks with `bash scripts/verify-m034-s02-workflows.sh release`, `ruby -e 'require "yaml"; YAML.load_file(".github/workflows/release.yml")'`, and `rg -n 'verify-release-assets|x86_64-pc-windows-msvc' .github/workflows/release.yml`. Also reran `bash scripts/verify-m034-s03.sh` to prove the updated Unix verifier still passes in full local mode, then staged local archives and reran the verifier in prebuilt-asset mode to mirror the new workflow smoke path. Confirmed the expected observability artifacts exist under `.tmp/m034-s02/verify/` and `.tmp/m034-s03/verify/run/`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash scripts/verify-m034-s02-workflows.sh release` | 0 | ✅ pass | 448ms |
| 2 | `ruby -e 'require "yaml"; YAML.load_file(".github/workflows/release.yml")'` | 0 | ✅ pass | 464ms |
| 3 | `rg -n 'verify-release-assets|x86_64-pc-windows-msvc' .github/workflows/release.yml` | 0 | ✅ pass | 40ms |
| 4 | `bash scripts/verify-m034-s03.sh` | 0 | ✅ pass | 35408ms |
| 5 | `prepare .tmp/m034-s03-prebuilt-check and run M034_S03_PREBUILT_RELEASE_DIR=.tmp/m034-s03-prebuilt-check bash scripts/verify-m034-s03.sh` | 0 | ✅ pass | 26482ms |
| 6 | `test -f .tmp/m034-s02/verify/release.log && test -f .tmp/m034-s03/verify/run/staged-layout.txt && test -f .tmp/m034-s03/verify/run/server-urls.log` | 0 | ✅ pass | 15ms |


## Deviations

Aligned non-tag release archive naming away from `dev` and onto the repo Cargo version so the new installer smoke stays honest about the version written into `~/.mesh/version` and reported by the installed binaries.

## Known Issues

`pwsh` is still unavailable on this host, so the updated `scripts/verify-m034-s03.ps1` verifier was not runtime-exercised locally. Windows runtime proof is now wired into the new `verify-release-assets` CI job, but local host-level PowerShell execution still requires a Windows-capable environment.

## Files Created/Modified

- `.github/workflows/release.yml`
- `scripts/verify-m034-s02-workflows.sh`
- `scripts/verify-m034-s03.sh`
- `scripts/verify-m034-s03.ps1`
- `.gsd/DECISIONS.md`


## Deviations
Aligned non-tag release archive naming away from `dev` and onto the repo Cargo version so the new installer smoke stays honest about the version written into `~/.mesh/version` and reported by the installed binaries.

## Known Issues
`pwsh` is still unavailable on this host, so the updated `scripts/verify-m034-s03.ps1` verifier was not runtime-exercised locally. Windows runtime proof is now wired into the new `verify-release-assets` CI job, but local host-level PowerShell execution still requires a Windows-capable environment.
