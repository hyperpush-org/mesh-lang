# M033 / S05 — Research

**Date:** 2026-03-25

## Summary

S05 is a targeted closeout slice, not another runtime-surface slice.

The ORM/migration/runtime work is already shipped in S01-S04. What remains is the **truth surface**:

1. publish the shipped neutral-vs-PG contract in the public database docs
2. add one canonical S05 acceptance command that replays the assembled live Postgres proof stack and mechanically checks that the public docs describe the real surface, not a nicer imaginary one

The active requirement pressure for this slice is concentrated:

- **R038** still needs final validation at the milestone level. S03 and S04 advanced it and enforced the honest keep-list mechanically, but S05 is the slice that can close the loop by replaying the full proof stack together and documenting the remaining honest raw boundary truthfully.
- **R040** is still a seam/contract requirement, not a runtime-SQLite requirement. S05 needs to make that seam explicit in public docs: the neutral core is `Expr` / `Query` / `Repo` / honest `Migration.create_index(...)`; PostgreSQL-only behavior stays under `Pg.*`; SQLite extras are later work.
- **R036 / R037 / R039** are already validated by S02-S04 runtime proof. S05 should explain those validated surfaces, not reopen or redesign them.

This is also one of the few slices where the repo already contains the right proof pieces. `compiler/meshc/tests/e2e_m033_s02.rs`, `e2e_m033_s03.rs`, `e2e_m033_s04.rs` and their verifier scripts already cover the real runtime/catalog behavior. The S05 job is to compose and document them honestly.

Skill-informed constraint: the installed **postgresql-database-engineering** skill is directly aligned with the existing M033 decisions — keep acceptance on **live Postgres behavior and catalogs**, not compile-only examples or SQL snapshots. For docs work, the repo uses **VitePress** (`website/package.json`), and I installed `antfu/skills@vitepress` globally so later units can use it if needed.

## Requirements Focus

### R038 — honest raw-tail collapse

S05 owns the final validation surface for R038.

What already exists:
- `scripts/verify-m033-s02.sh` enforces the S02-owned PG helper families and the explicit `extract_event_fields` raw keep-site.
- `scripts/verify-m033-s03.sh` enforces the short named S03 raw-read keep-list.
- `scripts/verify-m033-s04.sh` enforces the migration/runtime partition boundary and bans the old raw DDL/query ownership sites.

What S05 still needs:
- one integrated acceptance command that runs the assembled proof stack **serially**
- public docs that describe the remaining raw boundary as an **honest leftover list / escape-hatch story**, not as zero-raw marketing

### R040 — credible SQLite seam

S05 supports and likely closes the public-proof side of R040 by documenting the actual boundary:

- portable/neutral: `Expr`, `Query`, `Repo`, honest `Migration.create_index(...)`
- PostgreSQL-only: JSONB, full-text search, pgcrypto, partition/schema helpers under `Pg.*`
- deferred: SQLite-specific extras are later work, not implied by the PG helpers

Do **not** let the docs imply that JSONB/search/crypto/partition helpers are neutral APIs.

## Skills Discovered

- Already installed and directly relevant:
  - `postgresql-database-engineering` — supports the live-Postgres verification posture for the integrated replay
- Newly discovered and installed for later units:
  - `antfu/skills@vitepress` — directly relevant because `website/` is a VitePress docs site (`website/package.json`, `npm --prefix website run build`)

No other new external skill looked necessary. This slice is repo-local Mesh + VitePress + existing Postgres proof surfaces.

## Recommendation

Treat S05 as **two seams**:

1. **Public docs rewrite** in `website/docs/docs/databases/index.md`
2. **Canonical S05 verifier** in `scripts/verify-m033-s05.sh`

The verifier should:
- run the assembled M033 proof stack **serially**
- build the docs site
- do an exact-string docs-truth sweep in the style of `reference-backend/scripts/verify-production-proof-surface.sh`

### Strong recommendation: do not add a new large Rust harness unless you find a real missing behavior

