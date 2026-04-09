# S07: Clustered HTTP route wrapper completion — UAT

**Milestone:** M047
**Written:** 2026-04-02T00:52:42.946Z

# S07: Clustered HTTP route wrapper completion — UAT

**Milestone:** M047
**Written:** 2026-04-02

## UAT Type

- UAT mode: mixed (compiler/unit rails + live two-node runtime rail + retained guardrail replay)
- Why this mode is sufficient: S07 only counts as done if the wrapper is real at every seam — source diagnostics, lowering/registration, runtime continuity, and live HTTP behavior — while the older M032 route-limit controls stay intact.

## Preconditions

- Run from the repo root.
- Rust workspace dependencies are installed.
- Local loopback ports are available for a two-node cluster and two HTTP listeners.
- Do not run `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` in parallel with another local cluster harness; it owns `.tmp/m047-s07/clustered-http-routes-two-node/`.

## Smoke Test

1. Run `cargo test -p mesh-typeck m047_s07 -- --nocapture`.
2. Run `cargo test -p mesh-lsp m047_s07 -- --nocapture`.
3. Run `cargo test -p mesh-codegen m047_s07 -- --nocapture`.
4. Run `cargo test -p mesh-rt m047_s07 -- --nocapture`.
5. Run `cargo test -p meshc --test e2e_m047_s07 -- --nocapture`.
6. Run `cargo test -p meshc --test e2e_stdlib e2e_m032_route_ -- --nocapture`.
7. **Expected:** all six commands pass; the last two retain `.tmp/m047-s07/clustered-http-routes-two-node/` evidence and leave the historical M032 bare-handler / closure-route controls green.

## Test Cases

### 1. Wrapper typing, metadata, and diagnostics are source-local

1. Run `cargo test -p mesh-typeck m047_s07 -- --nocapture`.
2. **Expected:** tests cover direct `HTTP.on_get(router, "/x", HTTP.clustered(handle))` and pipe-form `router |> HTTP.on_get("/x", HTTP.clustered(handle))`.
3. **Expected:** imported bare handlers preserve their defining-module runtime names; default count records as `2`, explicit count records as the literal value.
4. **Expected:** misuse cases fail with wrapper-specific diagnostics, not generic `UnboundVariable` / `NoSuchField` noise:
   - non-route-position wrapper use
   - closure/anonymous handler arguments
   - private handlers
   - conflicting counts for one runtime name
   - imported handler origin drift

### 2. LSP points at the wrapper call, not unrelated route tokens

1. Run `cargo test -p mesh-lsp m047_s07 -- --nocapture`.
2. **Expected:** a valid project with imported bare handlers produces no diagnostics.
3. **Expected:** the invalid non-route-position case reports a diagnostic whose range starts at `HTTP.clustered` and spans the wrapper expression.

### 3. Lowering emits deterministic route shims and reuses declared-handler registration

1. Run `cargo test -p mesh-codegen m047_s07 -- --nocapture`.
2. **Expected:** direct and pipe-form wrappers for the same handler dedupe to one bare shim (for example `__declared_route_app_router_handle_local`).
3. **Expected:** imported handlers preserve defining-module runtime identity in the shim name (for example `__declared_route_api_todos_handle_list_todos`).
4. **Expected:** route wrappers lower away completely; no runtime `HTTP.clustered(...)` calls survive in MIR.
5. **Expected:** route declared-handler plan entries use the shared runtime-registration name and count seam, while `prepare_startup_work_registrations(...)` filters them out of startup work.

### 4. Runtime transport preserves request/response truth and rejects unsupported fanout durably

1. Run `cargo test -p mesh-rt m047_s07 -- --nocapture`.
2. **Expected:** request transport roundtrip preserves method, path, body, headers, query params, and path params.
3. **Expected:** response transport roundtrip preserves status, body, and headers.
4. **Expected:** malformed request/response payloads fail closed.
5. **Expected:** unsupported replication count returns HTTP 503, records `phase=rejected`, `result=rejected`, `replication_count=3`, and `error=unsupported_replication_count:3`, and does **not** invoke the real handler.
6. **Expected:** invoking the route handler from encoded payload executes the real route boundary once and returns the real `Response`.

