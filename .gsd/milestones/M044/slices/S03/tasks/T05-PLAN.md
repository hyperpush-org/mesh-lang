---
estimated_steps: 6
estimated_files: 5
skills_used: []
---

# T05: Add `meshc init --clustered` and the public clustered bootstrap template

Once the read-only operator CLI is truthful, expose the public clustered bootstrap path. This task extends `meshc init` so users can generate a buildable clustered app with a narrow declared work boundary, the standard `MESH_*` bootstrap contract, and no copied `cluster-proof` internals.

Steps
1. Extend `mesh_pkg::scaffold_project` and `meshc init` so `meshc init --clustered <name>` emits a multi-file project with a valid `[cluster]` block and a declared `work` target.
2. Standardize the scaffold on `MESH_CLUSTER_COOKIE`, `MESH_NODE_NAME` (or derived identity when absent), `MESH_DISCOVERY_SEED`, `MESH_CLUSTER_PORT`, `MESH_CONTINUITY_ROLE`, and `MESH_CONTINUITY_PROMOTION_EPOCH` instead of any `CLUSTER_PROOF_*` literals.
3. Keep the generated app minimal and honest: declared work submit through the runtime-owned clustered boundary, and inspection through the public `meshc cluster` commands rather than app-authored operator routes.
4. Extend tooling/e2e coverage so the generated project builds, starts in clustered mode, and can be inspected through the new CLI without any proof-app helper modules.

## Inputs

- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/src/main.rs`
- `compiler/meshc/src/cluster.rs`
- `compiler/meshc/tests/tooling_e2e.rs`

## Expected Output

- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/mesh-pkg/src/lib.rs`
- `compiler/meshc/src/main.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/e2e_m044_s03.rs`

## Verification

`cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`
`cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture`
