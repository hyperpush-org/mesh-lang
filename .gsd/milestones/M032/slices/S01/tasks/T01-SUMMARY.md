---
id: T01
parent: S01
milestone: M032
provides:
  - Durable `e2e_m032_supported_*` CLI-path proofs for the four stale-supported mesher folklore families.
key_files:
  - compiler/meshc/tests/e2e.rs
  - .gsd/milestones/M032/slices/S01/S01-PLAN.md
  - .gsd/milestones/M032/slices/S01/tasks/T01-PLAN.md
key_decisions:
  - Anchored the new proofs to the audited `.tmp/m032-s01` source programs via `include_str!` so the e2e harness exercises the exact repro shapes the slice investigated.
patterns_established:
  - M032 supported-folklore coverage lives under `e2e_m032_supported_*` and asserts exact stdout through the real `meshc build` path.
observability_surfaces:
  - `cargo test -p meshc --test e2e m032_supported -- --nocapture`
  - `rg -c 'fn e2e_m032_supported_' compiler/meshc/tests/e2e.rs`
  - `cargo test -p meshc --test e2e m032_ -- --nocapture`
duration: 1h 15m
verification_result: passed
completed_at: 2026-03-24T15:54:13-0400
blocker_discovered: false
---

# T01: Encode stale-folklore paths as CLI e2e proofs

**Added four `e2e_m032_supported_*` CLI e2e proofs tied to the stale mesher workaround families and verified they pass through the real `meshc build` path.**

## What Happened

I first fixed the pre-flight artifact gaps: `S01-PLAN.md` now names an explicit `m032_limit` failure-surface gate, and `T01-PLAN.md` now includes an `## Observability Impact` section describing the new inspection path.

For the implementation itself, I reproduced the audited `.tmp/m032-s01` programs with the actual CLI build path to confirm their claimed outputs before changing the harness. Then I added four new tests in `compiler/meshc/tests/e2e.rs`:

- `e2e_m032_supported_request_query`
- `e2e_m032_supported_cross_module_from_json`
- `e2e_m032_supported_service_call_case`
- `e2e_m032_supported_cast_if_else`

Each test uses the real `meshc build` flow, asserts exact stdout, and carries a nearby comment naming the mesher cleanup target it retires:

- `mesher/ingestion/routes.mpl` for `Request.query(...)`
- `mesher/services/event_processor.mpl`, `mesher/storage/queries.mpl`, and `mesher/storage/writer.mpl` for cross-module `from_json`
- `mesher/services/user.mpl` for inline `case` in a service call body
- `mesher/services/stream_manager.mpl` for inline `if/else` in a cast handler

I kept the canonical source shapes anchored to the audited `.tmp/m032-s01` programs via `include_str!` so later slices can trace a passing proof directly back to the repro that motivated it.

## Verification

Task-level verification passed:

- `cargo test -p meshc --test e2e m032_supported -- --nocapture` ran the four new proofs and all passed.
- `rg -c 'fn e2e_m032_supported_' compiler/meshc/tests/e2e.rs` returned `4`.

I also ran the slice-level verification commands to establish current progress. At this intermediate task boundary:

- `cargo test -p meshc --test e2e m032_ -- --nocapture` passes because T01's four supported-path proofs now exist and pass.
- `cargo run -q -p meshc -- fmt --check mesher` and `cargo run -q -p meshc -- build mesher` both pass, so the audit work did not regress current dogfood formatting or compilation.
- The T02/T03-owned slice gates are still incomplete: the targeted `m032_limit` and `e2e_stdlib m032_` filters currently run `0` tests, `scripts/verify-m032-s01.sh` does not exist yet, and `S01-SUMMARY.md` has not been written yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e m032_supported -- --nocapture` | 0 | ✅ pass | 11.00s |
| 2 | `rg -c 'fn e2e_m032_supported_' compiler/meshc/tests/e2e.rs` | 0 | ✅ pass | 0.05s |
| 3 | `cargo test -p meshc --test e2e m032_ -- --nocapture` | 0 | ✅ pass | 13.28s |
| 4 | `cargo test -p meshc --test e2e m032_limit -- --nocapture` *(ran 0 tests)* | 0 | ❌ fail | 2.53s |
| 5 | `cargo test -p meshc --test e2e_stdlib m032_ -- --nocapture` *(ran 0 tests)* | 0 | ❌ fail | 8.27s |
| 6 | `bash scripts/verify-m032-s01.sh` | 127 | ❌ fail | 0.03s |
| 7 | `cargo run -q -p meshc -- fmt --check mesher` | 0 | ✅ pass | 7.27s |
| 8 | `cargo run -q -p meshc -- build mesher` | 0 | ✅ pass | 14.04s |
| 9 | `test -s .gsd/milestones/M032/slices/S01/S01-SUMMARY.md` | 1 | ❌ fail | 0.05s |

## Diagnostics

The new inspection path is deliberately narrow:

- `cargo test -p meshc --test e2e m032_supported -- --nocapture` is the authoritative supported-path proof surface for this task.
- `compiler/meshc/tests/e2e.rs` now contains one named proof per stale folklore family, with comments pointing at the exact mesher files later cleanup slices should touch.
- The compile-time fixture paths under `.tmp/m032-s01/` show the canonical source shapes the tests are exercising.
- If one of these behaviors regresses, the failing test name plus the exact stdout mismatch identifies which workaround family became questionable again.

## Deviations

None.

## Known Issues

- `cargo test -p meshc --test e2e m032_limit -- --nocapture` currently exits 0 while running 0 tests because T02 has not added the retained-limit proofs yet.
- `cargo test -p meshc --test e2e_stdlib m032_ -- --nocapture` currently exits 0 while running 0 tests because the route-runtime proofs belong to T02.
- `scripts/verify-m032-s01.sh` does not exist yet; creating it is part of T02.
- `.gsd/milestones/M032/slices/S01/S01-SUMMARY.md` does not exist yet; writing it is part of T03.

## Files Created/Modified

- `compiler/meshc/tests/e2e.rs` — added four `e2e_m032_supported_*` CLI e2e proofs, each tied to a stale mesher workaround family and asserting exact stdout.
- `.gsd/milestones/M032/slices/S01/S01-PLAN.md` — added an explicit slice-level `m032_limit` failure-surface verification gate and marked T01 complete.
- `.gsd/milestones/M032/slices/S01/tasks/T01-PLAN.md` — added the missing `## Observability Impact` section required by the pre-flight checks.
- `.gsd/KNOWLEDGE.md` — recorded that the M032 slice filters can exit 0 while running 0 tests, which must be treated as incomplete coverage rather than a passing gate.
- `.gsd/milestones/M032/slices/S01/tasks/T01-SUMMARY.md` — recorded the implementation, verification evidence, and partial slice-gate status for the next task.
