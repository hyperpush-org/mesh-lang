# T02: 109-upserts-returning-subqueries 02

**Slice:** S04 — **Milestone:** M021

## Description

Verify upsert, RETURNING delete, and subquery WHERE execute correctly against real SQLite data at runtime by calling Repo.insert_or_update and Repo.delete_where_returning directly.

Purpose: Plan 01 verifies the compiler pipeline accepts the new functions. This plan verifies the generated SQL is syntactically valid and produces correct results against a real database. Unlike 108-02 which used raw SQL equivalents, this plan calls `Repo.insert_or_update` and `Repo.delete_where_returning` directly to prove the `mesh_repo_insert_or_update` and `mesh_repo_delete_where_returning` runtime functions and their SQL generation work end-to-end. Uses SQLite (same pattern as phases 107-02 and 108-02) since it runs in-memory without external dependencies.

Output: Mesh fixture with Rust E2E test proving all three Phase 109 requirements work at runtime via actual Repo function calls.

## Must-Haves

- [ ] "Repo.insert_or_update executes against real SQLite, proving ON CONFLICT DO UPDATE SET RETURNING SQL is syntactically valid and produces correct upsert semantics"
- [ ] "Repo.delete_where_returning executes against real SQLite, proving DELETE ... RETURNING * SQL works and returns deleted rows"
- [ ] "Subquery WHERE (IN subselect) correctly filters rows based on nested query results"
- [ ] "All three features execute against real SQLite data with exact value assertions"

## Files

- `tests/e2e/sqlite_upsert_subquery_runtime.mpl`
- `crates/meshc/tests/e2e_stdlib.rs`
