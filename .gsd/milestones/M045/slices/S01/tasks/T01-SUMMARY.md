---
id: T01
parent: S01
milestone: M045
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-rt/src/dist/bootstrap.rs", "compiler/mesh-rt/src/dist/node.rs", "compiler/mesh-rt/src/dist/mod.rs", "compiler/mesh-rt/src/lib.rs", ".gsd/KNOWLEDGE.md"]
key_decisions: ["D212: mirror the existing cluster-proof env validation/error contract in Rust while keeping low-level `mesh_node_start` :0 bind support behind a separate bind parser", "Keep bootstrap validation pure and side-effect free, then delegate the actual listener start through a thin node adapter only after a fully validated cluster plan exists"]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-level verification passed: `cargo test -p mesh-rt bootstrap_ -- --nocapture` proved the new bootstrap helper across standalone, explicit `MESH_NODE_NAME`, Fly identity fallback, cookie-less cluster hints, blank discovery seed, malformed node name, invalid cluster port, port mismatch, and bind-failure error surfacing; `cargo test -p mesh-rt test_mesh_node_start_binds_listener -- --nocapture` proved the low-level `mesh_node_start(...)` path still binds a listener and reports the assigned port. Slice-level verification was also run for this intermediate task and is still red because later-task artifacts are not in tree yet: `cargo test -p meshc --test e2e_m045_s01 -- --nocapture` fails because the test target does not exist, and `bash scripts/verify-m045-s01.sh` fails because the assembled verifier script does not exist yet."
completed_at: 2026-03-30T17:52:28.310Z
blocker_discovered: false
---

# T01: Added a runtime-owned bootstrap helper with typed startup status and fail-closed cluster env validation.

> Added a runtime-owned bootstrap helper with typed startup status and fail-closed cluster env validation.

## What Happened
---
id: T01
parent: S01
milestone: M045
key_files:
  - compiler/mesh-rt/src/dist/bootstrap.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-rt/src/dist/mod.rs
  - compiler/mesh-rt/src/lib.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D212: mirror the existing cluster-proof env validation/error contract in Rust while keeping low-level `mesh_node_start` :0 bind support behind a separate bind parser
  - Keep bootstrap validation pure and side-effect free, then delegate the actual listener start through a thin node adapter only after a fully validated cluster plan exists
duration: ""
verification_result: mixed
completed_at: 2026-03-30T17:52:28.312Z
blocker_discovered: false
---

# T01: Added a runtime-owned bootstrap helper with typed startup status and fail-closed cluster env validation.

**Added a runtime-owned bootstrap helper with typed startup status and fail-closed cluster env validation.**

## What Happened

Added `compiler/mesh-rt/src/dist/bootstrap.rs` as the runtime-owned bootstrap seam for clustered startup. The helper now owns standalone-vs-cluster detection, `MESH_CLUSTER_COOKIE` gating, `MESH_DISCOVERY_SEED` requirements, strict `MESH_NODE_NAME` validation, Fly identity fallback, cluster-port parsing, and the typed `BootstrapMode` / `BootstrapStatus` payload returned to callers. `compiler/mesh-rt/src/dist/node.rs` now exposes `start_from_env()` as the thin integration layer and only calls the existing low-level `mesh_node_start(...)` path after cluster input is fully validated, so standalone mode remains side-effect free. While verifying the required listener rail, I found that the low-level `mesh_node_start("name@host:0", ...)` test was red because the normal node-name parser rejects port `0`; I fixed that by splitting the bind parser from the public bootstrap validator so the runtime keeps OS-assigned-port support without relaxing the new env contract. I also re-exported the bootstrap types/function surface from `mesh-rt`, added focused runtime unit coverage for standalone, explicit-node, Fly-identity, malformed-input, and bind-failure cases, recorded decision D212, and appended a matching knowledge entry for the bind-parser split.

## Verification

Task-level verification passed: `cargo test -p mesh-rt bootstrap_ -- --nocapture` proved the new bootstrap helper across standalone, explicit `MESH_NODE_NAME`, Fly identity fallback, cookie-less cluster hints, blank discovery seed, malformed node name, invalid cluster port, port mismatch, and bind-failure error surfacing; `cargo test -p mesh-rt test_mesh_node_start_binds_listener -- --nocapture` proved the low-level `mesh_node_start(...)` path still binds a listener and reports the assigned port. Slice-level verification was also run for this intermediate task and is still red because later-task artifacts are not in tree yet: `cargo test -p meshc --test e2e_m045_s01 -- --nocapture` fails because the test target does not exist, and `bash scripts/verify-m045-s01.sh` fails because the assembled verifier script does not exist yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-rt bootstrap_ -- --nocapture` | 0 | ✅ pass | 12560ms |
| 2 | `cargo test -p mesh-rt test_mesh_node_start_binds_listener -- --nocapture` | 0 | ✅ pass | 8120ms |
| 3 | `cargo test -p meshc --test e2e_m045_s01 -- --nocapture` | 101 | ❌ fail | 620ms |
| 4 | `bash scripts/verify-m045-s01.sh` | 127 | ❌ fail | 0ms |


## Deviations

Added a bind-only node-name parser in `compiler/mesh-rt/src/dist/node.rs` so the low-level `mesh_node_start("name@host:0", ...)` listener rail stays truthful without weakening the new bootstrap validator’s cluster-port rules.

## Known Issues

`compiler/meshc/tests/e2e_m045_s01.rs` is still missing, so `cargo test -p meshc --test e2e_m045_s01 -- --nocapture` fails until T02 lands. `scripts/verify-m045-s01.sh` is still missing, so the assembled slice verifier cannot pass until T04 lands.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/bootstrap.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/dist/mod.rs`
- `compiler/mesh-rt/src/lib.rs`
- `.gsd/KNOWLEDGE.md`


## Deviations
Added a bind-only node-name parser in `compiler/mesh-rt/src/dist/node.rs` so the low-level `mesh_node_start("name@host:0", ...)` listener rail stays truthful without weakening the new bootstrap validator’s cluster-port rules.

## Known Issues
`compiler/meshc/tests/e2e_m045_s01.rs` is still missing, so `cargo test -p meshc --test e2e_m045_s01 -- --nocapture` fails until T02 lands. `scripts/verify-m045-s01.sh` is still missing, so the assembled slice verifier cannot pass until T04 lands.
