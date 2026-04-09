---
id: T01
parent: S04
milestone: M034
provides: []
requires: []
affects: []
key_files: ["tools/editors/vscode-mesh/scripts/vsix-path.mjs", "tools/editors/vscode-mesh/scripts/vsix-path.test.mjs", "tools/editors/vscode-mesh/package.json", "tools/editors/vscode-mesh/.vscodeignore", "tools/editors/vscode-mesh/README.md", "website/docs/docs/tooling/index.md", ".gitignore", "tools/editors/vscode-mesh/mesh-lang-0.2.0.vsix", "tools/editors/vscode-mesh/mesh-lang-0.3.0.vsix", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Centralized VSIX path resolution and package/install orchestration in `tools/editors/vscode-mesh/scripts/vsix-path.mjs` so humans, scripts, and later workflow gates all target the same `dist/mesh-lang-<version>.vsix` artifact.", "Treat VSIX packaging as a contract verified by archive contents and deterministic artifact location, not by `vsce package` exit status alone."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-level verification passed: `npm --prefix tools/editors/vscode-mesh run test:vsix-path`, `npm --prefix tools/editors/vscode-mesh run compile`, and `npm --prefix tools/editors/vscode-mesh run package` all succeeded; the packaged `tools/editors/vscode-mesh/dist/mesh-lang-0.3.0.vsix` contains `extension/node_modules/vscode-languageclient/...` and `extension/node_modules/vscode-jsonrpc/...`; the package surface check confirmed `src/`, `scripts/`, `dist/`, and `*.test.*` files stayed out of the shipped VSIX; and the grep gate confirmed the extension package/docs no longer contain `mesh-lang-0.x.y.vsix` or `--no-dependencies`. Slice-level verification is partial at T01: the two `scripts/verify-m034-s04-*.sh` commands and the workflow YAML load check still fail because those later-task files do not exist yet."
completed_at: 2026-03-27T00:42:57.795Z
blocker_discovered: false
---

# T01: Made VSIX packaging deterministic under dist/, restored runtime dependencies to the shipped extension, and removed hardcoded local-install VSIX drift.

> Made VSIX packaging deterministic under dist/, restored runtime dependencies to the shipped extension, and removed hardcoded local-install VSIX drift.

## What Happened
---
id: T01
parent: S04
milestone: M034
key_files:
  - tools/editors/vscode-mesh/scripts/vsix-path.mjs
  - tools/editors/vscode-mesh/scripts/vsix-path.test.mjs
  - tools/editors/vscode-mesh/package.json
  - tools/editors/vscode-mesh/.vscodeignore
  - tools/editors/vscode-mesh/README.md
  - website/docs/docs/tooling/index.md
  - .gitignore
  - tools/editors/vscode-mesh/mesh-lang-0.2.0.vsix
  - tools/editors/vscode-mesh/mesh-lang-0.3.0.vsix
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Centralized VSIX path resolution and package/install orchestration in `tools/editors/vscode-mesh/scripts/vsix-path.mjs` so humans, scripts, and later workflow gates all target the same `dist/mesh-lang-<version>.vsix` artifact.
  - Treat VSIX packaging as a contract verified by archive contents and deterministic artifact location, not by `vsce package` exit status alone.
duration: ""
verification_result: mixed
completed_at: 2026-03-27T00:42:57.796Z
blocker_discovered: false
---

# T01: Made VSIX packaging deterministic under dist/, restored runtime dependencies to the shipped extension, and removed hardcoded local-install VSIX drift.

**Made VSIX packaging deterministic under dist/, restored runtime dependencies to the shipped extension, and removed hardcoded local-install VSIX drift.**

## What Happened

Reproduced the broken extension packaging contract, confirmed that `vsce package --no-dependencies` plus `.vscodeignore`'s `node_modules/**` exclusion produced a green package command but a broken VSIX, and then repaired the packaging path before touching CI. Added `tools/editors/vscode-mesh/scripts/vsix-path.mjs` as the single source of truth for `dist/mesh-lang-<version>.vsix`, with `path`, `package`, and `install-local` modes plus stale `dist/*.vsix` cleanup. Added a no-dependency Node built-in test file for the helper, which caught and drove the fix for an import-side-effect bug and the later `--absolute` CLI parsing ergonomics. Updated `tools/editors/vscode-mesh/package.json` to route package/install-local through the helper and drop `--no-dependencies`, rewrote `.vscodeignore` so runtime `vscode-languageclient` / `vscode-jsonrpc` content ships while source/tests/maps/dev-only clutter stay out, removed the stale checked-in root VSIX artifacts, updated `.gitignore` to ignore future extension VSIXs and the real extension `node_modules/` path explicitly, and rewrote the extension README plus website tooling docs to use `npm run package`, the deterministic `dist/mesh-lang-<version>.vsix` output, and `npm run install-local` instead of hardcoded `mesh-lang-0.3.0.vsix` guidance.

## Verification

Task-level verification passed: `npm --prefix tools/editors/vscode-mesh run test:vsix-path`, `npm --prefix tools/editors/vscode-mesh run compile`, and `npm --prefix tools/editors/vscode-mesh run package` all succeeded; the packaged `tools/editors/vscode-mesh/dist/mesh-lang-0.3.0.vsix` contains `extension/node_modules/vscode-languageclient/...` and `extension/node_modules/vscode-jsonrpc/...`; the package surface check confirmed `src/`, `scripts/`, `dist/`, and `*.test.*` files stayed out of the shipped VSIX; and the grep gate confirmed the extension package/docs no longer contain `mesh-lang-0.x.y.vsix` or `--no-dependencies`. Slice-level verification is partial at T01: the two `scripts/verify-m034-s04-*.sh` commands and the workflow YAML load check still fail because those later-task files do not exist yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix tools/editors/vscode-mesh run test:vsix-path` | 0 | ✅ pass | 762ms |
| 2 | `npm --prefix tools/editors/vscode-mesh run compile` | 0 | ✅ pass | 1794ms |
| 3 | `npm --prefix tools/editors/vscode-mesh run package` | 0 | ✅ pass | 7477ms |
| 4 | `VERSION=$(node -p "require('./tools/editors/vscode-mesh/package.json').version"); VSIX="tools/editors/vscode-mesh/dist/mesh-lang-${VERSION}.vsix"; test -f "$VSIX" && unzip -l "$VSIX" | rg 'extension/node_modules/(vscode-languageclient|vscode-jsonrpc)' >/dev/null` | 0 | ✅ pass | 201ms |
| 5 | `! rg -n 'mesh-lang-0\.[0-9]+\.vsix|--no-dependencies' tools/editors/vscode-mesh/package.json tools/editors/vscode-mesh/README.md website/docs/docs/tooling/index.md` | 0 | ✅ pass | 34ms |
| 6 | `helper_path=$(node tools/editors/vscode-mesh/scripts/vsix-path.mjs); expected="dist/mesh-lang-$(node -p "require('./tools/editors/vscode-mesh/package.json').version").vsix"; test "$helper_path" = "$expected" && test "$(find tools/editors/vscode-mesh/dist -maxdepth 1 -name '*.vsix' | wc -l | tr -d ' ')" = "1" && test -z "$(find tools/editors/vscode-mesh -maxdepth 1 -name '*.vsix' -print -quit)" && rg -n 'tools/editors/vscode-mesh/(node_modules|\*\.vsix)' .gitignore >/dev/null` | 0 | ✅ pass | 460ms |
| 7 | `VSIX="$(node tools/editors/vscode-mesh/scripts/vsix-path.mjs --absolute)"; ! unzip -l "$VSIX" | rg "extension/(scripts/|dist/|src/)|\.test\."` | 0 | ✅ pass | 265ms |
| 8 | `EXPECTED_TAG="ext-v$(node -p "require('./tools/editors/vscode-mesh/package.json').version")" bash scripts/verify-m034-s04-extension.sh` | 127 | ❌ fail | 244ms |
| 9 | `bash scripts/verify-m034-s04-workflows.sh` | 127 | ❌ fail | 40ms |
| 10 | `ruby -e 'require "yaml"; %w[.github/workflows/extension-release-proof.yml .github/workflows/publish-extension.yml].each { |f| YAML.load_file(f) }'` | 1 | ❌ fail | 278ms |


## Deviations

Added a lightweight Node built-in test (`tools/editors/vscode-mesh/scripts/vsix-path.test.mjs`) and `npm run test:vsix-path` even though the original expected-output list did not mention a test file. The extension had no existing JS test harness, and the new helper introduced path/CLI behavior that needed a mechanical proof surface.

## Known Issues

The slice-level verification scripts and workflow files referenced by the slice plan do not exist yet, so `scripts/verify-m034-s04-extension.sh`, `scripts/verify-m034-s04-workflows.sh`, and the YAML-load check for `.github/workflows/extension-release-proof.yml` / `.github/workflows/publish-extension.yml` remain red until later tasks land those files. `vsce package` also still emits pre-existing warnings about the missing LICENSE file and lack of bundling, but those warnings did not block this task’s packaging-contract repair.

## Files Created/Modified

- `tools/editors/vscode-mesh/scripts/vsix-path.mjs`
- `tools/editors/vscode-mesh/scripts/vsix-path.test.mjs`
- `tools/editors/vscode-mesh/package.json`
- `tools/editors/vscode-mesh/.vscodeignore`
- `tools/editors/vscode-mesh/README.md`
- `website/docs/docs/tooling/index.md`
- `.gitignore`
- `tools/editors/vscode-mesh/mesh-lang-0.2.0.vsix`
- `tools/editors/vscode-mesh/mesh-lang-0.3.0.vsix`
- `.gsd/KNOWLEDGE.md`


## Deviations
Added a lightweight Node built-in test (`tools/editors/vscode-mesh/scripts/vsix-path.test.mjs`) and `npm run test:vsix-path` even though the original expected-output list did not mention a test file. The extension had no existing JS test harness, and the new helper introduced path/CLI behavior that needed a mechanical proof surface.

## Known Issues
The slice-level verification scripts and workflow files referenced by the slice plan do not exist yet, so `scripts/verify-m034-s04-extension.sh`, `scripts/verify-m034-s04-workflows.sh`, and the YAML-load check for `.github/workflows/extension-release-proof.yml` / `.github/workflows/publish-extension.yml` remain red until later tasks land those files. `vsce package` also still emits pre-existing warnings about the missing LICENSE file and lack of bundling, but those warnings did not block this task’s packaging-contract repair.
