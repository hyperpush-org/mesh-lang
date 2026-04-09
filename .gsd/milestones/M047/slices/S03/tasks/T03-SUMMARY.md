---
id: T03
parent: S03
milestone: M047
provides: []
requires: []
affects: []
key_files: [".gsd/milestones/M047/slices/S03/tasks/T03-SUMMARY.md"]
key_decisions: ["Do not synthesize clustered route registrations or generated route shims until `HTTP.clustered(...)` exists as compiler-known metadata threaded from `InferCtx`/`TypeckResult` into the lowerer."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "I verified the blocker directly with repo searches and code inspection. `rg -n "m047_s03" compiler -g '*.rs'` returned no tests, `rg -n "HTTP\.clustered|clustered_route|clustered_wrapper|route shim" compiler/mesh-typeck compiler/mesh-codegen compiler/meshc -g '*.rs'` returned no implementation hits, and the inspected `PreparedBuild` / declared-handler planning path still only handles ordinary clustered declarations."
completed_at: 2026-04-01T08:16:49.500Z
blocker_discovered: true
---

# T03: Stopped T03 after confirming the missing `HTTP.clustered(...)` metadata seam still blocks clustered route shim and registration work.

> Stopped T03 after confirming the missing `HTTP.clustered(...)` metadata seam still blocks clustered route shim and registration work.

## What Happened
---
id: T03
parent: S03
milestone: M047
key_files:
  - .gsd/milestones/M047/slices/S03/tasks/T03-SUMMARY.md
key_decisions:
  - Do not synthesize clustered route registrations or generated route shims until `HTTP.clustered(...)` exists as compiler-known metadata threaded from `InferCtx`/`TypeckResult` into the lowerer.
duration: ""
verification_result: mixed
completed_at: 2026-04-01T08:16:49.500Z
blocker_discovered: true
---

# T03: Stopped T03 after confirming the missing `HTTP.clustered(...)` metadata seam still blocks clustered route shim and registration work.

**Stopped T03 after confirming the missing `HTTP.clustered(...)` metadata seam still blocks clustered route shim and registration work.**

## What Happened

I read the slice/task contracts and inspected the current clustered execution seam in `compiler/meshc/src/main.rs`, `compiler/mesh-pkg/src/manifest.rs`, `compiler/mesh-codegen/src/declared.rs`, `compiler/mesh-codegen/src/codegen/mod.rs`, `compiler/mesh-codegen/src/codegen/expr.rs`, `compiler/mesh-codegen/src/mir/lower.rs`, plus the typeck surfaces in `compiler/mesh-typeck/src/unify.rs`, `compiler/mesh-typeck/src/lib.rs`, and `compiler/mesh-typeck/src/infer.rs`. The blocker is concrete: the tree still has no compiler-known `HTTP.clustered(...)` surface, no metadata map parallel to `overloaded_call_targets`, and no route-wrapper entry in `PreparedBuild.clustered_execution_plan`. Because T03’s planned work depends on that missing handoff, I stopped without changing source and wrote a durable summary that names the exact resume seam for the next unit.

## Verification

I verified the blocker directly with repo searches and code inspection. `rg -n "m047_s03" compiler -g '*.rs'` returned no tests, `rg -n "HTTP\.clustered|clustered_route|clustered_wrapper|route shim" compiler/mesh-typeck compiler/mesh-codegen compiler/meshc -g '*.rs'` returned no implementation hits, and the inspected `PreparedBuild` / declared-handler planning path still only handles ordinary clustered declarations.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `rg -n "m047_s03" compiler -g '*.rs'` | 1 | ❌ fail | 0ms |
| 2 | `rg -n "HTTP\.clustered|clustered_route|clustered_wrapper|route shim" compiler/mesh-typeck compiler/mesh-codegen compiler/meshc -g '*.rs'` | 1 | ❌ fail | 0ms |
| 3 | `rg -n "clustered_execution_plan|PreparedBuild|runtime_registration_name|replication_count" compiler/meshc/src/main.rs compiler/mesh-codegen/src/declared.rs compiler/mesh-codegen/src/codegen/mod.rs -g '*.rs'` | 0 | ✅ pass | 0ms |


## Deviations

Did not implement T03 code because the upstream T02 compiler metadata seam for `HTTP.clustered(...)` is still absent in the current tree. Starting route-shim/registration work without that handoff would require backfilling T02 first, which is beyond a minor local adaptation for this unit.

## Known Issues

`HTTP.clustered(...)` remains unimplemented in typeck, there are no `m047_s03` rails in `compiler/`, and `PreparedBuild.clustered_execution_plan` still only reflects source/manifest clustered declarations. T03 is blocked until the missing T02 handoff lands.

## Files Created/Modified

- `.gsd/milestones/M047/slices/S03/tasks/T03-SUMMARY.md`


## Deviations
Did not implement T03 code because the upstream T02 compiler metadata seam for `HTTP.clustered(...)` is still absent in the current tree. Starting route-shim/registration work without that handoff would require backfilling T02 first, which is beyond a minor local adaptation for this unit.

## Known Issues
`HTTP.clustered(...)` remains unimplemented in typeck, there are no `m047_s03` rails in `compiler/`, and `PreparedBuild.clustered_execution_plan` still only reflects source/manifest clustered declarations. T03 is blocked until the missing T02 handoff lands.
