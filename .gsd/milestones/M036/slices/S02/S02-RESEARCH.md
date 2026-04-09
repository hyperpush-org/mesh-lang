# S02 Research — Repo-owned first-class Neovim support pack

**Researched:** 2026-03-27  
**Status:** Ready for planning

## Summary

S02 should stay deliberately small: ship a repo-owned Neovim runtime pack that can be installed directly from this repo, detects `*.mpl`, provides honest syntax highlighting, and starts `meshc lsp` without requiring `nvim-lspconfig` or a plugin manager. The strongest path is **native Neovim runtime + native LSP config**, not Tree-sitter.

Why this is the smallest honest delivery:

- Neovim runtimepath already supports `ftdetect/`, `syntax/`, `lua/`, and `lsp/` directly, so the pack can be a plain directory under `tools/editors/neovim-mesh/`.
- The current Mesh repo already has the two truths S02 should reuse instead of re-inventing:
  - syntax truth corpus + interpolation contract from S01
  - transport/LSP truth from `compiler/meshc/tests/e2e_lsp.rs`
- There is **no existing Tree-sitter grammar** in-repo, and current Tree-sitter install paths require a grammar project plus highlight queries plus parser installation plumbing. That is a scope multiplier, not the smallest honest Neovim proof.

This slice directly supports:

- **R006** — makes the non-VSCode editor story daily-driver credible with a repo-owned install path and proof
- **R008** — replaces vague “other editors can wire it up” guidance with a real workflow the repo owns
- **R010** — gives Mesh a concrete DX advantage: native Neovim support without plugin-manager folklore or editor-specific server wrappers

## Skill Notes

Two loaded-skill rules should shape the implementation and proof:

- From **`debug-like-expert`**: **“VERIFY, DON’T ASSUME.”** S02 should prove Neovim behavior with headless editor smoke, not README-only claims or inferred compatibility.
- From **`test`**: **“MATCH EXISTING PATTERNS”** and **“VERIFY GENERATED TESTS.”** S02 should reuse the repo’s existing proof style: repo-root verify wrapper + focused test/smoke artifacts, then run them for real.

## Skills Discovered

Installed because they are directly relevant to this slice’s core technologies:

- `julianobarbosa/claude-code-skills@neovim` — discovered via `npx skills find "neovim"` (100 installs)
- `plurigrid/asi@tree-sitter` — discovered via `npx skills find "tree-sitter"` (9 installs)

## What Exists Today

### 1. VS Code already contains the best reusable Mesh editor glue

Relevant files:

- `tools/editors/vscode-mesh/src/extension.ts`
- `tools/editors/vscode-mesh/package.json`
- `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json`
- `tools/editors/vscode-mesh/language-configuration.json`

Useful reusable behavior:

- `findMeshc()` in `tools/editors/vscode-mesh/src/extension.ts` already defines a practical `meshc` discovery order:
  1. explicit user override
  2. workspace-local `target/debug/meshc` or `target/release/meshc`
  3. `~/.mesh/bin/meshc`
  4. `/usr/local/bin/meshc`
  5. `/opt/homebrew/bin/meshc`
  6. PATH fallback
- The shared TextMate grammar in `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json` is now the audited source of truth for broad token classes and for both `#{...}` plus `${...}` interpolation.
- `language-configuration.json` shows which behaviors are currently VS Code-only convenience surfaces (brackets, auto-closing, indentation, folding). S02 does **not** need to reproduce all of that to be honest.

### 2. S01 already created the right syntax proof input

Relevant files:

- `scripts/fixtures/m036-s01-syntax-corpus.json`
- `scripts/fixtures/m036-s01/interpolation_edge_cases.mpl`
- `website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs`
- `scripts/verify-m036-s01.sh`

What matters for S02:

- The corpus manifest already names representative Mesh snippets from `mesher/`, `reference-backend/`, test fixtures, and docs.
- The contract already encodes the hard part of string truth: both interpolation forms are valid; plain strings must stay plain; triple-quoted and nested-brace cases are explicitly covered.
- The S01 verifier already has line-slice and interpolation-scan logic. S02 does not need a new corpus format.

Important nuance:

- Some S01 corpus cases come from Markdown files under `website/docs/docs/...`. Neovim syntax smoke cannot open those files directly as Mesh buffers. If S02 reuses the corpus manifest, it needs a small materialization step that writes the selected line ranges into temporary `.mpl` files before opening them in Neovim.

### 3. The LSP server truth is already editor-agnostic and project-aware

Relevant files:

- `compiler/meshc/tests/e2e_lsp.rs`
- `compiler/mesh-lsp/src/server.rs`
- `compiler/mesh-lsp/src/analysis.rs`

Confirmed behavior:

- `cargo test -q -p meshc --test e2e_lsp -- --nocapture` passes on the current branch.
- `meshc lsp` is a real stdio JSON-RPC server; S02 should only prove Neovim wiring/attach, not re-prove hover/definition/formatting semantics.
- `compiler/mesh-lsp/src/analysis.rs` discovers project context by walking ancestors until it finds **`main.mpl`**. That is the current project-root truth for project-aware analysis.
- If no `main.mpl` ancestor exists, analysis falls back to single-document parse/typecheck.

Planning consequence:

- Neovim root markers should prioritize **`main.mpl`** to match current server truth.
- Do **not** make `mesh.toml` the primary root marker yet; the server does not currently use it for project-aware imports.
- Single-file support is still useful and honest, but docs should not overstate workspace-import behavior outside `main.mpl`-rooted projects.

### 4. The local development environment is missing Neovim

Observed locally:

- `nvim` is **not** installed in this environment (`nvim: command not found`)
- `vim` 9.1 is installed
- `target/debug/meshc` exists

Planning consequence:

- Any repo-owned verifier must fail fast with a clear missing-Neovim message or accept `NEOVIM_BIN=...`.
- Do not make the proof depend on a user’s personal Neovim config.
- For actual regression value, the smoke path should be runnable in automation where Neovim is provisioned explicitly.

## Neovim Runtime Findings

Using current Neovim docs and current `nvim-lspconfig` docs:

- `runtimepath` supports `ftdetect/`, `syntax/`, `lua/`, `lsp/`, `parser/`, and `queries/` directly.
- Package layout under `pack/*/start/<name>` is the native no-plugin-manager install path.
- `vim.lsp.config()` + `vim.lsp.enable()` are the native LSP path for Neovim **0.11+**.
- `nvim-lspconfig` guidance says the minimal config shape is `cmd`, `filetypes`, and `root_markers`.
- Native config now assumes single-file support by default, which aligns well with Mesh’s current project-aware-or-single-file analysis split.

This makes the cleanest repo-owned installation contract:

1. put the pack at `tools/editors/neovim-mesh/`
2. document symlink/copy into `~/.config/nvim/pack/mesh/start/mesh-nvim` (or equivalent XDG path)
3. let the pack itself provide `ftdetect`, `syntax`, Lua helper(s), and `lsp/mesh.lua`

That path is materially better than “install nvim-lspconfig, add a custom snippet, then maybe borrow some syntax file from somewhere.”

## Tree-sitter vs Classic Vim Syntax

### Tree-sitter is not the smallest honest path

Current Tree-sitter / nvim-treesitter docs show that a custom language path would require at least:

- a Tree-sitter grammar project
- generated parser sources or generation rules
- `queries/highlights.scm`
- parser installation metadata (`install_info`) or prebuilt parser distribution
- usually `nvim-treesitter` integration or another parser-install story

In this repo today, **none of those artifacts exist** for Mesh.

That means a Tree-sitter-first S02 would expand into:

- new grammar authoring
- parser generation/build/install questions
- query maintenance
- extra docs about parser installation
- a second syntax truth surface before Neovim support is even usable

### Classic Vim syntax is the right first-class proof target for M036

A Vim syntax file can stay bounded to the audited S01 contract and the existing TextMate token classes:

- comments (`#`, `##`, `##!`, `#= =#`)
- strings and escapes
- `#{...}` and `${...}` interpolation, including nested braces
- atoms and regex literals
- numbers
- keywords / declarations / language constants
- type names, module-qualified calls, function names, variables, operators

This is sufficient to satisfy the slice acceptance bar without inventing a second full grammar project.

## Recommended Pack Shape

Recommended new surface:

- `tools/editors/neovim-mesh/README.md` — pack-local install and usage contract
- `tools/editors/neovim-mesh/ftdetect/mesh.vim` — `*.mpl` → `mesh`
- `tools/editors/neovim-mesh/syntax/mesh.vim` — honest first-class highlighting
- `tools/editors/neovim-mesh/lua/mesh/init.lua` (or `lua/mesh.lua`) — helper(s), especially `meshc` discovery
- `tools/editors/neovim-mesh/lsp/mesh.lua` — native Neovim 0.11+ LSP config
- optional `tools/editors/neovim-mesh/plugin/mesh.lua` or `ftplugin/mesh.lua` — auto-enable or buffer-local setup if needed
- `scripts/verify-m036-s02.sh` — repo-root verifier
- `tools/editors/neovim-mesh/tests/smoke.lua` (or similar) — headless Neovim smoke entrypoint

### `meshc` discovery recommendation

Mirror the VS Code extension’s search order instead of forcing PATH-only setup. That gives S02 a concrete DX win and makes repo-local proof easy because `target/debug/meshc` already exists after building/testing.

Suggested contract:

- explicit override first (`vim.g.mesh_lsp_path` or `require('mesh').setup({ meshc_path = ... })`)
- repo/workspace-local `target/debug/meshc` and `target/release/meshc`
- well-known installed locations (`~/.mesh/bin/meshc`, `/usr/local/bin/meshc`, `/opt/homebrew/bin/meshc`)
- PATH fallback