The repo already has truthful live-Postgres proof surfaces:
- S02 = PG helper families on Mesher storage paths
- S03 = live Mesher HTTP/read/composed behavior
- S04 = migration/catalog/runtime partition lifecycle + startup bootstrap/logging

A new `compiler/meshc/tests/e2e_m033_s05.rs` would mostly duplicate Docker, Postgres, Mesher spawn, HTTP, and DB helper code already copied across S02-S04. There is **no shared M033 test helper module** in `compiler/meshc/tests/`, so a new Rust target is expensive boilerplate unless S05 truly needs a new missing cross-slice scenario.

Right now the lower-risk, more honest path is:
- keep runtime proof in the existing S02-S04 harnesses
- add one S05 wrapper/verifier that composes them and checks doc truth mechanically

If an executor later discovers a real user-visible gap that is not already covered by S02-S04, then a small dedicated Rust test is justified. But that should be a response to an actual missing proof, not the default starting point.

## Implementation Landscape

### `website/docs/docs/databases/index.md`

This is the main docs gap.

Current state:
- the page is still a **generic databases guide**
- it documents `Sqlite`, `Pg`, `Pool`, and `deriving(Row)`
- it does **not** document `Expr`, `Query`, `Repo`, `Migration`, the neutral-vs-PG split, or the Mesher-backed M033 proof path
- it already contains the repo’s current public-doc honesty pattern near the top: a short “Production backend proof” callout that points readers to the canonical proof surface instead of overclaiming on the guide page

That means S05 docs work is not a tiny append. It is a real content expansion / partial rewrite of this page.

What the page should cover now:

#### Neutral expression core
Use real parseable Mesh examples drawn from the shipped surface, not pseudo-API prose.

Best source examples:
- `compiler/meshc/tests/e2e_m033_s01.rs`
  - `Query.select_exprs(...)` with `Expr.coalesce(...)`, `Expr.add(...)`, `Expr.case_when(...)`
  - `Repo.update_where_expr(...)`
  - `Repo.insert_or_update_expr(...)`
  - `Expr.null()` for real NULL assignment

Important gotcha:
- **Use `Expr.label(...)`, not `Expr.alias(...)`.**
- This is a real parser boundary already captured in knowledge/decisions. Runtime still lowers through the alias intrinsic, but the Mesh-callable public surface for docs/examples is `Expr.label(...)`.

#### Explicit PostgreSQL extras
Best Mesher-backed examples already exist in real code:
- `mesher/storage/queries.mpl`
  - `create_user(...)` / `authenticate_user(...)` for `Pg.crypt(...)` + `Pg.gen_salt(...)`
  - `search_events_fulltext(...)` for `Pg.to_tsvector(...)`, `Pg.plainto_tsquery(...)`, `Pg.ts_rank(...)`, and `Pg.tsvector_matches(...)`
  - `create_alert_rule(...)` / `fire_alert(...)` for JSONB extract/build helpers on the real app path
- `mesher/storage/writer.mpl`
  - `insert_event(...)` for `Repo.insert_expr(...)` + `Pg.jsonb(...)` + JSON extraction/defaulting
- `mesher/migrations/20260216120000_create_initial_schema.mpl`
  - `Pg.create_extension(...)`
  - `Pg.create_range_partitioned_table(...)`
  - `Pg.create_gin_index(...)`
- `mesher/storage/schema.mpl`
  - `Pg.create_daily_partitions_ahead(...)`
  - `Pg.list_daily_partitions_before(...)`
  - `Pg.drop_partition(...)`

Important gotcha if docs show `jsonb_build_object(...)` through `Expr.fn_call(...)`:
- string arguments need `Pg.text(...)` casts or PostgreSQL can fail parameter typing (`42P18`).
- If the docs do not need that exact example, it is simpler to avoid it than to teach a brittle half-truth.

#### Honest boundary / leftovers section
The docs should explicitly say:
- `Repo.query_raw`, `Repo.execute_raw`, and `Migration.execute` still exist as honest escape hatches
- M033 reduced the recurring Mesher raw families substantially, but it did **not** claim zero raw SQL/DDL
- the remaining raw tail is mechanical and named in the verifier scripts rather than hidden in prose

