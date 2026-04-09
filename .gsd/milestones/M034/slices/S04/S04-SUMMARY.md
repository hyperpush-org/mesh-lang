---
id: S04
parent: M034
milestone: M034
provides:
  - A deterministic extension packaging contract centered on `tools/editors/vscode-mesh/dist/mesh-lang-<version>.vsix`, with real runtime dependency shipping and no stale root-level VSIX shadow path.
  - A canonical repo-local extension prepublish verifier (`scripts/verify-m034-s04-extension.sh`) that packages once, audits the VSIX, checks tag/docs/package drift, reruns the shared `e2e_lsp` prerequisite, and records the exact verified artifact for downstream reuse.
  - A reusable GitHub Actions proof workflow and a thin fail-closed publish workflow that publish only the exact verified VSIX to both extension registries.
  - Version-agnostic extension/docs guidance that now points contributors and downstream release work at `npm run package`, `npm run install-local`, and the deterministic `dist/mesh-lang-<version>.vsix` path.
requires:
  - slice: S02
    provides: The reusable-workflow and local workflow-contract verification pattern that S04 extends from authoritative live package proof to the extension publish lane.
affects:
  - S05
key_files:
  - tools/editors/vscode-mesh/scripts/vsix-path.mjs
  - tools/editors/vscode-mesh/scripts/vsix-path.test.mjs
  - tools/editors/vscode-mesh/package.json
  - tools/editors/vscode-mesh/.vscodeignore
  - tools/editors/vscode-mesh/README.md
  - website/docs/docs/tooling/index.md
  - scripts/verify-m034-s04-extension.sh
  - compiler/meshc/tests/e2e_lsp.rs
  - .github/workflows/extension-release-proof.yml
  - .github/workflows/publish-extension.yml
  - scripts/verify-m034-s04-workflows.sh
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - Centralize VSIX path resolution and local package/install orchestration in `tools/editors/vscode-mesh/scripts/vsix-path.mjs` so every actor targets the same `dist/mesh-lang-<version>.vsix` artifact.
  - Record intended and verified VSIX paths as repo-root-relative files under `.tmp/m034-s04/verify/` so workflows and local verification consume the same exact artifact without globs.
  - Make `compiler/meshc/tests/e2e_lsp.rs` derive hover/definition/signature-help positions from `jobs_source` text instead of fixed line numbers so unrelated backend edits do not create false release-gate regressions.
  - Make `.github/workflows/extension-release-proof.yml` the only workflow that runs `scripts/verify-m034-s04-extension.sh`, and require `.github/workflows/publish-extension.yml` to publish only the proof job's downloaded verified VSIX to both registries.
patterns_established:
  - Keep release truth in repo-local verifier scripts and make GitHub Actions thin callers that transport outputs and diagnostics instead of reimplementing proof logic in YAML.
  - Use deterministic artifact paths plus recorded path files as the handoff contract; do not let local commands or workflow jobs rediscover release artifacts through hardcoded versions or `*.vsix` globs.
  - When a shared prerequisite test depends on live repo source, derive positions and lookup points from the source text itself rather than fixed line numbers so delivery gates stay stable as the backend evolves.
  - Pair workflow-contract verifiers with phase-scoped local logs (`phase-report.txt`, `reusable.log`, `publish.log`, `full-contract.log`) so YAML drift fails locally before CI.
observability_surfaces:
  - `.tmp/m034-s04/verify/current-phase.txt`, `status.txt`, `intended-vsix-path.txt`, `verified-vsix-path.txt`, `vsix-contents.txt`, `prereq-sweep.log`, `zip-audit.log`, and `e2e-lsp.log` for extension proof phases and artifact truth.
  - `.tmp/m034-s04/workflows/phase-report.txt` plus `reusable.log`, `publish.log`, and `full-contract.log` for local workflow-contract drift diagnosis.
  - GitHub Actions failure artifact upload `extension-release-proof-diagnostics`, which retains `.tmp/m034-s04/verify/**` when the reusable proof job fails.
