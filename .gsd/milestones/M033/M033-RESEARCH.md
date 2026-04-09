# M033 — Research

**Date:** 2026-03-24

## Summary

M033 should not start by attacking the hardest PostgreSQL-only queries in `mesher/`. The first thing to prove is the contract of the neutral data-layer surface itself. The current runtime already has a composable read builder in `compiler/mesh-rt/src/db/query.rs`, SQL assembly and basic write helpers in `compiler/mesh-rt/src/db/repo.rs`, and a small migration DSL in `compiler/mesh-rt/src/db/migration.rs`. But the write side is still shaped around `Map<String,String>` literal field maps, the migration side is still limited to simple tables/indexes, and the public docs still stop at `Sqlite`, `Pg`, and `Pool`. Mesher is telling the truth about the remaining pressure: the real gaps are bound expressions in `SELECT`/`SET`/`ON CONFLICT`, server-side JSONB read/write paths, harder scalar/derived-table reads, full-text search, pgcrypto helpers, and partition/catalog DDL.

The pressure is concentrated enough to slice cleanly. In `mesher/storage/queries.mpl` alone there are 15 `Repo.query_raw(...)` call sites, 7 `Repo.execute_raw(...)` call sites, 42 `Query.where_raw(...)` call sites, 25 `Query.select_raw(...)` call sites, 5 `Query.order_by_raw(...)` call sites, and 2 `Query.group_by_raw(...)` call sites. Those sites cluster into a few repeat families rather than a hundred unrelated one-offs. That means the roadmap should sequence work around pressure families and boundary contracts, not around endpoints or around broad “rewrite Mesher” passes.

The main planning trap is false proof. Existing lower-level coverage for `Query`, `Repo`, and `Migration` is mostly compile-shape coverage in `compiler/meshc/tests/e2e.rs`; it proves the surface lowers through the compiler, not that the emitted SQL is operationally trustworthy on a live Postgres database. M033 needs live Postgres proof early for the new surface, then Mesher dogfood rewrites, then the migration/partition path, and only after that public docs. Otherwise the milestone risks landing an API that looks nicer in examples while the real Mesher boundaries stay raw.

## Recommendation

Build M033 around a **small structured expression layer** that extends the existing `Query`/`Repo`/`Migration` serializers, rather than around a new universal SQL AST. Reuse the current immutable query pattern, slot-based builders, `renumber_placeholders(...)`, `build_where_from_query_parts(...)`, and the existing SQL builders in `compiler/mesh-rt/src/db/orm.rs`. The missing capability is not “another raw string escape hatch.” The missing capability is a way to carry parameterized expressions and computed write clauses through the existing runtime without lying about vendor-specific behavior.

The neutral core should cover only shapes that are honestly reusable across backends: column references, parameters, literals, null, simple function calls, arithmetic/comparison, `CASE`, `COALESCE`, expression-valued `SET`, expression-valued `SELECT`, and the subquery forms that do not force a PG-only abstraction. PostgreSQL-specific behavior should remain explicit and namespaced: JSONB extraction/building/operators, full-text search, pgcrypto helpers, partition lifecycle helpers, index-method/operator-class DDL, and catalog-backed partition discovery. The milestone should keep `Repo.query_raw`, `Repo.execute_raw`, and `Migration.execute` as the escape hatch for dishonest leftovers instead of chasing a zero-raw metric.

The first slice should prove the write-side expression contract on narrow, high-leverage Mesher sites: `upsert_issue`, the `now()` update sites, and the `NULL` assignment edge. Those are the cheapest way to validate whether the neutral expression surface is real. Only after that should the roadmap move into PG-specific JSONB/search/crypto rewrites, then the harder read-side scalar/derived-table shapes, then partition/migration helpers, and finally docs plus closeout.

## Implementation Landscape

### Key Files

