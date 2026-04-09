# S07: Clustered HTTP route wrapper completion

**Goal:** Ship the real clustered HTTP route-wrapper seam so `HTTP.clustered(handler)` / `HTTP.clustered(N, handler)` lower through the shared declared-handler runtime-name and replication-count model, execute the route handler itself as the clustered boundary, and prove the contract with live two-node HTTP plus continuity rails while leaving scaffold/docs adoption to S08.
**Demo:** After this: After this: router chains can use `HTTP.clustered(handle)` and `HTTP.clustered(N, handle)`, the compiler/runtime lower selected routes onto the shared clustered declaration + replication-count seam, and live HTTP requests plus continuity inspection prove the route handler is the clustered boundary.

## Tasks
- [x] **T01: Added compiler-known HTTP.clustered wrapper typing, metadata, and source-local typecheck/LSP diagnostics for clustered HTTP routes.** — Make `HTTP.clustered` a compiler-known surface instead of an undefined stdlib lookup. This task should accept only bare route-handler references, validate both direct and pipe-form registrations, preserve imported handler origin for runtime names, and turn misuse into source-local diagnostics instead of generic `undefined variable` fallout.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| typecheck stdlib/builtin HTTP surface | fail with wrapper-specific diagnostics at the call range; do not fall back to generic `NoSuchField` / `UnboundVariable` noise | N/A | reject mismatched wrapper shapes instead of inferring a fake helper type |
| import-context handler resolution | preserve the defining module for bare imported handlers or fail the focused tests; do not collapse `Api.Todos.handle_list_todos` to the local module name | N/A | treat missing or ambiguous origin metadata as a contract failure, not as a guessed runtime name |
| LSP diagnostic projection | surface the same wrapper misuse spans through editor diagnostics or fail the focused tests; do not leave stale clustering diagnostics pointing at unrelated tokens | N/A | malformed diagnostic ranges are test failures, not acceptable best-effort output |

## Load Profile

- **Shared resources**: import-context maps, route-wrapper metadata tables, and LSP diagnostic rendering.
- **Per-operation cost**: one parse/typecheck pass plus focused unit/integration assertions.
- **10x breakpoint**: conflicting wrapper metadata and imported-name origin drift fail long before throughput matters.

## Negative Tests

- **Malformed inputs**: closure/anonymous-fn arguments, call expressions, non-route-position use, and private handlers.
- **Error paths**: conflicting duplicate counts for the same handler, wrapper use under direct vs piped route registration, and imported bare handlers with missing origin metadata.
- **Boundary conditions**: default vs explicit counts, module-qualified handlers, and imported bare handlers all produce stable runtime identities.

## Steps

1. Add compiler-known `HTTP.clustered` typing and a sibling metadata map on `InferCtx` / `TypeckResult` that records wrapper callsite info plus defining-module origin for bare imported handlers.
2. Validate both direct `HTTP.on_get(router, "/x", HTTP.clustered(handle))` and piped `router |> HTTP.on_get("/x", HTTP.clustered(handle))` forms, accepting only bare handler refs or module-qualified refs and rejecting non-route-position use.
3. Thread new error variants and renderers through `compiler/mesh-typeck/src/error.rs`, `compiler/mesh-typeck/src/diagnostics.rs`, and `compiler/mesh-lsp/src/analysis.rs` so misuse localizes to the wrapper call instead of unrelated route tokens.
4. Add focused `mesh-typeck` / `mesh-lsp` tests covering imported bare-handler origin, default vs explicit counts, direct and piped forms, and the closed-failure cases.

## Must-Haves

- [ ] `HTTP.clustered(handler)` and `HTTP.clustered(N, handler)` type-check only in route-handler position.
- [ ] Imported bare handlers keep their defining-module runtime identity for later lowering.
- [ ] Misuse produces named, source-local diagnostics rather than generic undefined-symbol fallout.
  - Estimate: 4h
  - Files: compiler/mesh-typeck/src/infer.rs, compiler/mesh-typeck/src/builtins.rs, compiler/mesh-typeck/src/unify.rs, compiler/mesh-typeck/src/lib.rs, compiler/mesh-typeck/src/error.rs, compiler/mesh-typeck/src/diagnostics.rs, compiler/mesh-lsp/src/analysis.rs, compiler/mesh-typeck/tests/http_clustered_routes.rs
  - Verify: - `cargo test -p mesh-typeck m047_s07 -- --nocapture`
- `cargo test -p mesh-lsp m047_s07 -- --nocapture`
- [x] **T02: Lowered clustered HTTP route wrappers into deterministic route shims and shared declared-handler registration.** — Once typecheck captures truthful wrapper metadata, lower it through the same runtime-name and replication-count registry used by ordinary clustered functions. Generate bare route shims that keep the public handler signature `fn(Request) -> Response`, preserve the real handler runtime name, and keep startup-work registration unchanged.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| MIR lowering of route-wrapper metadata | fail codegen with a named lowering error; do not silently emit ordinary local routes when wrapper metadata is missing | N/A | reject inconsistent shim plans or conflicting counts instead of guessing |
| declared-handler registration emission | keep route handlers on the shared runtime-name and replication-count registry or fail before LLVM emission succeeds | N/A | malformed runtime names or missing lowered symbols are test failures, not fallback-to-local behavior |
| startup-work filtering | keep clustered routes out of startup registration or fail a focused assertion; do not auto-start HTTP handlers | N/A | a route handler appearing in startup registrations is a contract failure |

