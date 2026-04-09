# S01 Research: Parser & Codegen Fixes

## Summary

Three compiler bugs, each in a different compiler phase. All reproduced and root-caused. The fixes are independent and can be built/tested in parallel once understood, but should ship together to avoid partial breakage. Risk is highest on trailing-closure disambiguation (parser context plumbing), moderate on else-if codegen (type storage gap), and low on multiline fn calls (same category of bug as else-if).

## Recommendation

Build in this order: (1) trailing-closure parser fix, (2) else-if codegen fix, (3) multiline fn call typechecker fix. Each produces its own e2e regression tests. Verify all 303 existing e2e tests pass after each fix to catch regressions early. The trailing-closure fix is the riskiest because it touches the Pratt parser's postfix loop and must not break existing trailing-closure patterns (`test("name") do ... end`).

## Requirements Targeted

- **R016** (active, owner:M031/S01) — Trailing-closure disambiguation in control flow
- **R015** (active, owner:M031/S01) — `else if` chains must produce correct branch values
- **R017** (active, owner:M031/S01) — Multiline fn calls must resolve correct return types

## Implementation Landscape

### Bug 1 — R016: Trailing-Closure Disambiguation

**Confirmed behavior:** `if is_big(15) do ... end` fails with parse error. The parser sees `is_big(15)` as a call expression, then at line 111–113 of `compiler/mesh-parser/src/parser/expressions.rs`, the postfix loop checks `p.at(SyntaxKind::DO_KW)` and eagerly parses a trailing closure. The `if` condition parser then fails to find its expected `DO_KW`.

**Root cause:** The Pratt parser's postfix CALL_EXPR handler unconditionally checks for trailing closures after any `(...)` arg list when `DO_KW` follows. There's no awareness of whether the call is inside a control-flow condition.

**Affected control flows:** `if`, `while`, `case`, `for ... in` — all four call `expr(p)` for their condition/scrutinee/iterable, and all four expect `DO_KW` immediately after.

**Fix approach (per D027):** Add a `suppress_trailing_closure: bool` field to `Parser` (line ~97 of `compiler/mesh-parser/src/parser/mod.rs`). Set it to `true` before calling `expr(p)` in each control-flow condition position, restore it after. Check it at line 111 of `expressions.rs`:

```
// In the postfix CALL_EXPR handler:
if p.at(SyntaxKind::DO_KW) && !p.suppress_trailing_closure {
    parse_trailing_closure(p);
}
```

**Condition sites to patch (4 total):**
- `parse_if_expr` — line 870 (`expr(p)` for condition)
- `parse_while_expr` — line 1470 (`expr(p)` for condition)
- `parse_case_expr` — line 919 (`expr(p)` for scrutinee)
- `parse_for_in_expr` — line 1554 (`expr(p)` for iterable, after `in`)

**Must-not-break:** `test("...") do ... end`, `describe("...") do ... end` — these are top-level expression statements, not inside control-flow conditions, so `suppress_trailing_closure` will be `false` and they work unchanged.

**Files changed:**
- `compiler/mesh-parser/src/parser/mod.rs` — add field + accessor
- `compiler/mesh-parser/src/parser/expressions.rs` — guard trailing closure + set flag in 4 functions

**Tests:**
- Parser unit tests: `if is_big(15) do ... end`, `while running() do ... end`, `case get_val() do ... end`, `for x in get_list() do ... end` — all should produce correct CST
- Parser regression tests: `test("name") do ... end`, `describe("name") do ... end` — must still parse as trailing closures
- E2e tests: compile and run programs using `if fn_call() do` patterns

### Bug 2 — R015: `else if` Codegen

**Confirmed behavior:** `else if` chains with strings crash at runtime (misaligned pointer dereference). With integers, they return wrong values (e.g., 9 instead of 20 for `x == 2` branch).

**Root cause:** In the typechecker (`compiler/mesh-typeck/src/infer.rs`), `infer_if` at line 6976 handles `else if` chains by recursively calling itself at line 7014. However, the outer call goes through `infer_expr` (line 5766), which stores the result type at line 5987: `types.insert(expr.syntax().text_range(), resolved.clone())`. The recursive inner call at line 7014 calls `infer_if` **directly** — NOT through `infer_expr` — so the inner if-expression's type is **never stored** in the `types` map.

In the codegen (`compiler/mesh-codegen/src/mir/lower.rs`), `lower_if_expr` at line 8937 does `self.resolve_range(if_.syntax().text_range())`. For the recursively-lowered inner `else if`, this lookup finds nothing in the `types` map and falls back to `MirType::Unit` (line 456). The LLVM if-expression codegen at `compiler/mesh-codegen/src/codegen/expr.rs:1786` then allocates a Unit-sized result slot and stores a String/Int value into it — type mismatch causes crashes or garbage.

**Fix approach:** In `infer_if`, store the result type in the `types` map before returning:

