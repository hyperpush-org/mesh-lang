---
estimated_steps: 4
estimated_files: 4
skills_used:
  - test
---

# T02: Build the narrow cluster proof app and expose truthful membership from live sessions

**Slice:** S01 — General DNS Discovery & Membership Truth
**Milestone:** M039

## Description

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

## Inputs

- `compiler/mesh-rt/src/dist/discovery.rs` — runtime discovery loop and candidate filtering from T01.
- `.gsd/milestones/M039/M039-CONTEXT.md` — milestone-level proof-app and operator constraints.
- `tests/e2e/stdlib_http_server_runtime.mpl` — minimal HTTP runtime reference.
- `mesher/main.mpl` — existing env-driven startup pattern to keep the new app small.
- `website/docs/docs/distributed/index.md` — current public runtime contract the proof app should not overclaim beyond.

## Expected Output

- `cluster-proof/main.mpl` — proof-app entrypoint and HTTP server wiring.
- `cluster-proof/config.mpl` — env parsing and Fly/local advertised-identity construction.
- `cluster-proof/cluster.mpl` — membership payload shaping from `Node.self()` and `Node.list()`.
- `compiler/meshc/tests/e2e_m039_s01.rs` — named convergence proof for the new app.
