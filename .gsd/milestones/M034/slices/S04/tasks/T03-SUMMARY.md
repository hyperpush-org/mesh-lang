---
id: T03
parent: S04
milestone: M034
provides: []
requires: []
affects: []
key_files: [".github/workflows/extension-release-proof.yml", ".github/workflows/publish-extension.yml", "scripts/verify-m034-s04-workflows.sh", ".gsd/DECISIONS.md", ".gsd/milestones/M034/slices/S04/tasks/T03-SUMMARY.md"]
key_decisions: ["D087: Only `.github/workflows/extension-release-proof.yml` may run `scripts/verify-m034-s04-extension.sh`; it emits exact VSIX path/artifact outputs and uploads `extension-release-vsix` for the publish job to consume.", "Use `.tmp/m034-s04/workflows/phase-report.txt` plus per-phase logs as the local observability surface for workflow-contract drift."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "`npm --prefix tools/editors/vscode-mesh run test:vsix-path` passed; `EXPECTED_TAG="ext-v$(node -p \"require('./tools/editors/vscode-mesh/package.json').version\")" bash scripts/verify-m034-s04-extension.sh` reran the full extension prereq/package/zip-audit/`e2e_lsp` proof successfully; `bash -n scripts/verify-m034-s04-workflows.sh`, `bash scripts/verify-m034-s04-workflows.sh`, and the Ruby YAML-load sweep all passed; grep gates confirmed publish-extension references `./.github/workflows/extension-release-proof.yml`, both registry steps publish the same `extensionFile`, and the old `continue-on-error`, `ls *.vsix`, and direct verifier invocation patterns are gone. Observability checks passed for `.tmp/m034-s04/workflows/phase-report.txt` and `.tmp/m034-s04/verify/{verified-vsix-path.txt,vsix-contents.txt,status.txt}`, and the shipped VSIX manifest still contains `vscode-languageclient` / `vscode-jsonrpc` runtime entries."
completed_at: 2026-03-27T01:22:37.755Z
blocker_discovered: false
---

# T03: Added a reusable extension proof workflow and fail-closed publish handoff for the exact verified VSIX.

> Added a reusable extension proof workflow and fail-closed publish handoff for the exact verified VSIX.

## What Happened
---
id: T03
parent: S04
milestone: M034
key_files:
  - .github/workflows/extension-release-proof.yml
  - .github/workflows/publish-extension.yml
  - scripts/verify-m034-s04-workflows.sh
  - .gsd/DECISIONS.md
  - .gsd/milestones/M034/slices/S04/tasks/T03-SUMMARY.md
key_decisions:
  - D087: Only `.github/workflows/extension-release-proof.yml` may run `scripts/verify-m034-s04-extension.sh`; it emits exact VSIX path/artifact outputs and uploads `extension-release-vsix` for the publish job to consume.
  - Use `.tmp/m034-s04/workflows/phase-report.txt` plus per-phase logs as the local observability surface for workflow-contract drift.
duration: ""
verification_result: passed
completed_at: 2026-03-27T01:22:37.758Z
blocker_discovered: false
---

# T03: Added a reusable extension proof workflow and fail-closed publish handoff for the exact verified VSIX.

**Added a reusable extension proof workflow and fail-closed publish handoff for the exact verified VSIX.**

## What Happened

Created `.github/workflows/extension-release-proof.yml` as the sole workflow owner of `bash scripts/verify-m034-s04-extension.sh`, including Node/LLVM/Rust setup, exact VSIX path capture, verified artifact upload, and failure diagnostics retention. Rewrote `.github/workflows/publish-extension.yml` into a thin tag-triggered caller that depends on the reusable proof, downloads the exact verified artifact back into the deterministic dist path, logs/tests the handoff, and publishes the same `needs.proof.outputs.verified_vsix_path` file to both registries with no `continue-on-error`, no `ls *.vsix`, and no inline repackaging. Added `scripts/verify-m034-s04-workflows.sh` to parse both workflows with Ruby YAML and mechanically reject reusable-owner drift, missing outputs/artifact handoff, diagnostics drift, deterministic `extensionFile` drift, fail-open publication, or globbing/inline verifier logic before CI.

## Verification

`npm --prefix tools/editors/vscode-mesh run test:vsix-path` passed; `EXPECTED_TAG="ext-v$(node -p \"require('./tools/editors/vscode-mesh/package.json').version\")" bash scripts/verify-m034-s04-extension.sh` reran the full extension prereq/package/zip-audit/`e2e_lsp` proof successfully; `bash -n scripts/verify-m034-s04-workflows.sh`, `bash scripts/verify-m034-s04-workflows.sh`, and the Ruby YAML-load sweep all passed; grep gates confirmed publish-extension references `./.github/workflows/extension-release-proof.yml`, both registry steps publish the same `extensionFile`, and the old `continue-on-error`, `ls *.vsix`, and direct verifier invocation patterns are gone. Observability checks passed for `.tmp/m034-s04/workflows/phase-report.txt` and `.tmp/m034-s04/verify/{verified-vsix-path.txt,vsix-contents.txt,status.txt}`, and the shipped VSIX manifest still contains `vscode-languageclient` / `vscode-jsonrpc` runtime entries.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix tools/editors/vscode-mesh run test:vsix-path` | 0 | ✅ pass | 1240ms |
| 2 | `EXPECTED_TAG="ext-v$(node -p "require('./tools/editors/vscode-mesh/package.json').version")" bash scripts/verify-m034-s04-extension.sh` | 0 | ✅ pass | 37074ms |
| 3 | `bash -n scripts/verify-m034-s04-workflows.sh` | 0 | ✅ pass | 38ms |
| 4 | `bash scripts/verify-m034-s04-workflows.sh` | 0 | ✅ pass | 935ms |
| 5 | `ruby -e 'require "yaml"; %w[.github/workflows/extension-release-proof.yml .github/workflows/publish-extension.yml].each { |f| YAML.load_file(f) }'` | 0 | ✅ pass | 318ms |
| 6 | `! rg -n 'continue-on-error|ls \*\.vsix|bash scripts/verify-m034-s04-extension.sh' .github/workflows/publish-extension.yml` | 0 | ✅ pass | 83ms |
| 7 | `rg -n './.github/workflows/extension-release-proof.yml|extensionFile:' .github/workflows/publish-extension.yml` | 0 | ✅ pass | 43ms |
| 8 | `test -f .tmp/m034-s04/workflows/phase-report.txt && rg -n 'reusable.*passed|publish.*passed|full-contract.*passed' .tmp/m034-s04/workflows/phase-report.txt` | 0 | ✅ pass | 53ms |
| 9 | `test -f .tmp/m034-s04/verify/verified-vsix-path.txt && test -f .tmp/m034-s04/verify/vsix-contents.txt && test -f .tmp/m034-s04/verify/status.txt` | 0 | ✅ pass | 42ms |
| 10 | `rg -n 'vscode-languageclient|vscode-jsonrpc' .tmp/m034-s04/verify/vsix-contents.txt` | 0 | ✅ pass | 96ms |
| 11 | `VERSION=$(node -p "require('./tools/editors/vscode-mesh/package.json').version"); VSIX="tools/editors/vscode-mesh/dist/mesh-lang-${VERSION}.vsix"; test -f "$VSIX" && unzip -l "$VSIX" | rg 'extension/node_modules/(vscode-languageclient|vscode-jsonrpc)' >/dev/null` | 0 | ✅ pass | 382ms |
| 12 | `! rg -n 'mesh-lang-0\.[0-9]+\.vsix|--no-dependencies' tools/editors/vscode-mesh/package.json tools/editors/vscode-mesh/README.md website/docs/docs/tooling/index.md` | 0 | ✅ pass | 40ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `.github/workflows/extension-release-proof.yml`
- `.github/workflows/publish-extension.yml`
- `scripts/verify-m034-s04-workflows.sh`
- `.gsd/DECISIONS.md`
- `.gsd/milestones/M034/slices/S04/tasks/T03-SUMMARY.md`


## Deviations
None.

## Known Issues
None.
