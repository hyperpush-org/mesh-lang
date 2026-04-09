---
estimated_steps: 4
estimated_files: 4
skills_used:
  - test
  - debug-like-expert
---

# T03: Prove the pack against the shared corpus and document the first-class install path

**Slice:** S02 — Repo-owned first-class Neovim support pack
**Milestone:** M036

## Description

Close the slice with proof and install docs: reuse the S01 corpus instead of duplicating examples, wrap the headless phases in one repo-root verifier, and publish the exact pack-local install contract.

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

## Verification

- `NEOVIM_BIN="${NEOVIM_BIN:-nvim}" bash scripts/verify-m036-s02.sh`
- The command exits 0 only after replaying S01 grammar proof, `meshc lsp` transport proof, and the headless Neovim corpus/install smoke in distinct named phases.

## Observability Impact

- Signals added/changed: the final wrapper should emit named phases, temp corpus artifact locations, and explicit syntax/LSP assertion results.
- How a future agent inspects this: run `NEOVIM_BIN=... bash scripts/verify-m036-s02.sh` and inspect the phase banners plus `.tmp/m036-s02/` artifacts.
- Failure state exposed: missing Neovim, missing `meshc`, bad corpus materialization, syntax drift, or attach failures become phase-localized instead of collapsing into one opaque command failure.

## Inputs

- `scripts/fixtures/m036-s01-syntax-corpus.json` — audited corpus manifest to reuse instead of creating a separate Neovim-only example list.
- `scripts/verify-m036-s01.sh` — upstream shared-grammar proof the final wrapper must replay.
- `compiler/meshc/tests/e2e_lsp.rs` — upstream transport proof the final wrapper must replay before claiming Neovim support.
- `tools/editors/neovim-mesh/tests/syntax_smoke.lua` — earlier syntax proof surface to fold into the final runner.
- `tools/editors/neovim-mesh/tests/lsp_smoke.lua` — earlier LSP proof surface to fold into the final runner.
- `scripts/verify-m036-s02.sh` — wrapper entrypoint to finalize into the slice-level verifier.

## Expected Output

- `scripts/tests/verify-m036-s02-materialize-corpus.mjs` — helper that materializes markdown-backed corpus cases into temporary `.mpl` files.
- `tools/editors/neovim-mesh/tests/smoke.lua` — final headless Neovim runner covering syntax and LSP assertions.
- `tools/editors/neovim-mesh/README.md` — pack-local install, support-boundary, override, and verification contract.
- `scripts/verify-m036-s02.sh` — final repo-root verifier for the Neovim support pack.
