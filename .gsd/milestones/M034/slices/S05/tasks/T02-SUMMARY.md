---
id: T02
parent: S05
milestone: M034
provides: []
requires: []
affects: []
key_files: ["README.md", "website/docs/docs/getting-started/index.md", "website/docs/docs/tooling/index.md", "tools/editors/vscode-mesh/package.json", "tools/editors/vscode-mesh/README.md", ".gsd/milestones/M034/slices/S05/tasks/T02-SUMMARY.md"]
key_decisions: ["Standardized the public install contract around the exact installer-pair URLs plus both-binary wording so S05 can verify release truth with simple exact-string checks."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the exact task-contract grep checks for installer URLs and meshpkg --version, confirmed the stale source-build-only wording and old mesh-lang/mesh slug are absent, read the extension manifest URLs with Node, verified the installer sources still expose snowdamiz/mesh-lang and meshpkg, and built the VitePress docs site with npm --prefix website run build. All commands passed."
completed_at: 2026-03-27T02:35:14.633Z
blocker_discovered: false
---

# T02: Aligned README/docs install-proof wording and VS Code extension metadata with the S05 public release contract.

> Aligned README/docs install-proof wording and VS Code extension metadata with the S05 public release contract.

## What Happened
---
id: T02
parent: S05
milestone: M034
key_files:
  - README.md
  - website/docs/docs/getting-started/index.md
  - website/docs/docs/tooling/index.md
  - tools/editors/vscode-mesh/package.json
  - tools/editors/vscode-mesh/README.md
  - .gsd/milestones/M034/slices/S05/tasks/T02-SUMMARY.md
key_decisions:
  - Standardized the public install contract around the exact installer-pair URLs plus both-binary wording so S05 can verify release truth with simple exact-string checks.
duration: ""
verification_result: passed
completed_at: 2026-03-27T02:35:14.635Z
blocker_discovered: false
---

# T02: Aligned README/docs install-proof wording and VS Code extension metadata with the S05 public release contract.

**Aligned README/docs install-proof wording and VS Code extension metadata with the S05 public release contract.**

## What Happened

Updated README.md, website/docs/docs/getting-started/index.md, website/docs/docs/tooling/index.md, and tools/editors/vscode-mesh/README.md so they all describe the same verified public installer pair (https://meshlang.dev/install.sh and https://meshlang.dev/install.ps1), explicitly name both meshc and meshpkg, and point readers at the same Production Backend Proof surface. Corrected tools/editors/vscode-mesh/package.json so the extension’s public repository and bugs URLs now point at snowdamiz/mesh-lang. Rechecked both installer sources and confirmed they already carried the expected snowdamiz/mesh-lang repo slug plus meshpkg references, so they did not need content changes.

## Verification

Ran the exact task-contract grep checks for installer URLs and meshpkg --version, confirmed the stale source-build-only wording and old mesh-lang/mesh slug are absent, read the extension manifest URLs with Node, verified the installer sources still expose snowdamiz/mesh-lang and meshpkg, and built the VitePress docs site with npm --prefix website run build. All commands passed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `rg -n 'meshlang.dev/install.sh|meshlang.dev/install.ps1|meshpkg --version' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md tools/editors/vscode-mesh/README.md` | 0 | ✅ pass | 97ms |
| 2 | `! rg -n 'Today the verified install path is building \`meshc\` from source|mesh-lang/mesh' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md website/docs/public/install.ps1 tools/editors/vscode-mesh/package.json tools/editors/vscode-mesh/README.md` | 0 | ✅ pass | 43ms |
| 3 | `node -p "require('./tools/editors/vscode-mesh/package.json').repository.url"` | 0 | ✅ pass | 205ms |
| 4 | `node -p "require('./tools/editors/vscode-mesh/package.json').bugs.url"` | 0 | ✅ pass | 214ms |
| 5 | `rg -n 'snowdamiz/mesh-lang|meshpkg' website/docs/public/install.sh website/docs/public/install.ps1` | 0 | ✅ pass | 16ms |
| 6 | `npm --prefix website run build` | 0 | ✅ pass | 26186ms |


## Deviations

None.

## Known Issues

T03 and T04 still need to compose these aligned local truth surfaces into the canonical S05 verifier, add public HTTP proof, and record hosted rollout evidence. This task only reconciles the local public contract those later checks will assert.

## Files Created/Modified

- `README.md`
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/tooling/index.md`
- `tools/editors/vscode-mesh/package.json`
- `tools/editors/vscode-mesh/README.md`
- `.gsd/milestones/M034/slices/S05/tasks/T02-SUMMARY.md`


## Deviations
None.

## Known Issues
T03 and T04 still need to compose these aligned local truth surfaces into the canonical S05 verifier, add public HTTP proof, and record hosted rollout evidence. This task only reconciles the local public contract those later checks will assert.
