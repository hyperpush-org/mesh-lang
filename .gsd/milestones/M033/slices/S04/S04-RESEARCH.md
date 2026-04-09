# M033 / S04 — Research

**Date:** 2026-03-25
**Slice:** S04 — Schema extras and live partition lifecycle proof

## Summary

S04 owns the remaining schema/DDL side of M033: it is the slice that can validate **R039** directly and close the remaining **R037/R038** proof gap while preserving the **R040** vendor-seam constraint. The pressure is concentrated and tractable.

Current S04-owned raw boundaries are:
- `mesher/storage/schema.mpl` — 1 `Repo.query_raw(...)` + 1 `Repo.execute_raw(...)` for runtime partition creation.
- `mesher/storage/queries.mpl` — 2 S04-owned raw functions: `get_expired_partitions(...)` and `drop_partition(...)`.
- `mesher/migrations/20260216120000_create_initial_schema.mpl` — **23** `Pool.execute(...)` raw DDL sites.

The migration raw DDL is not one giant unknown. It breaks down cleanly:
- **1 PG-only extension** keep-site: `CREATE EXTENSION IF NOT EXISTS pgcrypto`
- **3 raw table creates**
  - `org_memberships` and `issues` are not PG-only and are likely cheap rewrites to `Migration.create_table(...)`
  - `events ... PARTITION BY RANGE (received_at)` is the real PG-only table helper gap
- **19 raw index creates**
  - **13** simple or partial indexes are already close to `Migration.create_index(...)` territory if name drift is acceptable, or if a small neutral `name:` option is added
  - **5** ordered indexes (`... received_at DESC`, `... last_seen DESC`, `... triggered_at DESC`) need a small honest neutral extension for column sort specs
  - **1** real PG-only index family remains: `USING GIN(tags jsonb_path_ops)`

So the core work is not “invent a DDL AST.” It is:
1. add explicit **PG schema helpers** for the truly PG-only cases (extension, partitioned table creation, runtime partition create/list/drop, GIN/opclass index),
2. optionally make **small neutral migration improvements** where they are honestly portable (explicit index name, ordered index columns),
3. move Mesher’s runtime partition lifecycle onto those helpers and prove it live.

Two constraints matter immediately:
- `mesher/services/retention.mpl` is **not** a good primary proof surface for cleanup because `retention_cleaner` sleeps for 24h before first run and `run_retention_cleanup(...)` is private.
- `mesher/main.mpl` **is** a good proof surface for startup partition creation because it already calls `create_partitions_ahead(pool, 7)` on boot.

## Requirements Focus

### Primary
- **R039** — Mesh migrations should cover the recurring schema and partition-management cases that force `mesher/` into raw DDL today, with explicit extras where needed.

### Strong supporting requirements
- **R037** — final validation still depends on S04’s partition/schema-extra proof.
- **R038** — S03’s verifier still excludes the partition/catalog keep-sites; S04 needs to retire them and add its own mechanical enforcement.
- **R040** — S04 must keep PG-only schema behavior explicit instead of widening the neutral migration API until it lies.
- **R036** — supporting slice only: shared builder changes must stay honestly neutral.

## Skills Discovered

Direct core technologies for this slice already have installed skills:
- **`postgresql-database-engineering`** — directly relevant for declarative partitioning, partition lifecycle, GIN/jsonb indexing, and retention/drop strategy.
- **`rust-best-practices`** — directly relevant for adding runtime builders/intrinsics and test harness code in `compiler/mesh-rt` / `meshc`.

No additional skill install was needed.

Implementation-relevant rules used from these skills:
- From **postgresql-database-engineering**:
  - range partitioning is the right fit for time-series/event tables,
  - dropping an old partition is the correct fast-retention primitive instead of row-by-row deletes,
  - GIN + `jsonb_path_ops` is the honest PG-specific index family for JSONB containment,
  - partition/index behavior should be proven against real catalogs, not string snapshots.
- From **rust-best-practices**:
  - keep SQL builders pure and unit-testable,
  - extend small focused parsers/builders instead of scattering ad hoc string logic,
  - keep fallible runtime helpers on `Result` paths and verify them with targeted tests.

## Recommendation

