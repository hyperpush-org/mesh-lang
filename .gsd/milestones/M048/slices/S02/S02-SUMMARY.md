---
id: S02
parent: M048
milestone: M048
provides:
  - A shared manifest-first root and entrypoint contract across compiler/test/LSP/editor/package surfaces.
  - Retained override-entry acceptance fixtures for stdio JSON-RPC, Neovim, and VS Code editor-host proofs.
  - Recursive publish archive rules that preserve nested Mesh source trees relative to project root.
requires:
  - slice: S01
    provides: The shared `[package].entrypoint` default-plus-override contract, the common `resolve_entrypoint(...)` seam, and D317 path-derived naming for non-root executable modules.
affects:
  - S04
  - S05
key_files:
  - compiler/mesh-lsp/src/analysis.rs
  - compiler/meshc/tests/e2e_lsp.rs
  - tools/editors/neovim-mesh/lua/mesh.lua
  - tools/editors/neovim-mesh/lsp/mesh.lua
  - tools/editors/neovim-mesh/tests/smoke.lua
  - tools/editors/neovim-mesh/README.md
  - scripts/tests/verify-m036-s02-contract.test.mjs
  - tools/editors/vscode-mesh/src/test/suite/extension.test.ts
  - tools/editors/vscode-mesh/src/test/runTest.ts
  - compiler/meshpkg/src/publish.rs
key_decisions:
  - Tooling roots now prefer the nearest `mesh.toml`, resolve the executable entry through `mesh_pkg::manifest::resolve_entrypoint(...)`, and only fall back to legacy `main.mpl` / `.git` / single-file behavior when no manifest root exists.
  - Non-root executable entries stay path-derived modules (for example `lib/start.mpl` -> `Lib.Start`) instead of gaining a second special `Main` naming rule.
  - `meshpkg publish` now mirrors Mesh source discovery from the project root and preserves project-root-relative archive paths instead of assuming a root-only executable or `src/` layout.
  - Live override-entry LSP transport proof uses hover for nested imports because the current cross-file `goto_definition` path is still limited to the open document.
patterns_established:
  - Use `mesh_pkg::manifest::resolve_entrypoint(...)` as the single executable-entry resolver for every project-aware tooling surface instead of re-encoding entrypoint logic per tool.
  - Materialize temporary override-entry fixtures (`mesh.toml`, `lib/start.mpl`, nested support module) when proving end-to-end tooling behavior; do not rely on root-`main.mpl` fixtures to stand in for the override contract.
  - Fail closed with explicit project diagnostics when manifest or entrypoint resolution breaks; do not silently fall back to isolated single-file analysis for project files.
  - Keep only root `main.mpl` special-cased as `Main`; non-root executable entries stay path-derived modules even when marked executable.
observability_surfaces:
  - `mesh-lsp` now emits project diagnostics when `mesh.toml` parsing or entrypoint resolution fails, instead of silently degrading to misleading single-file errors.
  - The live JSON-RPC and editor-host smokes assert both sides of the diagnostics contract: override-entry entry/support files open cleanly, and an intentional invalid `didChange` on `reference-backend/api/health.mpl` still produces a real diagnostic.
drill_down_paths:
  - .gsd/milestones/M048/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M048/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M048/slices/S02/tasks/T03-SUMMARY.md
  - .gsd/milestones/M048/slices/S02/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-02T09:34:52.881Z
blocker_discovered: false
---

# S02: Entrypoint-aware LSP, editors, and package surfaces

**Extended the S01 default-plus-override entrypoint contract through `mesh-lsp`, `meshc lsp`, Neovim/VS Code hosts, and `meshpkg publish` so non-`main.mpl` executables behave like first-class Mesh projects.**

## What Happened

S02 closed the remaining hardcoded `main.mpl` seams outside compiler/test discovery. `mesh-lsp` now discovers the nearest `mesh.toml` before any legacy root `main.mpl`, resolves the effective executable entry through the shared manifest resolver, preserves D317 path-derived naming for non-root entries, and reports explicit project diagnostics when manifest parsing or entrypoint resolution fails instead of silently dropping override-entry workspaces back to single-file analysis.

The live `meshc lsp` rail now proves that contract over real stdio JSON-RPC. An override-entry project rooted by `mesh.toml` + `lib/start.mpl` opens cleanly, its nested support module stays diagnostic-free, and hover over an imported nested helper returns real type information. The existing reference-backend proof still publishes clean diagnostics and still surfaces a deliberate invalid `didChange` as a real diagnostic, so the slice preserved the honest failure signal while extending project-aware analysis.

On the editor-host side, the repo-owned Neovim pack and VS Code smoke both now treat `mesh.toml` as the first-class root marker, materialize override-entry fixtures inside their proof harnesses, and assert the expected root/client behavior instead of only opening root-`main.mpl` projects. Neovim smoke proves manifest-first rooting, client reuse, and honest single-file mode; VS Code smoke proves clean diagnostics plus semantic-provider behavior on both reference-backend and override-entry projects.

