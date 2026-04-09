---
id: T02
parent: S04
milestone: M032
provides:
  - Truthful extract_event_fields boundary comments aligned with Mesher’s real JSONB/ORM keep-surface
key_files:
  - mesher/storage/queries.mpl
  - .gsd/milestones/M032/slices/S04/S04-PLAN.md
  - .gsd/milestones/M032/slices/S04/tasks/T02-PLAN.md
key_decisions:
  - Kept Mesher’s SQL-side extraction and writer-side raw-SQL insertion flow unchanged; this task only corrected the explanatory boundary surface and closed the slice proof gate.
patterns_established:
  - When retiring stale from_json folklore in Mesher, preserve type-file from_json notes that describe Row/JSONB text decoding, and keep the raw-SQL rationale anchored in mesher/storage/queries.mpl plus mesher/storage/writer.mpl.
observability_surfaces:
  - Storage.Queries.extract_event_fields(...) -> Err("extract_event_fields: no result")
  - EventProcessor.process_event(...) -> Ingestion.Routes.route_to_processor(...) -> bad_request_response(reason)
  - Slice grep gates for stale from_json folklore and ORM-boundary keep-sites
duration: 5m
verification_result: passed
completed_at: 2026-03-24 19:12:22 EDT
blocker_discovered: false
---

# T02: Retire stale extract_event_fields folklore and close the Mesher proof gate

**Rewrote Mesher’s `extract_event_fields(...)` banner to the real JSONB/ORM boundary and closed the slice proof gate.**

## What Happened

The unit’s pre-flight contract still required an `## Observability Impact` section in `T02-PLAN.md`, so I added that first.

Then I reread `mesher/storage/queries.mpl` against `mesher/storage/writer.mpl`, `mesher/types/event.mpl`, `mesher/types/issue.mpl`, `mesher/services/event_processor.mpl`, the existing compiler proof in `compiler/meshc/tests/e2e.rs`, and the reference replay in `scripts/verify-m032-s01.sh`.

The only stale surface left was the `extract_event_fields(...)` banner in `mesher/storage/queries.mpl`. I rewrote it so it explains the real reason the query stays in raw SQL: the fingerprint fallback chain depends on PostgreSQL JSONB operators plus CASE / `jsonb_array_elements` / `string_agg`, and that extraction boundary intentionally stays on the same raw-SQL side as `Storage.Writer.insert_event(...)`. I did not change any runtime behavior, SQL text, or signatures.

`mesher/storage/writer.mpl` stayed untouched as the guard file, and `mesher/types/event.mpl` plus `mesher/types/issue.mpl` kept their row-shape `from_json` notes intact.

## Verification

I ran the full slice verification gate after the comment rewrite. The supported cross-module `from_json` compiler proof still passes, `meshc fmt --check mesher` is clean, `meshc build mesher` succeeds, the stale-folklore grep across `event_processor.mpl` and `queries.mpl` is now empty, `storage/writer.mpl` remains free of `from_json`, the intended keep-sites in `mesher/types/*.mpl` and the storage ORM-boundary comments still exist, and the named `extract_event_fields: no result` diagnostic surface is still present.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -q -p meshc --test e2e e2e_m032_supported_cross_module_from_json -- --nocapture` | 0 | ✅ pass | 10.29s |
| 2 | `cargo run -q -p meshc -- fmt --check mesher` | 0 | ✅ pass | 8.01s |
| 3 | `cargo run -q -p meshc -- build mesher` | 0 | ✅ pass | 16.16s |
| 4 | `bash -lc '! rg -n "cross-module from_json limitation|from_json limitation per decision \[88-02\]|Validation is done by the caller|caller is responsible for JSON parsing and field validation" mesher/services/event_processor.mpl mesher/storage/queries.mpl'` | 0 | ✅ pass | 0.07s |
| 5 | `bash -lc '! rg -n "from_json" mesher/storage/writer.mpl'` | 0 | ✅ pass | 0.15s |
| 6 | `rg -n "from_json" mesher/types/event.mpl mesher/types/issue.mpl` | 0 | ✅ pass | 0.03s |
| 7 | `rg -n "ORM boundary: ORM fragments cannot express CASE/jsonb_array_elements/string_agg|Repo.insert cannot express server-side JSONB extraction" mesher/storage/queries.mpl mesher/storage/writer.mpl` | 0 | ✅ pass | 0.03s |
| 8 | `rg -n 'extract_event_fields: no result' mesher/storage/queries.mpl` | 0 | ✅ pass | 0.03s |

## Diagnostics

Future inspection should start at three places:

- `mesher/storage/queries.mpl`: the `extract_event_fields(...)` banner now documents the real JSONB/fingerprint/ORM boundary, and the function still returns `Err("extract_event_fields: no result")` when the helper query yields no row.
- `mesher/storage/writer.mpl`: the insert-side raw-SQL note remains the matching keep-site for server-side JSONB extraction during event persistence.
- `mesher/services/event_processor.mpl` plus `mesher/ingestion/routes.mpl`: query-layer failures still flow unchanged through `ProcessEvent(...)` to `bad_request_response(reason)` with no second JSON parsing path.

## Deviations

Added the missing `## Observability Impact` section to `.gsd/milestones/M032/slices/S04/tasks/T02-PLAN.md` before implementation because the unit pre-flight contract explicitly required that fix.

## Known Issues

None.

## Files Created/Modified

- `mesher/storage/queries.mpl` — rewrote the stale `extract_event_fields(...)` banner to the real PostgreSQL JSONB / ORM-boundary rationale without changing behavior.
- `.gsd/milestones/M032/slices/S04/tasks/T02-PLAN.md` — added the missing `## Observability Impact` section required by the unit pre-flight contract.
- `.gsd/milestones/M032/slices/S04/tasks/T02-SUMMARY.md` — recorded the task outcome, verification evidence, and future inspection surfaces.
- `.gsd/milestones/M032/slices/S04/S04-PLAN.md` — marked T02 complete.
