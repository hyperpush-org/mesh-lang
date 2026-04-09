---
id: T05
parent: S01
milestone: M044
provides: []
requires: []
affects: []
key_files: ["cluster-proof/mesh.toml", "cluster-proof/work_continuity.mpl", "cluster-proof/work.mpl", "cluster-proof/work_legacy.mpl", "cluster-proof/main.mpl", "cluster-proof/tests/work.test.mpl", "compiler/meshc/tests/e2e_m044_s01.rs", ".gsd/milestones/M044/slices/S01/tasks/T05-SUMMARY.md"]
key_decisions: ["D191: consume builtin ContinuityAuthorityStatus/ContinuityRecord/ContinuitySubmitDecision directly in cluster-proof and keep only HTTP request/response payload wrappers as local Json-derived structs."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the new named Rust rail with `cargo test -p meshc --test e2e_m044_s01 cluster_proof_ -- --nocapture`, then reran the task-plan commands `cargo run -q -p meshc -- build cluster-proof` and `cargo run -q -p meshc -- test cluster-proof/tests`. I also ran the negative grep from the task plan and confirmed the deprecated stringly helper names are absent from `cluster-proof/work_continuity.mpl`."
completed_at: 2026-03-29T18:52:09.577Z
blocker_discovered: false
---

# T05: Rewrote `cluster-proof` onto typed continuity values, added a real clustered manifest, and pinned the rewrite to named `cluster_proof_` proof rails.

> Rewrote `cluster-proof` onto typed continuity values, added a real clustered manifest, and pinned the rewrite to named `cluster_proof_` proof rails.

## What Happened
---
id: T05
parent: S01
milestone: M044
key_files:
  - cluster-proof/mesh.toml
  - cluster-proof/work_continuity.mpl
  - cluster-proof/work.mpl
  - cluster-proof/work_legacy.mpl
  - cluster-proof/main.mpl
  - cluster-proof/tests/work.test.mpl
  - compiler/meshc/tests/e2e_m044_s01.rs
  - .gsd/milestones/M044/slices/S01/tasks/T05-SUMMARY.md
key_decisions:
  - D191: consume builtin ContinuityAuthorityStatus/ContinuityRecord/ContinuitySubmitDecision directly in cluster-proof and keep only HTTP request/response payload wrappers as local Json-derived structs.
duration: ""
verification_result: passed
completed_at: 2026-03-29T18:52:09.578Z
blocker_discovered: false
---

# T05: Rewrote `cluster-proof` onto typed continuity values, added a real clustered manifest, and pinned the rewrite to named `cluster_proof_` proof rails.

**Rewrote `cluster-proof` onto typed continuity values, added a real clustered manifest, and pinned the rewrite to named `cluster_proof_` proof rails.**

## What Happened

Added a real `cluster-proof/mesh.toml` clustered manifest, removed the app-owned runtime continuity JSON shim path, and rewrote the proof app to consume builtin `ContinuityAuthorityStatus`, `ContinuityRecord`, and `ContinuitySubmitDecision` values directly. I removed the duplicate local record type from `work.mpl`, updated the legacy probe and main entrypoint to use the builtin continuity shapes, rewrote the package tests around typed continuity values plus HTTP-boundary JSON payloads, and added named `cluster_proof_` coverage in `compiler/meshc/tests/e2e_m044_s01.rs` so the consumer rewrite is now proven by explicit compiler/package rails.

## Verification

Verified the new named Rust rail with `cargo test -p meshc --test e2e_m044_s01 cluster_proof_ -- --nocapture`, then reran the task-plan commands `cargo run -q -p meshc -- build cluster-proof` and `cargo run -q -p meshc -- test cluster-proof/tests`. I also ran the negative grep from the task plan and confirmed the deprecated stringly helper names are absent from `cluster-proof/work_continuity.mpl`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m044_s01 cluster_proof_ -- --nocapture` | 0 | ✅ pass | 22215ms |
| 2 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 12481ms |
| 3 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 18082ms |
| 4 | `rg -n 'ContinuityAuthorityStatus\.from_json|ContinuitySubmitDecision\.from_json|WorkRequestRecord\.from_json|parse_authority_status_json|parse_continuity_submit_response|parse_continuity_record' cluster-proof/work_continuity.mpl` | 1 | ✅ pass | 41ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `cluster-proof/mesh.toml`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/work_legacy.mpl`
- `cluster-proof/main.mpl`
- `cluster-proof/tests/work.test.mpl`
- `compiler/meshc/tests/e2e_m044_s01.rs`
- `.gsd/milestones/M044/slices/S01/tasks/T05-SUMMARY.md`


## Deviations
None.

## Known Issues
None.
