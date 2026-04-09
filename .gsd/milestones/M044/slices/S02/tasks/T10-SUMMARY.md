---
id: T10
parent: S02
milestone: M044
provides: []
requires: []
affects: []
key_files: [".gsd/KNOWLEDGE.md", ".gsd/milestones/M044/slices/S02/tasks/T10-SUMMARY.md"]
key_decisions: ["Recorded that T10 should resume from the `cluster-proof` app seam, not from `mesh-rt`, because `mesh_continuity_submit_declared_work(...) -> node::submit_declared_work(...)` already exists locally.", "Stopped without editing code once the context-budget warning fired, so the next unit inherits an honest compile stop-point instead of a half-applied rewrite."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the stop-point with a direct `meshc build cluster-proof` replay and two targeted source checks. I did not run the full `bash scripts/verify-m044-s02.sh` rail after the context-budget warning because the package still fails earlier at `meshc build cluster-proof`."
completed_at: 2026-03-29T22:19:09.740Z
blocker_discovered: false
---

# T10: Captured the real T10 stop-point: runtime-owned declared work exists locally, but `cluster-proof` still fails before the S02 verifier can reach the new rails.

> Captured the real T10 stop-point: runtime-owned declared work exists locally, but `cluster-proof` still fails before the S02 verifier can reach the new rails.

## What Happened
---
id: T10
parent: S02
milestone: M044
key_files:
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M044/slices/S02/tasks/T10-SUMMARY.md
key_decisions:
  - Recorded that T10 should resume from the `cluster-proof` app seam, not from `mesh-rt`, because `mesh_continuity_submit_declared_work(...) -> node::submit_declared_work(...)` already exists locally.
  - Stopped without editing code once the context-budget warning fired, so the next unit inherits an honest compile stop-point instead of a half-applied rewrite.
duration: ""
verification_result: mixed
completed_at: 2026-03-29T22:19:09.741Z
blocker_discovered: false
---

# T10: Captured the real T10 stop-point: runtime-owned declared work exists locally, but `cluster-proof` still fails before the S02 verifier can reach the new rails.

**Captured the real T10 stop-point: runtime-owned declared work exists locally, but `cluster-proof` still fails before the S02 verifier can reach the new rails.**

## What Happened

Loaded the T10 contract and the earlier handoffs, then checked the live compiler/runtime/app seams before editing. The runtime side is already present locally: `mesh_continuity_submit_declared_work(...)` delegates to `compiler/mesh-rt/src/dist/node.rs::submit_declared_work(...)`, which resolves manifest-approved declared handlers, computes placement internally, submits continuity state, and dispatches through `mesh_actor_spawn` / `mesh_node_spawn`. Reproducing `cargo run -q -p meshc -- build cluster-proof` showed the real remaining seam is the proof app: `cluster-proof/work_continuity.mpl` still calls an undefined `declared_work_target()` helper, and the build then stops again in `cluster-proof/main.mpl` around the authority-status/router setup block. The context-budget warning fired during that read/repro pass, so I stopped without starting edits, recorded the resume point in `.gsd/KNOWLEDGE.md`, and wrote the task summary instead of leaving a half-applied rewrite.

## Verification

Verified the stop-point with a direct `meshc build cluster-proof` replay and two targeted source checks. I did not run the full `bash scripts/verify-m044-s02.sh` rail after the context-budget warning because the package still fails earlier at `meshc build cluster-proof`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- build cluster-proof` | 1 | ❌ fail | 0ms |
| 2 | `rg -n "declared_work_target\(" -S .` | 0 | ✅ pass | 0ms |
| 3 | `rg -n "mesh_continuity_submit_declared_work|submit_declared_work\(|declared_work_placement|spawn_declared_work_local|spawn_declared_work_remote" compiler/mesh-rt/src/dist/continuity.rs compiler/mesh-rt/src/dist/node.rs` | 0 | ✅ pass | 0ms |


## Deviations

Did not perform the planned code rewrite. I stopped at the reproduced compile seam when the context-budget warning fired and wrote the resume artifacts instead.

## Known Issues

`cargo run -q -p meshc -- build cluster-proof` still fails on `declared_work_target()` being undefined in `cluster-proof/work_continuity.mpl`. The same build also stops in `cluster-proof/main.mpl` around the authority-log / router setup block with `expected Response, found String` and `undefined variable: router` diagnostics. Because of that earlier compile failure, the named `m044_s02_declared_work_` / `m044_s02_cluster_proof_` rails were not replayed in this unit.

## Files Created/Modified

- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M044/slices/S02/tasks/T10-SUMMARY.md`


## Deviations
Did not perform the planned code rewrite. I stopped at the reproduced compile seam when the context-budget warning fired and wrote the resume artifacts instead.

## Known Issues
`cargo run -q -p meshc -- build cluster-proof` still fails on `declared_work_target()` being undefined in `cluster-proof/work_continuity.mpl`. The same build also stops in `cluster-proof/main.mpl` around the authority-log / router setup block with `expected Response, found String` and `undefined variable: router` diagnostics. Because of that earlier compile failure, the named `m044_s02_declared_work_` / `m044_s02_cluster_proof_` rails were not replayed in this unit.
