# M033: ORM Expressiveness & Schema Extras

**Gathered:** 2026-03-24
**Status:** Ready for planning

## Project Description

M033 is the follow-on to M032. Its job is to strengthen Mesh's data layer against the real recurring pressure still visible in `mesher/`, not to reopen solved Mesh-language folklore or redesign the app.

The intended shape is explicit:
- a **broader neutral core** for honestly reusable query/update/insert/select expression work
- that broader core should take the form of a **real expression DSL**, not just a few narrow helper methods
- **explicit Postgres extras** for behavior that is genuinely PG-specific
- a clean path for **SQLite extras later**, but **design only** in M033 rather than live SQLite implementation or proof

The user still does **not** want fake portability, does **not** want a PG-only trap, and does **not** want a purity chase. But the completion bar is now sharper than the earlier "major recurring families" framing: M033 should push toward **near-total coverage** of the recurring Mesher raw SQL/raw DDL boundaries and leave only **dishonest leftovers** — cases where a first-class helper would lie, especially truly dynamic catalog work.

This milestone is primarily for **backend authors** building real apps in Mesh, with `mesher/` acting as the proof case.

## Why This Milestone

M032 already did the cleanup pass that this work depends on: it retired stale workaround folklore, separated real Mesh/tooling blockers from real data-layer pressure, and left an honest boundary map anchored in live Mesher files.

That means the remaining friction is no longer “Mesh cannot do this at the language level.” The remaining friction is that `Repo`, `Query`, and `Migration` still cannot honestly express several recurring shapes that `mesher/` already uses today: expression-heavy updates, JSONB-heavy read/write paths, parameterized select expressions, harder read-side subqueries, full-text search, crypto helpers, and partition lifecycle DDL/catalog work.

This milestone needs to happen now because the repo already has a truthful pressure map. Leaving those boundaries as permanent app-local raw SQL / DDL after M032 would amount to knowingly preserving the next wave of dogfood pain instead of turning it into platform capability.

## User-Visible Outcome

### When this milestone is complete, the user can:

- express almost all recurring Mesher query/update/insert/select and schema patterns on stronger Mesh data-layer surfaces without dropping to raw SQL or raw DDL, except for the short list of **dishonest leftovers**
- read the outcome in **public Mesh docs** that explain the new neutral DSL and explicit PG extras through a real Mesher-backed example path instead of a broad docs sweep

### Entry point / environment

- Entry point: `mesher/` data layer, targeted `meshc` compiler/runtime tests, Postgres-backed Mesher flows, and public Mesh docs under `website/docs/docs/databases/index.md`
- Environment: local dev with a live Postgres database plus repo-local docs/example updates
- Live dependencies involved: PostgreSQL

## Completion Class

- Contract complete means: targeted compiler/runtime tests, milestone artifacts, and public Mesh docs prove a **broader neutral core** through a real expression DSL, explicit PG extras for the real PG-only families, a credible SQLite seam, and the near-total-coverage bar with only **dishonest leftovers** retained
- Integration complete means: live Postgres-backed Mesher query, write, search, alert, and migration/schema paths still work after the rewrites, with the recurring raw-SQL/raw-DDL families retired everywhere the new surfaces honestly cover them
- Operational complete means: real Postgres migration and partition lifecycle operations (create/list/drop) work against a live database and real catalogs; the affected Mesher flows still hold under real runtime conditions; live SQLite implementation is not part of M033 closeout

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- a live Postgres-backed Mesher path can ingest and store events, upsert/query the affected issue and event data, and preserve the same user-visible behavior while the underlying storage code uses the broader neutral expression DSL and explicit PG extras instead of today's recurring raw SQL
- a live Postgres-backed Mesher path can exercise the JSONB-heavy, search-heavy, alert-rule, and schema/update flows that currently sit on `ORM boundary` comments, while the public Mesh docs explain the new neutral DSL and PG extras through a real Mesher-backed example path
- partition lifecycle work cannot be treated as done through mocks or SQL-string tests alone; the milestone must run create/list/drop partition behavior against a real Postgres database because catalog behavior and partition DDL are part of the truth surface