- `compiler/mesh-rt/src/db/query.rs` — immutable query builder with `join_as`, `where_raw`, `select_raw`, `where_sub`, and `fragment`; likely home for any neutral expression/projection expansion.
- `compiler/mesh-rt/src/db/repo.rs` — SQL assembly, placeholder renumbering, `update_where`, `insert_or_update`, raw repo escape hatches, and the current PG-bound `mesh_repo_transaction`; this is the narrowest file that defines the real contract.
- `compiler/mesh-rt/src/db/orm.rs` — existing insert/update/delete/upsert SQL builders used by `Repo`; should be extended, not bypassed, if write expressions land below `Repo`.
- `compiler/mesh-rt/src/db/migration.rs` — current migration DSL; supports simple tables/columns/indexes plus raw execute, but not extensions, partitioned tables, GIN operator classes, or partition lifecycle.
- `compiler/mesh-rt/src/db/pg.rs` — real PostgreSQL backend implementation; the right home for explicit PG-only lowering/runtime helpers, not for pretending the whole surface is neutral.
- `compiler/mesh-rt/src/db/sqlite.rs` — real SQLite backend implementation with `?` parameter binding; keeps the future SQLite seam honest and argues against baking `$N` or PG casts into the neutral layer.
- `mesher/storage/queries.mpl` — the primary M033 pressure map. The boundary comments are already grouped around the exact missing capabilities.
- `mesher/storage/writer.mpl` — event ingest path using `INSERT ... SELECT $4::jsonb AS j`; the best write-side dogfood target for JSONB insert expressions.
- `mesher/storage/schema.mpl` — runtime partition creation helper used at Mesher startup.
- `mesher/migrations/20260216120000_create_initial_schema.mpl` — the clearest migration-gap artifact: 7 `Migration.create_table(...)` calls surrounded by many raw `Pool.execute(...)` DDL statements.
- `mesher/services/event_processor.mpl` — consumer of `extract_event_fields(...)` and `upsert_issue(...)`; this is where behavior stability matters.
- `mesher/api/search.mpl` — consumer of `search_events_fulltext(...)` and filtered issue/event queries.
- `mesher/ingestion/pipeline.mpl` — consumer of `check_volume_spikes(...)`, `evaluate_threshold_rule(...)`, and `fire_alert(...)`.
- `mesher/services/retention.mpl` — consumer of `get_expired_partitions(...)` and `drop_partition(...)`.
- `website/docs/docs/databases/index.md` — currently documents `Sqlite`, `Pg`, `Pool`, and `deriving(Row)`, but not `Query`, `Repo`, or `Migration`.

### Pressure Families

1. **Computed write expressions**
   - `mesher/storage/queries.mpl:319` — `upsert_issue(...)`
   - `mesher/storage/queries.mpl:886` — `acknowledge_alert(...)`
   - `mesher/storage/queries.mpl:897` — `resolve_fired_alert(...)`
   - `mesher/storage/queries.mpl:985` — `update_project_settings(...)`
   - `mesher/storage/queries.mpl:383` — `assign_issue(...)` still drops to raw SQL for `NULL`
   - These are the highest-leverage first targets because they validate `SET`, `ON CONFLICT`, `NULL`, `CASE`, `COALESCE`, and `now()` without forcing the hardest read-side design first.

2. **JSONB-heavy read/write paths**
   - `mesher/storage/writer.mpl:20` — `insert_event(...)`
   - `mesher/storage/queries.mpl:492` — `extract_event_fields(...)`
   - `mesher/storage/queries.mpl:762` — `create_alert_rule(...)`
   - `mesher/storage/queries.mpl:828` — `fire_alert(...)`
   - These should stay explicit PG extras where the behavior is genuinely PostgreSQL-specific.

3. **Bound select expressions / harder read-side expressions**
   - `mesher/storage/queries.mpl:541` — `search_events_fulltext(...)` binds `$2` inside `ts_rank(...)`
   - `mesher/storage/queries.mpl:646` — `event_breakdown_by_tag(...)` binds `$2` inside `tags->>$2`
   - `mesher/storage/queries.mpl:673` — `project_health_summary(...)` uses three scalar subqueries in one `SELECT`
   - `mesher/storage/queries.mpl:699` — `get_event_neighbors(...)` uses dual scalar subqueries plus tuple comparison
   - `mesher/storage/queries.mpl:805` — `evaluate_threshold_rule(...)` uses two derived tables and a `CASE`
   - `mesher/storage/queries.mpl:475` — `check_volume_spikes(...)` uses nested subquery + `JOIN` + `HAVING` + interval math

4. **PG search / crypto extras**
   - `mesher/storage/queries.mpl:182` — `create_user(...)` via pgcrypto
   - `mesher/storage/queries.mpl:200` — `authenticate_user(...)`
   - `mesher/storage/queries.mpl:541` — full-text search
   - `mesher/storage/queries.mpl:562` — JSONB containment already works through `where_raw(...)`; explicit PG helpers should be added only where they retire real recurring raw sites.

