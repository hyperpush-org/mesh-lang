---
estimated_steps: 4
estimated_files: 6
skills_used:
  - test
  - debug-like-expert
---

# T01: Encode stale-folklore paths as CLI e2e proofs

**Slice:** S01 — Limitation Truth Audit and Repro Matrix
**Milestone:** M032

## Description

Turn the "already supported" side of the audit into durable compiler evidence. This task adds `meshc` CLI-path tests for the four stale-folklore families the research proved green today: query-string access, cross-module `from_json`, inline `case` inside a service call body, and inline `if/else` inside a cast handler. These are the proofs later slices will cite when deleting stale comments from `mesher/ingestion/routes.mpl`, `mesher/services/event_processor.mpl`, `mesher/storage/queries.mpl`, `mesher/storage/writer.mpl`, `mesher/services/user.mpl`, and `mesher/services/stream_manager.mpl`.

## Steps

1. Add `e2e_m032_supported_*` tests to `compiler/meshc/tests/e2e.rs`, using the existing helper style in that file so every proof still goes through the real `meshc build` path.
2. Use the current `.tmp/m032-s01` programs as the canonical shapes for the new tests: `request_query` must build and print `request_query_ok`, `xmod_from_json` must round-trip and print `Scout 7`, `service_call_case` must print `yes` and `no`, and `cast_if_else` must print `1` and `2`.
3. Keep the test names or nearby comments explicit about which mesher folklore they retire so later cleanup slices can remove comments without re-running the full investigation.
4. Run `cargo test -p meshc --test e2e m032_supported -- --nocapture` and tighten any assertions until the new tests pass cleanly and fail loudly on drift.

## Must-Haves

- [ ] `compiler/meshc/tests/e2e.rs` contains passing `e2e_m032_supported_*` coverage for all four stale-supported families
- [ ] Each test asserts real build/run behavior or exact stdout, not only parser or typechecker success
- [ ] The test names/comments make the mesher cleanup targets obvious to S03

## Verification

- `cargo test -p meshc --test e2e m032_supported -- --nocapture`
- `rg -c 'fn e2e_m032_supported_' compiler/meshc/tests/e2e.rs`

## Inputs

- `compiler/meshc/tests/e2e.rs` — existing CLI-path harness and nearby cross-module examples
- `.tmp/m032-s01/request_query/main.mpl` — minimal query-string support repro
- `.tmp/m032-s01/xmod_from_json/main.mpl` — cross-module `from_json` caller
- `.tmp/m032-s01/xmod_from_json/models.mpl` — cross-module `from_json` data model
- `.tmp/m032-s01/service_call_case/main.mpl` — inline `case` in a service call body
- `.tmp/m032-s01/cast_if_else/main.mpl` — inline `if/else` in a cast handler

## Expected Output

- `compiler/meshc/tests/e2e.rs` — `e2e_m032_supported_*` tests covering the four stale-supported folklore families

## Observability Impact

- New inspectable signals: `cargo test -p meshc --test e2e m032_supported -- --nocapture` must expose four named `e2e_m032_supported_*` proofs whose stdout assertions fail loudly if the supposedly-supported behavior drifts.
- Future-agent inspection path: read `compiler/meshc/tests/e2e.rs` for the folklore-retirement comments and rerun the targeted `m032_supported` filter instead of redoing the ad hoc `.tmp/m032-s01` investigation.
- Failure visibility added by this task: query access, cross-module `from_json`, service-call `case`, and cast-handler `if/else` each get their own test name plus exact stdout expectation so regressions point at the stale comment family that became false again.
