# S01: Parser & Codegen Fixes — UAT Script

## Preconditions

- Rust toolchain installed with `cargo` available
- Working directory: repo root (`mesh-lang/`)
- No stale `reference-backend` processes on port 18080

## Test Cases

### TC-01: Trailing-closure disambiguation — `if fn_call() do`

**Steps:**
1. Run `cargo test -p meshc --test e2e e2e_trailing_closure_if_fn_call_condition -- --nocapture`
2. Observe test compiles Mesh source with `if is_big(15) do` and runs the binary

**Expected:** Test passes. Output includes the correct branch result (e.g., "big" or "small" based on the condition). No parse errors.

### TC-02: Trailing-closure disambiguation — `while fn_call() do`

**Steps:**
1. Run `cargo test -p meshc --test e2e e2e_trailing_closure_while_fn_call_condition -- --nocapture`

**Expected:** Test passes. `while always_false() do` correctly parses `do` as block opener and the loop body never executes.

### TC-03: Trailing-closure disambiguation — `case fn_call() do`

**Steps:**
1. Run `cargo test -p meshc --test e2e e2e_trailing_closure_case_fn_call_scrutinee -- --nocapture`

**Expected:** Test passes. `case get_value() do` parses correctly and pattern-matches the return value.

### TC-04: Trailing-closure disambiguation — `for x in fn_call() do`

**Steps:**
1. Run `cargo test -p meshc --test e2e e2e_trailing_closure_for_fn_call_iterable -- --nocapture`

**Expected:** Test passes. `for x in get_list() do` iterates over the function's return value.

### TC-05: Trailing-closure regression — `test("name") do ... end`

**Steps:**
1. Run `cargo test -p meshc --test e2e e2e_trailing_closure_still_works -- --nocapture`

**Expected:** Test passes. `test("name") do ... end` and `describe("name") do ... end` still parse as trailing closures (not misinterpreted as control flow).

### TC-06: Else-if chain — Int return value

**Steps:**
1. Run `cargo test -p meshc --test e2e e2e_else_if_chain_int_value -- --nocapture`

**Expected:** Test passes. An `if/else if/else` chain returns the correct integer for each branch.

### TC-07: Else-if chain — String return value (crash sentinel)

**Steps:**
1. Run `cargo test -p meshc --test e2e e2e_else_if_chain_string_value -- --nocapture`

**Expected:** Test passes with correct string output. No segfault, no misaligned pointer crash. This is the most sensitive regression sentinel — a type-map failure here causes a runtime crash.

### TC-08: Else-if chain — Bool return value

**Steps:**
1. Run `cargo test -p meshc --test e2e e2e_else_if_chain_bool_value -- --nocapture`

**Expected:** Test passes with correct boolean output.

### TC-09: Else-if chain — 3-level deep

**Steps:**
1. Run `cargo test -p meshc --test e2e e2e_else_if_three_level_chain -- --nocapture`

**Expected:** Test passes. An `if/else if/else if/else` chain with 4 branches returns the correct value from each branch.

### TC-10: Else-if chain — let binding

**Steps:**
1. Run `cargo test -p meshc --test e2e e2e_else_if_let_binding -- --nocapture`

**Expected:** Test passes. The result of an `if/else if/else` chain can be bound with `let` and used in a subsequent expression.

### TC-11: Multiline function call — Int return

**Steps:**
1. Run `cargo test -p meshc --test e2e e2e_multiline_call_int_return -- --nocapture`

**Expected:** Test passes. A function call with int arguments on separate lines returns the correct sum, not 0 or garbage.

### TC-12: Multiline function call — String return (crash sentinel)

**Steps:**
1. Run `cargo test -p meshc --test e2e e2e_multiline_call_string_return -- --nocapture`

**Expected:** Test passes with correct string output. No segfault, no misaligned pointer crash. This is the most sensitive sentinel — trivia leaking through `Literal::token()` causes a crash here.

### TC-13: Multiline function call — 3 arguments

**Steps:**
1. Run `cargo test -p meshc --test e2e e2e_multiline_call_three_args -- --nocapture`

**Expected:** Test passes. A function with 3 arguments on separate lines returns the correct result.

### TC-14: Multiline function call — mixed single-line and multiline

**Steps:**
1. Run `cargo test -p meshc --test e2e e2e_multiline_call_mixed_single_and_multi -- --nocapture`

**Expected:** Test passes. The same function called in single-line and multiline form produces identical results.

### TC-15: Multiline function call — let binding

**Steps:**
1. Run `cargo test -p meshc --test e2e e2e_multiline_call_let_binding -- --nocapture`

**Expected:** Test passes. The result of a multiline function call can be bound with `let` and used in a subsequent expression.

### TC-16: No regressions in existing test suite

**Steps:**
1. Run `cargo test -p meshc --test e2e` (full suite)

**Expected:** 308+ tests pass. The only failures are the 10 pre-existing `try_*`/`from_try_*` tests (runtime crashes, exit code None) — these are unrelated to S01.

### TC-17: Dogfood builds

**Steps:**
1. Run `cargo run -p meshc -- build reference-backend`
2. Run `cargo run -p meshc -- build mesher`

**Expected:** Both compile successfully with no errors.

### TC-18: Parser unit tests

**Steps:**
1. Run `cargo test -p mesh-parser --lib`

**Expected:** 17/17 tests pass.

## Edge Cases

### EC-01: Nested control-flow with fn-call conditions

Write Mesh source with nested `if fn_call() do ... if other_fn() do ... end ... end`. The suppress_trailing_closure flag uses save/restore, so nesting should work correctly.

### EC-02: Deeply nested else-if (4+ levels)

Write Mesh source with `if/else if/else if/else if/else`. Each level should produce the correct type because `types.insert` fires on every recursive `infer_if` return.

### EC-03: Multiline call inside an if condition

`if add(\n  1,\n  2\n) == 3 do` should parse and typecheck correctly — the trailing-closure suppression and multiline literal fix interact here.
