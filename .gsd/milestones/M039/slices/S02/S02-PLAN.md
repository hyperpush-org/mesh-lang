# S02: Native Cluster Work Routing Proof App

**Goal:** Extend `cluster-proof` with one typed work-routing endpoint that proves Mesh moved work internally across the cluster by reporting ingress node, target node, and execution node separately while preserving `/membership` as the truthful membership diagnostic surface.
**Demo:** After this: After this: one proof endpoint can show ingress node and execution node separately, proving that Mesh moved work internally across the cluster.

## Tasks
- [x] **T01: Started the cluster-proof work-routing implementation, but stopped at a real runtime mismatch: Mesh HTTP handlers are not actor contexts and the distributed transport still only carries raw scalar bytes.** — Add the actual routing proof surface inside `cluster-proof` instead of inventing a service layer. The endpoint should stay read-only and typed: it captures ingress from the handler context, chooses a deterministic peer-preferred target from live membership, spawns a one-shot worker on the target node (or locally when no peer exists), waits for a reply with a bounded timeout, and returns a JSON body that makes internal routing obvious.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Runtime `Node.self()` / `Node.list()` membership snapshot | Return a truthful self-only route decision and keep `/membership` untouched instead of inventing peers. | N/A | Treat malformed node identities as an app bug and fail the follow-up e2e proof instead of normalizing bad routing state. |
| `Node.spawn` / local `spawn` request-reply path | Return an explicit error response with `request_id` and target metadata instead of hanging the HTTP connection. | Bound the handler wait with a fixed timeout and log which target missed the reply. | Treat missing or malformed reply payloads as route failure, not partial success. |
| Mesh HTTP JSON response path | Keep the response on typed struct/string/bool fields and avoid inline integer success bindings that have been brittle in handlers. | Fail fast through the handler error response; do not retry in-process. | Treat missing ingress/execution fields as a contract failure for the e2e proof. |

## Load Profile

- **Shared resources**: one HTTP request actor, one spawned worker, and cross-node message delivery per work request.
- **Per-operation cost**: one membership snapshot, one local or remote actor spawn, one reply wait, and one JSON encode.
- **10x breakpoint**: request-time actor churn and timeout backlog before CPU; the endpoint must stay side-effect free and bounded.

## Negative Tests

- **Malformed inputs**: direct HTTP callers cannot supply or override the target node; request ids and target selection come from server-side state only.
- **Error paths**: no peers available, remote spawn failure, or reply timeout must produce truthful fallback or error responses.
- **Boundary conditions**: single-node self fallback, two-node peer-preferred routing, and preserved `/membership` output alongside the new route.

## Steps

1. Add `cluster-proof/work.mpl` with typed route/worker reply structs, deterministic peer-preferred target selection, and request-id generation.
2. Implement a one-shot execution actor that logs request correlation data on the execution node and sends a typed reply back to the handler pid without shipping local handles across nodes.
3. Wire a new read-only route in `cluster-proof/main.mpl`, reusing or lightly extracting membership snapshot helpers from `cluster-proof/cluster.mpl` while leaving `/membership` unchanged.
4. Add `cluster-proof/tests/work.test.mpl` coverage for pure target-selection and standalone-fallback helpers, and keep the route response on string/bool-heavy typed payloads.

## Must-Haves

- [ ] `GET /membership` stays unchanged while the new proof route exposes ingress, target, execution, `routed_remotely`, and request correlation truth.
- [ ] When peers exist, target selection deterministically prefers a non-self peer; when no peer exists, the endpoint reports a truthful local fallback.
- [ ] Execution-node logs and handler timeout/error responses share the same `request_id` so later failures are inspectable.
  - Estimate: 2h
  - Files: cluster-proof/main.mpl, cluster-proof/cluster.mpl, cluster-proof/work.mpl, cluster-proof/tests/work.test.mpl
  - Verify: `cargo run -q -p meshc -- test cluster-proof/tests`
`cargo run -q -p meshc -- build cluster-proof`
  - Blocker: `cluster-proof/work.mpl` does not compile; `cluster-proof/tests/work.test.mpl` does not compile; `/work` is wired in `cluster-proof/main.mpl` but the app does not build because the route module is blocked; the next unit needs a runtime-supported return path instead of handler-side mailbox waiting.
- [x] **T02: Started the coordinator/result-registry refactor for `/work`, but the new Mesh service/actor path still does not compile or verify.** — 1. Refactor `cluster-proof/work.mpl` so the HTTP handler only captures request context and calls a local registered coordinator service; remove `self()` / `receive` from the HTTP handler path.
2. Add an ingress-owned coordinator/result-registry pattern using local `Process.register(...)` and distributed `Global.register(...)` / lookup where needed so remote work can report completion back without shipping handler pids or other local-only handles.
3. Keep all cross-node spawn/send inputs scalar-only (`request_token`, membership indexes, booleans, and other raw-safe correlation values) and reconstruct string-heavy JSON fields on the ingress node before returning the HTTP response.
4. Leave `/membership` untouched, repair `cluster-proof/tests/work.test.mpl` around pure selection/correlation helpers, and get `cluster-proof` building again.
  - Estimate: 3h
  - Files: cluster-proof/main.mpl, cluster-proof/cluster.mpl, cluster-proof/work.mpl, cluster-proof/tests/work.test.mpl
  - Verify: cargo run -q -p meshc -- test cluster-proof/tests
cargo run -q -p meshc -- build cluster-proof
- [x] **T03: Added the direct-port S02 e2e proofs and repaired cluster-proof so /work returns truthful ingress/target/execution data with matching execution-node log evidence.** — 1. Add `compiler/meshc/tests/e2e_m039_s02.rs` by reusing the S01 harness patterns for repo-root resolution, port selection, child-process lifecycle, raw HTTP GETs, and per-node stdout/stderr capture.
2. Add a two-node proof that hits node A and node B directly, asserts `ingress_node` matches the contacted port, `target_node` / `execution_node` match the peer, `routed_remotely` is true, and the peer log contains the matching `request_id` execution line.
3. Add a single-node proof that starts one node and asserts truthful local fallback (`target_node == execution_node == ingress_node`, `routed_remotely == false`, no invented peer).
4. Fail closed on malformed JSON, missing route fields, early process exits, or zero-test filters by preserving raw bodies and log paths under `.tmp/m039-s02/`.
  - Estimate: 2h
  - Files: compiler/meshc/tests/e2e_m039_s02.rs
  - Verify: cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_routes_work_to_peer_and_logs_execution -- --nocapture
cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_falls_back_locally_without_peers -- --nocapture
- [x] **T04: Added the canonical S02 verifier with a fail-closed phase ledger, S01 prerequisite replay, and copied node-artifact bundles.** — 1. Add `scripts/verify-m039-s02.sh` as the canonical local replay wrapper for the slice.
2. Run `cluster-proof/tests`, rebuild `cluster-proof`, recheck S01 convergence explicitly, and then run the named S02 e2e filters so routing proof never hides a broken cluster bootstrap.
3. Preserve per-phase logs and per-node artifacts under `.tmp/m039-s02/verify/`, and fail closed if any named filter runs zero tests, any phase stalls, or any node log is missing.
  - Estimate: 1h
  - Files: scripts/verify-m039-s02.sh
  - Verify: bash scripts/verify-m039-s02.sh