5. **Schema / migration / partition lifecycle**
   - `mesher/storage/schema.mpl:7` — `create_partition(...)`
   - `mesher/storage/queries.mpl:943` — `get_expired_partitions(...)`
   - `mesher/storage/queries.mpl:953` — `drop_partition(...)`
   - `mesher/migrations/20260216120000_create_initial_schema.mpl:7` — `CREATE EXTENSION IF NOT EXISTS pgcrypto`
   - `mesher/migrations/20260216120000_create_initial_schema.mpl:37` — partitioned `events` table via raw `PARTITION BY RANGE`
   - `mesher/migrations/20260216120000_create_initial_schema.mpl:81` — `USING GIN(tags jsonb_path_ops)` raw index DDL

### Existing Patterns To Reuse

- **Immutable query builders** in `compiler/mesh-rt/src/db/query.rs` — new surface should fit the existing pipe/composable builder style rather than introducing a second query model.
- **Placeholder renumbering** in `compiler/mesh-rt/src/db/repo.rs` — `renumber_placeholders(...)` and `build_where_from_query_parts(...)` already solve mixed raw/structured parameter numbering.
- **Current upsert/update/delete builders** in `compiler/mesh-rt/src/db/orm.rs` — extend them for expression-valued writes instead of replacing them wholesale.
- **Compile-pipeline e2e style** in `compiler/meshc/tests/e2e.rs` — keep using this for language/runtime surface availability, but pair it with live Postgres proof for M033-specific behavior.
- **Mesher’s own boundary comments** in `mesher/storage/queries.mpl` — they are already a truthful grouping of remaining pressure after M032.

### Boundary Contracts That Matter

- **Neutral vs vendor-specific must be visible in the API.** `compiler/mesh-rt/src/db/sqlite.rs` and `compiler/mesh-rt/src/db/pg.rs` are genuinely different backends; the neutral layer should sit above them, not pretend they already match.
- **`Repo.transaction` is not neutral today.** `compiler/mesh-rt/src/db/repo.rs:1263` calls `mesh_pg_begin`, `mesh_pg_commit`, and `mesh_pg_rollback` directly. Do not let M033 implicitly promise SQLite parity here unless the transaction boundary is intentionally reworked.
- **The current `Query` source model is single-table.** `Query.from(...)` stores one `source` string. Harder read-side families that need derived tables or more general subquery sources may require a real source abstraction, not just more raw fragments.
- **Write contracts are still literal-map based.** `Repo.update_where(...)` and `Repo.insert_or_update(...)` currently accept only `Map<String,String>` plus field-name lists. That is why `NULL`, `now()`, `CASE`, `COALESCE`, JSONB extraction, and computed upsert logic fall off the surface.
- **Migration index helpers are too narrow for Mesher’s real schema.** `Migration.create_index(...)` supports column lists plus `unique:true` and `where:...`, but not index methods, operator classes, or explicit names.
- **The public docs still reflect only the low-level database surface.** `website/docs/docs/databases/index.md` has no `Query`, `Repo`, or `Migration` coverage.

### Build Order

1. **Prove the neutral expression/write contract first.**
   - Extend the runtime around expression-valued `SET` and `ON CONFLICT` updates.
   - Dogfood immediately on:
     - `mesher/storage/queries.mpl:319` — `upsert_issue(...)`
     - `mesher/storage/queries.mpl:163` — `revoke_api_key(...)`
     - `mesher/storage/queries.mpl:383` — `assign_issue(...)` unassign branch
     - `mesher/storage/queries.mpl:886` and `:897` — alert state transitions with `now()`
   - Why first: this retires several raw sites with the smallest honest surface and proves whether the neutral core is real before the roadmap touches PG-only query families.

2. **Add explicit PG extras for JSONB/search/crypto on top of that core.**
   - Dogfood on:
     - `mesher/storage/writer.mpl:20` — `insert_event(...)`
     - `mesher/storage/queries.mpl:492` — `extract_event_fields(...)`
     - `mesher/storage/queries.mpl:762` — `create_alert_rule(...)`
     - `mesher/storage/queries.mpl:828` — `fire_alert(...)`
     - `mesher/storage/queries.mpl:182` / `:200` — pgcrypto auth helpers
     - `mesher/storage/queries.mpl:541` — full-text search
   - Why second: these are the real PG-first pressure sites, but they should land after the neutral expression mechanics exist.

