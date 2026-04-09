# S08 Research — Clustered route adoption in scaffold, docs, and closeout proof

## Summary
- S08 is **adoption work, not new compiler/runtime work**. S07 already shipped the real `HTTP.clustered(handler)` / `HTTP.clustered(N, handler)` path end to end. The remaining gap is that the scaffold, docs, and closeout verifier still teach the old **“wrapper is unshipped”** story.
- The exact stale surfaces are already concentrated and easy to name:
  - `compiler/mesh-pkg/src/scaffold.rs` still says the Todo template “does **not** pretend `HTTP.clustered(...)` exists yet” and generates only plain `HTTP.on_*` handlers.
  - `compiler/meshc/tests/tooling_e2e.rs`, `compiler/meshc/tests/e2e_m047_s05.rs`, and `compiler/meshc/tests/e2e_m047_s06.rs` all enforce that stale contract.
  - `README.md`, `website/docs/docs/tooling/index.md`, `website/docs/docs/getting-started/clustered-example/index.md`, `website/docs/docs/distributed/index.md`, and `website/docs/docs/distributed-proof/index.md` all still say `HTTP.clustered(...)` is not shipped.
  - `scripts/verify-m047-s06.sh` has an explicit contract guard that fails if that non-goal wording disappears.
- The **first real technical gotcha** is not in runtime code; it is in generated handler signatures. I verified with a temp package that wrapping the current untyped scaffold-style handler shape fails with:
  - `E0047 invalid HTTP.clustered(...) usage`
  - ``handle_list_todos` must have type `(Request) -> Response``
  So any generated handler that gets wrapped must be explicitly typed.
- The **second real choice** is replication count strategy for the starter:
  - `HTTP.clustered(handler)` on a single node returns `503 {"error":"replica_required_unavailable"}`.
  - `HTTP.clustered(1, handler)` works on a single node and, when booted through `Node.start_from_env()`, records truthful continuity with `declared_handler_runtime_name=Api.Todos.handle_list_todos`, `replication_count=1`, `replication_health=local_only`, `fell_back_locally=true`, and `routed_remotely=false`.
- That makes the main product/planning decision clear: **if the Todo starter must stay a one-process / one-container starting point, use `HTTP.clustered(1, ...)` on selected routes.** If the starter instead uses default-count `HTTP.clustered(handler)`, then current native/Docker starter rails break and the slice grows into a mandatory two-node/multi-container proof harness.
- You do **not** need shared multi-node SQLite just to prove route adoption. The starter already has S05 CRUD/persistence proof. S08 can prove clustered-route adoption by hitting an empty wrapped `GET /todos` route and inspecting continuity truth, while leaving the existing database/persistence story delegated to S05/S06 rails.

## Requirements Focus
- **Primary:**
  - **R099** — keep clustering as a general function capability, not an HTTP-only story.
  - **R102** — remove the stale public authority that still presents `HTTP.clustered(...)` as unshipped.
  - **R103** — preserve the route-free canonical surfaces while layering truthful route-wrapper adoption on top.
  - **R104** — keep the Todo scaffold and Docker path honest while adding shipped clustered-route syntax.
  - **R105** — keep the scaffold readable and starting-point-like instead of turning it into a proof app.
  - **R106** — teach one coherent source-first clustered model across README, docs, scaffold, and verifier rails.
- **Supporting consumed truth from dependency slices:**
  - **R100 / R101** are already satisfied by S07; S08 only needs to dogfood that shipped wrapper truthfully.

## Skills Discovered
No new installs were needed. The directly relevant skills were already present.

- `rust-best-practices`
  - **Chapter 5 (Automated Testing):** keep new rails narrow and descriptive. This argues for a clean split between fast scaffold-contract assertions, runtime-adoption proof, and docs/verifier-contract proof instead of shoving all of S08 into one giant monolithic test.
  - **Chapter 4 (Error Handling):** helper/seam changes should keep fail-closed artifact paths and clear diagnostics instead of hiding drift behind implicit fallbacks.
- `vitepress`
  - The docs site is file-routed Markdown. The touched pages are already present in `website/docs/.vitepress/config.mts`, so S08 is a **content-only docs task** unless page titles/locations change.
- `multi-stage-dockerfile`
  - Preserve the current builder/runtime separation and minimal runtime image. S08 should **not** reopen the old in-image compiler/install path; it should keep the current prebuilt `./output` packaging model.

## Decision Constraints
Relevant append-only decisions already on record:

