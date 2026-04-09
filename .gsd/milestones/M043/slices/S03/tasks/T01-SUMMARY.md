---
id: T01
parent: S03
milestone: M043
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/e2e_m043_s03.rs", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Keep the same-image Docker proof strict on ownership, epoch, hostname-derived node names, and stale-primary fencing, but allow the promoted standby's post-rejoin replication_health to be either local_only or healthy because the runtime truth varies with timing.", "Retain sanitized Docker inspect JSON plus per-phase HTTP and per-container stdout/stderr logs in the Rust e2e so later verifier phases can diagnose operator truth from artifacts without leaking CLUSTER_PROOF_COOKIE."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-level verification passed: cargo run -q -p meshc -- test cluster-proof/tests, cargo run -q -p meshc -- build cluster-proof, and cargo test -p meshc --test e2e_m043_s03 -- --nocapture all succeeded. Slice-level verification is partially green at T01 as expected: bash scripts/verify-m043-s03.sh fails closed with exit 127 because the packaged verifier is a T02 deliverable, not a T01 regression surface."
completed_at: 2026-03-29T10:37:13.837Z
blocker_discovered: false
---

# T01: Added the same-image Docker failover e2e with retained artifacts and stale-primary fencing checks.

> Added the same-image Docker failover e2e with retained artifacts and stale-primary fencing checks.

## What Happened
---
id: T01
parent: S03
milestone: M043
key_files:
  - compiler/meshc/tests/e2e_m043_s03.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Keep the same-image Docker proof strict on ownership, epoch, hostname-derived node names, and stale-primary fencing, but allow the promoted standby's post-rejoin replication_health to be either local_only or healthy because the runtime truth varies with timing.
  - Retain sanitized Docker inspect JSON plus per-phase HTTP and per-container stdout/stderr logs in the Rust e2e so later verifier phases can diagnose operator truth from artifacts without leaking CLUSTER_PROOF_COOKIE.
duration: ""
verification_result: mixed
completed_at: 2026-03-29T10:37:13.839Z
blocker_discovered: false
---

# T01: Added the same-image Docker failover e2e with retained artifacts and stale-primary fencing checks.

**Added the same-image Docker failover e2e with retained artifacts and stale-primary fencing checks.**

## What Happened

Added compiler/meshc/tests/e2e_m043_s03.rs as the same-image Docker regression for the M043/S03 operator rail. The test builds the repo-root cluster-proof image, starts primary and standby from that image on one Docker bridge network without explicit identity env, proves hostname-derived node names, selects a deterministic request key whose owner is primary and replica is standby, and retains scenario-meta.json, raw HTTP snapshots, sanitized inspect JSON, image/network metadata, and per-container stdout/stderr logs under .tmp/m043-s03/. The destructive scenario now covers mirrored pending truth, primary kill, degraded standby truth, explicit promotion, retry rollover to a new attempt, completion on the promoted standby, and fenced stale-primary rejoin on the original primary/epoch-0 env. I also added negative tests for malformed/incomplete HTTP payloads and for request keys that do not place owner=primary and replica=standby, and recorded the post-rejoin health variability in .gsd/KNOWLEDGE.md so T02 does not re-learn it while assembling the packaged verifier.

## Verification

Task-level verification passed: cargo run -q -p meshc -- test cluster-proof/tests, cargo run -q -p meshc -- build cluster-proof, and cargo test -p meshc --test e2e_m043_s03 -- --nocapture all succeeded. Slice-level verification is partially green at T01 as expected: bash scripts/verify-m043-s03.sh fails closed with exit 127 because the packaged verifier is a T02 deliverable, not a T01 regression surface.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 11018ms |
| 2 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 7005ms |
| 3 | `cargo test -p meshc --test e2e_m043_s03 -- --nocapture` | 0 | ✅ pass | 26324ms |
| 4 | `bash scripts/verify-m043-s03.sh` | 127 | ❌ fail | 60ms |


## Deviations

None. The only local adaptation was allowing the promoted standby's post-rejoin replication_health to settle as either local_only or healthy while keeping ownership, epoch, hostname identity, and stale-primary fence assertions strict.

## Known Issues

scripts/verify-m043-s03.sh does not exist yet, so the packaged slice-level verifier still fails closed with exit 127 until T02 assembles it.

## Files Created/Modified

- `compiler/meshc/tests/e2e_m043_s03.rs`
- `.gsd/KNOWLEDGE.md`


## Deviations
None. The only local adaptation was allowing the promoted standby's post-rejoin replication_health to settle as either local_only or healthy while keeping ownership, epoch, hostname identity, and stale-primary fence assertions strict.

## Known Issues
scripts/verify-m043-s03.sh does not exist yet, so the packaged slice-level verifier still fails closed with exit 127 until T02 assembles it.
