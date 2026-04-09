# S02: Repo-owned first-class Neovim support pack

**Goal:** Ship a repo-owned native Neovim support pack under `tools/editors/neovim-mesh/` that installs through Neovim's package runtime, detects `*.mpl`, provides honest first-class classic syntax support for the audited Mesh contract, and auto-starts `meshc lsp` through Neovim 0.11+ native LSP with repo-local `meshc` discovery.
**Demo:** After this: Follow the repo docs to install the Mesh Neovim pack, open a `.mpl` file, and get filetype/syntax support plus `meshc lsp` through the documented first-class path.

## Tasks
- [x] **T01: Added an installable Neovim Mesh pack with forced `*.mpl` filetype detection, classic syntax highlighting, and a headless syntax verifier.** — Implement the smallest truthful editor surface first: opening `*.mpl` in a stock Neovim package install should identify the buffer as Mesh and highlight the audited language forms before any LSP wiring is involved.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json` and the S01 interpolation contract | Keep the Neovim syntax narrower rather than inventing unsupported groups; fail the smoke if `#{...}` / `${...}` or plain-string cases drift. | Not applicable — local files only. | Treat unexpected interpolation or string-kind cases as unsupported and surface the failing case id/position. |
| `NEOVIM_BIN` / headless Neovim | Fail fast with a clear missing-binary or unsupported-version message. | Abort the syntax smoke and print the hanging command/phase. | Treat missing syntax APIs or zero syntax groups as failure, not as a silent pass. |

## Load Profile

- **Shared resources**: Headless Neovim runtimepath and syntax engine only.
- **Per-operation cost**: Opens one representative Mesh buffer at a time and checks a handful of syntax positions; trivial.
- **10x breakpoint**: Corpus size/debuggability, not compute; failures should stay localized by case id instead of degenerating into whole-file noise.

## Negative Tests

- **Malformed inputs**: Plain quoted strings and explicit no-interpolation cases must not receive interpolation groups.
- **Error paths**: Missing `ftdetect` or unloaded syntax files must fail the smoke instead of silently falling back to `text`.
- **Boundary conditions**: Double-quoted, triple-quoted, `#{...}`, `${...}`, and nested-brace interpolation all remain distinguishable.

## Steps

1. Create `tools/editors/neovim-mesh/ftdetect/mesh.vim` so `*.mpl` resolves to filetype `mesh` through native runtime discovery.
2. Implement `tools/editors/neovim-mesh/syntax/mesh.vim` as a bounded classic Vim syntax file covering the audited Mesh token classes from S01: comments, strings/escapes, interpolation, atoms, regex, numbers, keywords, types, module-qualified calls, functions, variables, and operators.
3. Add `tools/editors/neovim-mesh/tests/syntax_smoke.lua` plus the first `syntax` phase in `scripts/verify-m036-s02.sh` to open representative Mesh files headlessly, assert `&filetype == 'mesh'`, and inspect syntax groups at known interpolation/plain-string positions.
4. Keep the implementation fail-closed: if Neovim cannot distinguish a promised token class, the smoke output should name the failing file/case/position rather than passing optimistically.

## Must-Haves

- [ ] Installing the pack alone is enough for `*.mpl` buffers to become `mesh`.
- [ ] Syntax highlighting stays honest to the S01 interpolation contract instead of inventing Tree-sitter or broader claims.
- [ ] The syntax smoke proves both positive interpolation cases and negative plain-string cases in headless Neovim.
  - Estimate: 1.5h
  - Files: tools/editors/neovim-mesh/ftdetect/mesh.vim, tools/editors/neovim-mesh/syntax/mesh.vim, tools/editors/neovim-mesh/tests/syntax_smoke.lua, scripts/verify-m036-s02.sh
  - Verify: NEOVIM_BIN="${NEOVIM_BIN:-nvim}" bash scripts/verify-m036-s02.sh syntax
- [x] **T02: Added native Neovim Mesh LSP bootstrap with repo-local meshc discovery, honest root handling, and headless attach proof.** — Once Mesh buffers identify and highlight correctly, add the repo-owned LSP path so a plain runtime-pack install can attach `meshc lsp` on Neovim 0.11+ without `nvim-lspconfig` or a plugin manager.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `meshc` discovery / local binary resolution | Surface the resolved candidate list and fail with an actionable override message (`vim.g.mesh_lsp_path` or setup option). | Fail the LSP smoke after an attach timeout and print the buffer path/root marker used. | Treat non-executable paths or non-zero spawn errors as failure, not PATH fallback. |
| Neovim 0.11 `vim.lsp` APIs | Guard older versions and fail with a clear minimum-version message. | Not applicable — local API check. | Treat missing `vim.lsp.enable` or broken `lsp/mesh.lua` loading as failure. |
| Mesh project root detection | Prefer `main.mpl`, then `.git`, and fall back to single-file support without overclaiming workspace imports. | Fail attach if root detection loops or never resolves. | Treat bogus roots as failure and print the chosen root in smoke output. |

## Load Profile

- **Shared resources**: One `meshc lsp` process per resolved project root plus buffer attach state inside Neovim.
- **Per-operation cost**: One stdio LSP subprocess and a few opened buffers; low, but root-selection mistakes can multiply processes.
- **10x breakpoint**: Spawning duplicate servers for adjacent files or mis-rooted projects; smoke should prove root reuse on real repo files.

## Negative Tests

