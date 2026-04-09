---
id: S04
parent: M032
milestone: M032
provides:
  - Truthful Mesher event-ingestion/storage boundary comments with stale module-boundary `from_json` folklore removed
  - Revalidated cross-module `from_json` proof plus green Mesher fmt/build on the cleaned codebase
requires:
  - slice: S01
    provides: audited stale-vs-real workaround matrix and the module-boundary `from_json` repro classification
  - slice: S02
    provides: repaired cross-module `from_json` behavior and the compiler proof reused as the slice closeout gate
affects:
  - S05
  - M033/S01
key_files:
  - mesher/services/event_processor.mpl
  - mesher/storage/queries.mpl
  - .gsd/milestones/M032/slices/S04/tasks/T02-VERIFY.json
key_decisions:
  - Cross-module `from_json` support is already proven, but Mesher's event path should stay on the SQL-side JSONB extract/write boundary until parity evidence justifies a Mesh-side rewrite.
patterns_established:
  - Retire stale workaround comments without deleting real keep-sites; keep the raw-SQL rationale anchored in `mesher/storage/queries.mpl` and `mesher/storage/writer.mpl` when the pressure is still PostgreSQL JSONB/ORM expressiveness.
  - Preserve `from_json` notes in type files when they describe row-shape decoding rather than a language limitation claim.
observability_surfaces:
  - `Storage.Queries.extract_event_fields(...) -> Err("extract_event_fields: no result")`
  - `Services.EventProcessor.process_event(...) -> Ingestion.Routes.route_to_processor(...) -> bad_request_response(reason)`
  - grep gates for stale `from_json` folklore and the retained JSONB/ORM boundary comments
drill_down_paths:
  - .gsd/milestones/M032/slices/S04/tasks/T01-SUMMARY.md
  - .gsd/milestones/M032/slices/S04/tasks/T02-SUMMARY.md
duration: 33m
verification_result: passed
completed_at: 2026-03-24 19:16:59 EDT
---

# S04: Module-boundary JSON and workaround convergence

**Retired the last stale module-boundary `from_json` folklore in Mesher's event ingestion/storage path, while keeping the real PostgreSQL JSONB/ORM boundary explicit and revalidated.**

## What Happened

S04 closed the last stale `from_json` workaround family that was still misdescribing Mesher's real event path.

T01 cleaned `mesher/services/event_processor.mpl`. The service header and route comments now match the live flow: `mesher/ingestion/routes.mpl` does auth/rate-limit/payload-size checks, `EventProcessor` forwards raw JSON to `Storage.Queries.extract_event_fields(...)`, and the service then runs discard checks, issue upsert, and `StorageWriter` forwarding. The dead `compute_fingerprint` import was removed, and no runtime behavior or API shape changed.

T02 cleaned `mesher/storage/queries.mpl`. The `extract_event_fields(...)` banner no longer blames a solved cross-module `from_json` limitation. It now states the real keep-surface: the fingerprint fallback chain depends on PostgreSQL JSONB operators plus `CASE` / `jsonb_array_elements` / `string_agg`, and that extraction intentionally stays on the same raw-SQL side of the boundary as `Storage.Writer.insert_event(...)`. `mesher/storage/writer.mpl` remained the guard file, while `mesher/types/event.mpl` and `mesher/types/issue.mpl` kept their row-shape `from_json` notes.

During closeout, a stale quote in `.gsd/milestones/M032/slices/S04/tasks/T02-VERIFY.json` was causing a shell-parse failure even though the code and slice gate were already green. That artifact was rewritten to match the real passing state so downstream verification stops failing on bad quoting instead of code truth.

## Verification

Verified against the slice plan's real gates:

- `cargo test -q -p meshc --test e2e e2e_m032_supported_cross_module_from_json -- --nocapture`
- `cargo run -q -p meshc -- fmt --check mesher`
- `cargo run -q -p meshc -- build mesher`
- negative grep over `mesher/services/event_processor.mpl` and `mesher/storage/queries.mpl` for stale `from_json` / caller-validation folklore
- negative grep proving `mesher/storage/writer.mpl` stays free of `from_json`
- positive grep proving the intended `from_json` notes remain in `mesher/types/event.mpl` and `mesher/types/issue.mpl`
- positive grep proving the JSONB/ORM keep-sites remain in `mesher/storage/queries.mpl` and `mesher/storage/writer.mpl`
- positive grep proving `extract_event_fields: no result` still exists
- path inspection of `mesher/ingestion/routes.mpl` confirming payload-size validation still happens before `route_to_processor(...)`, and query-layer failures still flow unchanged to `bad_request_response(reason)`

