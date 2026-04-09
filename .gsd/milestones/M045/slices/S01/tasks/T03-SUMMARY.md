---
id: T03
parent: S01
milestone: M045
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-rt/src/dist/node.rs", "compiler/mesh-rt/src/lib.rs", "compiler/mesh-typeck/src/infer.rs", "compiler/mesh-typeck/src/builtins.rs", "compiler/mesh-codegen/src/mir/lower.rs", "compiler/mesh-codegen/src/codegen/intrinsics.rs", "compiler/mesh-pkg/src/scaffold.rs", "compiler/meshc/tests/tooling_e2e.rs", "compiler/meshc/tests/e2e_m044_s03.rs", "compiler/meshc/tests/e2e_m045_s01.rs", "scripts/verify-m044-s03.sh"]
key_decisions: ["D213: expose `Node.start_from_env()` as a public typed Mesh surface and make clustered scaffolds consume `BootstrapStatus` instead of reading bootstrap env or calling `Node.start(...)` directly.", "Clustered scaffold startup now emits one runtime-owned bootstrap log line and keeps bootstrap failure fail-closed instead of serving with partial app-owned env parsing."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "I ran the new M045 target first to prove the compiler/runtime/public bootstrap seam, including standalone, explicit node identity, Fly identity fallback, fail-closed missing-cookie behavior, bind failure, compile-time wrong-arity/missing-field/type-mismatch rails, and the scaffold shape assertion. Then I reran the task’s required scaffold verifiers: the direct `meshc init --clustered` tooling rail, the scaffold runtime truth e2e that builds/runs the generated app, checks `/health`, checks `meshc cluster status`, and inspects the new startup log, and the protected assembled `scripts/verify-m044-s03.sh` wrapper to confirm the updated scaffold contract and retained operator/public-contract rails still agree."
completed_at: 2026-03-30T18:28:34.838Z
blocker_discovered: false
---

# T03: Moved `meshc init --clustered` onto `Node.start_from_env()` and added scaffold/bootstrap proof rails.

> Moved `meshc init --clustered` onto `Node.start_from_env()` and added scaffold/bootstrap proof rails.

## What Happened
---
id: T03
parent: S01
milestone: M045
key_files:
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-rt/src/lib.rs
  - compiler/mesh-typeck/src/infer.rs
  - compiler/mesh-typeck/src/builtins.rs
  - compiler/mesh-codegen/src/mir/lower.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/tests/tooling_e2e.rs
  - compiler/meshc/tests/e2e_m044_s03.rs
  - compiler/meshc/tests/e2e_m045_s01.rs
  - scripts/verify-m044-s03.sh
key_decisions:
  - D213: expose `Node.start_from_env()` as a public typed Mesh surface and make clustered scaffolds consume `BootstrapStatus` instead of reading bootstrap env or calling `Node.start(...)` directly.
  - Clustered scaffold startup now emits one runtime-owned bootstrap log line and keeps bootstrap failure fail-closed instead of serving with partial app-owned env parsing.
duration: ""
verification_result: passed
completed_at: 2026-03-30T18:28:34.839Z
blocker_discovered: false
---

# T03: Moved `meshc init --clustered` onto `Node.start_from_env()` and added scaffold/bootstrap proof rails.

**Moved `meshc init --clustered` onto `Node.start_from_env()` and added scaffold/bootstrap proof rails.**

## What Happened

The local tree did not actually contain the T02 compiler/runtime seam yet, so I finished the missing prerequisite before rewriting the scaffold. I added the Mesh-facing `mesh_node_start_from_env()` runtime export plus a public `BootstrapStatus` result layout in `compiler/mesh-rt/src/dist/node.rs`, re-exported it from `compiler/mesh-rt/src/lib.rs`, registered `Node.start_from_env() -> Result<BootstrapStatus, String>` and the builtin `BootstrapStatus` struct in `compiler/mesh-typeck`, mapped the builtin to `mesh_node_start_from_env` and pre-seeded the matching MIR struct in `compiler/mesh-codegen`, and declared the new intrinsic in `compiler/mesh-codegen/src/codegen/intrinsics.rs`.

With that seam in place, I rewrote `compiler/mesh-pkg/src/scaffold.rs` so clustered `main.mpl` no longer reads `MESH_CLUSTER_COOKIE` / `MESH_NODE_NAME` / `MESH_DISCOVERY_SEED` or calls `Node.start(...)` directly. The generated app now calls `Node.start_from_env()`, logs the returned `BootstrapStatus`, and starts only the HTTP/work surface locally. I also updated the scaffold README text to keep the public `MESH_*` contract while explicitly describing the runtime-owned bootstrap boundary.

Then I realigned the proof surfaces: `compiler/meshc/tests/tooling_e2e.rs` and the scaffold unit tests now assert the smaller runtime-owned source shape, `compiler/meshc/tests/e2e_m044_s03.rs` now verifies the scaffolded app logs the runtime bootstrap outcome while still exposing `/health` and truthful `meshc cluster status` output, `scripts/verify-m044-s03.sh` now fail-closes on stale bootstrap literals in generated `main.mpl`, and `compiler/meshc/tests/e2e_m045_s01.rs` adds a dedicated M045 rail for typed bootstrap behavior, fail-closed runtime errors, compile-fail cases, and scaffold shape.

## Verification

I ran the new M045 target first to prove the compiler/runtime/public bootstrap seam, including standalone, explicit node identity, Fly identity fallback, fail-closed missing-cookie behavior, bind failure, compile-time wrong-arity/missing-field/type-mismatch rails, and the scaffold shape assertion. Then I reran the task’s required scaffold verifiers: the direct `meshc init --clustered` tooling rail, the scaffold runtime truth e2e that builds/runs the generated app, checks `/health`, checks `meshc cluster status`, and inspects the new startup log, and the protected assembled `scripts/verify-m044-s03.sh` wrapper to confirm the updated scaffold contract and retained operator/public-contract rails still agree.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m045_s01 m045_s01_bootstrap_api_ -- --nocapture` | 0 | ✅ pass | 15550ms |
| 2 | `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture` | 0 | ✅ pass | 10800ms |
| 3 | `cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture` | 0 | ✅ pass | 15090ms |
| 4 | `bash scripts/verify-m044-s03.sh` | 0 | ✅ pass | 30000ms |


## Deviations

The local repo snapshot was missing the T02 compiler/runtime `Node.start_from_env()` seam even though T03 assumed it existed, so I finished that prerequisite in this unit before landing the scaffold rewrite. That was necessary to make the generated `main.mpl` compile and to add the dedicated M045 proof rail the task plan expected.

## Known Issues

`scripts/verify-m045-s01.sh` still belongs to T04 and is not in the tree yet. This task leaves the scaffold/runtime bootstrap surfaces green and ready for T04’s `cluster-proof` adoption and assembled closeout rail.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/lib.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-typeck/src/builtins.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/e2e_m044_s03.rs`
- `compiler/meshc/tests/e2e_m045_s01.rs`
- `scripts/verify-m044-s03.sh`


## Deviations
The local repo snapshot was missing the T02 compiler/runtime `Node.start_from_env()` seam even though T03 assumed it existed, so I finished that prerequisite in this unit before landing the scaffold rewrite. That was necessary to make the generated `main.mpl` compile and to add the dedicated M045 proof rail the task plan expected.

## Known Issues
`scripts/verify-m045-s01.sh` still belongs to T04 and is not in the tree yet. This task leaves the scaffold/runtime bootstrap surfaces green and ready for T04’s `cluster-proof` adoption and assembled closeout rail.