- **Malformed inputs**: An explicit override to a missing binary must fail loudly instead of silently falling through.
- **Error paths**: Opening a standalone `.mpl` file without `main.mpl` ancestry should still attach in single-file mode.
- **Boundary conditions**: Repo-local `target/debug/meshc`, well-known install locations, and PATH fallback stay ordered and inspectable.

## Steps

1. Add `tools/editors/neovim-mesh/lua/mesh.lua` with a single discovery helper mirroring the VS Code search order: explicit override, workspace-local `target/debug` / `target/release`, well-known install paths, then PATH.
2. Add `tools/editors/neovim-mesh/lsp/mesh.lua` returning the native Neovim config with `cmd`, `filetypes = { 'mesh' }`, `root_markers = { 'main.mpl', '.git' }`, and honest single-file support.
3. Add `tools/editors/neovim-mesh/plugin/mesh.lua` (or equivalent pack bootstrap) so installing the pack enables the Mesh config automatically on supported Neovim versions instead of requiring extra user Lua.
4. Extend headless proof with `tools/editors/neovim-mesh/tests/lsp_smoke.lua` and a `lsp` phase in `scripts/verify-m036-s02.sh` that opens real Mesh files, asserts a Mesh client attaches, and reports the resolved `meshc` path plus chosen root.

## Must-Haves

- [ ] The pack starts `meshc lsp` through native `vim.lsp`, not `nvim-lspconfig`.
- [ ] `main.mpl` is the primary root marker so Neovim matches current server-side analysis truth.
- [ ] Binary discovery is repo-local friendly and user-overridable, matching VS Code's dogfooding ergonomics.
  - Estimate: 1.5h
  - Files: tools/editors/neovim-mesh/lua/mesh.lua, tools/editors/neovim-mesh/lsp/mesh.lua, tools/editors/neovim-mesh/plugin/mesh.lua, tools/editors/neovim-mesh/tests/lsp_smoke.lua, scripts/verify-m036-s02.sh
  - Verify: NEOVIM_BIN="${NEOVIM_BIN:-nvim}" bash scripts/verify-m036-s02.sh lsp
- [x] **T03: Proved the Neovim pack against the shared corpus, unified the headless verifier, and documented the pack-local install contract.** — Close the slice with proof and install docs: reuse the S01 corpus instead of duplicating examples, wrap the headless phases in one repo-root verifier, and publish the exact pack-local install contract.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| S01 corpus manifest / materialized snippets | Fail the verifier with the specific corpus case or source path that could not be materialized. | Abort materialization and print the stuck phase. | Treat markdown-backed snippets that do not render valid `.mpl` text as failure. |
| `cargo test -q -p meshc --test e2e_lsp -- --nocapture` | Stop the wrapper before Neovim proof and report the failing upstream LSP phase. | Fail the wrapper with the timed-out phase name. | Treat non-passing test output as failure even if the command exits unexpectedly. |
| `NEOVIM_BIN` headless smoke | Stop with a clear missing-binary or unsupported-version message. | Fail with the last phase (`syntax`, `lsp`, or `corpus`) and preserve logs under `.tmp/m036-s02/`. | Treat partial attach or missing syntax groups as failure, not best-effort pass. |

## Load Profile

- **Shared resources**: Temporary materialized corpus directory, one headless Neovim process, and one LSP subprocess.
- **Per-operation cost**: Linear in corpus cases; small enough for CI/local smoke.
- **10x breakpoint**: Corpus expansion; the verifier should stay phase-oriented and case-oriented so bigger corpora remain debuggable.

## Negative Tests

- **Malformed inputs**: Docs-backed corpus cases must be materialized to temporary `.mpl` files before opening them in Neovim; raw markdown paths must not be treated as Mesh buffers.
- **Error paths**: Missing Neovim binary, missing `meshc`, or failing upstream S01/LSP proof must stop the wrapper before any green claim.
- **Boundary conditions**: Verified docs must state Neovim 0.11+, the `pack/*/start/mesh-nvim` install path, override knobs, and the exact repo verification command without pulling public support-tier work forward from S03.

## Steps

1. Add `scripts/tests/verify-m036-s02-materialize-corpus.mjs` (or equivalent) to reuse `scripts/fixtures/m036-s01-syntax-corpus.json`, extracting markdown-backed line ranges to temporary `.mpl` files while preserving case ids and expected interpolation forms.
2. Collapse syntax/LSP probes into a final headless runner under `tools/editors/neovim-mesh/tests/smoke.lua` and wrap it with `scripts/verify-m036-s02.sh`, including phase banners, `NEOVIM_BIN` override support, and upstream replays of `scripts/verify-m036-s01.sh` plus `e2e_lsp`.
3. Write `tools/editors/neovim-mesh/README.md` with the exact install path, Neovim 0.11+ floor, what the pack does and does not prove, how `meshc` is resolved/overridden, and how to run the repo verifier.
4. Keep docs local to the pack: do not widen public support-tier claims here beyond a factual pointer that S03 can later fold into public tooling docs.

## Must-Haves

- [ ] The final verifier exercises the real repo-owned install/runtime path end-to-end and fails closed by phase/case.
- [ ] The syntax smoke reuses the S01 corpus instead of creating a second hand-maintained example list.
- [ ] README instructions are sufficient for a fresh Neovim user to install the pack and run the same proof locally.
  - Estimate: 2h
  - Files: scripts/tests/verify-m036-s02-materialize-corpus.mjs, tools/editors/neovim-mesh/tests/smoke.lua, tools/editors/neovim-mesh/README.md, scripts/verify-m036-s02.sh
  - Verify: NEOVIM_BIN="${NEOVIM_BIN:-nvim}" bash scripts/verify-m036-s02.sh
