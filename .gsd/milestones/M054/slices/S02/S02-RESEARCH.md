# M054 S02 Research — Clustered HTTP request correlation

## Requirement focus

- **R123 (primary):** replace the current before/after continuity-diff seam with a direct, runtime-owned request-correlation surface for clustered HTTP.
- **R124 (guard):** keep this as an operator/debug surface; do **not** turn it into frontend-aware routing or a client-topology contract.
- **R060 (supporting):** keep the correlation surface platform-agnostic and runtime-owned rather than Fly-specific.

## Skills Discovered

- Loaded **`debug-like-expert`** and followed its core rule: **verify, don’t assume**. I traced the live request path, ran the current clustered-route e2e, and inspected retained raw HTTP responses before recommending a seam.
- No new skill installs were needed. This slice is repo-owned Rust/runtime/test-harness work, not an unfamiliar library integration.

## Executive summary

The current clustered HTTP path already creates the exact continuity record S02 needs, and the existing CLI already supports direct lookup by `request_key`. The missing piece is that clustered HTTP responses do **not** return that `request_key` anywhere, so both the lower-level clustered-route proof and the new S01 serious-starter public-ingress proof still have to discover the route record by diffing continuity lists before and after a request.

The narrowest real fix is a **runtime-added clustered-route response header** carrying the continuity request key. The right interception point is `compiler/mesh-rt/src/http/server.rs::clustered_route_response_from_request(...)`, because that function already has:

- the generated `request_key`
- the declared runtime name
- the final route response (local or remote)
- the single boundary all clustered HTTP routes pass through, including middleware-wrapped routes

That lets S02 land as a runtime-owned observability slice instead of a starter-only workaround or a new operator API.

## Verified current state

### 1. The runtime already has the direct record lookup surface

`compiler/meshc/src/cluster.rs` already exposes:

- `meshc cluster continuity <node> --json` → list form
- `meshc cluster continuity <node> <request_key> --json` → direct single-record lookup

`continuity_record_json(...)` already includes the fields S02 cares about:

- `request_key`
- `attempt_id`
- `ingress_node`
- `owner_node`
- `replica_node`
- `execution_node`
- `declared_handler_runtime_name`
- `phase` / `result` / `error`

No new CLI shape is required.

### 2. The clustered HTTP runtime already generates the correlation key, but drops it before the response leaves the server

`compiler/mesh-rt/src/http/server.rs`

- `build_clustered_http_route_identity(...)` creates `(request_key, payload_hash)` using `http-route::<runtime>::<seq>`.
- `clustered_route_response_from_request(...)` calls that function, then calls `crate::dist::node::execute_clustered_http_route(...)`, then decodes the final response payload and returns it.
- It does **not** attach the generated `request_key` to the HTTP response.

That means the key exists at exactly the right layer already; it just never escapes the runtime.

### 3. The serious-starter S01 proof still depends on continuity diffing

`compiler/meshc/tests/e2e_m054_s01.rs` currently does this for the selected public `GET /todos`:

1. capture `continuity-before-selected-route` on primary + standby
2. issue the public request
3. call `wait_for_new_route_request_key(...)` on both nodes
4. compare the two discovered request keys
5. then do direct continuity-record lookups using that key

The retained bundle shape in S01 is built around that diff seam:

- `selected-route-key-primary.log/json`
- `selected-route-key-standby.log/json`
- `selected-route.diff.json`

That is the current gap S02 is supposed to close.

### 4. The lower-level clustered-route regression rail also depends on diffing

`compiler/meshc/tests/e2e_m047_s07.rs` still uses:

- `wait_for_new_request_key_for_runtime_name(...)`
- `route_free::new_request_keys_for_runtime_name(...)`

So the issue is not just the serious starter. The lower runtime proof still has the same operator-unfriendly seam.

### 5. Real retained HTTP responses currently have no correlation header

I verified this in live retained artifacts.

Lower-level clustered-route proof:

- `.tmp/m047-s07/clustered-http-routes-two-node-1775486417143040000/route-success-first.http`

Current raw response:

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 86
Connection: close

{"handler":"Api.Todos.handle_list_todos","method":"GET","path":"/todos","status":"ok"}
```

Serious-starter one-public-URL proof:

- `.tmp/m054-s01/proof-bundles/retained-one-public-url-proof-1775456513099725000/retained-m054-s01-artifacts/staged-postgres-public-ingress-truth-1775456495651189000/public-selected-list.http`

Current raw response:

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Length: 2
Connection: close

[]
```

So S02 would be adding a real new product surface, not renaming an existing one.

### 6. The current clustered-route rail is green as-is

I replayed the current route-level proof:

- `cargo test -p meshc --test e2e_m047_s07 m047_s07_clustered_http_routes_two_node_end_to_end -- --nocapture`

Result: **passed**.

That confirms the current stable state is "diff first, direct lookup second," not a stale code-reading assumption.