```rust
fn infer_if(...) -> Result<Ty, TypeError> {
    // ... existing code ...
    
    let result = if let Some(else_branch) = if_.else_branch() {
        // ... existing else handling ...
        then_ty
    } else {
        then_ty
    };
    
    // Store the if-expression's type so codegen can find it
    let resolved = ctx.resolve(result.clone());
    types.insert(if_.syntax().text_range(), resolved);
    
    Ok(result)
}
```

**Files changed:**
- `compiler/mesh-typeck/src/infer.rs` — add `types.insert` in `infer_if`

**Tests:**
- E2e tests: `else if` chains returning Int, String, and Bool — verify correct values
- E2e tests: nested `else if` chains (3+ levels)
- E2e tests: `else if` with expression values used in `let` bindings

### Bug 3 — R017: Multiline Fn Call Type Resolution

**Confirmed behavior:** `add(\n  1,\n  2\n)` where `add` is a user-defined function resolves to `Unit`. Single-line `add(1, 2)` compiles (though has a separate inference issue). Built-in calls like `println(\n  "hello"\n)` work fine because their types are resolved through different paths.

**Root cause:** Same category as Bug 2 — the `types` map is missing an entry for the CALL_EXPR's text range when the call spans multiple lines. The parser produces a correct single CALL_EXPR node (confirmed by formatter round-trip and existing parser test `newlines_inside_parens_ignored`). The typechecker's `infer_expr` at line 5987 stores `types.insert(expr.syntax().text_range(), resolved)`. The question is whether `infer_call` returns the correct type for multiline calls or whether the lookup in the codegen uses a different range.

**Investigation needed during implementation:** The exact failure point needs to be traced during the fix. Hypotheses in priority order:
1. `infer_call` correctly resolves the type but something about how `infer_expr` stores it differs for multiline spans — check if `text_range()` of a CST node spanning multiple lines matches between typechecker storage and codegen lookup
2. `infer_call`'s arg inference fails silently for multiline calls — check if `arg_list.args()` returns the correct args when the ARG_LIST spans multiple lines
3. The issue is in how `infer_call` unifies the callee type with the expected function type — the fresh type variable might not get resolved properly

The fix is likely in `compiler/mesh-typeck/src/infer.rs` in or around `infer_call` (line 6265).

**Files changed:**
- `compiler/mesh-typeck/src/infer.rs` — fix in or around `infer_call`

**Tests:**
- E2e tests: multiline user-defined function calls with Int, String return types
- E2e tests: multiline calls with 2+ args on separate lines
- E2e tests: mixed single-line and multiline calls in same function

## Verification

### During implementation (per-fix)
```bash
# After each fix, run the full existing suite to detect regressions:
cargo test -p mesh-parser -- --lib               # 17 parser unit tests
cargo test -p mesh-parser                         # 238 parser tests (including integration)
cargo test -p mesh-typeck                         # 13 typechecker tests
cargo test -p mesh-codegen --lib                  # 179 codegen unit tests
cargo test -p meshc --test e2e                    # 303 e2e tests
```

### Slice acceptance
```bash
# All existing tests pass:
cargo test -p meshc --test e2e                    # 303+ tests (some new ones added)
cargo test -p mesh-parser                         # 238+ tests

# New e2e test files pass (written during this slice):
# - tests covering `if fn_call() do`, `while fn_call() do`, `case fn_call() do`, `for x in fn_call() do`
# - tests covering `else if` chains with Int, String, Bool
# - tests covering multiline function calls

# Dogfood builds still work:
cargo run -p meshc -- build reference-backend
cargo run -p meshc -- build mesher
```

## Risks and Mitigations

1. **Trailing-closure disambiguation breaks test DSL** — Mitigated by the design: `suppress_trailing_closure` is only `true` inside control-flow condition positions. `test("name") do` and `describe("name") do` are top-level expression statements where the flag is `false`. Regression test confirms.

2. **`else if` fix changes type resolution for working code** — Low risk. The fix adds type storage that was missing. Existing code that works only goes through `infer_expr` which already stores types. The fix only affects the recursive `infer_if` path.

3. **Multiline fn call fix has unknown root cause** — Medium risk. The exact failure point isn't fully traced yet. Implementation may require debugging with print statements or a test harness to compare `TextRange` keys. Budget extra time for diagnosis.

## Natural Task Boundaries

1. **T01: Trailing-closure parser fix** — `mod.rs` field + `expressions.rs` guard + 4 condition sites. Parser tests + e2e tests. Independent of other fixes.
2. **T02: Else-if codegen fix** — Single `types.insert` in `infer.rs`. E2e tests. Independent.
3. **T03: Multiline fn call typechecker fix** — Diagnosis + fix in `infer.rs`. E2e tests. Independent but may take longer due to unknown root cause.

All three can be verified independently against the existing 303 e2e tests. No ordering dependency between them.
