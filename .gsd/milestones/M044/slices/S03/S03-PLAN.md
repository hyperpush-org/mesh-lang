# S03: Built-in Operator Surfaces & Clustered Scaffold

**Goal:** Make the clustered-app operator story and bootstrap public, standard, and reusable instead of proof-app folklore.
**Demo:** After this: After this: `meshc init --clustered` scaffolds a clustered app, and built-in runtime/CLI surfaces can inspect membership, authority, continuity status, and failover diagnostics without app-defined operator wiring.

## Tasks
- [x] **T01: Added a runtime-owned operator query/diagnostics seam in mesh-rt for membership, authority, continuity, and recent failover state.** — **Slice:** S03 — Built-in Operator Surfaces & Clustered Scaffold
**Milestone:** M044

## Description

S02 made declared clustered execution real, but the only truthful inspection surface is still proof-app HTTP plus stderr. This task makes operator truth runtime-owned by adding a dedicated read-only operator snapshot/query surface in `mesh-rt` that can answer membership, authority, per-key continuity status, and recent failover transitions without depending on app-authored routes or zero-record continuity state.

## Steps

1. Extract a dedicated operator seam under `compiler/mesh-rt/src/dist/` for structured operator data, keeping `node.rs` focused on transport/session mechanics and `continuity.rs` focused on record state transitions.
2. Add authenticated read-only query/reply support over the node transport plus safe Rust helpers that return normalized membership (self + peers), authority, continuity lookup/listing, and bounded recent failover diagnostics.
3. Retain diagnostics as structured runtime entries instead of stderr-only strings, but keep the existing log lines so current proofs and future debugging stay correlated.
4. Add runtime tests that cover zero-record authority/membership truth, bounded diagnostic retention/truncation, and malformed query frame rejection.

## Must-Haves

- [ ] `mesh-rt` exposes a safe read-only operator snapshot/query API without parsing app HTTP or raw FFI payloads in `meshc`.
- [ ] Membership snapshots include the local node plus connected peers and do not disappear when there are zero continuity records.
- [ ] Recent failover/continuity transitions are queryable as structured diagnostics with bounded retention and no cookie leakage.
  - Estimate: 90m
  - Files: compiler/mesh-rt/src/dist/operator.rs, compiler/mesh-rt/src/dist/mod.rs, compiler/mesh-rt/src/dist/node.rs, compiler/mesh-rt/src/dist/continuity.rs, compiler/mesh-rt/src/lib.rs
  - Verify: `cargo test -p mesh-rt operator_query_ -- --nocapture`
`cargo test -p mesh-rt operator_diagnostics_ -- --nocapture`
- [x] **T02: Confirmed that the current runtime operator query path would make `meshc cluster` join the target cluster and pollute the membership snapshot it is supposed to inspect.** — **Slice:** S03 — Built-in Operator Surfaces & Clustered Scaffold
**Milestone:** M044

## Description

S03 only becomes public once ordinary operators can inspect a live node without `cluster-proof` routes. This task adds read-only `meshc cluster` commands on top of the safe `mesh-rt` operator API and proves them against a live two-node declared-handler runtime.

## Steps

1. Add a dedicated `compiler/meshc/src/cluster.rs` command module and wire read-only subcommands for cluster status (membership + authority), continuity lookup, and recent diagnostics with default human output plus `--json`.
2. Make the CLI use the runtime operator client/query seam directly, authenticated via the standard clustered-app cookie/env contract, and fail closed on unreachable or malformed responses.
3. Add `compiler/meshc/tests/e2e_m044_s03.rs` live-node proofs that launch a real two-node declared-handler app, assert zero-record status truth, assert per-key continuity lookup after declared submit, and assert diagnostics surface degraded or owner-loss transitions after a controlled fault.
4. Keep the scope read-only: no manual promotion, mutation, or app HTTP fallback in the new CLI surface.

## Must-Haves

- [ ] `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics` report runtime-owned truth instead of app-authored HTTP payloads.
- [ ] Every command supports `--json` and returns non-zero on timeout, auth, or malformed-response failures.
- [ ] The named `m044_s03_operator_` rail proves zero-record inspection and post-fault diagnostics against a live two-node runtime.
  - Estimate: 90m
  - Files: compiler/meshc/src/main.rs, compiler/meshc/src/cluster.rs, compiler/meshc/Cargo.toml, compiler/meshc/tests/e2e_m044_s03.rs
  - Verify: `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`
  - Blocker: A future task or replan now needs a runtime-side transient authenticated operator query path that does not register a cluster session or send peer/continuity sync, otherwise `meshc cluster status` cannot report truthful membership for the inspected node.
- [x] **T03: Add a transient authenticated operator query transport that never joins the inspected cluster** — The blocker in T02 showed that the current operator helpers are honest for runtime-internal inspection but unusable for `meshc cluster`: they connect as a real node session and mutate membership. This task adds a dedicated transient operator query path in `mesh-rt` that authenticates with the cluster cookie, serves read-only operator snapshots, and closes without registering a peer, sending peer lists, or syncing continuity state.

Steps
1. Split the current operator-query client/server path so transient queries can reuse frame auth and request/reply I/O without calling node-session registration or sync hooks.
2. Add a dedicated request/reply handler for status, continuity lookup/listing, and recent diagnostics that can run outside cluster membership registration.
3. Keep the surface fail-closed and read-only: malformed frames or auth failures reject immediately, diagnostics stay bounded and cookie-free, and transient queries never appear in membership or peer state.
4. Add runtime tests that prove zero-record status truth, malformed query rejection, bounded diagnostics retention, and non-registering transient query behavior.
  - Estimate: 90m
  - Files: compiler/mesh-rt/src/dist/operator.rs, compiler/mesh-rt/src/dist/node.rs, compiler/mesh-rt/src/dist/mod.rs, compiler/mesh-rt/src/dist/continuity.rs, compiler/mesh-rt/src/lib.rs
  - Verify: `cargo test -p mesh-rt operator_query_ -- --nocapture`