## Load Profile

- **Shared resources**: typecheck metadata maps, merged MIR, declared-handler planning, and LLVM registration markers.
- **Per-operation cost**: one lowering/codegen pass plus focused unit assertions.
- **10x breakpoint**: duplicate shim generation and symbol drift fail before throughput matters.

## Negative Tests

- **Malformed inputs**: missing lowered symbols, conflicting duplicate counts for the same handler, and wrapper metadata that does not resolve to a concrete route call.
- **Error paths**: generated shims without declared-handler registration, route handlers leaking into startup registration, or runtime names collapsing to shim-only identifiers.
- **Boundary conditions**: default and explicit counts both reach the declared-handler registry, identical repeated wrappers dedupe cleanly, and direct/piped route forms lower the same way.

## Steps

1. Extend `PreparedBuild` and the route-lowering seam so wrapper metadata from typecheck reaches MIR lowering without teaching `mesh-pkg::collect_source_cluster_declarations(...)` to parse route wrappers.
2. Teach MIR lowering and/or HTTP route call lowering to replace `HTTP.clustered(...)` with generated bare route shims that preserve the actual handler runtime name and fail closed on conflicting count reuse.
3. Add a route-capable declared-handler kind and registration path so explicit/default counts reach `mesh_register_declared_handler`, while `prepare_startup_work_registrations` continues to filter startup-only work.
4. Add focused `mesh-codegen` tests for shim generation, runtime-name/count markers, conflict handling, missing lowered symbols, and route handlers excluded from startup registration.

## Must-Haves

- [ ] Clustered routes reuse the ordinary declared-handler runtime-name and replication-count seam.
- [ ] Generated route shims keep the public handler signature while preserving the real handler identity.
- [ ] Clustered HTTP handlers are never added to startup-work registration.
  - Estimate: 4h
  - Files: compiler/meshc/src/main.rs, compiler/mesh-codegen/src/mir/lower.rs, compiler/mesh-codegen/src/declared.rs, compiler/mesh-codegen/src/codegen/mod.rs, compiler/mesh-codegen/src/codegen/expr.rs, compiler/mesh-codegen/src/codegen/intrinsics.rs
  - Verify: - `cargo test -p mesh-codegen m047_s07 -- --nocapture`
- [x] **T03: Added real clustered HTTP route dispatch and truthful continuity/operator evidence for route handlers.** — The HTTP runtime already knows how to build `MeshHttpRequest` and crack `MeshHttpResponse`; the missing seam is clustered execution. This task should bridge that boundary into continuity and declared-handler execution without widening generic closure routes or generic remote-spawn arg tags.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| clustered route dispatch from HTTP server | fail the request with an explicit 5xx/503 path plus continuity evidence; do not silently fall back to local direct invocation | bound request waits and fail the focused runtime/e2e rails with retained logs | reject malformed request/response encode-decode as a transport failure, not as an empty 200 |
| declared-handler continuity integration | keep `declared_handler_runtime_name`, `replication_count`, and rejection reasons truthful or fail the runtime tests | preserve current bounded continuity waits; do not add unbounded polling | malformed continuity records are a contract failure, not acceptable best effort |
| operator/inspection surfaces | route records must remain queryable even when multiple requests share one runtime name; do not depend on list order | N/A | malformed list ordering or missing request keys should fail the focused helper assertions |

## Load Profile

- **Shared resources**: HTTP server actors, continuity registry state, cluster membership, and route request/response serialization.
- **Per-operation cost**: one clustered HTTP request plus continuity submit/complete bookkeeping; unsupported higher fanout should reject durably before extra work starts.
- **10x breakpoint**: request-key generation, serialization correctness, and replica-availability checks fail before throughput matters.

## Negative Tests

- **Malformed inputs**: invalid or empty request-key/payload-hash generation, malformed encoded request/response payloads, and missing route metadata on dispatch.
- **Error paths**: unsupported explicit counts produce durable rejection plus HTTP failure, and clustered dispatch never degrades into a silent local-success path.
- **Boundary conditions**: default-count route success, repeated requests against one runtime name, and request/response roundtrip fidelity all stay truthful together.

## Steps

