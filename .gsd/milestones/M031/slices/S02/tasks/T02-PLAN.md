---
estimated_steps: 4
estimated_files: 1
skills_used: []
---

# T02: Formatter support for parenthesized imports and trailing comma cleanup

**Slice:** S02 — Trailing Commas & Multiline Imports
**Milestone:** M031

## Description

The formatter's `walk_from_import_decl` currently has no handling for `L_PAREN`/`R_PAREN` tokens — they fall through to the `_ =>` catch-all (`add_token_with_context`), producing something like `from Module import(a, b)` (missing space before paren, flat formatting). This task adds explicit paren handling that preserves multiline intent: when parens are present, names are emitted on separate indented lines. It also fixes `walk_paren_list` so trailing commas in arg lists don't produce an ugly space before `)`.

## Steps

1. **Modify `walk_from_import_decl` in `compiler/mesh-fmt/src/walker.rs`** (line ~1311). The function iterates `children_with_tokens()`. Changes:
   - Add a local `has_parens: bool = false` flag.
   - In the token match, add `SyntaxKind::L_PAREN => { has_parens = true; }` — don't emit the paren yet.
   - Add `SyntaxKind::R_PAREN => {}` — handled at the end.
   - After the loop, if `has_parens`, restructure output: emit `ir::text("(")`, `ir::hardline()`, indent the name list with commas and hardlines between each name, emit trailing newline, `ir::text(")")`. If not `has_parens`, emit the flat list as currently done.
   - The key insight: when parens are present, collect name parts separately and emit them in a hardline-separated indented group. When parens are absent, preserve current flat behavior.

2. **Fix trailing comma space in `walk_paren_list`** (line ~802). Currently COMMA always gets `ir::text(",")` + `ir::space()`. After emitting the comma, check if the next sibling token after COMMA is `R_PAREN` — if so, skip the space. Use `tok.next_sibling_or_token()` to peek ahead, filtering trivia tokens (NEWLINE, WHITESPACE).

3. **Add formatter unit tests** at the end of the test module (after line ~2210 where `from_import` test lives):
   - `from_import_paren_single_line` — input `"from Math import (sqrt, pow)"` → formats as multiline with one name per indented line inside parens
   - `from_import_paren_multiline` — input `"from Math import (\n  sqrt,\n  pow\n)"` → preserves multiline structure
   - `from_import_paren_trailing_comma` — input `"from Math import (\n  sqrt,\n  pow,\n)"` → formats cleanly (trailing comma preserved or stripped, no extra space before `)`)
   - `trailing_comma_arg_list` — input `"fn main() do\n  add(1, 2,)\nend"` → no space before `)`

4. **Run full verification:** `cargo test -p mesh-fmt --lib && cargo test -p meshc --test e2e`

## Must-Haves

- [ ] `walk_from_import_decl` handles `L_PAREN`/`R_PAREN` without falling through to catch-all
- [ ] Parenthesized imports format with one name per indented line (hardlines, not flat)
- [ ] Non-parenthesized imports continue to format flat (no regression)
- [ ] Trailing comma before `)` in arg lists does not produce an extra space
- [ ] All existing formatter tests pass
- [ ] All existing e2e tests still pass

## Verification

- `cargo test -p mesh-fmt --lib` — all formatter tests pass including 4 new tests
- `cargo test -p meshc --test e2e` — all e2e tests still pass (no formatter regressions)

## Inputs

- `compiler/mesh-fmt/src/walker.rs` — contains `walk_from_import_decl` (line ~1311) and `walk_paren_list` (line ~802); existing `from_import` formatter test at line ~2210
- `compiler/mesh-parser/src/parser/items.rs` — T01's parser change means the formatter will now encounter `L_PAREN`/`R_PAREN` in `FROM_IMPORT_DECL` nodes

## Expected Output

- `compiler/mesh-fmt/src/walker.rs` — modified `walk_from_import_decl` with paren handling, modified `walk_paren_list` with trailing-comma space suppression, 4 new formatter test functions

## Observability Impact

- **Formatter regression visibility:** 4 new unit tests pin the exact formatter output for parenthesized imports (single-line, multiline, trailing comma) and trailing-comma arg lists. Any regression produces a direct `assert_eq` diff.
- **No new runtime signals:** This is a compile-time formatting feature — no runtime observability changes.
- **Diagnostic preservation:** Parse errors on malformed parenthesized imports (`expected R_PAREN`) continue to surface through the parser; the formatter only processes valid CST nodes.
