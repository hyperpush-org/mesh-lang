---
id: T04
parent: S03
milestone: M046
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-rt/src/dist/node.rs", "compiler/meshc/tests/e2e_m046_s03.rs", "scripts/verify-m046-s03.sh", ".gsd/milestones/M046/slices/S03/S03-PLAN.md", ".gsd/KNOWLEDGE.md", ".gsd/DECISIONS.md"]
key_decisions: ["D244: use a bounded language-owned startup dispatch window for replicated startup work and surface it as `startup_dispatch_window` diagnostics instead of any env-driven delay control.", "For enduring slice plans that preserve historical text, assert an explicit current-state override line instead of treating every historical timing reference as current-state drift."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the T04 contract with `cargo test -p mesh-rt startup_work_ -- --nocapture`, `cargo run -q -p meshc -- build tiny-cluster`, `cargo run -q -p meshc -- test tiny-cluster/tests`, `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_failover_ -- --nocapture`, and `bash scripts/verify-m046-s03.sh`. The direct verifier replay also re-ran the focused M046/S02 startup rail and retained a fresh failover bundle; `.tmp/m046-s03/verify/status.txt` is `ok` and `.tmp/m046-s03/verify/latest-proof-bundle.txt` points at `.tmp/m046-s03/verify/retained-m046-s03-artifacts/tiny-cluster-failover-runtime-truth-1774992455626971000`."
completed_at: 2026-03-31T21:29:51.093Z
blocker_discovered: false
---

# T04: Replaced the last tiny-cluster failover timing knob with a language-owned startup dispatch window and fail-closed contract guards.

> Replaced the last tiny-cluster failover timing knob with a language-owned startup dispatch window and fail-closed contract guards.

## What Happened
---
id: T04
parent: S03
milestone: M046
key_files:
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/meshc/tests/e2e_m046_s03.rs
  - scripts/verify-m046-s03.sh
  - .gsd/milestones/M046/slices/S03/S03-PLAN.md
  - .gsd/KNOWLEDGE.md
  - .gsd/DECISIONS.md
key_decisions:
  - D244: use a bounded language-owned startup dispatch window for replicated startup work and surface it as `startup_dispatch_window` diagnostics instead of any env-driven delay control.
  - For enduring slice plans that preserve historical text, assert an explicit current-state override line instead of treating every historical timing reference as current-state drift.
duration: ""
verification_result: passed
completed_at: 2026-03-31T21:29:51.095Z
blocker_discovered: false
---

# T04: Replaced the last tiny-cluster failover timing knob with a language-owned startup dispatch window and fail-closed contract guards.

**Replaced the last tiny-cluster failover timing knob with a language-owned startup dispatch window and fail-closed contract guards.**

## What Happened

Removed the remaining env-driven startup delay seam from the runtime path by replacing `MESH_STARTUP_WORK_DELAY_MS` with a bounded language-owned clustered startup dispatch window in `compiler/mesh-rt/src/dist/node.rs`, surfaced through `startup_dispatch_window` diagnostics/logs. Updated `compiler/meshc/tests/e2e_m046_s03.rs` so the failover rail no longer injects a delay env var, archives the enduring slice plan and verifier script into proof artifacts, records `startup_pending_window_source` instead of a delay value, and asserts the new runtime signal plus the absence of the legacy `startup_dispatch_delay` signal. Hardened `scripts/verify-m046-s03.sh` with a fail-closed `contract-guards` phase that checks tiny-cluster source/readme/smoke/e2e drift and the new T04 override line in `S03-PLAN.md`, then replays the runtime/package/failover proof and retains a fresh bundle under `.tmp/m046-s03/verify/retained-m046-s03-artifacts`. Also recorded D244 in `.gsd/DECISIONS.md` and the enduring-plan override-check pattern in `.gsd/KNOWLEDGE.md`.

## Verification

Verified the T04 contract with `cargo test -p mesh-rt startup_work_ -- --nocapture`, `cargo run -q -p meshc -- build tiny-cluster`, `cargo run -q -p meshc -- test tiny-cluster/tests`, `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_failover_ -- --nocapture`, and `bash scripts/verify-m046-s03.sh`. The direct verifier replay also re-ran the focused M046/S02 startup rail and retained a fresh failover bundle; `.tmp/m046-s03/verify/status.txt` is `ok` and `.tmp/m046-s03/verify/latest-proof-bundle.txt` points at `.tmp/m046-s03/verify/retained-m046-s03-artifacts/tiny-cluster-failover-runtime-truth-1774992455626971000`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-rt startup_work_ -- --nocapture` | 0 | ✅ pass | 53064ms |
| 2 | `cargo run -q -p meshc -- build tiny-cluster` | 0 | ✅ pass | 25275ms |
| 3 | `cargo run -q -p meshc -- test tiny-cluster/tests` | 0 | ✅ pass | 7568ms |
| 4 | `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_failover_ -- --nocapture` | 0 | ✅ pass | 63593ms |
| 5 | `bash scripts/verify-m046-s03.sh` | 0 | ✅ pass | 134669ms |


## Deviations

Instead of trying to prove cleanup by banning all historical timing terms from the entire enduring slice plan, I added an explicit T04 override line to `S03-PLAN.md` and had the verifier/e2e contract assert that current-state line directly. The historical T01-T03 text remains intentionally preserved.

## Known Issues

None.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/meshc/tests/e2e_m046_s03.rs`
- `scripts/verify-m046-s03.sh`
- `.gsd/milestones/M046/slices/S03/S03-PLAN.md`
- `.gsd/KNOWLEDGE.md`
- `.gsd/DECISIONS.md`


## Deviations
Instead of trying to prove cleanup by banning all historical timing terms from the entire enduring slice plan, I added an explicit T04 override line to `S03-PLAN.md` and had the verifier/e2e contract assert that current-state line directly. The historical T01-T03 text remains intentionally preserved.

## Known Issues
None.
