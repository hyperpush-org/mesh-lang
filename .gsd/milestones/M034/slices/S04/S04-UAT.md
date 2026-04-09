# S04: Extension release path hardening — UAT

**Milestone:** M034
**Written:** 2026-03-27T01:36:21.962Z

# S04: Extension release path hardening — UAT

**Milestone:** M034  
**Written:** 2026-03-26

# S04 UAT — Extension release path hardening

## Preconditions
- Worktree contains the landed S04 changes.
- Local tooling: `node`, `npm`, `cargo`, `python3`, `ruby`, `unzip`, and `rg` are installed.
- `tools/editors/vscode-mesh/package-lock.json` is present so `npm ci` can recreate the extension dependency tree.
- No manual cleanup is required under `.tmp/m034-s04/`; both verifier scripts recreate their own artifact directories.
- Marketplace publish tokens are **not** required for these local UAT checks; the publish lane is verified through contract checks and exact-artifact handoff rather than by doing a public publish from the workstation.

## Test Case 1 — Deterministic VSIX path helper and local package contract
1. Run `npm --prefix tools/editors/vscode-mesh run test:vsix-path`.
   - **Expected:** all three helper tests pass, including the `--absolute` CLI path case.
2. Run `npm --prefix tools/editors/vscode-mesh run compile`.
   - **Expected:** TypeScript compile succeeds.
3. Run `npm --prefix tools/editors/vscode-mesh run package`.
   - **Expected:** packaging succeeds and reports a VSIX under `tools/editors/vscode-mesh/dist/mesh-lang-<version>.vsix`.
4. Run `node tools/editors/vscode-mesh/scripts/vsix-path.mjs`.
   - **Expected:** stdout is exactly `dist/mesh-lang-<current-version>.vsix`.
5. Run `find tools/editors/vscode-mesh/dist -maxdepth 1 -name '*.vsix' | wc -l` and `find tools/editors/vscode-mesh -maxdepth 1 -name '*.vsix' -print -quit`.
   - **Expected:** the `dist/` count is `1`, and no root-level `*.vsix` file is present.

## Test Case 2 — The shipped VSIX contains real runtime dependencies and no stale guidance
1. Run:
   ```bash
   VERSION=$(node -p "require('./tools/editors/vscode-mesh/package.json').version")
   VSIX="tools/editors/vscode-mesh/dist/mesh-lang-${VERSION}.vsix"
   test -f "$VSIX"
   unzip -l "$VSIX" | rg 'extension/node_modules/(vscode-languageclient|vscode-jsonrpc)'
   ```
   - **Expected:** the archive listing includes entries under both runtime dependency roots.
2. Run:
   ```bash
   VERSION=$(node -p "require('./tools/editors/vscode-mesh/package.json').version")
   VSIX="tools/editors/vscode-mesh/dist/mesh-lang-${VERSION}.vsix"
   ! unzip -l "$VSIX" | rg 'extension/(scripts/|dist/|src/)|\.test\.'
   ```
   - **Expected:** packaging does not ship source files, helper scripts, nested `dist/`, or test files.
3. Run `! rg -n 'mesh-lang-0\.[0-9]+\.vsix|--no-dependencies' tools/editors/vscode-mesh/package.json tools/editors/vscode-mesh/README.md website/docs/docs/tooling/index.md`.
   - **Expected:** no hardcoded versioned VSIX filename or `--no-dependencies` packaging guidance remains.

## Test Case 3 — Canonical extension verifier proves prereqs, archive truth, and shared LSP truth
1. Run:
   ```bash
   EXPECTED_TAG="ext-v$(node -p "require('./tools/editors/vscode-mesh/package.json').version")" \
     bash scripts/verify-m034-s04-extension.sh
   ```
   - **Expected:** the script ends with `verify-m034-s04-extension: ok`.
2. Inspect `.tmp/m034-s04/verify/verified-vsix-path.txt` and `.tmp/m034-s04/verify/intended-vsix-path.txt`.
   - **Expected:** both point at the deterministic repo-relative VSIX path under `tools/editors/vscode-mesh/dist/`.
3. Inspect `.tmp/m034-s04/verify/prereq-sweep.log`.
   - **Expected:** it records the current package version, `ext-v<version>`, the expected `dist/mesh-lang-<version>.vsix` path, and successful checks for README/tooling-doc guidance plus the `ext-v*` publish trigger.
4. Inspect `.tmp/m034-s04/verify/zip-audit.log` and `.tmp/m034-s04/verify/vsix-contents.txt`.
   - **Expected:** the log reports runtime entries and shipped `.js` counts for both `vscode-languageclient` and `vscode-jsonrpc`, and `vsix-contents.txt` is a concrete manifest of the exact verified artifact.
5. Inspect `.tmp/m034-s04/verify/e2e-lsp.log`.
   - **Expected:** stdout includes `running 1 test` and `test result: ok.`.
6. Inspect `.tmp/m034-s04/verify/status.txt` and `.tmp/m034-s04/verify/current-phase.txt`.
   - **Expected:** `status.txt` contains `ok` and `current-phase.txt` contains `complete`.

## Test Case 4 — Workflow contract stays thin, reusable-owned, and fail-closed
1. Run `bash scripts/verify-m034-s04-workflows.sh`.
   - **Expected:** exits 0 and reports `verify-m034-s04-workflows: ok (all)`.
2. Run `ruby -e 'require "yaml"; %w[.github/workflows/extension-release-proof.yml .github/workflows/publish-extension.yml].each { |f| YAML.load_file(f) }'`.
   - **Expected:** both workflow files parse as YAML.
3. Run `! rg -n 'continue-on-error|ls \*\.vsix|bash scripts/verify-m034-s04-extension.sh' .github/workflows/publish-extension.yml`.
   - **Expected:** the publish workflow does not repackage, glob, or directly rerun the verifier.
4. Run `rg -n './.github/workflows/extension-release-proof.yml|extensionFile:' .github/workflows/publish-extension.yml`.
   - **Expected:** the publish workflow calls the reusable proof exactly once and both publish actions use the proof job's `extensionFile` output.
5. Inspect `.tmp/m034-s04/workflows/phase-report.txt`.
   - **Expected:** it contains `reusable`, `publish`, and `full-contract` phases marked `passed`.

## Test Case 5 — Negative prereq drift is rejected before publish
1. Run `EXPECTED_TAG=ext-v0.0.0 bash scripts/verify-m034-s04-extension.sh`.
   - **Expected:** exit is non-zero.
2. Inspect `.tmp/m034-s04/verify/failed-phase.txt` and `.tmp/m034-s04/verify/prereq-sweep.log` after the failing run.
   - **Expected:** the failure is attributed to `prereq-sweep`, and the log reports `EXPECTED_TAG` drift instead of allowing packaging/publication to continue.

## Edge-case follow-up checks
- If `bash scripts/verify-m034-s04-extension.sh` fails in `zip-audit`, inspect `.vscodeignore`, `tools/editors/vscode-mesh/package.json`, and the produced VSIX manifest before touching workflow YAML; S04's ownership boundary is that packaging truth lives in the repo-local verifier and helper, not in the publish job.
- If `bash scripts/verify-m034-s04-workflows.sh` fails, use `.tmp/m034-s04/workflows/reusable.log`, `publish.log`, or `full-contract.log` to see whether the drift is trigger/permission/output wiring, missing artifact handoff, or fail-open publication logic.
- After the next push/tag, capture the first live `extension-release-proof` and `publish-extension` runner results as supporting evidence that the locally verified contract also survives on hosted runners with real publish credentials.
