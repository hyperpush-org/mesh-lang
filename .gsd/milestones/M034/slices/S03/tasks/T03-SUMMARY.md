---
id: T03
parent: S03
milestone: M034
provides: []
requires: []
affects: []
key_files: ["README.md", "website/docs/docs/getting-started/index.md", "website/docs/docs/tooling/index.md", "tools/editors/vscode-mesh/README.md", ".gsd/milestones/M034/slices/S03/tasks/T03-SUMMARY.md"]
key_decisions: ["Document the docs-served installer scripts as the authoritative public install path and keep source builds framed as an explicit alternative workflow."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the full slice verification bar because T03 is the final task in S03. The installer copy drift checks still pass, the staged Unix installer smoke still proves install/version/hello-run truth, the release workflow contract still includes staged asset verification and Windows coverage, and the updated documentation now references the canonical install URLs while removing the old getting-started claim that source builds were the only verified path."
completed_at: 2026-03-27T00:00:49.478Z
blocker_discovered: false
---

# T03: Rewrote public install docs to use the verified installer path for meshc and meshpkg, with source builds kept as an explicit alternative.

> Rewrote public install docs to use the verified installer path for meshc and meshpkg, with source builds kept as an explicit alternative.

## What Happened
---
id: T03
parent: S03
milestone: M034
key_files:
  - README.md
  - website/docs/docs/getting-started/index.md
  - website/docs/docs/tooling/index.md
  - tools/editors/vscode-mesh/README.md
  - .gsd/milestones/M034/slices/S03/tasks/T03-SUMMARY.md
key_decisions:
  - Document the docs-served installer scripts as the authoritative public install path and keep source builds framed as an explicit alternative workflow.
duration: ""
verification_result: passed
completed_at: 2026-03-27T00:00:49.479Z
blocker_discovered: false
---

# T03: Rewrote public install docs to use the verified installer path for meshc and meshpkg, with source builds kept as an explicit alternative.

**Rewrote public install docs to use the verified installer path for meshc and meshpkg, with source builds kept as an explicit alternative.**

## What Happened

Updated the public documentation surfaces that introduce Mesh installation so they now consistently point at the verified docs-served installer scripts instead of presenting source builds as the only proven path. README Quick Start, the getting-started guide, the tooling guide, and the VS Code extension README now describe the installer contract for both meshc and meshpkg, state the platform coverage proven by the staged release flow, and keep source builds explicitly labeled as an alternative contributor / unsupported-target workflow.

## Verification

Ran the full slice verification bar because T03 is the final task in S03. The installer copy drift checks still pass, the staged Unix installer smoke still proves install/version/hello-run truth, the release workflow contract still includes staged asset verification and Windows coverage, and the updated documentation now references the canonical install URLs while removing the old getting-started claim that source builds were the only verified path.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -n tools/install/install.sh` | 0 | ✅ pass | 30ms |
| 2 | `diff -u tools/install/install.sh website/docs/public/install.sh` | 0 | ✅ pass | 24ms |
| 3 | `diff -u tools/install/install.ps1 website/docs/public/install.ps1` | 0 | ✅ pass | 24ms |
| 4 | `bash scripts/verify-m034-s03.sh` | 0 | ✅ pass | 35934ms |
| 5 | `bash scripts/verify-m034-s02-workflows.sh release` | 0 | ✅ pass | 570ms |
| 6 | `ruby -e 'require "yaml"; YAML.load_file(".github/workflows/release.yml")'` | 0 | ✅ pass | 442ms |
| 7 | `rg -n 'verify-release-assets|x86_64-pc-windows-msvc' .github/workflows/release.yml` | 0 | ✅ pass | 76ms |
| 8 | `rg -n 'meshlang.dev/install.sh|meshlang.dev/install.ps1' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md tools/editors/vscode-mesh/README.md` | 0 | ✅ pass | 57ms |
| 9 | `! rg -n 'Today the verified install path is building `meshc` from source' website/docs/docs/getting-started/index.md` | 0 | ✅ pass | 41ms |
| 10 | `rg -n 'meshc --version|meshpkg --version' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md` | 0 | ✅ pass | 61ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `README.md`
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/tooling/index.md`
- `tools/editors/vscode-mesh/README.md`
- `.gsd/milestones/M034/slices/S03/tasks/T03-SUMMARY.md`


## Deviations
None.

## Known Issues
None.