- **D266** — clustered HTTP routes should use wrapper syntax `HTTP.clustered(handler)` / `HTTP.clustered(N, handler)`, not verb-specific APIs.
- **D273 / D275** — replication count is runtime-owned truth via declared-handler registration; explicit counts are real, and unsupported fanout must fail closed instead of being clipped.
- **D293** — S06 deliberately kept docs/verifier surfaces on a route-free canonical story and explicitly marked `HTTP.clustered(...)` as unshipped. S08 must supersede that with a **new decision entry**, not edit history.
- **D299 / D300 / D301** — S07 shipped `HTTP.clustered(...)` as wrapper metadata + deterministic route shims + shared declared-handler runtime truth. S08 should consume that seam exactly; it should not invent scaffold-local or docs-local clustering behavior.

## Current Repro / Proof Evidence
I ran direct temp-package probes under `.tmp/` to narrow the actual adoption constraints.

### 1. Untyped wrapped handlers do not compile
A minimal scaffold-shaped route handler:

```mesh
pub fn handle_list_todos(_request) do
  HTTP.response(200, "ok")
end
```

fails when used as:

```mesh
HTTP.router() |> HTTP.on_get("/todos", HTTP.clustered(handle_list_todos))
```

with:

- `E0047 invalid HTTP.clustered(...) usage`
- ``handle_list_todos` must have type `(Request) -> Response``

**Implication:** any generated handler that is wrapped must gain explicit `Request`/`Response` annotations.

### 2. Default-count wrapper is not one-node-friendly
A minimal single-node app using:

```mesh
HTTP.router() |> HTTP.on_get("/todos", HTTP.clustered(handle_list_todos))
```

builds, but a live request returns:

- `HTTP 503`
- `{"error":"replica_required_unavailable"}`

**Implication:** if the scaffold adopts default-count wrapper syntax on starter-critical routes, the current one-process local run and one-container Docker path become red unless the proof and README shift to mandatory multi-node topology.

### 3. Explicit-count `1` works on one node and still records continuity truth
A minimal single-node app using:

```mesh
HTTP.router() |> HTTP.on_get("/todos", HTTP.clustered(1, handle_list_todos))
```

returns `200` on one node.

When booted through `Node.start_from_env()` with `MESH_*` env, the same request leaves continuity truth accessible through Mesh CLI:

- `declared_handler_runtime_name = Api.Todos.handle_list_todos`
- `replication_count = 1`
- `phase = completed`
- `result = succeeded`
- `replication_health = local_only`
- `fell_back_locally = true`
- `routed_remotely = false`

Two small gotchas from the probe:
- the continuity **list** can lag briefly right after the HTTP 200; a poll/wait helper is needed
- single-node `MESH_DISCOVERY_SEED=localhost` emits harmless self-discovery noise (`TCP connect to ::1:<port> failed`) before settling on `local_only`

**Implication:** S08 can prove native and Docker clustered-route adoption honestly without forcing a two-node starter if the scaffold uses explicit count `1` on selected routes.

## Implementation Landscape

### 1. Scaffold generator and fast scaffold-contract rails
**Files:**
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`

**Current state:**
- `compiler/mesh-pkg/src/scaffold.rs:223-225` still describes the Todo template as route-free and explicitly says it does **not** pretend `HTTP.clustered(...)` exists yet.
- `compiler/mesh-pkg/src/scaffold.rs:409-418` generates only plain route registrations:
  - `HTTP.on_get("/todos", handle_list_todos)`
  - `HTTP.on_get("/todos/:id", handle_get_todo)`
  - `HTTP.on_post(...)`, `HTTP.on_put(...)`, `HTTP.on_delete(...)`
- Wrapped handler candidates are currently untyped:
  - `handle_list_todos` at `compiler/mesh-pkg/src/scaffold.rs:458`
  - `handle_get_todo` at `compiler/mesh-pkg/src/scaffold.rs:465`
  - mutations at `:477`, `:493`, `:509`
- The scaffold unit test and fast tooling smoke both still forbid the wrapper:
  - `compiler/mesh-pkg/src/scaffold.rs:969-985, 1042`
  - `compiler/meshc/tests/tooling_e2e.rs:542-600`

**Natural seam:**
- Update router generation, wrapped handler signatures, and generated README wording **together**.
- Then rebaseline the scaffold unit test and `tooling_e2e` smoke as one fast-contract task.

**Low-risk starter shape:**
- Keep `work.mpl` on `@cluster pub fn sync_todos()`.
- Keep write routes ordinary local handlers.
- Wrap only selected read routes, e.g.:
  - `GET /todos`
  - optionally `GET /todos/:id`
- Prefer `HTTP.clustered(1, handle_list_todos)` / `HTTP.clustered(1, handle_get_todo)` if the starter must remain honest in one-node local and Docker paths.

### 2. Generated-project runtime and Docker proof support
**Files:**
- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
- `compiler/meshc/tests/support/m046_route_free.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `compiler/meshc/tests/e2e_m047_s07.rs`