3. **Take the harder read-side subquery/derived-table families only after the serializer contract is stable.**
   - Focus on:
     - `list_issues_filtered(...)`
     - `project_health_summary(...)`
     - `get_event_neighbors(...)`
     - `evaluate_threshold_rule(...)`
     - `check_volume_spikes(...)`
   - Why third: this is where the roadmap is most likely to overbuild. If the slice hits dishonest-helper territory, keep a short explicit raw keep-list instead of widening the neutral core for its own sake.

4. **Do the migration/schema/partition slice separately and require live Postgres proof.**
   - Cover:
     - migration-time `PARTITION BY RANGE`
     - `CREATE EXTENSION IF NOT EXISTS pgcrypto`
     - GIN/jsonb_path_ops index helper story
     - runtime `create/list/drop` partition lifecycle
   - Why separate: the operational truth surface is different from query expressiveness, and the live catalog behavior is part of acceptance.

5. **Finish with docs and integrated acceptance, not before.**
   - Update `website/docs/docs/databases/index.md` once the public surface is settled.
   - Anchor docs in one real Mesher-backed example path rather than adding a broad tutorial sweep.
   - Reconcile the final justified raw keep-list at the same time.

### Verification Approach

Use the existing tests for compile-shape regression, but do not treat them as M033 acceptance on their own.

**Existing compile/runtime surface to keep green**
- `cargo test -q -p mesh-rt`
- `cargo test -q -p meshc --test e2e e2e_query_builder_fragment_crypt -- --nocapture`
- `cargo test -q -p meshc --test e2e e2e_query_builder_where_sub -- --nocapture`
- `cargo test -q -p meshc --test e2e e2e_repo_insert_or_update -- --nocapture`
- `cargo test -q -p meshc --test e2e e2e_migration_index_ops_compile -- --nocapture`
- `cargo test -q -p meshc --test e2e e2e_migration_execute_compiles -- --nocapture`
- `cargo test -q -p meshc --test e2e_stdlib e2e_sqlite_join_runtime -- --nocapture`
- `cargo test -q -p meshc --test e2e_stdlib e2e_sqlite_aggregate_runtime -- --nocapture`

**M033-specific live Postgres proof the roadmap should require**
- A targeted runtime/e2e test for the new expression-valued write surface that actually executes against Postgres.
- A targeted live Postgres proof for at least one JSONB-heavy insert/read path and one full-text/search path.
- A targeted live Postgres proof for partition create/list/drop against real catalogs.
- `cargo run -q -p meshc -- build mesher` after each dogfood wave.
- For real Postgres-backed commands in this repo, load the repo-root `.env` into the subprocess first; non-interactive shells do not inherit it automatically in this environment.

**Behavioral consumer checks to keep honest**
- event ingest path: `mesher/services/event_processor.mpl` + `mesher/storage/writer.mpl`
- search path: `mesher/api/search.mpl`
- alerts path: `mesher/api/alerts.mpl` + `mesher/ingestion/pipeline.mpl`
- retention path: `mesher/services/retention.mpl`
- settings path: `mesher/api/settings.mpl`

## Don't Hand-Roll

| Problem | Existing Solution | Why Use It |
|---------|------------------|------------|
| Mixed placeholder numbering across composed clauses | `renumber_placeholders(...)` and `build_where_from_query_parts(...)` in `compiler/mesh-rt/src/db/repo.rs` | They already handle `?` and `$N` renumbering and are the natural place to extend numbering logic. |
| Basic insert/update/delete/upsert SQL assembly | `compiler/mesh-rt/src/db/orm.rs` plus `Repo.update_where(...)` / `Repo.insert_or_update(...)` | M033 should extend the current write builder contract, not replace it with a disconnected SQL AST. |
| Honest escape hatch for leftovers | `Repo.query_raw(...)`, `Repo.execute_raw(...)`, `Migration.execute(...)` | The milestone bar is a short justified keep-list, not fake purity. |
| Existing compile-surface coverage | `compiler/meshc/tests/e2e.rs` and `compiler/mesh-rt` unit tests | Reuse them for regression coverage, but add live Postgres proof where M033 changes semantics. |

## Constraints

