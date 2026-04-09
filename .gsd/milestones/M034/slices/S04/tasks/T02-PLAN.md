---
estimated_steps: 4
estimated_files: 5
skills_used:
  - debug-like-expert
  - test
---

# T02: Add the canonical extension prepublish verifier

**Slice:** S04 — Extension release path hardening
**Milestone:** M034

## Description

Once packaging is truthful, freeze it behind one repo-local proof surface that CI and S05 can call unchanged. This task creates the extension verifier that packages the deterministic VSIX, audits the archive contents, checks release prerequisites, and reuses the already-existing `meshc lsp` transport proof so the publish lane inherits real language-server truth without pretending S04 proves full editor parity.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `npm ci` / `vsce package` | Stop at the failing phase and keep compile/package output plus the intended VSIX path under `.tmp/m034-s04/verify/`; never continue with an old artifact. | Abort the verifier and record which phase stalled. | Treat a VSIX missing required files or runtime dependency entries as proof failure. |
| `cargo test -q -p meshc --test e2e_lsp -- --nocapture` | Fail the prepublish gate and keep the named `e2e_lsp` output visible; do not replace it with VS Code-specific guesses. | Treat the timeout as verifier failure and report the stalled prerequisite phase. | Treat missing JSON-RPC proof output or a failing test filter as a broken upstream contract. |
| Version/tag/docs prereq sweep | Reject drift immediately when the expected tag, package version, workflow comment, or local-install docs disagree. | N/A | Treat malformed tag strings, missing docs/install commands, or an unreadable `package.json` version as proof failure. |

## Load Profile

- **Shared resources**: the extension `node_modules/` tree, Cargo build/test caches, and `.tmp/m034-s04/verify/` diagnostics.
- **Per-operation cost**: one clean extension dependency install, one compile/package cycle, one VSIX archive scan, and one targeted `e2e_lsp` cargo test run.
- **10x breakpoint**: repeated verifier runs would spend most time in Node/Cargo setup, so the script should package once, audit that one VSIX, and reuse the same diagnostics root for all phases.

## Negative Tests

- **Malformed inputs**: mismatched `EXPECTED_TAG`, stale doc/install filename, missing icon/readme/changelog entries, or a VSIX with only `.d.ts` files from runtime dependencies.
- **Error paths**: `npm ci` failure, `vsce package` success with a broken archive, or `e2e_lsp` regressions must each stop the proof at a named phase.
- **Boundary conditions**: the verifier must reject both package-level drift (wrong version/file path) and content-level drift (missing runtime JS) even when `out/extension.js` exists.

## Steps

1. Create `scripts/verify-m034-s04-extension.sh` as the single extension release proof entrypoint; it should prepare `.tmp/m034-s04/verify/`, derive the extension version, and call the deterministic packaging path from T01.
2. Make the verifier run `npm ci`, `npm run compile`, and `npm run package` in `tools/editors/vscode-mesh/`, then inspect the generated VSIX with Python `zipfile` checks for required shipped files and runtime dependency entries.
3. Add prereq checks for `EXPECTED_TAG` / `ext-vX.Y.Z`, current package version, `install-local`, extension README, tooling docs, and any workflow-facing version examples so release-lane drift fails before publish.
4. Rerun `cargo test -q -p meshc --test e2e_lsp -- --nocapture` inside the verifier and write the exact verified VSIX path plus archive manifest into `.tmp/m034-s04/verify/` for downstream workflow reuse and postmortems.

## Must-Haves

- [ ] `scripts/verify-m034-s04-extension.sh` owns compile/package/audit/prereq checks for the extension lane
- [ ] The verifier leaves phase-scoped diagnostics plus the exact verified VSIX path under `.tmp/m034-s04/verify/`
- [ ] Tag/package/docs/install-local drift fails before publish
- [ ] The existing `e2e_lsp` proof is rerun as an upstream prerequisite without adding new editor-parity claims

## Verification

- `EXPECTED_TAG="ext-v$(node -p "require('./tools/editors/vscode-mesh/package.json').version")" bash scripts/verify-m034-s04-extension.sh`
- `test -f .tmp/m034-s04/verify/verified-vsix-path.txt`
- `test -f .tmp/m034-s04/verify/vsix-contents.txt`
- `rg -n 'vscode-languageclient|vscode-jsonrpc' .tmp/m034-s04/verify/vsix-contents.txt`

## Observability Impact

- Signals added/changed: named verifier phases plus persisted VSIX path/content manifests and `e2e_lsp` output under `.tmp/m034-s04/verify/`.
- How a future agent inspects this: rerun `bash scripts/verify-m034-s04-extension.sh` or open the files in `.tmp/m034-s04/verify/`.
- Failure state exposed: the exact failing phase (`npm ci`, compile, package, zip audit, docs/tag drift, or `e2e_lsp`) and the artifact/path involved.

## Inputs

- `tools/editors/vscode-mesh/package.json`
- `tools/editors/vscode-mesh/.vscodeignore`
- `tools/editors/vscode-mesh/scripts/vsix-path.mjs`
- `tools/editors/vscode-mesh/README.md`
- `website/docs/docs/tooling/index.md`
- `compiler/meshc/tests/e2e_lsp.rs`

## Expected Output

- `scripts/verify-m034-s04-extension.sh`
- `tools/editors/vscode-mesh/package.json`
- `.tmp/m034-s04/verify/verified-vsix-path.txt`
- `.tmp/m034-s04/verify/vsix-contents.txt`
- `.tmp/m034-s04/verify/e2e-lsp.log`
