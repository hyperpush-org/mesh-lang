---
id: T05
parent: S03
milestone: M047
provides: []
requires: []
affects: []
key_files: [".gsd/milestones/M047/slices/S03/tasks/T05-SUMMARY.md", "compiler/meshc/tests/e2e_stdlib.rs", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Do not fabricate the M047/S03 proof rail while `HTTP.clustered(...)` is still missing from the compiler/typeck/runtime path; keep the retained M032 controls green, then fail closed on the absent M047 target."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "I reran the exact task verification commands. `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` still fails closed with `no test target named e2e_m047_s03`. `cargo test -p meshc --test e2e_m047_s02 -- --nocapture` passes, proving ordinary `@cluster` continuity/runtime-name/count truth still works. After embedding stable Mesh sources in `compiler/meshc/tests/e2e_stdlib.rs`, both retained M032 controls now pass: `cargo test -p meshc --test e2e_stdlib e2e_m032_route_bare_handler_control -- --nocapture` and `cargo test -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture`. I also verified the blocker directly with `rg -n 'HTTP\.clustered|http_clustered' compiler/mesh-typeck/src compiler/mesh-codegen/src compiler/mesh-rt/src compiler/meshc/tests -g '*.rs'`, which returned no matches."
completed_at: 2026-04-01T08:28:52.288Z
blocker_discovered: true
---

# T05: Restored the self-contained M032 route-limit control rails and confirmed T05 is still blocked because the tree still has no `HTTP.clustered(...)` compiler/runtime surface or `e2e_m047_s03` target.

> Restored the self-contained M032 route-limit control rails and confirmed T05 is still blocked because the tree still has no `HTTP.clustered(...)` compiler/runtime surface or `e2e_m047_s03` target.

## What Happened
---
id: T05
parent: S03
milestone: M047
key_files:
  - .gsd/milestones/M047/slices/S03/tasks/T05-SUMMARY.md
  - compiler/meshc/tests/e2e_stdlib.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Do not fabricate the M047/S03 proof rail while `HTTP.clustered(...)` is still missing from the compiler/typeck/runtime path; keep the retained M032 controls green, then fail closed on the absent M047 target.
duration: ""
verification_result: mixed
completed_at: 2026-04-01T08:28:52.289Z
blocker_discovered: true
---

# T05: Restored the self-contained M032 route-limit control rails and confirmed T05 is still blocked because the tree still has no `HTTP.clustered(...)` compiler/runtime surface or `e2e_m047_s03` target.

**Restored the self-contained M032 route-limit control rails and confirmed T05 is still blocked because the tree still has no `HTTP.clustered(...)` compiler/runtime surface or `e2e_m047_s03` target.**

## What Happened

I verified the T05 assumption before changing code and found that the clustered HTTP route path still does not exist locally: the compiler/type environment still only exposes ordinary `HTTP.route` / `HTTP.on_*` handlers, the lowerer still maps those names directly to `mesh_http_route*`, the HTTP runtime still executes matched handlers through the plain `call_handler(...)` path, and the named proof rail `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` still fails because the target does not exist. While replaying the retained M032 controls from the task verification list, I found a separate harness problem: `compiler/meshc/tests/e2e_stdlib.rs` depended on `include_str!("../../../.tmp/m032-s01/.../main.mpl")`, so the whole target failed to compile whenever that transient fixture tree was absent. I fixed that drift by embedding stable Mesh sources for the bare-handler and closure-route controls directly in the Rust test file. After that repair, both retained M032 runtime-limit tests passed again and the existing M047/S02 ordinary `@cluster` continuity rail still passed. The new M047/S03 proof rail remains blocked until the missing `HTTP.clustered(...)` typing/lowering/runtime seam actually lands.

## Verification

I reran the exact task verification commands. `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` still fails closed with `no test target named e2e_m047_s03`. `cargo test -p meshc --test e2e_m047_s02 -- --nocapture` passes, proving ordinary `@cluster` continuity/runtime-name/count truth still works. After embedding stable Mesh sources in `compiler/meshc/tests/e2e_stdlib.rs`, both retained M032 controls now pass: `cargo test -p meshc --test e2e_stdlib e2e_m032_route_bare_handler_control -- --nocapture` and `cargo test -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture`. I also verified the blocker directly with `rg -n 'HTTP\.clustered|http_clustered' compiler/mesh-typeck/src compiler/mesh-codegen/src compiler/mesh-rt/src compiler/meshc/tests -g '*.rs'`, which returned no matches.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` | 101 | ❌ fail | 505ms |
| 2 | `cargo test -p meshc --test e2e_m047_s02 -- --nocapture` | 0 | ✅ pass | 9725ms |
| 3 | `cargo test -p meshc --test e2e_stdlib e2e_m032_route_bare_handler_control -- --nocapture` | 0 | ✅ pass | 5982ms |
| 4 | `cargo test -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture` | 0 | ✅ pass | 5897ms |
| 5 | `rg -n 'HTTP\.clustered|http_clustered' compiler/mesh-typeck/src compiler/mesh-codegen/src compiler/mesh-rt/src compiler/meshc/tests -g '*.rs'` | 1 | ✅ pass | 50ms |


## Deviations

T05 assumed T02–T04 had already landed the clustered HTTP route implementation and only needed a live proof rail. In the current checkout that upstream implementation still does not exist, so I did not add a fake `e2e_m047_s03` target. I only repaired the retained M032 control harness drift so the non-blocked verification surfaces stayed truthful.

## Known Issues

`HTTP.clustered(...)` is still absent from the compiler/typeck/runtime sources, `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` still fails with `no test target named e2e_m047_s03`, and the HTTP lowering/runtime path still only models ordinary `HTTP.route` / `HTTP.on_*` handlers that execute through the direct `call_handler(...)` path.

## Files Created/Modified

- `.gsd/milestones/M047/slices/S03/tasks/T05-SUMMARY.md`
- `compiler/meshc/tests/e2e_stdlib.rs`
- `.gsd/KNOWLEDGE.md`


## Deviations
T05 assumed T02–T04 had already landed the clustered HTTP route implementation and only needed a live proof rail. In the current checkout that upstream implementation still does not exist, so I did not add a fake `e2e_m047_s03` target. I only repaired the retained M032 control harness drift so the non-blocked verification surfaces stayed truthful.

## Known Issues
`HTTP.clustered(...)` is still absent from the compiler/typeck/runtime sources, `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` still fails with `no test target named e2e_m047_s03`, and the HTTP lowering/runtime path still only models ordinary `HTTP.route` / `HTTP.on_*` handlers that execute through the direct `call_handler(...)` path.
