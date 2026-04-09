# M033 / S02 — Research

**Date:** 2026-03-25
**Slice:** S02 — Explicit PG extras for JSONB, search, and crypto

## Summary

S02 primarily owns **R037** and directly supports **R038** and **R040**. It should **consume** the S01 neutral `Expr` core, not reopen it. The key job here is to make PostgreSQL-only behavior explicit and composable on the real runtime path: JSONB extraction/building, full-text search helpers, and pgcrypto-backed auth.

The strongest implementation seam is already in place:

- `compiler/mesh-rt/src/db/expr.rs` now serializes portable `SqlExpr` trees.
- `compiler/mesh-rt/src/db/repo.rs` already knows how to embed serialized expressions into `SET` and `ON CONFLICT DO UPDATE` clauses.
- `compiler/mesh-rt/src/db/query.rs` already reserves **slot 13** for `select_params`, but nothing in `repo.rs` reads that slot yet.

That means S02 does **not** need a new SQL AST. It needs a small extension of the existing one plus the missing query/insert plumbing:

- **add a cast-capable expression node** so PG helpers can emit `$N::jsonb`, `(... )::int`, etc.
- **add query-side expression entrypoints** (`select_expr`, `where_expr`; possibly `order_by_expr` if alias-ordering proves awkward)
- **add `Repo.insert_expr`** so PG-only insert families stop depending on raw `INSERT ... SELECT` / `SELECT crypt(...)` fragments
- **add explicit `Pg.*` expression constructors** rather than hiding PG operators inside `Query.where_raw(...)` or whole-string SQL

With that plumbing, the slice-owned Mesher rewrites fall into place without changing product behavior:

- `create_user`
- `authenticate_user`
- `search_events_fulltext`
- `filter_events_by_tag`
- `event_breakdown_by_tag`
- `create_alert_rule`
- `fire_alert`
- `insert_event`
- likely also the cheap JSONB operator cleanups in `get_event_alert_rules` / `get_threshold_rules`

The likely honest leftover is still **`extract_event_fields`**. It uses `CASE`, `jsonb_array_elements(... ) WITH ORDINALITY`, `string_agg`, and scalar subselects in one query. That overlaps the S03 “hard read-side” bucket more than the S02 “explicit PG extras” bucket. The planner should treat it as a candidate keep-site unless the helper work stays obviously honest.

A separate hard constraint remains from S01: the live Mesher Rust harness in `compiler/meshc/tests/e2e_m033_s01.rs` still blocks on HTTP readiness. S02 can and should prove the runtime/ORM work with direct Postgres-backed tests first, but final live Mesher replay will stay noisy until that blocker is fixed or worked around.

## Skills Discovered

- Existing installed skill: **`postgresql-database-engineering`**
- No new skills were installed. PostgreSQL is the only direct external technology this slice depends on, and it is already covered.

Relevant guidance from that skill that applies here:

- keep **JSONB containment** on operators/index pairs that match the actual index class; the current `idx_events_tags` uses `GIN(tags jsonb_path_ops)`, so `@>` is aligned but `?` is not
- keep **FTS helpers** shaped around `to_tsvector(...)`, `plainto_tsquery(...)`, and `ts_rank(...)`; do not turn S02 into schema/index redesign just to avoid raw SQL
- prefer small composable database primitives over big one-off abstractions

## Recommendation

### 1. Keep the public boundary explicit: PG helpers should live under `Pg`, not `Expr`

The Mesh-level API already has a `Pg` module in `compiler/mesh-typeck/src/infer.rs`. That is the cleanest explicit namespace for PG-only helpers. Do **not** smuggle JSONB, FTS, or pgcrypto behind neutral `Expr.*` names.

Recommended shape:

- `Pg.jsonb(expr_or_string)` / `Pg.cast_jsonb(...)`
- `Pg.int(expr)` for `(... )::int`
- `Pg.json_get(json_expr, key_expr)` for `->`
- `Pg.json_get_text(json_expr, key_expr)` for `->>`
- `Pg.json_contains(lhs, rhs)` for `@>`
- `Pg.json_has_key(lhs, rhs)` for `?`
- `Pg.json_build_object(args)` for `jsonb_build_object(...)`
- `Pg.to_tsvector(config, expr)`
- `Pg.plainto_tsquery(config, expr)`
- `Pg.ts_rank(vector, query)`
- `Pg.ts_match(vector, query)` for `@@`
- `Pg.crypt(password_expr, salt_expr)`
- `Pg.gen_salt_bf(rounds_expr)`

