---
id: T01
parent: S03
milestone: M036
provides: []
requires: []
affects: []
key_files: ["website/docs/docs/tooling/index.md", "tools/editors/vscode-mesh/README.md", "tools/editors/neovim-mesh/README.md", "scripts/tests/verify-m036-s03-contract.test.mjs", ".gsd/milestones/M036/slices/S03/tasks/T01-SUMMARY.md"]
key_decisions: ["Enforced the support-tier contract with targeted string/section assertions plus an embedded replay of the existing M034 local-docs helper instead of a broad snapshot test."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-level verification passed with `node --test scripts/tests/verify-m036-s03-contract.test.mjs && python3 scripts/lib/m034_public_surface_contract.py local-docs --root "$PWD"`. Slice-level checks already available in T01 also passed: `npm --prefix website run build` and `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh`. I also ran the remaining slice-level commands to record the current boundary honestly: `npm --prefix tools/editors/vscode-mesh run test:smoke` failed because the script is not implemented yet, and `bash scripts/verify-m036-s03.sh` failed because the assembled verifier has not been created yet."
completed_at: 2026-03-28T06:39:10.951Z
blocker_discovered: false
---

# T01: Published explicit first-class vs best-effort editor support tiers across tooling docs and editor READMEs, backed by a fail-closed contract test.

> Published explicit first-class vs best-effort editor support tiers across tooling docs and editor READMEs, backed by a fail-closed contract test.

## What Happened
---
id: T01
parent: S03
milestone: M036
key_files:
  - website/docs/docs/tooling/index.md
  - tools/editors/vscode-mesh/README.md
  - tools/editors/neovim-mesh/README.md
  - scripts/tests/verify-m036-s03-contract.test.mjs
  - .gsd/milestones/M036/slices/S03/tasks/T01-SUMMARY.md
key_decisions:
  - Enforced the support-tier contract with targeted string/section assertions plus an embedded replay of the existing M034 local-docs helper instead of a broad snapshot test.
duration: ""
verification_result: mixed
completed_at: 2026-03-28T06:39:10.952Z
blocker_discovered: false
---

# T01: Published explicit first-class vs best-effort editor support tiers across tooling docs and editor READMEs, backed by a fail-closed contract test.

**Published explicit first-class vs best-effort editor support tiers across tooling docs and editor READMEs, backed by a fail-closed contract test.**

## What Happened

Updated the public tooling page to define one explicit first-class versus best-effort editor support contract, split editor guidance into VS Code, Neovim, and best-effort sections, and bounded format-on-save plus LSP configuration wording to those tiers. Updated the VS Code README to call VS Code first-class and point back to the tooling contract without speaking for other editors. Updated the Neovim README to graduate from the prior repo-local caveat to the public first-class promise while keeping claims limited to the audited classic syntax plus native `meshc lsp` path already proven in S02. Added a repo-owned node:test contract suite that fails closed on stale tier wording, missing headings/table rows, and regressed Neovim language while also replaying the existing M034 local-docs helper so tooling-page marker drift still fails inside this task.

## Verification

Task-level verification passed with `node --test scripts/tests/verify-m036-s03-contract.test.mjs && python3 scripts/lib/m034_public_surface_contract.py local-docs --root "$PWD"`. Slice-level checks already available in T01 also passed: `npm --prefix website run build` and `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh`. I also ran the remaining slice-level commands to record the current boundary honestly: `npm --prefix tools/editors/vscode-mesh run test:smoke` failed because the script is not implemented yet, and `bash scripts/verify-m036-s03.sh` failed because the assembled verifier has not been created yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m036-s03-contract.test.mjs && python3 scripts/lib/m034_public_surface_contract.py local-docs --root "$PWD"` | 0 | ✅ pass | 586ms |
| 2 | `npm --prefix website run build` | 0 | ✅ pass | 13733ms |
| 3 | `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh` | 0 | ✅ pass | 8113ms |
| 4 | `npm --prefix tools/editors/vscode-mesh run test:smoke` | 1 | ❌ fail | 240ms |
| 5 | `bash scripts/verify-m036-s03.sh` | 127 | ❌ fail | 11ms |


## Deviations

None.

## Known Issues

`npm --prefix tools/editors/vscode-mesh run test:smoke` does not exist yet (owned by T02), and `scripts/verify-m036-s03.sh` does not exist yet (owned by T03).

## Files Created/Modified

- `website/docs/docs/tooling/index.md`
- `tools/editors/vscode-mesh/README.md`
- `tools/editors/neovim-mesh/README.md`
- `scripts/tests/verify-m036-s03-contract.test.mjs`
- `.gsd/milestones/M036/slices/S03/tasks/T01-SUMMARY.md`


## Deviations
None.

## Known Issues
`npm --prefix tools/editors/vscode-mesh run test:smoke` does not exist yet (owned by T02), and `scripts/verify-m036-s03.sh` does not exist yet (owned by T03).
