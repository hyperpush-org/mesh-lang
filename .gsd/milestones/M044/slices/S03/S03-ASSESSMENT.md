# M044 / S03 closeout assessment

## Verdict

S03 is **not ready for slice completion** in the current tree. The runtime operator payload layer from T01 exists, but the public operator CLI, non-registering query transport, clustered scaffold, slice verifier, and docs surfaces required by the slice goal are still missing.

## Verified current state

- `compiler/mesh-rt/src/dist/operator.rs` still routes remote `query_operator_*` calls through `execute_query(...)`, which requires `node_state()` and an already-registered `state.sessions[target]` entry. That is still the session-joining path T02 identified as dishonest for operator inspection.
- `compiler/meshc/src/main.rs` still exposes only `Init { name }`; there is no `cluster` subcommand, no `--clustered` flag, and no `compiler/meshc/src/cluster.rs` module.
- `compiler/mesh-pkg/src/scaffold.rs` still emits only `mesh.toml` plus a hello-world `main.mpl`.
- `compiler/meshc/tests/tooling_e2e.rs` still contains only the plain `test_init_creates_project` path for scaffolding.
- `compiler/meshc/tests/e2e_m044_s03.rs` does not exist.
- `scripts/verify-m044-s03.sh` does not exist.
- `cluster-proof/` still consumes proof-app env names (`CLUSTER_PROOF_COOKIE`, `CLUSTER_PROOF_NODE_BASENAME`, `CLUSTER_PROOF_ADVERTISE_HOST`, `CLUSTER_PROOF_DURABILITY`), so it is not yet the generic public clustered-app bootstrap contract the S03 scaffold/docs work wants to teach.

## Why the slice cannot be closed honestly yet

The slice goal is public and assembled: `meshc init --clustered` must scaffold a clustered app, and built-in runtime/CLI surfaces must inspect membership, authority, continuity status, and failover diagnostics without app-authored operator wiring. The current checkout still lacks every public surface that makes that claim true.

Closing the slice from this state would misrepresent the repo in two ways:

1. `meshc cluster` does not exist, and the only current remote query path would still mutate membership by joining the inspected cluster.
2. `meshc init --clustered` does not exist, so there is no public clustered scaffold to verify or document.

## Resume order for the next fresh-context unit

1. **Finish T03 in `mesh-rt`.** Add a transient authenticated operator query transport that performs cookie auth but answers one query before `register_session()`, with no peer-list broadcast, no global/continuity sync, and no nodeup/nodedown side effects.
2. **Add the Rust-side query helper for `meshc`.** The CLI needs a non-node-started client path that can open TLS, authenticate with the cluster cookie, send the operator query, read the reply, and close.
3. **Land T04 public CLI work.** Add `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics` with human output + `--json`, and prove against live `cluster-proof` nodes that the CLI never appears in membership.
4. **Land T05 scaffold work.** Extend `meshc init` / `mesh_pkg::scaffold_project` with `--clustered`, emitting a minimal multi-file clustered app and a generic `MESH_*` bootstrap contract.
5. **Land T06 assembled proof/docs.** Add `scripts/verify-m044-s03.sh`, update README + docs pages, rerun the full slice rail, and only then write `S03-SUMMARY.md`, `S03-UAT.md`, and call `gsd_complete_slice`.

## Resume checkpoints / likely touch points

- Runtime transport: `compiler/mesh-rt/src/dist/node.rs`, `compiler/mesh-rt/src/dist/operator.rs`, `compiler/mesh-rt/src/lib.rs`
- CLI: `compiler/meshc/src/main.rs`, new `compiler/meshc/src/cluster.rs`
- Scaffold: `compiler/mesh-pkg/src/scaffold.rs`, `compiler/mesh-pkg/src/lib.rs`
- Tests: `compiler/meshc/tests/e2e_m044_s03.rs`, `compiler/meshc/tests/tooling_e2e.rs`
- Assembled verification/docs: `scripts/verify-m044-s03.sh`, `README.md`, `website/docs/docs/getting-started/index.md`, `website/docs/docs/tooling/index.md`

## Important constraint for the next unit

Do **not** call `gsd_complete_slice` until the full S03 rail is green. Right now the slice checkbox should remain open.