Follow the existing M033 decisions (`D052`, `D054`) literally: keep the neutral migration surface small and put the real PG-only schema behavior behind explicit PG namespacing.

### Recommended API split

#### Keep neutral / shared
Use the existing `Migration` surface only for features that are honestly portable or at least vendor-agnostic enough to justify a neutral slot:
- `create_table` for ordinary tables, including table constraints passed as raw entries in the column list
- `create_index` for ordinary/partial indexes
- if needed, extend `create_index` only in small honest ways:
  - **explicit index name** (`name:`)
  - **ordered column specs** (`DESC` / `ASC`)

Do **not** turn the current `options: String` into a fake generic schema DSL for `USING`, operator classes, partition clauses, and extension management. The current parser is already substring-based (`unique:true`, `where:`) and gets brittle fast.

#### Make explicit PG-only
Add PG-only schema helpers under the existing **`Pg`** namespace rather than bloating `Migration` with behavior that SQLite cannot honestly claim later. Reusing `Pg` is lower-risk than inventing a fresh stdlib module.

The real PG-only helper families needed by Mesher are:
- `CREATE EXTENSION IF NOT EXISTS ...`
- `CREATE TABLE ... PARTITION BY RANGE (...)`
- runtime child partition create/list/drop for Mesher’s daily `events_YYYYMMDD` lifecycle
- `CREATE INDEX ... USING GIN(... jsonb_path_ops)`

This keeps the SQLite seam credible: later SQLite-specific extras can be added beside `Pg`, not by backing PG syntax out of `Migration`.

### Recommended Mesher reshaping

Consolidate partition lifecycle into **`mesher/storage/schema.mpl`** (or an equivalently named schema-focused module). Right now the slice’s runtime work is split awkwardly:
- startup partition creation lives in `storage/schema.mpl`
- retention partition listing + dropping live in `storage/queries.mpl`

That split made sense when everything was raw SQL, but once S04 adds first-class schema helpers, the natural seam is:
- `storage/schema.mpl` owns create/list/drop partition helpers
- `services/retention.mpl` imports those schema helpers instead of query helpers
- `storage/queries.mpl` stops carrying schema/catalog responsibilities

## Implementation Landscape

### Key files

- `compiler/mesh-rt/src/db/migration.rs`
  - current neutral DDL builders and tests
  - already has the right pattern: pure Rust SQL builders + extern wrappers
  - current `build_create_index_sql(...)` is the narrow shared seam and currently only understands `unique:true` + `where:`
- `compiler/mesh-rt/src/db/pool.rs`
  - already exposes `mesh_pool_query(...)` returning `Result<List<Map<String, String>>, String>`
  - important: S04 does **not** need a new low-level runtime primitive to support catalog/list helpers
- `compiler/mesh-rt/src/lib.rs`
  - re-export surface for any new `mesh_pg_*` or migration helpers
- `compiler/mesh-typeck/src/infer.rs`
  - `Pg` module signatures and `Migration` module signatures live here
- `compiler/mesh-codegen/src/mir/lower.rs`
  - intrinsic name mapping; any new stdlib entrypoint has to be wired here
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
  - LLVM declarations for new externs
- `compiler/meshc/tests/e2e.rs`
  - existing compile-shape coverage for `Migration`; optional place for lightweight compile-only checks if new stdlib surface is added
- `mesher/migrations/20260216120000_create_initial_schema.mpl`
  - the main raw-DDL collapse target
- `mesher/storage/schema.mpl`
  - current runtime create-ahead raw helper
- `mesher/storage/queries.mpl`
  - current S04-owned raw keep-sites: `get_expired_partitions(...)`, `drop_partition(...)`
- `mesher/main.mpl`
  - startup consumer of `create_partitions_ahead(...)`
- `mesher/services/retention.mpl`
  - retention consumer of expired-partition list/drop helpers; note the 24h sleep before first actor run
- `compiler/meshc/tests/e2e_m033_s02.rs`
  - reusable copied-storage probe harness (`compile_and_run_mesher_storage_probe(...)`)
- `compiler/meshc/tests/e2e_m033_s03.rs`
  - reusable Docker/Postgres + live Mesher spawn harness (`with_mesher_postgres(...)`, `spawn_mesher(...)`, `wait_for_mesher(...)`)
