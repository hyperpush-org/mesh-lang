---
id: T02
parent: S02
milestone: M048
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/e2e_lsp.rs", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M048/slices/S02/tasks/T02-SUMMARY.md"]
key_decisions: ["Use `textDocument/hover` for the cross-file override-entry semantic proof because the live `goto_definition` path is still intra-document only even when project analysis loads the nested import graph correctly.", "Persist per-session JSON-RPC trace and stderr artifacts under `.tmp/m048-s02-lsp/...` so startup, diagnostics, and provider drift stay inspectable from the retained rail."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran `cargo test -p meshc --test e2e_lsp -- --nocapture`, which passed all six tests including the new override-entry fixture guards, malformed-payload regression, and live override-entry diagnostics + hover proof. Then ran `NEOVIM_BIN="${NEOVIM_BIN:-nvim}" bash scripts/verify-m036-s02.sh lsp`; its `upstream-lsp` phase reran the updated `e2e_lsp` rail successfully and showed the override-entry proof in stdout, but the wrapper exited non-zero afterward at Neovim preflight because `nvim` is not installed in this environment."
completed_at: 2026-04-02T08:55:21.025Z
blocker_discovered: false
---

# T02: Extended the retained `meshc lsp` JSON-RPC rail to prove override-entry projects through live diagnostics and nested-import hover.

> Extended the retained `meshc lsp` JSON-RPC rail to prove override-entry projects through live diagnostics and nested-import hover.

## What Happened
---
id: T02
parent: S02
milestone: M048
key_files:
  - compiler/meshc/tests/e2e_lsp.rs
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M048/slices/S02/tasks/T02-SUMMARY.md
key_decisions:
  - Use `textDocument/hover` for the cross-file override-entry semantic proof because the live `goto_definition` path is still intra-document only even when project analysis loads the nested import graph correctly.
  - Persist per-session JSON-RPC trace and stderr artifacts under `.tmp/m048-s02-lsp/...` so startup, diagnostics, and provider drift stay inspectable from the retained rail.
duration: ""
verification_result: mixed
completed_at: 2026-04-02T08:55:21.025Z
blocker_discovered: false
---

# T02: Extended the retained `meshc lsp` JSON-RPC rail to prove override-entry projects through live diagnostics and nested-import hover.

**Extended the retained `meshc lsp` JSON-RPC rail to prove override-entry projects through live diagnostics and nested-import hover.**

## What Happened

Extended `compiler/meshc/tests/e2e_lsp.rs` so the retained `meshc lsp` JSON-RPC rail now covers a manifest-selected override-entry temp project with `mesh.toml`, `lib/start.mpl`, and a nested support module. Added fail-closed fixture materialization guards for missing entry/support files and invalid relative paths, a malformed JSON-RPC parser regression, and stronger LSP session observability via archived request/response traces and stderr logs under `.tmp/m048-s02-lsp/...`. The live override-entry proof initializes `meshc lsp`, opens the override entry and nested support module, asserts clean diagnostics for both, and exercises a semantic provider over the nested import graph. After confirming that the current live `goto_definition` implementation is still intra-document only, I switched the cross-file transport proof to `textDocument/hover`, which successfully proves imported nested-module typing through the real server path. I also recorded that gotcha in `.gsd/KNOWLEDGE.md` for future agents.

## Verification

Ran `cargo test -p meshc --test e2e_lsp -- --nocapture`, which passed all six tests including the new override-entry fixture guards, malformed-payload regression, and live override-entry diagnostics + hover proof. Then ran `NEOVIM_BIN="${NEOVIM_BIN:-nvim}" bash scripts/verify-m036-s02.sh lsp`; its `upstream-lsp` phase reran the updated `e2e_lsp` rail successfully and showed the override-entry proof in stdout, but the wrapper exited non-zero afterward at Neovim preflight because `nvim` is not installed in this environment.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_lsp -- --nocapture` | 0 | ✅ pass | 10067ms |
| 2 | `NEOVIM_BIN="${NEOVIM_BIN:-nvim}" bash scripts/verify-m036-s02.sh lsp` | 1 | ❌ fail | 10587ms |


## Deviations

None.

## Known Issues

`scripts/verify-m036-s02.sh lsp` still fails after the upstream-LSP phase because this environment does not have a local `nvim` binary for the Neovim smoke preflight.

## Files Created/Modified

- `compiler/meshc/tests/e2e_lsp.rs`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M048/slices/S02/tasks/T02-SUMMARY.md`


## Deviations
None.

## Known Issues
`scripts/verify-m036-s02.sh lsp` still fails after the upstream-LSP phase because this environment does not have a local `nvim` binary for the Neovim smoke preflight.