## Risks and Unknowns

- The broader first pass could drift into a fake universal SQL AST detached from real `mesher/` pressure — that would violate the dogfood-first bar and make the API worse rather than better
- A wider neutral core can blur the line between honestly reusable behavior and PG-only behavior — if that line is wrong, the milestone recreates the fake-portability problem it was supposed to fix
- The harder push toward near-total coverage can tempt the milestone into lying helpers just to retire more raw sites — that would miss the point; the remaining tail must be only **dishonest leftovers**
- Partition helpers can hardcode the wrong abstraction boundary — if they leak into the neutral core, later SQLite extras may require backing out the design
- Aggressive rewrites in `mesher/` could accidentally change behavior, data shape, or operational signals — M033 is supposed to improve the platform underneath the app, not smuggle in product redesign
- A broad docs wave would dilute the real platform work — the docs outcome should stay in the public Mesh docs and stay tightly tied to the shipped surfaces

## Existing Codebase / Prior Art

- `mesher/storage/queries.mpl` — the concentrated `ORM boundary` map for computed `ON CONFLICT` updates, function-valued updates, parameterized select expressions, harder read-side subqueries, full-text search, JSONB-heavy read/write paths, and partition cleanup helpers
- `mesher/storage/writer.mpl` — the insert-side JSONB extraction boundary and the storage-local dogfood surface that should benefit from stronger insert expressions
- `mesher/storage/schema.mpl` — the current runtime partition creation helper surface and the most obvious prior art for common create/list/drop partition work
- `mesher/migrations/20260216120000_create_initial_schema.mpl` — the current migration-time `PARTITION BY` keep-site plus existing PG-specific extension/index prior art
- `compiler/mesh-rt/src/db/query.rs` — the current query surface already supports joins, raw where/select, and a limited subquery path; this is the starting point for the broader neutral expression DSL
- `compiler/mesh-rt/src/db/repo.rs` — the current insert/update/delete/upsert surfaces show the literal-field-map bias and the raw escape hatches M033 is meant to reduce
- `compiler/mesh-rt/src/db/migration.rs` — the current neutral migration baseline plus raw execute escape hatch; the obvious home for honest PG partition helpers
- `website/docs/docs/databases/index.md` — the public Mesh docs surface that needs to explain the new neutral DSL and explicit PG extras truthfully
- `.gsd/milestones/M032/M032-SUMMARY.md` — the authoritative handoff separating supported-now Mesh behavior from the real M033 data-layer boundary families

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R036 — advances the neutral-core-plus-explicit-extras contract by locking in a **broader neutral core** through a real expression DSL while keeping PG-only behavior explicit
- R037 — advances the PG-specific query/migration requirement by explicitly targeting JSONB-heavy paths, expression-heavy updates, full-text search, crypto helpers, and partition lifecycle work
- R038 — advances the cleanup bar by pushing toward **near-total coverage** of the recurring Mesher raw-SQL/raw-DDL families while retaining only **dishonest leftovers**
- R039 — advances migration and schema capability by requiring real partition create/list/drop coverage instead of stopping at the current `PARTITION BY` raw DDL note
- R040 — advances the SQLite-path constraint by requiring M033's design to leave a clean extension path without forcing live SQLite work into this milestone
- R041 — informs the deferred boundary: SQLite-specific extras remain later work, so M033 should shape the seams rather than trying to ship that implementation now

## Scope

### In Scope

