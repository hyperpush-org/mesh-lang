---
id: S01
parent: M033
milestone: M033
provides:
  - A portable `Expr` / `Query.select_exprs` / expression-aware `Repo` contract for structured SELECT, UPDATE, and ON CONFLICT work with stable placeholder ordering.
  - Rewritten Mesher write paths for issue upsert, assign/unassign, API-key revoke, alert acknowledge/resolve, and settings updates on the real Postgres-backed storage path.
  - A live Postgres-backed acceptance harness and serialized verify script that prove first-event ingest, truthful rate limiting, mutation-side DB state changes, repeated issue upsert, and the raw keep-list boundary.
  - A repaired low-volume StorageWriter persistence path that downstream slices can rely on when they add PG JSONB/search helpers and harder read-side rewrites.
requires:
  []
affects:
  - S02
  - S03
  - S04
  - S05
key_files:
  - compiler/mesh-rt/src/db/query.rs
  - compiler/mesh-rt/src/db/repo.rs
  - compiler/mesh-typeck/src/infer.rs
  - compiler/mesh-codegen/src/mir/lower.rs
  - compiler/meshc/tests/e2e_m033_s01.rs
  - mesher/storage/queries.mpl
  - mesher/services/rate_limiter.mpl
  - mesher/ingestion/pipeline.mpl
  - mesher/services/event_processor.mpl
  - mesher/ingestion/routes.mpl
  - mesher/services/writer.mpl
  - mesher/storage/writer.mpl
  - scripts/verify-m033-s01.sh
key_decisions:
  - Use `Expr.label(...)` as the Mesh-callable alias surface for expression-valued SELECT items until the `Expr.alias(...)` parser keyword collision is repaired.
  - For Mesh services on the live Mesher path, compute branch-local values first and return one final `(state, reply)` tuple instead of branching directly between different tuple shapes.
  - Keep Mesher’s 60-second / 1000-event limiter defaults, but read optional env overrides and always spawn the reset ticker so live threshold proofs exercise the real fixed-window limiter honestly.
  - Make async writer batches carry `project_id|||issue_id|||fingerprint|||event_json`, start writers through `start_writer(...)`, and return non-integer success markers from flush helpers when counts are not needed.
patterns_established:
  - For Mesh services, compute branch-local values first and return one final `(state, reply)` tuple; branching directly between different tuples is still a live miscompile/crash risk.
  - Use the neutral `Expr` builder plus expression-aware `Query`/`Repo` entrypoints for portable SELECT/UPDATE/UPSERT work, and keep PostgreSQL-only behavior behind explicit keep-sites instead of leaking it into the baseline API.
  - On the live Mesher path, assert database rows directly rather than trusting HTTP status alone; route success was not sufficient to catch the ingest and StorageWriter failures this slice exposed.
  - Buffered async writer payloads must carry every database-critical identifier they need at flush time; relying on service-local labels or inferred context produced false project IDs on the real write path.
observability_surfaces:
  - Named compiler/runtime acceptance tests in `compiler/meshc/tests/e2e_m033_s01.rs` (`e2e_m033_expr_*`, `e2e_m033_mesher_ingest_first_event`, `e2e_m033_mesher_mutations`, `e2e_m033_mesher_issue_upsert`).
  - `scripts/verify-m033-s01.sh`, which reruns the full slice e2e suite, Mesher fmt/build checks, and the explicit raw keep-list sweep.
  - Mesher stdout/stderr startup banners plus rate-limiter, health-check, and load-monitor logs captured by the live harness when diagnosing ingest/writer failures.
  - Direct Postgres row assertions over `issues`, `alerts`, `projects`, `api_keys`, and `events`, which remained the authoritative truth surface for S01 behavior.
drill_down_paths:
  - .gsd/milestones/M033/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M033/slices/S01/tasks/T02-SUMMARY.md
  - .gsd/milestones/M033/slices/S01/tasks/T03-SUMMARY.md
  - .gsd/milestones/M033/slices/S01/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-25T16:12:59.856Z
