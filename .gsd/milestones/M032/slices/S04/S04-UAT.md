# S04: Module-boundary JSON and workaround convergence — UAT

**Milestone:** M032
**Written:** 2026-03-24

## UAT Type

- UAT mode: artifact-driven
- Why this mode is sufficient: S04 intentionally changed boundary truth and verification surfaces, not runtime behavior. The honest acceptance bar is compiler/build/format proof plus targeted source/grep inspection of the audited Mesher files and diagnostic strings.

## Preconditions

- Run from repo root: `/Users/sn0w/Documents/dev/mesh-lang`
- Rust/Cargo toolchain is available
- `rg` is available
- The working tree includes the S04 edits in `mesher/services/event_processor.mpl` and `mesher/storage/queries.mpl`

## Smoke Test

Run the stale-folklore sweep over the two audited files:

1. `bash -lc '! rg -n "cross-module from_json limitation|from_json limitation per decision \[88-02\]|Validation is done by the caller|caller is responsible for JSON parsing and field validation" mesher/services/event_processor.mpl mesher/storage/queries.mpl'`
2. **Expected:** Exit code 0 with no matches. This confirms the stale module-boundary and caller-validation folklore is gone from the real event path.

## Test Cases

### 1. Cross-module `from_json` support still proves cleanly after the Mesher cleanup

1. Run `cargo test -q -p meshc --test e2e e2e_m032_supported_cross_module_from_json -- --nocapture`
2. Confirm Cargo reports `running 1 test`.
3. **Expected:** The test passes. This proves S04 did not regress the repaired module-boundary `from_json` path from S02.

### 2. Mesher still formats and builds on the cleaned codebase

1. Run `cargo run -q -p meshc -- fmt --check mesher`
2. Run `cargo run -q -p meshc -- build mesher`
3. **Expected:** `fmt --check` exits cleanly with no required rewrites, and the build finishes with `Compiled: mesher/mesher`.

### 3. The audited event-ingestion/storage files now tell the truthful boundary story

1. Open `mesher/services/event_processor.mpl`.
2. Confirm the header and `route_event(...)` comments describe the live flow as: route-layer auth/rate-limit/payload-size checks, raw JSON passed into `Storage.Queries.extract_event_fields(...)`, then discard/upsert/store handling.
3. Confirm there is no `from Ingestion.Fingerprint import compute_fingerprint` import.
4. Open `mesher/storage/queries.mpl` around `extract_event_fields(...)`.
5. **Expected:** The comments explain the PostgreSQL JSONB / fingerprint / ORM boundary and do not blame a cross-module `from_json` limitation.

### 4. Keep-sites and diagnostics remain intact

1. Run `bash -lc '! rg -n "from_json" mesher/storage/writer.mpl'`
2. Run `rg -n "from_json" mesher/types/event.mpl mesher/types/issue.mpl`
3. Run `rg -n "ORM boundary: ORM fragments cannot express CASE/jsonb_array_elements/string_agg|Repo.insert cannot express server-side JSONB extraction" mesher/storage/queries.mpl mesher/storage/writer.mpl`
4. Run `rg -n 'extract_event_fields: no result' mesher/storage/queries.mpl`
5. Run `rg -n 'route_to_processor|bad_request_response|validate_payload_size|process_event' mesher/ingestion/routes.mpl mesher/services/event_processor.mpl`
6. **Expected:**
   - `mesher/storage/writer.mpl` still has no `from_json` references
   - the type files still contain their row-shape `from_json` notes
   - the JSONB/ORM keep-site comments are still present in `queries.mpl` and `writer.mpl`
   - `extract_event_fields(...)` still exposes `Err("extract_event_fields: no result")`
   - `routes.mpl` still validates payload size before `route_to_processor(...)`, and service/query-layer errors still flow to `bad_request_response(reason)`

## Edge Cases

### Guard file stays clean while control files retain their legitimate `from_json` notes

1. Run `bash -lc '! rg -n "from_json" mesher/storage/writer.mpl'`
2. Run `rg -n "from_json" mesher/types/event.mpl mesher/types/issue.mpl`
3. **Expected:** The guard file stays empty, while the type files keep their existing explanatory notes. This catches an over-broad cleanup that would erase truthful row-shape documentation or reintroduce folklore in the wrong file.

### Caller-validation folklore does not creep back into the route/service boundary

1. Run `bash -lc '! rg -n "Validation is done by the caller|caller is responsible for JSON parsing and field validation" mesher/services/event_processor.mpl mesher/ingestion/routes.mpl'`
2. Inspect `mesher/ingestion/routes.mpl` around `process_event_body(...)`.
3. **Expected:** No stale caller-validation claims exist, and the only pre-service validation in this path is payload-size enforcement before forwarding raw JSON to `EventProcessor.process_event(...)`.

## Failure Signals

- Any match for the stale `from_json` / caller-validation folklore in `event_processor.mpl` or `queries.mpl`
- `e2e_m032_supported_cross_module_from_json` fails or reports `running 0 tests`
- `meshc fmt --check mesher` wants rewrites or `meshc build mesher` fails
- `mesher/storage/writer.mpl` gains a `from_json` reference
- The `extract_event_fields: no result` string disappears or the `routes.mpl` error path stops routing failures through `bad_request_response(reason)`

## Requirements Proved By This UAT

- R035 — the audited Mesher limitation comments now reflect current verified reality for the module-boundary `from_json` family
- R013 — the repaired cross-module `from_json` path still works on the real Mesher codebase after the cleanup

## Not Proven By This UAT

- A Mesh-side replacement for the PostgreSQL JSONB fingerprint/extraction path
- The full retained-limit ledger for all remaining Mesher workaround comments; that remains S05 closeout work
- Any new ORM or migration capability beyond the existing truthful keep-sites

## Notes for Tester

- This slice is a truth/convergence cleanup, not a product-path redesign. If a proposed “improvement” deletes the SQL extraction path, treat that as out of scope unless there is new parity proof.
- Ignore the failed closeout artifact from the prior attempt; this file is the current UAT.
