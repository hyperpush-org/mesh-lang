---
id: T04
parent: S05
milestone: M047
provides: []
requires: []
affects: []
key_files: [".gsd/milestones/M047/slices/S05/tasks/T04-SUMMARY.md"]
key_decisions: ["Do not rewrite `meshc init --clustered`, `tiny-cluster/`, or `cluster-proof/` onto the no-ceremony contract until `@cluster pub fn add() -> Int` builds successfully; the current tree still fails in declared-work wrapper codegen before any route-free rebaseline can be verified."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the blocker directly with a minimal temp package build under `.tmp/m047-s05-t04-probe/`: `cargo run -q -p meshc -- build .tmp/m047-s05-t04-probe --emit-llvm` still fails with the declared-work wrapper continuity-argument error for a no-ceremony `@cluster` function. I also reran the existing scaffold unit rail, `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture`, which still passes and therefore confirms the stale route-free contract is what the tree currently proves. I did not run the rest of the T04 verification bundle because the prerequisite compiler capability is absent, so the route-free rebaseline cannot be implemented or validated honestly yet."
completed_at: 2026-04-01T16:24:23.890Z
blocker_discovered: true
---

# T04: Confirmed that T04 is still blocked because the no-ceremony `@cluster` wrapper seam has not landed, so the route-free scaffold/example cutover cannot be verified honestly.

> Confirmed that T04 is still blocked because the no-ceremony `@cluster` wrapper seam has not landed, so the route-free scaffold/example cutover cannot be verified honestly.

## What Happened
---
id: T04
parent: S05
milestone: M047
key_files:
  - .gsd/milestones/M047/slices/S05/tasks/T04-SUMMARY.md
key_decisions:
  - Do not rewrite `meshc init --clustered`, `tiny-cluster/`, or `cluster-proof/` onto the no-ceremony contract until `@cluster pub fn add() -> Int` builds successfully; the current tree still fails in declared-work wrapper codegen before any route-free rebaseline can be verified.
duration: ""
verification_result: passed
completed_at: 2026-04-01T16:24:23.891Z
blocker_discovered: true
---

# T04: Confirmed that T04 is still blocked because the no-ceremony `@cluster` wrapper seam has not landed, so the route-free scaffold/example cutover cannot be verified honestly.

**Confirmed that T04 is still blocked because the no-ceremony `@cluster` wrapper seam has not landed, so the route-free scaffold/example cutover cannot be verified honestly.**

## What Happened

Read the T04 plan, the slice plan, the prior T02/T03 summaries, the task-summary template, and the local route-free scaffold/example/test surfaces. The public route-free contract is still stale in exactly the places the plan named: `compiler/mesh-pkg/src/scaffold.rs`, `compiler/meshc/tests/tooling_e2e.rs`, `compiler/meshc/tests/support/m046_route_free.rs`, `tiny-cluster/`, and `cluster-proof/` still teach `@cluster pub fn execute_declared_work(_request_key :: String, _attempt_id :: String)` and the `Work.execute_declared_work` runtime-name guidance. Before rewriting those surfaces, I verified the prerequisite compiler seam in the current checkout. It is still missing: a minimal temp package containing `@cluster pub fn add() -> Int do 1 + 1 end` fails with `declared work wrapper '__declared_work_work_add' expected request_key and attempt_id arguments`, and `compiler/mesh-codegen/src/codegen/expr.rs` still contains that exact hard failure in the declared-work wrapper path. That means the task contract's opening assumption ('Once T03 lands...') is still false locally. I stopped there instead of rewriting scaffold/examples onto a contract the compiler cannot build, because that would only create text drift and red verification rails.

## Verification

Verified the blocker directly with a minimal temp package build under `.tmp/m047-s05-t04-probe/`: `cargo run -q -p meshc -- build .tmp/m047-s05-t04-probe --emit-llvm` still fails with the declared-work wrapper continuity-argument error for a no-ceremony `@cluster` function. I also reran the existing scaffold unit rail, `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture`, which still passes and therefore confirms the stale route-free contract is what the tree currently proves. I did not run the rest of the T04 verification bundle because the prerequisite compiler capability is absent, so the route-free rebaseline cannot be implemented or validated honestly yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- build .tmp/m047-s05-t04-probe --emit-llvm` | 1 | ✅ pass | 7215ms |
| 2 | `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture` | 0 | ✅ pass | 889ms |


## Deviations

The task plan assumed the T03 compiler/runtime prerequisite had already landed. Local reality did not match, so I had to stop at the prerequisite verification step instead of rewriting scaffold/examples onto an unsupported source contract.

## Known Issues

Zero-ceremony `@cluster` declared work is still unsupported in the current tree. As long as `@cluster pub fn add() -> Int` fails in declared-work wrapper codegen, the route-free scaffold, `tiny-cluster/`, `cluster-proof/`, and their exact-string rails cannot be rebaselined honestly to the intended public contract.

## Files Created/Modified

- `.gsd/milestones/M047/slices/S05/tasks/T04-SUMMARY.md`


## Deviations
The task plan assumed the T03 compiler/runtime prerequisite had already landed. Local reality did not match, so I had to stop at the prerequisite verification step instead of rewriting scaffold/examples onto an unsupported source contract.

## Known Issues
Zero-ceremony `@cluster` declared work is still unsupported in the current tree. As long as `@cluster pub fn add() -> Int` fails in declared-work wrapper codegen, the route-free scaffold, `tiny-cluster/`, `cluster-proof/`, and their exact-string rails cannot be rebaselined honestly to the intended public contract.
