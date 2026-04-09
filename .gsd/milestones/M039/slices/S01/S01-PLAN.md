# S01: General DNS Discovery & Membership Truth

**Goal:** Turn the current manual peer-connect story into a runtime-owned DNS-first discovery path with truthful membership visibility through a new narrow proof app.
**Demo:** After this: After this: multiple nodes started from the same image can auto-discover and report truthful membership locally and on Fly without manual peer lists.

## Tasks
- [x] **T01: Added a runtime-owned DNS discovery loop in mesh-rt with candidate filtering, IPv6-safe node parsing, and validated handshake identities.** — ---
estimated_steps: 4
estimated_files: 4
skills_used:
  - rust-best-practices
  - test
---
Close the real blocker first: discovery has to live in `mesh-rt`, not in Mesh application code, and it has to connect by candidate socket address while preserving the remote node’s advertised identity from the handshake. Implement the first provider as plain DNS A/AAAA lookup plus a fixed cluster port, but make the reconcile logic explicit and testable instead of burying it in ad hoc peer-list side effects.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| System DNS resolution for the configured seed name | Keep existing sessions untouched, log the failing seed/reason, and retry on the next reconcile tick. | Record a skipped reconcile and leave current membership intact instead of blocking the runtime. | Discard unusable addresses and keep discovery state truthful rather than synthesizing fake peers. |
| Outbound `mesh_node_connect` + handshake | Leave the session table unchanged and log the target/reason so the next tick can retry cleanly. | Back off to the next reconcile interval instead of spawning unbounded connect threads. | Reject invalid advertised names or cookie mismatches without mutating membership truth. |

## Load Profile

- **Shared resources**: DNS lookups, the node session table, and reconcile-triggered outbound connect attempts.
- **Per-operation cost**: one DNS lookup plus up to N filtered connection attempts per reconcile tick.
- **10x breakpoint**: too-frequent reconcile intervals or large answer sets would cause duplicate dial churn first, so filtering and dedupe must happen before connect.

## Negative Tests

- **Malformed inputs**: blank discovery seed, invalid cluster port, zero/negative reconcile interval, and malformed candidate host strings.
- **Error paths**: DNS returns no answers, connect target refuses handshake, or the same candidate appears repeatedly across ticks.
- **Boundary conditions**: duplicate A/AAAA answers, self-address candidates, already-connected peers, and bracketed IPv6 literals in advertised names.

## Steps

1. Add `compiler/mesh-rt/src/dist/discovery.rs` with config parsing, candidate normalization, dedupe, self/connected filtering, and the periodic reconcile loop for one DNS seed name plus fixed cluster port.
2. Wire discovery startup from `mesh_node_start` in `compiler/mesh-rt/src/dist/node.rs`, reusing `mesh_node_connect` with synthesized temporary targets so the handshake-provided remote name remains the membership source of truth.
3. Emit discovery logs that expose provider, seed, accepted/rejected candidates, and last failure reason without ever echoing the shared cookie.
4. Add unit coverage in the discovery module for candidate filtering, duplicate suppression, self-filtering, and IPv6/bracketed-name handling.

## Must-Haves

- [ ] DNS discovery runs inside `mesh-rt` and does not require Mesh code to hand-roll peer resolution.
- [ ] Discovery candidates are filtered against self and already-connected peers before any dial attempt.
- [ ] Advertised node identity remains the canonical membership truth even though discovery starts from a shared seed hostname.

## Verification

- `cargo test -p mesh-rt discovery_ -- --nocapture`

## Observability Impact

- Signals added/changed: discovery reconcile logs with candidate counts and reject reasons.
- How a future agent inspects this: rerun `cargo test -p mesh-rt discovery_ -- --nocapture` and inspect runtime logs from `scripts/verify-m039-s01.sh`.
- Failure state exposed: whether convergence broke in DNS resolution, candidate filtering, or outbound connect/handshake.

  - Estimate: 2h
  - Files: compiler/mesh-rt/src/dist/discovery.rs, compiler/mesh-rt/src/dist/node.rs, compiler/mesh-rt/src/dist/mod.rs, mesher/main.mpl
  - Verify: `cargo test -p mesh-rt discovery_ -- --nocapture`