drill_down_paths:
  - .gsd/milestones/M034/slices/S04/tasks/T01-SUMMARY.md
  - .gsd/milestones/M034/slices/S04/tasks/T02-SUMMARY.md
  - .gsd/milestones/M034/slices/S04/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-27T01:36:21.960Z
blocker_discovered: false
---

# S04: Extension release path hardening

**S04 turned the VS Code extension release lane into a fail-closed proof flow: the repo now builds one deterministic VSIX with real runtime dependencies, verifies tag/docs/package drift plus shared LSP truth locally, and only lets publication proceed with the exact verified artifact through a reusable workflow handoff.**

## What Happened

Closed the extension release slice by moving the VS Code lane from artifact folklore to an explicit proof contract. On the packaging side, `tools/editors/vscode-mesh/scripts/vsix-path.mjs` is now the single owner of the local artifact path, so humans, scripts, and workflows all target `tools/editors/vscode-mesh/dist/mesh-lang-<version>.vsix` instead of a hardcoded `mesh-lang-0.3.0.vsix` or a glob. `npm run package` now packages through that helper, `npm run install-local` installs the same helper-resolved artifact, `.vscodeignore` keeps the real `vscode-languageclient` / `vscode-jsonrpc` runtime tree in the shipped VSIX while still excluding dev-only clutter, stale checked-in root-level VSIX outputs are gone, and the extension README plus website tooling docs now describe the version-agnostic local install path instead of pinning an old filename.

The slice then froze that packaging contract behind one repo-local proof surface: `scripts/verify-m034-s04-extension.sh`. That verifier recreates `.tmp/m034-s04/verify/`, derives the extension version plus deterministic VSIX path, records both intended and verified VSIX paths repo-root-relative for downstream reuse, fails fast on `EXPECTED_TAG` / package-script / docs / workflow-comment drift, runs one `npm ci` + compile + package cycle, audits the packaged archive with Python `zipfile`, and persists phase logs and manifests for postmortems. To keep the reused upstream prerequisite honest, S04 also repaired `compiler/meshc/tests/e2e_lsp.rs` so hover/definition/signature-help positions are derived from `reference-backend/api/jobs.mpl` source text instead of fixed line numbers; that removed a real false-regression mode while preserving the same stdio JSON-RPC proof the extension lane depends on.

Finally, S04 applied the S02 pattern to the extension publish lane itself. `.github/workflows/extension-release-proof.yml` is now the only workflow allowed to run `bash scripts/verify-m034-s04-extension.sh`; it sets up Node/LLVM/Rust, captures the exact verified VSIX path, uploads that file as `extension-release-vsix`, and retains `.tmp/m034-s04/verify/**` diagnostics on failure. `.github/workflows/publish-extension.yml` is now a thin tag-triggered caller that depends on the reusable proof, downloads the verified artifact back into the deterministic `dist/` path, checks the handoff, and publishes the same `needs.proof.outputs.verified_vsix_path` to both Open VSX and Visual Studio Marketplace with no `continue-on-error`, no `ls *.vsix`, and no inline repackaging or proof logic. `scripts/verify-m034-s04-workflows.sh` mechanically enforces that contract locally and writes its own phase report/logs under `.tmp/m034-s04/workflows/`.

