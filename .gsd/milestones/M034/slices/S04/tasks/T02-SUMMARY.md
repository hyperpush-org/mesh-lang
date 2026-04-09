---
id: T02
parent: S04
milestone: M034
provides: []
requires: []
affects: []
key_files: ["scripts/verify-m034-s04-extension.sh", "compiler/meshc/tests/e2e_lsp.rs", ".github/workflows/publish-extension.yml", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M034/slices/S04/tasks/T02-SUMMARY.md"]
key_decisions: ["Record intended and verified VSIX paths repo-root-relative in `.tmp/m034-s04/verify/*-vsix-path.txt` so downstream workflows can consume the same artifact from the repo root.", "Make `compiler/meshc/tests/e2e_lsp.rs` derive hover/definition/signature positions from `jobs_source` text instead of fixed line numbers so backend-file edits do not create false prerequisite regressions."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-level verification passed: `bash -n scripts/verify-m034-s04-extension.sh`, `npm --prefix tools/editors/vscode-mesh run test:vsix-path`, `cargo test -q -p meshc --test e2e_lsp -- --nocapture`, the malformed-tag negative path, the full verifier command, artifact file checks, runtime dependency grep, repo-root VSIX path resolution, observability file-presence check, and single-file YAML parse for `.github/workflows/publish-extension.yml` all succeeded. Slice-level verification is intentionally partial at T02: `bash scripts/verify-m034-s04-workflows.sh` and the two-file YAML parse still fail because T03 has not created `.github/workflows/extension-release-proof.yml` or the workflow verifier yet."
completed_at: 2026-03-27T01:05:08.627Z
blocker_discovered: false
---

# T02: Added the canonical extension prepublish verifier with VSIX audit, prereq drift checks, and reused `e2e_lsp` proof.

> Added the canonical extension prepublish verifier with VSIX audit, prereq drift checks, and reused `e2e_lsp` proof.

## What Happened
---
id: T02
parent: S04
milestone: M034
key_files:
  - scripts/verify-m034-s04-extension.sh
  - compiler/meshc/tests/e2e_lsp.rs
  - .github/workflows/publish-extension.yml
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M034/slices/S04/tasks/T02-SUMMARY.md
key_decisions:
  - Record intended and verified VSIX paths repo-root-relative in `.tmp/m034-s04/verify/*-vsix-path.txt` so downstream workflows can consume the same artifact from the repo root.
  - Make `compiler/meshc/tests/e2e_lsp.rs` derive hover/definition/signature positions from `jobs_source` text instead of fixed line numbers so backend-file edits do not create false prerequisite regressions.
duration: ""
verification_result: mixed
completed_at: 2026-03-27T01:05:08.629Z
blocker_discovered: false
---

# T02: Added the canonical extension prepublish verifier with VSIX audit, prereq drift checks, and reused `e2e_lsp` proof.

**Added the canonical extension prepublish verifier with VSIX audit, prereq drift checks, and reused `e2e_lsp` proof.**

## What Happened

Added `scripts/verify-m034-s04-extension.sh` as the canonical repo-local extension release proof surface. The script now clears and recreates `.tmp/m034-s04/verify/`, derives the version and deterministic VSIX path from the T01 helper, records intended and verified VSIX paths repo-root-relative for downstream workflow reuse, fails fast on `EXPECTED_TAG` / package-script / README / tooling-doc / workflow-comment drift, runs one `npm ci` / `npm run compile` / `npm run package` cycle, audits the shipped VSIX with Python `zipfile` checks for required files plus real `vscode-languageclient` / `vscode-jsonrpc` `.js` runtime entries, and persists prereq/package/audit/LSP logs under `.tmp/m034-s04/verify/`. While wiring the reused prerequisite, I found that `cargo test -q -p meshc --test e2e_lsp -- --nocapture` was red because `compiler/meshc/tests/e2e_lsp.rs` still used hardcoded positions into `reference-backend/api/jobs.mpl`; I fixed that root cause by deriving hover/definition/signature positions from `jobs_source` text at runtime. I also updated the stale `ext-v0.2.0` example in `.github/workflows/publish-extension.yml` to a generic `ext-vX.Y.Z` comment and recorded the quiet-mode `e2e_lsp` output/position-drift gotcha in `.gsd/KNOWLEDGE.md`.

## Verification

Task-level verification passed: `bash -n scripts/verify-m034-s04-extension.sh`, `npm --prefix tools/editors/vscode-mesh run test:vsix-path`, `cargo test -q -p meshc --test e2e_lsp -- --nocapture`, the malformed-tag negative path, the full verifier command, artifact file checks, runtime dependency grep, repo-root VSIX path resolution, observability file-presence check, and single-file YAML parse for `.github/workflows/publish-extension.yml` all succeeded. Slice-level verification is intentionally partial at T02: `bash scripts/verify-m034-s04-workflows.sh` and the two-file YAML parse still fail because T03 has not created `.github/workflows/extension-release-proof.yml` or the workflow verifier yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -n scripts/verify-m034-s04-extension.sh` | 0 | ✅ pass | 74ms |
| 2 | `npm --prefix tools/editors/vscode-mesh run test:vsix-path` | 0 | ✅ pass | 1636ms |
| 3 | `cargo test -q -p meshc --test e2e_lsp -- --nocapture` | 0 | ✅ pass | 12169ms |
| 4 | `EXPECTED_TAG=ext-v0.0.0 bash scripts/verify-m034-s04-extension.sh` | 1 | ✅ pass | 1428ms |
| 5 | `EXPECTED_TAG="ext-v$(node -p \"require('./tools/editors/vscode-mesh/package.json').version\")" bash scripts/verify-m034-s04-extension.sh` | 0 | ✅ pass | 43841ms |
| 6 | `test -f .tmp/m034-s04/verify/verified-vsix-path.txt` | 0 | ✅ pass | 90ms |
| 7 | `test -f .tmp/m034-s04/verify/vsix-contents.txt` | 0 | ✅ pass | 87ms |
| 8 | `rg -n 'vscode-languageclient|vscode-jsonrpc' .tmp/m034-s04/verify/vsix-contents.txt` | 0 | ✅ pass | 118ms |
| 9 | `VSIX="$(cat .tmp/m034-s04/verify/verified-vsix-path.txt)" && test -f "$VSIX"` | 0 | ✅ pass | 82ms |
| 10 | `find .tmp/m034-s04/verify -maxdepth 1 -type f | sort | rg 'verified-vsix-path.txt|vsix-contents.txt|e2e-lsp.log|prereq-sweep.log|zip-audit.log|status.txt|current-phase.txt'` | 0 | ✅ pass | 82ms |
| 11 | `ruby -e 'require "yaml"; YAML.load_file(".github/workflows/publish-extension.yml")'` | 0 | ✅ pass | 394ms |
| 12 | `bash scripts/verify-m034-s04-workflows.sh` | 127 | ❌ fail | 102ms |
| 13 | `ruby -e 'require "yaml"; %w[.github/workflows/extension-release-proof.yml .github/workflows/publish-extension.yml].each { |f| YAML.load_file(f) }'` | 1 | ❌ fail | 446ms |


## Deviations

Touched `compiler/meshc/tests/e2e_lsp.rs` to replace drift-prone hardcoded positions with source-derived coordinates, and updated the stale `ext-v0.2.0` comment in `.github/workflows/publish-extension.yml` to the generic `ext-vX.Y.Z` example so the prereq sweep could stay truthful. These were local execution corrections, not slice replanning.

## Known Issues

T03 workflow assets are still missing, so `bash scripts/verify-m034-s04-workflows.sh` and the two-file YAML parse for `.github/workflows/extension-release-proof.yml` plus `.github/workflows/publish-extension.yml` remain red until T03 lands those files. The publish workflow itself still contains the old fail-open packaging/publication behavior; only the stale trigger comment was corrected here because the actual workflow rewrite belongs to T03.

## Files Created/Modified

- `scripts/verify-m034-s04-extension.sh`
- `compiler/meshc/tests/e2e_lsp.rs`
- `.github/workflows/publish-extension.yml`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M034/slices/S04/tasks/T02-SUMMARY.md`


## Deviations
Touched `compiler/meshc/tests/e2e_lsp.rs` to replace drift-prone hardcoded positions with source-derived coordinates, and updated the stale `ext-v0.2.0` comment in `.github/workflows/publish-extension.yml` to the generic `ext-vX.Y.Z` example so the prereq sweep could stay truthful. These were local execution corrections, not slice replanning.

## Known Issues
T03 workflow assets are still missing, so `bash scripts/verify-m034-s04-workflows.sh` and the two-file YAML parse for `.github/workflows/extension-release-proof.yml` plus `.github/workflows/publish-extension.yml` remain red until T03 lands those files. The publish workflow itself still contains the old fail-open packaging/publication behavior; only the stale trigger comment was corrected here because the actual workflow rewrite belongs to T03.
