---
id: T05
parent: S04
milestone: M047
provides: []
requires: []
affects: []
key_files: ["README.md", "website/docs/docs/getting-started/clustered-example/index.md", "website/docs/docs/tooling/index.md", "website/docs/docs/distributed-proof/index.md", "website/docs/docs/distributed/index.md", "tiny-cluster/README.md", "compiler/mesh-rt/src/dist/node.rs", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M047/slices/S04/tasks/T05-SUMMARY.md"]
key_decisions: ["Kept migration guidance explicit but avoided the literal `[cluster]` token in public runbooks because the historical M046 docs rails treat that exact legacy marker as drift even inside negative examples.", "Startup automatic recovery now reuses the startup single-node replica relaxation for `startup::...` request keys before resubmitting declared work, so promoted single-node standby recovery stays truthful instead of rejecting itself for missing a replica."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Rebuilt the VitePress docs site and reran the two red historical rails from the gate. `cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture` passed with the updated closeout-rail/runbook wording, `cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture` passed with the startup automatic recovery fix, and `cargo test -p mesh-rt startup_automatic_recovery_relaxes_single_node_required_replica_count -- --nocapture` passed as the focused regression test. `npm --prefix website run build` passed and rebuilt the docs site successfully."
completed_at: 2026-04-01T10:12:22.631Z
blocker_discovered: false
---

# T05: Rewrote the public clustered docs to the source-first `@cluster` model and repaired startup failover recovery so the historical cutover rails stay green.

> Rewrote the public clustered docs to the source-first `@cluster` model and repaired startup failover recovery so the historical cutover rails stay green.

## What Happened
---
id: T05
parent: S04
milestone: M047
key_files:
  - README.md
  - website/docs/docs/getting-started/clustered-example/index.md
  - website/docs/docs/tooling/index.md
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/distributed/index.md
  - tiny-cluster/README.md
  - compiler/mesh-rt/src/dist/node.rs
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M047/slices/S04/tasks/T05-SUMMARY.md
key_decisions:
  - Kept migration guidance explicit but avoided the literal `[cluster]` token in public runbooks because the historical M046 docs rails treat that exact legacy marker as drift even inside negative examples.
  - Startup automatic recovery now reuses the startup single-node replica relaxation for `startup::...` request keys before resubmitting declared work, so promoted single-node standby recovery stays truthful instead of rejecting itself for missing a replica.
duration: ""
verification_result: passed
completed_at: 2026-04-01T10:12:22.632Z
blocker_discovered: false
---

# T05: Rewrote the public clustered docs to the source-first `@cluster` model and repaired startup failover recovery so the historical cutover rails stay green.

**Rewrote the public clustered docs to the source-first `@cluster` model and repaired startup failover recovery so the historical cutover rails stay green.**

## What Happened

Updated the public clustered surfaces so README, the clustered-example page, tooling docs, the distributed proof page, and the distributed overview all teach one route-free clustered contract: clustered work is declared in source with `@cluster`, `mesh.toml` stays package-only, operators inspect runtime truth through `meshc cluster status|continuity|diagnostics`, and `HTTP.clustered(...)` is still explicitly unshipped. Added migration wording for older packages, but kept that wording fail-safe for the historical docs rails by describing the retired manifest form generically instead of repeating the literal legacy marker they ban. The verification gate also exposed a deterministic runtime regression outside the markdown itself: after standby promotion, startup automatic recovery was resubmitting with the raw declared-handler replica requirement and rejecting itself with `replica_required_unavailable`. Narrowed that seam in `compiler/mesh-rt/src/dist/node.rs` so startup-key automatic recovery reuses the existing single-node startup replica relaxation before calling `submit_declared_work(...)`, then added a focused unit test and recorded the rule in `.gsd/KNOWLEDGE.md`.

## Verification

Rebuilt the VitePress docs site and reran the two red historical rails from the gate. `cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture` passed with the updated closeout-rail/runbook wording, `cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture` passed with the startup automatic recovery fix, and `cargo test -p mesh-rt startup_automatic_recovery_relaxes_single_node_required_replica_count -- --nocapture` passed as the focused regression test. `npm --prefix website run build` passed and rebuilt the docs site successfully.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-rt startup_automatic_recovery_relaxes_single_node_required_replica_count -- --nocapture` | 0 | ✅ pass | 21250ms |
| 2 | `cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture` | 0 | ✅ pass | 1070ms |
| 3 | `cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture` | 0 | ✅ pass | 28900ms |
| 4 | `npm --prefix website run build` | 0 | ✅ pass | 14150ms |


## Deviations

The task plan was docs-only, but the verification gate exposed a deterministic runtime-owned failover regression in `compiler/mesh-rt/src/dist/node.rs`. I fixed that narrow startup automatic recovery seam in the same task because the red historical rail was part of this task’s required verification surface and the failure was local, reproducible, and not plan-invalidating.

## Known Issues

None.

## Files Created/Modified

- `README.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `tiny-cluster/README.md`
- `compiler/mesh-rt/src/dist/node.rs`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M047/slices/S04/tasks/T05-SUMMARY.md`


## Deviations
The task plan was docs-only, but the verification gate exposed a deterministic runtime-owned failover regression in `compiler/mesh-rt/src/dist/node.rs`. I fixed that narrow startup automatic recovery seam in the same task because the red historical rail was part of this task’s required verification surface and the failure was local, reproducible, and not plan-invalidating.

## Known Issues
None.
