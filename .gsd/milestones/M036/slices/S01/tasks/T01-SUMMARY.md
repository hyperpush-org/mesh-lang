---
id: T01
parent: S01
milestone: M036
provides: []
requires: []
affects: []
key_files: ["scripts/fixtures/m036-s01-syntax-corpus.json", "scripts/fixtures/m036-s01/interpolation_edge_cases.mpl", "website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs", "scripts/verify-m036-s01.sh", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Used a line-range corpus manifest so repo-owned `.mpl` sources and docs snippets can share one explicit proof contract without duplicating source text.", "Validated docs-side scope parity through Shiki's loaded grammar tokenizer and kept `codeToHtml(...)` as the render-path smoke check."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the compiler lexer interpolation proof, the new Node parity test, and the repo-root wrapper. `cargo test -p mesh-lexer string_interpolation -- --nocapture` passed. `node --test website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs` failed closed with named `engine/file/case/form` drift output, which is the expected T01 outcome because the shared grammar has not been repaired yet. `bash scripts/verify-m036-s01.sh` passed the compiler-truth phase and then failed in the parity phase with the same localized drift output."
completed_at: 2026-03-28T04:57:22.002Z
blocker_discovered: false
---

# T01: Added an audited syntax corpus plus a fail-closed TextMate/Shiki parity verifier that reproduces the current interpolation drift with named engine/file/case output.

> Added an audited syntax corpus plus a fail-closed TextMate/Shiki parity verifier that reproduces the current interpolation drift with named engine/file/case output.

## What Happened
---
id: T01
parent: S01
milestone: M036
key_files:
  - scripts/fixtures/m036-s01-syntax-corpus.json
  - scripts/fixtures/m036-s01/interpolation_edge_cases.mpl
  - website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs
  - scripts/verify-m036-s01.sh
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Used a line-range corpus manifest so repo-owned `.mpl` sources and docs snippets can share one explicit proof contract without duplicating source text.
  - Validated docs-side scope parity through Shiki's loaded grammar tokenizer and kept `codeToHtml(...)` as the render-path smoke check.
duration: ""
verification_result: passed
completed_at: 2026-03-28T04:57:22.006Z
blocker_discovered: false
---

# T01: Added an audited syntax corpus plus a fail-closed TextMate/Shiki parity verifier that reproduces the current interpolation drift with named engine/file/case output.

**Added an audited syntax corpus plus a fail-closed TextMate/Shiki parity verifier that reproduces the current interpolation drift with named engine/file/case output.**

## What Happened

Built the proof harness for M036/S01 before touching the grammar itself. Added a machine-readable corpus manifest covering real Mesh source lines from mesher, reference-backend, tests, and docs, plus one minimal extra fixture for uncovered triple-quoted and nested-brace interpolation edges. Added a Node parity test that loads the shared grammar through standalone TextMate and the docs-side Shiki path, validates malformed corpus/loader failures, and checks each engine against explicit interpolation expectations rather than only comparing the two engines to each other. Added a repo-root wrapper script that reruns compiler lexer truth first and then the shared-surface parity proof so one rerun localizes failures immediately. The resulting harness is intentionally red on the current grammar and now exposes that `#{...}` is still tokenized as plain string content and that the nested-brace edge case is not scoped correctly.

## Verification

Ran the compiler lexer interpolation proof, the new Node parity test, and the repo-root wrapper. `cargo test -p mesh-lexer string_interpolation -- --nocapture` passed. `node --test website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs` failed closed with named `engine/file/case/form` drift output, which is the expected T01 outcome because the shared grammar has not been repaired yet. `bash scripts/verify-m036-s01.sh` passed the compiler-truth phase and then failed in the parity phase with the same localized drift output.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-lexer string_interpolation -- --nocapture` | 0 | ✅ pass | 424ms |
| 2 | `node --test website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs` | 1 | ✅ pass | 797ms |
| 3 | `bash scripts/verify-m036-s01.sh` | 1 | ✅ pass | 1275ms |


## Deviations

Used line-range corpus entries for docs markdown snippets instead of parsing markdown fences, and used Shiki's loaded grammar tokenizer (`getLanguage('mesh').tokenizeLine(...)`) for scope parity while keeping `codeToHtml(...)` as the render smoke path because the public token API collapses Mesh strings too aggressively for scope-level proof.

## Known Issues

The shared TextMate grammar still misses `#{...}` interpolation scopes and mishandles the nested-brace interpolation edge case, so the new parity verifier remains red until T02 repairs the grammar.

## Files Created/Modified

- `scripts/fixtures/m036-s01-syntax-corpus.json`
- `scripts/fixtures/m036-s01/interpolation_edge_cases.mpl`
- `website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs`
- `scripts/verify-m036-s01.sh`
- `.gsd/KNOWLEDGE.md`


## Deviations
Used line-range corpus entries for docs markdown snippets instead of parsing markdown fences, and used Shiki's loaded grammar tokenizer (`getLanguage('mesh').tokenizeLine(...)`) for scope parity while keeping `codeToHtml(...)` as the render smoke path because the public token API collapses Mesh strings too aggressively for scope-level proof.

## Known Issues
The shared TextMate grammar still misses `#{...}` interpolation scopes and mishandles the nested-brace interpolation edge case, so the new parity verifier remains red until T02 repairs the grammar.
