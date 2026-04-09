---
id: T02
parent: S01
milestone: M036
provides: []
requires: []
affects: []
key_files: ["tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json", "tools/editors/vscode-mesh/README.md", "tools/editors/vscode-mesh/CHANGELOG.md", "website/docs/docs/tooling/index.md", ".gsd/KNOWLEDGE.md", ".gsd/DECISIONS.md"]
key_decisions: ["Kept interpolation scoping in one shared grammar repository rule reused by both double- and triple-quoted strings.", "Added a recursive nested-brace matcher inside interpolation bodies so nested map/object literals do not steal the outer interpolation closing scope."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran `cargo test -p mesh-lexer string_interpolation -- --nocapture` to confirm compiler-side interpolation truth remained green, then ran `bash scripts/verify-m036-s01.sh` to verify the full slice contract end-to-end, including the shared TextMate/Shiki parity harness against the audited corpus. Both commands passed."
completed_at: 2026-03-28T05:01:37.643Z
blocker_discovered: false
---

# T02: Repaired shared TextMate interpolation parity for `#{...}` and `${...}` and aligned the VS Code/docs syntax contract to the verified corpus.

> Repaired shared TextMate interpolation parity for `#{...}` and `${...}` and aligned the VS Code/docs syntax contract to the verified corpus.

## What Happened
---
id: T02
parent: S01
milestone: M036
key_files:
  - tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json
  - tools/editors/vscode-mesh/README.md
  - tools/editors/vscode-mesh/CHANGELOG.md
  - website/docs/docs/tooling/index.md
  - .gsd/KNOWLEDGE.md
  - .gsd/DECISIONS.md
key_decisions:
  - Kept interpolation scoping in one shared grammar repository rule reused by both double- and triple-quoted strings.
  - Added a recursive nested-brace matcher inside interpolation bodies so nested map/object literals do not steal the outer interpolation closing scope.
duration: ""
verification_result: passed
completed_at: 2026-03-28T05:01:37.644Z
blocker_discovered: false
---

# T02: Repaired shared TextMate interpolation parity for `#{...}` and `${...}` and aligned the VS Code/docs syntax contract to the verified corpus.

**Repaired shared TextMate interpolation parity for `#{...}` and `${...}` and aligned the VS Code/docs syntax contract to the verified corpus.**

## What Happened

Reproduced the failing shared-surface parity verifier, confirmed the grammar only recognized `${...}` and lost the closing interpolation scope on nested-brace cases, then repaired the shared TextMate grammar with one reusable interpolation rule for both `#{...}` and `${...}` plus a recursive brace matcher inside interpolation bodies. After the grammar fix turned the parity harness green for both TextMate and Shiki, aligned the extension README, changelog, and tooling docs to describe only the audited shared-grammar contract and recorded the recursive-brace gotcha in project knowledge.

## Verification

Ran `cargo test -p mesh-lexer string_interpolation -- --nocapture` to confirm compiler-side interpolation truth remained green, then ran `bash scripts/verify-m036-s01.sh` to verify the full slice contract end-to-end, including the shared TextMate/Shiki parity harness against the audited corpus. Both commands passed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash scripts/verify-m036-s01.sh` | 0 | ✅ pass | 1945ms |
| 2 | `cargo test -p mesh-lexer string_interpolation -- --nocapture` | 0 | ✅ pass | 256ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json`
- `tools/editors/vscode-mesh/README.md`
- `tools/editors/vscode-mesh/CHANGELOG.md`
- `website/docs/docs/tooling/index.md`
- `.gsd/KNOWLEDGE.md`
- `.gsd/DECISIONS.md`


## Deviations
None.

## Known Issues
None.
