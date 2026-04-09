# S07 Research — Clustered HTTP route wrapper completion

## Summary
- S07 primarily owns **R100** (`HTTP.clustered(...)` route-local clustering) and **R101** (the route handler itself is the clustered boundary). It also has to preserve **R099** (general clustered-function model, not an HTTP-only special case) and reuse the **R098** truth seam from S02 (runtime-name + replication-count registry / continuity truth).
- The current tree still has **no compiler-known `HTTP.clustered(...)` surface**, **no clustered-route metadata in `TypeckResult` / `PreparedBuild`**, **no runtime clustered route request/reply path**, and **no `e2e_m047_s03`/S07 target**. `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` still fails with `no test target named e2e_m047_s03`.
- A minimal temp-project repro confirms the feature is genuinely absent today: plain `HTTP.on_get("/todos", handle_list_todos)` builds, but replacing the handler with `HTTP.clustered(handle_list_todos)` currently fails with unfocused type errors (`undefined variable: HTTP` twice, plus `undefined variable: router` in the piped route call). The failure is real, but the diagnostic quality is poor because `HTTP.clustered` is not a known stdlib/compiler surface yet.
- The blocked S03 decision is still correct: **do not implement this as a plain stdlib helper or closure wrapper**. Current route registration intrinsics only accept plain fn pointers, and the retained M032 control rail proves generic closure routes still fail at live request time.
- The largest runtime gap is **reply transport**. Route-free declared work is registered as actor-spawnable wrappers that ultimately return `Unit`; HTTP route handlers return `Response`. The existing remote spawn ABI only supports `int|float|bool|string|pid|unit`, so the route-free declared-work execution path cannot honestly carry `Request` / `Response`.
- S07 should stay off scaffold/docs. `compiler/mesh-pkg/src/scaffold.rs`, `compiler/meshc/tests/tooling_e2e.rs`, `compiler/meshc/tests/e2e_m047_s05.rs`, and `compiler/meshc/tests/e2e_m047_s06.rs` still assert the wrapper is unshipped. Adoption belongs to S08.

## Requirements Focus
- **R100 primary:** route-local clustering through `HTTP.clustered(handler)` / `HTTP.clustered(N, handler)`.
- **R101 primary:** the HTTP route handler is the clustered boundary; downstream helper/storage calls stay natural.
- **R099 supporting:** wrapper lowering must reuse the same clustered-function identity/runtime-name model as ordinary clustered functions instead of inventing a route-only public seam.
- **R098 supporting:** explicit/default counts must come from the declared-handler registry / continuity truth surface, not from wrapper-only ephemeral args.

## Skills Discovered
- `rust-best-practices` (already installed; no additional installs needed).
  - **Chapter 4:** new runtime/compiler helpers should return `Result` and use `?` for fallible decode/transport work; do not hide operational failures behind panics.
  - **Chapter 5:** keep tests narrow and descriptive — separate acceptance of direct/piped wrapper forms, closure misuse rejection, explicit-count truth, and runtime reply behavior instead of burying everything in one mega rail.
  - **Chapter 6:** prefer static generated shims over boxed/dyn handler abstractions or generic closure ABI widening.

## Current Repro / Blocker Evidence
- `cargo run -q -p meshc -- build .tmp/m047-s07-repro-http-local --no-color` succeeds for plain `HTTP.on_get("/todos", handle_list_todos)`.
- The same temp project with `HTTP.on_get("/todos", HTTP.clustered(handle_list_todos))` fails today with unfocused type errors. This is a real blocker repro, not just a missing search hit.
- `cargo test -p meshc --test e2e_stdlib e2e_m032_route_ -- --nocapture` passes and remains the key control rail: bare route handlers work; generic closure routes still fail only at live request time.
- `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` still fails with `no test target named e2e_m047_s03`.

## Implementation Landscape

### 1. Typeck / metadata seam
- `compiler/mesh-typeck/src/infer.rs`
  - `stdlib_modules()` defines the module-qualified `HTTP.*` surface used by `infer_field_access`.
  - `infer_call(...)` already has the right interception pattern for special call shapes and the existing metadata handoff model (`overloaded_call_targets`).
  - `ctx.imported_functions` only preserves **bare imported names**; it does **not** preserve defining-module origin. This matters because real router chains use `from Api.Todos import handle_list_todos`, but S07 needs runtime names like `Api.Todos.handle_list_todos`, not `Api.Router.handle_list_todos`.
  - `TypeckResult` currently collapses `qualified_modules` to `Vec<String>` names only. If wrapper metadata waits until after typecheck, imported bare-handler origin is already gone.
- `compiler/mesh-typeck/src/builtins.rs`
  - Duplicates the `HTTP` stdlib typing surface for builtin lookup. Missing `HTTP.clustered` here and in `infer.rs` is why the compiler currently has no truthful wrapper surface.
