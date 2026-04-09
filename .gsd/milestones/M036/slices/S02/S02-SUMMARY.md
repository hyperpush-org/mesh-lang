---
id: S02
parent: M036
milestone: M036
provides:
  - A repo-owned Neovim runtime pack install contract under `tools/editors/neovim-mesh/` that turns `*.mpl` buffers into `filetype=mesh`.
  - A first-class classic Vim syntax surface for the audited Mesh interpolation/string contract, backed by corpus-based headless smoke.
  - A native Neovim 0.11+ `meshc lsp` bootstrap path with repo-local binary discovery, honest `main.mpl` root selection, standalone single-file support, and one end-to-end verifier.
requires:
  - slice: S01
    provides: The audited shared interpolation corpus plus `scripts/verify-m036-s01.sh` as the existing syntax-truth surface reused by the Neovim pack.
affects:
  - S03
key_files:
  - tools/editors/neovim-mesh/ftdetect/mesh.vim
  - tools/editors/neovim-mesh/syntax/mesh.vim
  - tools/editors/neovim-mesh/lua/mesh.lua
  - tools/editors/neovim-mesh/lsp/mesh.lua
  - tools/editors/neovim-mesh/plugin/mesh.lua
  - tools/editors/neovim-mesh/tests/smoke.lua
  - scripts/tests/verify-m036-s02-materialize-corpus.mjs
  - scripts/verify-m036-s02.sh
  - tools/editors/neovim-mesh/README.md
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - Implement the non-VS Code editor story as a repo-owned native Neovim runtime pack under `tools/editors/neovim-mesh/` instead of external ecosystem setup or Tree-sitter work.
  - Keep LSP workspace rooting honest with `main.mpl` then `.git`, while resolving `meshc` independently through explicit override, repo-local build outputs, well-known install paths, then `PATH`, and allowing standalone buffers in true single-file mode.
  - Materialize every shared S01 syntax corpus case into temporary `.mpl` files and drive one consolidated Neovim smoke runner from that manifest so docs-backed markdown snippets fail closed instead of being opened as raw markdown buffers.
patterns_established:
  - For editor support work, prefer a repo-owned install/runtime path plus one repo-root verifier over plugin-manager-specific setup or hand-maintained manual smoke steps.
  - Reuse shared syntax corpora across editors by materializing editor-specific fixtures from one source of truth instead of copying example snippets into each verifier.
  - Separate compiler/LSP binary discovery from workspace root detection so repo-local dogfooding stays good without overstating cross-file project semantics.
observability_surfaces:
  - `scripts/verify-m036-s02.sh` phase banners and fail-closed logs under `.tmp/m036-s02/<phase>/`.
  - `tools/editors/neovim-mesh/tests/smoke.lua` case-level syntax output naming filetype, syntax stack, marker/probe positions, root marker, client ids, and resolved `meshc` path/class.
  - `tools/editors/neovim-mesh/README.md` as the local operator-facing runbook for install path, override knobs, and exact verification commands.
drill_down_paths:
  - .gsd/milestones/M036/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M036/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M036/slices/S02/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-28T06:10:44.433Z
blocker_discovered: false
---

# S02: Repo-owned first-class Neovim support pack

**Delivered a repo-owned Neovim pack under `tools/editors/neovim-mesh/` with native package install docs, forced `*.mpl` filetype detection, audited classic syntax highlighting, and native Neovim 0.11+ `meshc lsp` bootstrap proven by corpus-backed headless smoke.**

## What Happened

S02 turned the non-VS Code editor goal into an installable repo-owned path instead of a docs-only claim. The slice added `tools/editors/neovim-mesh/` as a native Neovim runtime pack that installs through `pack/*/start/mesh-nvim`, forces `*.mpl` buffers to `filetype=mesh` so Vim/Neovim's built-in Maple mapping cannot win, and provides a bounded classic syntax file aligned to the audited S01 interpolation contract rather than inventing broader Tree-sitter-style claims. The syntax proof is fail-closed: corpus-backed interpolation cases and plain-string controls must load as `mesh`, load `b:current_syntax == 'mesh'`, and report the expected highlight stack at named positions.

The slice then added the first-class Neovim LSP path without depending on `nvim-lspconfig`. `tools/editors/neovim-mesh/lua/mesh.lua`, `lsp/mesh.lua`, and `plugin/mesh.lua` now auto-enable a native `vim.lsp` config on Neovim 0.11+, resolve `meshc lsp` through an honest search order (explicit override, repo/workspace-local `target/{debug,release}`, well-known install paths, then `PATH`), root projects at `main.mpl` before `.git`, and keep standalone buffers in true single-file mode (`root_dir = nil`) while still allowing repo-local dogfooding through cwd ancestry. The smoke output makes that behavior inspectable by printing the resolved `meshc` class/path, root marker, root directory, and attached client ids for rooted and standalone cases.

Finally, S02 closed the loop with one repo-root verifier and local install contract. `scripts/tests/verify-m036-s02-materialize-corpus.mjs` converts the shared S01 syntax corpus into temporary `.mpl` files so markdown-backed docs snippets are proven honestly in Neovim, `tools/editors/neovim-mesh/tests/smoke.lua` consolidates syntax and LSP checks, `scripts/verify-m036-s02.sh` replays the shared grammar proof and upstream `meshc` JSON-RPC LSP proof before running the real Neovim package-runtime smoke, and `tools/editors/neovim-mesh/README.md` documents the exact install path, Neovim 0.11+ floor, override knobs, and verifier command. For downstream work, S03 can now publish support tiers and public tooling docs from a concrete repo-owned Neovim contract instead of a speculative editor story.

## Verification

Re-ran the slice proof from the repo root using the repo-local Neovim 0.11.6 binary at `.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim` because this host does not provide a system `nvim`.