- `compiler/mesh-rt/src/db/repo.rs:1263` makes `Repo.transaction(...)` PostgreSQL-specific today.
- `compiler/mesh-rt/src/db/query.rs` stores a single string `source`; harder derived-table query shapes may need a bigger source contract than `Query.from("table")`.
- `Repo.update_where(...)` and `Repo.insert_or_update(...)` still consume literal `Map<String,String>` values, which is the direct cause of the `NULL`, `now()`, `CASE`, `COALESCE`, JSONB extraction, and computed upsert gaps.
- `Migration.create_index(...)` can express unique + partial indexes, but not GIN operator classes (`jsonb_path_ops`), index methods, or partition-specific DDL.
- Public docs currently describe the low-level DB APIs only; `Query`, `Repo`, and `Migration` are still undocumented for users.
- M033 must keep the later SQLite path credible even though SQLite extras are deferred. The current separate `pg.rs` / `sqlite.rs` split is the thing to preserve, not to hide.

## Common Pitfalls

- **Mistaking compile-only proof for acceptance** — `compiler/meshc/tests/e2e.rs` proves many current surfaces only through compilation/JIT shape, not through live SQL execution.
- **Treating `Query.where_sub(...)` as if it solves the hard read-side family** — it only covers `field IN (SELECT ...)`; it does not solve scalar subqueries in `SELECT` or derived-table sources.
- **Treating `Migration.create_index(...)` as if schema extras are already mostly done** — Mesher’s real indexes still need raw DDL for GIN/jsonb_path_ops and other PG-specific cases.
- **Letting PG-only syntax leak into the neutral contract** — `::uuid`, JSONB operators, `ts_rank`, `crypt`, and catalog tables belong in explicit PG extras.
- **Starting with the hardest derived-table reads** — `check_volume_spikes(...)` and `evaluate_threshold_rule(...)` are real pressure, but they are poor first-slice design anchors.
- **Pushing JSON extraction client-side just to reduce raw SQL count** — that would move logic away from the database truth surface and would not satisfy the dogfood-first intent of the milestone.

## Open Risks

- The current slot-based `Query` shape may be too narrow for the hardest read-side families; if so, the roadmap should widen source/projection contracts deliberately instead of hiding more SQL inside `fragment(...)`.
- A neutral expression layer can drift into a fake universal SQL AST if it tries to solve every subquery and every DDL edge in S01.
- Partition helpers need an honest identifier-safety story. `drop_partition(...)` currently trusts names returned from the catalog query; any first-class helper must preserve that trust boundary.
- Docs can easily get ahead of reality because the current public page does not mention the high-level data-layer surfaces at all.

## Requirement Assessment

- **Table stakes already covered by active requirements:**
  - `R036` / `R040`: the neutral-vs-vendor boundary must be explicit before Mesher rewrites start.
  - `R037`: JSONB, search, crypto, and partition work should be treated as explicit PG work, not generic ORM polish.
  - `R038`: behaviorally stable Mesher rewrites plus a short justified keep-list remain the correct cleanup bar.
  - `R039`: partition lifecycle must be proven live against Postgres catalogs, not through compile-only or string-level tests.
- **Likely omission:** no new mandatory requirement is obviously missing. The one thing worth watching is the proof mode: the current repo has many compile-only tests for these surfaces, so planners should keep live Postgres verification explicit in slice acceptance.
- **Overbuilt risk:** treating `R036` as justification for a large relational AST or for solving every scalar/derived-table pattern before landing the easy recurring write-side wins.
- **Domain-standard but optional:** explicit `set_null`, `set_expr`, `excluded(...)`, named-index/method/operator-class helpers, and returning-expression helpers are all reasonable, but they should ship only when they retire a real Mesher site.

## Candidate Requirements

- **Advisory only:** make the live-proof bar explicit in planning for any new `Query`/`Repo`/`Migration` surface added in M033. Current `meshc` e2e coverage is valuable, but M033 should not accept new ORM/migration capability on compile-only evidence.
- **Advisory only:** ensure the public database docs document `Query`, `Repo`, and `Migration` once the surface stabilizes, because the current docs page does not teach the layer M033 is extending.

## Skills Discovered

| Technology | Skill | Status |
|------------|-------|--------|
| Rust | `rust-best-practices` | installed |
| PostgreSQL | `manutej/luxor-claude-marketplace@postgresql-database-engineering` | installed |
| SQLite | `martinholovsky/claude-skills-generator@sqlite-database-expert` | available |