- `compiler/mesh-typeck/src/unify.rs` + `compiler/mesh-typeck/src/lib.rs`
  - `InferCtx` / `TypeckResult` already carry one structured callsite map (`overloaded_call_targets`). S03’s blocked direction is still right: `HTTP.clustered` wants a **sibling metadata map keyed by call range**, not a fake stdlib helper.
- `compiler/mesh-typeck/src/error.rs`, `compiler/mesh-typeck/src/diagnostics.rs`, `compiler/mesh-lsp/src/analysis.rs`
  - If S07 adds wrapper-specific diagnostics (non-route-position, non-bare handler, conflicting counts, etc.), all three surfaces have to move together.
- **Pipe-form nuance:** in `router |> HTTP.on_get("/todos", HTTP.clustered(handle))`, the route call only has **2 explicit args**. Validation must accept both direct 3-arg and piped 2-arg forms.

### 2. Build planning / lowering seam
- `compiler/meshc/src/main.rs`
  - `PreparedBuild` only carries `merged_mir` + route-free `clustered_execution_plan`.
  - That plan is built **before** MIR lowering via `mesh-pkg`’s source declaration collector; it has no route-position metadata today.
- `compiler/mesh-pkg/src/manifest.rs`
  - Good fit for `@cluster` source decorators.
  - Bad fit for `HTTP.clustered(...)`: wrapper validity depends on **call position** (`HTTP.route` / `HTTP.on_*`, direct or piped) and **imported bare-handler resolution**, not just AST shape.
  - I would **not** extend `collect_source_cluster_declarations(...)` to “parse” route wrappers. The S03 recommendation to keep wrapper metadata in typeck/lowering is stronger than trying to shoehorn route wrappers into manifest-style declaration validation.
- `compiler/mesh-codegen/src/mir/lower.rs`
  - `Lowerer` already receives `TypeckResult` metadata maps and can synthesize new MIR functions.
  - This is the natural place to:
    - intercept `HTTP.clustered(...)` at route-registration callsites,
    - generate **bare route shims**, and
    - lower wrapper usage without pretending the wrapper is first-class.
- `compiler/mesh-codegen/src/declared.rs`
  - Current `DeclaredHandlerKind::Work` wrapper generation only supports:
    - zero-arg work functions, or
    - legacy `(request_key :: String, attempt_id :: String)` work signatures.
  - `fn(request) -> Response` route handlers do **not** fit this ABI.
  - If route handlers share declared-handler registration, they need either:
    - a new codegen/runtime kind (route handler / HTTP route), or
    - a no-wrapper registration path that stores the real handler symbol and does **not** reuse the route-free actor-entry ABI.
  - `prepare_startup_work_registrations(...)` must keep filtering startup-only work; clustered routes must not auto-start.
- `compiler/mesh-codegen/src/codegen/mod.rs`
  - The main wrapper already registers:
    - all lowered top-level functions for remote lookup, and
    - declared handlers with runtime name + executable + replication count.
  - This is the right registration seam once route handlers can produce declared-handler plan entries.
- `compiler/mesh-codegen/src/codegen/expr.rs`
  - Important current constraint: the HTTP route intrinsics take a **plain fn pointer**, not a closure pair. The comments call this out explicitly.
  - This matches the retained M032 control rail and kills the “just return a closure from `HTTP.clustered`” idea.

### 3. Runtime seam
- `compiler/mesh-rt/src/http/router.rs`
  - `RouteEntry` only stores `pattern`, `method`, `handler_fn`, and `handler_env`.
  - `route_with_method(...)` hardcodes `handler_env = null`.
  - Route matching itself is fine; the missing seam is clustered execution, not path matching.
- `compiler/mesh-rt/src/http/server.rs`
  - `process_request(...)` is the concrete inbound seam:
    - it already parses method/path/body/headers/query/path params,
    - constructs `MeshHttpRequest`,
    - and currently ends at direct `call_handler(...)`.
  - The same file also has the outbound seam:
    - it already cracks `MeshHttpResponse` back into status/body/headers for the HTTP socket.
  - If S07 needs dedicated request/reply transport, this file already has the request/response pieces.
- `compiler/mesh-rt/src/dist/node.rs`
  - Declared-handler registry is the current source of truth for `runtime_name -> replication_count + fn_ptr + executable_name`.
  - `submit_declared_work(...)` still assumes actor-spawnable declared work and routes through `spawn_declared_work_local/remote(...)`.
  - Remote spawn encoding only supports `int|float|bool|string|pid|unit`; it cannot carry `Request` or `Response`.
  - So clustered HTTP routes cannot honestly reuse the existing declared-work actor-dispatch path unchanged.
