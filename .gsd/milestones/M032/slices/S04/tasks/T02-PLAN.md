---
estimated_steps: 5
estimated_files: 7
skills_used:
  - debug-like-expert
---

# T02: Retire stale extract_event_fields folklore and close the Mesher proof gate

**Slice:** S04 — Module-boundary JSON and workaround convergence
**Milestone:** M032

## Description

Clean up the last stale `from_json` rationale in `mesher/storage/queries.mpl` and then prove the slice on the real Mesher surface. The `extract_event_fields(...)` comment should explain the still-real PostgreSQL JSONB / ORM expressiveness boundary, not a solved module-boundary compiler limit. Treat `mesher/storage/writer.mpl` as a guard file that must stay free of revived folklore, and keep the type-file `from_json` notes intact because they are row-shape notes, not limitation claims.

## Steps

1. Re-read `mesher/storage/queries.mpl` with `mesher/storage/writer.mpl`, `mesher/types/event.mpl`, and `mesher/types/issue.mpl` so the rewrite preserves the real keep-surface and does not disturb the non-folklore notes.
2. Rewrite the `extract_event_fields(...)` banner in `mesher/storage/queries.mpl` so it attributes the SQL path to the JSONB/fingerprint/ORM boundary instead of cross-module `from_json`.
3. Leave `mesher/storage/writer.mpl` untouched unless verification proves it somehow drifted; do not convert the event flow to Mesh-side `EventPayload.from_json(...)` parsing in this slice.
4. Reuse the existing cross-module `from_json` proof plus `meshc fmt --check mesher`, `meshc build mesher`, and grep-based keep-list checks as the authoritative slice closeout.
5. If any closeout check fails, fix the truthfulness issue it exposed before finishing rather than weakening the gate.

## Must-Haves

- [ ] `mesher/storage/queries.mpl` no longer blames a module-boundary `from_json` limit for `extract_event_fields(...)`.
- [ ] The honest raw-SQL boundary comments remain visible in `mesher/storage/queries.mpl` and `mesher/storage/writer.mpl`.
- [ ] `mesher/storage/writer.mpl` stays free of revived `from_json` folklore, while `mesher/types/event.mpl` and `mesher/types/issue.mpl` keep their row-shape `from_json` notes.
- [ ] The existing compiler proof and Mesher fmt/build gates pass after the cleanup.

## Verification

- `cargo test -q -p meshc --test e2e e2e_m032_supported_cross_module_from_json -- --nocapture`
- `cargo run -q -p meshc -- fmt --check mesher`
- `cargo run -q -p meshc -- build mesher`
- `bash -lc '! rg -n "cross-module from_json limitation|from_json limitation per decision \\[88-02\\]" mesher/storage/queries.mpl && ! rg -n "from_json" mesher/storage/writer.mpl'`
- `rg -n "from_json" mesher/types/event.mpl mesher/types/issue.mpl`
- `rg -n "ORM boundary: ORM fragments cannot express CASE/jsonb_array_elements/string_agg|Repo.insert cannot express server-side JSONB extraction" mesher/storage/queries.mpl mesher/storage/writer.mpl`

## Inputs

- `mesher/storage/queries.mpl` — stale `extract_event_fields(...)` rationale to rewrite.
- `mesher/storage/writer.mpl` — guard file that must remain free of revived `from_json` folklore and keep its server-side JSONB boundary note.
- `mesher/types/event.mpl` — control file whose `from_json` note is about row shape, not a Mesh limitation.
- `mesher/types/issue.mpl` — control file whose `from_json` note is about row shape, not a Mesh limitation.
- `mesher/services/event_processor.mpl` — T01 output that should remain aligned with the same truthful boundary story.
- `compiler/meshc/tests/e2e.rs` — source of the existing `e2e_m032_supported_cross_module_from_json` proof reused for closeout.
- `scripts/verify-m032-s01.sh` — reference replay surface showing `xmod_from_json` is already a supported path.

## Expected Output

- `mesher/storage/queries.mpl` — truthful `extract_event_fields(...)` boundary comment aligned with the remaining Mesher keep-sites.

## Observability Impact

- `Storage.Queries.extract_event_fields(...)` must keep the named `Err("extract_event_fields: no result")` failure surface unchanged; the closeout grep against that string remains the fastest diagnostic proof that this cleanup did not hide or rename the query-layer failure.
- `Services.EventProcessor.ProcessEvent(...)` must keep forwarding `extract_event_fields(...)` errors unchanged so `mesher/ingestion/routes.mpl` still turns them into `bad_request_response(reason)` without adding a second JSON parsing path.
- Future agents should inspect the boundary story in three places: `mesher/ingestion/routes.mpl` for pre-service payload-size checks only, `mesher/storage/queries.mpl` for SQL-side extraction/fingerprint rationale, and `mesher/storage/writer.mpl` for the matching insert-side JSONB boundary note.
- Redaction rule: comments and diagnostics in this slice stay structural only. Do not add raw event JSON, API keys, or payload contents to comments, errors, or verification output.
