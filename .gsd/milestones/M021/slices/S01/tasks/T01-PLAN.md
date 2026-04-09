# T01: 106-advanced-where-operators-and-raw-sql-fragments 01

**Slice:** S01 — **Milestone:** M021

## Description

Add NOT IN, BETWEEN, ILIKE, and OR operators to the Mesh query builder, completing WHERE-02 through WHERE-06 requirements.

Purpose: Mesh programs need rich WHERE clause capabilities to replace raw SQL queries in Mesher. NOT IN, BETWEEN, ILIKE, and OR are used extensively in Mesher's 68+ raw SQL queries.
Output: Four new Query builder functions (where_not_in, where_between, ILIKE via where_op, where_or) with full pipeline from Mesh syntax through MIR lowering to SQL generation, plus E2E tests.

## Must-Haves

- [ ] "Query.where_not_in(q, :status, [\"archived\", \"deleted\"]) generates WHERE status NOT IN ($1, $2)"
- [ ] "Query.where_between(q, :age, \"18\", \"65\") generates WHERE age BETWEEN $1 AND $2"
- [ ] "Query.where_op(q, :name, :ilike, \"%alice%\") generates WHERE name ILIKE $1"
- [ ] "Query.where_or(q, [[\"status\", \"active\"], [\"level\", \"error\"]]) generates WHERE (status = $1 OR level = $2)"
- [ ] "All new WHERE operators work in pipe chains with existing Query builder methods"
- [ ] "All parameter indices are correctly sequenced across mixed clause types"

## Files

- `crates/mesh-rt/src/db/query.rs`
- `crates/mesh-rt/src/db/repo.rs`
- `crates/mesh-rt/src/lib.rs`
- `crates/mesh-codegen/src/mir/lower.rs`
- `crates/mesh-codegen/src/codegen/intrinsics.rs`
- `crates/mesh-repl/src/jit.rs`
- `crates/meshc/tests/e2e.rs`