This matters for R038 and R040. Without it, the page will drift back into fake portability.

#### Proof section
The page should end with a short public proof section, analogous to the reference-backend proof page but smaller:
- what is proven
- which Mesher-backed files/examples anchor the story
- exact named commands for the integrated replay / docs truth gate
- where to look when proof fails

### `scripts/verify-m033-s05.sh`

This should be the canonical S05 acceptance command.

Best pattern to copy:
- `reference-backend/scripts/verify-production-proof-surface.sh`

Why that script matters here:
- it treats docs drift as a first-class failure mode
- it uses exact string checks over canonical command lists and vocabulary
- it makes “public docs describe real repo proof” mechanically enforceable

For S05, the new verifier should likely do three things:

1. **Run existing runtime proof surfaces serially**
   - `bash scripts/verify-m033-s02.sh`
   - `bash scripts/verify-m033-s03.sh`
   - `bash scripts/verify-m033-s04.sh`

2. **Build the public docs site**
   - `npm --prefix website run build`

3. **Run a Python exact-string docs-truth sweep** against `website/docs/docs/databases/index.md`
   - require the actual neutral API names
   - require the actual PG-only API names
   - require Mesher-backed file references
   - require the explicit portable-vs-PG-vs-later-SQLite wording
   - require the canonical proof commands you want to publicly stand behind

### Existing proof surfaces to compose

#### `scripts/verify-m033-s02.sh`
Owns the explicit PG-helper runtime proof and the `extract_event_fields` keep-site discipline.

Use it as the authoritative proof for:
- pgcrypto auth
- full-text search
- JSONB insert/filter/breakdown/defaulting
- alert-rule create/fire helpers
- S02 raw-boundary enforcement

#### `scripts/verify-m033-s03.sh`
Owns the honest read-side collapse proof.

Use it as the authoritative proof for:
- live Mesher HTTP/API read behavior
- composed reads and dashboard/detail/alerts/team surfaces
- named S03 raw-read keep-list enforcement

#### `scripts/verify-m033-s04.sh`
Owns the schema/runtime partition proof.

Use it as the authoritative proof for:
- helper-driven migration apply
- catalog truth (`pg_extension`, `pg_partitioned_table`, `pg_inherits`, index metadata)
- runtime partition create/list/drop
- Mesher startup partition bootstrap/logging

### `website/package.json`

This confirms the docs stack is VitePress and the build command is already standard:
- `npm --prefix website run build`

I ran this command during research. It passes. The build emits a large-chunk warning but exits 0; that warning is informational, not a blocker.

### `compiler/meshc/src/migrate.rs` and `compiler/meshc/tests/e2e.rs`

These are useful alignment sources for docs examples:
- the generated migration scaffold in `compiler/meshc/src/migrate.rs` already teaches the current honest split between `Migration.*` and `Pg.*`
- `compiler/meshc/tests/e2e.rs` already has compile coverage for the new migration/index/PG schema helper examples

The docs should stay consistent with these sources instead of inventing a cleaner but unshipped API.

## Natural Seams

### Seam 1 — docs content
Files:
- `website/docs/docs/databases/index.md`

Likely independent content chunks inside that one file:
- neutral `Expr` / `Query` / `Repo` section
- PG-only `Pg.*` section
- migration/schema extras section
- honest boundary / remaining raw / later SQLite section
- proof commands / failure inspection map

### Seam 2 — verifier / docs-truth automation
Files:
- `scripts/verify-m033-s05.sh`

Likely responsibilities:
- command runner wrapper
- artifact log directory like the prior slice verifiers
- Python exact-string docs-truth sweep

### Optional seam only if truly needed — a new Rust test
Files:
- maybe `compiler/meshc/tests/e2e_m033_s05.rs`

I do **not** recommend starting here. Only add this if, during implementation, you can name a real integrated Mesher behavior that S02-S04 do not already prove.

## What To Build Or Prove First

