---
id: T03
parent: S02
milestone: M036
provides: []
requires: []
affects: []
key_files: ["scripts/tests/verify-m036-s02-materialize-corpus.mjs", "scripts/tests/verify-m036-s02-materialize-corpus.test.mjs", "scripts/tests/verify-m036-s02-contract.test.mjs", "tools/editors/neovim-mesh/tests/smoke.lua", "tools/editors/neovim-mesh/tests/syntax_smoke.lua", "tools/editors/neovim-mesh/tests/lsp_smoke.lua", "tools/editors/neovim-mesh/README.md", "scripts/verify-m036-s02.sh", ".gsd/DECISIONS.md", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Materialize every shared syntax corpus case into a temporary `.mpl` snippet and make one consolidated `smoke.lua` runner consume only that generated manifest, so docs-backed markdown ranges fail closed instead of being opened as raw markdown buffers in Neovim."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the new helper and documentation contract with Node tests, then ran `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh syntax`, `... bash scripts/verify-m036-s02.sh lsp`, and the full `... bash scripts/verify-m036-s02.sh`. The full verifier exited 0 only after materializing all 15 shared corpus cases, replaying the upstream S01 grammar proof, replaying the upstream `meshc lsp` transport proof, and passing the consolidated Neovim syntax and LSP smoke against the real package-runtime install path."
completed_at: 2026-03-28T06:06:20.512Z
blocker_discovered: false
---

# T03: Proved the Neovim pack against the shared corpus, unified the headless verifier, and documented the pack-local install contract.

> Proved the Neovim pack against the shared corpus, unified the headless verifier, and documented the pack-local install contract.

## What Happened
---
id: T03
parent: S02
milestone: M036
key_files:
  - scripts/tests/verify-m036-s02-materialize-corpus.mjs
  - scripts/tests/verify-m036-s02-materialize-corpus.test.mjs
  - scripts/tests/verify-m036-s02-contract.test.mjs
  - tools/editors/neovim-mesh/tests/smoke.lua
  - tools/editors/neovim-mesh/tests/syntax_smoke.lua
  - tools/editors/neovim-mesh/tests/lsp_smoke.lua
  - tools/editors/neovim-mesh/README.md
  - scripts/verify-m036-s02.sh
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Materialize every shared syntax corpus case into a temporary `.mpl` snippet and make one consolidated `smoke.lua` runner consume only that generated manifest, so docs-backed markdown ranges fail closed instead of being opened as raw markdown buffers in Neovim.
duration: ""
verification_result: passed
completed_at: 2026-03-28T06:06:20.512Z
blocker_discovered: false
---

# T03: Proved the Neovim pack against the shared corpus, unified the headless verifier, and documented the pack-local install contract.

**Proved the Neovim pack against the shared corpus, unified the headless verifier, and documented the pack-local install contract.**

## What Happened

Added a corpus materializer that turns the shared S01 interpolation corpus into per-case temporary `.mpl` snippets, including markdown-backed docs ranges that now fail closed unless they come from a `mesh`/`mpl` fenced block. Consolidated the split Neovim syntax and LSP proofs into one `tools/editors/neovim-mesh/tests/smoke.lua` runner, kept the old phase-specific Lua scripts as thin wrappers, and rewrote `scripts/verify-m036-s02.sh` so it defaults to the full repo proof: materialize corpus, replay `scripts/verify-m036-s01.sh`, replay `cargo test -q -p meshc --test e2e_lsp -- --nocapture`, then run the real package-runtime Neovim smoke through `pack/*/start/mesh-nvim`. Wrote `tools/editors/neovim-mesh/README.md` with the exact install path, Neovim 0.11+ floor, `meshc` discovery/override rules, and the exact verification command, then recorded the durable verification rule in `.gsd/DECISIONS.md` and `.gsd/KNOWLEDGE.md`.

## Verification

Verified the new helper and documentation contract with Node tests, then ran `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh syntax`, `... bash scripts/verify-m036-s02.sh lsp`, and the full `... bash scripts/verify-m036-s02.sh`. The full verifier exited 0 only after materializing all 15 shared corpus cases, replaying the upstream S01 grammar proof, replaying the upstream `meshc lsp` transport proof, and passing the consolidated Neovim syntax and LSP smoke against the real package-runtime install path.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m036-s02-materialize-corpus.test.mjs scripts/tests/verify-m036-s02-contract.test.mjs` | 0 | ✅ pass | 254ms |
| 2 | `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh syntax` | 0 | ✅ pass | 564ms |
| 3 | `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh lsp` | 0 | ✅ pass | 5091ms |
| 4 | `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh` | 0 | ✅ pass | 5886ms |


## Deviations

Added two small repo tests for the materializer and README/verifier contract beyond the plan’s minimum file list so the corpus-materialization rule and install-path docs stay mechanically enforced.

## Known Issues

None.

## Files Created/Modified

- `scripts/tests/verify-m036-s02-materialize-corpus.mjs`
- `scripts/tests/verify-m036-s02-materialize-corpus.test.mjs`
- `scripts/tests/verify-m036-s02-contract.test.mjs`
- `tools/editors/neovim-mesh/tests/smoke.lua`
- `tools/editors/neovim-mesh/tests/syntax_smoke.lua`
- `tools/editors/neovim-mesh/tests/lsp_smoke.lua`
- `tools/editors/neovim-mesh/README.md`
- `scripts/verify-m036-s02.sh`
- `.gsd/DECISIONS.md`
- `.gsd/KNOWLEDGE.md`


## Deviations
Added two small repo tests for the materializer and README/verifier contract beyond the plan’s minimum file list so the corpus-materialization rule and install-path docs stay mechanically enforced.

## Known Issues
None.