**Current state:**
- `m047_todo_scaffold.rs` is still S05-shaped:
  - hardcoded artifact bucket at `:67-68` -> `.tmp/m047-s05/...`
  - hardcoded Docker builder image tag at `:16`
  - `spawn_todo_app(...)` at `:165` only injects `PORT` + `TODO_*`; no `MESH_*`
  - `TodoDockerContainerConfig` at `:34-41` only models HTTP publication and data-dir mounting
  - `docker_spawn_todo_container(...)` at `:611+` creates a one-container HTTP proof with no cluster identity/cluster-port publication
- `m046_route_free.rs` already has the runtime-owned cluster helpers S08 needs:
  - `dual_stack_cluster_port()`
  - `spawn_route_free_runtime(...)`
  - `wait_for_cluster_status_matching(...)`
  - `wait_for_continuity_list_matching(...)`
  - `wait_for_continuity_record_matching(...)`
  - `new_request_keys_for_runtime_name(...)`
- `e2e_m047_s07.rs:292-410` already contains small helper functions for clustered-route continuity discovery that can be reused or extracted.
- `e2e_m047_s05.rs` currently mixes three concerns:
  - scaffold content assertions
  - native CRUD/persistence proof
  - container packaging/runtime proof
  and it assumes wrapped routes do **not** exist.

**Natural seam:**
- Keep the existing S05 CRUD/persistence proof as the lower-level starter rail.
- Add the smallest new helper surface needed for S08 adoption proof:
  - native single-node cluster-mode boot for the generated Todo app
  - Docker single-container cluster-mode boot for the generated image
  - continuity/status inspection against the wrapped route runtime name
- If the helper is reused from a new S08 test, parameterize the artifact bucket instead of reusing the S05 constant.

**Important scope shortcut:**
- S08 does **not** need new shared SQLite or multi-node database correctness just to prove route adoption.
- The existing S05/S06 rails already prove CRUD/rate limiting/restart persistence and Docker packaging.
- For clustered-route adoption, an empty `GET /todos` request is enough.

### 3. Public docs and closeout-verifier authority
**Files:**
- `README.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/.vitepress/config.mts`
- `compiler/meshc/tests/e2e_m047_s06.rs`
- `scripts/verify-m047-s06.sh`

