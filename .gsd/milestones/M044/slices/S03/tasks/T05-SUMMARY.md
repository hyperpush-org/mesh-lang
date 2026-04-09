---
id: T05
parent: S03
milestone: M044
provides: []
requires: []
affects: []
key_files: [".gsd/KNOWLEDGE.md", ".gsd/milestones/M044/slices/S03/tasks/T05-SUMMARY.md"]
key_decisions: ["Treat T05 as blocked until the truthful `meshc cluster` surface exists; the current tree still lacks the T04 CLI/test files and the runtime query path still depends on a started local node plus an existing target session."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "I verified the blocker directly against the planned T05 rails and the current runtime/CLI seams. The tooling filter named by the task still exits 0 while running `0 tests`, the dedicated S03 e2e target still does not exist, and the static seam check confirms both the missing files and the pre-blocker operator-query transport shape."
completed_at: 2026-03-30T01:05:03.510Z
blocker_discovered: true
---

# T05: Recorded that `meshc init --clustered` remains blocked because the public cluster CLI and scaffold proof rails do not exist yet.

> Recorded that `meshc init --clustered` remains blocked because the public cluster CLI and scaffold proof rails do not exist yet.

## What Happened
---
id: T05
parent: S03
milestone: M044
key_files:
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M044/slices/S03/tasks/T05-SUMMARY.md
key_decisions:
  - Treat T05 as blocked until the truthful `meshc cluster` surface exists; the current tree still lacks the T04 CLI/test files and the runtime query path still depends on a started local node plus an existing target session.
duration: ""
verification_result: mixed
completed_at: 2026-03-30T01:05:03.512Z
blocker_discovered: true
---

# T05: Recorded that `meshc init --clustered` remains blocked because the public cluster CLI and scaffold proof rails do not exist yet.

**Recorded that `meshc init --clustered` remains blocked because the public cluster CLI and scaffold proof rails do not exist yet.**

## What Happened

I stopped before changing the scaffold because the task's required public dependency is still missing in the local tree. The runtime operator client in `compiler/mesh-rt/src/dist/operator.rs` still routes remote reads through `execute_query(...)`, which requires `node_state()` and an already-registered session in `state.sessions`; it still fails with `target_not_connected` rather than opening a transient inspection channel. On the CLI side, `compiler/meshc/src/main.rs` still exposes only the plain `Init { name }` path and still calls the two-argument `mesh_pkg::scaffold_project(&name, &dir)`, with no `--clustered` flag and no `cluster` command module. The expected T04/T05 proof files are also still absent: `compiler/meshc/src/cluster.rs` and `compiler/meshc/tests/e2e_m044_s03.rs` do not exist, so the task's planned scaffold rail cannot run honestly.

Because the public clustered scaffold is supposed to bootstrap inspection through the runtime-owned `meshc cluster` surface rather than app-defined helper routes, shipping only the template here would have produced a dishonest slice state: users could generate a clustered-looking app, but the operator/verification path the plan depends on still would not exist. I recorded the current proof-truth signal in `.gsd/KNOWLEDGE.md` and wrote the blocker handoff instead of fabricating a partial scaffold implementation on top of the missing T04 seam.

## Verification

I verified the blocker directly against the planned T05 rails and the current runtime/CLI seams. The tooling filter named by the task still exits 0 while running `0 tests`, the dedicated S03 e2e target still does not exist, and the static seam check confirms both the missing files and the pre-blocker operator-query transport shape.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture` | 0 | ❌ fail | 4668ms |
| 2 | `cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture` | 101 | ❌ fail | 1053ms |
| 3 | `if [ -f compiler/meshc/src/cluster.rs ]; then echo cluster_rs_present; else echo cluster_rs_missing; fi; if [ -f compiler/meshc/tests/e2e_m044_s03.rs ]; then echo e2e_present; else echo e2e_missing; fi; rg -n "pub fn query_operator_status|let state = node_state\(|sessions\.get\(target\)|target_not_connected" compiler/mesh-rt/src/dist/operator.rs; rg -n "Init \{|Commands::Init|mesh_pkg::scaffold_project|--clustered" compiler/meshc/src/main.rs compiler/mesh-pkg/src/scaffold.rs` | 0 | ✅ pass | 289ms |


## Deviations

I did not modify `compiler/mesh-pkg/src/scaffold.rs`, `compiler/mesh-pkg/src/lib.rs`, `compiler/meshc/src/main.rs`, `compiler/meshc/tests/tooling_e2e.rs`, or create `compiler/meshc/tests/e2e_m044_s03.rs`. The written task plan assumes the truthful T04 cluster CLI surface already exists, and that assumption is still false in the current tree.

## Known Issues

T05 is still blocked on the unfinished T04 dependency chain. Until the runtime has a truthful non-session operator inspection path and `meshc cluster` exists as a real public command/test surface, `meshc init --clustered` cannot honestly ship the public scaffold story described by S03.

## Files Created/Modified

- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M044/slices/S03/tasks/T05-SUMMARY.md`


## Deviations
I did not modify `compiler/mesh-pkg/src/scaffold.rs`, `compiler/mesh-pkg/src/lib.rs`, `compiler/meshc/src/main.rs`, `compiler/meshc/tests/tooling_e2e.rs`, or create `compiler/meshc/tests/e2e_m044_s03.rs`. The written task plan assumes the truthful T04 cluster CLI surface already exists, and that assumption is still false in the current tree.

## Known Issues
T05 is still blocked on the unfinished T04 dependency chain. Until the runtime has a truthful non-session operator inspection path and `meshc cluster` exists as a real public command/test surface, `meshc init --clustered` cannot honestly ship the public scaffold story described by S03.