### Root marker recommendation

Use `main.mpl` first. Keep any broader fallback honest.

Best initial shape:

- `filetypes = { 'mesh' }`
- `root_markers = { 'main.mpl', '.git' }`

Why not `mesh.toml` first:

- current server-side project-aware analysis keys off `main.mpl`, not `mesh.toml`
- using `mesh.toml` as if it were authoritative would overclaim current behavior

## Verification Strategy

S02 should follow the repo’s existing evidence-first pattern:

1. **Keep dependency proof green**
   - `bash scripts/verify-m036-s01.sh`
   - `cargo test -q -p meshc --test e2e_lsp -- --nocapture`
2. **Add a repo-root Neovim verifier**
   - `bash scripts/verify-m036-s02.sh`
3. **Drive real headless Neovim** against the repo-owned pack with a temporary runtimepath, not the user’s personal config

Recommended smoke checks:

- open a real `.mpl` file and assert `&filetype == 'mesh'`
- assert syntax is loaded for representative token positions
- assert interpolation highlighting covers both `#{...}` and `${...}` and that plain strings do not get interpolation groups
- assert a Mesh LSP client attaches for a real project file
- optionally assert the resolved LSP command points at the expected `meshc` path class (override/local build/install/PATH)

### Best corpus reuse shape

Reuse `scripts/fixtures/m036-s01-syntax-corpus.json` for syntax samples, but materialize each snippet to a temp `.mpl` file before opening it in Neovim when the source case came from Markdown.

That gives S02:

- the same audited syntax contract as S01
- no hand-maintained second corpus
- fail-closed detection if Neovim drifts on the same representative forms

### What S02 does **not** need to re-prove

- full LSP transport semantics already proven by `compiler/meshc/tests/e2e_lsp.rs`
- VS Code/docs TextMate parity already proven by S01
- public support-tier wording cleanup already belongs to S03

## Natural Task Seams

### Task 1 — Pack contract and runtime layout

Decide and scaffold the new `tools/editors/neovim-mesh/` runtime-pack layout and the canonical install path (`pack/*/start`).

### Task 2 — Runtime implementation

Implement:

- filetype detection
- syntax file
- native Neovim LSP config
- `meshc` resolution helper

This is the main build task.

### Task 3 — Headless proof

Build the repo-root verifier and headless Neovim smoke script. Reuse the S01 corpus manifest and the existing e2e LSP proof instead of inventing a parallel contract.

### Task 4 — Pack-local docs

Write `tools/editors/neovim-mesh/README.md` with:

- exact install path
- minimum Neovim version (0.11+ if using native `lsp/` config)
- what is first-class in this pack
- how `meshc` is resolved / overridden
- how to run the repo verification command

Keep broad public support-tier wording out of S02 unless a minimal pointer is required; S03 owns that cleanup.

## Risks / Watchouts

- **Do not start with Tree-sitter.** That turns S02 into a grammar project.
- **Do not claim `mesh.toml` workspace truth the server does not currently implement.**
- **Do not duplicate the syntax corpus.** Reuse S01’s manifest.
- **Do not depend on user Neovim config or plugin managers for the first-class path.**
- **Do not quietly require PATH-only `meshc`.** Reuse the repo-local discovery pattern; otherwise dogfooding inside this repo is worse than VS Code.
- **Do not let S02 absorb S03’s support-tier/public-doc scope.** Pack-local docs are enough for this slice.

## Verification Notes From This Research

Commands run successfully on the current branch:

- `bash scripts/verify-m036-s01.sh`
- `cargo test -q -p meshc --test e2e_lsp -- --nocapture`

Environment observations:

- `target/debug/meshc` exists
- `nvim` is not installed here
- `vim` 9.1 is installed, which may help syntax iteration locally but is **not** sufficient as the slice’s contractual proof target

## Sources

Code surfaces read:

- `tools/editors/vscode-mesh/src/extension.ts`
- `tools/editors/vscode-mesh/package.json`
- `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json`
- `tools/editors/vscode-mesh/language-configuration.json`
- `scripts/fixtures/m036-s01-syntax-corpus.json`
- `website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs`
- `scripts/verify-m036-s01.sh`
- `compiler/meshc/tests/e2e_lsp.rs`
- `compiler/mesh-lsp/src/server.rs`
- `compiler/mesh-lsp/src/analysis.rs`
- `website/docs/docs/tooling/index.md`
- `scripts/verify-m034-s04-extension.sh`

External docs checked:

- Neovim runtimepath / packages / native runtime directory docs (`runtimepath`, `pack/*/start`, `lsp/`, `parser/`, `queries/`)
- `nvim-lspconfig` docs for `vim.lsp.config()`, `vim.lsp.enable()`, `cmd`, `filetypes`, `root_markers`, and root-marker priority
- Tree-sitter / nvim-treesitter docs for custom parser install shape (`install_info`, parser source, queries)
