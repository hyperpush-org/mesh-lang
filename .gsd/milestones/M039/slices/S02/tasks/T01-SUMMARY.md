---
id: T01
parent: S02
milestone: M039
provides: []
requires: []
affects: []
key_files: ["cluster-proof/work.mpl", "cluster-proof/cluster.mpl", "cluster-proof/main.mpl", "cluster-proof/tests/work.test.mpl", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M039/slices/S02/tasks/T01-SUMMARY.md"]
key_decisions: ["Kept the attempted cross-node worker protocol scalar-only after confirming Mesh currently ships distributed spawn/send payloads as raw bytes rather than deep-serialized structs or strings.", "Recorded the handler-context/runtime mismatch as a blocker instead of papering over it with a fake local-only implementation."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the task-level verification gates. `cargo run -q -p meshc -- test cluster-proof/tests` failed during compile/type-check with `self() used outside actor block`, cross-module field-access issues, and follow-on scope/type failures in the attempted handler-side request/reply implementation. `cargo run -q -p meshc -- build cluster-proof` failed with the same blocker set."
completed_at: 2026-03-28T10:35:51.143Z
blocker_discovered: true
---

# T01: Started the cluster-proof work-routing implementation, but stopped at a real runtime mismatch: Mesh HTTP handlers are not actor contexts and the distributed transport still only carries raw scalar bytes.

> Started the cluster-proof work-routing implementation, but stopped at a real runtime mismatch: Mesh HTTP handlers are not actor contexts and the distributed transport still only carries raw scalar bytes.

## What Happened
---
id: T01
parent: S02
milestone: M039
key_files:
  - cluster-proof/work.mpl
  - cluster-proof/cluster.mpl
  - cluster-proof/main.mpl
  - cluster-proof/tests/work.test.mpl
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M039/slices/S02/tasks/T01-SUMMARY.md
key_decisions:
  - Kept the attempted cross-node worker protocol scalar-only after confirming Mesh currently ships distributed spawn/send payloads as raw bytes rather than deep-serialized structs or strings.
  - Recorded the handler-context/runtime mismatch as a blocker instead of papering over it with a fake local-only implementation.
duration: ""
verification_result: mixed
completed_at: 2026-03-28T10:35:51.154Z
blocker_discovered: true
---

# T01: Started the cluster-proof work-routing implementation, but stopped at a real runtime mismatch: Mesh HTTP handlers are not actor contexts and the distributed transport still only carries raw scalar bytes.

**Started the cluster-proof work-routing implementation, but stopped at a real runtime mismatch: Mesh HTTP handlers are not actor contexts and the distributed transport still only carries raw scalar bytes.**

## What Happened

I added the planned work-routing module scaffold in `cluster-proof/work.mpl`, wired `/work` in `cluster-proof/main.mpl`, extracted a reusable membership snapshot helper in `cluster-proof/cluster.mpl`, and added pure helper coverage in `cluster-proof/tests/work.test.mpl`. Verification then exposed two plan-invalidating runtime truths: `self()` is rejected inside the HTTP handler path, so the planned handler-side request/reply flow cannot wait on a spawned worker reply without a different actor/service boundary; and Mesh distributed `Node.spawn` args plus mailbox `send(...)` payloads still move as raw value bytes rather than deep-serialized Mesh strings/structs, so the cross-node protocol cannot safely carry the string-heavy routing context the original design assumed. I preserved the partial implementation on disk and recorded both runtime constraints in `.gsd/KNOWLEDGE.md` for the next unit.

## Verification

Ran the task-level verification gates. `cargo run -q -p meshc -- test cluster-proof/tests` failed during compile/type-check with `self() used outside actor block`, cross-module field-access issues, and follow-on scope/type failures in the attempted handler-side request/reply implementation. `cargo run -q -p meshc -- build cluster-proof` failed with the same blocker set.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- test cluster-proof/tests` | 1 | ❌ fail | 5231ms |
| 2 | `cargo run -q -p meshc -- build cluster-proof` | 1 | ❌ fail | 4621ms |


## Deviations

Implementation stopped as a blocker handoff instead of a working route once local verification proved the handler/request-reply assumption was invalid against the current Mesh runtime.

## Known Issues

`cluster-proof/work.mpl` does not compile; `cluster-proof/tests/work.test.mpl` does not compile; `/work` is wired in `cluster-proof/main.mpl` but the app does not build because the route module is blocked; the next unit needs a runtime-supported return path instead of handler-side mailbox waiting.

## Files Created/Modified

- `cluster-proof/work.mpl`
- `cluster-proof/cluster.mpl`
- `cluster-proof/main.mpl`
- `cluster-proof/tests/work.test.mpl`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M039/slices/S02/tasks/T01-SUMMARY.md`


## Deviations
Implementation stopped as a blocker handoff instead of a working route once local verification proved the handler/request-reply assumption was invalid against the current Mesh runtime.

## Known Issues
`cluster-proof/work.mpl` does not compile; `cluster-proof/tests/work.test.mpl` does not compile; `/work` is wired in `cluster-proof/main.mpl` but the app does not build because the route module is blocked; the next unit needs a runtime-supported return path instead of handler-side mailbox waiting.
