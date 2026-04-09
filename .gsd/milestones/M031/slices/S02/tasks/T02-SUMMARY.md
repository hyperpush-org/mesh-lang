---
id: T02
parent: S02
milestone: M031
provides:
  - Formatter support for parenthesized imports with one name per indented line
  - Trailing comma space suppression before closing paren in arg lists
key_files:
  - compiler/mesh-fmt/src/walker.rs
key_decisions:
  - Extracted IMPORT_LIST from walk_tokens_inline into a dedicated walk_import_list function rather than modifying walk_from_import_decl, because L_PAREN/R_PAREN tokens live inside IMPORT_LIST not FROM_IMPORT_DECL
patterns_established:
  - Paren-aware node handlers: check first child for L_PAREN, collect names, emit hardline-separated indent block; fall through to walk_tokens_inline for non-paren form
observability_surfaces:
  - 4 unit tests pin exact formatter output for paren imports and trailing-comma arg lists
duration: 12m
verification_result: passed
completed_at: 2026-03-24
blocker_discovered: false
---

# T02: Formatter support for parenthesized imports and trailing comma cleanup

**Added dedicated walk_import_list handler for parenthesized imports and trailing-comma space suppression in walk_paren_list**

## What Happened

Created a `walk_import_list` function and routed `IMPORT_LIST` to it instead of the generic `walk_tokens_inline`. The function detects parens by scanning for `L_PAREN` among children. When parens are present, it collects name nodes, then emits them on separate indented lines inside `(\n  name1,\n  name2\n)`. When no parens are present, it delegates to `walk_tokens_inline` preserving the existing flat format.

Fixed `walk_paren_list` to suppress the trailing space after COMMA when the next non-trivia sibling token is `R_PAREN`. This uses `next_sibling_or_token()` to peek ahead, skipping NEWLINE and WHITESPACE tokens.

The plan suggested modifying `walk_from_import_decl`'s token match for `L_PAREN`/`R_PAREN`, but the CST structure puts those tokens inside `IMPORT_LIST`, not as direct children of `FROM_IMPORT_DECL`. The actual approach handles them at the right level in the tree.

## Verification

- `cargo test -p mesh-fmt --lib` — 119 passed, 0 failed (4 new tests included)
- `cargo test -p meshc --test e2e` — 313 passed, 10 failed (same pre-existing try-related failures)
- `cargo run -p meshc -- build reference-backend` — success
- `cargo run -p meshc -- build mesher` — success
- `cargo test -p mesh-parser --test parser_tests from_import_paren` — 3 passed

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-fmt --lib` | 0 | ✅ pass | 3.3s |
| 2 | `cargo test -p meshc --test e2e` | 101 | ✅ pass (313 ok, 10 pre-existing failures) | 229.6s |
| 3 | `cargo run -p meshc -- build reference-backend` | 0 | ✅ pass | 9.2s |
| 4 | `cargo run -p meshc -- build mesher` | 0 | ✅ pass | 13.0s |
| 5 | `cargo test -p mesh-parser --test parser_tests from_import_paren` | 0 | ✅ pass | 3.3s |

## Diagnostics

- 4 formatter unit tests (`from_import_paren_single_line`, `from_import_paren_multiline`, `from_import_paren_trailing_comma`, `trailing_comma_arg_list`) pin exact output — any regression produces a direct `assert_eq` diff in CI.
- No runtime diagnostics — this is a compile-time formatting feature.

## Deviations

- The plan assumed `L_PAREN`/`R_PAREN` would be direct children of `FROM_IMPORT_DECL`, but the CST places them inside `IMPORT_LIST`. Created a dedicated `walk_import_list` function instead of modifying `walk_from_import_decl`'s token match. Same outcome, different code location.

## Known Issues

None.

## Files Created/Modified

- `compiler/mesh-fmt/src/walker.rs` — Added `walk_import_list` function with paren-aware formatting, routed `IMPORT_LIST` to it in `walk_node` dispatch, fixed trailing-comma space suppression in `walk_paren_list`, added 4 new formatter unit tests
