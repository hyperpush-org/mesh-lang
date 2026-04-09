---
estimated_steps: 4
estimated_files: 4
skills_used:
  - test
  - debug-like-expert
---

# T01: Add the installable Neovim pack surface for Mesh filetype and classic syntax

**Slice:** S02 — Repo-owned first-class Neovim support pack
**Milestone:** M036

## Description

Implement the smallest truthful editor surface first: opening `*.mpl` in a stock Neovim package install should identify the buffer as Mesh and highlight the audited language forms before any LSP wiring is involved.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json` and the S01 interpolation contract | Keep the Neovim syntax narrower rather than inventing unsupported groups; fail the smoke if `#{...}` / `${...}` or plain-string cases drift. | Not applicable — local files only. | Treat unexpected interpolation or string-kind cases as unsupported and surface the failing case id/position. |
| `NEOVIM_BIN` / headless Neovim | Fail fast with a clear missing-binary or unsupported-version message. | Abort the syntax smoke and print the hanging command/phase. | Treat missing syntax APIs or zero syntax groups as failure, not as a silent pass. |

## Load Profile

- **Shared resources**: Headless Neovim runtimepath and syntax engine only.
- **Per-operation cost**: Opens one representative Mesh buffer at a time and checks a handful of syntax positions; trivial.
- **10x breakpoint**: Corpus size/debuggability, not compute; failures should stay localized by case id instead of degenerating into whole-file noise.

## Negative Tests

- **Malformed inputs**: Plain quoted strings and explicit no-interpolation cases must not receive interpolation groups.
- **Error paths**: Missing `ftdetect` or unloaded syntax files must fail the smoke instead of silently falling back to `text`.
- **Boundary conditions**: Double-quoted, triple-quoted, `#{...}`, `${...}`, and nested-brace interpolation all remain distinguishable.

## Steps

1. Create `tools/editors/neovim-mesh/ftdetect/mesh.vim` so `*.mpl` resolves to filetype `mesh` through native runtime discovery.
2. Implement `tools/editors/neovim-mesh/syntax/mesh.vim` as a bounded classic Vim syntax file covering the audited Mesh token classes from S01: comments, strings/escapes, interpolation, atoms, regex, numbers, keywords, types, module-qualified calls, functions, variables, and operators.
3. Add `tools/editors/neovim-mesh/tests/syntax_smoke.lua` plus the first `syntax` phase in `scripts/verify-m036-s02.sh` to open representative Mesh files headlessly, assert `&filetype == 'mesh'`, and inspect syntax groups at known interpolation/plain-string positions.
4. Keep the implementation fail-closed: if Neovim cannot distinguish a promised token class, the smoke output should name the failing file/case/position rather than passing optimistically.

## Must-Haves

- [ ] Installing the pack alone is enough for `*.mpl` buffers to become `mesh`.
- [ ] Syntax highlighting stays honest to the S01 interpolation contract instead of inventing Tree-sitter or broader claims.
- [ ] The syntax smoke proves both positive interpolation cases and negative plain-string cases in headless Neovim.

## Verification

- `NEOVIM_BIN="${NEOVIM_BIN:-nvim}" bash scripts/verify-m036-s02.sh syntax`
- The command exits 0 only after reporting the checked filetype and syntax-group assertions for representative interpolation and no-interpolation cases.

## Observability Impact

- Signals added/changed: the syntax smoke should emit the checked file/case id, resolved buffer filetype, and inspected syntax groups.
- How a future agent inspects this: run `NEOVIM_BIN=... bash scripts/verify-m036-s02.sh syntax` and read the named case output.
- Failure state exposed: missing filetype detection, unloaded syntax, or interpolation drift becomes visible as a named case/position failure instead of a generic non-zero exit.

## Inputs

- `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json` — shared token-class contract to mirror honestly where Neovim can support it.
- `scripts/fixtures/m036-s01-syntax-corpus.json` — audited syntax cases to reuse for representative Neovim smoke coverage.
- `scripts/fixtures/m036-s01/interpolation_edge_cases.mpl` — explicit interpolation edge fixture covering nested braces and plain-string negatives.

## Expected Output

- `tools/editors/neovim-mesh/ftdetect/mesh.vim` — native package filetype detection for `*.mpl`.
- `tools/editors/neovim-mesh/syntax/mesh.vim` — bounded classic Vim syntax file aligned to the audited Mesh contract.
- `tools/editors/neovim-mesh/tests/syntax_smoke.lua` — headless syntax/filetype smoke script for the pack.
- `scripts/verify-m036-s02.sh` — verifier entrypoint with an initial `syntax` phase.