### 5. Live two-node clustered HTTP success path stays truthful

1. Run `cargo test -p meshc --test e2e_m047_s07 -- --nocapture`.
2. Open `.tmp/m047-s07/clustered-http-routes-two-node/route-success-first.json` and `.tmp/m047-s07/clustered-http-routes-two-node/route-success-second.json`.
3. **Expected:** both requests return HTTP 200 with JSON:
   - `status = "ok"`
   - `handler = "Api.Todos.handle_list_todos"`
   - `method = "GET"`
   - `path = "/todos"`
4. Open `continuity-first-completed-primary.json`, `continuity-first-completed-standby.json`, `continuity-second-completed-primary.json`, and `continuity-second-completed-standby.json`.
5. **Expected:** each record reports:
   - `declared_handler_runtime_name = Api.Todos.handle_list_todos`
   - `replication_count = 2`
   - `phase = completed`
   - `result = succeeded`
   - `replica_status = mirrored`
   - `execution_node == owner_node`
   - `owner_node != replica_node`
6. **Expected:** the second success request produces a new `request_key`, and continuity diffing is based on `request_key` + runtime name rather than list order.

### 6. Live unsupported-count route fails honestly

1. In the same retained bundle, open `route-unsupported-count.json`, `continuity-rejected-primary.json`, and `continuity-rejected-standby.json`.
2. **Expected:** `/todos/retry` returns HTTP 503 with `error = "unsupported_replication_count:3"`.
3. **Expected:** both rejected continuity records report:
   - `declared_handler_runtime_name = Api.Todos.handle_retry_todos`
   - `replication_count = 3`
   - `phase = rejected`
   - `result = rejected`
   - `error = unsupported_replication_count:3`
4. **Expected:** there is no silent local 200 fallback for the unsupported route.

### 7. Historical M032 route-limit controls stay green

1. Run `cargo test -p meshc --test e2e_stdlib e2e_m032_route_ -- --nocapture`.
2. **Expected:** the bare-handler control still passes.
3. **Expected:** the closure-route runtime-failure control still behaves as the retained negative guard, proving `HTTP.clustered(...)` did not widen generic route closure support.

## Edge Cases

### Imported bare handler identity drifts to the local module or shim name

1. Inspect the successful continuity records and route-success JSON.
2. **Expected:** the runtime identity remains `Api.Todos.handle_list_todos`, not `App.Router.handle_list_todos` and not `__declared_route_api_todos_handle_list_todos`.

### Continuity inspection depends on list order

1. Inspect the retained before/after continuity JSON pairs.
2. **Expected:** new records are discovered by new `request_key` values for the target runtime name; reordered unrelated records must not break the proof.

### Missing request keys or malformed continuity JSON

1. Re-run the targeted `m047_s07_continuity_diff_helpers_fail_closed_on_missing_request_keys` case if debugging helper regressions.
2. **Expected:** the helper panics/fails closed instead of treating malformed continuity output as a passing empty diff.

### Repeated same-runtime traffic uses multiple ingress nodes

1. If you customize the rail, keep repeated success requests on one ingress node unless route identities become cluster-global.
2. **Expected:** today's truthful behavior is node-local request-key generation; cross-ingress repetition can collide and reject as duplicate.

## Notes for Tester

- S07 ships the compiler/runtime/e2e route-wrapper seam, but it does **not** migrate the Todo scaffold or public docs yet; that adoption belongs to S08.
- Start debugging in `.tmp/m047-s07/clustered-http-routes-two-node/` before changing public wording or scaffold output. The retained bundle already contains the generated app, build logs, cluster status snapshots, HTTP responses, continuity snapshots, and both node logs.