- `scripts/verify-m033-s03.sh`
  - current mechanical raw keep-list enforcement that still explicitly excludes S04 partition/catalog sites
- `compiler/meshc/src/migrate.rs`
  - migration scaffold examples still point developers at raw `Migration.execute(...)` for extensions; update if new PG helper becomes public API

### What already works and should be reused

- `Migration.create_table(...)` is more capable than Mesher currently uses. Because `build_create_table_sql(...)` passes non-colon entries through verbatim, table-level constraints like `UNIQUE(user_id, org_id)` are already expressible. This is why `org_memberships` and `issues` look like cheap cleanup.
- `mesh_pool_query(...)` already exists, so a PG partition-list helper can query catalogs directly in Rust and return the same row shape Mesher already consumes.
- S02/S03 already established the pattern of proving Mesher storage helpers via copied temporary Mesh projects instead of routing every proof through the HTTP server.
- S03 already has the heavier live Mesher process harness if S04 wants to prove startup partition creation through `main.mpl` itself.

### What does **not** work cleanly yet

- `Migration.create_index(...)` cannot honestly represent the current remaining Mesher indexes without either:
  - accepting name drift, or
  - growing a small neutral feature bump (`name`, ordered column specs)
- the current `options: String` parser is too weak to absorb PG-only `USING GIN` / operator-class behavior without becoming fragile
- runtime partition creation still trusts raw string concatenation in Mesh code
- partition dropping currently concatenates `DROP TABLE IF EXISTS ` with an unquoted identifier from catalog rows; safe enough by provenance, but exactly the kind of trust boundary S04 should retire

### Natural seams for task breakdown

#### 1. Rust/runtime + compiler plumbing
Own the public helper boundary first.

