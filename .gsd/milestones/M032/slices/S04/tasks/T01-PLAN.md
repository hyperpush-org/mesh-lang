---
estimated_steps: 4
estimated_files: 5
skills_used:
  - debug-like-expert
---

# T01: Rewrite EventProcessor boundary comments to match the live ingestion flow

**Slice:** S04 — Module-boundary JSON and workaround convergence
**Milestone:** M032

## Description

Make `mesher/services/event_processor.mpl` truthful again without changing product behavior. The file should stop blaming solved cross-module `from_json` limits, stop claiming caller-side `validate_event(...)` already runs, and drop the dead `compute_fingerprint` import if it remains unused. Keep the SQL-backed `extract_event_fields(...)` flow and the `process_extracted_fields(...)` helper intact; this task is comment convergence, not an event-pipeline redesign.

## Steps

1. Re-read `mesher/services/event_processor.mpl` alongside `mesher/ingestion/routes.mpl`, `mesher/ingestion/validation.mpl`, `mesher/ingestion/fingerprint.mpl`, and `mesher/storage/queries.mpl` to anchor the comment rewrite to the actual call path and current keep-sites.
2. Rewrite the top-of-file summary, the `route_event(...)` banner, and the `ProcessEvent` call comment so they describe the real flow: SQL-side field extraction, discard check, issue upsert, and forwarding to `StorageWriter`.
3. Remove `from Ingestion.Fingerprint import compute_fingerprint` if it is still unused after the wording cleanup, but do not introduce Mesh-side payload parsing, `validate_event(...)` calls, or other behavior changes.
4. Verify the stale phrases and dead import are gone from `mesher/services/event_processor.mpl`.

## Must-Haves

- [ ] `mesher/services/event_processor.mpl` no longer claims a cross-module `from_json` limitation or pre-existing caller-side `validate_event(...)` enforcement.
- [ ] `mesher/services/event_processor.mpl` keeps `route_event(...)`, `process_extracted_fields(...)`, and the existing `EventProcessor` API shape unchanged.
- [ ] The unused `compute_fingerprint` import is removed if it is still dead after the rewrite.

## Observability Impact

- No new runtime signal is introduced here; the task keeps the existing `EventProcessor.process_event(...) -> route_to_processor(...) -> bad_request_response(reason)` error path intact while making the comments point future agents at the real inspection surfaces.
- The authoritative caller-side signal remains `mesher/ingestion/routes.mpl`, which currently performs payload-size validation only before `ProcessEvent`. If a future agent expects `validate_event(...)` to run here, this file pair should now make that mismatch obvious.
- The authoritative extraction/failure surface remains `mesher/storage/queries.mpl`, where `extract_event_fields(...)` can still return `Err("extract_event_fields: no result")`. This task must not rename, swallow, or relocate that diagnostic.
- Redaction constraint: keep comments structural. Do not add examples or diagnostics that embed raw event payloads, fingerprints, or auth material.

## Verification

- `bash -lc '! rg -n "cross-module from_json limitation|from_json limitation per decision \\[88-02\\]|Validation is done by the caller|caller is responsible for JSON parsing and field validation" mesher/services/event_processor.mpl'`
- `bash -lc '! rg -n "^from Ingestion\\.Fingerprint import compute_fingerprint$" mesher/services/event_processor.mpl'`

## Inputs

- `mesher/services/event_processor.mpl` — stale module-boundary and validation comments to rewrite.
- `mesher/ingestion/routes.mpl` — authoritative caller flow showing only payload-size validation before `ProcessEvent`.
- `mesher/ingestion/validation.mpl` — authoritative location of `validate_event(...)`, which is currently unused by the route path.
- `mesher/ingestion/fingerprint.mpl` — guardrail for the Mesh-side fingerprint helper that should not be pulled into this slice.
- `mesher/storage/queries.mpl` — current SQL-side field extraction contract that the service comments need to describe truthfully.

## Expected Output

- `mesher/services/event_processor.mpl` — truthful service-level comments with no dead `compute_fingerprint` import.
