# S01: Runtime-Owned Cluster Bootstrap

**Goal:** Move clustered app startup onto a public runtime-owned bootstrap API so generated apps and `cluster-proof` stop hand-rolling cluster mode detection, identity resolution, and direct `Node.start(...)` orchestration.
**Demo:** After this: After this: `meshc init --clustered` produces a visibly smaller clustered app whose startup and inspection path are mostly runtime/public-surface owned instead of proof-app-shaped bootstrap code.

## Tasks
- [x] **T01: Added a runtime-owned bootstrap helper with typed startup status and fail-closed cluster env validation.** — Create the Rust-owned bootstrap core that decides standalone vs cluster mode, validates `MESH_*` / Fly identity inputs, starts the node when needed, and returns a typed status object that Mesh code can inspect without re-reading env.

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
  - Estimate: 2h
  - Files: compiler/mesh-rt/src/dist/bootstrap.rs, compiler/mesh-rt/src/dist/node.rs, compiler/mesh-rt/src/dist/discovery.rs, compiler/mesh-rt/src/lib.rs
  - Verify: cargo test -p mesh-rt bootstrap_ -- --nocapture
- [x] **T02: Expose `Node.start_from_env()` through the compiler and add bootstrap e2e rails** — Wire the new bootstrap boundary through typeck, MIR lowering, intrinsic declarations, and codegen so Mesh code can call it directly, then prove the public API with a dedicated M045 compiler e2e target.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Node builtin registration across typeck / MIR / codegen | Fail the compile or tests loudly on missing symbol, wrong arity, or stale lowering. | N/A — compile-time plumbing is synchronous. | Reject field/shape drift rather than boxing status back into strings. |
| Runtime export / Mesh result layout | Stop the e2e target with the exact ABI mismatch; do not fall back to app-owned env parsing. | N/A — temp-project compile/run is bounded by Cargo. | Treat wrong payload boxing or field layout as a contract regression. |

## Load Profile

- **Shared resources**: compiler builtin tables, temp-project build output, and the `mesh-rt` staticlib freshness requirement.
- **Per-operation cost**: one runtime rebuild plus one temp-project compile/run per proof case.
- **10x breakpoint**: stale `mesh-rt` artifacts and repeated relink churn fail before performance matters; the task must keep the ABI surface narrow and explicit.

## Negative Tests

- **Malformed inputs**: wrong-arity calls, missing-field access on the status struct, and invalid use as an `Int` return.
- **Error paths**: malformed bootstrap env returned as `Err(String)` through Mesh code, and runtime bootstrap failure propagating without a string decode shim.
- **Boundary conditions**: standalone no-op startup, explicit `MESH_NODE_NAME`, and Fly identity without an explicit node name.

## Steps

1. Register the new Node builtin and bootstrap status type in `compiler/mesh-typeck` and `compiler/mesh-codegen` alongside the existing `Node.start(...)` primitive.
2. Export the runtime symbol and typed result layout so the generated LLVM calls the new bootstrap helper directly.
3. Add `compiler/meshc/tests/e2e_m045_s01.rs` with named `m045_s01_bootstrap_api_` coverage for standalone, explicit-node, Fly-identity, and malformed-env cases.
4. Keep the runtime build freshness hook or equivalent in place so temp-project linking cannot silently use stale `mesh-rt` symbols.

## Must-Haves

- [ ] Mesh code can call `Node.start_from_env()` and inspect typed status fields directly.
- [ ] Typeck, MIR, intrinsics, and runtime exports agree on arity, symbol name, and payload layout.
- [ ] `compiler/meshc/tests/e2e_m045_s01.rs` proves both happy-path and fail-closed bootstrap behavior.
  - Estimate: 2h
  - Files: compiler/mesh-typeck/src/infer.rs, compiler/mesh-typeck/src/builtins.rs, compiler/mesh-codegen/src/mir/lower.rs, compiler/mesh-codegen/src/codegen/intrinsics.rs, compiler/mesh-codegen/src/codegen/expr.rs, compiler/meshc/tests/e2e_m045_s01.rs
  - Verify: cargo test -p meshc --test e2e_m045_s01 m045_s01_bootstrap_api_ -- --nocapture
- [x] **T03: Moved `meshc init --clustered` onto `Node.start_from_env()` and added scaffold/bootstrap proof rails.** — Shrink the primary docs-grade clustered example by rewriting the scaffolded `main.mpl` to delegate bootstrap to the runtime helper, then update the existing scaffold/tooling rails to assert the new contract instead of the old env-parsing shape.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Scaffold generator and README contract in `compiler/mesh-pkg/src/scaffold.rs` | Fail tests loudly if the generated app no longer builds or loses the public `meshc cluster ...` inspection contract. | N/A — generation is synchronous. | Reject stale literal checks that still pin raw env parsing instead of the public bootstrap surface. |
| Existing scaffold/operator rails in `tooling_e2e`, `e2e_m044_s03`, and `scripts/verify-m044-s03.sh` | Update the assertions together so failures represent real contract drift, not stale implementation checks. | Preserve logs and fail closed if the generated app never exposes `/health` or `meshc cluster status` stalls. | Archive raw command output or HTTP bodies instead of treating malformed status data as success. |

## Load Profile

- **Shared resources**: temporary scaffold directories, spawned scaffold processes, local ports, and `.tmp/...` verifier artifacts.
- **Per-operation cost**: one `meshc init --clustered`, one scaffold build/run, and one `meshc cluster status` query per proof case.
- **10x breakpoint**: process cleanup and port allocation flake before logic does; the verifier must keep startup and teardown deterministic.

