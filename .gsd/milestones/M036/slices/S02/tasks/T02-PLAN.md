---
estimated_steps: 4
estimated_files: 5
skills_used:
  - test
  - debug-like-expert
---

# T02: Wire native Neovim LSP bootstrap with repo-local meshc discovery

**Slice:** S02 — Repo-owned first-class Neovim support pack
**Milestone:** M036

## Description

Once Mesh buffers identify and highlight correctly, add the repo-owned LSP path so a plain runtime-pack install can attach `meshc lsp` on Neovim 0.11+ without `nvim-lspconfig` or a plugin manager.

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

## Verification

- `NEOVIM_BIN="${NEOVIM_BIN:-nvim}" bash scripts/verify-m036-s02.sh lsp`
- The command exits 0 only after reporting the resolved `meshc` path class, selected root, and attached Mesh client for real repo files.

## Observability Impact

- Signals added/changed: the LSP smoke should emit the resolved binary path, chosen root directory, and attached client name.
- How a future agent inspects this: run `NEOVIM_BIN=... bash scripts/verify-m036-s02.sh lsp` and inspect the attach diagnostics.
- Failure state exposed: bad binary discovery, bad root selection, unsupported Neovim version, or attach timeout becomes visible without interactive editor debugging.

## Inputs

- `tools/editors/vscode-mesh/src/extension.ts` — existing `meshc` search-order truth to mirror for Neovim dogfooding.
- `compiler/mesh-lsp/src/analysis.rs` — current `main.mpl`-driven project-root truth that Neovim must respect.
- `compiler/meshc/tests/e2e_lsp.rs` — proven `meshc lsp` transport surface this task wires into Neovim rather than redefines.
- `tools/editors/neovim-mesh/ftdetect/mesh.vim` — prior task filetype surface the native LSP config should consume.
- `tools/editors/neovim-mesh/syntax/mesh.vim` — prior task syntax surface the LSP smoke should open alongside attach checks.
- `scripts/verify-m036-s02.sh` — verifier wrapper to extend with an `lsp` phase.

## Expected Output

- `tools/editors/neovim-mesh/lua/mesh.lua` — shared binary-discovery helper for the runtime pack.
- `tools/editors/neovim-mesh/lsp/mesh.lua` — native Neovim LSP config for Mesh.
- `tools/editors/neovim-mesh/plugin/mesh.lua` — pack bootstrap that enables Mesh LSP automatically on supported Neovim versions.
- `tools/editors/neovim-mesh/tests/lsp_smoke.lua` — headless attach/root-resolution smoke script.
- `scripts/verify-m036-s02.sh` — verifier entrypoint extended with an `lsp` phase.
