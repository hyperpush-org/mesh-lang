---
id: S01
milestone: M031
outcome: success
tasks_completed: 3
tasks_total: 3
duration: ~72m
completed_at: 2026-03-24
requirements_validated:
  - R015: else-if chain value correctness
  - R016: trailing-closure disambiguation in control-flow conditions
  - R017: multiline function call type resolution
---

# S01: Parser & Codegen Fixes — Summary

Fixed three compiler bugs that blocked idiomatic Mesh code patterns, each proven by targeted e2e tests. All 308 pre-existing passing tests continue to pass; 15 new tests added. Both dogfood codebases (`reference-backend/`, `mesher/`) still build clean.

## What Changed

### T01: Trailing-closure disambiguation

**Problem:** The Pratt parser's postfix loop treated `do` after `)` as a trailing closure even inside control-flow conditions, so `if fn_call() do ... end` was misparsed.

**Fix:** Added `suppress_trailing_closure: bool` to `Parser` struct. Each of the 4 control-flow parsers (`parse_if_expr`, `parse_while_expr`, `parse_case_expr`, `parse_for_in_expr`) saves the flag, sets it `true` before `expr(p)`, and restores it after. The `DO_KW` trailing-closure check is guarded by `!p.suppress_trailing_closure()`.

**Files:** `compiler/mesh-parser/src/parser/mod.rs`, `compiler/mesh-parser/src/parser/expressions.rs`

### T02: Else-if chain value correctness

**Problem:** `infer_if` recursively calls itself for `else if` chains, bypassing `infer_expr` which normally stores resolved types in the `types` map. Codegen fell back to `MirType::Unit`, producing garbage integer values or misaligned pointer crashes for String branches.

**Fix:** Added `types.insert(if_.syntax().text_range(), resolved_type)` before both return paths in `infer_if`, matching the storage pattern `infer_expr` uses for all other expression types.

**Files:** `compiler/mesh-typeck/src/infer.rs`

### T03: Multiline function call type resolution

**Problem:** Function calls with arguments on separate lines resolved to `()`. NEWLINE trivia tokens became leading children of LITERAL CST nodes, and `Literal::token()` used `.next()` which returned the trivia token instead of the meaningful literal.

**Fix:** Changed `Literal::token()` from `.next()` to `.find(|t| !t.kind().is_trivia())`. The fix is in the AST layer, protecting all downstream callers.

**Files:** `compiler/mesh-parser/src/ast/expr.rs`

## Test Surface

15 new e2e tests added across 3 categories:

| Category | Tests | Sentinel |
|----------|-------|----------|
| `trailing_closure` | if, while, case, for with fn-call conditions; trailing-closure regression | Parse error at `do` position |
| `else_if` | Int, String, Bool return chains; 3-level chain; let binding | `e2e_else_if_chain_string_value` (crash on misaligned pointer) |
| `multiline_call` | Int, String return; 3-arg; mixed single/multi; let binding | `e2e_multiline_call_string_return` (crash on misaligned pointer) |

Run targeted: `cargo test -p meshc --test e2e trailing_closure`, `cargo test -p meshc --test e2e else_if`, `cargo test -p meshc --test e2e multiline_call`

## Patterns for Downstream Slices

- **Parser flag save/restore:** When parsing a subexpression that needs different postfix behavior, save the parser flag before `expr(p)` and restore after. This is the established pattern for context-sensitive disambiguation.
- **Type-map storage in recursive inference:** Any inference function that bypasses `infer_expr` must store its resolved type in the `types` map before returning, or codegen will fall back to `MirType::Unit`.
- **Trivia-aware AST accessors:** AST methods iterating `children_with_tokens()` must filter trivia (`.find(|t| !t.kind().is_trivia())`) instead of using `.next()`. The `Literal::token()` fix is the precedent.

## What S02+ Should Know

- All 4 control-flow forms now handle fn-call conditions correctly — S02/S03/S04 can use `if fn_call() do` freely in dogfood cleanup.
- `else if` chains return correct values — S03/S04 can use `else if` without workarounds.
- Multiline fn calls resolve correct types — S02's trailing-comma work can build on this foundation.
- The 10 pre-existing `try_*`/`from_try_*` e2e failures (runtime crashes, exit code None) are unrelated to S01 and remain unchanged. They are a known pre-existing issue.
- Untyped polymorphic functions still produce wrong runtime values due to a separate `Ty::Var` → `MirType::Unit` monomorphization gap. All tests use typed function signatures.

## Verification

| Check | Result |
|-------|--------|
| `cargo test -p meshc --test e2e` | 308 pass, 10 pre-existing fail (318 total) |
| `cargo test -p meshc --test e2e trailing_closure` | 5/5 pass |
| `cargo test -p meshc --test e2e else_if` | 5/5 pass |
| `cargo test -p meshc --test e2e multiline_call` | 5/5 pass |
| `cargo test -p mesh-parser --lib` | 17/17 pass |
| `cargo run -p meshc -- build reference-backend` | success |
| `cargo run -p meshc -- build mesher` | success |
