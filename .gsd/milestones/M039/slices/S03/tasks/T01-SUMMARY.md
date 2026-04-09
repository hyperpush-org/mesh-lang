---
id: T01
parent: S03
milestone: M039
provides: []
requires: []
affects: []
key_files: ["cluster-proof/work.mpl", "cluster-proof/tests/work.test.mpl", "compiler/meshc/tests/e2e_m039_s02.rs", "compiler/meshc/tests/e2e_m039_s03.rs", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M039/slices/S03/tasks/T01-SUMMARY.md"]
key_decisions: ["D137: keep request correlation ingress-owned via `work dispatched ...` logs plus run-numbered peer logs/artifacts instead of cross-node actor arguments.", "Record the cross-node actor-argument restart gotcha in `.gsd/KNOWLEDGE.md` so later M039 work does not reuse that seam for proof correlation."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "T01-owned checks passed: `cargo run -q -p meshc -- test cluster-proof/tests`, `cargo run -q -p meshc -- build cluster-proof`, `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_falls_back_locally_without_peers -- --nocapture`, `cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_degrades_safely_and_serves_locally_after_peer_loss -- --nocapture`, and `cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair -- --nocapture`. Observability was verified directly from `.tmp/m039-s03/e2e-m039-s03-rejoin-1774698935459642000/`, where `pre-loss-work.json`, `degraded-work.json`, and `post-rejoin-work.json` show `work-0`/`work-1`/`work-2`, `node-a-run1.stdout.log` contains the matching ingress `work dispatched ...` lines, and `node-b-run2.stdout.log` is preserved separately from `node-b-run1.stdout.log`. The slice-level wrapper check `bash scripts/verify-m039-s03.sh` currently fails because the wrapper file does not exist yet; that is the planned T02 deliverable, not a T01 blocker."
completed_at: 2026-03-28T12:00:15.234Z
blocker_discovered: false
---

# T01: Added ingress-owned request correlation and live S03 degrade/rejoin continuity proofs for cluster-proof.

> Added ingress-owned request correlation and live S03 degrade/rejoin continuity proofs for cluster-proof.

## What Happened
---
id: T01
parent: S03
milestone: M039
key_files:
  - cluster-proof/work.mpl
  - cluster-proof/tests/work.test.mpl
  - compiler/meshc/tests/e2e_m039_s02.rs
  - compiler/meshc/tests/e2e_m039_s03.rs
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M039/slices/S03/tasks/T01-SUMMARY.md
key_decisions:
  - D137: keep request correlation ingress-owned via `work dispatched ...` logs plus run-numbered peer logs/artifacts instead of cross-node actor arguments.
  - Record the cross-node actor-argument restart gotcha in `.gsd/KNOWLEDGE.md` so later M039 work does not reuse that seam for proof correlation.
duration: ""
verification_result: mixed
completed_at: 2026-03-28T12:00:15.236Z
blocker_discovered: false
---

# T01: Added ingress-owned request correlation and live S03 degrade/rejoin continuity proofs for cluster-proof.

**Added ingress-owned request correlation and live S03 degrade/rejoin continuity proofs for cluster-proof.**

## What Happened

Added a local request-counter service to `cluster-proof/work.mpl` so repeated `/work` calls from one node lifetime emit deterministic `work-0`, `work-1`, `work-2`, ... ids without widening the HTTP response contract. Updated the Mesh unit tests and the existing S02 Rust harness to validate parsed request tokens and the new log contract. Added `compiler/meshc/tests/e2e_m039_s03.rs` with live two-node degrade/rejoin proofs that preserve phase-specific membership/work artifacts plus run-numbered node logs so a restarted peer cannot overwrite the first incarnation’s evidence. During verification, cross-node actor arguments proved unreliable through same-identity restart (`Int` tokens reappeared as `work-0` on the restarted peer and `String` request ids crashed the peer in `mesh-rt` string handling), so the final proof keeps correlation ingress-owned via `work dispatched ...` logs and uses run-numbered peer logs plus `*-work.json` artifacts as the truthful remote-execution evidence. Recorded that gotcha in `.gsd/KNOWLEDGE.md` and saved the observability decision as D137.

## Verification

T01-owned checks passed: `cargo run -q -p meshc -- test cluster-proof/tests`, `cargo run -q -p meshc -- build cluster-proof`, `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_falls_back_locally_without_peers -- --nocapture`, `cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_degrades_safely_and_serves_locally_after_peer_loss -- --nocapture`, and `cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair -- --nocapture`. Observability was verified directly from `.tmp/m039-s03/e2e-m039-s03-rejoin-1774698935459642000/`, where `pre-loss-work.json`, `degraded-work.json`, and `post-rejoin-work.json` show `work-0`/`work-1`/`work-2`, `node-a-run1.stdout.log` contains the matching ingress `work dispatched ...` lines, and `node-b-run2.stdout.log` is preserved separately from `node-b-run1.stdout.log`. The slice-level wrapper check `bash scripts/verify-m039-s03.sh` currently fails because the wrapper file does not exist yet; that is the planned T02 deliverable, not a T01 blocker.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 7905ms |
| 2 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 5548ms |
| 3 | `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_falls_back_locally_without_peers -- --nocapture` | 0 | ✅ pass | 10612ms |
| 4 | `cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_degrades_safely_and_serves_locally_after_peer_loss -- --nocapture` | 0 | ✅ pass | 10707ms |
| 5 | `cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair -- --nocapture` | 0 | ✅ pass | 10070ms |
| 6 | `bash scripts/verify-m039-s03.sh` | 127 | ❌ fail | 8ms |


## Deviations

None.

## Known Issues

`scripts/verify-m039-s03.sh` is still missing, so the slice-level wrapper verification remains pending until T02 lands.

## Files Created/Modified

- `cluster-proof/work.mpl`
- `cluster-proof/tests/work.test.mpl`
- `compiler/meshc/tests/e2e_m039_s02.rs`
- `compiler/meshc/tests/e2e_m039_s03.rs`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M039/slices/S03/tasks/T01-SUMMARY.md`


## Deviations
None.

## Known Issues
`scripts/verify-m039-s03.sh` is still missing, so the slice-level wrapper verification remains pending until T02 lands.
