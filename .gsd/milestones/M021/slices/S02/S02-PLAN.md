# S02: Joins

**Goal:** Add table alias support to Query.
**Demo:** Add table alias support to Query.

## Must-Haves


## Tasks

- [x] **T01: 107-joins 01** `est:6min`
  - Add table alias support to Query.join via a new Query.join_as function, and add comprehensive unit tests and E2E tests verifying inner join, left join, multi-join chaining, and aliased join SQL generation across the full compiler pipeline.

Purpose: The existing Query.join function works for basic joins but Mesher's real-world queries use table aliases extensively (e.g., `projects p JOIN api_keys ak ON ak.project_id = p.id`). Adding alias support now and verifying all join variants with thorough tests prepares for the Mesher rewrite in Phases 110-113 and satisfies all four JOIN requirements (JOIN-01 through JOIN-04).

Output: Query.join_as runtime function registered across the full pipeline (typechecker, MIR, LLVM codegen, JIT, runtime), unit tests for all join SQL patterns, and E2E tests proving the compiler pipeline handles join queries correctly.
- [x] **T02: 107-joins 02** `est:1min`
  - Close verification gaps 1-3 from 107-VERIFICATION.md by adding runtime database tests for JOIN queries and updating requirement tracking.

Purpose: Phase 107 implemented JOIN support and verified it compiles, but ROADMAP success criteria SC2 and SC4 require runtime verification -- executing joins against a real database and inspecting returned rows. The existing `e2e_sqlite` test pattern (Sqlite.open(":memory:"), execute DDL/DML, query, inspect Map results) is the proven approach. Gap 3 is bookkeeping.

Output: One new Mesh fixture + one new Rust E2E test proving JOINs work at runtime, plus updated requirement tracking files.

## Files Likely Touched

- `crates/mesh-rt/src/db/query.rs`
- `crates/mesh-rt/src/db/repo.rs`
- `crates/mesh-rt/src/lib.rs`
- `crates/mesh-codegen/src/mir/lower.rs`
- `crates/mesh-codegen/src/codegen/intrinsics.rs`
- `crates/mesh-repl/src/jit.rs`
- `crates/mesh-typeck/src/infer.rs`
- `crates/meshc/tests/e2e.rs`
- `tests/e2e/sqlite_join_runtime.mpl`
- `crates/meshc/tests/e2e_stdlib.rs`
- `.planning/REQUIREMENTS.md`
- `.planning/phases/107-joins/107-01-SUMMARY.md`
