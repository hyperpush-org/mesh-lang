---
id: T01
parent: S01
milestone: M040
key_files:
  - cluster-proof/work.mpl
  - cluster-proof/main.mpl
  - cluster-proof/tests/work.test.mpl
  - .gsd/milestones/M040/slices/S01/tasks/T01-SUMMARY.md
key_decisions:
  - Drafted the keyed contract around request-key identity, attempt IDs, and status snapshots instead of extending the old anonymous `request_id` shape.
  - Stopped execution and wrote a detailed handoff once Mesh compile/type failures persisted and the context budget warning required wrap-up.
duration:
verification_result: mixed
completed_at: 2026-03-28T17:45:04.495Z
blocker_discovered: false
---

# T01: Attempted a keyed `/work` submit/status refactor and test rewrite, but the Mesh package still fails to compile.

**Attempted a keyed `/work` submit/status refactor and test rewrite, but the Mesh package still fails to compile.**

## What Happened

Activated the required skills, read the local `cluster-proof` plans and sources, and replaced the previous one-shot `/work` proof with a draft keyed contract in `cluster-proof/work.mpl`. Rewired `cluster-proof/main.mpl` to `POST /work` plus `GET /work/:request_key`, and rewrote `cluster-proof/tests/work.test.mpl` around keyed parsing, idempotent same-key reuse, conflict rejection, and completion/status assertions. The work did not reach a shippable state: after parser fixes, later verification exposed deeper Mesh-specific compile/type issues in the new registry/service/helper shape, so the package remains red and the summary captures the exact handoff.

## Verification

Ran `cargo run -q -p meshc -- test cluster-proof/tests` and `cargo run -q -p meshc -- build cluster-proof`. Both failed during compilation, so task-level verification did not pass and slice-level verification was not attempted. The task summary records the failed commands and the current resume points.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- test cluster-proof/tests` | 1 | ❌ fail | 6000ms |
| 2 | `cargo run -q -p meshc -- build cluster-proof` | 1 | ❌ fail | 6000ms |

## Deviations

Stopped before finishing implementation and before slice-level verification because the package was still compile-failing when the context budget warning required an immediate wrap-up.

## Known Issues

`cluster-proof/work.mpl` does not compile; the keyed registry/service draft is incomplete; `cluster-proof/tests/work.test.mpl` depends on the unfinished draft contract; and `cluster-proof/main.mpl` now points to keyed handlers that are not yet buildable.

## Files Created/Modified

- `cluster-proof/work.mpl`
- `cluster-proof/main.mpl`
- `cluster-proof/tests/work.test.mpl`
- `.gsd/milestones/M040/slices/S01/tasks/T01-SUMMARY.md`
