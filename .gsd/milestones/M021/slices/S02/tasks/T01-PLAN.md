# T01: 107-joins 01

**Slice:** S02 — **Milestone:** M021

## Description

Add table alias support to Query.join via a new Query.join_as function, and add comprehensive unit tests and E2E tests verifying inner join, left join, multi-join chaining, and aliased join SQL generation across the full compiler pipeline.

Purpose: The existing Query.join function works for basic joins but Mesher's real-world queries use table aliases extensively (e.g., `projects p JOIN api_keys ak ON ak.project_id = p.id`). Adding alias support now and verifying all join variants with thorough tests prepares for the Mesher rewrite in Phases 110-113 and satisfies all four JOIN requirements (JOIN-01 through JOIN-04).

Output: Query.join_as runtime function registered across the full pipeline (typechecker, MIR, LLVM codegen, JIT, runtime), unit tests for all join SQL patterns, and E2E tests proving the compiler pipeline handles join queries correctly.

## Must-Haves

- [ ] "A Mesh program can write Query.join(:inner, \"table\", \"on_clause\") and the generated SQL includes INNER JOIN with correct ON clause"
- [ ] "A Mesh program can write a left join and the generated SQL includes LEFT JOIN"
- [ ] "A Mesh program can chain multiple joins and all generate correct SQL with proper table references"
- [ ] "A Mesh program can use Query.join_as to create a join with a table alias for qualified column access"
- [ ] "Unit tests verify multi-join, left join, and alias join SQL generation produce correct parameterized SQL"

## Files

- `crates/mesh-rt/src/db/query.rs`
- `crates/mesh-rt/src/db/repo.rs`
- `crates/mesh-rt/src/lib.rs`
- `crates/mesh-codegen/src/mir/lower.rs`
- `crates/mesh-codegen/src/codegen/intrinsics.rs`
- `crates/mesh-repl/src/jit.rs`
- `crates/mesh-typeck/src/infer.rs`
- `crates/meshc/tests/e2e.rs`
