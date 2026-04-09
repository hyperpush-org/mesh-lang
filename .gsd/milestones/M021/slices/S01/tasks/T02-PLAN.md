# T02: 106-advanced-where-operators-and-raw-sql-fragments 02

**Slice:** S01 — **Milestone:** M021

## Description

Fix fragment parameter renumbering ($1-style placeholders) and add fragment support in ORDER BY and GROUP BY positions, completing FRAG-01 through FRAG-04 requirements.

Purpose: Mesher uses PG-specific expressions like `crypt($1, gen_salt('bf'))`, `metadata @> $1::jsonb`, `date_trunc('hour', received_at)`, and `random()` in queries. Fragments must support $1-style parameter placeholders that get renumbered correctly when combined with other WHERE clauses, and must work in ORDER BY and GROUP BY positions (not just appended to the end of the query).
Output: Fixed $N renumbering in fragment/where_raw, new Query.order_by_raw and Query.group_by_raw functions, comprehensive E2E tests.

## Must-Haves

- [ ] "Query.fragment(q, \"crypt($1, gen_salt('bf'))\", [password]) embeds the SQL with $1 renumbered to the correct parameter index"
- [ ] "Fragment $1-style placeholders are renumbered when prior WHERE clauses consume parameters"
- [ ] "Query.order_by_raw(q, \"random()\") generates ORDER BY random() verbatim"
- [ ] "Query.group_by_raw(q, \"date_trunc('hour', received_at)\") generates GROUP BY date_trunc('hour', received_at) verbatim"
- [ ] "Query.where_raw combined with Query.fragment in the same query generates correct parameter numbering"
- [ ] "Fragments with PG functions (crypt, gen_random_bytes, date_trunc, random) produce syntactically correct SQL"
- [ ] "Fragments with JSONB operators (metadata @> $1::jsonb, tags ? $1) produce correct SQL"

## Files

- `crates/mesh-rt/src/db/query.rs`
- `crates/mesh-rt/src/db/repo.rs`
- `crates/mesh-rt/src/lib.rs`
- `crates/mesh-codegen/src/mir/lower.rs`
- `crates/mesh-codegen/src/codegen/intrinsics.rs`
- `crates/mesh-repl/src/jit.rs`
- `crates/meshc/tests/e2e.rs`
