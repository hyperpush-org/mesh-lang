# T01: 108-aggregations 01

**Slice:** S03 — **Milestone:** M021

## Description

Add five aggregate select functions to the Query builder (select_count, select_sum, select_avg, select_min, select_max) with full compiler pipeline registration and tests.

Purpose: Enable Mesh programs to compute aggregate statistics via the Query builder API, completing AGG-01 and AGG-02. The existing group_by (AGG-03) and having (AGG-04) functions are already implemented in the pipeline -- this plan verifies them via unit tests and E2E compilation tests that compose aggregates with grouping.

Output: Five new extern C functions in query.rs, registered across typechecker/MIR/codegen/JIT, with unit tests for SQL generation and E2E tests for full compiler pipeline.

## Must-Haves

- [ ] "Query.select_count() generates SELECT count(*) in SQL output"
- [ ] "Query.select_sum/avg/min/max(field) generate correct aggregate SELECT expressions"
- [ ] "Query.group_by(field) already works and generates GROUP BY clause"
- [ ] "Query.having(clause, value) already works and generates HAVING clause with parameterized value"
- [ ] "Aggregation functions compose with group_by and having in pipe chains"

## Files

- `crates/mesh-rt/src/db/query.rs`
- `crates/mesh-rt/src/db/repo.rs`
- `crates/mesh-rt/src/lib.rs`
- `crates/mesh-codegen/src/mir/lower.rs`
- `crates/mesh-codegen/src/codegen/intrinsics.rs`
- `crates/mesh-repl/src/jit.rs`
- `crates/mesh-typeck/src/infer.rs`
- `crates/meshc/tests/e2e.rs`
