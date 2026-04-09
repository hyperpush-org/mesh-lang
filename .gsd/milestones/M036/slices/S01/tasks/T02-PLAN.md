---
estimated_steps: 7
estimated_files: 4
skills_used:
  - test
---

# T02: Repair interpolation handling in the shared grammar and public truth surfaces

**Slice:** S01 — Corpus-backed syntax parity for the shared VS Code/docs surface
**Milestone:** M036

## Description

Use the corpus proof from T01 to make the shared VS Code/docs grammar truthful, then align the outward-facing editor/docs surfaces with exactly what the verifier covers. This keeps the fix boundary small: one shared grammar, one proof bundle, no duplicated editor-specific logic.

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

1. Update `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json` so both interpolation syntaxes emit the same scopes in quoted and triple-quoted strings without regressing escapes or comment boundaries.
2. Touch only the docs/editor truth surfaces that actually drift under the corpus proof, keeping both VS Code and docs anchored to the same shared grammar contract.
3. Rerun the slice verifier and leave the repo with a green parity contract plus user-facing breadcrumbs for the shipped fix.

## Must-Haves

- [ ] `#{...}` and `${...}` both highlight correctly through the shared grammar in double- and triple-quoted strings.
- [ ] The docs and extension surfaces do not claim syntax behavior outside the verified corpus contract.
- [ ] The slice verifier passes end-to-end from repo root.

## Verification

- `bash scripts/verify-m036-s01.sh`
- `cargo test -p mesh-lexer string_interpolation -- --nocapture`

## Inputs

- `scripts/fixtures/m036-s01-syntax-corpus.json` — corpus cases that define the slice proof surface.
- `scripts/fixtures/m036-s01/interpolation_edge_cases.mpl` — edge-case fixture added in T01.
- `website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs` — parity test that must go green after the fix.
- `scripts/verify-m036-s01.sh` — repo-root proof entrypoint that must stay green.
- `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json` — shared grammar to repair.
- `tools/editors/vscode-mesh/README.md` — extension-facing public truth surface for shipped syntax support.
- `tools/editors/vscode-mesh/CHANGELOG.md` — release-facing breadcrumb for the shipped fix.
- `website/docs/docs/tooling/index.md` — docs-facing public truth surface for the shared editor story.

## Expected Output

- `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json` — repaired shared interpolation rules.
- `tools/editors/vscode-mesh/README.md` — extension docs aligned with the verified grammar contract.
- `tools/editors/vscode-mesh/CHANGELOG.md` — release note for the corpus-backed interpolation repair.
- `website/docs/docs/tooling/index.md` — public tooling docs aligned with the verified shared-surface proof.
