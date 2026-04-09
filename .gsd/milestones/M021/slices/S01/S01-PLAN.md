# S01: Advanced Where Operators And Raw Sql Fragments

**Goal:** Add NOT IN, BETWEEN, ILIKE, and OR operators to the Mesh query builder, completing WHERE-02 through WHERE-06 requirements.
**Demo:** Add NOT IN, BETWEEN, ILIKE, and OR operators to the Mesh query builder, completing WHERE-02 through WHERE-06 requirements.

## Must-Haves


## Tasks

- [x] **T01: 106-advanced-where-operators-and-raw-sql-fragments 01** `est:8min`
  - Add NOT IN, BETWEEN, ILIKE, and OR operators to the Mesh query builder, completing WHERE-02 through WHERE-06 requirements.

Purpose: Mesh programs need rich WHERE clause capabilities to replace raw SQL queries in Mesher. NOT IN, BETWEEN, ILIKE, and OR are used extensively in Mesher's 68+ raw SQL queries.
Output: Four new Query builder functions (where_not_in, where_between, ILIKE via where_op, where_or) with full pipeline from Mesh syntax through MIR lowering to SQL generation, plus E2E tests.
- [x] **T02: 106-advanced-where-operators-and-raw-sql-fragments 02** `est:8min`
  - Fix fragment parameter renumbering ($1-style placeholders) and add fragment support in ORDER BY and GROUP BY positions, completing FRAG-01 through FRAG-04 requirements.

Purpose: Mesher uses PG-specific expressions like `crypt($1, gen_salt('bf'))`, `metadata @> $1::jsonb`, `date_trunc('hour', received_at)`, and `random()` in queries. Fragments must support $1-style parameter placeholders that get renumbered correctly when combined with other WHERE clauses, and must work in ORDER BY and GROUP BY positions (not just appended to the end of the query).
Output: Fixed $N renumbering in fragment/where_raw, new Query.order_by_raw and Query.group_by_raw functions, comprehensive E2E tests.

## Files Likely Touched

- `crates/mesh-rt/src/db/query.rs`
- `crates/mesh-rt/src/db/repo.rs`
- `crates/mesh-rt/src/lib.rs`
- `crates/mesh-codegen/src/mir/lower.rs`
- `crates/mesh-codegen/src/codegen/intrinsics.rs`
- `crates/mesh-repl/src/jit.rs`
- `crates/meshc/tests/e2e.rs`
- `crates/mesh-rt/src/db/query.rs`
- `crates/mesh-rt/src/db/repo.rs`
- `crates/mesh-rt/src/lib.rs`
- `crates/mesh-codegen/src/mir/lower.rs`
- `crates/mesh-codegen/src/codegen/intrinsics.rs`
- `crates/mesh-repl/src/jit.rs`
- `crates/meshc/tests/e2e.rs`
