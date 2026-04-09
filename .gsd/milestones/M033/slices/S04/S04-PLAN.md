# S04: Schema extras and live partition lifecycle proof

**Goal:** Retire Mesher’s S04-owned raw schema and partition DDL honestly by adding only the neutral migration improvements that stay truthful (`Migration.create_index` name/order support), putting PostgreSQL-only schema behavior behind explicit `Pg` helpers, rewriting the initial migration and runtime partition lifecycle around those helpers, and proving the result against live Postgres catalogs plus the real Mesher startup path.
**Demo:** After this: After this: Mesher migrations and runtime retention/schema flows create, list, and drop partitions plus related PG schema extras through first-class helpers on a live Postgres database.

## Tasks
- [x] **T01: Add explicit Pg schema helpers and honest Migration.create_index support** — Why: S04 cannot safely rewrite Mesher until the runtime/compiler boundary exposes the honest helper split the roadmap calls for.

Steps:
1. Extend `Migration.create_index(...)` in `compiler/mesh-rt/src/db/migration.rs` so `options` supports exact `name:...` and the `columns` list can carry `:ASC` / `:DESC` sort specs, with unit tests proving names, partial predicates, and ordered-column rendering while keeping PG-only features out of the neutral parser.
2. Add explicit PostgreSQL schema helpers under the `Pg` namespace for `create_extension(pool, name)`, `create_range_partitioned_table(pool, table, columns, partition_column)`, `create_gin_index(pool, table, index_name, column, opclass)`, `create_daily_partitions_ahead(pool, parent_table, days)`, `list_daily_partitions_before(pool, parent_table, max_days)`, and a quoted `drop_partition(pool, partition_name)` helper that never trusts unquoted identifiers.
3. Wire those helpers through `mesh-rt`, `mesh-typeck`, MIR lowering, LLVM intrinsics, and the REPL JIT using the same explicit `Pg` namespacing pattern S02 established.
4. Keep the helper implementations pure/testable where possible, and make DB-clock/date math stay inside the PG helper family instead of moving partition naming onto host time.

Must-Haves:
- [ ] `Migration.create_index(...)` can preserve Mesher’s exact index names and ordered-column definitions without pretending `USING` / opclass / partition DDL are neutral.
- [ ] The explicit `Pg` helper family covers the extension, partitioned-parent, GIN/opclass, and runtime daily partition create/list/drop cases Mesher actually uses.
- [ ] Compiler/runtime/repl wiring is complete enough for Mesh code and migration generation to call the new helpers.
  - Estimate: 3h
  - Files: compiler/mesh-rt/src/db/migration.rs, compiler/mesh-rt/src/db/pg_schema.rs, compiler/mesh-rt/src/db/mod.rs, compiler/mesh-rt/src/lib.rs, compiler/mesh-typeck/src/infer.rs, compiler/mesh-codegen/src/mir/lower.rs, compiler/mesh-codegen/src/codegen/intrinsics.rs, compiler/mesh-repl/src/jit.rs
  - Verify: cargo test -p mesh-rt migration -- --nocapture
cargo build -p meshc
- [x] **T02: Rewrite Mesher's initial migration onto Migration/Pg helpers and update migration scaffolds** — Why: The densest S04-owned raw DDL cluster is the initial migration, so it should move first once the helper seam exists.

Steps:
1. Rewrite `mesher/migrations/20260216120000_create_initial_schema.mpl` so ordinary tables use `Migration.create_table(...)`, ordinary/partial/ordered indexes use the upgraded `Migration.create_index(...)` with exact `name:` options, and only the truly PG-only sites use the new `Pg` schema helpers.
2. Replace the raw PG-only families in that migration with explicit calls to `Pg.create_extension(...)`, `Pg.create_range_partitioned_table(...)`, and `Pg.create_gin_index(...)`, preserving the existing table/index names and predicates instead of introducing migration-name drift.
3. Update migration scaffolding/examples and compile-only coverage so new generated migrations teach the explicit PG helper path instead of `Migration.execute(...)` for schema extras.

Must-Haves:
- [ ] `mesher/migrations/20260216120000_create_initial_schema.mpl` no longer uses raw `Pool.execute(...)` for the S04-owned extension / partitioned-table / recurring-index families.
- [ ] Exact Mesher schema names (`idx_projects_slug`, `idx_issues_project_last_seen`, `idx_events_tags`, etc.) stay stable after the rewrite.
- [ ] `compiler/meshc/src/migrate.rs` and `compiler/meshc/tests/e2e.rs` stop teaching raw `Migration.execute(...)` as the default extension/schema-extra path.
  - Estimate: 2h
  - Files: mesher/migrations/20260216120000_create_initial_schema.mpl, compiler/meshc/src/migrate.rs, compiler/meshc/tests/e2e.rs
  - Verify: cargo test -p meshc --test e2e e2e_migration -- --nocapture
