---
id: T03
parent: S01
milestone: M031
provides:
  - Trivia-aware Literal::token() in AST layer
  - E2e tests for multiline function call correctness (Int, String, 3-arg, mixed, let binding)
key_files:
  - compiler/mesh-parser/src/ast/expr.rs
  - compiler/meshc/tests/e2e.rs
key_decisions:
  - Fixed Literal::token() in the AST layer (not the typechecker or codegen) to skip trivia tokens, protecting all downstream callers
patterns_established:
  - AST accessor methods that iterate children_with_tokens should filter trivia when looking for meaningful tokens
observability_surfaces:
  - cargo test -p meshc --test e2e multiline_call isolates multiline call correctness
  - String-return multiline test is the most sensitive sentinel (crashes on misaligned pointer if trivia leaks through)
duration: 35m
verification_result: passed
completed_at: 2026-03-24
blocker_discovered: false
---

# T03: Fix multiline function call type resolution

**Fixed Literal::token() to skip trivia tokens so multiline call arguments resolve to correct types instead of Unit**

## What Happened

Function calls with arguments spanning multiple lines (e.g., `add(\n  1,\n  2\n)`) resolved to `()` instead of the correct return type. The compiler error was "Unsupported binop type: Unit" — the function body couldn't use its parameters because they were typed as Unit.

**Root cause:** In the parser's CST, when arguments span multiple lines, NEWLINE tokens become leading children of LITERAL nodes. `Literal::token()` used `.next()` to get the first token child, which returned the NEWLINE trivia token instead of the INT_LITERAL. The typechecker's `infer_literal` matched the NEWLINE kind against the `_` wildcard arm, returning `Ty::Tuple([])` (Unit). The codegen's `lower_literal` similarly fell through to `MirExpr::Unit` or parsed `"\n".parse::<i64>()` as 0.

**Fix:** Changed `Literal::token()` in `compiler/mesh-parser/src/ast/expr.rs` from `.next()` to `.find(|t| !t.kind().is_trivia())`. This skips NEWLINE, WHITESPACE, and COMMENT tokens, returning the meaningful literal token (INT_LITERAL, FLOAT_LITERAL, TRUE_KW, etc.). The fix is in the AST layer, protecting all callers — `infer_literal` (typechecker), `lower_literal` (codegen), and any future consumers.

## Verification

- `cargo test -p meshc --test e2e multiline_call` — 5/5 pass
- `cargo test -p mesh-parser --lib` — 17/17 pass
- `cargo test -p meshc --test e2e` — 308/318 pass (10 failures are pre-existing `try_*` tests, same as T01/T02)
- `cargo run -p meshc -- build reference-backend` — success
- `cargo run -p meshc -- build mesher` — success

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e multiline_call` | 0 | ✅ pass | 8.3s |
| 2 | `cargo test -p mesh-parser --lib` | 0 | ✅ pass | 10.0s |
| 3 | `cargo test -p meshc --test e2e` | 101 | ✅ pass (10 pre-existing failures, 0 regressions) | 212.0s |
| 4 | `cargo run -p meshc -- build reference-backend` | 0 | ✅ pass | 16.1s |
| 5 | `cargo run -p meshc -- build mesher` | 0 | ✅ pass | 17.3s |

## Diagnostics

When multiline function calls produce wrong values or "Unsupported binop type: Unit" errors, the root cause is a trivia token (NEWLINE, WHITESPACE) leaking through `Literal::token()` in the AST layer. The fix ensures `Literal::token()` always returns the meaningful literal token. The `e2e_multiline_call_string_return` test is the most sensitive sentinel — it crashes on misaligned pointer dereference if the literal type regresses to Unit.

## Deviations

- **Root cause was in AST layer, not typechecker or codegen.** The plan hypothesized the bug was in `infer_call`, `infer_expr` type storage, or `TextRange` mismatch. The actual root cause was `Literal::token()` returning NEWLINE trivia instead of the meaningful literal token. The fix was a one-line change in `compiler/mesh-parser/src/ast/expr.rs`, not in `infer.rs`.
- **Tests use typed functions.** The plan didn't specify type annotations, but untyped polymorphic functions have a separate pre-existing codegen issue (Ty::Var → MirType::Unit in `resolve_type`). Tests use typed function signatures matching the existing e2e test idiom.

## Known Issues

- **Untyped polymorphic functions produce wrong runtime values.** `fn add(a, b) do a + b end` (no type annotations) correctly type-checks at the call site after this fix, but codegen still produces `0` because `resolve_type` maps `Ty::Var` to `MirType::Unit` for generalized function parameters. This is a pre-existing monomorphization gap, not a multiline-specific bug. All existing tests use typed function definitions.
- 10 pre-existing `try_*`/`from_try_*` e2e test failures (runtime crashes with exit code None) exist on clean main — unrelated to this fix.

## Files Created/Modified

- `compiler/mesh-parser/src/ast/expr.rs` — Changed `Literal::token()` to skip trivia tokens (`.next()` → `.find(|t| !t.kind().is_trivia())`)
- `compiler/meshc/tests/e2e.rs` — Added 5 e2e tests for multiline function call correctness (Int, String, 3-arg, mixed single/multi, let binding)
- `.gsd/milestones/M031/slices/S01/S01-PLAN.md` — Marked T03 done; added multiline call diagnostic entry
