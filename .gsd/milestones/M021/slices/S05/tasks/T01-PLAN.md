# T01: 109.1-fix-the-issues-encountered-in-109 01

**Slice:** S05 — **Milestone:** M021

## Description

Fix the type checker arity bug (E0003) where `let x = Sqlite.execute(db, sql, params)?` followed by `f(x)` triggers a spurious arity mismatch error.

Purpose: This blocks using the result of any `Result`-returning function after the `?` operator, which is a core language feature. The Mesher rewrite (phases 110-113) will use this pattern extensively.

Output: Fixed type checker, regression E2E test.

## Must-Haves

- [ ] "let x = Sqlite.execute(db, sql, params)? followed by Int.to_string(x) compiles without E0003"
- [ ] "let x = Sqlite.execute(db, sql, params)? followed by println(Int.to_string(x)) compiles and runs correctly"
- [ ] "All existing typeck and E2E tests continue to pass"

## Files

- `crates/mesh-typeck/src/infer.rs`
- `crates/meshc/tests/e2e.rs`
