# T01: 109-upserts-returning-subqueries 01

**Slice:** S04 — **Milestone:** M021

## Description

Add upsert (INSERT ON CONFLICT DO UPDATE), RETURNING for delete_where, and subquery WHERE support to the Mesh ORM with full compiler pipeline registration and tests.

Purpose: Enable Mesh programs to perform upsert operations (critical for Mesher's issue deduplication), retrieve deleted rows via RETURNING, and use nested subqueries for complex filtering -- completing all three Phase 109 requirements.

Output: Three new extern C functions (mesh_repo_insert_or_update, mesh_repo_delete_where_returning, mesh_query_where_sub) registered across typechecker/MIR/codegen/JIT, with unit tests for SQL generation and E2E tests for full compiler pipeline.

## Must-Haves

- [ ] "Repo.insert_or_update generates INSERT ... ON CONFLICT (target) DO UPDATE SET ... RETURNING * SQL"
- [ ] "Repo.delete_where_returning deletes matching rows and returns the deleted rows instead of a count"
- [ ] "Query.where_sub nests a subquery in the WHERE clause with correct parameter binding"
- [ ] "All three functions compile and execute through the full pipeline (typechecker, MIR, codegen, JIT) -- Repo tests call the functions against in-memory SQLite, Query.where_sub builds a query structure in memory"

## Files

- `crates/mesh-rt/src/db/query.rs`
- `crates/mesh-rt/src/db/repo.rs`
- `crates/mesh-rt/src/db/orm.rs`
- `crates/mesh-rt/src/lib.rs`
- `crates/mesh-codegen/src/mir/lower.rs`
- `crates/mesh-codegen/src/codegen/intrinsics.rs`
- `crates/mesh-repl/src/jit.rs`
- `crates/mesh-typeck/src/infer.rs`
- `crates/meshc/tests/e2e.rs`
