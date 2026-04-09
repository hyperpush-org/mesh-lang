---
estimated_steps: 8
estimated_files: 4
skills_used:
  - test
---

# T01: Codify the Mesh syntax corpus and shared-surface parity verifier

**Slice:** S01 — Corpus-backed syntax parity for the shared VS Code/docs surface
**Milestone:** M036

## Description

Build the proof harness before touching the grammar so execution can reproduce the current drift on real Mesh code and keep the repair honest. This task turns the slice risk into a bounded contract: compiler truth, shared grammar loading, and docs/VS Code parity must all agree on the same corpus.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/mesh-lexer/src/lib.rs` string-interpolation truth | Stop and determine whether compiler truth or the highlighting layer is wrong before changing regexes. | Local cargo test should not hang; if it does, fail the task and capture the blocked command. | Treat unexpected token shapes as contract drift and update the corpus only with evidence. |
| Shiki/TextMate loaders from `website/node_modules` | Fail fast with a clear missing-dependency message instead of silently skipping one engine. | Fail the test process and report which engine stalled. | Reject the corpus entry and print the file/case id that could not be tokenized. |

## Load Profile

- **Shared resources**: local filesystem reads for a small corpus plus one Node test process.
- **Per-operation cost**: one grammar load per engine and one tokenization/render pass per corpus case.
- **10x breakpoint**: runtime grows linearly with corpus size, but a repo-sized corpus should stay well under normal CI limits.

## Negative Tests

- **Malformed inputs**: missing corpus file paths, empty snippet selections, and cases without either interpolation form must fail with a named corpus entry.
- **Error paths**: break the shared grammar import path or remove an engine dependency and ensure the verifier fails closed instead of silently passing one surface.
- **Boundary conditions**: cover double-quoted strings, triple-quoted strings, `#{...}`, `${...}`, nested braces inside interpolation, and strings without interpolation so false positives stay visible.

## Steps

1. Create `scripts/fixtures/m036-s01-syntax-corpus.json` from representative repo-owned Mesh sources and add only the smallest extra fixture needed for uncovered interpolation edges.
2. Add `website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs` so it loads the shared grammar into TextMate and Shiki, runs every corpus case, and reports engine/file/case diffs.
3. Add `scripts/verify-m036-s01.sh` as the repo-root verifier that replays compiler lexer truth plus the parity test.

## Must-Haves

- [ ] The corpus references real repo Mesh files plus only the smallest extra fixture needed for uncovered interpolation edges.
- [ ] The parity test exercises both VS Code/TextMate tokenization and docs/Shiki rendering against the same shared grammar file.
- [ ] Failures name the engine, corpus file, and case so drift is localizable in one rerun.

## Verification

- `bash scripts/verify-m036-s01.sh`
- `node --test website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs`

## Observability Impact

- Signals added/changed: named engine/file/case failure output for parity drift plus a wrapper phase boundary between compiler truth and shared-surface proof.
- How a future agent inspects this: run `bash scripts/verify-m036-s01.sh` and read the first failing corpus case.
- Failure state exposed: whether the break came from compiler truth, shared grammar loading, or one renderer path.

## Inputs

- `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json` — shared grammar contract the verifier must load.
- `website/docs/.vitepress/config.mts` — docs-side grammar registration path that must stay on the shared grammar.
- `website/docs/.vitepress/theme/composables/useShiki.ts` — runtime Shiki loader the parity test should mirror.
- `mesher/main.mpl` — real repo corpus file with preferred `#{...}` interpolation.
- `reference-backend/main.mpl` — real repo corpus file with preferred `#{...}` interpolation in the backend proof surface.
- `tests/fixtures/interpolation.mpl` — existing fixture with `${...}` coverage.
- `website/docs/docs/language-basics/index.md` — docs examples that define the public interpolation contract.
- `website/docs/docs/cheatsheet/index.md` — concise docs examples that should remain covered by the proof.

## Expected Output

- `scripts/fixtures/m036-s01-syntax-corpus.json` — machine-readable corpus manifest for the parity harness.
- `scripts/fixtures/m036-s01/interpolation_edge_cases.mpl` — minimal extra fixture for uncovered interpolation edge cases.
- `website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs` — Node test that compares TextMate and Shiki over the shared grammar.
- `scripts/verify-m036-s01.sh` — repo-root verification entrypoint for this slice.
