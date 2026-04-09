---
id: T02
parent: S03
milestone: M044
provides: []
requires: []
affects: []
key_files: [".gsd/KNOWLEDGE.md", ".gsd/milestones/M044/slices/S03/tasks/T02-SUMMARY.md"]
key_decisions: ["Treat the current `mesh-rt` `query_operator_*` helpers as insufficient for `meshc cluster` because they require a live node session and therefore mutate the target cluster's peer view."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "I did not run the task's live `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture` rail because the blocker was discovered during command-path validation, before any honest CLI implementation could be written. I confirmed the mismatch with targeted codebase probes and preserved the result in the task summary plus `.gsd/KNOWLEDGE.md`."
completed_at: 2026-03-30T00:24:22.861Z
blocker_discovered: true
---

# T02: Confirmed that the current runtime operator query path would make `meshc cluster` join the target cluster and pollute the membership snapshot it is supposed to inspect.

> Confirmed that the current runtime operator query path would make `meshc cluster` join the target cluster and pollute the membership snapshot it is supposed to inspect.

## What Happened
---
id: T02
parent: S03
milestone: M044
key_files:
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M044/slices/S03/tasks/T02-SUMMARY.md
key_decisions:
  - Treat the current `mesh-rt` `query_operator_*` helpers as insufficient for `meshc cluster` because they require a live node session and therefore mutate the target cluster's peer view.
duration: ""
verification_result: passed
completed_at: 2026-03-30T00:24:22.863Z
blocker_discovered: true
---

# T02: Confirmed that the current runtime operator query path would make `meshc cluster` join the target cluster and pollute the membership snapshot it is supposed to inspect.

**Confirmed that the current runtime operator query path would make `meshc cluster` join the target cluster and pollute the membership snapshot it is supposed to inspect.**

## What Happened

I stopped before implementation after validating the local runtime seam. The planner assumed T01 had already produced a safe read-only operator client for `meshc`, but the current Rust-side helpers do not meet that bar. In `compiler/mesh-rt/src/dist/operator.rs`, every remote `query_operator_*` call flows through `execute_query(...)`, which requires `node_state()` and uses `peer_names(state)` from a live local node. In `compiler/mesh-rt/src/dist/node.rs`, the only outgoing connect path is `connect_to_remote_node(...)`, which registers a real session and then sends peer-list and continuity sync messages. That means a straightforward `meshc cluster --target <node>` implementation would need to start its own runtime node, connect to the target, and become a visible peer long enough for the target runtime to include the CLI in membership. For `cluster status`, that would corrupt the runtime-owned truth the task is meant to expose. The missing capability is a transient authenticated operator channel, or an equivalent non-registering query path, that can query a node without joining its cluster. I recorded the rule in `.gsd/KNOWLEDGE.md` and wrote the blocker summary instead of shipping a misleading CLI wrapper around the existing session-based path.

## Verification

I did not run the task's live `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture` rail because the blocker was discovered during command-path validation, before any honest CLI implementation could be written. I confirmed the mismatch with targeted codebase probes and preserved the result in the task summary plus `.gsd/KNOWLEDGE.md`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `rg -n "execute_query\(|node_state\(|peer_names\(|query_operator_status|target_not_connected" compiler/mesh-rt/src/dist/operator.rs` | 0 | ✅ pass | 41ms |
| 2 | `rg -n "register_session\(|send_peer_list\(|send_continuity_sync\(|mesh_node_connect|connect_to_remote_node" compiler/mesh-rt/src/dist/node.rs` | 0 | ✅ pass | 45ms |


## Deviations

I did not create `compiler/meshc/src/cluster.rs`, update `compiler/meshc/src/main.rs`, or add `compiler/meshc/tests/e2e_m044_s03.rs` because the current runtime seam would make those changes dishonest rather than incomplete.

## Known Issues

A future task or replan now needs a runtime-side transient authenticated operator query path that does not register a cluster session or send peer/continuity sync, otherwise `meshc cluster status` cannot report truthful membership for the inspected node.

## Files Created/Modified

- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M044/slices/S03/tasks/T02-SUMMARY.md`


## Deviations
I did not create `compiler/meshc/src/cluster.rs`, update `compiler/meshc/src/main.rs`, or add `compiler/meshc/tests/e2e_m044_s03.rs` because the current runtime seam would make those changes dishonest rather than incomplete.

## Known Issues
A future task or replan now needs a runtime-side transient authenticated operator query path that does not register a cluster session or send peer/continuity sync, otherwise `meshc cluster status` cannot report truthful membership for the inspected node.