On the package surface, `meshpkg publish` no longer assumes executable projects are root-only or `src/`-shaped. It walks the project root recursively, archives all non-hidden non-test Mesh source files under their project-root-relative paths, and keeps root `main.mpl` when it exists alongside an override entry. Downstream slices can now treat the default-plus-override entrypoint contract as shared tooling truth rather than compiler-only behavior.

## Verification

- `cargo test -p mesh-lsp -- --nocapture` ✅ passed (`61 passed`). Focused M048/S02 analysis tests covered manifest-first root detection, override-only and override-precedence entry handling, and explicit project diagnostics for missing/invalid entrypoints.
- `cargo test -p meshc --test e2e_lsp -- --nocapture` ✅ passed (`6 passed`). The live JSON-RPC rail opened override-entry entry/support files with zero diagnostics, returned a `String`-bearing hover for nested imports, preserved reference-backend definition support, and still surfaced a deliberate invalid `didChange` as a real diagnostic.
- `NEOVIM_BIN="${NEOVIM_BIN:-nvim}" bash scripts/verify-m036-s02.sh lsp && node --test scripts/tests/verify-m036-s02-contract.test.mjs && npm --prefix tools/editors/vscode-mesh run test:smoke` ✅ passed. Neovim proved manifest-first root markers plus honest single-file mode; the Node contract test kept the README/runtime/smoke assertions synchronized; VS Code smoke proved clean diagnostics, definition/hover behavior, and override-entry host coverage through the real Extension Development Host.
- `cargo test -p meshpkg -- --nocapture` ✅ passed (`8 passed`). Archive-member regressions confirmed recursive project-root discovery, hidden/test exclusions, and root-main retention when both root and override entries coexist.

## Requirements Advanced

- R112 — Extended the default-plus-override executable-entry contract from compiler/test discovery into `mesh-lsp`, `meshc lsp`, editor hosts, and published package archives so non-root entries behave like first-class projects across tooling surfaces.
- R114 — Advanced the editor-host half of the parity reset by making VS Code and Neovim root detection, diagnostics, and smoke coverage truthful for manifest-first override-entry projects ahead of the remaining syntax/skill work in S04/S05.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Local verification required a one-time `brew install neovim` because this host did not have `nvim`; no product code or proof contract changed.

## Known Limitations

Cross-file imported-call `goto_definition` is still not the truthful override-entry proof surface for `meshc lsp`; the current live server reliably proves nested-import typing through hover, but definition remains effectively open-document-local. S04 still owns syntax-highlighting parity and init-time skill refresh, so this slice only closes the host/rooting/discovery half of editor truth.

## Follow-ups

S04 still owns syntax-highlighting and init-skill parity, and S05 still needs the assembled closeout rail plus minimal public-surface updates. If imported cross-module `goto_definition` becomes part of the public override-entry contract, extend `mesh-lsp` beyond its current open-document-local definition path and add a transport-level regression for it.

## Files Created/Modified

- `compiler/mesh-lsp/src/analysis.rs` — Made project-aware LSP analysis prefer manifest roots, resolve executable entries through the shared resolver, preserve non-root entry module naming, and fail closed with project diagnostics plus focused regressions.
- `compiler/meshc/tests/e2e_lsp.rs` — Added live stdio JSON-RPC override-entry LSP coverage for clean diagnostics and nested-import hover proof.
- `tools/editors/neovim-mesh/lua/mesh.lua` — Switched Neovim root detection to manifest-first markers and kept binary discovery/root truth explicit.
- `tools/editors/neovim-mesh/lsp/mesh.lua` — Kept the exported native Neovim LSP config synchronized with the shared manifest-first root markers.
- `tools/editors/neovim-mesh/tests/smoke.lua` — Extended the Neovim smoke runner with a materialized override-entry project, root-marker assertions, client reuse checks, and honest single-file coverage.
- `tools/editors/neovim-mesh/README.md` — Updated the Neovim README to document the manifest-first root contract and the bounded verifier surface.
- `scripts/tests/verify-m036-s02-contract.test.mjs` — Pinned the Neovim README/runtime/smoke contract in a repo-local Node test.
- `tools/editors/vscode-mesh/src/test/suite/extension.test.ts` — Extended the VS Code smoke suite to materialize an override-entry project, wait for clean diagnostics, and prove semantic providers against both reference-backend and override-entry fixtures.
- `tools/editors/vscode-mesh/src/test/runTest.ts` — Kept the VS Code smoke harness on a repo-root workspace while injecting the override-entry fixture inside that workspace for truthful root detection.
- `compiler/meshpkg/src/publish.rs` — Replaced root-only/src-only publish member discovery with a recursive project-root walk that preserves relative paths and excludes hidden/test-only Mesh sources.