## Implementation landscape

### Runtime seam: `compiler/mesh-rt/src/http/server.rs`

This is the key file.

Relevant pieces:

- `build_clustered_http_route_identity(...)` — generates the key
- `clustered_route_response_from_request(...)` — the best place to attach a correlation header
- `process_request(...)` — extracts response headers and writes them to the client
- `chain_next(...)` — middleware-wrapped clustered routes still funnel through `clustered_route_response_from_request(...)`
- `write_response(...)` — already emits arbitrary custom headers
- `mesh_http_response_new(...)` / `mesh_http_response_with_headers(...)` plus `pairs_to_mesh_map(...)` — existing machinery is enough to preserve/augment headers

Planner note: this is a **single-boundary** change. Do not push correlation into starter code or route handlers.

### Runtime continuity seam: `compiler/mesh-rt/src/dist/node.rs` + `compiler/mesh-rt/src/dist/continuity.rs`

These files already do the important work.

- `execute_clustered_http_route(...)` creates the continuity record and dispatches local/remote execution.
- `prepare_declared_handler_submission(...)` already records `ingress_node`, `owner_node`, `replica_node`, `declared_handler_runtime_name`, `replication_count`, and continuity authority metadata.
- `ContinuityRecord` already stores everything S02 needs.

Planner note: S02 does **not** need a new continuity record shape unless you want extra HTTP-specific metadata. For the slice goal, `request_key` exposure is enough.

### Serious-starter proof seam: `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs`

This support file already has the direct lookup helpers S02 wants to reuse:

- `wait_for_continuity_record_completed_pair(...)`
- `wait_for_request_diagnostics_pair(...)`

The only missing input is the request key from the HTTP response.

Planner note: once the response carries a correlation key, the new serious-starter proof can skip continuity-list diffing entirely and jump straight into these helpers.

### Public-ingress harness seam: `compiler/meshc/tests/support/m054_public_ingress.rs`

This helper already preserves raw proxied response bytes in `PublicIngressRequestRecord.response_raw` and `public-ingress.requests.json`.

Important constraint:

- its internal `ParsedHttpMessage` currently keeps the first line and raw bytes, but **drops parsed response headers**.

Planner note: do **not** widen this harness unless you actually need structured header retention for all requests. For S02, a smaller move is possible: parse the selected response header from the existing raw HTTP response and write one dedicated artifact.

### Response parsing seam: `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`

`HttpResponse` currently stores:

- `status_code`
- `body`
- `raw`

There is no helper for structured response-header lookup.

Planner note: if S02 needs reusable header assertions for the serious starter, this is the natural small helper location. Avoid inventing a brand-new cross-suite HTTP parsing abstraction unless two or more current callers really benefit.

### Lower-level regression seam: `compiler/meshc/tests/e2e_m047_s07.rs`

This file is the best low-level proof surface for the new runtime header because it already exercises:

- successful clustered GET route execution
- repeated runtime-name route requests
- unsupported replication-count rejection

Planner note: keep the current diff-helper unit tests. They are still useful for historical rails and fail-closed behavior. S02 should add a direct-correlation proof, not delete the old helper coverage.

### Starter guidance seam: `compiler/mesh-pkg/src/scaffold.rs` → `examples/todo-postgres/README.md`

The current generated README already teaches:

- `meshc cluster continuity <node> <request-key> --json`
- "Use the continuity list form first to discover runtime-owned startup records."

If S02 lands a response header, the README can teach a tighter clustered-route workflow without widening the public claim:

- get the request key from the clustered HTTP response header
- use `meshc cluster continuity <node> <request-key> --json`
- keep the continuity-list guidance for startup records only

Planner note: because the committed example is generator-truthful, update `compiler/mesh-pkg/src/scaffold.rs` first, then re-materialize `examples/todo-postgres/README.md`. Do **not** hand-edit the committed README in isolation.

## Recommendation

### Recommended product seam

Ship a runtime-added correlation header on clustered HTTP responses.

**Minimal recommended header:**

- `X-Mesh-Continuity-Request-Key: <request_key>`

Optional but deferrable:

- `X-Mesh-Continuity-Attempt-Id: <attempt_id>`

Why this is the right seam:

1. **Runtime-owned** — satisfies the slice goal without starter-owned hacks
2. **Path-local** — one change in `clustered_route_response_from_request(...)`
3. **Platform-agnostic** — works through direct-node calls and the S01 one-public-URL ingress harness
4. **Reuses current CLI** — operators can immediately call `meshc cluster continuity <node> <request_key> --json`
5. **Does not change routing semantics** — correlation only, no placement algorithm churn

### Error-path recommendation

If possible without widening scope, attach the same request-key header to clustered-route 503 responses **when a request key was generated**.

That makes rejected/unsupported clustered routes directly traceable too, and it is cheap because the request key exists before dispatch.

### What not to do