Likely files:
- `compiler/mesh-rt/src/db/migration.rs` and/or a new PG schema helper file under `compiler/mesh-rt/src/db/`
- `compiler/mesh-rt/src/lib.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- possibly `compiler/mesh-repl/src/jit.rs` if the new externs need REPL symbol registration in the same way `Migration` helpers do

#### 2. Mesher migration rewrite
Once helper shapes exist, rewrite `mesher/migrations/20260216120000_create_initial_schema.mpl`.

Good split inside that file:
- easy cleanup first: move ordinary raw tables/indexes to `Migration.create_table(...)` / `Migration.create_index(...)`
- PG extras second: replace raw extension / partitioned events table / GIN index with explicit PG helpers

#### 3. Runtime partition lifecycle rewrite
Consolidate runtime schema helpers and move consumers.

Likely files:
- `mesher/storage/schema.mpl`
- `mesher/storage/queries.mpl`
- `mesher/main.mpl`
- `mesher/services/retention.mpl`

The likely clean end state is that `storage/schema.mpl` owns:
- create daily partitions ahead
- list expired daily partitions
- drop partition

#### 4. Live proof + verification
Add a slice-specific harness and verifier.

Likely files:
- `compiler/meshc/tests/e2e_m033_s04.rs`
- `scripts/verify-m033-s04.sh`
- maybe a small follow-up edit to `scripts/verify-m033-s03.sh` to remove the S04 exclusion once the new verifier exists

## What To Build Or Prove First

1. **Settle the API boundary first.**
   - Decide exactly which features stay neutral (`Migration`) and which become explicit PG helpers (`Pg`).
   - This is the real R040 guardrail.

2. **Implement pure Rust SQL builders and unit tests before touching Mesher.**
   - This follows the rust-best-practices guidance and matches the existing `migration.rs` pattern.
   - It also localizes failures before the live harness enters the picture.

3. **Rewrite the Mesher migration file next.**
   - It is the densest raw-DDL cluster and the easiest place to prove the new helper boundary is real.

4. **Then rewrite runtime partition lifecycle and consumer imports.**
   - `main.mpl` startup create-ahead and `retention.mpl` cleanup should use the same first-class helper family.

5. **Add live Postgres proof last.**
   - The slice is not done until it proves real catalog behavior, not just generated SQL strings.

## Verification Strategy

### Rust/unit level
Run focused unit coverage for builder/rendering logic in the runtime file that owns the new helpers.

At minimum, add tests for:
- extension SQL rendering
- partitioned parent-table SQL rendering
- daily child partition naming + bound rendering
- partition list catalog SQL rendering if it is generated dynamically
- ordered index rendering (`DESC`)
- GIN + `jsonb_path_ops` rendering if helperized
- index-name derivation if `name:` / decorated column specs are added

### Live Postgres proof
Add `compiler/meshc/tests/e2e_m033_s04.rs` and prove three things explicitly:

1. **Migration-time schema extras apply correctly**
   - run `meshc migrate mesher up`
   - assert `pgcrypto` exists in `pg_extension`
   - assert `events` is a partitioned table in the catalogs
   - assert the `tags` index is a GIN/jsonb-path-ops index on `events`

2. **Runtime startup partition creation works on live catalogs**
   - preferably reuse the S03 Mesher spawn harness and let `main.mpl` call `create_partitions_ahead(pool, 7)`
   - assert that today + future partition tables now exist under `events`
   - if practical, assert that the runtime-created partition picked up the matching partition indexes from the parent definition

3. **Runtime list/drop lifecycle works**
   - do **not** wait on `retention_cleaner`; it sleeps first
   - instead, use a copied storage probe or a small direct helper path to call the real Mesher partition list/drop helpers
   - assert expired partitions are discovered from the live catalogs, dropped successfully, and no longer exist in `pg_inherits` / `to_regclass(...)`

### Mechanical verifier
Add `scripts/verify-m033-s04.sh` mirroring S02/S03 style.

Recommended contract:
- `cargo test -p meshc --test e2e_m033_s04 -- --nocapture`
- `cargo run -q -p meshc -- fmt --check mesher`
- `cargo run -q -p meshc -- build mesher`
- python sweep that enforces:
  - `mesher/storage/schema.mpl` no longer uses `Repo.query_raw` / `Repo.execute_raw` for the owned partition lifecycle
  - `mesher/storage/queries.mpl` no longer contains raw `get_expired_partitions` / `drop_partition` keep-sites
  - `mesher/migrations/20260216120000_create_initial_schema.mpl` no longer uses raw `Pool.execute(...)` for the S04-owned families, with any explicit leftover list named if one remains

Also update `scripts/verify-m033-s03.sh` if you want the older verifier to stop silently tolerating S04 raw sites.

## Constraints, Risks, and Planner Notes

- **Do not test retention cleanup by waiting for the actor.** `retention_cleaner` sleeps for 86,400,000ms before its first run.
- **Keep date math aligned to the database clock.** The current startup helper computes partition suffixes from PostgreSQL time, not host time. If the new helper moves date math into Rust without querying DB time first, you can create host/DB clock drift around day boundaries.
- **Do not invent per-partition index DDL unless the live proof forces it.** PostgreSQL’s partitioning docs say indexes declared on the partitioned parent automatically create matching indexes on existing and future partitions. That should let S04 keep runtime child creation focused on partition DDL only.
- **Do not over-generalize the catalog/list API.** The requirement note already allows truly dynamic catalog work to remain raw if the helper would lie. S04 only needs the common daily-range partition lifecycle Mesher actually uses.
- **Current neutral `Migration.create_index(...)` naming is not Mesher-compatible if exact names matter.** It derives `idx_{table}_{columns...}` from raw column tokens. If you add decorated column specs like `received_at:DESC`, sanitize base names for derived names or add an explicit `name:` option.
- **Cheap cleanup exists.** `org_memberships` and `issues` look like low-risk rewrites to `Migration.create_table(...)`; do not let the planner bury those behind the harder PG-only work.
- **Scaffold/docs ripple is real.** If S04 ships public PG schema helpers, update `compiler/meshc/src/migrate.rs` examples so new generated migrations stop teaching raw extension DDL by default.

## Sources

- PostgreSQL docs on declarative partitioning and parent/child index behavior: https://www.postgresql.org/docs/current/ddl-partitioning.html
- PostgreSQL docs note that dropping a partition with `DROP TABLE` is a normal fast lifecycle operation for old data: https://www.postgresql.org/docs/current/ddl-partitioning.html