blocker_discovered: false
---

# S01: Neutral expression core on real write paths

**Shipped a neutral expression-aware Query/Repo core and re-proved Mesher’s real Postgres-backed write paths — including repeated issue upsert, direct mutations, first-event ingest, and low-volume event persistence — without reintroducing fake portability.**

## What Happened

S01 finished the neutral expression core and proved it on the real Mesher write path instead of stopping at compiler-only plumbing. T01 added the portable `Expr`/`Query`/`Repo` surface end to end: Mesh code can now build structured column/literal/NULL/function/CASE/COALESCE expressions for expression-valued SELECT, UPDATE, and ON CONFLICT work, with compiler/runtime wiring and stable placeholder ordering proven in `compiler/meshc/tests/e2e_m033_s01.rs`.

The middle of the slice shifted from pure feature work into blocker retirement on the live Mesher path. T02 confirmed the portable mutation rewrites were already in place for API-key revoke, issue assign/unassign, alert acknowledge/resolve, and settings updates, then isolated the clean-start ingest blocker. T03 fixed that ingress path by rewriting `RateLimiter.CheckLimit` and `EventProcessor.ProcessEvent` to the stable single-return service pattern, restoring the rate-window ticker, and adding env-driven threshold controls for truthful live limit proofs. T04 then repaired the next live-route failure by removing brittle `Ok(n)` bindings from HTTP handlers that did not actually need the integer count and by tightening the direct UUID-expression proof so route/lowering issues could be distinguished from real expression-runtime regressions.

The final closeout work retired the last remaining blocker the task summaries had exposed: low-volume repeated ingest updated the `issues` row correctly but did not persist `events` rows. The root cause was in the async writer path, not the upsert core. `StorageWriter.start(...)` was being used without the timer flush actor, so small batches never flushed, and the buffered entry format also assumed the writer state's `project_id` label instead of carrying the real project UUID. The slice closeout added `start_writer(...)` to spawn the flush ticker, made the enriched writer entry carry `project_id|||issue_id|||fingerprint|||event_json`, and changed the writer/storage flush helpers to return non-integer success markers so the background path no longer hit the same boxed-int lowering trap already seen in HTTP handlers. After that, repeated event ingest created or updated the same issue, incremented `event_count`, advanced `last_seen`, reopened resolved issues through the structured upsert path, and persisted the expected `events` rows.

By slice close, Mesher's S01-owned write families now use the neutral expression surface on the real Postgres-backed path, the raw-write keep-list is short and explicit, and downstream slices can build PG extras and harder read-side rewrites on top of a proven neutral serializer/runtime contract instead of recurring raw SQL.

## Verification

Verified the slice at three levels. Focused proof commands passed: `cargo test -p meshc --test e2e_m033_s01 expr_ -- --nocapture`, `cargo test -p meshc --test e2e_m033_s01 mesher_ingest_first_event -- --nocapture`, `cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture`, and `cargo test -p meshc --test e2e_m033_s01 mesher_issue_upsert -- --nocapture`. Runtime/build hygiene also passed with `cargo run -q -p meshc -- build mesher` and `cargo run -q -p meshc -- fmt --check mesher`. Final slice closure passed through `bash scripts/verify-m033-s01.sh`, which reruns the full `e2e_m033_s01` suite, Mesher fmt/build, and the explicit raw keep-list sweep. During debugging, the live harness and direct Postgres row checks were used as the authoritative signals for `assigned_to`, `acknowledged_at`, `resolved_at`, `retention_days`, `sample_rate`, `revoked_at`, `event_count`, `last_seen`, and persisted `events` rows.

## Requirements Advanced

