---
id: T01
parent: S05
milestone: M046
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-pkg/src/scaffold.rs", "compiler/meshc/tests/tooling_e2e.rs", ".gsd/milestones/M046/slices/S05/tasks/T01-SUMMARY.md"]
key_decisions: ["Recorded D253 to make source-owned declared work plus CLI-only inspection the clustered scaffold contract.", "Made the fast scaffold unit and CLI smoke tests reject leftover routeful strings so drift fails closed early."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-level verification passed via cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture and cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture. Slice-level spot checks also showed cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture && cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture and npm --prefix website run build still pass, while the broader scaffold-regression chain currently fails on stale routeful assertions in e2e_m044_s03 and the authoritative S05 verifier script is not present yet."
completed_at: 2026-04-01T00:38:34.302Z
blocker_discovered: false
---

# T01: Rewrote `meshc init --clustered` to generate the route-free clustered-work scaffold contract and fail fast on routeful drift.

> Rewrote `meshc init --clustered` to generate the route-free clustered-work scaffold contract and fail fast on routeful drift.

## What Happened
---
id: T01
parent: S05
milestone: M046
key_files:
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/tests/tooling_e2e.rs
  - .gsd/milestones/M046/slices/S05/tasks/T01-SUMMARY.md
key_decisions:
  - Recorded D253 to make source-owned declared work plus CLI-only inspection the clustered scaffold contract.
  - Made the fast scaffold unit and CLI smoke tests reject leftover routeful strings so drift fails closed early.
duration: ""
verification_result: mixed
completed_at: 2026-04-01T00:38:34.308Z
blocker_discovered: false
---

# T01: Rewrote `meshc init --clustered` to generate the route-free clustered-work scaffold contract and fail fast on routeful drift.

**Rewrote `meshc init --clustered` to generate the route-free clustered-work scaffold contract and fail fast on routeful drift.**

## What Happened

Rewrote the clustered scaffold templates so meshc init --clustered now emits a package-only mesh.toml, a route-free main.mpl that only logs Node.start_from_env() bootstrap success/failure, and a proof-aligned work.mpl with declared_work_runtime_name(), a single clustered(work) declaration, and visible 1 + 1 work. Rewrote the generated README to teach the runtime-owned clustered story and CLI inspection flow instead of app-owned HTTP submit/health routes. Updated the embedded mesh-pkg scaffold unit test and the meshc tooling_e2e clustered-init smoke test to assert the new contract directly and fail on surviving [cluster], HTTP.serve(...), /health, /work, Continuity.submit_declared_work(...), and Timer.sleep(...) drift.

## Verification

Task-level verification passed via cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture and cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture. Slice-level spot checks also showed cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture && cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture and npm --prefix website run build still pass, while the broader scaffold-regression chain currently fails on stale routeful assertions in e2e_m044_s03 and the authoritative S05 verifier script is not present yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture` | 0 | ✅ pass | 8312ms |
| 2 | `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture` | 0 | ✅ pass | 24234ms |
| 3 | `cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture && cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture && cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture && cargo test -p meshc --test e2e_m045_s03 m045_s03_ -- --nocapture && cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture && cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture` | 101 | ❌ fail | 12153ms |
| 4 | `cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture && cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture` | 0 | ✅ pass | 44094ms |
| 5 | `npm --prefix website run build` | 0 | ✅ pass | 36727ms |
| 6 | `bash scripts/verify-m046-s05.sh && bash scripts/verify-m045-s05.sh` | 127 | ❌ fail | 81ms |


## Deviations

None.

## Known Issues

Historical scaffold regression rails still expect the removed routeful contract and will need T02 updates before the full S05 chain passes. scripts/verify-m046-s05.sh does not exist yet and is expected to land in T04.

## Files Created/Modified

- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `.gsd/milestones/M046/slices/S05/tasks/T01-SUMMARY.md`


## Deviations
None.

## Known Issues
Historical scaffold regression rails still expect the removed routeful contract and will need T02 updates before the full S05 chain passes. scripts/verify-m046-s05.sh does not exist yet and is expected to land in T04.
