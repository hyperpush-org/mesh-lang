---
id: T02
parent: S02
milestone: M036
provides: []
requires: []
affects: []
key_files: ["tools/editors/neovim-mesh/lua/mesh.lua", "tools/editors/neovim-mesh/lsp/mesh.lua", "tools/editors/neovim-mesh/plugin/mesh.lua", "tools/editors/neovim-mesh/tests/lsp_smoke.lua", "scripts/verify-m036-s02.sh", ".gsd/DECISIONS.md", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Resolve meshc separately from workspace rooting: keep main.mpl then .git as the Neovim LSP root truth, but search override, root/buffer/cwd ancestor target/{debug,release}, well-known paths, then PATH for the binary.", "Keep standalone .mpl buffers in true single-file mode (root_dir = nil) and use cwd ancestry only for repo-local compiler discovery, not to invent a workspace root."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the Neovim syntax regression smoke, the new Neovim LSP smoke, and the existing upstream meshc JSON-RPC LSP e2e test. The syntax phase still passed with the new plugin/bootstrap present. The lsp phase exited 0 only after printing the resolved meshc class/path, chosen root marker/root, and attached client ids for rooted repo files plus the standalone single-file case. The upstream Rust e2e_lsp test still passed unchanged."
completed_at: 2026-03-28T05:53:36.534Z
blocker_discovered: false
---

# T02: Added native Neovim Mesh LSP bootstrap with repo-local meshc discovery, honest root handling, and headless attach proof.

> Added native Neovim Mesh LSP bootstrap with repo-local meshc discovery, honest root handling, and headless attach proof.

## What Happened
---
id: T02
parent: S02
milestone: M036
key_files:
  - tools/editors/neovim-mesh/lua/mesh.lua
  - tools/editors/neovim-mesh/lsp/mesh.lua
  - tools/editors/neovim-mesh/plugin/mesh.lua
  - tools/editors/neovim-mesh/tests/lsp_smoke.lua
  - scripts/verify-m036-s02.sh
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Resolve meshc separately from workspace rooting: keep main.mpl then .git as the Neovim LSP root truth, but search override, root/buffer/cwd ancestor target/{debug,release}, well-known paths, then PATH for the binary.
  - Keep standalone .mpl buffers in true single-file mode (root_dir = nil) and use cwd ancestry only for repo-local compiler discovery, not to invent a workspace root.
duration: ""
verification_result: passed
completed_at: 2026-03-28T05:53:36.535Z
blocker_discovered: false
---

# T02: Added native Neovim Mesh LSP bootstrap with repo-local meshc discovery, honest root handling, and headless attach proof.

**Added native Neovim Mesh LSP bootstrap with repo-local meshc discovery, honest root handling, and headless attach proof.**

## What Happened

Implemented the repo-owned Neovim LSP path under tools/editors/neovim-mesh by adding a shared Lua helper for Neovim-version gating, root inspection, ordered meshc discovery, and observable startup failure state; a native lsp/mesh.lua config that starts meshc lsp through vim.lsp with filetypes = { 'mesh' }, root_markers = { 'main.mpl', '.git' }, and single-file support; and a plugin bootstrap that auto-enables the config on Neovim 0.11+ without nvim-lspconfig. Extended the headless verifier with tools/editors/neovim-mesh/tests/lsp_smoke.lua plus an lsp phase in scripts/verify-m036-s02.sh. The smoke now proves a loud missing-override failure, rooted client reuse on real reference-backend files, and standalone single-file attach with nil root_dir while still discovering the repo-local target/debug/meshc from cwd ancestry. During verification, fixed two real bugs at the root cause: path-case drift between shell and Neovim realpaths on macOS, and a Lua ipairs(nil-truncation) bug that suppressed standalone cwd fallback.

## Verification

Ran the Neovim syntax regression smoke, the new Neovim LSP smoke, and the existing upstream meshc JSON-RPC LSP e2e test. The syntax phase still passed with the new plugin/bootstrap present. The lsp phase exited 0 only after printing the resolved meshc class/path, chosen root marker/root, and attached client ids for rooted repo files plus the standalone single-file case. The upstream Rust e2e_lsp test still passed unchanged.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh syntax` | 0 | ✅ pass | 291ms |
| 2 | `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh lsp` | 0 | ✅ pass | 326ms |
| 3 | `cargo test -q -p meshc --test e2e_lsp -- --nocapture` | 0 | ✅ pass | 3972ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `tools/editors/neovim-mesh/lua/mesh.lua`
- `tools/editors/neovim-mesh/lsp/mesh.lua`
- `tools/editors/neovim-mesh/plugin/mesh.lua`
- `tools/editors/neovim-mesh/tests/lsp_smoke.lua`
- `scripts/verify-m036-s02.sh`
- `.gsd/DECISIONS.md`
- `.gsd/KNOWLEDGE.md`


## Deviations
None.

## Known Issues
None.
