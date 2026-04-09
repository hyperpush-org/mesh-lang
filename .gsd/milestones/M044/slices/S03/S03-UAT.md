# S03: Built-in Operator Surfaces & Clustered Scaffold — UAT

**Milestone:** M044
**Written:** 2026-03-30T02:32:18.040Z

# S03: Built-in Operator Surfaces & Clustered Scaffold — UAT

**Milestone:** M044
**Written:** 2026-03-28

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: S03 is a CLI/runtime/scaffold slice. The truthful proof needs both live clustered runtime behavior and artifact-level checks over the generated scaffold and docs surface.

## Preconditions

1. Work from the repo root.
2. The Rust workspace builds locally.
3. `cluster-proof/` is available for the live two-node operator replay.
4. Node ports on loopback are free.
5. The docs site dependencies are installed so `npm --prefix website run build` can run.

## Smoke Test

Run the assembled verifier from the repo root:

```bash
bash scripts/verify-m044-s03.sh
```

Expected: the script exits 0, writes `.tmp/m044-s03/verify/status.txt` with `ok`, and records every named phase as `passed` in `.tmp/m044-s03/verify/phase-report.txt`.

## Test Cases

### 1. Inspect a live two-node clustered runtime without joining it

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture
   ```
2. Wait for the test to start two `cluster-proof` nodes and replay the operator CLI against them.
3. Inspect the retained artifacts under the newest `.tmp/m044-s03/operator-status-truth-*` and `.tmp/m044-s03/operator-continuity-diagnostics-*` directories.
4. **Expected:**
   - `meshc cluster status --json` returns runtime-owned membership + authority JSON.
   - The inspected runtime still reports only the real cluster members after the CLI query; the CLI never appears as a peer.
   - `meshc cluster continuity --json` returns the submitted request key from runtime continuity state.
   - `meshc cluster diagnostics --json` returns a bounded transition entry for the controlled fault (`degraded` or `owner_lost`, depending on the live target selected by the harness).
   - A wrong cookie path exits non-zero with an explicit auth/handshake error.

### 2. Generate the public clustered scaffold and inspect it through the built-in CLI

1. Run:
   ```bash
   cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture
   cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture
   ```
2. Let the tooling test create a clustered project and assert the generated files.
3. Let the scaffold e2e build the generated app, start it with the generic `MESH_*` env contract, and run `meshc cluster status --json` against that app.
4. **Expected:**
   - `meshc init --clustered` creates `mesh.toml`, `main.mpl`, `work.mpl`, and `README.md`.
   - The generated files contain `MESH_CLUSTER_COOKIE`, `MESH_NODE_NAME`, `MESH_DISCOVERY_SEED`, `MESH_CLUSTER_PORT`, `MESH_CONTINUITY_ROLE`, and `MESH_CONTINUITY_PROMOTION_EPOCH`.
   - No generated file contains any `CLUSTER_PROOF_*` literal.
   - The generated app builds and starts.
   - `meshc cluster status --json` reports the generated app’s node as `local_node` with no extra peers.

### 3. Rebuild the public docs surface

1. Run:
   ```bash
   npm --prefix website run build
   ```
2. Inspect `README.md`, `website/docs/docs/getting-started/index.md`, and `website/docs/docs/tooling/index.md`.
3. **Expected:**
   - The docs build exits 0.
   - The public text mentions `meshc init --clustered`.
   - The public text mentions `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics`.
   - The tooling page explicitly describes these commands as read-only inspection surfaces.

## Edge Cases

### Auth mismatch on operator inspection

1. Start a live clustered runtime through the operator e2e or by running the cluster-proof nodes manually.
2. Run:
   ```bash
   meshc cluster status <node-name@host:port> --json --cookie wrong-cookie
   ```
3. **Expected:** the command exits non-zero and reports an auth/handshake failure; it must not silently succeed or mutate membership.

### Zero-record operator truth

1. Start the live two-node runtime but do not submit any work yet.
2. Run:
   ```bash
   meshc cluster status <node-name@host:port> --json
   ```
3. **Expected:** the command still returns membership and authority. `replication_health` may truthfully remain `local_only` even though peers are connected; that is the expected zero-record state in this slice.

### Generated scaffold contract drift

1. Run:
   ```bash
   bash scripts/verify-m044-s03.sh
   ```
2. Inspect `.tmp/m044-s03/verify/scaffold-contract.scaffold-check.log` if the script fails.
3. **Expected:** the verifier fail-closes if the scaffold stops generating the generic `MESH_*` contract or reintroduces any `CLUSTER_PROOF_*` literal.

## Failure Signals

- `meshc cluster ...` exits 0 while the inspected node’s membership suddenly includes the CLI process.
- `meshc cluster ... --json` returns malformed or partial JSON.
- `cargo test -p meshc --test e2e_m044_s03 ...` reports `running 0 tests`.
- The generated scaffold contains `CLUSTER_PROOF_*` names or lacks the `[cluster]` declaration.
- `npm --prefix website run build` fails after docs edits.
- `.tmp/m044-s03/verify/status.txt` is not `ok`.

## Requirements Proved By This UAT

- R065 — Proves that clustered apps now expose built-in runtime-owned operator truth through the public `meshc cluster` CLI.
- R066 — Proves that `meshc init --clustered` generates a real clustered app on the public `MESH_*` contract.

## Not Proven By This UAT

- Bounded automatic promotion and stale-primary fencing after promotion; those remain S04 work.
- The final `cluster-proof` rewrite onto the public clustered-app model; that remains S05 work.
- A richer operator remediation surface beyond read-only inspection.

## Notes for Tester

- The S03 operator e2e intentionally chooses the reachable live operator target from runtime truth instead of assuming every advertised address will be equally queryable on every local dual-stack path.
- Do not treat `replication_health=local_only` as a failure on the zero-record operator surface; that is the expected runtime answer before continuity activity exists.
- The scaffold replay is intentionally narrow. Its acceptance bar is “generate, build, start, inspect through `meshc cluster status`," not “behave like the full proof app.”

