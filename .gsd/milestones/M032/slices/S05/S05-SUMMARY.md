---
id: S05
parent: M032
milestone: M032
provides:
  - Final M032 closeout bundle tying named Mesher proof commands, replay logs, and the retained-limit ledger into one current evidence surface
  - Supported-now versus retained-limit ledger that keeps `xmod_identity` visible and hands ORM / migration pressure to explicit M033 families
requires:
  - slice: S02
    provides: repaired inferred-export lowering with the `xmod_identity` cross-module success path dogfooded back into mesher
  - slice: S03
    provides: cleaned request, handler, nested-control-flow folklore with only the real Mesh keep-sites left in mesher
  - slice: S04
    provides: truthful module-boundary `from_json` comments and the preserved PostgreSQL JSONB / ORM keep-sites for M033 handoff
affects:
  - M033/S01
  - M033/S02
  - M033/S04
  - M033/S05
key_files:
  - .gsd/milestones/M032/slices/S05/S05-SUMMARY.md
  - .gsd/milestones/M032/slices/S05/S05-UAT.md
  - .gsd/milestones/M032/M032-ROADMAP.md
  - .gsd/PROJECT.md
  - .gsd/REQUIREMENTS.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - M032 closes with a short retained-limit ledger, not a fake “all limitations are gone” claim: supported-now paths stay backed by named proof, real Mesh keep-sites stay named by Mesher file, and the broader ORM boundary / migration pressure is grouped into M033 follow-on families.
patterns_established:
  - Close a comment-truth milestone with three linked surfaces: named regression proof, a retained-limit ledger keyed to real files, and grep/build replay that keeps the wording honest.
  - Keep supported row-shape `from_json` notes in `mesher/types/event.mpl` and `mesher/types/issue.mpl`; only comments that claim a Mesh limitation belong in the retained-limit ledger.
observability_surfaces:
  - bash scripts/verify-m032-s01.sh
  - cargo test -q -p meshc --test e2e m032_inferred -- --nocapture
  - cargo test -q -p meshc --test e2e e2e_m032_supported_nested_wrapper_list_from_json -- --nocapture
  - cargo test -q -p meshc --test e2e e2e_m032_supported_inline_writer_cast_body -- --nocapture
  - cargo test -q -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture
  - cargo run -q -p meshc -- fmt --check mesher
  - cargo run -q -p meshc -- build mesher
  - retained keep-site grep over mesher/
drill_down_paths:
  - .gsd/milestones/M032/slices/S05/tasks/T01-SUMMARY.md
  - .gsd/milestones/M032/slices/S05/tasks/T02-SUMMARY.md
duration: 62m
verification_result: passed
completed_at: 2026-03-24 19:58:01 EDT
---

# S05: Integrated mesher proof and retained-limit ledger

**Replayed the full Mesher proof matrix, kept `xmod_identity` and the other repaired paths visible as supported-now behavior, and closed M032 with a file-backed retained-limit ledger plus explicit M033 handoff families.**

## What Happened

S05 closed M032 by turning the repaired proof surface into one current bundle instead of leaving the truth scattered across tasks and comments.

T01 added the last two named regressions that the slice still needed: `e2e_m032_supported_nested_wrapper_list_from_json` proves Mesh can decode a nested wrapper `List < BulkEvent >` payload, and `e2e_m032_supported_inline_writer_cast_body` proves the writer-style cast body can keep its append/capacity logic inline. With those proofs in place, the stale overstatements in `mesher/ingestion/routes.mpl` and `mesher/services/writer.mpl` were narrowed to the real remaining limits.

T02 then replayed the integrated proof matrix and wrote the closeout ledger. The supported-now landmarks are now explicit and current:

- `m032_inferred` keeps the repaired cross-module inferred-export path visible through `.tmp/m032-s01/xmod_identity`
- `e2e_m032_supported_request_query` keeps direct `Request.query(...)` dogfood in bounds
- `e2e_m032_supported_cross_module_from_json` keeps module-boundary `from_json` support visible
- `e2e_m032_supported_service_call_case` and `e2e_m032_supported_cast_if_else` keep the handler/control-flow cleanup grounded in named proof
- `e2e_m032_supported_nested_wrapper_list_from_json` and `e2e_m032_supported_inline_writer_cast_body` freeze the last two S05 supported paths

The retained Mesh keep-sites are now called out as a short, real ledger instead of folklore:

- `mesher/ingestion/routes.mpl` — `HTTP routing does not support closures`; the authoritative control is the live `e2e_m032_route_closure_runtime_failure` proof
- `mesher/services/stream_manager.mpl` — nested `&&` still needs `both_match(...)`
- `mesher/services/writer.mpl` and `mesher/ingestion/pipeline.mpl` — `Timer.send_after` still delivers raw bytes instead of a service-dispatchable cast payload
- `mesher/services/event_processor.mpl`, `mesher/ingestion/fingerprint.mpl`, `mesher/services/retention.mpl`, and `mesher/api/team.mpl` — parser-bound single-expression case arm extraction keep-sites remain truthful and intentionally explicit

The wider data-layer pressure is no longer treated as a pile of one-off Mesher comments. It is grouped into the M033 follow-on families that actually match the codebase:

- `mesher/storage/queries.mpl` and `mesher/storage/writer.mpl` — recurring `ORM boundary` cases around JSONB extraction, expression-heavy updates, subqueries, server-side functions, search ranking, and computed insert/update paths
- `mesher/migrations/20260216120000_create_initial_schema.mpl` — the `PARTITION BY` migration gap remains an honest migration-surface limitation
- `mesher/types/event.mpl` and `mesher/types/issue.mpl` keep their `from_json` notes because they describe row-shape decoding, not a Mesh limitation claim