1. **Design the exact docs wording and section layout first.**
   The verifier should check exact phrases/API names, so the docs truth contract needs to exist before the sweep is written.

2. **Build the S05 verifier around that exact wording.**
   This is the cheapest way to turn doc drift into a mechanical failure.

3. **Run the cheap docs gate before the expensive replay.**
   - `npm --prefix website run build`
   Catch markdown/frontmatter/code-fence errors before spending minutes in the full live-Postgres stack.

4. **Run the full integrated replay last, serially.**
   The underlying slice verifiers are already slow and all use Docker/Postgres on host port 5432.

## Constraints And Gotchas

- **Serial only.** S02-S04 test/verifier flows all assume Docker can bind `5432:5432`. Do not parallelize the integrated replay.
- **No shared M033 Rust helper module exists.** A new Rust S05 harness means more copied container/HTTP/spawn code unless you first extract support code, which is probably too much churn this late in the milestone.
- **Use `Expr.label(...)` in docs examples.** `Expr.alias(...)` is not the Mesh-source-safe public spelling right now.
- **Do not let docs imply PG extras are portable.** `Migration.create_index(...)` is intentionally limited to honest neutral features; opclasses/methods/partition DDL stay under `Pg.*`.
- **Do not market zero raw.** S03 still has a named keep-list by design, and S02 keeps `extract_event_fields` explicit. The truthful story is “short named leftovers,” not “all raw SQL is gone.”
- **Keep the docs Mesher-backed.** The roadmap explicitly wants a Mesher-backed path, not a generic ORM brochure.
- **VitePress build warning is non-blocking.** `npm --prefix website run build` prints a chunk-size warning but still succeeds.

## Verification

### Cheap gate

Run this immediately after editing docs:

```bash
npm --prefix website run build
```

### Canonical S05 acceptance command

The slice should end with a new wrapper command:

```bash
bash scripts/verify-m033-s05.sh
```

### Underlying proof commands S05 should compose

Run these **serially** inside the wrapper:

```bash
bash scripts/verify-m033-s02.sh
bash scripts/verify-m033-s03.sh
bash scripts/verify-m033-s04.sh
npm --prefix website run build
```

### What the S05 docs-truth sweep should require

At minimum, require exact mention of the shipped surfaces the page is supposed to teach:

#### Neutral surface
- `Expr.label`
- `Expr.value`
- `Expr.column`
- `Expr.null`
- `Expr.case_when`
- `Expr.coalesce`
- `Query.where_expr`
- `Query.select_exprs`
- `Repo.insert_expr`
- `Repo.update_where_expr`
- `Repo.insert_or_update_expr`
- `Migration.create_index`

#### PostgreSQL-only surface
- `Pg.crypt`
- `Pg.gen_salt`
- `Pg.to_tsvector`
- `Pg.plainto_tsquery`
- `Pg.ts_rank`
- `Pg.jsonb`
- `Pg.create_extension`
- `Pg.create_range_partitioned_table`
- `Pg.create_gin_index`
- `Pg.create_daily_partitions_ahead`
- `Pg.list_daily_partitions_before`
- `Pg.drop_partition`

#### Honest boundary wording
Require explicit wording for all three:
- portable neutral core
- PostgreSQL-only helpers under `Pg.*`
- SQLite extras later / not yet runtime-proven here

#### Mesher-backed anchors
Require file references to the real proof/example path:
- `mesher/storage/queries.mpl`
- `mesher/storage/writer.mpl`
- `mesher/migrations/20260216120000_create_initial_schema.mpl`
- `mesher/storage/schema.mpl`

#### Proof commands
Require the exact public commands you want the docs to claim.

### Failure inspection order

If the final S05 verifier fails, inspect in this order:

1. docs build failure (`npm --prefix website run build`)
2. docs-truth exact-string failure in `scripts/verify-m033-s05.sh`
3. first failing underlying slice verifier (`verify-m033-s02`, then `s03`, then `s04`)
4. only then consider new runtime changes or a new integrated Rust harness
