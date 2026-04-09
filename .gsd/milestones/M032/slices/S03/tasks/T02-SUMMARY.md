---
id: T02
parent: S03
milestone: M032
provides:
  - Inline Mesh-supported control flow in mesher user and stream service handlers without disturbing the retained nested-&& and timer keep-sites
key_files:
  - mesher/services/user.mpl
  - mesher/services/stream_manager.mpl
  - .gsd/milestones/M032/slices/S03/S03-PLAN.md
  - .gsd/milestones/M032/slices/S03/tasks/T02-SUMMARY.md
key_decisions:
  - Kept the supported control flow inline in the real service handlers and preserved `both_match(...)` plus the timer comments as explicit retained-limit guard sites
patterns_established:
  - Mesh service call bodies can return `(state, case ...)` directly, and cast handlers can use inline `if/else` bodies without helper extraction
observability_surfaces:
  - compiler/meshc/tests/e2e.rs::e2e_m032_supported_request_query
  - compiler/meshc/tests/e2e.rs::e2e_m032_supported_service_call_case
  - compiler/meshc/tests/e2e.rs::e2e_m032_supported_cast_if_else
  - compiler/meshc/tests/e2e.rs::e2e_m032_limit_nested_and
  - compiler/meshc/tests/e2e_stdlib.rs::e2e_m032_route_closure_runtime_failure
  - rg checks over stale folklore comments and retained keep-site comments in mesher/
  - cargo run -q -p meshc -- fmt --check mesher
  - cargo run -q -p meshc -- build mesher
duration: 24m
verification_result: passed
completed_at: 2026-03-24T22:39:41Z
blocker_discovered: false
---

# T02: Inline supported control flow in user and stream services

**Inlined UserService login case handling and StreamManager buffer gating directly in the mesher handlers.**

## What Happened

I removed the stale helper/comment pair from `mesher/services/user.mpl` and moved the `authenticate_user(...)` result handling directly into `UserService.Login`, preserving the existing `Err(_) -> Err("authentication failed")` behavior. The final handler now returns its `(pool, case ...)` tuple directly, which is the support shape this slice is supposed to dogfood.

I also removed the stale `buffer_if_client(...)` helper/comment pair from `mesher/services/stream_manager.mpl` and inlined the `if is_stream_client(...)` branch directly into `StreamManager.BufferMessage`. I left `both_match(...)` and its nested-`&&` keep-site comment untouched, along with the timer keep-sites in `mesher/services/writer.mpl` and `mesher/ingestion/pipeline.mpl`.

The only follow-up during verification was formatting: the first `meshc fmt --check` pass reported that `mesher/services/user.mpl` needed canonical formatting after the inline-case rewrite, so I ran `meshc fmt` on that file and reran the full slice gate. The final gate passed cleanly.

## Verification

I ran the full slice verification set because T02 closes S03: the request-query proof from T01, the two supported handler-control-flow proofs, the retained nested-`&&` limit proof, the route-closure runtime guard, the stale/retained comment greps, and the mesher format/build checks. All final checks passed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -q -p meshc --test e2e e2e_m032_supported_request_query -- --nocapture` | 0 | ✅ pass | 8.62s |
| 2 | `cargo test -q -p meshc --test e2e e2e_m032_supported_service_call_case -- --nocapture` | 0 | ✅ pass | 8.57s |
| 3 | `cargo test -q -p meshc --test e2e e2e_m032_supported_cast_if_else -- --nocapture` | 0 | ✅ pass | 7.86s |
| 4 | `cargo test -q -p meshc --test e2e e2e_m032_limit_nested_and -- --nocapture` | 0 | ✅ pass | 6.23s |
| 5 | `cargo test -q -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture` | 0 | ✅ pass | 7.95s |
| 6 | `! rg -n "query string parsing not available in Mesh|complex case expressions|parser limitation with if/else in cast handlers" mesher/ingestion/routes.mpl mesher/services/user.mpl mesher/services/stream_manager.mpl` | 0 | ✅ pass | 0.05s |
| 7 | `rg -n "HTTP routing does not support closures|avoids && codegen issue inside nested if blocks|Timer.send_after delivers raw bytes" mesher/ingestion/routes.mpl mesher/services/stream_manager.mpl mesher/services/writer.mpl mesher/ingestion/pipeline.mpl` | 0 | ✅ pass | 0.03s |
| 8 | `cargo run -q -p meshc -- fmt --check mesher` | 0 | ✅ pass | 6.51s |
| 9 | `cargo run -q -p meshc -- build mesher` | 0 | ✅ pass | 12.03s |

## Diagnostics

Future inspection stays on the existing M032 surfaces: rerun `e2e_m032_supported_service_call_case` for inline service-call case support, rerun `e2e_m032_supported_cast_if_else` for inline cast-handler control flow, rerun `e2e_m032_limit_nested_and` plus the retained-comment grep to confirm `both_match(...)` still marks the real nested-`&&` limit, rerun `e2e_m032_route_closure_runtime_failure` for the preserved route limitation, and use `cargo run -q -p meshc -- fmt --check mesher` plus `cargo run -q -p meshc -- build mesher` to confirm mesher still composes cleanly.

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `mesher/services/user.mpl` — removed the stale login helper/comment pair and inlined the supported `case authenticate_user(...)` flow directly in `UserService.Login`.
- `mesher/services/stream_manager.mpl` — removed the stale buffer helper/comment pair and inlined the supported `if is_stream_client(...)` branch directly in `StreamManager.BufferMessage` while preserving `both_match(...)`.
- `.gsd/milestones/M032/slices/S03/tasks/T02-SUMMARY.md` — recorded the implementation details and final slice verification evidence.