- `compiler/mesh-rt/src/dist/continuity.rs`
  - `SubmitRequest` still requires non-empty `request_key`, `payload_hash`, and valid replica counts.
  - `mesh_continuity_submit_declared_work(...)` derives required replicas from the declared-handler registry; explicit `HTTP.clustered(N, handler)` only stays truthful if the wrapper also lands `N` in that registry (or changes this runtime API).
- `compiler/mesh-rt/src/dist/operator.rs`
  - `meshc cluster continuity --json` list output sorts by `request_key`, not recency. E2E discovery should use before/after diffs or explicit request keys, not “last record wins”.

### 4. Existing proof / control surfaces
- `compiler/meshc/tests/e2e_stdlib.rs`
  - `e2e_m032_route_bare_handler_control` = positive control.
  - `e2e_m032_route_closure_runtime_failure` = negative control.
  - These should stay green after S07 to prove wrapper-local clustering did **not** accidentally widen generic route closure support.
- `compiler/meshc/tests/support/m046_route_free.rs`
  - Good for cluster membership, continuity, and diagnostics queries.
  - `find_record_for_runtime_name(...)` assumes one record per runtime name; that is fine for startup work, not for HTTP routes with repeated requests.
- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
  - Already has HTTP port helpers, JSON request helpers, and artifact utilities. Useful if S07 adds a new custom clustered-route e2e target.
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `compiler/meshc/tests/e2e_m047_s06.rs`
  - All still assert the wrapper is unshipped. Leave them untouched in S07; S08 owns adoption.
- `reference-backend/api/router.mpl`
  - Useful real-world target shape: imported bare handlers in a pipe-based router chain. Good dogfood pattern, but a temp e2e package is the safer S07 proof surface.

## Recommendation
1. **Keep the compiler-known wrapper design from blocked S03.**
   - Implement `HTTP.clustered(...)` as a special `infer_call(...)` surface.
   - Record structured metadata keyed by wrapper call range and/or enclosing route call range.
   - Capture the resolved handler target **at typecheck time**, including defining-module origin for bare imported handlers.
   - Validate both direct and piped route forms.
   - Accept only bare handler references (`NameRef` or module-qualified function ref), not closures/anonymous fns/call expressions.
   - Reject conflicting duplicate wrappers for the same handler with different counts; dedupe identical repeats.

2. **Lower to generated bare route shims, not closures.**
   - The shim should keep the public route handler signature `fn(Request) -> Response`.
   - `runtime_name` should remain the actual handler identity (`Api.Todos.handle_list_todos`), not the shim name.
   - Keep startup work filtering unchanged; route wrappers are not startup work.

3. **Use the declared-handler registry for count truth, but do not reuse the route-free actor wrapper ABI.**
   - Route handler registrations need count truth in the same runtime registry from S02.
   - They likely need a distinct execution kind / ABI path from route-free `Work` entries because `(Request) -> Response` is not actor-spawnable via the current `request_key/attempt_id` wrapper path.

4. **Add a dedicated request/reply runtime seam for clustered HTTP routes.**
   - Do not patch `HTTP.clustered` into app-level `Continuity.submit_declared_work` + status polling.
   - Do not widen generic route closure support.
   - Do not rely on generic remote spawn args; that ABI cannot carry request/response.
   - The clean seam is a runtime helper/transport that knows how to:
     - serialize inbound `MeshHttpRequest`,
     - run the actual handler as the clustered boundary,
     - serialize `MeshHttpResponse` back,
     - and tie that call into continuity submit/complete truth.
   - If the implementation chooses a local-only first step instead of remote owner execution, call that out explicitly as a scope decision: it is smaller, but it weakens the shared clustered-function story from R099/R101.

5. **Use a new dedicated e2e target for S07; do not repurpose scaffold/docs yet.**
   - A custom temp package or new test fixture is the cleanest proof surface.
   - S08 can migrate the todo scaffold and docs once the runtime/compiler seam is real.

## Risks / Unknowns
- **Handler-origin tracking is currently missing for bare imported names.** Without new metadata, `HTTP.clustered(handle_list_todos)` in `Api.Router` cannot recover `Api.Todos.handle_list_todos` later.
- **Single-node proof is misleading for default count 2.** S02 only added the single-node relaxation for startup work. Ordinary clustered requests still want a replica. If S07 wants a positive default-wrapper success rail without adding a new carveout, use a 2-node cluster in e2e.
- **Current continuity list helpers assume one record per runtime name.** Clustered routes will create repeated records for the same handler runtime name; tests need before/after diffs or explicit request-key capture.
- **Request-key policy is still open.** The wrapper/runtime must generate or surface a non-empty `request_key` and `payload_hash`. Runtime-owned unique keys are the smallest honest default. If a deterministic per-request key is desired, collision semantics become product/API design.
- **Conflicting reuse of the same handler across wrapped routes needs a policy.** Same handler on multiple wrapped routes with the same count can dedupe; same handler with different counts should fail closed.

