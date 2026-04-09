---
id: T03
parent: S01
milestone: M039
provides: []
requires: []
affects: []
key_files: ["cluster-proof/cluster.mpl", "cluster-proof/main.mpl", "compiler/meshc/tests/e2e_m039_s01.rs", "scripts/verify-m039-s01.sh", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M039/slices/S01/tasks/T03-SUMMARY.md"]
key_decisions: ["Kept the `/membership` payload on a concrete `struct ... deriving(Json)` path for string/list fields and sourced port values from env strings, because direct request-path string/int helper lowering was still producing misaligned-pointer crashes and garbage port numbers.", "Made `scripts/verify-m039-s01.sh` the authoritative local replay wrapper and fail-closed on missing `running N test` output so named test filters cannot silently pass with zero executed tests."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed `cargo test -p meshc --test e2e_m039_s01 -- --nocapture`, the explicit task gate `cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_membership_updates_after_node_loss -- --nocapture`, the slice runtime gate `cargo test -p mesh-rt discovery_ -- --nocapture`, and the authoritative replay `bash scripts/verify-m039-s01.sh`. The wrapper left phase logs under `.tmp/m039-s01/verify/`, and the Rust harness left per-node stdout/stderr logs under `.tmp/m039-s01/e2e-m039-s01-*` for convergence and node-loss runs."
completed_at: 2026-03-28T10:01:12.349Z
blocker_discovered: false
---

# T03: Made the local cluster-proof verifier authoritative with per-node logs, a real node-loss shrinkage proof, and a fail-closed replay script.

> Made the local cluster-proof verifier authoritative with per-node logs, a real node-loss shrinkage proof, and a fail-closed replay script.

## What Happened
---
id: T03
parent: S01
milestone: M039
key_files:
  - cluster-proof/cluster.mpl
  - cluster-proof/main.mpl
  - compiler/meshc/tests/e2e_m039_s01.rs
  - scripts/verify-m039-s01.sh
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M039/slices/S01/tasks/T03-SUMMARY.md
key_decisions:
  - Kept the `/membership` payload on a concrete `struct ... deriving(Json)` path for string/list fields and sourced port values from env strings, because direct request-path string/int helper lowering was still producing misaligned-pointer crashes and garbage port numbers.
  - Made `scripts/verify-m039-s01.sh` the authoritative local replay wrapper and fail-closed on missing `running N test` output so named test filters cannot silently pass with zero executed tests.
duration: ""
verification_result: passed
completed_at: 2026-03-28T10:01:12.350Z
blocker_discovered: false
---

# T03: Made the local cluster-proof verifier authoritative with per-node logs, a real node-loss shrinkage proof, and a fail-closed replay script.

**Made the local cluster-proof verifier authoritative with per-node logs, a real node-loss shrinkage proof, and a fail-closed replay script.**

## What Happened

Stabilized the `cluster-proof` `/membership` handler by moving the response onto a typed `MembershipPayload` JSON path for the string/list fields and by sourcing port values from env strings, which avoided the request-path Mesh lowering crashes and garbage port values that were still blocking T02. Extended `compiler/meshc/tests/e2e_m039_s01.rs` so the convergence proof now fails early when a child process exits, accepts truthful port fields returned as either strings or integers, and leaves durable per-node stdout/stderr logs behind. Added the real `e2e_m039_s01_membership_updates_after_node_loss` proof that starts the dual-stack local cluster, waits for convergence, kills one node, and asserts the survivor shrinks membership to self-only with zero peers and no manual peer repair. Added `scripts/verify-m039-s01.sh` as the authoritative local replay wrapper that builds `cluster-proof`, runs the named runtime and e2e checks in order, records phase status under `.tmp/m039-s01/verify/phase-report.txt`, and fails closed when a named test filter runs zero tests or omits the `running N test` line. Recorded the stable cluster-proof HTTP payload workaround in `.gsd/KNOWLEDGE.md`.

## Verification

Passed `cargo test -p meshc --test e2e_m039_s01 -- --nocapture`, the explicit task gate `cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_membership_updates_after_node_loss -- --nocapture`, the slice runtime gate `cargo test -p mesh-rt discovery_ -- --nocapture`, and the authoritative replay `bash scripts/verify-m039-s01.sh`. The wrapper left phase logs under `.tmp/m039-s01/verify/`, and the Rust harness left per-node stdout/stderr logs under `.tmp/m039-s01/e2e-m039-s01-*` for convergence and node-loss runs.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m039_s01 -- --nocapture` | 0 | ✅ pass | 17200ms |
| 2 | `cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_membership_updates_after_node_loss -- --nocapture` | 0 | ✅ pass | 8600ms |
| 3 | `cargo test -p mesh-rt discovery_ -- --nocapture` | 0 | ✅ pass | 8500ms |
| 4 | `bash scripts/verify-m039-s01.sh` | 0 | ✅ pass | 22900ms |


## Deviations

The endpoint still returns the required `self` JSON key, but it now reaches that contract by encoding a typed struct with a temporary `node` field and renaming the emitted key in the final JSON string. That is a local workaround for current Mesh request-path lowering limits, not a change to the proof contract.

## Known Issues

None.

## Files Created/Modified

- `cluster-proof/cluster.mpl`
- `cluster-proof/main.mpl`
- `compiler/meshc/tests/e2e_m039_s01.rs`
- `scripts/verify-m039-s01.sh`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M039/slices/S01/tasks/T03-SUMMARY.md`


## Deviations
The endpoint still returns the required `self` JSON key, but it now reaches that contract by encoding a typed struct with a temporary `node` field and renaming the emitted key in the final JSON string. That is a local workaround for current Mesh request-path lowering limits, not a change to the proof contract.

## Known Issues
None.