Operational readiness: health signal = green `EXPECTED_TAG="ext-v$(node -p "require('./tools/editors/vscode-mesh/package.json').version")" bash scripts/verify-m034-s04-extension.sh`, green `bash scripts/verify-m034-s04-workflows.sh`, `.tmp/m034-s04/verify/status.txt` containing `ok`, `.tmp/m034-s04/workflows/phase-report.txt` showing `reusable`, `publish`, and `full-contract` as `passed`, and after push/tag, green `extension-release-proof` plus `publish-extension` runs on GitHub Actions; failure signal = phase-local logs under `.tmp/m034-s04/verify/` or `.tmp/m034-s04/workflows/`, a mismatched `verified-vsix-path.txt`, a missing runtime dependency in `vsix-contents.txt`, or a red `extension-release-proof-diagnostics` artifact in CI; recovery procedure = rerun the repo-local verifier and workflow-contract script, inspect the first failing phase (`prereq-sweep`, `zip-audit`, `e2e-lsp`, `reusable`, or `publish`), repair the drift at the single owning surface (`vsix-path.mjs`, `.vscodeignore`, the extension verifier, or the workflow files), then rerun the same proof commands before allowing publication; monitoring gap = the first live remote tag-triggered proof/publish evidence is still pending the next push/tag, and local closeout did not perform real registry publication because marketplace tokens are not used in repo-local verification.

## Verification

Passed the full slice-plan verification bundle and the closeout observability checks:
- `npm --prefix tools/editors/vscode-mesh run test:vsix-path`
- `npm --prefix tools/editors/vscode-mesh run compile`
- `npm --prefix tools/editors/vscode-mesh run package`
- `VERSION=$(node -p "require('./tools/editors/vscode-mesh/package.json').version"); VSIX="tools/editors/vscode-mesh/dist/mesh-lang-${VERSION}.vsix"; test -f "$VSIX" && unzip -l "$VSIX" | rg 'extension/node_modules/(vscode-languageclient|vscode-jsonrpc)'`
- `! rg -n 'mesh-lang-0\.[0-9]+\.vsix|--no-dependencies' tools/editors/vscode-mesh/package.json tools/editors/vscode-mesh/README.md website/docs/docs/tooling/index.md`
- `EXPECTED_TAG="ext-v$(node -p "require('./tools/editors/vscode-mesh/package.json').version")" bash scripts/verify-m034-s04-extension.sh`
- `test -f .tmp/m034-s04/verify/verified-vsix-path.txt`
- `test -f .tmp/m034-s04/verify/vsix-contents.txt`
- `rg -n 'vscode-languageclient|vscode-jsonrpc' .tmp/m034-s04/verify/vsix-contents.txt`
- `bash scripts/verify-m034-s04-workflows.sh`
- `ruby -e 'require "yaml"; %w[.github/workflows/extension-release-proof.yml .github/workflows/publish-extension.yml].each { |f| YAML.load_file(f) }'`
- `! rg -n 'continue-on-error|ls \*\.vsix|bash scripts/verify-m034-s04-extension.sh' .github/workflows/publish-extension.yml`
- `rg -n './.github/workflows/extension-release-proof.yml|extensionFile:' .github/workflows/publish-extension.yml`
- `test -f .tmp/m034-s04/workflows/phase-report.txt && rg -n 'reusable\tpassed|publish\tpassed|full-contract\tpassed' .tmp/m034-s04/workflows/phase-report.txt`

Key closeout evidence: `.tmp/m034-s04/verify/verified-vsix-path.txt` points at `tools/editors/vscode-mesh/dist/mesh-lang-0.3.0.vsix`; `prereq-sweep.log` confirmed `ext-v0.3.0`, docs, and workflow-comment alignment; `zip-audit.log` reported 130 `vscode-languageclient` entries with 61 shipped `.js` files plus 48 `vscode-jsonrpc` entries with 21 shipped `.js` files; and `.tmp/m034-s04/verify/e2e-lsp.log` showed `running 1 test` and `test result: ok.` for the shared LSP prerequisite.

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

No plan-invalidating deviations. The only execution-time shape changes were deliberate hardening moves inside the slice scope: T01 added `tools/editors/vscode-mesh/scripts/vsix-path.test.mjs` to mechanically prove the new VSIX-path helper, and T02 updated `compiler/meshc/tests/e2e_lsp.rs` to derive source positions dynamically so the planned reused prerequisite stopped false-failing on unrelated backend-file edits.

## Known Limitations