These should all return `Ptr` expressions so they compose with the existing `Expr` tree.

### 2. Extend the existing expression runtime minimally

`compiler/mesh-rt/src/db/expr.rs` already has enough structure for most of S02:

- function calls already cover `jsonb_build_object`, `to_tsvector`, `plainto_tsquery`, `ts_rank`, `crypt`, `gen_salt`
- binary operators are already enough for `@>`, `?`, `@@`, `->`, `->>` if the runtime exposes PG-specific constructors for them

The one obvious missing piece is **casts**. Without a cast node, S02 cannot honestly build `$1::jsonb`, `(j->>'cooldown_minutes')::int`, or similar PG shapes.

So the smallest runtime extension is:

- add `SqlExpr::Cast { expr, ty }`
- add serializer support
- expose only the PG-namespaced constructors publicly

That preserves R040: the neutral core stays small, but the internal AST becomes capable enough for explicit vendor helpers.

### 3. Reuse the existing query object instead of inventing a new read DSL

`compiler/mesh-rt/src/db/query.rs` already reserves **`SLOT_SELECT_PARAMS`** but the current repo SQL builder ignores it. That is the best existing seam for S02.

Recommended additions:

- `Query.select_expr(query, exprs)` or `Query.select_exprs(query, exprs)`
  - serialize each expression immediately
  - append its SQL to the select list as `RAW:...`
  - append its params to `SLOT_SELECT_PARAMS`
- `Query.where_expr(query, expr)`
  - serialize expression immediately
  - append it as a raw where clause plus serialized params
- optional: `Query.order_by_expr(query, expr, direction)`
  - only needed if `ORDER BY rank DESC` via alias proves brittle

Then update `compiler/mesh-rt/src/db/repo.rs` to:

- add `SLOT_SELECT_PARAMS = 13`
- read select params in `query_to_select_sql(...)`
- prepend/select-param-order them before WHERE/HAVING/fragment params

That gives S02 expression-valued `SELECT` and boolean `WHERE` without a new query model.

### 4. Add `Repo.insert_expr` before rewriting Mesher JSONB/crypto inserts

`compiler/mesh-rt/src/db/repo.rs` already has the reusable pieces:

- `map_to_columns_and_exprs(...)`
- `build_set_expr_parts(...)`
- `build_insert_or_update_expr_sql_pure(...)`

The natural next function is `build_insert_expr_sql_pure(...)` plus `mesh_repo_insert_expr(...)`.

That one addition unlocks four important slice-owned paths cleanly:

- `create_user` — insert `password_hash = Pg.crypt(...)`
- `create_alert_rule` — JSONB extraction/defaulting without `INSERT ... SELECT`
- `fire_alert` — `condition_snapshot = Pg.json_build_object(...)`
- `insert_event` — JSONB extraction/defaulting into event columns

This is materially better than trying to add a specialized `INSERT ... SELECT` API. The `VALUES (...)` form is enough once expression-valued inserts exist.

### 5. Keep S02 scoped: do not mix schema/index redesign into it

Two current storage constraints matter:

- `mesher/migrations/20260216120000_create_initial_schema.mpl` defines `idx_events_tags` as `GIN(tags jsonb_path_ops)`
  - good for containment `@>`
  - **not** the right proof point for broad `?`-heavy JSONB querying
- `search_events_fulltext` intentionally uses inline `to_tsvector('english', message)` because event partitioning makes a stored/generated `tsvector` column a separate schema problem

So S02 should wrap the existing search/JSONB behavior in explicit helpers, not broaden into index/operator-class migration work. That belongs with S04 schema extras if it becomes necessary.

## Implementation Landscape

### Runtime / compiler surface

- `compiler/mesh-rt/src/db/expr.rs`
  - current neutral AST and serializer
  - supports `Column`, `Value`, `Null`, `Call`, `Binary`, `Case`, `Coalesce`, `Excluded`, `Alias`
  - missing cast support
- `compiler/mesh-rt/src/db/query.rs`
  - immutable query object
  - already defines **slot 13 = `select_params`**
  - only public query extension points today are string-based (`select_raw`, `where_raw`, `order_by_raw`)