- R036 — Established the neutral expression baseline through `Expr`, `Query.select_exprs`, and expression-aware `Repo` entrypoints, while keeping PostgreSQL-only JSONB/crypto/event-write behavior as explicit keep-sites instead of pretending it is portable.
- R040 — Kept the S01 baseline API vendor-neutral and pushed PostgreSQL-specific behavior into explicit seams, preserving a credible later path for SQLite-specific extras without backing out a PG-only abstraction.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Several S01-owned Mesher mutation rewrites were already present when slice execution began, so T02 and part of T04 became verification-and-repro work rather than first-time implementation. Closing the slice also required repairing the async StorageWriter path that only surfaced after the ingest and route-handler blockers were removed, and hardening `scripts/verify-m033-s01.sh` so the raw keep-list sweep ignores comment text and only evaluates executable code.

## Known Limitations

S01 intentionally leaves the PostgreSQL-shaped JSONB/crypto/search/event-write sites explicit: `create_alert_rule`, `fire_alert`, and `insert_event` remain raw keep-sites for S02. Mesh source still needs `Expr.label(...)` rather than `Expr.alias(...)`, and route/background code should still avoid binding unused `Int ! String` success payloads until the broader lowering/runtime bug is fixed.

## Follow-ups

S02 should replace the remaining explicit raw PG keep-sites (`create_alert_rule`, `fire_alert`, and `insert_event`) with honest PostgreSQL helper surfaces where that remains clearer than raw SQL. Future compiler/runtime work should also repair the `Expr.alias(...)` keyword collision and the broader `Int ! String` success-payload lowering bug so route/service code can bind integer success values safely again.

## Files Created/Modified

- `compiler/mesh-rt/src/db/query.rs` — Added `Query.select_exprs` and SELECT-side expression plumbing for portable expression-valued query work.
- `compiler/mesh-rt/src/db/repo.rs` — Implemented expression-aware select/update/upsert SQL rendering, placeholder ordering, and repo runtime support.
- `compiler/mesh-rt/src/lib.rs` — Exported the new expression/query runtime intrinsics.
- `compiler/mesh-typeck/src/infer.rs` — Typechecked the new expression-aware Query/Repo calls.
- `compiler/mesh-codegen/src/mir/lower.rs` — Lowered new expression/query calls into MIR and runtime intrinsic invocations.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — Wired codegen intrinsics for the S01 expression surface.
- `compiler/meshc/tests/e2e_m033_s01.rs` — Added live Postgres-backed expression, ingest, mutation, and issue-upsert acceptance coverage.
- `mesher/storage/queries.mpl` — Rewrote portable Mesher write families around the neutral expression core, including issue upsert and direct mutation paths.
- `mesher/services/rate_limiter.mpl` — Repaired rate-limit state handling, restored the reset ticker path, and added env-configurable live thresholds while keeping defaults stable.
- `mesher/ingestion/pipeline.mpl` — Started the configured rate limiter and writer correctly in the live ingestion pipeline.
- `mesher/services/event_processor.mpl` — Stabilized the processor service return path and made buffered writer entries carry the real project UUID.
- `mesher/ingestion/routes.mpl` — Kept the live event route on the repaired ingestion path used by the acceptance harness.
- `mesher/api/team.mpl` — Stopped binding unused integer success payloads in issue/API-key mutation handlers so live routes no longer crash after successful writes.
- `mesher/api/alerts.mpl` — Stopped binding unused integer success payloads in alert mutation handlers.
- `mesher/api/settings.mpl` — Stopped binding unused integer success payloads in settings update handlers.
- `mesher/services/writer.mpl` — Added a `start_writer(...)` wrapper that spawns the timer flush ticker and kept the writer retry path crash-free under low-volume ingest.
- `mesher/storage/writer.mpl` — Made batch persistence use project-aware buffered entries and string success markers while preserving the raw PG JSONB keep-site.
- `scripts/verify-m033-s01.sh` — Serialized the slice acceptance bundle and enforced the comment-aware raw keep-list sweep.
