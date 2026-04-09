# S01: Corpus-backed syntax parity for the shared VS Code/docs surface

**Goal:** Ship a real shared-surface parity repair for VS Code and docs by grounding the TextMate grammar and adjacent truth surfaces in an audited non-toy Mesh corpus.
**Demo:** After this: Open representative Mesh files in VS Code and the docs highlighter and see `#{...}` plus `${...}` handled according to compiler truth, with parity checks pinpointing any corpus sample that regresses.

## Tasks
- [x] **T01: Added an audited syntax corpus plus a fail-closed TextMate/Shiki parity verifier that reproduces the current interpolation drift with named engine/file/case output.** — Build the proof harness before touching the grammar so the executor can reproduce the current drift on real Mesh code and keep the repair honest.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/mesh-lexer` string-interpolation truth | Stop and identify whether compiler truth or the highlighting layer is wrong before changing regexes. | Local cargo test should not hang; if it does, fail the task and capture the blocked command. | Treat unexpected token shapes as contract drift and update the manifest only with evidence. |
| Shiki/TextMate loaders from `website/node_modules` | Fail fast with a clear missing-dependency message instead of silently skipping one engine. | Fail the test process and report which engine stalled. | Reject the corpus entry and print the file/case id that could not be tokenized. |

## Load Profile

- **Shared resources**: local filesystem reads for a small corpus plus one Node test process.
- **Per-operation cost**: one grammar load per engine and one tokenization/render pass per corpus case.
- **10x breakpoint**: runtime grows linearly with corpus size, but the repo-sized corpus should stay well under normal CI limits.

## Negative Tests

- **Malformed inputs**: missing corpus file paths, empty snippet selections, and cases without either interpolation form must fail with a named corpus entry.
- **Error paths**: break the shared grammar import path or remove an engine dependency and ensure the verifier fails closed instead of silently passing one surface.
- **Boundary conditions**: cover double-quoted strings, triple-quoted strings, `#{...}`, `${...}`, nested braces inside interpolation, and strings without interpolation so false positives stay visible.

## Steps

1. Create a machine-readable corpus manifest that points at representative repo-owned Mesh sources and any minimal extra fixture needed for uncovered interpolation edges.
2. Add a Node test under `website/scripts/tests/` that loads the shared grammar into TextMate and Shiki, runs every corpus case, and reports engine/file/case diffs.
3. Add a repo-root verifier script that replays compiler lexer truth plus the parity test so future slices have one entrypoint.

## Must-Haves

- [ ] The corpus references real repo Mesh files plus only the smallest extra fixture needed for uncovered interpolation edges.
- [ ] The parity test exercises both VS Code/TextMate tokenization and docs/Shiki rendering against the same shared grammar file.
- [ ] Failures name the engine, corpus file, and case so drift is localizable in one rerun.
  - Estimate: 1.5h
  - Files: scripts/fixtures/m036-s01-syntax-corpus.json, scripts/fixtures/m036-s01/interpolation_edge_cases.mpl, website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs, scripts/verify-m036-s01.sh
  - Verify: bash scripts/verify-m036-s01.sh
- [x] **T02: Repaired shared TextMate interpolation parity for `#{...}` and `${...}` and aligned the VS Code/docs syntax contract to the verified corpus.** — Use the corpus proof to make the shared VS Code/docs grammar truthful, then align the outward-facing syntax surfaces with exactly what the verifier covers.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json` interpolation rules | Adjust only the shared grammar and rerun the corpus proof before widening the edit set. | Static-file work has no expected timeout; if verification hangs, fail closed and inspect the verifier. | Reject scope changes that make TextMate and Shiki disagree on interpolation boundaries. |
| Public syntax docs / editor README surfaces | Keep claims at the proven contract if wording or examples drift from what the verifier covers. | N/A | Treat contradictory docs as a failing contract and align them with the verified behavior. |

## Load Profile

- **Shared resources**: one shared grammar file consumed by VS Code and VitePress.
- **Per-operation cost**: static regex matching during highlighting; no network or persistent state.
- **10x breakpoint**: a wider corpus mainly increases verifier time, not runtime cost, so correctness drift is the primary risk.

## Negative Tests

- **Malformed inputs**: strings near `#` comments must not mis-tokenize `#{...}` as a comment boundary.
- **Error paths**: broken interpolation begin/end scopes or broken docs imports must fail the parity verifier and wrapper script.
- **Boundary conditions**: mixed `#{...}` and `${...}` in the same file, triple-quoted strings, and plain strings without interpolation must all stay green.

## Steps

1. Update the shared TextMate string rules so both interpolation syntaxes emit the same scopes in quoted and triple-quoted strings without regressing escapes or comments.
2. Touch only the docs/editor truth surfaces that actually drift under the corpus proof, keeping the VitePress and VS Code paths anchored to the same grammar file.
3. Rerun the slice verifier and leave the repo with a green parity contract plus user-facing breadcrumbs for the shipped fix.

## Must-Haves

- [ ] `#{...}` and `${...}` both highlight correctly through the shared grammar in double- and triple-quoted strings.
- [ ] The docs and extension surfaces do not claim syntax behavior outside the verified corpus contract.
- [ ] The slice verifier passes end-to-end from repo root.
  - Estimate: 1h
  - Files: tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json, tools/editors/vscode-mesh/README.md, tools/editors/vscode-mesh/CHANGELOG.md, website/docs/docs/tooling/index.md
  - Verify: bash scripts/verify-m036-s01.sh
