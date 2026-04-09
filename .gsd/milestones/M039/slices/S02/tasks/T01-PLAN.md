---
estimated_steps: 24
estimated_files: 4
skills_used: []
---

# T01: Started the cluster-proof work-routing implementation, but stopped at a real runtime mismatch: Mesh HTTP handlers are not actor contexts and the distributed transport still only carries raw scalar bytes.

Add the actual routing proof surface inside `cluster-proof` instead of inventing a service layer. The endpoint should stay read-only and typed: it captures ingress from the handler context, chooses a deterministic peer-preferred target from live membership, spawns a one-shot worker on the target node (or locally when no peer exists), waits for a reply with a bounded timeout, and returns a JSON body that makes internal routing obvious.

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

## Inputs

- ``cluster-proof/main.mpl` — existing HTTP routing and startup logs.`
- ``cluster-proof/cluster.mpl` — current membership payload and live session truth.`
- ``cluster-proof/config.mpl` — env contract that the new route should not widen.`
- ``tools/skill/mesh/skills/http/SKILL.md` — repo-local Mesh HTTP handler contract.`
- ``tools/skill/mesh/skills/actors/SKILL.md` — repo-local request/reply actor guidance.`

## Expected Output

- ``cluster-proof/work.mpl` — typed routing protocol, target selection, and execution worker helpers.`
- ``cluster-proof/main.mpl` — new proof route wired next to the unchanged `/membership` endpoint.`
- ``cluster-proof/cluster.mpl` — any shared membership snapshot helper extraction needed by the work route.`
- ``cluster-proof/tests/work.test.mpl` — pure Mesh tests for target selection and standalone fallback truth.`

## Verification

`cargo run -q -p meshc -- test cluster-proof/tests`
`cargo run -q -p meshc -- build cluster-proof`

## Observability Impact

- Signals added/changed: execution-node proof logs keyed by `request_id`, plus explicit handler timeout/error logs.
- How a future agent inspects this: hit the work endpoint and inspect `cluster-proof` stdout for matching request ids.
- Failure state exposed: whether the route chose the wrong target, timed out waiting for a reply, or returned a malformed payload.
