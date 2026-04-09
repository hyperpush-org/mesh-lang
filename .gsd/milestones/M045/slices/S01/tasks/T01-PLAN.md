---
estimated_steps: 4
estimated_files: 4
skills_used:
  - rust-best-practices
  - test
---

# T01: Add the runtime-owned bootstrap helper and typed startup status

**Slice:** S01 — Runtime-Owned Cluster Bootstrap
**Milestone:** M045

## Description

Create the Rust-owned bootstrap core that decides standalone vs cluster mode, validates `MESH_*` / Fly identity inputs, starts the node when needed, and returns a typed status object that Mesh code can inspect without re-reading env.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Runtime env / identity resolution around `compiler/mesh-rt/src/dist/node.rs` | Return an explicit `Err(String)` and do not call `mesh_node_start`. | N/A — env parsing is synchronous. | Reject partial cluster hints or partial Fly identity instead of coercing to standalone. |
| Existing listener/discovery startup in `compiler/mesh-rt/src/dist/node.rs` / `compiler/mesh-rt/src/dist/discovery.rs` | Keep the old `mesh_node_start` path intact and fail closed before half-starting a node. | N/A — listener bind is synchronous and discovery stays on the existing async path. | Reject invalid node-name/port inputs before they reach bind/discovery. |

## Load Profile

- **Shared resources**: global node state, listener bind port, and env-backed discovery configuration.
- **Per-operation cost**: one env parse plus optional listener bind/startup on cluster mode.
- **10x breakpoint**: conflicting startup attempts and noisy invalid-env retries break before throughput does; the helper must stay side-effect free on rejected input.

## Negative Tests

- **Malformed inputs**: blank cookie/seed, invalid `MESH_NODE_NAME`, partial Fly identity, and cluster hints without a cookie.
- **Error paths**: cluster-mode request with malformed identity, bind failure after valid parsing, and invalid port/name combinations.
- **Boundary conditions**: standalone with no cluster env, explicit `MESH_NODE_NAME`, and Fly identity fallback with no explicit node name.

## Steps

1. Extract cluster-mode detection, env parsing, and identity resolution into a focused runtime helper around the existing node start/discovery code.
2. Define a typed bootstrap status payload carrying mode, node name, cluster port, and discovery seed without exposing cluster cookies.
3. Wire the helper to call the existing low-level `mesh_node_start` path only in cluster mode while keeping standalone startup side-effect free.
4. Add Rust unit coverage for standalone, explicit node name, Fly identity, and malformed cluster-env matrices.

## Must-Haves

- [ ] The runtime owns cluster mode detection and fail-closes malformed cluster env with explicit errors.
- [ ] The helper returns typed bootstrap status data that downstream Mesh code can inspect.
- [ ] The low-level `Node.start(name, cookie)` primitive remains intact for low-level docs and explicit callers.
- [ ] Unit coverage proves standalone, explicit-node, Fly-identity, and malformed-input behavior.

## Verification

- `cargo test -p mesh-rt bootstrap_ -- --nocapture`
- `cargo test -p mesh-rt test_mesh_node_start_binds_listener -- --nocapture`

## Observability Impact

- Signals added/changed: typed bootstrap mode/node/seed/cluster-port status plus explicit fail-closed bootstrap error strings.
- How a future agent inspects this: runtime unit tests and the later `Node.start_from_env()` compiler e2e surface.
- Failure state exposed: rejected env matrix reason before any partial node startup.

## Inputs

- `compiler/mesh-rt/src/dist/node.rs` — existing low-level node-start entrypoint and test surface.
- `compiler/mesh-rt/src/dist/discovery.rs` — current env-backed discovery seed handling.
- `compiler/mesh-rt/src/lib.rs` — runtime export surface for new bootstrap types/functions.
- `cluster-proof/config.mpl` — current example-owned bootstrap contract to preserve while moving ownership downward.
- `compiler/meshc/tests/e2e_m044_s05.rs` — protected public-contract expectations for malformed env handling.

## Expected Output

- `compiler/mesh-rt/src/dist/bootstrap.rs` — new runtime-owned bootstrap helper and typed status definitions.
- `compiler/mesh-rt/src/dist/node.rs` — bootstrap entrypoint integration plus focused unit coverage.
- `compiler/mesh-rt/src/lib.rs` — exported bootstrap type/function surface for compiler use.
