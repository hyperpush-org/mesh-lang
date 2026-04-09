# S02: Trailing Commas & Multiline Imports — Research

**Depth:** Light-to-targeted. Both features touch known parser code with established patterns. The multiline import fix is a small parser change; trailing commas in call args already work.

## Requirements Targeted

- **R018** (active, owner: M031/S02) — Multiline parenthesized imports: `from Module import (\n  a,\n  b\n)` must parse correctly.
- **R019** (active, owner: M031/S02) — Trailing commas in function call arguments must parse correctly.

## Summary

**R019 is already satisfied.** Trailing commas in function call args (`add(1, 2,)`) and function definition params (`fn add(a :: Int, b :: Int,) -> Int do`) already parse, compile, and run correctly. The `parse_arg_list` function (expressions.rs:629) and `parse_param_list` (expressions.rs:1270) both have `if p.at(R_PAREN) { break; }` after `p.eat(COMMA)`. The `paren_depth` tracking makes newlines insignificant inside `()`, so multiline trailing-comma variants also work.

**R018 requires a parser change.** `parse_from_import_decl` (items.rs:178) currently expects `IDENT` tokens directly after `import`. When it sees `L_PAREN`, it emits `"expected import name"` and bails. The fix is small: check for `L_PAREN` after `import`, advance (which bumps `paren_depth`, making newlines insignificant automatically), parse the name list as before, add `R_PAREN` to the trailing-comma break condition, then expect `R_PAREN`.

**Formatter needs updates for both features:**
- `walk_from_import_decl` (walker.rs:1311) will encounter `L_PAREN`/`R_PAREN` tokens that currently fall through to the `_ =>` catch-all. It needs explicit handling to format parenthesized imports idiomatically — either preserving the multiline form or using a group/softline/indent to auto-wrap long import lists.
- `walk_paren_list` (walker.rs:802) handles ARG_LIST commas as `, ` (comma+space). After a trailing comma this produces `, )` — cosmetically ugly. Minor formatter polish: suppress the space when the next token after comma is `)`.

## Implementation Landscape

### Parser: Multiline Parenthesized Imports

**File:** `compiler/mesh-parser/src/parser/items.rs`, function `parse_from_import_decl` (line 178)

**Current flow:**
1. Advance past `"from"` IDENT
2. `parse_module_path(p)` — dot-separated path
3. `p.expect(IMPORT_KW)` — consume `import`
4. Check `p.at(IDENT)` — parse comma-separated names
5. Break on `NEWLINE || EOF` after a comma

**Fix:**
1. After step 3, check `p.at(L_PAREN)`. If true: `p.advance()` (bumps `paren_depth`; newlines become insignificant).
2. Parse name list identically to current code.
3. Change the trailing-comma break condition from `NEWLINE || EOF` to `NEWLINE || EOF || R_PAREN`.
4. After the name loop, if parens were opened: `p.expect(R_PAREN)`.

The `R_PAREN` break condition is safe for both forms — in the non-paren case, `R_PAREN` would never appear mid-import-list. In the paren case, `NEWLINE` is auto-skipped by the parser's `should_skip` logic.

**No downstream changes needed.** `FromImportDecl::import_list()` → `ImportList::names()` iterates child `Name` nodes, ignoring tokens. All consumers (typechecker infer.rs:3917, meshc discovery.rs:148, LSP analysis.rs, codegen lower.rs) use this AST interface. The `L_PAREN`/`R_PAREN` tokens are invisible to them.

### Formatter: Multiline Import Support

**File:** `compiler/mesh-fmt/src/walker.rs`, function `walk_from_import_decl` (line 1311)

The current implementation iterates `children_with_tokens()` and handles IDENT ("from"), IMPORT_KW, NEWLINE, and child nodes. `L_PAREN`/`R_PAREN` would fall into the `_ =>` catch-all (`add_token_with_context`), producing `from Module import(name1, name2)` — missing space before `(`.

**Fix:** Add explicit `L_PAREN`/`R_PAREN` matching in the token match arm. Two formatting approaches:

1. **Flat preservation:** Always emit `from Module import (name1, name2, name3)` — collapse to single line.
2. **Group-based wrapping:** Use `ir::group(ir::concat([...]))` with `ir::softline()` between names inside the parens, and `ir::indent(...)` for the name list. Short lists stay on one line; long lists wrap.

Recommendation: **Flat preservation for now.** The formatter doesn't currently auto-wrap anything based on line width. Adding width-aware wrapping for imports alone would be inconsistent. Users write multiline imports in source; the formatter flattens them. This matches how `walk_paren_list` already handles ARG_LIST (wraps in a `group` that flattens).