cargo run -q -p meshc -- build mesher
! rg -n "Pool\.execute\(pool" mesher/migrations/20260216120000_create_initial_schema.mpl
- [x] **T03: Move Mesher partition lifecycle into Storage.Schema and add live S04 verification coverage** — Why: S04 owns the remaining runtime partition/catalog keep-sites, and they should collapse onto the new explicit helper family instead of staying split between storage modules.

Steps:
1. Expand `mesher/storage/schema.mpl` so it owns partition create-ahead, expired-partition listing, and quoted drop behavior through the new `Pg.create_daily_partitions_ahead(...)`, `Pg.list_daily_partitions_before(...)`, and `Pg.drop_partition(...)` helpers.
2. Remove `get_expired_partitions(...)` / `drop_partition(...)` from `mesher/storage/queries.mpl`, update `mesher/services/retention.mpl` imports and call sites to use `Storage.Schema`, and keep per-project row deletion logic in `Storage.Queries` untouched.
3. Keep partition naming/date math aligned to PostgreSQL’s clock, not host time, and preserve or improve startup/retention logging in `mesher/main.mpl` / `mesher/services/retention.mpl` so failures localize cleanly.
4. Do not widen the generic query API here; all remaining schema/catalog behavior should stay explicitly PG-shaped.

Must-Haves:
- [ ] `mesher/storage/schema.mpl` becomes the sole Mesher module that owns partition create/list/drop helpers.
- [ ] `mesher/storage/queries.mpl` no longer exports the S04 partition/catalog keep-sites.
- [ ] Mesher startup and retention flows still call real partition lifecycle code on the live runtime path, but without `Repo.query_raw(...)` / `Repo.execute_raw(...)` in the owned functions.
  - Estimate: 2h
  - Files: mesher/storage/schema.mpl, mesher/storage/queries.mpl, mesher/services/retention.mpl, mesher/main.mpl
  - Verify: cargo run -q -p meshc -- fmt --check mesher
cargo run -q -p meshc -- build mesher
rg -n "pub fn (create_partitions_ahead|get_expired_partitions|drop_partition)" mesher/storage/schema.mpl
! rg -n "pub fn get_expired_partitions|pub fn drop_partition" mesher/storage/queries.mpl
- [x] **T04: Tightened S04 live Postgres proofs and verifier sweeps for migration/runtime partition boundaries** — Why: S04 is only complete once the new helper boundary is proven against real Postgres catalogs and the real Mesher startup path, not just string snapshots.

Steps:
1. Add `compiler/meshc/tests/e2e_m033_s04.rs`, reusing the S02/S03 Docker/Postgres and Mesher-spawn patterns, with named proofs for migration-time schema extras, startup partition creation from `mesher/main.mpl`, and runtime expired-partition list/drop behavior through the real storage helpers.
2. Assert catalog truth directly: `pg_extension` contains `pgcrypto`, `events` is a partitioned table in `pg_partitioned_table` / `pg_inherits`, the `tags` index uses `GIN` with `jsonb_path_ops`, startup-created future partitions exist, and dropped partitions disappear from `to_regclass(...)` / inheritance catalogs.
3. Add `scripts/verify-m033-s04.sh` to run the full S04 suite, Mesher fmt/build, and a mechanical sweep that bans S04 raw DDL/query regressions in the migration and runtime partition files.
4. Update `scripts/verify-m033-s03.sh` so the old verifier no longer silently excludes S04 partition/catalog keep-sites.

Must-Haves:
- [ ] `compiler/meshc/tests/e2e_m033_s04.rs` proves migration apply, startup partition creation, and list/drop cleanup on live Postgres with named `e2e_m033_s04_*` tests.
- [ ] `scripts/verify-m033-s04.sh` is the stable slice-level acceptance command and names the offending proof family or raw keep-site when it fails.
- [ ] `scripts/verify-m033-s03.sh` stops exempting the S04 partition/catalog helpers once the new verifier exists.
  - Estimate: 3h
  - Files: compiler/meshc/tests/e2e_m033_s04.rs, scripts/verify-m033-s04.sh, scripts/verify-m033-s03.sh, compiler/meshc/tests/e2e_m033_s03.rs
  - Verify: cargo test -p meshc --test e2e_m033_s04 -- --nocapture
cargo run -q -p meshc -- fmt --check mesher
cargo run -q -p meshc -- build mesher
bash scripts/verify-m033-s04.sh
