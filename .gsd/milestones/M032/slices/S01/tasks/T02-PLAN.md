---
estimated_steps: 4
estimated_files: 8
skills_used:
  - test
  - debug-like-expert
---

# T02: Capture live blocker and retained-limit proofs in automation

**Slice:** S01 — Limitation Truth Audit and Repro Matrix
**Milestone:** M032

## Description

Encode the "still real" side of the audit as repeatable automation. This task makes the priority S02 blocker and the retained keep-sites executable instead of folkloric: cross-module inferred polymorphic export must fail on the real CLI path, nested `&&` inside nested `if` must still fail in codegen, `Timer.send_after` to a service cast must still no-op, and HTTP route closures must still fail on a live request while a bare-function control succeeds. These proofs are the safety rails for `mesher/storage/writer.mpl`, `mesher/services/stream_manager.mpl`, `mesher/services/writer.mpl`, `mesher/ingestion/pipeline.mpl`, and `mesher/ingestion/routes.mpl`.

## Steps

1. Extend `compiler/meshc/tests/e2e.rs` with `e2e_m032_limit_*` coverage for the non-HTTP cases: assert that `xmod_identity` fails with the imported-polymorphic LLVM call-signature mismatch, `nested_and` fails with the LLVM PHI mismatch, and `timer_service_cast` still runs and prints `0`.
2. Extend `compiler/meshc/tests/e2e_stdlib.rs` with a live-server runtime proof that uses `route_closure_server` as the failing case and `route_bare_server` as the control. Start both through the existing server harness, send real requests, and assert the closure path fails by crash / empty reply / non-200 while the bare-function path returns `200` with `bare_ok`.
3. Add `scripts/verify-m032-s01.sh` to replay the audited `.tmp/m032-s01` matrix from repo root, including the stale-supported commands, the real-failure commands, the timer no-op, the route-closure runtime trap, and the mesher fmt/build baselines. Make it stop on the first drift and print the exact failing command.
4. Run the targeted test filters and the script until the failure signatures are crisp, stable, and reusable by later slices without rereading the research notes.

## Must-Haves

- [ ] `compiler/meshc/tests/e2e.rs` contains named `e2e_m032_limit_*` coverage for `xmod_identity`, `nested_and`, and `timer_service_cast`
- [ ] `compiler/meshc/tests/e2e_stdlib.rs` contains a live-request route-closure proof with a bare-handler control
- [ ] `scripts/verify-m032-s01.sh` replays the slice matrix and fails with the exact drifted command or symptom

## Verification

- `cargo test -p meshc --test e2e m032_limit -- --nocapture`
- `cargo test -p meshc --test e2e_stdlib m032_route -- --nocapture`
- `test -x scripts/verify-m032-s01.sh && bash scripts/verify-m032-s01.sh`

## Observability Impact

- Signals added/changed: named test failures for `xmod_identity`, `nested_and`, timer-service no-op, and route-closure runtime failure
- How a future agent inspects this: `cargo test -p meshc --test e2e m032_limit -- --nocapture`, `cargo test -p meshc --test e2e_stdlib m032_route -- --nocapture`, and `bash scripts/verify-m032-s01.sh`
- Failure state exposed: exact fixture/test name plus the LLVM verifier substring, stdout `0`, or HTTP empty-reply/non-200 symptom that made the classification fail

## Inputs

- `compiler/meshc/tests/e2e.rs` — existing CLI-path harness and failure-test helpers
- `compiler/meshc/tests/e2e_stdlib.rs` — existing HTTP runtime harness
- `.tmp/m032-s01/xmod_identity/main.mpl` — cross-module inferred-polymorphic export caller
- `.tmp/m032-s01/xmod_identity/utils.mpl` — cross-module inferred-polymorphic export callee
- `.tmp/m032-s01/nested_and/main.mpl` — nested `&&` codegen failure repro
- `.tmp/m032-s01/timer_service_cast/main.mpl` — timer-to-service-cast no-op repro
- `.tmp/m032-s01/route_closure_server/main.mpl` — runtime-failing closure-route server
- `.tmp/m032-s01/route_bare_server/main.mpl` — runtime-success bare-handler control

## Expected Output

- `compiler/meshc/tests/e2e.rs` — `e2e_m032_limit_*` tests for the real CLI-path blocker and retained limits
- `compiler/meshc/tests/e2e_stdlib.rs` — `e2e_m032_route_*` runtime proof for closure routes vs bare handlers
- `scripts/verify-m032-s01.sh` — executable slice verification script that replays the audited matrix
