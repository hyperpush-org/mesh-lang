---
id: T03
parent: S04
milestone: M042
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-rt/src/dist/node.rs", "cluster-proof/work_continuity.mpl", "compiler/meshc/tests/e2e_m039_s02.rs", "compiler/meshc/tests/e2e_m042_s01.rs", ".gsd/milestones/M042/slices/S04/tasks/T03-SUMMARY.md"]
key_decisions: ["Treated `bash scripts/verify-m042-s03.sh` passing as current truth and stopped treating S03 instability as an active blocker.", "Escalated the remaining T03 work as a blocker because the packaged keyed path and inherited M039 `/work` crash both point at runtime transport/admission seams beyond the original task snapshot."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Reproduced the legacy M039 failure directly, verified that the M042 S03 prerequisite replay is green in the current checkout, reproduced the packaged Docker keyed-owner rejection in isolation, and confirmed that the older two-node keyed continuity e2e still fails with the same remote-owner `replica_required_unavailable` contract drift."
completed_at: 2026-03-29T02:20:15.751Z
blocker_discovered: true
---

# T03: Confirmed the remaining T03 blockers: remote `Node.spawn` string-arg transport still breaks the inherited M039 legacy `/work` path, and remote-owner keyed submits still reject with `replica_required_unavailable` while `verify-m042-s03.sh` now passes.

> Confirmed the remaining T03 blockers: remote `Node.spawn` string-arg transport still breaks the inherited M039 legacy `/work` path, and remote-owner keyed submits still reject with `replica_required_unavailable` while `verify-m042-s03.sh` now passes.

## What Happened
---
id: T03
parent: S04
milestone: M042
key_files:
  - compiler/mesh-rt/src/dist/node.rs
  - cluster-proof/work_continuity.mpl
  - compiler/meshc/tests/e2e_m039_s02.rs
  - compiler/meshc/tests/e2e_m042_s01.rs
  - .gsd/milestones/M042/slices/S04/tasks/T03-SUMMARY.md
key_decisions:
  - Treated `bash scripts/verify-m042-s03.sh` passing as current truth and stopped treating S03 instability as an active blocker.
  - Escalated the remaining T03 work as a blocker because the packaged keyed path and inherited M039 `/work` crash both point at runtime transport/admission seams beyond the original task snapshot.
duration: ""
verification_result: mixed
completed_at: 2026-03-29T02:20:15.752Z
blocker_discovered: true
---

# T03: Confirmed the remaining T03 blockers: remote `Node.spawn` string-arg transport still breaks the inherited M039 legacy `/work` path, and remote-owner keyed submits still reject with `replica_required_unavailable` while `verify-m042-s03.sh` now passes.

**Confirmed the remaining T03 blockers: remote `Node.spawn` string-arg transport still breaks the inherited M039 legacy `/work` path, and remote-owner keyed submits still reject with `replica_required_unavailable` while `verify-m042-s03.sh` now passes.**

## What Happened

I did not ship code changes in this unit. I reproduced the blocker paths separately and reduced them to two concrete seams. The inherited M039 failure is still local to the legacy remote `/work` execution path: `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_routes_work_to_peer_and_logs_execution -- --nocapture` still fails because the ingress response falls back to local execution after a remote dispatch attempt, and the peer aborts in `compiler/mesh-rt/src/string.rs:104:21` with a null-pointer dereference. The active path in `cluster-proof/work_continuity.mpl` still remote-spawns `execute_work(request_key, attempt_id)` with Mesh strings, and the runtime distribution layer in `compiler/mesh-rt/src/dist/node.rs` still treats remote spawn args as raw bytes/pointers. Separately, `bash scripts/verify-m042-s03.sh` now passes cleanly, so S03 instability is not an active blocker in this checkout. The packaged Docker keyed phase is still red for a different reason: with `M042_S04_SKIP_S03=1 M042_S04_SKIP_LEGACY_WORK=1 bash scripts/verify-m042-s04.sh`, the first remote-owner candidate returns HTTP 503 with `replica_required_unavailable` because the ingress node submits a record whose owner is the peer and whose replica is itself, while the runtime continuity prepare path only looks for replicas through remote sessions. The same drift is already visible in `cargo test -p meshc --test e2e_m042_s01 continuity_api_two_node_cluster_syncs_status_between_ingress_and_owner -- --nocapture`. That leaves the remaining T03 work blocked on runtime-side transport/admission fixes beyond the original task snapshot.

## Verification

Reproduced the legacy M039 failure directly, verified that the M042 S03 prerequisite replay is green in the current checkout, reproduced the packaged Docker keyed-owner rejection in isolation, and confirmed that the older two-node keyed continuity e2e still fails with the same remote-owner `replica_required_unavailable` contract drift.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_routes_work_to_peer_and_logs_execution -- --nocapture` | 101 | ❌ fail | 0ms |
| 2 | `bash scripts/verify-m042-s03.sh` | 0 | ✅ pass | 0ms |
| 3 | `M042_S04_SKIP_S03=1 M042_S04_SKIP_LEGACY_WORK=1 bash scripts/verify-m042-s04.sh` | 1 | ❌ fail | 0ms |
| 4 | `cargo test -p meshc --test e2e_m042_s01 continuity_api_two_node_cluster_syncs_status_between_ingress_and_owner -- --nocapture` | 101 | ❌ fail | 0ms |


## Deviations

Did not implement code changes or rerun the final combined slice verification chain because the context-budget wrap-up hit after the blocker reproduction phase. I wrote the exact blocker seams and resume evidence instead.

## Known Issues

`compiler/mesh-rt/src/dist/node.rs` still handles remote spawn args as raw bytes/pointers, which leaves the inherited M039 remote `/work` proof red when `execute_work(request_key, attempt_id)` is spawned on a peer. `cluster-proof/work_continuity.mpl` still submits remote-owner keyed work from the ingress node, and the runtime continuity prepare path treats `replica_node == self` as unavailable, which keeps remote-owner keyed submits red in both the non-Docker and packaged rails. `compiler/meshc/tests/e2e_m039_s02.rs` and `compiler/meshc/tests/e2e_m042_s01.rs` are currently failing against local reality for the reasons above.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/node.rs`
- `cluster-proof/work_continuity.mpl`
- `compiler/meshc/tests/e2e_m039_s02.rs`
- `compiler/meshc/tests/e2e_m042_s01.rs`
- `.gsd/milestones/M042/slices/S04/tasks/T03-SUMMARY.md`


## Deviations
Did not implement code changes or rerun the final combined slice verification chain because the context-budget wrap-up hit after the blocker reproduction phase. I wrote the exact blocker seams and resume evidence instead.

## Known Issues
`compiler/mesh-rt/src/dist/node.rs` still handles remote spawn args as raw bytes/pointers, which leaves the inherited M039 remote `/work` proof red when `execute_work(request_key, attempt_id)` is spawned on a peer. `cluster-proof/work_continuity.mpl` still submits remote-owner keyed work from the ingress node, and the runtime continuity prepare path treats `replica_node == self` as unavailable, which keeps remote-owner keyed submits red in both the non-Docker and packaged rails. `compiler/meshc/tests/e2e_m039_s02.rs` and `compiler/meshc/tests/e2e_m042_s01.rs` are currently failing against local reality for the reasons above.