## Don’t Hand-Roll
- Do **not** model `HTTP.clustered(...)` as a plain stdlib identity helper or closure-returning helper.
  - Current route intrinsics expect plain fn pointers, and the retained M032 rail proves generic closure routes still fail at live request time.
- Do **not** extend `mesh-pkg::collect_source_cluster_declarations(...)` to parse route wrappers as if they were `@cluster`.
  - Route-position validation and imported bare-handler origin live in typeck/lowering, not in manifest-style AST scanning.
- Do **not** resurrect app-authored `POST /work` / `GET /work/:request_key` submit/status surfaces.
  - The old `work_continuity.mpl` pattern is exactly the proof-only seam M047 is trying to retire.
- Do **not** silently treat clustered routes as ordinary local handlers with only extra diagnostics unless the slice is explicitly re-scoped.
  - That would weaken R099/R101 by sidestepping the shared clustered execution model.

## Verification Plan
- **Compiler/typeck unit rails**
  - `cargo test -p mesh-typeck m047_s07 -- --nocapture`
    - accept inline wrapper in direct + pipe form
    - reject non-route-position use
    - reject closure / anonymous fn / private handler misuse
    - preserve defining-module runtime name for imported bare handlers
    - preserve/detect conflicting explicit counts
- **Lowering/codegen unit rails**
  - `cargo test -p mesh-codegen m047_s07 -- --nocapture`
    - generated bare shim exists
    - declared-handler registration uses real handler runtime name + count
    - route handlers are not added to startup registrations
    - explicit `N` reaches registration/IR markers
- **Runtime unit rails**
  - `cargo test -p mesh-rt m047_s07 -- --nocapture`
    - local clustered route call returns handler response and completes continuity
    - unsupported explicit count produces rejected record + 503/error path
    - request/response encode/decode roundtrip for any new transport
- **E2E proof**
  - `cargo test -p meshc --test e2e_m047_s07 -- --nocapture`
    - boot a 2-node HTTP app
    - hit one `HTTP.clustered(handler)` route and assert:
      - live HTTP 200
      - continuity record `declared_handler_runtime_name=<actual handler>`
      - `replication_count=2`
      - `phase=completed`, `result=succeeded`
    - hit one `HTTP.clustered(3, handler)` route and assert:
      - live HTTP 503 (or the chosen rejected-route contract)
      - continuity record `replication_count=3`
      - durable rejection reason `unsupported_replication_count:3`
- **Guardrail**
  - `cargo test -p meshc --test e2e_stdlib e2e_m032_route_ -- --nocapture`
    - bare route control still passes
    - generic closure route still fails

## Key Files
- `.gsd/milestones/M047/slices/S03/S03-SUMMARY.md` — blocker mapping and previously narrowed implementation direction
- `compiler/mesh-typeck/src/infer.rs` — compiler-known `HTTP.clustered` surface, route-position validation, imported-origin capture
- `compiler/mesh-typeck/src/builtins.rs` — builtin HTTP surface duplication
- `compiler/mesh-typeck/src/unify.rs`
- `compiler/mesh-typeck/src/lib.rs` — metadata handoff parallel to `overloaded_call_targets`
- `compiler/mesh-typeck/src/error.rs`
- `compiler/mesh-typeck/src/diagnostics.rs`
- `compiler/mesh-lsp/src/analysis.rs`
- `compiler/meshc/src/main.rs` — `PreparedBuild` / plan plumbing
- `compiler/mesh-codegen/src/mir/lower.rs` — route wrapper interception + shim synthesis
- `compiler/mesh-codegen/src/declared.rs` — declared-handler kind / registration path for route handlers
- `compiler/mesh-codegen/src/codegen/mod.rs` — main-wrapper registration emission
- `compiler/mesh-codegen/src/codegen/expr.rs` — current plain-fn route ABI limitation
- `compiler/mesh-rt/src/http/server.rs` — request/response serialization seam
- `compiler/mesh-rt/src/http/router.rs` — route entry shape
- `compiler/mesh-rt/src/dist/node.rs` — declared-handler registry + current remote spawn limits
- `compiler/mesh-rt/src/dist/continuity.rs` — count/request/payload truth
- `compiler/mesh-rt/src/dist/operator.rs` — continuity list ordering / query behavior
- `compiler/meshc/tests/e2e_stdlib.rs` — M032 control rails
- `compiler/meshc/tests/support/m046_route_free.rs`
- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `compiler/meshc/tests/e2e_m047_s06.rs`
- `reference-backend/api/router.mpl`
