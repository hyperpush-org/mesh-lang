---
id: T02
parent: S02
milestone: M054
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/e2e_m054_s02.rs", "compiler/meshc/tests/e2e_m047_s07.rs", "compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs", ".gsd/milestones/M054/slices/S02/tasks/T02-SUMMARY.md"]
key_decisions: ["Reuse the shared raw HTTP response helper in `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` so both the low-level clustered-route rail and the serious starter fail closed on the same header-parsing rules.", "Keep `compiler/meshc/tests/e2e_m054_s01.rs` intact and land the serious-starter direct-correlation proof as a separate `e2e_m054_s02.rs` target with its own retained bundle."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran `cargo test -p meshc --test e2e_m054_s02 -- --nocapture` against a disposable local Docker Postgres because this shell did not already have `DATABASE_URL` populated; the full serious-starter rail passed, including the staged two-node runtime proof and the new negative tests. Re-ran `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` to verify the shared header parser did not regress the low-level clustered-route rail. Finally, inspected the newest `.tmp/m054-s02/...` bundle directly and confirmed the extracted request key matches both retained continuity records and both retained diagnostics entry files."
completed_at: 2026-04-06T15:25:47.192Z
blocker_discovered: false
---

# T02: Added a serious-starter public-ingress rail that follows one runtime header straight to one continuity record pair and one diagnostics pair.

> Added a serious-starter public-ingress rail that follows one runtime header straight to one continuity record pair and one diagnostics pair.

## What Happened
---
id: T02
parent: S02
milestone: M054
key_files:
  - compiler/meshc/tests/e2e_m054_s02.rs
  - compiler/meshc/tests/e2e_m047_s07.rs
  - compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs
  - .gsd/milestones/M054/slices/S02/tasks/T02-SUMMARY.md
key_decisions:
  - Reuse the shared raw HTTP response helper in `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` so both the low-level clustered-route rail and the serious starter fail closed on the same header-parsing rules.
  - Keep `compiler/meshc/tests/e2e_m054_s01.rs` intact and land the serious-starter direct-correlation proof as a separate `e2e_m054_s02.rs` target with its own retained bundle.
duration: ""
verification_result: passed
completed_at: 2026-04-06T15:25:47.193Z
blocker_discovered: false
---

# T02: Added a serious-starter public-ingress rail that follows one runtime header straight to one continuity record pair and one diagnostics pair.

**Added a serious-starter public-ingress rail that follows one runtime header straight to one continuity record pair and one diagnostics pair.**

## What Happened

Added a shared fail-closed raw HTTP response-header helper to `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`, reused it in `compiler/meshc/tests/e2e_m047_s07.rs`, and added `compiler/meshc/tests/e2e_m054_s02.rs` as the dedicated serious-starter direct-correlation rail. The new target stages the generated Postgres starter, runs the standby-first public-ingress harness, extracts `X-Mesh-Continuity-Request-Key` from the selected public `GET /todos` response, writes `public-selected-list.request-key.{txt,json}`, and jumps directly to paired continuity-record and diagnostics lookups on both nodes without diffing continuity lists. The retained `.tmp/m054-s02/...` bundle now includes the raw public response, extracted request-key artifact, direct primary/standby record JSON, request-scoped diagnostics entry JSON, and runtime/ingress logs. Added negative tests so malformed raw responses and primary/standby drift fail closed with explicit panic messages.

## Verification

Ran `cargo test -p meshc --test e2e_m054_s02 -- --nocapture` against a disposable local Docker Postgres because this shell did not already have `DATABASE_URL` populated; the full serious-starter rail passed, including the staged two-node runtime proof and the new negative tests. Re-ran `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` to verify the shared header parser did not regress the low-level clustered-route rail. Finally, inspected the newest `.tmp/m054-s02/...` bundle directly and confirmed the extracted request key matches both retained continuity records and both retained diagnostics entry files.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `DATABASE_URL=<disposable local postgres> cargo test -p meshc --test e2e_m054_s02 -- --nocapture` | 0 | ✅ pass | 29550ms |
| 2 | `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` | 0 | ✅ pass | 18959ms |
| 3 | `python3 -c '<m054-s02 retained bundle correlation check>'` | 0 | ✅ pass | 103ms |


## Deviations

Added both `public-selected-list.request-key.txt` and `public-selected-list.request-key.json` so the retained bundle is easy to inspect manually and script against. Otherwise the task followed the written plan.

## Known Issues

None. The assembled verifier/docs alignment remains intentionally deferred to T03.

## Files Created/Modified

- `compiler/meshc/tests/e2e_m054_s02.rs`
- `compiler/meshc/tests/e2e_m047_s07.rs`
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`
- `.gsd/milestones/M054/slices/S02/tasks/T02-SUMMARY.md`


## Deviations
Added both `public-selected-list.request-key.txt` and `public-selected-list.request-key.json` so the retained bundle is easy to inspect manually and script against. Otherwise the task followed the written plan.

## Known Issues
None. The assembled verifier/docs alignment remains intentionally deferred to T03.
