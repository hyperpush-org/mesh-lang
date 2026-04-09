---
id: T02
parent: S01
milestone: M032
provides:
  - Durable `e2e_m032_limit_*` / `e2e_m032_route_*` proofs plus a repo-root replay script for the retained M032 limitation families.
key_files:
  - compiler/meshc/tests/e2e.rs
  - compiler/meshc/tests/e2e_stdlib.rs
  - scripts/verify-m032-s01.sh
  - .gsd/KNOWLEDGE.md
  - .gsd/DECISIONS.md
key_decisions:
  - Kept both named Rust e2e proofs and a direct repo-root replay script so later slices can inspect either stable test names or the exact audited `.tmp/m032-s01` commands.
patterns_established:
  - M032 retained-limit coverage lives under `e2e_m032_limit_*` and `e2e_m032_route_*`, and route-closure truth must come from a live HTTP request rather than a compile-only build.
observability_surfaces:
  - `cargo test -p meshc --test e2e m032_limit -- --nocapture`
  - `cargo test -p meshc --test e2e_stdlib m032_route -- --nocapture`
  - `bash scripts/verify-m032-s01.sh`
duration: 1h 40m
verification_result: passed
completed_at: 2026-03-24T16:19:51-0400
blocker_discovered: false
---

# T02: Capture live blocker and retained-limit proofs in automation

**Added durable retained-limit proofs for the live M032 blockers and a repo-root matrix replay script that reproduces their real failure surfaces.**

## What Happened

I first reproduced the live failure signatures before editing anything so the new assertions would freeze actual behavior instead of plan paraphrases. The CLI-path failures were still exactly where the research left them: `xmod_identity` dies in LLVM verification with a call-signature mismatch, `nested_and` dies in LLVM verification with the PHI predecessor mismatch, and `timer_service_cast` still builds and prints `0` because the delayed message never reaches the service cast handler.

With that evidence in hand, I extended `compiler/meshc/tests/e2e.rs` with three new `e2e_m032_limit_*` proofs tied to the same `.tmp/m032-s01` fixtures the slice research used:

- `e2e_m032_limit_xmod_identity`
- `e2e_m032_limit_nested_and`
- `e2e_m032_limit_timer_service_cast`

Each one uses the real `meshc build` / binary execution path and asserts the relevant retained symptom instead of a vague “still broken” claim.

For the HTTP side, I extended `compiler/meshc/tests/e2e_stdlib.rs` with two named runtime proofs:

- `e2e_m032_route_bare_handler_control`
- `e2e_m032_route_closure_runtime_failure`

I also corrected the misleading `compile_and_start_server(...)` comment and added two small helper seams, `wait_for_server_ready(...)` and `wait_for_server_exit(...)`, so the new route proof could explicitly wait for the runtime listening signal and then poll for the post-request crash surface. The bare-function control now proves a live `200 OK` + `bare_ok` response, while the closure case proves the opposite: no successful `200 OK`, no success body, and an observable failure signal only once a real request is sent.

To make the slice reproducible without rereading research notes, I added `scripts/verify-m032-s01.sh`. That script replays the audited fixture matrix directly from repo root: the four stale-supported commands, the two real CLI failures, the timer no-op, the live bare-route control, the live closure-route failure, and the `mesher` fmt/build baselines. It stops on the first drift and prints the exact command plus the captured log artifact under `.tmp/m032-s01/verify/`.

Finally, I recorded the non-obvious route-closure gotcha in `.gsd/KNOWLEDGE.md` and saved a verification decision in `D047`: this slice intentionally keeps both named Rust tests and a direct fixture replay script because compile-only evidence lies for the closure-route limitation.

## Verification

Task-level verification passed:

- `cargo test -p meshc --test e2e m032_limit -- --nocapture` passed with the three new retained-limit proofs.
- `cargo test -p meshc --test e2e_stdlib m032_route -- --nocapture` passed with the live bare-route control and live closure-route failure proof.
- `test -x scripts/verify-m032-s01.sh && bash scripts/verify-m032-s01.sh` passed and replayed the full audited matrix from repo root.

I also reran the slice-level verification surface after landing T02. At this task boundary:

- `cargo test -p meshc --test e2e m032_ -- --nocapture` now passes with all seven M032 CLI proofs (T01 + T02).
- `cargo test -p meshc --test e2e_stdlib m032_ -- --nocapture` now passes with both M032 route-runtime proofs.
- `cargo run -q -p meshc -- fmt --check mesher` and `cargo run -q -p meshc -- build mesher` both still pass.
- `test -s .gsd/milestones/M032/slices/S01/S01-SUMMARY.md` still fails because that artifact belongs to T03 and has not been written yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e m032_limit -- --nocapture` | 0 | ✅ pass | 5.83s |
| 2 | `cargo test -p meshc --test e2e_stdlib m032_route -- --nocapture` | 0 | ✅ pass | 6.17s |
| 3 | `test -x scripts/verify-m032-s01.sh && bash scripts/verify-m032-s01.sh` | 0 | ✅ pass | 92.57s |
| 4 | `cargo test -p meshc --test e2e m032_ -- --nocapture` | 0 | ✅ pass | 8.41s |
| 5 | `cargo test -p meshc --test e2e_stdlib m032_ -- --nocapture` | 0 | ✅ pass | 6.36s |
| 6 | `cargo run -q -p meshc -- fmt --check mesher` | 0 | ✅ pass | 7.69s |
| 7 | `cargo run -q -p meshc -- build mesher` | 0 | ✅ pass | 14.38s |
| 8 | `test -s .gsd/milestones/M032/slices/S01/S01-SUMMARY.md` | 1 | ❌ fail | 0.01s |

## Diagnostics

The durable inspection surfaces for this task are now:

- `cargo test -p meshc --test e2e m032_limit -- --nocapture` for the `xmod_identity`, `nested_and`, and `timer_service_cast` proofs.
- `cargo test -p meshc --test e2e_stdlib m032_route -- --nocapture` for the live bare-route control and closure-route failure proof.
- `bash scripts/verify-m032-s01.sh` for the repo-root replay matrix.
- `.tmp/m032-s01/verify/` for direct command logs, output diffs, curl responses, and server logs when the script catches drift.

If one of the retained limitations changes, the failure is now inspectable by exact test name or exact replayed command rather than by rereading `S01-RESEARCH.md`.

## Deviations

None.

## Known Issues

- `.gsd/milestones/M032/slices/S01/S01-SUMMARY.md` still does not exist, so the final slice gate `test -s .gsd/milestones/M032/slices/S01/S01-SUMMARY.md` remains red until T03 publishes the audit matrix.

## Files Created/Modified

- `compiler/meshc/tests/e2e.rs` — added `e2e_m032_limit_*` CLI-path proofs for `xmod_identity`, `nested_and`, and `timer_service_cast`.
- `compiler/meshc/tests/e2e_stdlib.rs` — added live route-runtime proofs, introduced `wait_for_server_ready(...)` / `wait_for_server_exit(...)`, and corrected the spawned-server helper contract.
- `scripts/verify-m032-s01.sh` — added the repo-root replay script for the audited `.tmp/m032-s01` matrix plus the `mesher` fmt/build baselines.
- `.gsd/KNOWLEDGE.md` — recorded that compile-only route-closure builds are not authoritative; later slices must use the live server fixture plus the bare-function control.
- `.gsd/DECISIONS.md` — appended D047 documenting why M032/S01 keeps both direct fixture replay and named Rust proofs.
- `.gsd/milestones/M032/slices/S01/tasks/T02-SUMMARY.md` — recorded the implementation, verification evidence, and the remaining T03-owned slice gate.