`cargo test -p mesh-rt operator_diagnostics_ -- --nocapture`
- [x] **T04: Stopped T04 before dishonest CLI work because auto-mode advanced past incomplete T03 transient transport work.** — With the runtime-side transient query path in place, add the public CLI surface on top of that truthful transport instead of the session-joining peer path. This task wires `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics` to the runtime-owned operator API, keeps the scope read-only, and proves live two-node inspection without the CLI becoming a visible peer.

Steps
1. Add a dedicated `compiler/meshc/src/cluster.rs` command module and wire read-only subcommands for cluster status, per-key continuity lookup, and recent diagnostics with default human output plus `--json`.
2. Make the CLI use the transient authenticated operator client/query seam directly, and fail closed on timeout, auth failure, malformed reply, or unreachable target.
3. Add live `compiler/meshc/tests/e2e_m044_s03.rs` proofs that assert zero-record membership/authority truth, per-key continuity lookup after declared work submit, and post-fault diagnostics after controlled owner-loss or degrade events.
4. Prove the key blocker outcome explicitly: querying a live node through `meshc cluster status` must not cause the inspected runtime to report the CLI as a cluster peer.
  - Estimate: 90m
  - Files: compiler/meshc/src/main.rs, compiler/meshc/src/cluster.rs, compiler/meshc/Cargo.toml, compiler/meshc/tests/e2e_m044_s03.rs
  - Verify: `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`
  - Blocker: T03 is still the real next task. Until the transient authenticated operator query transport lands in `mesh-rt`, T04 cannot honestly ship the read-only `meshc cluster` commands described by the plan.
- [x] **T05: Recorded that `meshc init --clustered` remains blocked because the public cluster CLI and scaffold proof rails do not exist yet.** — Once the read-only operator CLI is truthful, expose the public clustered bootstrap path. This task extends `meshc init` so users can generate a buildable clustered app with a narrow declared work boundary, the standard `MESH_*` bootstrap contract, and no copied `cluster-proof` internals.

Steps
1. Extend `mesh_pkg::scaffold_project` and `meshc init` so `meshc init --clustered <name>` emits a multi-file project with a valid `[cluster]` block and a declared `work` target.
2. Standardize the scaffold on `MESH_CLUSTER_COOKIE`, `MESH_NODE_NAME` (or derived identity when absent), `MESH_DISCOVERY_SEED`, `MESH_CLUSTER_PORT`, `MESH_CONTINUITY_ROLE`, and `MESH_CONTINUITY_PROMOTION_EPOCH` instead of any `CLUSTER_PROOF_*` literals.
3. Keep the generated app minimal and honest: declared work submit through the runtime-owned clustered boundary, and inspection through the public `meshc cluster` commands rather than app-authored operator routes.
4. Extend tooling/e2e coverage so the generated project builds, starts in clustered mode, and can be inspected through the new CLI without any proof-app helper modules.
  - Estimate: 90m
  - Files: compiler/mesh-pkg/src/scaffold.rs, compiler/mesh-pkg/src/lib.rs, compiler/meshc/src/main.rs, compiler/meshc/tests/tooling_e2e.rs, compiler/meshc/tests/e2e_m044_s03.rs
  - Verify: `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`
`cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture`
  - Blocker: T05 is still blocked on the unfinished T04 dependency chain. Until the runtime has a truthful non-session operator inspection path and `meshc cluster` exists as a real public command/test surface, `meshc init --clustered` cannot honestly ship the public scaffold story described by S03.
- [x] **T06: Recorded that T06 remains blocked because the S03 verifier, public `meshc cluster` CLI, and clustered scaffold surfaces still do not exist.** — After the transient operator transport, public CLI, and clustered scaffold all exist, close the slice with one fail-closed acceptance rail and public docs that describe the actual shipped scope. The docs need to point at `meshc init --clustered` plus the read-only `meshc cluster` inspection commands without claiming automatic promotion or a finished `cluster-proof` rewrite.

Steps
1. Add `scripts/verify-m044-s03.sh` that replays `scripts/verify-m044-s02.sh`, runs the named `m044_s03_operator_` and `m044_s03_scaffold_` filters, fails closed on `running 0 tests`, checks scaffold output for the generic `MESH_*` contract and absence of `CLUSTER_PROOF_*`, and archives live operator/scaffold artifacts under `.tmp/m044-s03/verify/`.
2. Update `README.md` plus the tooling/getting-started VitePress pages so the public command surface shows both `meshc init` and `meshc init --clustered`, names the new `meshc cluster` inspection commands, and keeps low-level distributed docs separate from the new scaffolded story.
3. Keep the docs honest about scope: S03 is read-only operator inspection only; bounded automatic promotion and the full `cluster-proof` rewrite remain later slices.
4. Rebuild the docs site and rerun the assembled verifier so the public text and proof rail match.
  - Estimate: 60m
  - Files: scripts/verify-m044-s03.sh, README.md, website/docs/docs/getting-started/index.md, website/docs/docs/tooling/index.md
  - Verify: `bash scripts/verify-m044-s03.sh`
`npm --prefix website run build`
  - Blocker: The unfinished T03/T04/T05 dependency chain still blocks S03 closeout. Until the runtime has a truthful non-registering operator query transport, `meshc cluster` exists as a real public command with tests, and `meshc init --clustered` has an honest scaffold/e2e rail, T06 cannot ship the assembled verifier or public docs promised by the slice plan.