- `compiler/mesh-rt/src/db/repo.rs`
  - current SELECT builder reads only slots 0..12; it ignores query slot 13 entirely
  - already has expression-aware write builders for update/upsert
  - obvious home for `insert_expr` and select-param consumption
- `compiler/mesh-rt/src/db/mod.rs`
  - currently exports `expr`, `json`, `migration`, `orm`, `pg`, `pool`, `query`, `repo`, `row`, `sqlite`
  - if a new PG-helper runtime file is added, it plugs in here cleanly
- `compiler/mesh-rt/src/lib.rs`
  - re-export choke point for new runtime externs
- `compiler/mesh-typeck/src/infer.rs`
  - already exposes `Pg`, `Expr`, `Query`, `Repo`
  - S02 needs new `Pg.*` signatures and likely `Query.select_expr` / `Query.where_expr` / `Repo.insert_expr`
- `compiler/mesh-codegen/src/mir/lower.rs`
  - builtin-to-runtime symbol mapping
  - S01 already established the pattern for `Expr.*`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
  - LLVM extern declarations for any new runtime helpers

### Mesher storage rewrite targets

- `mesher/storage/writer.mpl`
  - `insert_event(...)` is still a raw JSONB-heavy `INSERT ... SELECT` keep-site
- `mesher/storage/queries.mpl`
  - `create_user(...)` uses `Repo.query_raw("SELECT crypt(...)")`
  - `authenticate_user(...)` hides pgcrypto in `Query.where_raw(...)`
  - `search_events_fulltext(...)` is a full raw FTS query
  - `filter_events_by_tag(...)` hides `tags @> ?::jsonb` behind neutral raw where
  - `event_breakdown_by_tag(...)` is a raw JSONB select/filter/group query
  - `create_alert_rule(...)` is raw JSONB extraction/defaulting on insert
  - `fire_alert(...)` is raw `jsonb_build_object(...)` plus follow-up raw update
  - `get_event_alert_rules(...)` and `get_threshold_rules(...)` hide `condition_json->>'condition_type'` behind raw where clauses
  - `extract_event_fields(...)` is the hard JSONB read-side outlier

### Callers that should stay stable

- `mesher/services/event_processor.mpl`
  - depends on `extract_event_fields(...)` + writer ingest path
- `mesher/api/search.mpl`
  - depends on `search_events_fulltext(...)` and tag filtering
- `mesher/api/alerts.mpl`
  - depends on alert-rule creation/listing and alert transitions
- `mesher/ingestion/pipeline.mpl`
  - depends on `fire_alert(...)`, `get_event_alert_rules(...)`, `get_threshold_rules(...)`
- `mesher/services/user.mpl`
  - depends on `create_user(...)` / `authenticate_user(...)`

The good news: almost all S02 work stays storage-local. The service and API layers should only need incidental import/signature updates if the Mesh helper surfaces change names.

### Schema / query-shape constraints already in the repo

- `mesher/migrations/20260216120000_create_initial_schema.mpl`
  - creates `pgcrypto`
  - creates `idx_events_tags` as `GIN(tags jsonb_path_ops)`
- `registry/src/db/packages.rs`
  - shows the same `plainto_tsquery(...)` + `ts_rank(...)` FTS ranking pattern S02 wants to expose explicitly
- `registry/migrations/20260228000002_fts_index.sql`
  - demonstrates the later schema direction for stored `tsvector` + GIN, but that is not S02’s immediate runtime target

## Natural Seams

### Seam 1 — Runtime/compiler API work

Files likely touched together:

- `compiler/mesh-rt/src/db/expr.rs`
- `compiler/mesh-rt/src/db/query.rs`
- `compiler/mesh-rt/src/db/repo.rs`
- `compiler/mesh-rt/src/db/mod.rs`
- `compiler/mesh-rt/src/lib.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`

Deliverable:

- PG expression constructors
- query-side expression support
- insert-side expression support

### Seam 2 — Mesher storage rewrites

Files likely touched together:

- `mesher/storage/queries.mpl`
- `mesher/storage/writer.mpl`

Suggested rewrite order inside this seam:

1. `create_user` / `authenticate_user`
2. `search_events_fulltext` (+ cheap JSONB/read-side helpers like `filter_events_by_tag`)
3. `create_alert_rule` / `fire_alert`
4. `insert_event`
5. decide whether `extract_event_fields` is still honest to keep raw

### Seam 3 — Verification bundle

Files likely touched together:

- `compiler/meshc/tests/e2e.rs`
- `compiler/meshc/tests/e2e_m033_s02.rs` (new, likely)
- `scripts/verify-m033-s02.sh` (new, likely)

The S01 Rust harness already has good Docker/Postgres helpers. Reusing that pattern is better than inventing a second live-test framework.

## What to Build or Prove First

### First proof: pgcrypto auth path

This is the smallest high-value vertical slice.

Why first:

- forces `Repo.insert_expr`
- forces `Query.where_expr`
- forces PG-only helper exposure under `Pg`
- does **not** require the hardest JSONB or subquery work

Concrete targets:

- `create_user(...)`
- `authenticate_user(...)`

### Second proof: full-text search path

Why second:

- exercises `select_expr` + aliasing + expression-valued WHERE
- validates parameter ordering through SELECT + WHERE together, which is the most likely query-plumbing bug

Concrete target:

- `search_events_fulltext(...)`

### Third proof: JSONB insert families

Why third:

- after `insert_expr` exists, these are mostly Mesher rewrites plus helper composition
- they cover the slice’s named raw keep-sites from S01 forward intelligence

Concrete targets:

- `create_alert_rule(...)`
- `fire_alert(...)`
- `insert_event(...)`

### Last decision: `extract_event_fields`

Do not make this the first task. Re-evaluate it only after the smaller PG helper surface is real. If it still wants table-valued JSON operators, ordinality, and scalar subqueries all at once, keep it explicit and name it as the honest leftover for S03.

## Verification

### Contract verification

Add focused compiler/runtime tests that prove the new surfaces compile and execute:

- compile-shape tests in `compiler/meshc/tests/e2e.rs` for:
  - `Pg.*` helper calls
  - `Query.select_expr` / `Query.where_expr`
  - `Repo.insert_expr`
- direct Postgres-backed execution tests in a new `compiler/meshc/tests/e2e_m033_s02.rs` for:
  - pgcrypto hash + verify roundtrip
  - full-text search/rank result ordering
  - JSONB insert/extract/defaulting roundtrips for alert/event payloads

### Integration verification

After the direct runtime tests are green, add live Mesher-backed coverage for:

- event ingest using the rewritten `insert_event(...)`
- full-text search route using rewritten search helpers
- alert-rule create + fire paths using explicit PG helpers

Practical warning: this is currently gated by the S01 startup blocker in `compiler/meshc/tests/e2e_m033_s01.rs`. The planner should either:

- fix that blocker first, or
- separate “runtime contract passes” from “live Mesher replay passes” so failures stay attributable

### Raw-boundary verification

Add a slice script similar to `scripts/verify-m033-s01.sh` that checks the owned functions no longer use:

- `Repo.query_raw(...)`
- `Repo.execute_raw(...)`
- `Query.where_raw(...)` / `Query.select_raw(...)` for PG-only operators

except for the explicitly named leftovers.

The initial likely keep-list candidate is:

- `extract_event_fields(...)`

### Repo-wide safety checks

- `cargo run -q -p meshc -- build mesher`
- `cargo run -q -p meshc -- fmt --check mesher`
- targeted `cargo test -p meshc --test e2e_m033_s02 -- --nocapture`

## Risks / Unknowns

- **Parameter ordering bug risk:** once SELECT expressions can bind params, `repo.rs` must preserve SELECT params before WHERE/HAVING/fragment params. The existing unused `select_params` slot strongly suggests this was anticipated, but nothing consumes it today.
- **AST creep risk:** if S02 adds a generic raw-expression escape hatch instead of a cast node + PG helpers, it recreates the fake-portability problem in another form.
- **Index mismatch risk:** `filter_events_by_tag` is aligned with the existing `jsonb_path_ops` index, but `event_breakdown_by_tag` uses `tags ? key`, which is not the same operator family. Do not over-promise performance there without schema work.
- **Harness risk:** even correct runtime work can still look red if the S01 Mesher readiness blocker is not fixed.

## Planner Notes

- Treat this as **runtime/compiler plumbing first, Mesher rewrite second**.
- The biggest unlock is not one helper function; it is the combination of:
  - cast-capable expressions
  - query-side expression params
  - insert-side expression params
- Do **not** spend the first task budget on `extract_event_fields(...)`.
- Keep the public rule crisp: **neutral core for reusable expression mechanics, `Pg.*` for JSONB/FTS/pgcrypto behavior**.
