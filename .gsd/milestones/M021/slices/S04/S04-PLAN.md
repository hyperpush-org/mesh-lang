# S04: Upserts Returning Subqueries

**Goal:** Add upsert (INSERT ON CONFLICT DO UPDATE), RETURNING for delete_where, and subquery WHERE support to the Mesh ORM with full compiler pipeline registration and tests.
**Demo:** Add upsert (INSERT ON CONFLICT DO UPDATE), RETURNING for delete_where, and subquery WHERE support to the Mesh ORM with full compiler pipeline registration and tests.

## Must-Haves


## Tasks

- [x] **T01: 109-upserts-returning-subqueries 01** `est:10min`
  - Add upsert (INSERT ON CONFLICT DO UPDATE), RETURNING for delete_where, and subquery WHERE support to the Mesh ORM with full compiler pipeline registration and tests.

Purpose: Enable Mesh programs to perform upsert operations (critical for Mesher's issue deduplication), retrieve deleted rows via RETURNING, and use nested subqueries for complex filtering -- completing all three Phase 109 requirements.

Output: Three new extern C functions (mesh_repo_insert_or_update, mesh_repo_delete_where_returning, mesh_query_where_sub) registered across typechecker/MIR/codegen/JIT, with unit tests for SQL generation and E2E tests for full compiler pipeline.
- [x] **T02: 109-upserts-returning-subqueries 02** `est:20min`
  - Verify upsert, RETURNING delete, and subquery WHERE execute correctly against real SQLite data at runtime by calling Repo.insert_or_update and Repo.delete_where_returning directly.

Purpose: Plan 01 verifies the compiler pipeline accepts the new functions. This plan verifies the generated SQL is syntactically valid and produces correct results against a real database. Unlike 108-02 which used raw SQL equivalents, this plan calls `Repo.insert_or_update` and `Repo.delete_where_returning` directly to prove the `mesh_repo_insert_or_update` and `mesh_repo_delete_where_returning` runtime functions and their SQL generation work end-to-end. Uses SQLite (same pattern as phases 107-02 and 108-02) since it runs in-memory without external dependencies.

Output: Mesh fixture with Rust E2E test proving all three Phase 109 requirements work at runtime via actual Repo function calls.

## Files Likely Touched

- `crates/mesh-rt/src/db/query.rs`
- `crates/mesh-rt/src/db/repo.rs`
- `crates/mesh-rt/src/db/orm.rs`
- `crates/mesh-rt/src/lib.rs`
- `crates/mesh-codegen/src/mir/lower.rs`
- `crates/mesh-codegen/src/codegen/intrinsics.rs`
- `crates/mesh-repl/src/jit.rs`
- `crates/mesh-typeck/src/infer.rs`
- `crates/meshc/tests/e2e.rs`
- `tests/e2e/sqlite_upsert_subquery_runtime.mpl`
- `crates/meshc/tests/e2e_stdlib.rs`
