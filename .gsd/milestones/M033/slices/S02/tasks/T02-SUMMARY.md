---
id: T02
parent: S02
milestone: M033
provides: []
requires: []
affects: []
key_files: ["mesher/storage/queries.mpl", "mesher/storage/writer.mpl", ".gsd/DECISIONS.md", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Used `Expr.fn_call(...)` with PostgreSQL-native `jsonb_extract_path`, `jsonb_extract_path_text`, `jsonb_exists`, and `jsonb_build_object` alongside `Pg` casts/operators instead of widening the dedicated `Pg` helper list mid-slice.", "Kept `extract_event_fields` as the explicit S03 raw keep-site because its fingerprint fallback still depends on CASE + WITH ORDINALITY + scalar-subquery behavior, while moving the rest of the T02-owned search/JSONB/write helpers off raw whole-query strings."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "`cargo run -q -p meshc -- build mesher` passed both before and after formatting, proving the rewritten Mesher storage paths compile on the real app build. `cargo run -q -p meshc -- fmt --check mesher` initially failed because the edited Mesh files needed canonical formatting; after `cargo run -q -p meshc -- fmt mesher`, the formatter check passed. A timed raw-boundary scan over `search_events_fulltext`, `filter_events_by_tag`, `event_breakdown_by_tag`, `create_alert_rule`, `fire_alert`, `get_event_alert_rules`, `get_threshold_rules`, and `insert_event` confirmed those helpers no longer call `Repo.query_raw` or `Repo.execute_raw`, while `extract_event_fields` is still present as the named S03 raw keep-site with an explicit comment and `Repo.query_raw(pool, sql, [event_json])` line. For slice-level verification, I also ran `cargo test -p meshc --test e2e_m033_s02 -- --nocapture` and `bash scripts/verify-m033-s02.sh`; both fail for the expected T03-owned reasons because the test target and verifier script have not been added yet, so the task-level build/format bar is green while the T03 proof bundle remains pending by plan."
completed_at: 2026-03-25T17:02:52.885Z
blocker_discovered: false
---

# T02: Rewrite Mesher JSONB and search helpers onto explicit Pg expression surfaces

> Rewrite Mesher JSONB and search helpers onto explicit Pg expression surfaces

## What Happened
---
id: T02
parent: S02
milestone: M033
key_files:
  - mesher/storage/queries.mpl
  - mesher/storage/writer.mpl
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Used `Expr.fn_call(...)` with PostgreSQL-native `jsonb_extract_path`, `jsonb_extract_path_text`, `jsonb_exists`, and `jsonb_build_object` alongside `Pg` casts/operators instead of widening the dedicated `Pg` helper list mid-slice.
  - Kept `extract_event_fields` as the explicit S03 raw keep-site because its fingerprint fallback still depends on CASE + WITH ORDINALITY + scalar-subquery behavior, while moving the rest of the T02-owned search/JSONB/write helpers off raw whole-query strings.
duration: ""
verification_result: mixed
completed_at: 2026-03-25T17:02:52.887Z
blocker_discovered: false
---

# T02: Rewrite Mesher JSONB and search helpers onto explicit Pg expression surfaces

**Rewrite Mesher JSONB and search helpers onto explicit Pg expression surfaces**

## What Happened

Rewrote the remaining S02-owned Mesher PostgreSQL search/JSONB/write paths onto the explicit expression surface added in T01. In `mesher/storage/queries.mpl`, `search_events_fulltext` now builds the inline FTS vector/query through `Pg.to_tsvector`, `Pg.plainto_tsquery`, `Pg.ts_rank`, and `Query.where_expr`/`Query.select_expr`; `filter_events_by_tag` now uses `Pg.jsonb_contains` inside `Query.where_expr`; `event_breakdown_by_tag` now uses `jsonb_exists` plus `jsonb_extract_path_text` through structured expressions instead of a whole raw query string. I also rewrote the alert-rule JSONB family so `create_alert_rule` uses `Repo.insert_expr` with JSONB extraction/defaulting from the request body, `fire_alert` uses `Repo.insert_expr` plus `Repo.update_where_expr` for the follow-up timestamp update, and `get_event_alert_rules` / `get_threshold_rules` now filter on `condition_json` through structured JSONB extraction rather than raw `condition_json->>` fragments. In `mesher/storage/writer.mpl`, `insert_event` now uses `Repo.insert_expr` over the real `events` table with server-side JSONB extraction/defaulting for `exception`, `stacktrace`, `breadcrumbs`, `tags`, `extra`, `user_context`, `sdk_name`, and `sdk_version`, so event ingest stays on the real runtime path without duplicating JSON parsing in Mesh. I then reclassified `extract_event_fields` as the explicit honest raw S03 keep-site and rewrote its comment to name the actual reason it remains raw: the CASE + WITH ORDINALITY + `jsonb_array_elements` / `string_agg` fingerprint chain. I recorded the downstream API-pattern choice in decision D059 and added a knowledge note explaining that Mesh's PG query path already returns selected values as strings, so computed rank/count fields can stay numeric in SQL for ordering without forcing `::text` casts.

## Verification

`cargo run -q -p meshc -- build mesher` passed both before and after formatting, proving the rewritten Mesher storage paths compile on the real app build. `cargo run -q -p meshc -- fmt --check mesher` initially failed because the edited Mesh files needed canonical formatting; after `cargo run -q -p meshc -- fmt mesher`, the formatter check passed. A timed raw-boundary scan over `search_events_fulltext`, `filter_events_by_tag`, `event_breakdown_by_tag`, `create_alert_rule`, `fire_alert`, `get_event_alert_rules`, `get_threshold_rules`, and `insert_event` confirmed those helpers no longer call `Repo.query_raw` or `Repo.execute_raw`, while `extract_event_fields` is still present as the named S03 raw keep-site with an explicit comment and `Repo.query_raw(pool, sql, [event_json])` line. For slice-level verification, I also ran `cargo test -p meshc --test e2e_m033_s02 -- --nocapture` and `bash scripts/verify-m033-s02.sh`; both fail for the expected T03-owned reasons because the test target and verifier script have not been added yet, so the task-level build/format bar is green while the T03 proof bundle remains pending by plan.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- fmt --check mesher` | 1 | ❌ fail | 27600ms |
| 2 | `cargo run -q -p meshc -- build mesher` | 0 | ✅ pass | 27500ms |
| 3 | `cargo run -q -p meshc -- fmt mesher` | 0 | ✅ pass | 10500ms |
| 4 | `bash scripts/verify-m033-s02.sh` | 127 | ❌ fail | 10500ms |
| 5 | `cargo test -p meshc --test e2e_m033_s02 -- --nocapture` | 101 | ❌ fail | 10400ms |
| 6 | `cargo run -q -p meshc -- fmt --check mesher` | 0 | ✅ pass | 24800ms |
| 7 | `cargo run -q -p meshc -- build mesher` | 0 | ✅ pass | 24800ms |
| 8 | `python3 raw-boundary scan + rg extract_event_fields keep-site` | 0 | ✅ pass | 3000ms |


## Deviations

Ran the slice-level verification commands proactively even though T03 owns the verifier surfaces; they failed for expected missing-artifact reasons (`e2e_m033_s02` target and `scripts/verify-m033-s02.sh` do not exist yet). Also needed one `cargo run -q -p meshc -- fmt mesher` pass after the edits before the required `fmt --check` would pass. No scope or architecture-plan deviation was needed.

## Known Issues

The slice-level proof surfaces are still pending T03: `cargo test -p meshc --test e2e_m033_s02 -- --nocapture` fails because the `e2e_m033_s02` target does not exist yet, and `bash scripts/verify-m033-s02.sh` fails because the verifier script is not present yet.

## Files Created/Modified

- `mesher/storage/queries.mpl`
- `mesher/storage/writer.mpl`
- `.gsd/DECISIONS.md`
- `.gsd/KNOWLEDGE.md`


## Deviations
Ran the slice-level verification commands proactively even though T03 owns the verifier surfaces; they failed for expected missing-artifact reasons (`e2e_m033_s02` target and `scripts/verify-m033-s02.sh` do not exist yet). Also needed one `cargo run -q -p meshc -- fmt mesher` pass after the edits before the required `fmt --check` would pass. No scope or architecture-plan deviation was needed.

## Known Issues
The slice-level proof surfaces are still pending T03: `cargo test -p meshc --test e2e_m033_s02 -- --nocapture` fails because the `e2e_m033_s02` target does not exist yet, and `bash scripts/verify-m033-s02.sh` fails because the verifier script is not present yet.
