---
estimated_steps: 24
estimated_files: 8
skills_used: []
---

# T01: Move cluster-proof onto the public MESH_* bootstrap contract

Retarget `cluster-proof` startup, config, and live harnesses to the same clustered-app contract that `meshc init --clustered` already ships. This task should remove the proof-app-specific bootstrap/env dialect from the paths ordinary operators and the final docs teach, while keeping `CLUSTER_PROOF_WORK_DELAY_MS` as the only proof-only timing knob.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Config/bootstrap parsing in `cluster-proof/config.mpl` and `cluster-proof/main.mpl` | Fail startup with explicit config errors instead of silently falling back to standalone mode. | Bound live harness waits and surface which node never became ready. | Reject malformed `MESH_*` identity/cookie/topology values and keep the node down. |
| Container bootstrap in `cluster-proof/docker-entrypoint.sh` and `cluster-proof/fly.toml` | Stop the image before the binary starts and record the failing env contract. | N/A — startup validation path. | Reject partial identity or contradictory role/epoch env instead of synthesizing proof-app defaults. |
| Live harnesses in `compiler/meshc/tests/e2e_m044_s03.rs`, `compiler/meshc/tests/e2e_m044_s04.rs`, and `compiler/meshc/tests/e2e_m044_s05.rs` | Fail the affected rail with retained stdout/stderr artifacts. | Mark the exact startup/membership stage that stalled. | Reject malformed membership/authority payloads instead of letting the public-contract proof pass. |

## Load Profile

- **Shared resources**: live node ports, env-driven bootstrap parsing, and two-node startup/membership convergence in the M044 e2e harnesses.
- **Per-operation cost**: one process start per node plus membership polling and read-only operator inspection.
- **10x breakpoint**: local port contention and startup retries fail before app logic; the contract must stay small and deterministic.

## Negative Tests

- **Malformed inputs**: blank or missing `MESH_CLUSTER_COOKIE`, malformed `MESH_NODE_NAME`, blank `MESH_DISCOVERY_SEED`, and contradictory `MESH_CONTINUITY_ROLE` / `MESH_CONTINUITY_PROMOTION_EPOCH` values.
- **Error paths**: cluster identity hints without a cookie, Fly identity missing one required value, and old `CLUSTER_PROOF_*` bootstrap names no longer honored.
- **Boundary conditions**: standalone mode without a cookie, clustered mode with explicit `MESH_NODE_NAME`, and Fly-derived identity without `MESH_NODE_NAME`.

## Steps

1. Replace the proof-app-specific cookie and node-identity helpers in `cluster-proof/config.mpl` / `cluster-proof/main.mpl` with the public `MESH_CLUSTER_COOKIE` and `MESH_NODE_NAME` contract, while keeping Fly identity fallback and durability/topology validation honest.
2. Update `cluster-proof/docker-entrypoint.sh`, `cluster-proof/fly.toml`, and `cluster-proof/tests/config.test.mpl` so the local same-image and Fly startup paths use the same public contract and only `CLUSTER_PROOF_WORK_DELAY_MS` remains proof-specific.
3. Rewire `compiler/meshc/tests/e2e_m044_s03.rs` and `compiler/meshc/tests/e2e_m044_s04.rs` to launch `cluster-proof` with the public env surface, and add `compiler/meshc/tests/e2e_m044_s05.rs` coverage for live `cluster-proof` public-contract startup.
4. Keep the new proof rail fail-closed on bootstrap drift by asserting the proof app starts, joins, and reports membership/authority truth without any dependency on `CLUSTER_PROOF_COOKIE`, `CLUSTER_PROOF_NODE_BASENAME`, or `CLUSTER_PROOF_ADVERTISE_HOST`.

## Must-Haves

- [ ] `cluster-proof` bootstrap/config accepts the same public `MESH_*` contract the scaffolded app uses.
- [ ] The same-image local and Fly bootstrap paths stay truthful on the public contract, with `CLUSTER_PROOF_WORK_DELAY_MS` as the only remaining proof-only env knob.
- [ ] The M044 live harnesses and a new S05 e2e rail prove the contract with real cluster startup, not just file-content greps.

## Inputs

- `cluster-proof/config.mpl`
- `cluster-proof/main.mpl`
- `cluster-proof/docker-entrypoint.sh`
- `cluster-proof/fly.toml`
- `cluster-proof/tests/config.test.mpl`
- `compiler/meshc/tests/e2e_m044_s03.rs`
- `compiler/meshc/tests/e2e_m044_s04.rs`
- `compiler/mesh-pkg/src/scaffold.rs`

## Expected Output

- `cluster-proof/config.mpl`
- `cluster-proof/main.mpl`
- `cluster-proof/docker-entrypoint.sh`
- `cluster-proof/fly.toml`
- `cluster-proof/tests/config.test.mpl`
- `compiler/meshc/tests/e2e_m044_s03.rs`
- `compiler/meshc/tests/e2e_m044_s04.rs`
- `compiler/meshc/tests/e2e_m044_s05.rs`

## Verification

cargo run -q -p meshc -- test cluster-proof/tests/config.test.mpl
cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture
cargo test -p meshc --test e2e_m044_s05 m044_s05_public_contract_ -- --nocapture

## Observability Impact

- Signals added/changed: startup/config errors should name the failing `MESH_*` input without leaking secrets, and the new `e2e_m044_s05` artifacts should retain node stdout/stderr for public-contract failures.
- How a future agent inspects this: rerun `cluster-proof/tests/config.test.mpl`, then the named `m044_s05_public_contract_` e2e filter, then inspect the retained stdout/stderr logs for the node that never converged.
- Failure state exposed: missing cookie/identity/topology validation, startup refusal, or mismatched membership/authority truth under the public contract.
