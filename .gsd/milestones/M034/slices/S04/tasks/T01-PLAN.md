---
estimated_steps: 4
estimated_files: 8
skills_used:
  - test
---

# T01: Repair the VSIX package contract and deterministic local install path

**Slice:** S04 — Extension release path hardening
**Milestone:** M034

## Description

Fix the broken packaging contract before touching CI. The extension currently packages from a stale root, excludes the runtime JS tree behind `vscode-languageclient/node`, and teaches humans plus scripts to install a hardcoded `mesh-lang-0.3.0.vsix`. This task makes the VSIX path deterministic, keeps runtime dependencies in the package, and removes historical build-artifact/documentation drift that would otherwise let later verification pass against the wrong file.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `vsce` packaging / npm scripts | Stop immediately and leave the failing package command plus output path visible; do not fall back to a previous checked-in VSIX. | Treat packaging stalls as failure and keep the last attempted VSIX path in logs. | Treat a VSIX missing runtime dependency entries or missing `out/extension.js` as contract failure. |
| `.vscodeignore` filter rules | Refuse the task if runtime `node_modules` JS stays excluded, even if `vsce package` exits 0. | N/A | Treat over-broad excludes or declaration-only re-includes as a broken release contract. |
| Local install docs/scripts | Fail if `package.json`, `README.md`, or tooling docs still hardcode `mesh-lang-0.3.0.vsix` or another version-pinned filename. | N/A | Treat commands that can resolve multiple VSIXs or drift from the deterministic dist path as release-prereq failure. |

## Load Profile

- **Shared resources**: `tools/editors/vscode-mesh/node_modules/`, the new deterministic `tools/editors/vscode-mesh/dist/` packaging root, and checked-in extension artifacts that must stop shadowing fresh outputs.
- **Per-operation cost**: one extension compile plus one VSIX package build and one archive listing check.
- **10x breakpoint**: repeated packaging would first waste time on extension dependency install/build; the deterministic output path should let later tasks reuse one freshly produced VSIX instead of repackaging via globs.

## Negative Tests

- **Malformed inputs**: multiple historical `*.vsix` files in the extension directory, hardcoded stale version strings, or a `.vscodeignore` rule that still strips runtime JS.
- **Error paths**: packaging succeeds but the archive omits `vscode-languageclient` / `vscode-jsonrpc`, or local install still resolves the wrong file.
- **Boundary conditions**: the deterministic path must work when the extension version changes, not only for `0.3.0`, and package scripts/docs must no longer depend on repo-root globbing.

## Steps

1. Add a small helper under `tools/editors/vscode-mesh/scripts/` that computes the current versioned VSIX path under `tools/editors/vscode-mesh/dist/` so `package` and `install-local` stop hardcoding `0.3.0` or relying on `ls *.vsix`.
2. Update `tools/editors/vscode-mesh/package.json` and `.vscodeignore` so packaging emits the deterministic dist artifact, includes the runtime dependency tree required by `vscode-languageclient/node`, and still excludes source/tests/maps/dev-only clutter.
3. Fix extension artifact hygiene in `.gitignore` and remove the checked-in historical VSIXs if they are still present so future work cannot accidentally publish a stale file.
4. Rewrite `tools/editors/vscode-mesh/README.md` and `website/docs/docs/tooling/index.md` to use the version-agnostic local install path (`npm run package` / `npm run install-local` or the deterministic dist file) instead of `mesh-lang-0.3.0.vsix`.

## Must-Haves

- [ ] Packaging writes one deterministic VSIX under `tools/editors/vscode-mesh/dist/`
- [ ] The packaged VSIX contains runtime `vscode-languageclient` / `vscode-jsonrpc` files needed for activation
- [ ] Historical VSIX outputs and the real extension `node_modules/` path are ignored so they cannot shadow fresh packaging results
- [ ] Extension scripts and docs no longer hardcode a specific `mesh-lang-<version>.vsix` filename

## Verification

- `npm --prefix tools/editors/vscode-mesh run compile`
- `npm --prefix tools/editors/vscode-mesh run package`
- `VERSION=$(node -p "require('./tools/editors/vscode-mesh/package.json').version"); VSIX="tools/editors/vscode-mesh/dist/mesh-lang-${VERSION}.vsix"; test -f "$VSIX" && unzip -l "$VSIX" | rg 'extension/node_modules/(vscode-languageclient|vscode-jsonrpc)'`
- `! rg -n 'mesh-lang-0\.[0-9]+\.vsix|--no-dependencies' tools/editors/vscode-mesh/package.json tools/editors/vscode-mesh/README.md website/docs/docs/tooling/index.md`

## Observability Impact

- Signals added/changed: deterministic VSIX path resolution and archive listing checks replace ambiguous repo-root glob selection.
- How a future agent inspects this: rerun `npm --prefix tools/editors/vscode-mesh run package` and inspect the single `tools/editors/vscode-mesh/dist/*.vsix` archive.
- Failure state exposed: missing runtime dependency entries or stale versioned install commands show up before the workflow exists.

## Inputs

- `tools/editors/vscode-mesh/package.json`
- `tools/editors/vscode-mesh/.vscodeignore`
- `tools/editors/vscode-mesh/src/extension.ts`
- `tools/editors/vscode-mesh/README.md`
- `website/docs/docs/tooling/index.md`
- `.gitignore`
- `tools/editors/vscode-mesh/mesh-lang-0.2.0.vsix`
- `tools/editors/vscode-mesh/mesh-lang-0.3.0.vsix`

## Expected Output

- `tools/editors/vscode-mesh/package.json`
- `tools/editors/vscode-mesh/.vscodeignore`
- `tools/editors/vscode-mesh/scripts/vsix-path.mjs`
- `.gitignore`
- `tools/editors/vscode-mesh/README.md`
- `website/docs/docs/tooling/index.md`
- `tools/editors/vscode-mesh/mesh-lang-0.2.0.vsix`
- `tools/editors/vscode-mesh/mesh-lang-0.3.0.vsix`