However, mesher imports at 186-438 chars would be re-flattened by the formatter, defeating the purpose. If this is unacceptable, the formatter can detect the parenthesized form and preserve line breaks (emit `ir::hardline()` between names when parens are present). This is the simpler approach — no width calculation needed, just: "if parens are present, emit newlines between names."

### Formatter: Trailing Comma in Arg Lists

**File:** `compiler/mesh-fmt/src/walker.rs`, function `walk_paren_list` (line 802)

Currently emits `ir::text(",")` + `ir::space()` for every COMMA. When a trailing comma precedes `R_PAREN`, this produces `, )`. Fix: check if the next sibling token after COMMA is `R_PAREN` and skip the space, or strip trailing commas entirely during formatting.

### Parser Tests

**File:** `compiler/mesh-parser/tests/parser_tests.rs`

Existing snapshot test at line 581: `from_import` → `"from Math import sqrt, pow"`. Add new snapshot tests:
- `from_import_paren` → `"from Math import (sqrt, pow)"`
- `from_import_paren_trailing_comma` → `"from Math import (sqrt, pow,)"`
- `from_import_paren_multiline` → `"from Math import (\n  sqrt,\n  pow\n)"`

### E2E Tests

**File:** `compiler/meshc/tests/e2e.rs`

S01 established the pattern: inline source string → `compile_and_run` → assert output. For multiline import tests, stdlib imports work: `from String import (\n  length,\n  upcase\n)`.

New tests needed:
- `e2e_multiline_import_paren_basic` — parenthesized import on one line
- `e2e_multiline_import_paren_multiline` — names on separate lines
- `e2e_multiline_import_trailing_comma` — trailing comma in parenthesized import
- `e2e_trailing_comma_call_single_line` — `fn_call(a, b,)` compiles and runs
- `e2e_trailing_comma_call_multiline` — multiline call with trailing comma (confirms S01 fix + trailing comma)

### Formatter E2E Tests

Existing formatter tests in `compiler/mesh-fmt/src/walker.rs` at line 2210 (`from_import`). Add tests for:
- Parenthesized single-line import formatting
- Parenthesized multiline import formatting/preservation

## Risks and Constraints

1. **Formatter flattening vs user intent:** If the formatter always flattens parenthesized imports, `meshc fmt` will undo the multiline formatting users explicitly chose. The mesher has 6 import lines over 100 chars — flattening them defeats the purpose of R018. **Mitigation:** When parens are present, preserve line breaks (hardlines between names).

2. **Trailing-comma space before `)` is cosmetic, not blocking.** `add(1, 2, )` is ugly but compiles. Can be deferred to S05 or a future formatter polish pass if it's not trivial.

3. **No multi-file compile_and_run helper.** Import tests must use stdlib modules (`String`, etc.) unless a multi-file helper is added. This is sufficient for parser/compile verification.

## Recommendation

Three tasks:

**T01: Parser — Multiline parenthesized imports.**
Change `parse_from_import_decl` in `items.rs` to handle optional `L_PAREN`/`R_PAREN` around the import name list. Add `R_PAREN` to the trailing-comma break condition. Add parser snapshot tests. Add e2e tests for parenthesized imports (single-line, multiline, trailing comma). Verify with `cargo test -p mesh-parser --lib`, `cargo test -p meshc --test e2e`, and `cargo run -p meshc -- build reference-backend && cargo run -p meshc -- build mesher`.

**T02: Formatter — Handle new import syntax.**
Update `walk_from_import_decl` in `walker.rs` to handle `L_PAREN`/`R_PAREN` tokens. When parens are present, emit names on separate lines (hardline between names, indented inside parens) so the formatter preserves the multiline intent. Add formatter unit tests. Verify with `cargo test -p mesh-fmt --lib` and formatter e2e test against a parenthesized import.

**T03: E2E — Trailing comma confirmation tests.**
Trailing commas already work but have zero test coverage. Add e2e tests confirming: single-line trailing comma in call args, multiline trailing comma in call args, trailing comma in function definition params. These are pure verification tests — no code changes needed. Verify with `cargo test -p meshc --test e2e`.

T01 is the only parser change and unblocks T02. T03 is independent (no code changes, just tests). T01 and T03 can run in parallel; T02 depends on T01.

## Skills Discovered

None needed. This is compiler-internal Rust work on an established codebase with no external dependencies.