## Verification

S05 was closed against the full integrated matrix, not just the newly written docs:

- `bash scripts/verify-m032-s01.sh`
- `cargo test -q -p meshc --test e2e e2e_m032_supported_nested_wrapper_list_from_json -- --nocapture`
- `cargo test -q -p meshc --test e2e e2e_m032_supported_inline_writer_cast_body -- --nocapture`
- `cargo test -q -p meshc --test e2e m032_inferred -- --nocapture`
- `cargo test -q -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture`
- `cargo run -q -p meshc -- fmt --check mesher`
- `cargo run -q -p meshc -- build mesher`
- negative grep over `mesher/` for stale disproven limitation phrases
- positive grep over the retained keep-sites (`HTTP routing does not support closures`, nested `&&`, `Timer.send_after`, case arm extraction, `ORM boundary`, `PARTITION BY`, and row-shape `from_json` notes)
- artifact checks proving `S05-SUMMARY.md`, `S05-UAT.md`, and the completed roadmap entry exist and carry the required closeout strings

All checks passed.

## Requirements Advanced

- R013 — The final closeout replay kept the repaired inferred-export path visible through `xmod_identity`, `m032_inferred`, the integrated replay script, and Mesher fmt/build, so the fix remains a live dogfood surface instead of a one-off milestone note.

## Requirements Validated

- R010 — Closed with a current evidence bundle: the M028 native deploy proof remains the deployment anchor, and M032 now adds a truthful Mesher closeout proving specific backend-development wins instead of rhetoric.
- R011 — Validated by the full M032 wave: each language/runtime/tooling change came from real Mesher friction, and S05 closes that loop with a file-backed retained-limit ledger rather than hand-wavy backlog folklore.
- R035 — Validated by the named `e2e_m032_*` proofs, the integrated replay script, and the final grep sweeps proving Mesher limitation comments now match current verified reality.

## New Requirements Surfaced

- none

## Requirements Invalidated or Re-scoped

- none

## Deviations

None. S05 stayed inside the planned closeout scope: replay the proof matrix, publish the retained-limit ledger, and refresh the project-level GSD state.

## Known Limitations

The remaining limits are short, real, and intentionally retained:

- Mesh keep-sites still in play: route closures in `mesher/ingestion/routes.mpl`, nested `&&` in `mesher/services/stream_manager.mpl`, `Timer.send_after` cast delivery in `mesher/services/writer.mpl` / `mesher/ingestion/pipeline.mpl`, and parser-bound case-arm extraction in `mesher/services/event_processor.mpl`, `mesher/ingestion/fingerprint.mpl`, `mesher/services/retention.mpl`, and `mesher/api/team.mpl`
- M033 data-layer follow-ons still in play: the `ORM boundary` families in `mesher/storage/queries.mpl` and `mesher/storage/writer.mpl`, plus the migration-surface `PARTITION BY` gap in `mesher/migrations/20260216120000_create_initial_schema.mpl`
- S05 does **not** prove those remaining limits are solved; it proves they are the honest current keep-list

## Follow-ups

- M033/S01 should shape the neutral core plus explicit PG extras around the recurring `ORM boundary` families instead of comment-by-comment cleanup.
- M033/S02 and M033/S03 should retire the highest-value raw-SQL keep-sites in `mesher/storage/queries.mpl` and `mesher/storage/writer.mpl` without regressing Mesher behavior.
- M033/S04 should cover the migration/DDL family anchored by the retained `PARTITION BY` note.

## Files Created/Modified

- `.gsd/milestones/M032/slices/S05/S05-SUMMARY.md` — published the final supported-now versus retained-limit ledger for M032.
- `.gsd/milestones/M032/slices/S05/S05-UAT.md` — added the artifact-driven closeout acceptance script keyed to the final proof matrix.
- `.gsd/milestones/M032/M032-ROADMAP.md` — marked S05 complete and updated milestone requirement coverage to the closed state.
- `.gsd/PROJECT.md` — refreshed the current project state from “remaining M032 closeout” to “M032 complete, M033 next.”
- `.gsd/REQUIREMENTS.md` — closed R010 and refreshed R011 / R013 / R035 to the final proof state.
- `.gsd/KNOWLEDGE.md` — recorded the durable three-bucket closeout rule for future M033 work.

## Forward Intelligence

### What the next slice should know
- Start from `bash scripts/verify-m032-s01.sh`, `cargo test -q -p meshc --test e2e m032_inferred -- --nocapture`, and this ledger. That is the shortest trustworthy path from a failure signal to the affected Mesher family.
- `xmod_identity` is part of the supported-now surface now, not a historical repro to rediscover.

### What's fragile
- The retained keep-sites are easy to over-clean because nearby stale folklore is gone. If a future edit removes a route-closure, nested-`&&`, `Timer.send_after`, or case-arm comment, it needs new proof, not optimism.
- `mesher/storage/queries.mpl` / `mesher/storage/writer.mpl` are easy to misread as “leftover hacks.” They are the honest M033 pressure map.

### Authoritative diagnostics
- `bash scripts/verify-m032-s01.sh` — fastest integrated replay, with `.tmp/m032-s01/verify/*.log` as the first stop when the bundle drifts.
- `cargo test -q -p meshc --test e2e m032_inferred -- --nocapture` — authoritative `xmod_identity` / inferred-export proof.
- `cargo test -q -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture` plus the retained grep sweep — authoritative retained-limit proof for the Mesh keep-sites.

### What assumptions changed
- “Mesher still mostly carries broad limitation folklore” — false; after S05 the remaining notes are short, named, and tied to proof.
- “The next step after M032 is more comment cleanup” — false; the remaining pressure is family-level M033 ORM / migration work, not another folklore sweep.
