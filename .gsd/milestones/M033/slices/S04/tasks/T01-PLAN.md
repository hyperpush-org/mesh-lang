---
estimated_steps: 4
estimated_files: 8
skills_used:
  - rust-best-practices
  - postgresql-database-engineering
---

# T01: Add honest migration index support and explicit PG schema helpers

Why: S04 cannot safely rewrite Mesher until the runtime/compiler boundary exposes the honest helper split the roadmap calls for.

Steps:
1. Extend `Migration.create_index(...)` in `compiler/mesh-rt/src/db/migration.rs` so `options` supports exact `name:...` and the `columns` list can carry `:ASC` / `:DESC` sort specs, with unit tests proving names, partial predicates, and ordered-column rendering while keeping PG-only features out of the neutral parser.
2. Add explicit PostgreSQL schema helpers under the `Pg` namespace for `create_extension(pool, name)`, `create_range_partitioned_table(pool, table, columns, partition_column)`, `create_gin_index(pool, table, index_name, column, opclass)`, `create_daily_partitions_ahead(pool, parent_table, days)`, `list_daily_partitions_before(pool, parent_table, max_days)`, and a quoted `drop_partition(pool, partition_name)` helper that never trusts unquoted identifiers.
3. Wire those helpers through `mesh-rt`, `mesh-typeck`, MIR lowering, LLVM intrinsics, and the REPL JIT using the same explicit `Pg` namespacing pattern S02 established.
4. Keep the helper implementations pure/testable where possible, and make DB-clock/date math stay inside the PG helper family instead of moving partition naming onto host time.

Must-Haves:
- [ ] `Migration.create_index(...)` can preserve Mesher’s exact index names and ordered-column definitions without pretending `USING` / opclass / partition DDL are neutral.
- [ ] The explicit `Pg` helper family covers the extension, partitioned-parent, GIN/opclass, and runtime daily partition create/list/drop cases Mesher actually uses.
- [ ] Compiler/runtime/repl wiring is complete enough for Mesh code and migration generation to call the new helpers.

## Inputs

- `compiler/mesh-rt/src/db/migration.rs`
- `compiler/mesh-rt/src/db/mod.rs`
- `compiler/mesh-rt/src/lib.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-repl/src/jit.rs`

## Expected Output

- `compiler/mesh-rt/src/db/migration.rs`
- `compiler/mesh-rt/src/db/pg_schema.rs`
- `compiler/mesh-rt/src/db/mod.rs`
- `compiler/mesh-rt/src/lib.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-repl/src/jit.rs`

## Verification

cargo test -p mesh-rt migration -- --nocapture
cargo build -p meshc

## Observability Impact

- Signals added/changed: helper-specific runtime/typecheck errors should name the new `Pg` schema function or `Migration.create_index` parse surface instead of failing as generic raw SQL strings.
- How a future agent inspects this: run `cargo test -p mesh-rt migration -- --nocapture` and inspect the new builder/unit tests in `compiler/mesh-rt/src/db/{migration,pg_schema}.rs`.
- Failure state exposed: bad identifier quoting, incorrect order/name rendering, or missing intrinsic registration should fail deterministically during unit tests or `cargo build -p meshc`.