- build a **broader first pass** neutral expression DSL for honestly reusable expression-heavy query/update/insert/select work
- cover the recurring insert/update/select-expression wins now, including function-valued updates, computed upsert/update expressions, parameterized select expressions, and the harder read-side shapes where covering them retires real recurring Mesher keep-sites
- cover JSONB-heavy data paths used in `mesher/` where the reusable part belongs in the broader neutral core and the PG-only part belongs in explicit extras
- add explicit PG extras for the real recurring PG-only families: full-text search, crypto helpers, partition lifecycle helpers, and other genuinely PG-specific behavior
- own the common partition lifecycle **create/list/drop** path through honest first-class helpers, while keeping truly dynamic catalog work raw if a dedicated surface would start lying
- clean up the small sharp gaps that are cheap and honest to solve during this wave, such as `now()`-driven updates and other narrow recurring storage edges
- rewrite the recurring raw-SQL/raw-DDL families in `mesher/storage/queries.mpl`, `mesher/storage/writer.mpl`, and related migration/schema helpers as far as the new surfaces honestly reach, with the target of leaving only **dishonest leftovers**
- ship the docs outcome in the **public Mesh docs**: update the public database docs with the new neutral DSL and PG extras, and anchor them in a real Mesher-backed example path
- leave the SQLite path at **design only** in M033: the seams must be credible, but live SQLite extras and runtime proof are deferred

### Out of Scope / Non-Goals

- fake portability that hides PG-only behavior inside a misleading neutral API
- a PG-only trap that makes future SQLite extras awkward or forces a later redesign
- raw-SQL purity or zero raw DDL as the goal
- product redesign in `mesher/`
- a fake universal SQL AST or giant abstract ORM disconnected from the actual recurring Mesher pressure map
- live SQLite extras or runtime SQLite proof in this milestone
- a broad docs/tutorial/marketing sweep beyond the public Mesh docs surfaces M033 is actually changing

## Technical Constraints

- The broader neutral core is allowed to be ambitious, but it still has to be justified by recurring real Mesher pressure rather than abstraction for its own sake
- Vendor-specific behavior must stay explicit where the capability is not honestly portable, especially around JSONB, full-text search, crypto helpers, partition lifecycle, and catalog behavior
- `mesher/` should remain behaviorally stable from the product point of view while the platform underneath it improves
- The milestone should push hard on retiring recurring raw boundaries, but the final tail must be only **dishonest leftovers**, not helpers added just to make the raw count look smaller
- M033 must leave a clean SQLite extension path even though SQLite extras themselves are deferred
- Acceptance has to include live Postgres proof through real Mesher flows, not compile-only proof, SQL-string snapshots alone, or comment-level cleanup alone
- The docs/example work has to stay in the public Mesh docs and stay truthful and Mesher-backed, not drift into generic API marketing

## Integration Points

- `mesher/` — primary proof surface and the main consumer of the stronger data-layer surfaces
- `compiler/mesh-rt/src/db/query.rs` — neutral query/expression surface to expand
- `compiler/mesh-rt/src/db/repo.rs` — neutral insert/update/delete/upsert surface to expand or reshape around the new expression DSL
- `compiler/mesh-rt/src/db/migration.rs` — migration baseline plus explicit PG partition-helper surface
- PostgreSQL — required live integration target for JSONB, `ON CONFLICT`, `now()`, full-text search, crypto helpers, partitioned tables, and catalog-backed partition lifecycle behavior
- `website/docs/docs/databases/index.md` — public Mesh docs surface that needs to reflect the new neutral DSL and PG extras truthfully
- Mesh compiler/runtime tests — required lower-level proof surface for the new neutral DSL and explicit PG helpers
- SQLite — not a live M033 target, but the boundary decisions here must preserve a credible later explicit-extras path

## Open Questions

- How far the first DSL pass should go on the nastiest multi-derived-table / multi-scalar-subquery read shapes in `mesher/storage/queries.mpl` — current thinking: take them when they retire a real recurring Mesher boundary, stop where the helper would become a dishonest leftover in disguise
- How the final retained raw tail should be presented to downstream users and future slices — current thinking: make the remaining **dishonest leftovers** explicit in milestone artifacts and docs rather than implying portability or hiding raw boundaries
- How to stage the public Mesh docs updates across the roadmap — current thinking: keep the docs outcome anchored in `website/docs/docs/databases/index.md` and one real Mesher-backed example path, not a broader docs wave
