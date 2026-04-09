# S04: Module-boundary JSON and workaround convergence

**Goal:** Remove the last stale module-boundary `from_json` folklore in the real Mesher event ingestion/storage path without redesigning the JSONB-backed pipeline.
**Demo:** `mesher/services/event_processor.mpl` and `mesher/storage/queries.mpl` describe the current boundary truthfully, `mesher/storage/writer.mpl` stays free of revived folklore, and Mesher still formats/builds while the existing cross-module `from_json` proof stays green.

## Must-Haves

- `mesher/services/event_processor.mpl` no longer claims cross-module `from_json` is blocked or that caller-side `validate_event(...)` already runs today; any dead import exposed by that cleanup is removed.
- `mesher/storage/queries.mpl` no longer blames module-boundary `from_json` for `extract_event_fields(...)` while preserving the honest PostgreSQL JSONB / ORM expressiveness rationale.
- Slice verification proves the supported cross-module `from_json` path still passes, `mesher/storage/writer.mpl` stays clear of revived folklore, and `meshc fmt --check mesher` plus `meshc build mesher` remain green.

## Observability / Diagnostics

- `Storage.Queries.extract_event_fields(...)` must keep returning a named `Err("extract_event_fields: no result")` surface when the SQL helper yields no row; this slice only rewrites boundary comments and must not hide or rename that failure.
- `Services.EventProcessor.ProcessEvent(...)` must keep propagating query-layer failures unchanged so `Ingestion.Routes.route_to_processor(...)` still turns them into `bad_request_response(reason)` without inventing a second parsing path.
- Comment rewrites must stay aligned with inspectable keep-sites: `mesher/ingestion/routes.mpl` only enforces payload-size checks before `ProcessEvent`, while `mesher/storage/queries.mpl` remains the truthful place to inspect SQL-side extraction / ORM-boundary rationale.
- Redaction constraint: do not add comments or diagnostics that print raw event JSON, API keys, or other payload contents; keep references structural and path-level only.

## Verification

- `cargo test -q -p meshc --test e2e e2e_m032_supported_cross_module_from_json -- --nocapture`
- `cargo run -q -p meshc -- fmt --check mesher`
- `cargo run -q -p meshc -- build mesher`
- `bash -lc '! rg -n "cross-module from_json limitation|from_json limitation per decision \\[88-02\\]|Validation is done by the caller|caller is responsible for JSON parsing and field validation" mesher/services/event_processor.mpl mesher/storage/queries.mpl'`
- `bash -lc '! rg -n "from_json" mesher/storage/writer.mpl'`
- `rg -n "from_json" mesher/types/event.mpl mesher/types/issue.mpl`
- `rg -n "ORM boundary: ORM fragments cannot express CASE/jsonb_array_elements/string_agg|Repo.insert cannot express server-side JSONB extraction" mesher/storage/queries.mpl mesher/storage/writer.mpl`
- `rg -n 'extract_event_fields: no result' mesher/storage/queries.mpl`

## Tasks

- [x] **T01: Rewrite EventProcessor boundary comments to match the live ingestion flow** `est:45m`
  - Why: `mesher/services/event_processor.mpl` still mixes stale `from_json` folklore with a false claim about caller-side validation, so the file currently misstates both why the SQL path exists and what the route layer actually does.
  - Files: `mesher/services/event_processor.mpl`, `mesher/ingestion/routes.mpl`, `mesher/ingestion/validation.mpl`, `mesher/ingestion/fingerprint.mpl`, `mesher/storage/queries.mpl`
  - Do: Rewrite the top-of-file, `route_event(...)`, and service-call comments so they describe the real current flow; remove the unused `compute_fingerprint` import if it stays unused; keep `process_extracted_fields(...)`, the SQL-backed extraction path, and the service API shape unchanged.
  - Verify: `bash -lc '! rg -n "cross-module from_json limitation|from_json limitation per decision \\[88-02\\]|Validation is done by the caller|caller is responsible for JSON parsing and field validation" mesher/services/event_processor.mpl && ! rg -n "^from Ingestion\\.Fingerprint import compute_fingerprint$" mesher/services/event_processor.mpl'`
  - Done when: `event_processor.mpl` only makes truthful claims about the live ingestion path, no dead fingerprint import remains, and no behavioral rewrite was introduced.
- [x] **T02: Retire stale extract_event_fields folklore and close the Mesher proof gate** `est:45m`
  - Why: `mesher/storage/queries.mpl` still blames a solved module-boundary limitation even though the real keep-surface is the PostgreSQL JSONB / ORM boundary; this is also the slice that must prove S02’s repaired path stays clean in the real Mesher tree.
  - Files: `mesher/storage/queries.mpl`, `mesher/storage/writer.mpl`, `mesher/types/event.mpl`, `mesher/types/issue.mpl`, `compiler/meshc/tests/e2e.rs`, `scripts/verify-m032-s01.sh`
  - Do: Rewrite the `extract_event_fields(...)` banner around the real SQL/JSONB rationale without turning it into a Mesh-side parsing redesign; keep `storage/writer` as a read-only guard file; use the existing compiler proof plus Mesher fmt/build and grep checks as the slice closeout gate.
  - Verify: `cargo test -q -p meshc --test e2e e2e_m032_supported_cross_module_from_json -- --nocapture && cargo run -q -p meshc -- fmt --check mesher && cargo run -q -p meshc -- build mesher && bash -lc '! rg -n "cross-module from_json limitation|from_json limitation per decision \\[88-02\\]" mesher/storage/queries.mpl && ! rg -n "from_json" mesher/storage/writer.mpl' && rg -n "from_json" mesher/types/event.mpl mesher/types/issue.mpl && rg -n "ORM boundary: ORM fragments cannot express CASE/jsonb_array_elements/string_agg|Repo.insert cannot express server-side JSONB extraction" mesher/storage/queries.mpl mesher/storage/writer.mpl`
  - Done when: `storage/queries.mpl` keeps only the truthful JSONB/ORM rationale, guard files remain on the intended side of the keep-list, and the full slice verification passes on the real Mesher entrypoint.

## Files Likely Touched

- `mesher/services/event_processor.mpl`
- `mesher/storage/queries.mpl`