- Do **not** introduce a new `meshc cluster` subcommand for this slice.
- Do **not** add package-owned admin or status routes.
- Do **not** treat this header as a frontend-aware routing contract.
- Do **not** try to fix clustered-route request-key generation globally in S02.

## Natural task split

### Task 1 — Runtime correlation header

**Files:**

- `compiler/mesh-rt/src/http/server.rs`
- likely low-level tests in the same file
- likely `compiler/meshc/tests/e2e_m047_s07.rs`

**Goal:**

- add the runtime-owned response header for clustered HTTP routes
- preserve existing app headers
- verify direct continuity lookup works from that header without pre/post list diffing

**Build first because:** everything else depends on the header existing.

### Task 2 — Serious starter direct-correlation proof

**Files:**

- new `compiler/meshc/tests/e2e_m054_s02.rs`
- maybe a tiny response-header parser helper in `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`
- reuse `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs`

**Goal:**

- replay the one-public-URL serious-starter path from S01
- extract the request key from the public response header
- go directly to `wait_for_continuity_record_completed_pair(...)`
- retain direct-lookup artifacts without the before/after continuity diff step

### Task 3 — Assembled verifier + bounded starter guidance

**Files:**

- new `scripts/verify-m054-s02.sh`
- likely `scripts/tests/verify-m054-s02-contract.test.mjs`
- `compiler/mesh-pkg/src/scaffold.rs`
- `examples/todo-postgres/README.md`

**Goal:**

- delegate S01 instead of mutating its retained contract
- add a new retained S02 proof bundle with direct-correlation artifacts
- teach the clustered-route request-key lookup flow in the generated starter README
- keep the wording bounded to operator inspection, not client routing

## Risks and constraints

### 1. Do not rewrite S01 in place

`compiler/meshc/tests/e2e_m054_s01.rs`, `scripts/verify-m054-s01.sh`, and `scripts/tests/verify-m054-s01-contract.test.mjs` already lock a green retained bundle shape around `selected-route-key-*` and `selected-route.diff.json`.

Planner recommendation: **add S02 as a new rail** that delegates S01. Do not repurpose the S01 rail unless there is a compelling cleanup reason.

### 2. Keep the route-key collision issue out of scope

Known repo knowledge still applies: clustered HTTP request keys are per-process monotonic (`http-route::<runtime>::<seq>`), and repeated same-runtime requests through different ingress nodes can collide.

Planner recommendation:

- prove direct correlation on the **single selected public request** or a stable single-ingress request path
- do **not** expand S02 into cluster-global route-ID work

### 3. Do not overclaim browser-programmatic visibility

A response header is a good operator/debug surface, but it is **not automatically a browser-JS contract** in cross-origin setups without explicit header exposure.

Planner recommendation: document it as an **operator-facing correlation signal** (curl, devtools, logs, local proof harness), not as a frontend SDK routing API. That preserves R124.

### 4. Preserve existing custom route headers

Clustered handlers can already return custom headers. The correlation header logic must preserve them.

Planner recommendation: add an explicit regression asserting both the existing handler header and the new correlation header survive together.

### 5. Avoid exact numeric suffix assertions

`CLUSTERED_HTTP_ROUTE_REQUEST_SEQUENCE` is a process-global atomic and the current `server.rs` test reset helper does **not** reset it.

Planner recommendation: assert on:

- header presence
- `http-route::<runtime>::` prefix
- equality between response header value and direct continuity lookup value

Do not assert `::1` / `::2` exact suffixes in the new unit tests.

## Verification plan

### Current-state replay I ran

- `cargo test -p meshc --test e2e_m047_s07 m047_s07_clustered_http_routes_two_node_end_to_end -- --nocapture`

### Recommended post-implementation verification

Low-level runtime + clustered-route seam:

- `cargo test -p mesh-rt m047_s07 -- --nocapture`
- `cargo test -p meshc --test e2e_m047_s07 -- --nocapture`

Serious-starter direct-correlation seam:

- `DATABASE_URL=<redacted> cargo test -p meshc --test e2e_m054_s02 -- --nocapture`

Assembled slice rail:

- `node --test scripts/tests/verify-m054-s02-contract.test.mjs`
- `DATABASE_URL=<redacted> bash scripts/verify-m054-s02.sh`

### Expected retained S02 artifacts

At minimum, the new S02 proof bundle should retain:

- one raw public clustered HTTP response showing the correlation header
- one structured correlation artifact containing the extracted `request_key`
- direct primary/standby continuity single-record lookup JSON for that key
- direct diagnostics entries for that key
- copied S01 verifier state or proof-bundle pointer if the S02 wrapper delegates S01

## Bottom line for planning

This slice should be planned as **runtime correlation first, proof second, bounded README/verifier third**.

The runtime already knows the request key and the CLI already knows how to look it up. The missing product surface is just the handoff between them. A clustered-route response header is the smallest real follow-through that closes the S01 observability gap without turning this into a routing or platform architecture slice.
