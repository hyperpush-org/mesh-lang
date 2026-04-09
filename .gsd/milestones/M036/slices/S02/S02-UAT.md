# S02: Repo-owned first-class Neovim support pack — UAT

**Milestone:** M036
**Written:** 2026-03-28T06:10:44.434Z

# S02: Repo-owned first-class Neovim support pack — UAT

**Milestone:** M036
**Written:** 2026-03-28T23:49:09-04:00

## UAT: Repo-owned first-class Neovim support pack

### Preconditions
- Repo checkout contains the S02 changes.
- Rust workspace builds successfully and `target/debug/meshc` exists, or an explicit `meshc` override path is available.
- A Neovim 0.11+ binary is available, either on `PATH` or via `NEOVIM_BIN`.
- The repo can write temporary verifier artifacts under `.tmp/m036-s02/`.

### Test 1: Full repo-owned Neovim verifier passes end to end
1. Run `NEOVIM_BIN="${NEOVIM_BIN:-nvim}" bash scripts/verify-m036-s02.sh` from the repo root.
2. Observe the phase banners and final Neovim smoke output.

**Expected:**
- The verifier runs `corpus`, `shared-grammar`, `upstream-lsp`, and `neovim` in that order.
- The `corpus` phase materializes 15 named cases into `.tmp/m036-s02/all/corpus/cases/*.mpl` and writes `.tmp/m036-s02/all/corpus/materialized-corpus.json`.
- The `shared-grammar` phase replays `bash scripts/verify-m036-s01.sh` successfully.
- The `upstream-lsp` phase replays `cargo test -q -p meshc --test e2e_lsp -- --nocapture` successfully.
- The `neovim` phase installs the pack through `.tmp/m036-s02/all/site/pack/mesh/start/mesh-nvim`, passes both syntax and LSP checks, and exits 0.

### Test 2: Pack-local install path is sufficient for `*.mpl` filetype detection and classic syntax
1. Create a local Neovim package install path, for example:
   ```bash
   mkdir -p "${XDG_DATA_HOME:-$HOME/.local/share}/nvim/site/pack/mesh/start"
   ln -sfn \
     "$PWD/tools/editors/neovim-mesh" \
     "${XDG_DATA_HOME:-$HOME/.local/share}/nvim/site/pack/mesh/start/mesh-nvim"
   ```
2. Open a Mesh file with Neovim 0.11+, for example `reference-backend/main.mpl`.
3. Run `:set filetype?` and `:echo exists('b:current_syntax') ? b:current_syntax : ''`.
4. Inspect a known interpolation line such as the `#{port}` startup log in `reference-backend/main.mpl`.

**Expected:**
- `:set filetype?` reports `filetype=mesh`.
- `b:current_syntax` reports `mesh`.
- Double-quoted and triple-quoted interpolation markers render with the Mesh interpolation delimiter group, while plain strings remain plain strings.
- The pack works from the native `pack/*/start/mesh-nvim` runtime path without `nvim-lspconfig` or a plugin manager.

### Test 3: Syntax smoke proves both positive interpolation cases and negative plain-string controls
1. Run `NEOVIM_BIN="${NEOVIM_BIN:-nvim}" bash scripts/verify-m036-s02.sh syntax`.
2. Inspect the emitted syntax-case lines.

**Expected:**
- The smoke confirms `filetype=mesh` and `syntax=mesh` for every checked corpus case.
- Positive cases include both `#{...}` and `${...}` across real repo files, docs-backed snippets, and the nested-brace edge fixture.
- Negative controls such as `edge-plain-double-no-interpolation` and `fixture-no-interpolation` report `probe=plain-string` and stay on `meshStringDouble` / `String`, not an interpolation group.
- Failures, if any, name the exact case id, source path, line/column, and emitted syntax stack.

### Test 4: Native Neovim LSP bootstrap attaches on rooted project files and honest single-file mode
1. Run `NEOVIM_BIN="${NEOVIM_BIN:-nvim}" bash scripts/verify-m036-s02.sh lsp`.
2. Inspect the emitted `phase=lsp` lines.

**Expected:**
- The first check is a negative missing-override proof that passes only if a bad explicit `meshc` path fails loudly.
- `reference-backend/api/health.mpl` and `reference-backend/api/jobs.mpl` attach a `mesh` client with `marker=main.mpl` and `root=reference-backend`.
- The resolved `meshc` path is reported, preferring the repo-local `target/debug/meshc` when available.
- A standalone temporary `.mpl` file still attaches with `marker=single-file` and `root=<none>` instead of inventing a fake workspace root.

### Test 5: README documents the exact support contract and verifier usage
1. Open `tools/editors/neovim-mesh/README.md`.
2. Review the install, `meshc` resolution, and verification sections.

**Expected:**
- The README states the Neovim 0.11+ floor.
- The install section shows the exact `pack/*/start/mesh-nvim` path shape.
- The `meshc` section documents both explicit override knobs and the ordered discovery behavior.
- The verification section points to `NEOVIM_BIN="${NEOVIM_BIN:-nvim}" bash scripts/verify-m036-s02.sh` and explains the named phases it runs.
- The README does not claim Tree-sitter support, plugin-manager-specific setup, or broader public support tiers than this slice proves.

### Edge Cases
- If `NEOVIM_BIN` points to a missing or too-old binary, the verifier must fail during `preflight` with a clear missing-binary or minimum-version error.
- If an explicit `meshc` override points at a missing file, the LSP smoke must fail loudly instead of silently falling back to another candidate.
- Docs-backed syntax corpus cases must be materialized into temporary `.mpl` snippets first; raw markdown files must never be opened directly as Mesh buffers during proof.
- If future syntax work broadens highlighting claims, the shared S01 corpus and Neovim smoke must expand together before the README or support messaging is widened.