All checks passed.

## Requirements Advanced

- R035 — Retired the stale module-boundary `from_json` folklore in Mesher's event ingestion/storage comments while preserving the real JSONB/ORM keep-sites that still need an honest retained-limit ledger in S05.
- R013 — Replayed the repaired cross-module `from_json` proof on the real Mesher codebase after cleanup, confirming the S02 repair remains the supported path rather than a one-off repro fix.

## Requirements Validated

- none — S04 advanced R035 and reinforced existing R013 proof, but the integrated retained-limit ledger still belongs to S05.

## New Requirements Surfaced

- none

## Requirements Invalidated or Re-scoped

- none

## Deviations

Added the missing observability sections to the slice/task plan artifacts during execution, and repaired the malformed quote in `T02-VERIFY.json` during closeout so the machine-readable verification artifact matched the already-green code state.

## Known Limitations

Mesher's event ingestion/storage path still intentionally uses SQL-side extraction and raw-SQL insertion for this workload. Cross-module `from_json` support is fixed, but S04 did **not** prove a Mesh-side replacement for the PostgreSQL JSONB fingerprint chain, so removing that boundary would still be speculative.

## Follow-ups

- S05 should rerun the integrated Mesher proof on the cleaned codebase and publish the short retained-limit ledger.
- M033 should treat `mesher/storage/queries.mpl` and `mesher/storage/writer.mpl` as honest PostgreSQL JSONB/ORM pressure sites, not as leftover folklore targets.

## Files Created/Modified

- `mesher/services/event_processor.mpl` — rewrote stale ingestion-boundary comments and removed the dead fingerprint import.
- `mesher/storage/queries.mpl` — rewrote the `extract_event_fields(...)` banner to the real PostgreSQL JSONB/ORM rationale.
- `.gsd/milestones/M032/slices/S04/tasks/T02-VERIFY.json` — repaired the malformed grep command and rewrote the task verification artifact to the passing state.
- `.gsd/PROJECT.md` — refreshed current project state to include S04.
- `.gsd/KNOWLEDGE.md` — recorded the event-path keep-surface rule for future cleanup and ORM work.
- `.gsd/milestones/M032/M032-ROADMAP.md` — marked S04 complete.

## Forward Intelligence

### What the next slice should know
- The stale `from_json` wording is gone from the audited event path. S05 should not spend time rediscovering that family; the remaining work is integrated proof plus a truthful keep-list.
- The real boundary is not module imports anymore. It is the PostgreSQL JSONB/fingerprint/ORM boundary shared by `extract_event_fields(...)` and `insert_event(...)`.

### What's fragile
- `mesher/storage/queries.mpl` / `mesher/storage/writer.mpl` — they are easy to misread as stale workaround residue now that cross-module `from_json` is proven, but deleting or rewriting them without parity coverage would change product behavior under the guise of cleanup.

### Authoritative diagnostics
- `cargo test -q -p meshc --test e2e e2e_m032_supported_cross_module_from_json -- --nocapture` — proves the repaired module-boundary `from_json` path still works.
- `rg -n 'extract_event_fields: no result' mesher/storage/queries.mpl` and `rg -n 'route_to_processor|bad_request_response|validate_payload_size' mesher/ingestion/routes.mpl` — fastest proof that the named failure surface and propagation path stayed intact.
- `rg -n 'ORM boundary: ORM fragments cannot express CASE/jsonb_array_elements/string_agg|Repo.insert cannot express server-side JSONB extraction' mesher/storage/queries.mpl mesher/storage/writer.mpl` — trustworthy keep-site check for the real JSONB/ORM boundary.

### What assumptions changed
- "Mesher still needs a cross-module `from_json` workaround here" — false; that limitation is already repaired and regression-covered.
- "The caller already validates/parses the payload before `ProcessEvent(...)`" — false; the route layer still only enforces payload-size before passing raw JSON into the SQL-side extraction path.
