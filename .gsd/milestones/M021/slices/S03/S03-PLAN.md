# S03: Aggregations

**Goal:** Add five aggregate select functions to the Query builder (select_count, select_sum, select_avg, select_min, select_max) with full compiler pipeline registration and tests.
**Demo:** Add five aggregate select functions to the Query builder (select_count, select_sum, select_avg, select_min, select_max) with full compiler pipeline registration and tests.

## Must-Haves


## Tasks

- [x] **T01: 108-aggregations 01** `est:4min`
  - Add five aggregate select functions to the Query builder (select_count, select_sum, select_avg, select_min, select_max) with full compiler pipeline registration and tests.

Purpose: Enable Mesh programs to compute aggregate statistics via the Query builder API, completing AGG-01 and AGG-02. The existing group_by (AGG-03) and having (AGG-04) functions are already implemented in the pipeline -- this plan verifies them via unit tests and E2E compilation tests that compose aggregates with grouping.

Output: Five new extern C functions in query.rs, registered across typechecker/MIR/codegen/JIT, with unit tests for SQL generation and E2E tests for full compiler pipeline.
- [x] **T02: 108-aggregations 02** `est:1min`
  - Verify aggregation functions execute correctly at runtime against real SQLite databases, proving count/sum/avg/min/max return correct values with GROUP BY grouping and HAVING filtering.

Purpose: Close the runtime verification gap -- Plan 01 proves the compiler pipeline, this plan proves the generated SQL executes against real data and returns correct results. This follows the same pattern as Phase 107's runtime JOIN verification.

Output: Mesh fixture file and Rust E2E test with assertions for all four aggregation requirements.

## Files Likely Touched

- `crates/mesh-rt/src/db/query.rs`
- `crates/mesh-rt/src/db/repo.rs`
- `crates/mesh-rt/src/lib.rs`
- `crates/mesh-codegen/src/mir/lower.rs`
- `crates/mesh-codegen/src/codegen/intrinsics.rs`
- `crates/mesh-repl/src/jit.rs`
- `crates/mesh-typeck/src/infer.rs`
- `crates/meshc/tests/e2e.rs`
- `tests/e2e/sqlite_aggregate_runtime.mpl`
- `crates/meshc/tests/e2e_stdlib.rs`