- `node --test scripts/tests/verify-m036-s02-materialize-corpus.test.mjs scripts/tests/verify-m036-s02-contract.test.mjs` — passed (`tests 4`, `pass 4`, `fail 0`), confirming the corpus materializer and README/verifier contract stay mechanically enforced.
- `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh syntax` — passed; materialized all 15 shared corpus cases, installed the pack through a real `pack/*/start/mesh-nvim` path, confirmed `filetype=mesh`/`syntax=mesh`, and proved positive interpolation plus plain-string negative controls in headless Neovim.
- `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh lsp` — passed; replayed `cargo test -q -p meshc --test e2e_lsp -- --nocapture`, then proved the Neovim client attached on rooted `reference-backend` files with `marker=main.mpl`, reused the repo-local `target/debug/meshc`, and also attached for a standalone temporary `.mpl` file with `root=<none>` / `marker=single-file`.
- `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh` — passed end to end; replayed corpus materialization, `bash scripts/verify-m036-s01.sh`, the upstream Rust LSP transport proof, and the consolidated Neovim smoke against the real package-runtime install path.

### Operational Readiness
- **Health signal:** `scripts/verify-m036-s02.sh` emits explicit `corpus`, `shared-grammar`, `upstream-lsp`, and `neovim` phases; the Neovim smoke logs named syntax cases plus `phase=lsp` records that expose `client_id`, `buffer`, `marker`, `root`, `meshc_class`, and `meshc_path`.
- **Failure signal:** the verifier fail-closes with `first failing phase`, the chosen `neovim_bin`, and phase-local log paths under `.tmp/m036-s02/`. The LSP smoke also includes a negative missing-override case so bad explicit `meshc` overrides fail loudly instead of silently falling through.
- **Recovery procedure:** rerun the narrow phase first (`syntax` or `lsp`) with `NEOVIM_BIN` set, inspect the phase log under `.tmp/m036-s02/<phase>/`, then fix the specific boundary: pack install path / filetype detection, `vim.g.mesh_lsp_path` or `require('mesh').setup({ lsp_path = ... })`, or the nearest `main.mpl` root marker layout. Re-run the full verifier only after the failing narrow phase passes.
- **Monitoring gaps:** this is still a smoke-proof surface, not continuous in-editor telemetry. There is no Tree-sitter path, no support below Neovim 0.11, and no always-on runtime reporting beyond verifier logs and Neovim startup errors.

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Used the repo-local portable Neovim 0.11.6 binary under `.tmp/m036-s02/vendor/` because this environment does not have a system `nvim`. Also kept two small Node contract tests for corpus materialization and README/verifier drift beyond the plan's minimum file list so the install/proof contract stays mechanically enforced.

## Known Limitations

The first-class Neovim contract is intentionally bounded to the audited classic syntax surface and native `meshc lsp` bootstrap proven here. It does not claim Tree-sitter support, plugin-manager-specific installation, richer regex coverage than the stable classic Vim matcher currently proves, or the broader public support-tier language that belongs in S03. Continuous operational monitoring inside Neovim also remains out of scope; the repo-owned verifier is the authoritative truth surface.

## Follow-ups

S03 should fold this repo-local Neovim contract into the public tooling docs and explicit support-tier language without widening claims beyond what `scripts/verify-m036-s02.sh` proves. Any future syntax expansion should extend the shared S01 corpus first and keep the Neovim smoke fail-closed on the same case ids. If Mesh later wants Tree-sitter or broader Neovim semantics, that should land as a separate proof-backed slice rather than being implied by this classic pack.

## Files Created/Modified

- `tools/editors/neovim-mesh/ftdetect/mesh.vim` — Forces `*.mpl` buffers to `filetype=mesh` so the repo-owned pack wins over Vim/Neovim's built-in Maple detection.
- `tools/editors/neovim-mesh/syntax/mesh.vim` — Implements the bounded classic syntax contract for comments, strings, interpolation, regex, numbers, keywords, types, calls, variables, and operators.
- `tools/editors/neovim-mesh/lua/mesh.lua` — Centralizes Neovim-version gating, root inspection, ordered `meshc` discovery, and observable startup failure state for the pack.
- `tools/editors/neovim-mesh/lsp/mesh.lua` — Defines the native Neovim LSP config with `filetypes = { 'mesh' }`, `root_markers = { 'main.mpl', '.git' }`, and honest single-file support.
- `tools/editors/neovim-mesh/plugin/mesh.lua` — Auto-enables the `mesh` LSP config on supported Neovim versions without requiring `nvim-lspconfig` or extra user Lua.
- `tools/editors/neovim-mesh/tests/smoke.lua` — Runs the consolidated headless syntax and LSP smoke against the real package-runtime install path and generated corpus manifest.
- `scripts/tests/verify-m036-s02-materialize-corpus.mjs` — Materializes the shared S01 syntax corpus, including markdown-backed docs snippets, into per-case temporary `.mpl` files for Neovim proof.
- `scripts/verify-m036-s02.sh` — Wraps corpus materialization, shared grammar replay, upstream Rust LSP proof, and headless Neovim smoke into one fail-closed verifier.
- `tools/editors/neovim-mesh/README.md` — Documents the exact `pack/*/start/mesh-nvim` install path, Neovim 0.11+ floor, `meshc` override/discovery rules, and verifier commands.
- `.gsd/KNOWLEDGE.md` — Recorded the durable Neovim gotchas around the Maple `*.mpl` collision, standalone discovery ordering, corpus materialization, and intentionally narrow classic regex matcher.
- `.gsd/PROJECT.md` — Updated project state to note that M036 S02 now provides a repo-owned first-class Neovim path and end-to-end verifier.
