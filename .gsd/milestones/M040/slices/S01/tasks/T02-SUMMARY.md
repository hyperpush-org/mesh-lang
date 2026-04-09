---
id: T02
parent: S01
milestone: M040
provides: []
requires: []
affects: []
key_files: ["cluster-proof/work.mpl", ".gsd/milestones/M040/slices/S01/tasks/T02-SUMMARY.md"]
key_decisions: ["Replaced the failing keyed submit tuple-extraction path with explicit `submit_decision` and `submit_next_state` helpers so the Mesh compiler could type-check the runtime path again.", "Stopped before broad follow-on fixes once the context-budget warning fired, leaving the next unit a narrowed resume target: export contract types and rewrite `cluster-proof/tests/work.test.mpl` away from opaque cross-module tuple/struct assumptions."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Reproduced the original runtime build failure, then re-ran the same build after the `Work` refactor and confirmed `cluster-proof` now compiles again. Next, reran `cargo run -q -p meshc -- test cluster-proof/tests` and confirmed the remaining failures are in the keyed contract test module rather than the runtime binary. The task-plan verification commands for T02 (`cargo test -p meshc --test e2e_m040_s01 -- --nocapture` and `bash scripts/verify-m040-s01.sh`) were not reached before wrap-up."
completed_at: 2026-03-28T17:58:16.294Z
blocker_discovered: false
---

# T02: Recovered `cluster-proof` keyed runtime compilation, but the keyed contract tests still fail and the T02 e2e/verifier artifacts remain unfinished.

> Recovered `cluster-proof` keyed runtime compilation, but the keyed contract tests still fail and the T02 e2e/verifier artifacts remain unfinished.

## What Happened
---
id: T02
parent: S01
milestone: M040
key_files:
  - cluster-proof/work.mpl
  - .gsd/milestones/M040/slices/S01/tasks/T02-SUMMARY.md
key_decisions:
  - Replaced the failing keyed submit tuple-extraction path with explicit `submit_decision` and `submit_next_state` helpers so the Mesh compiler could type-check the runtime path again.
  - Stopped before broad follow-on fixes once the context-budget warning fired, leaving the next unit a narrowed resume target: export contract types and rewrite `cluster-proof/tests/work.test.mpl` away from opaque cross-module tuple/struct assumptions.
duration: ""
verification_result: mixed
completed_at: 2026-03-28T17:58:16.295Z
blocker_discovered: false
---

# T02: Recovered `cluster-proof` keyed runtime compilation, but the keyed contract tests still fail and the T02 e2e/verifier artifacts remain unfinished.

**Recovered `cluster-proof` keyed runtime compilation, but the keyed contract tests still fail and the T02 e2e/verifier artifacts remain unfinished.**

## What Happened

Activated the required debugging and test skills, read the task inputs, reproduced the inherited `cluster-proof` build failure, and narrowed the root cause to the keyed `Work` runtime rather than the outer harness. Refactored `cluster-proof/work.mpl` so the keyed submit/runtime path no longer depends on the failing tuple-extraction flow at runtime, using explicit decision/state helpers instead. After that change, `cargo run -q -p meshc -- build cluster-proof` passed again. I then reran `cargo run -q -p meshc -- test cluster-proof/tests` and found the next failure frontier in `cluster-proof/tests/work.test.mpl`: the proof file still depends on cross-module keyed contract visibility and tuple-return assumptions that the current compiler rejects. I stopped there because the context-budget warning required an immediate wrap-up. I did not create `compiler/meshc/tests/e2e_m040_s01.rs` or `scripts/verify-m040-s01.sh`.

## Verification

Reproduced the original runtime build failure, then re-ran the same build after the `Work` refactor and confirmed `cluster-proof` now compiles again. Next, reran `cargo run -q -p meshc -- test cluster-proof/tests` and confirmed the remaining failures are in the keyed contract test module rather than the runtime binary. The task-plan verification commands for T02 (`cargo test -p meshc --test e2e_m040_s01 -- --nocapture` and `bash scripts/verify-m040-s01.sh`) were not reached before wrap-up.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- build cluster-proof` | 1 | ❌ fail | 3000ms |
| 2 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 3000ms |
| 3 | `cargo run -q -p meshc -- test cluster-proof/tests` | 1 | ❌ fail | 1100ms |


## Deviations

Did not start the named Rust e2e proof or the repo-root verifier script from the task plan. This unit became a recovery pass to restore `cluster-proof` runtime buildability after inheriting the unresolved T01 compile break, and it ended at the next narrowed failure frontier because of the context-budget wrap-up requirement.

## Known Issues

`cluster-proof/tests/work.test.mpl` still fails to compile. The current failures indicate the test file cannot safely inspect the keyed contract through the existing exports: `WorkStatusPayload` and `SubmitMutation` need export/usage alignment, and the test should stop depending on the current tuple-helper extraction pattern if the compiler continues to treat those returned values opaquely across modules. `compiler/meshc/tests/e2e_m040_s01.rs` and `scripts/verify-m040-s01.sh` do not exist yet.

## Files Created/Modified

- `cluster-proof/work.mpl`
- `.gsd/milestones/M040/slices/S01/tasks/T02-SUMMARY.md`


## Deviations
Did not start the named Rust e2e proof or the repo-root verifier script from the task plan. This unit became a recovery pass to restore `cluster-proof` runtime buildability after inheriting the unresolved T01 compile break, and it ended at the next narrowed failure frontier because of the context-budget wrap-up requirement.

## Known Issues
`cluster-proof/tests/work.test.mpl` still fails to compile. The current failures indicate the test file cannot safely inspect the keyed contract through the existing exports: `WorkStatusPayload` and `SubmitMutation` need export/usage alignment, and the test should stop depending on the current tuple-helper extraction pattern if the compiler continues to treat those returned values opaquely across modules. `compiler/meshc/tests/e2e_m040_s01.rs` and `scripts/verify-m040-s01.sh` do not exist yet.
