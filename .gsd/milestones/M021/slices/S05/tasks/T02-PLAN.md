# T02: 109.1-fix-the-issues-encountered-in-109 02

**Slice:** S05 — **Milestone:** M021

## Description

Fix the latent service loop argument loading type mismatch where handler arguments are only distinguished as `ptr` vs `i64`, causing LLVM type errors for Bool (i1), Float (f64), and struct parameters.

Purpose: Without this fix, any service with Bool/Float/struct handler arguments will crash or produce incorrect results at the LLVM level. The Mesher rewrite will add services with diverse parameter types.

Output: Fixed service loop arg loading, E2E test with Bool handler argument.

## Must-Haves

- [ ] "Service handler receiving a Bool argument gets correctly truncated i1 value (not raw i64)"
- [ ] "Service handler receiving a Float argument gets correctly bitcast f64 value (not raw i64)"
- [ ] "Existing service tests (counter, bool_return, string_return, call_cast, state_management) all pass"

## Files

- `crates/mesh-codegen/src/codegen/expr.rs`
- `crates/meshc/tests/e2e_concurrency_stdlib.rs`
- `tests/e2e/service_bool_return.mpl`
