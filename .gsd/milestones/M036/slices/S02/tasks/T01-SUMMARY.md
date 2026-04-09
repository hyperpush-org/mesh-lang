---
id: T01
parent: S02
milestone: M036
provides: []
requires: []
affects: []
key_files: ["tools/editors/neovim-mesh/ftdetect/mesh.vim", "tools/editors/neovim-mesh/syntax/mesh.vim", "tools/editors/neovim-mesh/tests/syntax_smoke.lua", "scripts/verify-m036-s02.sh", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Override Vim/Neovim's built-in Maple `*.mpl` detection with `set filetype=mesh` so the pack wins consistently.", "Keep the classic regex-literal matcher intentionally narrower than the TextMate grammar when the broader Vim regex form becomes unstable."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the task-specific Neovim syntax smoke with a repo-local Neovim 0.11.6 binary, then reran the slice-level upstream checks. `bash scripts/verify-m036-s01.sh` passed, `cargo test -q -p meshc --test e2e_lsp -- --nocapture` passed, `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh syntax` passed, and the default `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh` entrypoint also passed."
completed_at: 2026-03-28T05:40:50.109Z
blocker_discovered: false
---

# T01: Added an installable Neovim Mesh pack with forced `*.mpl` filetype detection, classic syntax highlighting, and a headless syntax verifier.

> Added an installable Neovim Mesh pack with forced `*.mpl` filetype detection, classic syntax highlighting, and a headless syntax verifier.

## What Happened
---
id: T01
parent: S02
milestone: M036
key_files:
  - tools/editors/neovim-mesh/ftdetect/mesh.vim
  - tools/editors/neovim-mesh/syntax/mesh.vim
  - tools/editors/neovim-mesh/tests/syntax_smoke.lua
  - scripts/verify-m036-s02.sh
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Override Vim/Neovim's built-in Maple `*.mpl` detection with `set filetype=mesh` so the pack wins consistently.
  - Keep the classic regex-literal matcher intentionally narrower than the TextMate grammar when the broader Vim regex form becomes unstable.
duration: ""
verification_result: passed
completed_at: 2026-03-28T05:40:50.110Z
blocker_discovered: false
---

# T01: Added an installable Neovim Mesh pack with forced `*.mpl` filetype detection, classic syntax highlighting, and a headless syntax verifier.

**Added an installable Neovim Mesh pack with forced `*.mpl` filetype detection, classic syntax highlighting, and a headless syntax verifier.**

## What Happened

Created the initial repo-owned Neovim pack surface under `tools/editors/neovim-mesh/` with native filetype detection and a bounded classic Vim syntax file aligned to the audited S01 interpolation contract. The filetype detector intentionally forces `filetype=mesh` because Vim/Neovim's built-in Maple mapping for `*.mpl` otherwise wins. Added a headless `tools/editors/neovim-mesh/tests/syntax_smoke.lua` script plus the first `syntax` phase in `scripts/verify-m036-s02.sh`; the verifier installs the pack through a real `pack/*/start/mesh-nvim` symlink under `.tmp`, preflights Neovim, opens corpus-backed `.mpl` fixtures, asserts `filetype=mesh` and `b:current_syntax == 'mesh'`, and prints named case/position syntax-group evidence for interpolation and plain-string probes. For local proof, used a repo-local portable Neovim binary under `.tmp/m036-s02/vendor/` because no system `nvim` was installed.

## Verification

Passed the task-specific Neovim syntax smoke with a repo-local Neovim 0.11.6 binary, then reran the slice-level upstream checks. `bash scripts/verify-m036-s01.sh` passed, `cargo test -q -p meshc --test e2e_lsp -- --nocapture` passed, `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh syntax` passed, and the default `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh` entrypoint also passed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash scripts/verify-m036-s01.sh` | 0 | ✅ pass | 680ms |
| 2 | `cargo test -q -p meshc --test e2e_lsp -- --nocapture` | 0 | ✅ pass | 4573ms |
| 3 | `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh syntax` | 0 | ✅ pass | 340ms |
| 4 | `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh` | 0 | ✅ pass | 295ms |


## Deviations

Used a repo-local portable Neovim binary under `.tmp/m036-s02/vendor/` for verification because this environment did not have `nvim` installed. No product/runtime contract changed as a result.

## Known Issues

None.

## Files Created/Modified

- `tools/editors/neovim-mesh/ftdetect/mesh.vim`
- `tools/editors/neovim-mesh/syntax/mesh.vim`
- `tools/editors/neovim-mesh/tests/syntax_smoke.lua`
- `scripts/verify-m036-s02.sh`
- `.gsd/KNOWLEDGE.md`


## Deviations
Used a repo-local portable Neovim binary under `.tmp/m036-s02/vendor/` for verification because this environment did not have `nvim` installed. No product/runtime contract changed as a result.

## Known Issues
None.
