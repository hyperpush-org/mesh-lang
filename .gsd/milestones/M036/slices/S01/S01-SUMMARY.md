---
id: S01
parent: M036
milestone: M036
provides:
  - A repo-owned audited syntax corpus that exercises real Mesh and docs snippets plus the minimal extra interpolation-edge fixture.
  - A fail-closed shared-surface parity harness that proves the single VS Code/docs TextMate grammar through both TextMate and Shiki.
  - A repaired shared grammar and aligned docs/editor wording for `#{...}` and `${...}` in double- and triple-quoted strings, including nested-brace interpolation bodies.
requires:
  []
affects:
  - S02
  - S03
key_files:
  - scripts/fixtures/m036-s01-syntax-corpus.json
  - scripts/fixtures/m036-s01/interpolation_edge_cases.mpl
  - website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs
  - scripts/verify-m036-s01.sh
  - tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json
  - tools/editors/vscode-mesh/README.md
  - tools/editors/vscode-mesh/CHANGELOG.md
  - website/docs/docs/tooling/index.md
key_decisions:
  - Use a repo-owned line-range syntax corpus plus a repo-root wrapper script as the shared editor/docs proof surface instead of one-off ad hoc examples.
  - Model `#{...}` and `${...}` through one shared interpolation repository rule reused by both double- and triple-quoted strings.
  - Handle nested braces inside interpolation bodies recursively so map/object literals do not consume the outer interpolation closing scope.
patterns_established:
  - Pair compiler lexer truth with shared-surface TextMate/Shiki parity in one repo-root verifier so grammar drift is caught before docs or editor claims move.
  - Use a repo-owned line-range corpus manifest to prove syntax behavior against real Mesh sources and docs snippets without duplicating source text.
  - Keep interpolation semantics in one shared grammar repository rule reused by all string kinds, with recursive brace handling inside interpolation bodies.
observability_surfaces:
  - `scripts/verify-m036-s01.sh` phase banners for `compiler-truth` and `shared-surface-parity`.
  - `website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs` fail-closed `engine/file/case/form` drift output for localizing regressions.
drill_down_paths:
  - .gsd/milestones/M036/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M036/slices/S01/tasks/T02-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-28T05:05:17.065Z
blocker_discovered: false
---

# S01: Corpus-backed syntax parity for the shared VS Code/docs surface

**Built a corpus-backed parity harness, repaired the shared VS Code/docs TextMate interpolation rules, and aligned public syntax claims to the verified contract.**

## What Happened

S01 started by building an explicit proof surface for the already-shipping shared editor/docs grammar instead of patching one visible regex in isolation. The slice added a repo-owned syntax corpus manifest that points at representative Mesh sources in `mesher/`, `reference-backend/`, test fixtures, and docs snippets, plus one minimal extra fixture for the triple-quoted and nested-brace interpolation edges that were not already covered elsewhere. That corpus now drives a fail-closed Node verifier under `website/scripts/tests/` which loads the same shared TextMate grammar through standalone TextMate and docs-side Shiki, checks both engines against compiler-shaped interpolation expectations, and localizes any drift with named `engine/file/case/form` output. `scripts/verify-m036-s01.sh` wraps that parity proof with the compiler lexer interpolation replay so later slices have one repo-root entrypoint.

With the proof harness in place, the slice repaired the shared grammar itself. The TextMate grammar now uses one shared interpolation repository rule for both `#{...}` and `${...}` across double- and triple-quoted strings, and it includes a recursive nested-brace matcher inside interpolation bodies so expressions like `${Map.get(meta, {id: 1})}` keep the correct closing interpolation scope. Once the parity harness went green for both TextMate and Shiki, the slice aligned the VS Code extension README/changelog and the public tooling docs so they describe exactly the syntax contract the corpus proves rather than broader hand-wavy highlighting claims.

For downstream readers, this slice establishes the shared grammar contract that S02 and S03 should build on instead of re-litigating syntax behavior. S02 can treat the corpus manifest plus `scripts/verify-m036-s01.sh` as the existing truth surface when deciding how Neovim should mirror or clearly diverge from the shared VS Code/docs semantics. S03 can use the same verified contract when it publishes support tiers and editor guidance. If a later slice widens highlighting claims, extend the corpus manifest first and let the parity verifier go red before updating grammar docs or editor messaging.

## Verification

Ran `bash scripts/verify-m036-s01.sh` from the repo root. The wrapper replayed `cargo test -p mesh-lexer string_interpolation -- --nocapture` and confirmed compiler-side interpolation truth stayed green, then ran `node --test website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs` and passed all three checks: corpus manifest integrity, fail-closed verifier helper behavior, and TextMate/Shiki parity against the audited corpus. This also confirmed the slice's diagnostic surfaces: the wrapper prints distinct compiler/parity phases, and the Node verifier still enforces named drift localization for malformed corpus entries, missing loader paths, stalled engine setup, and scope mismatches.

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Used explicit line-range corpus entries for docs markdown snippets instead of parsing fenced code blocks, and used Shiki's loaded grammar tokenizer for scope-level parity while keeping `codeToHtml(...)` as the docs render smoke path. This kept the proof tied to exact source lines and avoided a false-green token API that collapses Mesh string scopes too aggressively.

## Known Limitations

The proof contract is intentionally bounded to the audited corpus and the shared VS Code/docs TextMate surface. It does not yet prove Neovim highlighting or any non-TextMate editor path, and future syntax widening still requires adding corpus coverage before editor/docs claims should expand.

## Follow-ups

S02 should decide whether the Neovim support pack can reuse this shared syntax contract directly or must document a narrower initial boundary, while keeping `scripts/verify-m036-s01.sh` green for any shared-surface edits. S03 should derive its support-tier and tooling wording from this corpus-backed contract instead of broader editor-compatibility claims.

## Files Created/Modified

- `scripts/fixtures/m036-s01-syntax-corpus.json` — Added the audited line-range syntax corpus covering real Mesh sources, docs snippets, and expected interpolation forms/string kinds.
- `scripts/fixtures/m036-s01/interpolation_edge_cases.mpl` — Added the minimal extra fixture for triple-quoted and nested-brace interpolation edge cases not already covered by repo sources.
- `website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs` — Added the fail-closed TextMate/Shiki parity verifier, corpus validation, and localized drift reporting.
- `scripts/verify-m036-s01.sh` — Added the repo-root wrapper that replays compiler interpolation truth before the shared-surface parity harness.
- `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json` — Repaired shared interpolation scoping for `#{...}` and `${...}` across double- and triple-quoted strings, including nested-brace handling.
- `tools/editors/vscode-mesh/README.md` — Updated the extension contract to describe the verified shared syntax surface honestly.
- `tools/editors/vscode-mesh/CHANGELOG.md` — Recorded the shared grammar interpolation-parity repair.
- `website/docs/docs/tooling/index.md` — Aligned public tooling docs with the audited shared VS Code/docs grammar contract.
- `.gsd/PROJECT.md` — Updated current project state to note that M036 S01 closed the shared VS Code/docs syntax parity gap with a corpus-backed verifier.
