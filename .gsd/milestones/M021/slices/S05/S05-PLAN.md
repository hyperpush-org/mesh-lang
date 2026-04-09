# S05: Fix The Issues Encountered In 109

**Goal:** Fix the type checker arity bug (E0003) where `let x = Sqlite.
**Demo:** Fix the type checker arity bug (E0003) where `let x = Sqlite.

## Must-Haves


## Tasks

- [x] **T01: 109.1-fix-the-issues-encountered-in-109 01** `est:13min`
  - Fix the type checker arity bug (E0003) where `let x = Sqlite.execute(db, sql, params)?` followed by `f(x)` triggers a spurious arity mismatch error.

Purpose: This blocks using the result of any `Result`-returning function after the `?` operator, which is a core language feature. The Mesher rewrite (phases 110-113) will use this pattern extensively.

Output: Fixed type checker, regression E2E test.
- [x] **T02: 109.1-fix-the-issues-encountered-in-109 02** `est:4min`
  - Fix the latent service loop argument loading type mismatch where handler arguments are only distinguished as `ptr` vs `i64`, causing LLVM type errors for Bool (i1), Float (f64), and struct parameters.

Purpose: Without this fix, any service with Bool/Float/struct handler arguments will crash or produce incorrect results at the LLVM level. The Mesher rewrite will add services with diverse parameter types.

Output: Fixed service loop arg loading, E2E test with Bool handler argument.

## Files Likely Touched

- `crates/mesh-typeck/src/infer.rs`
- `crates/meshc/tests/e2e.rs`
- `crates/mesh-codegen/src/codegen/expr.rs`
- `crates/meshc/tests/e2e_concurrency_stdlib.rs`
- `tests/e2e/service_bool_return.mpl`