**Current state:**
- The exact stale migration string is centralized in `compiler/meshc/tests/e2e_m047_s06.rs:6` and still ends with ``HTTP.clustered(...)` is still not shipped.``
- The five public docs surfaces all contain that stale wording:
  - `README.md:122, 191`
  - `website/docs/docs/tooling/index.md:222`
  - `website/docs/docs/getting-started/clustered-example/index.md:22`
  - `website/docs/docs/distributed/index.md:8`
  - `website/docs/docs/distributed-proof/index.md:25`
- `scripts/verify-m047-s06.sh:388-430` has a hard contract guard:
  - regex `HTTP\.clustered\(\.\.\.\).*not shipped`
- `website/docs/.vitepress/config.mts` already includes the touched pages in sidebar/nav. No routing or theme work appears necessary.

**Natural seam:**
- Treat docs + `e2e_m047_s06.rs` + `scripts/verify-m047-s06.sh` as one authority layer.
- Update them together so the public pages, Rust contract test, and shell verifier all teach the same story.

**Current-state recommendation:**
- Prefer updating the existing S06 authority files in place rather than creating a third competing closeout layer.
- If a new `S08` wrapper script is added anyway, immediately rebaseline or alias the current S06 files; do not leave stale red non-goal guards behind.

### 4. Two-node / multi-container fallback only if product insists on default-count starter routes
If the user/product explicitly wants the Todo starter itself to show zero-boilerplate `HTTP.clustered(handler)` instead of explicit `1`, the reference implementation pattern already exists, but it is much larger in scope:

**Files to reuse as pattern only:**
- `compiler/meshc/tests/e2e_m043_s03.rs`
- `scripts/verify-m039-s04.sh`
- `scripts/verify-m042-s04.sh`

They already show:
- Docker bridge-network creation
- shared discovery alias
- explicit hostnames
- deterministic node naming
- attached container log capture
- cleanup of networks/containers

This path is viable, but it is **not** the smallest S08.

## Recommendation
1. **Do not touch compiler/runtime/typecheck/lowering.** S07 is already the authority. S08 should stay in scaffold/tests/docs/verifier land.
2. **Adopt `HTTP.clustered(1, ...)` on selected read routes in the Todo scaffold.**
   - Best candidates: `GET /todos` and optionally `GET /todos/:id`.
   - Keep `POST`/`PUT`/`DELETE` ordinary local routes behind the actor-backed limiter.
   - Keep `work.mpl` route-free with `@cluster pub fn sync_todos()`.
3. **Add explicit `(Request) -> Response` type signatures to any wrapped handlers.**
4. **Keep the existing one-process / one-container starter story.**
   - Local run remains honest.
   - Docker packaging remains honest.
   - New cluster-mode proofs can be added without converting the starter into a mandatory two-node harness.
5. **Layer docs truthfully:**
   - route-free `@cluster` surfaces remain the canonical general clustered model
   - `HTTP.clustered(...)` is now shipped route-local sugar on the same runtime seam
   - the Todo starter chooses `HTTP.clustered(1, ...)` on selected routes so it still boots cleanly in the starter’s current standalone/native/Docker paths
   - S07 remains the authoritative default-count/two-node wrapper proof surface
6. **Keep verifier ownership current-state-first.**
   - Rebaseline the existing S05 lower-level starter rail and the existing S06 docs/verifier authority rail.
   - Add a focused S08 Rust e2e only if the new cluster-mode adoption proof does not fit cleanly into the existing S05 runtime target.

## Risks / Unknowns
- **Starter route choice is the main product decision.**
  - Default-count wrapper gives the prettier zero-argument syntax.
  - Explicit-count `1` keeps the starter and Docker path honest without turning the app into a two-node requirement.
- **Continuity list population is eventually consistent right after the HTTP request.** Use wait helpers, not immediate snapshots.
- **Single-node `localhost` discovery is noisy.** Expect transient `::1` self-connect failures before the node settles on `local_only`; do not treat those lines alone as failure.
- **The S05 helper module is bucket- and Docker-tag-hardcoded to S05.** If S08 adds a new test target, parameterize or wrap before reusing it.
- **If the planner chooses default-count starter routes anyway, scope jumps materially** into two-node native proof and likely a multi-container Docker harness.

## Don’t Hand-Roll
- Do **not** reopen S07’s compiler/runtime work in S08.
- Do **not** add a proof-only `/todos/retry` or other fake route just to replay S07’s explicit-count rejection in the public starter.
- Do **not** switch the Dockerfile back to in-image compilation or installer-based build.
- Do **not** make the starter’s ordinary local/Docker instructions require two nodes unless the user explicitly wants that heavier product story.
- Do **not** add package-owned cluster-control HTTP routes. Use Mesh-owned CLI continuity/status surfaces exactly as the route-free story already does.

## Verification Plan
### Fast scaffold-contract rails
- `cargo test -p mesh-pkg m047_s05 -- --nocapture`
- `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`

### Lower-level starter/runtime proof
- Keep `cargo test -p meshc --test e2e_m047_s05 -- --nocapture` as the lower-level Todo starter/runtime/Docker rail, but rebaseline it for wrapped read routes.
- If that file becomes too overloaded, add a focused adoption rail:
  - `cargo test -p meshc --test e2e_m047_s08 -- --nocapture`

### Route-wrapper authority rail
- Keep `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` as the authoritative default-count/two-node wrapper seam if docs reference the no-count form.

### Docs / closeout authority
- `cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture` if S06 authority files are updated in place
- `bash scripts/verify-m047-s06.sh` if S06 remains the final assembled verifier name
- `npm --prefix website run build`

### What the S08 adoption proof should actually assert
If using the recommended explicit-count-1 starter route:
- native cluster-mode single-node boot of generated Todo app
- hit wrapped `GET /todos`
- poll continuity until the route record appears
- assert:
  - `declared_handler_runtime_name = Api.Todos.handle_list_todos`
  - `replication_count = 1`
  - `phase = completed`
  - `result = succeeded`
  - `replication_health = local_only`
- Docker single-container cluster-mode boot of generated image
- hit the same wrapped route
- inspect `meshc cluster status|continuity` against the published cluster port

## Key Files
- `compiler/mesh-pkg/src/scaffold.rs` — Todo starter source generation, README generation, unit tests.
- `compiler/meshc/tests/tooling_e2e.rs` — fast scaffold smoke contract.
- `compiler/meshc/tests/support/m047_todo_scaffold.rs` — starter build/run/Docker helper seam; currently S05-hardcoded.
- `compiler/meshc/tests/support/m046_route_free.rs` — cluster env + continuity/status helper seam.
- `compiler/meshc/tests/e2e_m047_s05.rs` — lower-level starter runtime/Docker proof and public surface contract.
- `compiler/meshc/tests/e2e_m047_s07.rs` — authoritative wrapper-semantics proof and reusable continuity helpers.
- `compiler/meshc/tests/e2e_m047_s06.rs` — current docs/verifier contract harness.
- `scripts/verify-m047-s05.sh` — lower-level Todo/runtime subrail.
- `scripts/verify-m047-s06.sh` — current final closeout/verifier authority.
- `README.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/.vitepress/config.mts` — confirms this is content-only docs work.
- `.gsd/DECISIONS.md` — append a new S08 decision instead of mutating D293 history.
- `.gsd/KNOWLEDGE.md` — already records S07 route-request-key and S06 Docker/bundle gotchas; update if S08 chooses the explicit-count-1 starter strategy.
