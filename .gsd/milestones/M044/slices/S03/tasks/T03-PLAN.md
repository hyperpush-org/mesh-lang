---
estimated_steps: 6
estimated_files: 5
skills_used: []
---

# T03: Add a transient authenticated operator query transport that never joins the inspected cluster

The blocker in T02 showed that the current operator helpers are honest for runtime-internal inspection but unusable for `meshc cluster`: they connect as a real node session and mutate membership. This task adds a dedicated transient operator query path in `mesh-rt` that authenticates with the cluster cookie, serves read-only operator snapshots, and closes without registering a peer, sending peer lists, or syncing continuity state.

Steps
1. Split the current operator-query client/server path so transient queries can reuse frame auth and request/reply I/O without calling node-session registration or sync hooks.
2. Add a dedicated request/reply handler for status, continuity lookup/listing, and recent diagnostics that can run outside cluster membership registration.
3. Keep the surface fail-closed and read-only: malformed frames or auth failures reject immediately, diagnostics stay bounded and cookie-free, and transient queries never appear in membership or peer state.
4. Add runtime tests that prove zero-record status truth, malformed query rejection, bounded diagnostics retention, and non-registering transient query behavior.

## Inputs

- `compiler/mesh-rt/src/dist/operator.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/dist/wire.rs`

## Expected Output

- `compiler/mesh-rt/src/dist/operator.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/dist/mod.rs`
- `compiler/mesh-rt/src/lib.rs`

## Verification

`cargo test -p mesh-rt operator_query_ -- --nocapture`
`cargo test -p mesh-rt operator_diagnostics_ -- --nocapture`

## Observability Impact

- Signals added/changed: scaffold smoke failures identify which public contract drifted — manifest, bootstrap env parsing, declared work submit path, or operator CLI inspection.
- How a future agent inspects this: rerun `test_init_clustered_creates_project` or the named `m044_s03_scaffold_` filter against the generated temp app.
- Failure state exposed: missing clustered files, invalid `[cluster]` declaration, leaked `CLUSTER_PROOF_*` literal, or startup/query failure under the standard `MESH_*` env contract.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Clustered scaffold template emission in `compiler/mesh-pkg/src/scaffold.rs` | Fail `meshc init --clustered` before creating a half-written project tree. | N/A — local file writes. | Reject invalid template data (manifest, declaration, or env contract) in tests before release. |
| Manifest/build validation through `compiler/meshc` | Treat invalid `[cluster]` or declared work targets as scaffold regressions. | Bound live scaffold smoke tests and surface the failing phase. | Fail the generated project build loudly instead of degrading to hello-world local mode. |
| Standard `MESH_*` bootstrap contract | Fail startup with explicit missing or invalid env messages. | Bound CLI inspection and startup waits in e2e. | Reject partial identity or cookie state instead of synthesizing proof-app defaults. |

## Load Profile

- **Shared resources**: temp project directories, compiler builds, live node ports, and operator CLI smoke targets.
- **Per-operation cost**: one scaffold write, one build, and one live two-node smoke for the generated app.
- **10x breakpoint**: compile time and temp-port contention fail before runtime CPU; the template should stay small and predictable.

## Negative Tests

- **Malformed inputs**: missing `MESH_CLUSTER_COOKIE`, malformed `MESH_NODE_NAME`, blank discovery seed, and invalid `[cluster]` declarations in generated files.
- **Error paths**: generated app build failure, clustered startup rejection, and CLI query failure against the scaffolded app.
- **Boundary conditions**: standalone local build, two-node clustered run, and explicit checks that no generated file contains `CLUSTER_PROOF_*`.
