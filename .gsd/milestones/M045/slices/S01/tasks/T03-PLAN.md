---
estimated_steps: 4
estimated_files: 5
skills_used:
  - rust-best-practices
  - test
---

# T03: Migrate `meshc init --clustered` onto the runtime bootstrap surface

**Slice:** S01 — Runtime-Owned Cluster Bootstrap
**Milestone:** M045

## Description

Shrink the primary docs-grade clustered example by rewriting the scaffolded `main.mpl` to delegate bootstrap to the runtime helper, then update the existing scaffold/tooling rails to assert the new contract instead of the old env-parsing shape.

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

- **Malformed inputs**: generated `main.mpl` still reading `MESH_CLUSTER_COOKIE` / `MESH_NODE_NAME` / `MESH_DISCOVERY_SEED` directly or still calling `Node.start(...)` itself.
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

## Verification

- `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture`

## Observability Impact

- Signals added/changed: generated-app startup logs now report runtime-owned bootstrap outcome instead of app-owned env parsing branches.
- How a future agent inspects this: scaffold build logs, generated app stdout/stderr, `/health`, and `meshc cluster status` output retained by the scaffold rails.
- Failure state exposed: generated-app startup failure reason and scaffold verifier phase logs.

## Inputs

- `compiler/mesh-pkg/src/scaffold.rs` — clustered scaffold generator and README template.
- `compiler/meshc/tests/tooling_e2e.rs` — direct `meshc init --clustered` regression rail.
- `compiler/meshc/tests/e2e_m044_s03.rs` — generated-app build/health/cluster-status proof rail.
- `compiler/meshc/tests/e2e_m045_s01.rs` — dedicated bootstrap API test target to extend with scaffold-specific assertions.
- `scripts/verify-m044-s03.sh` — protected assembled scaffold/operator verifier.
- `compiler/mesh-typeck/src/infer.rs` — new Node builtin contract from T02.
- `compiler/mesh-codegen/src/codegen/expr.rs` — runtime bootstrap call lowering from T02.

## Expected Output

- `compiler/mesh-pkg/src/scaffold.rs` — smaller clustered scaffold template using the runtime bootstrap helper.
- `compiler/meshc/tests/tooling_e2e.rs` — updated source-shape and generated-project assertions.
- `compiler/meshc/tests/e2e_m044_s03.rs` — scaffold runtime-truth rail aligned with the new bootstrap path.
- `compiler/meshc/tests/e2e_m045_s01.rs` — additional scaffold-facing bootstrap assertions.
- `scripts/verify-m044-s03.sh` — protected scaffold/operator verifier updated away from raw env-read pinning.