- [x] **T02: Built the `cluster-proof/` app skeleton, config tests, and a live convergence harness, but the proof app still needs one more pass because startup/compiler failures are not fully retired.** — ---
estimated_steps: 4
estimated_files: 5
skills_used:
  - test
---
Once runtime discovery exists, add the new proof surface the milestone actually needs instead of extending Mesher again. The app should stay narrow: one HTTP endpoint, one small env contract, Fly-friendly identity defaults, and membership truth derived from `Node.self()` plus `Node.list()` rather than from discovery candidates or global registry guesses.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Env-driven node identity/discovery config | Surface an explicit startup/config error and keep the app from silently claiming cluster mode with unusable identity. | N/A | Reject invalid node/host/port combinations instead of guessing at a fallback that would make peer gossip dishonest. |
| Runtime `Node.self()` / `Node.list()` surfaces | Return a truthful standalone payload (`self` only or empty cluster mode) rather than inventing peers. | If discovery has not converged yet, return the current live membership state and let the verifier wait explicitly. | Treat malformed identity strings as a proof-app bug and fail the e2e assertion instead of normalizing bad membership silently. |
| HTTP membership endpoint | Keep it read-only and deterministic; fail the e2e test with the full response body if JSON shape drifts. | Timeout fails the convergence test and preserves the node logs for inspection. | Treat missing `membership` / `peers` fields as contract failure, not partial success. |

## Load Profile

- **Shared resources**: live session-table reads, env-derived config, and one JSON response per membership check.
- **Per-operation cost**: one `Node.self()` call, one `Node.list()` call, and JSON encoding for a small payload.
- **10x breakpoint**: repeated polling allocates JSON strings first; the proof app itself should remain read-only and cheap.

## Negative Tests

- **Malformed inputs**: missing discovery seed, blank advertised host, invalid port strings, or mixed Fly env that yields no unique identity.
- **Error paths**: discovery not yet converged, runtime started in standalone mode, or endpoint JSON shape drifts from the verifier contract.
- **Boundary conditions**: `Node.list()` empty, one peer present, and dual-stack local bootstrap where `membership` must still include `self` even though `Node.list()` is peer-only.

## Steps

1. Create `cluster-proof/` with a small env parser and identity builder that composes a unique advertised node name from explicit env or Fly defaults (`FLY_MACHINE_ID`, `FLY_PRIVATE_IP`, `FLY_REGION`, `FLY_APP_NAME`).
2. Start the runtime once with the configured cookie, cluster port, and discovery seed, keeping cluster mode optional but explicit.
3. Add one read-only HTTP endpoint that returns `self`, `peers`, `membership`, and non-secret config context (discovery seed/provider, cluster port, HTTP port, mode).
4. Add `compiler/meshc/tests/e2e_m039_s01.rs` coverage that compiles and runs the proof app on two local nodes, uses `localhost` dual-stack discovery for the local proof, and asserts truthful membership convergence without any manual peer list.

## Must-Haves

- [ ] The proof app is a new narrow surface under `cluster-proof/`, not another Mesher retrofit.
- [ ] The endpoint derives membership truth from live runtime sessions and explicitly includes `self` so peer-only `Node.list()` cannot under-report the cluster.
- [ ] The env contract stays small and Fly-ready while never echoing the shared cookie.

## Verification

- `cargo run -q -p meshc -- build cluster-proof`
- `cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_converges_without_manual_peers -- --nocapture`

## Observability Impact

- Signals added/changed: proof-app startup logs that name the advertised node identity, discovery seed, and cluster/HTTP ports.
- How a future agent inspects this: hit the proof endpoint or rerun `cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_converges_without_manual_peers -- --nocapture`.
- Failure state exposed: whether startup/config was wrong or live membership never converged.

  - Estimate: 2.5h
  - Files: cluster-proof/main.mpl, cluster-proof/config.mpl, cluster-proof/cluster.mpl, compiler/meshc/tests/e2e_m039_s01.rs, website/docs/docs/distributed/index.md
  - Verify: `cargo run -q -p meshc -- build cluster-proof`
`cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_converges_without_manual_peers -- --nocapture`
- [x] **T03: Made the local cluster-proof verifier authoritative with per-node logs, a real node-loss shrinkage proof, and a fail-closed replay script.** — ---
estimated_steps: 4
estimated_files: 3
skills_used:
  - rust-best-practices
  - test
---
Close the slice with one replayable local proof surface instead of a bag of ad hoc commands. Extend the Rust e2e harness from the convergence task so it can kill one node, wait for the surviving endpoint to shrink membership truthfully, and leave per-node logs behind when anything stalls. Then wrap the named runtime and e2e checks in a repo-local verifier script that fail-closes on missing tests, port collisions, or missing artifacts.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Spawned proof-app child processes | Tear them down, keep stdout/stderr per node, and fail the test with the last observed membership payload. | Timeout fails the proof and preserves node logs instead of hanging until CI kills the process. | Treat crash-loop or partial startup logs as a verifier failure, not as a transient warning. |
| Local dual-stack DNS seed (`localhost`) | Fail the local proof clearly if both address families are not usable on this host, with the missing family called out in logs. | N/A | Reject a single-address-only result as insufficient for the two-node local proof. |
| Verifier wrapper script | Stop on the first failing phase, keep logs under `.tmp/m039-s01/`, and refuse to pass if a named test filter ran 0 tests. | Fail the current phase and leave the partial artifacts in place for inspection. | Treat malformed JSON/log artifacts as proof failure rather than silently skipping checks. |

## Load Profile

- **Shared resources**: local TCP ports, child-process stdout/stderr logs, and repeated HTTP polling against the proof endpoint.
- **Per-operation cost**: two proof-app processes, short polling loops, and one wrapper-script replay.
- **10x breakpoint**: port collisions or slow convergence timeouts fail before CPU becomes a problem, so the verifier must report which phase stalled.

## Negative Tests

- **Malformed inputs**: missing required env for one node, invalid port reuse, or missing output directory for logs.
- **Error paths**: one node exits before convergence, the surviving node never drops the lost peer, or the verifier wrapper sees `running 0 tests`.
- **Boundary conditions**: surviving node reports zero peers after loss but still includes itself in `membership`; loss detection must not depend on manual peer removal.

## Steps

1. Extend `compiler/meshc/tests/e2e_m039_s01.rs` with a second named proof that starts the dual-stack local cluster, kills one node, and waits for the survivor’s endpoint to report truthful membership shrinkage.
2. Capture per-node stdout/stderr from the Rust harness so timeouts and malformed membership responses include concrete logs instead of generic assertions.
3. Add `scripts/verify-m039-s01.sh` as the single local replay command that builds `cluster-proof`, runs the named `mesh-rt` and `meshc` tests in order, archives logs under `.tmp/m039-s01/`, and fails closed on 0-test filters.
4. Keep the wrapper local-only for this slice; S04 will own the canonical one-image/Fly replay and docs reconciliation.

## Must-Haves

- [ ] The local proof covers both discovery convergence and membership shrinkage after node loss with no manual peer repair.
- [ ] Failures leave per-node logs under `.tmp/m039-s01/` so future debugging starts from evidence, not from guesswork.
- [ ] `scripts/verify-m039-s01.sh` becomes the authoritative slice acceptance command from repo root.

## Verification

- `cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_membership_updates_after_node_loss -- --nocapture`
- `bash scripts/verify-m039-s01.sh`

## Observability Impact

- Signals added/changed: verifier phase logs plus per-node stdout/stderr capture for convergence and loss assertions.
- How a future agent inspects this: rerun `bash scripts/verify-m039-s01.sh` and inspect `.tmp/m039-s01/`.
- Failure state exposed: which phase stalled, what the last endpoint payload was, and what each node logged before failure.

  - Estimate: 1.5h
  - Files: compiler/meshc/tests/e2e_m039_s01.rs, scripts/verify-m039-s01.sh, cluster-proof/main.mpl
  - Verify: `cargo test -p meshc --test e2e_m039_s01 e2e_m039_s01_membership_updates_after_node_loss -- --nocapture`
`bash scripts/verify-m039-s01.sh`
