---
id: T04
parent: S04
milestone: M044
provides: []
requires: []
affects: []
key_files: [".gsd/KNOWLEDGE.md", ".gsd/milestones/M044/slices/S04/tasks/T04-SUMMARY.md"]
key_decisions: ["Treat the missing S04 runtime/e2e rail as a blocker instead of shipping auto-only docs and verifier wording against an unproven local contract."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified that the current proof app still builds and its package tests still pass with `cargo run -q -p meshc -- build cluster-proof` and `cargo run -q -p meshc -- test cluster-proof/tests`. Then checked the intended S04 acceptance seam directly: `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture` still fails because the target does not exist, and `cargo test -p meshc --test e2e_m043_s03 e2e_m043_s03_same_image_failover_fences_stale_primary -- --nocapture` still fails in the current tree. A targeted `rg` over the runtime continuity/node code confirmed the disconnect path only marks/degrades records and leaves `promote_authority()` uncalled from live runtime flow."
completed_at: 2026-03-30T03:53:49.552Z
blocker_discovered: true
---

# T04: Recorded that T04 is blocked because the S04 auto-promotion/auto-resume proof rail is still missing locally.

> Recorded that T04 is blocked because the S04 auto-promotion/auto-resume proof rail is still missing locally.

## What Happened
---
id: T04
parent: S04
milestone: M044
key_files:
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M044/slices/S04/tasks/T04-SUMMARY.md
key_decisions:
  - Treat the missing S04 runtime/e2e rail as a blocker instead of shipping auto-only docs and verifier wording against an unproven local contract.
duration: ""
verification_result: mixed
completed_at: 2026-03-30T03:53:49.554Z
blocker_discovered: true
---

# T04: Recorded that T04 is blocked because the S04 auto-promotion/auto-resume proof rail is still missing locally.

**Recorded that T04 is blocked because the S04 auto-promotion/auto-resume proof rail is still missing locally.**

## What Happened

Started from the T04 contract, the S04 slice plan, and the T02/T03 summaries. Confirmed that `cluster-proof/main.mpl` already dropped `/promote`, but the remaining T04 surface is still inconsistent: `cluster-proof/work_continuity.mpl` still carries dead promotion-era helpers, package tests and docs still teach the manual failover story, and there is no `compiler/meshc/tests/e2e_m044_s04.rs` target for the planned auto-only same-image proof. Checked the actual runtime seam before rewriting docs: `compiler/mesh-rt/src/dist/node.rs::handle_node_disconnect(...)` only marks owner-loss/degraded continuity records and never invokes `promote_authority()`, while `submit_declared_work(...)` still only dispatches on submit and has no runtime-owned auto-resume redispatch path beyond the existing same-key retry seam. Verified that mismatch with commands: the expected S04 e2e target is still missing and the old same-image failover rail is still red. Because the repo cannot currently prove bounded automatic promotion plus automatic recovery, I did not rewrite the public docs/runbooks or add a fake `verify-m044-s04.sh`; instead I recorded the blocker and appended the exact seam to `.gsd/KNOWLEDGE.md` for the next executor.

## Verification

Verified that the current proof app still builds and its package tests still pass with `cargo run -q -p meshc -- build cluster-proof` and `cargo run -q -p meshc -- test cluster-proof/tests`. Then checked the intended S04 acceptance seam directly: `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture` still fails because the target does not exist, and `cargo test -p meshc --test e2e_m043_s03 e2e_m043_s03_same_image_failover_fences_stale_primary -- --nocapture` still fails in the current tree. A targeted `rg` over the runtime continuity/node code confirmed the disconnect path only marks/degrades records and leaves `promote_authority()` uncalled from live runtime flow.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 22500ms |
| 2 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 17000ms |
| 3 | `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture` | 101 | ❌ fail | 405500ms |
| 4 | `cargo test -p meshc --test e2e_m043_s03 e2e_m043_s03_same_image_failover_fences_stale_primary -- --nocapture` | 101 | ❌ fail | 401000ms |
| 5 | `rg -n "promote_authority\(|mark_owner_loss_records_for_node_loss\(|degrade_replication_health_for_node_loss\(|continuity_owner_loss_recovery_eligible\(" compiler/mesh-rt/src/dist/continuity.rs compiler/mesh-rt/src/dist/node.rs` | 0 | ✅ pass | 1000ms |


## Deviations

Did not edit the planned proof-app/docs/verifier files. Rewriting them now would overclaim an auto-only failover contract that the local runtime/e2e surface still cannot prove.

## Known Issues

`compiler/meshc/tests/e2e_m044_s04.rs` is still missing. `compiler/mesh-rt/src/dist/node.rs::handle_node_disconnect(...)` still lacks a runtime-owned call into the internal promotion seam. `submit_declared_work(...)` still has no runtime-owned auto-resume redispatch path, so the healthy S04 path cannot yet finish without the old retry-era seam. Public docs remain stale because the truthful S04 replacement rail is not ready to publish.

## Files Created/Modified

- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M044/slices/S04/tasks/T04-SUMMARY.md`


## Deviations
Did not edit the planned proof-app/docs/verifier files. Rewriting them now would overclaim an auto-only failover contract that the local runtime/e2e surface still cannot prove.

## Known Issues
`compiler/meshc/tests/e2e_m044_s04.rs` is still missing. `compiler/mesh-rt/src/dist/node.rs::handle_node_disconnect(...)` still lacks a runtime-owned call into the internal promotion seam. `submit_declared_work(...)` still has no runtime-owned auto-resume redispatch path, so the healthy S04 path cannot yet finish without the old retry-era seam. Public docs remain stale because the truthful S04 replacement rail is not ready to publish.
