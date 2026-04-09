---
id: T01
parent: S04
milestone: M032
provides:
  - Truthful EventProcessor boundary comments aligned with the live SQL-backed ingestion flow
key_files:
  - mesher/services/event_processor.mpl
  - .gsd/milestones/M032/slices/S04/S04-PLAN.md
  - .gsd/milestones/M032/slices/S04/tasks/T01-PLAN.md
key_decisions:
  - Kept EventProcessor runtime behavior and API shape unchanged; this task only corrected the explanatory boundary surface and removed the dead fingerprint import.
patterns_established:
  - For Mesher ingestion-boundary truth, inspect mesher/ingestion/routes.mpl for caller behavior and mesher/storage/queries.mpl for SQL-side extraction rationale before editing service comments.
observability_surfaces:
  - Storage.Queries.extract_event_fields(...) -> Err("extract_event_fields: no result")
  - EventProcessor.process_event(...) -> Ingestion.Routes.route_to_processor(...) -> bad_request_response(reason)
  - Slice/task plan grep gates for stale from_json folklore
duration: 28m
verification_result: passed
completed_at: 2026-03-24 19:07:36 EDT
blocker_discovered: false
---

# T01: Rewrite EventProcessor boundary comments to match the live ingestion flow

**Rewrote EventProcessor’s stale ingestion comments to match the live SQL-backed flow and removed the dead fingerprint import.**

## What Happened

The unit’s pre-flight checks flagged missing observability sections, so I first updated `S04-PLAN.md` with `## Observability / Diagnostics` plus an explicit diagnostic grep for `extract_event_fields: no result`, and updated `T01-PLAN.md` with `## Observability Impact`.

Then I reread `mesher/services/event_processor.mpl` against `mesher/ingestion/routes.mpl`, `mesher/ingestion/validation.mpl`, `mesher/ingestion/fingerprint.mpl`, and `mesher/storage/queries.mpl`. The actual call path is narrower than the stale comments claimed: the route layer does auth/rate-limit/payload-size checks, `EventProcessor` forwards raw JSON to SQL-side field extraction, and the service then does discard checks, issue upsert, and `StorageWriter` forwarding.

I rewrote the file header, the `route_event(...)` banner, and the `ProcessEvent` call comment to describe that real flow. I also removed the unused `from Ingestion.Fingerprint import compute_fingerprint` import. No runtime behavior, helper signatures, or service API shape changed.

`cargo run -q -p meshc -- fmt --check mesher` initially reported that `mesher/services/event_processor.mpl` needed formatting, so I ran `cargo run -q -p meshc -- fmt mesher/services/event_processor.mpl` and reran the full evidence set.

## Verification

Task-local verification passed: the stale EventProcessor phrases are gone and the dead fingerprint import is gone.

I also ran the slice-level checks to establish the current closeout picture. The supported cross-module `from_json` e2e proof still passes, `meshc fmt --check mesher` and `meshc build mesher` are green, `mesher/storage/writer.mpl` still has no revived `from_json` folklore, the intended keep-sites in `mesher/types/*.mpl` and the ORM-boundary rationale grep still exist, and the diagnostic `extract_event_fields: no result` surface remains present.

The only slice-level failure is the expected stale-comment grep against `mesher/storage/queries.mpl`, which still contains the old cross-module `from_json` wording at line 482. That belongs to T02, not this task.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -lc '! rg -n "cross-module from_json limitation|from_json limitation per decision \[88-02\]|Validation is done by the caller|caller is responsible for JSON parsing and field validation" mesher/services/event_processor.mpl'` | 0 | ✅ pass | 0.09s |
| 2 | `bash -lc '! rg -n "^from Ingestion\.Fingerprint import compute_fingerprint$" mesher/services/event_processor.mpl'` | 0 | ✅ pass | 0.20s |
| 3 | `cargo test -q -p meshc --test e2e e2e_m032_supported_cross_module_from_json -- --nocapture` | 0 | ✅ pass | 7.96s |
| 4 | `cargo run -q -p meshc -- fmt --check mesher` | 0 | ✅ pass | 6.75s |
| 5 | `cargo run -q -p meshc -- build mesher` | 0 | ✅ pass | 14.16s |
| 6 | `bash -lc '! rg -n "cross-module from_json limitation|from_json limitation per decision \[88-02\]|Validation is done by the caller|caller is responsible for JSON parsing and field validation" mesher/services/event_processor.mpl mesher/storage/queries.mpl'` | 1 | ❌ fail | 0.10s |
| 7 | `bash -lc '! rg -n "from_json" mesher/storage/writer.mpl'` | 0 | ✅ pass | 0.05s |
| 8 | `rg -n "from_json" mesher/types/event.mpl mesher/types/issue.mpl` | 0 | ✅ pass | 0.03s |
| 9 | `rg -n "ORM boundary: ORM fragments cannot express CASE/jsonb_array_elements/string_agg|Repo.insert cannot express server-side JSONB extraction" mesher/storage/queries.mpl mesher/storage/writer.mpl` | 0 | ✅ pass | 0.04s |
| 10 | `rg -n 'extract_event_fields: no result' mesher/storage/queries.mpl` | 0 | ✅ pass | 0.03s |

## Diagnostics

Future inspection should start at three surfaces:

- `mesher/ingestion/routes.mpl`: the route path still only does payload-size validation before calling `ProcessEvent`.
- `mesher/storage/queries.mpl`: `extract_event_fields(...)` remains the authoritative SQL-side extraction path and still exposes `Err("extract_event_fields: no result")`.
- `mesher/services/event_processor.mpl`: the rewritten comments now match the live flow from raw JSON input through discard check, issue upsert, and `StorageWriter` forwarding.

If the HTTP error path needs tracing later, follow `EventProcessor.process_event(...)` into `route_to_processor(...)`, which still turns service errors into `bad_request_response(reason)`.

## Deviations

Added the missing observability sections to `S04-PLAN.md` and `T01-PLAN.md` before implementation because the unit pre-flight contract explicitly required those fixes.

## Known Issues

The full slice stale-comment grep still fails because `mesher/storage/queries.mpl:482` retains the old `cross-module from_json limitation` wording. That is the planned T02 cleanup target, not a blocker for T01.

## Files Created/Modified

- `mesher/services/event_processor.mpl` — rewrote stale boundary comments to match the live SQL-backed ingestion flow, removed the dead fingerprint import, and formatted the file.
- `.gsd/milestones/M032/slices/S04/S04-PLAN.md` — added the missing Observability / Diagnostics section and an explicit diagnostic verification check.
- `.gsd/milestones/M032/slices/S04/tasks/T01-PLAN.md` — added the missing Observability Impact section.
- `.gsd/milestones/M032/slices/S04/tasks/T01-SUMMARY.md` — recorded the task outcome, verification evidence, and handoff notes.