1. Extend route entries and HTTP server dispatch so a route can carry clustered runtime metadata and submit/complete a declared handler around the actual route handler invocation instead of always calling `call_handler` directly.
2. Implement request-key and payload-hash generation plus the route request/response transport that serializes `MeshHttpRequest`, runs the real handler as the clustered boundary, and returns `MeshHttpResponse` without widening generic `spawn_declared_work` arg tags.
3. Surface route execution outcome through continuity and operator diagnostics with request key, handler runtime name, count, phase/result, and explicit `unsupported_replication_count:3` rejection while keeping request bodies out of continuity.
4. Add focused `mesh-rt` tests for request/response roundtrip, successful default-count route completion, rejected explicit-count route flow, and repeated runtime-name inspection behavior.

## Must-Haves

- [ ] The HTTP route handler, not a downstream helper, is the clustered boundary.
- [ ] Runtime continuity truth for clustered routes reuses the declared-handler registry and exposes the same runtime-name/count fields as ordinary clustered work.
- [ ] Unsupported explicit fanout returns an HTTP failure contract with a durable continuity rejection reason.
  - Estimate: 5h
  - Files: compiler/mesh-rt/src/http/router.rs, compiler/mesh-rt/src/http/server.rs, compiler/mesh-rt/src/dist/node.rs, compiler/mesh-rt/src/dist/continuity.rs, compiler/mesh-rt/src/dist/operator.rs
  - Verify: - `cargo test -p mesh-rt m047_s07 -- --nocapture`
- [x] **T04: Added a two-node clustered HTTP route e2e that proves live success/rejection continuity and preserves the M032 route guardrails.** — Close the slice with a dedicated two-node HTTP proof rail instead of adopting the wrapper in scaffold/docs yet. The e2e should build a temp multi-module package, hit both success and unsupported-count routes, inspect continuity by before/after diff rather than list order, and keep the bare-function and closure-function M032 controls green.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| temp-project build and two-node bootstrap | fail with retained build/runtime artifacts; do not reuse scaffold/docs rails or manual repros as substitutes | bound process readiness and HTTP waits, then fail the named e2e with retained logs | malformed runtime bootstrap output is failure evidence, not acceptable best effort |
| HTTP request plus continuity inspection harness | capture before/after continuity snapshots and fail closed on missing request keys or runtime-name drift | keep bounded poll loops for HTTP readiness and continuity completion | malformed HTTP or CLI JSON should fail the rail rather than being treated as empty success |
| retained M032 route controls | rerun the existing bare-handler and closure-handler controls unchanged; do not widen generic route closure support while landing `HTTP.clustered(...)` | use the existing e2e timeouts | any unexpected closure success is a regression, not a flaky pass |

## Load Profile

- **Shared resources**: dual-stack cluster ports, temp project directories, HTTP ports, continuity query artifacts, and retained `.tmp/m047-s07` bundles.
- **Per-operation cost**: one temp-project build, two runtime processes, a small number of HTTP requests, and continuity/diagnostic queries.
- **10x breakpoint**: port collisions, readiness waits, and continuity diff heuristics fail before throughput matters.

## Negative Tests

- **Malformed inputs**: missing or unstable request keys, malformed continuity list JSON, and imported bare handlers that lose their defining-module runtime name.
- **Error paths**: the explicit-count route must surface the chosen HTTP failure contract plus durable rejection, and the success route must fail if continuity never reaches `completed/succeeded`.
- **Boundary conditions**: repeated requests against the same runtime name, default-count success on two nodes, and the unchanged M032 closure failure control all stay truthful together.

## Steps

1. Add `compiler/meshc/tests/e2e_m047_s07.rs` that builds a temp app with imported bare handlers and both `HTTP.clustered(handler)` / `HTTP.clustered(3, handler)` route forms, then boots a two-node cluster and sends live HTTP requests.
2. Reuse or extend `compiler/meshc/tests/support/m046_route_free.rs` helpers so the harness records before/after continuity snapshots keyed by request key/runtime name instead of assuming list order, and archive build/runtime/CLI artifacts under `.tmp/m047-s07/...`.
3. Assert the success route returns HTTP 200 with continuity truth `declared_handler_runtime_name=<actual handler>`, `replication_count=2`, `phase=completed`, `result=succeeded`; assert the explicit-count route returns the chosen HTTP 503/rejection contract with durable `unsupported_replication_count:3`; rerun `e2e_m032_route_*` unchanged.
4. Leave scaffold/docs adoption untouched in this slice; S08 owns migration of public surfaces to the shipped wrapper.

## Must-Haves

- [ ] `e2e_m047_s07` proves both the default-count success path and the unsupported explicit-count failure path against a live clustered HTTP app.
- [ ] Continuity inspection does not depend on list order and preserves the actual imported handler runtime name.
- [ ] The existing M032 bare-handler success and closure-handler failure controls remain green.
  - Estimate: 4h
  - Files: compiler/meshc/tests/e2e_m047_s07.rs, compiler/meshc/tests/support/m046_route_free.rs
  - Verify: - `cargo test -p meshc --test e2e_m047_s07 -- --nocapture`
- `cargo test -p meshc --test e2e_stdlib e2e_m032_route_ -- --nocapture`