The reusable proof workflow and thin publish workflow are verified locally through repo-local verifiers, YAML parsing, and artifact-handoff checks, but the first live GitHub Actions tag run and first real marketplace publications using the exact-artifact handoff are still pending remote execution with real publish tokens. S04 proves the lane is fail-closed and deterministic; it does not yet provide hosted-run evidence for that lane.

## Follow-ups

1. Capture the first green remote `extension-release-proof` and `publish-extension` runs after the workflow files land on the remote branch and a real `ext-v*` tag is exercised.
2. In S05, compose S01 live registry proof, S02 workflow truth, S03 installer smoke, and S04 extension proof into one release-candidate acceptance sweep.
3. If extension dependencies or packaging filters change later, update `.vscodeignore` and `scripts/verify-m034-s04-extension.sh` together so runtime-JS inclusion and archive-audit expectations stay aligned.

## Files Created/Modified

- `tools/editors/vscode-mesh/scripts/vsix-path.mjs` — Added the single source of truth for deterministic VSIX path resolution plus package/install-local orchestration and stale `dist/*.vsix` cleanup.
- `tools/editors/vscode-mesh/scripts/vsix-path.test.mjs` — Added a Node built-in test surface that mechanically proves the helper's relative and `--absolute` path behavior.
- `tools/editors/vscode-mesh/package.json` — Routed package/install-local through the helper and removed the old hardcoded/stale VSIX assumptions from the extension scripts.
- `tools/editors/vscode-mesh/.vscodeignore` — Adjusted the packaging filter so the shipped VSIX keeps the `vscode-languageclient` / `vscode-jsonrpc` runtime tree while excluding dev-only sources and artifacts.
- `tools/editors/vscode-mesh/README.md` — Rewrote local install guidance around `npm run package`, `npm run install-local`, and `dist/mesh-lang-<version>.vsix` instead of a pinned historical VSIX filename.
- `website/docs/docs/tooling/index.md` — Updated the public tooling docs to describe the same deterministic extension packaging/install-local contract used by the repo-local verifier.
- `.gitignore` — Ignored future extension root VSIX artifacts and the real extension `node_modules/` path so stale local packaging outputs cannot shadow fresh proof artifacts.
- `scripts/verify-m034-s04-extension.sh` — Added the canonical extension prepublish verifier that owns tag/docs/package drift checks, one compile/package cycle, VSIX archive audit, shared `e2e_lsp` replay, and proof diagnostics under `.tmp/m034-s04/verify/`.
- `compiler/meshc/tests/e2e_lsp.rs` — Replaced drift-prone hardcoded backend positions with source-derived coordinates so the shared LSP prerequisite remains stable as `reference-backend/api/jobs.mpl` evolves.
- `.github/workflows/extension-release-proof.yml` — Added the reusable proof workflow that is solely responsible for running the extension verifier, capturing the exact VSIX outputs, uploading the verified artifact, and retaining failure diagnostics.
- `.github/workflows/publish-extension.yml` — Rewrote the tag-triggered publish lane into a thin caller that depends on reusable proof, downloads the exact verified VSIX, and publishes that same artifact to both registries without fail-open shortcuts.
- `scripts/verify-m034-s04-workflows.sh` — Added the local workflow-contract verifier that mechanically rejects reusable-owner drift, missing handoff outputs/artifacts, fail-open publication, and globbing/inline proof logic.
- `.gsd/DECISIONS.md` — Recorded the S04 release-verification decisions for exact VSIX handoff files and source-derived `e2e_lsp` lookup positions.
- `.gsd/KNOWLEDGE.md` — Captured the non-obvious S04 gotchas around runtime dependency shipping, `e2e_lsp` quiet-mode proof semantics, and the authoritative local workflow-contract gate.
- `.gsd/PROJECT.md` — Refreshed current project state to reflect the completed S04 extension release proof and the remaining M034 trust gap around first live remote workflow evidence plus S05 assembly.