## Negative Tests

- **Malformed inputs**: generated `main.mpl` still reading `MESH_CLUSTER_COOKIE`/`MESH_NODE_NAME`/`MESH_DISCOVERY_SEED` directly or still calling `Node.start(...)` itself.
- **Error paths**: scaffold build failure, generated app never exposing `/health`, or `meshc cluster status` auth/startup failure.
- **Boundary conditions**: standalone scaffold run, clustered scaffold run, and CLI inspection without adding a fake peer.

## Steps

1. Rewrite the clustered scaffold template so `main.mpl` delegates cluster startup to `Node.start_from_env()` and keeps only HTTP/server logic local.
2. Update the generated README/runtime contract text to stay on the stable `MESH_*` env contract while emphasizing runtime-owned startup and `meshc cluster ...` inspection.
3. Rewrite the scaffold unit/tooling/e2e/verifier assertions so they prove a smaller runtime-owned bootstrap surface instead of raw env-read literals.
4. Extend the new M045 e2e target or equivalent shape assertions so the scaffold contract has a dedicated proof rail beyond legacy M044 coverage.

## Must-Haves

- [ ] Generated clustered `main.mpl` no longer hand-rolls cluster mode detection or direct `Node.start(...)` orchestration.
- [ ] The scaffold README still teaches the public `MESH_*` contract and built-in `meshc cluster ...` inspection path.
- [ ] Tooling and scaffold e2e rails stay green after the bootstrap ownership move.
  - Estimate: 2h
  - Files: compiler/mesh-pkg/src/scaffold.rs, compiler/meshc/tests/tooling_e2e.rs, compiler/meshc/tests/e2e_m044_s03.rs, compiler/meshc/tests/e2e_m045_s01.rs, scripts/verify-m044-s03.sh
  - Verify: cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture
cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture
- [x] **T04: Moved `cluster-proof` onto `Node.start_from_env()` and added the assembled `verify-m045-s01.sh` acceptance rail.** — Prove the helper is real by consuming it in `cluster-proof`, trimming duplicate bootstrap/config/entrypoint logic, and finishing with one fail-closed verifier that replays the new M045 rails plus the protected M044 regression surfaces.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof` startup / Fly entrypoint contract | Preserve fail-closed startup on invalid local or Fly env; do not keep serving with half-valid bootstrap state. | Fail the verifier with retained startup logs and copied artifacts instead of hanging. | Archive raw HTTP/CLI output and treat malformed bootstrap/status payloads as proof failures. |
| Protected M044 operator/public-contract rails | Replay the existing rails in order and stop on the first drift; do not weaken filters to compensate for the new helper. | Preserve per-phase logs and copied artifact directories when a legacy rail stalls or times out. | Reject malformed JSON, bad pointer files, or zero-test runs instead of claiming the slice is green. |

## Load Profile

- **Shared resources**: `cluster-proof` build/test outputs, local ports, spawned proof-app processes, and `.tmp/m045-s01/...` artifact directories.
- **Per-operation cost**: one package build/test run plus the assembled local verifier replay.
- **10x breakpoint**: port conflicts, stale artifacts, and verifier replay churn fail before runtime throughput matters.

## Negative Tests

- **Malformed inputs**: old bootstrap env names, missing cookie with discovery hint, blank seed, invalid node name, and partial Fly identity.
- **Error paths**: runtime bootstrap returns `Err(String)`, entrypoint exits early, and protected operator/public-contract rails fail after the migration.
- **Boundary conditions**: standalone `cluster-proof`, explicit node-name cluster mode, and Fly identity cluster mode.

## Steps

1. Rewrite `cluster-proof/main.mpl` to consume the runtime bootstrap result and keep `config.mpl` only for the remaining continuity/durability/container concerns.
2. Trim `cluster-proof/config.mpl`, `cluster-proof/tests/config.test.mpl`, and `cluster-proof/docker-entrypoint.sh` so bootstrap validation lives in the runtime helper while packaged fail-fast behavior remains honest.
3. Extend `compiler/meshc/tests/e2e_m045_s01.rs` and protected public-contract coverage in `compiler/meshc/tests/e2e_m044_s05.rs` so the new helper is proven on both the tiny scaffold surface and the retained local proof app.
4. Add `scripts/verify-m045-s01.sh` as the slice stopping condition; it must replay the runtime/bootstrap/scaffold rails, `cluster-proof` build/tests, and the protected M044 operator/public-contract rails fail-closed.

## Must-Haves

- [ ] `cluster-proof` no longer owns cluster mode / identity bootstrap in Mesh code.
- [ ] Local and Fly fail-closed bootstrap behavior still works through the runtime helper.
- [ ] `scripts/verify-m045-s01.sh` is authoritative and fails closed on zero-test or stale-artifact drift.
  - Estimate: 3h
  - Files: cluster-proof/main.mpl, cluster-proof/config.mpl, cluster-proof/tests/config.test.mpl, cluster-proof/docker-entrypoint.sh, compiler/meshc/tests/e2e_m044_s05.rs, compiler/meshc/tests/e2e_m045_s01.rs, scripts/verify-m045-s01.sh
  - Verify: cargo run -q -p meshc -- build cluster-proof
cargo run -q -p meshc -- test cluster-proof/tests
bash scripts/verify-m045-s01.sh
