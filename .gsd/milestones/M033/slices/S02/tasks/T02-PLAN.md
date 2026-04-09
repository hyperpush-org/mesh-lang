---
estimated_steps: 4
estimated_files: 2
skills_used:
  - postgresql-database-engineering
---

# T02: Rewrite Mesher JSONB and search flows onto explicit Pg helpers

**Slice:** S02 — Explicit PG extras for JSONB, search, and crypto
**Milestone:** M033

## Description

Move the rest of the slice-owned Mesher PG families onto the explicit helper surface now that the compiler/runtime seam exists. This task covers the real runtime paths that make the slice demo true: full-text search, JSONB containment and extraction, JSONB-backed alert inserts, and event ingest. It must also keep the raw boundary honest by explicitly retaining `extract_event_fields` only if it still belongs to the harder S03 read-side bucket.

## Steps

1. Rewrite the FTS and JSONB read helpers in `mesher/storage/queries.mpl` — especially `search_events_fulltext`, `filter_events_by_tag`, and `event_breakdown_by_tag` — to use `Pg.*` plus expression-valued `SELECT` / `WHERE` composition instead of raw whole-query strings.
2. Rewrite the alert-rule JSONB helpers in `mesher/storage/queries.mpl` — `create_alert_rule`, `fire_alert`, `get_event_alert_rules`, and `get_threshold_rules` — to use explicit PG JSON builders, extractors, casts, and expression-valued inserts/updates.
3. Rewrite `mesher/storage/writer.mpl::insert_event` onto `Repo.insert_expr` plus explicit PG JSONB extraction/defaulting so event ingest stays on the real storage path without duplicating JSON parsing in Mesh.
4. Reassess `extract_event_fields`; if ordinality/subquery behavior still makes it dishonest for S02, leave it raw with a precise comment naming it as an S03 keep-site instead of forcing it through an overbuilt helper.

## Must-Haves

- [ ] The S02-owned Mesher search and JSONB helper families move onto explicit `Pg.*` surfaces instead of raw whole-query strings where the helper boundary is honest
- [ ] `insert_event` uses expression-valued inserts plus PG JSONB helpers on the real write path
- [ ] Any leftover raw keep-site is named explicitly and justified, with `extract_event_fields` retained only if it still belongs to S03

## Verification

- `cargo run -q -p meshc -- build mesher`
- `cargo run -q -p meshc -- fmt --check mesher`

## Observability Impact

- Signals added/changed: named storage helpers should fail as discrete JSONB/search families instead of disappearing inside generic `Repo.query_raw(...)` strings
- How a future agent inspects this: inspect `mesher/storage/queries.mpl` and `mesher/storage/writer.mpl`, then rerun the S02 proof bundle once T03 lands
- Failure state exposed: the remaining honest raw boundary should be visible from comments and keep-list checks rather than inferred from ad hoc SQL fragments

## Inputs

- `compiler/mesh-rt/src/db/query.rs` — expression-valued SELECT/WHERE entrypoints and placeholder ordering that this task consumes
- `compiler/mesh-rt/src/db/repo.rs` — expression-valued insert support that event/alert rewrites depend on
- `compiler/mesh-typeck/src/infer.rs` — Mesh-visible `Pg.*`, `Query.*_expr`, and `Repo.insert_expr` signatures from T01
- `mesher/storage/queries.mpl` — current S02-owned JSONB/search/auth keep-sites and lightweight JSONB predicates
- `mesher/storage/writer.mpl` — current raw JSONB-heavy event ingest insert

## Expected Output

- `mesher/storage/queries.mpl` — search, alert-rule, and lightweight JSONB helper families rewritten onto explicit PG helpers with an honest leftover boundary
- `mesher/storage/writer.mpl` — event ingest rewritten onto expression-valued insert plus explicit PG JSONB extraction/defaulting
