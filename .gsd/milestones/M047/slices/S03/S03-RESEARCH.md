# S03 Research — Clustered HTTP route wrapper

## Summary

S03 is a real compiler/runtime integration slice, not a syntax-only change. The repo has **no `HTTP.clustered(...)` surface today**. Current HTTP routing still registers **plain handler function pointers** through `mesh_http_route*`, while S02’s truthful clustered execution path lives in the **declared-handler runtime registry** keyed by `runtime_registration_name` plus `replication_count`.

The critical constraint is already proven elsewhere in the repo: **HTTP route closures still fail at live request time**. The retained M032 rail (`compiler/meshc/tests/e2e_stdlib.rs::e2e_m032_route_closure_runtime_failure`) and `mesher/ingestion/routes.mpl` both document the same thing: bare handlers work, closure-wrapped handlers do not. `compiler/mesh-codegen/src/codegen/expr.rs` confirms why — route intrinsics still expect a plain handler pointer and intentionally avoid closure expansion when that would change the ABI. So `HTTP.clustered(...)` cannot honestly be implemented as “just return a user closure” unless the route-registration ABI is widened.

There is also a second constraint underneath the syntax: S02’s declared clustered work path is still **submit/status oriented**, not **`Request -> Response` oriented**. `compiler/mesh-rt/src/dist/continuity.rs::mesh_continuity_submit_declared_work(...)` and `compiler/mesh-rt/src/dist/node.rs::submit_declared_work(...)` submit a continuity record and spawn declared work keyed by `request_key` / `attempt_id`; they do **not** return a route handler’s `Response`. If S03 needs a clustered HTTP route that returns a normal response on the same request, it needs a response-returning transport. The closest existing precedent is the **service call/reply** path in `compiler/mesh-rt/src/actor/service.rs` and `compiler/mesh-codegen/src/codegen/expr.rs::codegen_service_call_helper(...)`.

The honest narrow path for S03 is therefore:

- keep `HTTP.clustered(...)` as a **compiler-known wrapper surface**, not a user closure trick
- lower it onto the **same S02 runtime-name + replication-count truth surface**, with no route-local shadow metadata
- generate **bare route shims** or equivalent compiler-known wrappers so the router still receives a real handler function pointer
- if remote execution must return a `Response`, reuse a **service-style reply transport**, not continuity records alone

That stays aligned with R100/R101 and avoids accidentally broadening S03 into a general fix for all route closures.

## Requirements targeted

- **R100** — primary slice requirement: support route-local clustering through `HTTP.clustered(...)` wrappers.
- **R101** — primary slice requirement: make the route handler the clustered boundary while downstream calls remain natural.
- **R099** — supporting requirement: preserve clustering as the same general runtime-name/declared-handler model already used by non-HTTP clustered functions.
- **R098** — already validated by S02, but S03 must preserve its semantics: bare wrapper means `replication_count=2`, explicit count must stay visible/truthful, and unsupported higher fanout must remain durably queryable instead of silently clipped.

## Skills discovered

- Checked installed skills in the prompt. No directly relevant route-wrapper/compiler-integration skill was already installed.
- Ran `npx skills find "LLVM compiler codegen"`; it surfaced the existing `llvm` skill, but this slice is not primarily raw LLVM work, so no new skill was installed.
- No loaded skill materially changed the implementation approach for this slice.

## Recommendation

Prefer a **generated bare-wrapper design** over widening the public route ABI to closure env pairs.

Why:

1. It keeps S03 scoped to **clustered route wrappers**, not a repo-wide “HTTP route closures now work” cutover.
2. It matches the user-visible goal: `HTTP.clustered(handle)` should read like route-local sugar over the same clustered-function model, not a brand-new routing runtime.
3. It preserves the current truthful M032 limitation unless the project deliberately chooses to retire it in this slice.
4. The runtime router/server already knows how to *invoke* `(fn_ptr, env_ptr)` pairs, but the registration ABI does not accept them. Widening that ABI is possible, but it is a broader product decision than the slice strictly requires.

The architectural decision the planner should force early is:

- **Option A — generated bare shims (recommended):** keep `mesh_http_route*` signatures unchanged; compiler lowers `HTTP.clustered(...)` into a generated route handler symbol that owns clustered dispatch and reply handling.
- **Option B — route ABI widening:** teach route registration to accept `(fn_ptr, env_ptr)` and pass closures through. This likely retires the old route-closure limitation too, so it is materially wider in scope.

Given the milestone wording and the retained M032 proof surface, Option A is the better default unless early implementation proves it impossible.

## Implementation landscape

### 1) Type / stdlib surface

**Files:**
- `compiler/mesh-typeck/src/builtins.rs`
- `compiler/mesh-typeck/src/infer.rs`

**Current role:**
- `HTTP` module exports `router`, `route`, `on_get`, `on_post`, `on_put`, `on_delete`, all typed as taking `Fn(Request) -> Response`.
- Module-qualified stdlib access already works through `infer.rs` field-access resolution for stdlib modules.

**S03 implication:**
- `HTTP.clustered` belongs here first.
- The planner needs to decide whether `HTTP.clustered(...)` returns:
  - another `Fn(Request) -> Response`, or
  - a new compiler-only wrapper/sentinel type that only `HTTP.on_*` consumes.

Because the current route runtime still wants a plain function pointer, a fake “returns any ordinary function value” surface is risky unless lowering special-cases it immediately.

### 2) Lowering / intrinsic mapping

**Files:**
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-codegen/src/codegen/expr.rs`

**Current role:**
- `mir/lower.rs` maps `http_on_get` / `http_on_post` / etc. onto `mesh_http_route_*` intrinsics.
- `codegen/intrinsics.rs` declares `mesh_http_route*` with the current `(router, pattern, handler_fn)` ABI.
- `codegen/expr.rs` contains the closure/FnPtr expansion logic and explicitly avoids expanding route handlers into `(fn_ptr, env_ptr)` when the intrinsic signature would mismatch.

**S03 implication:**
- If the slice uses generated bare shims, this is where the compiler has to recognize `HTTP.clustered(...)` and produce a real symbol the router can register.
- If the slice instead widens the route ABI, all three of these files move together.
- `codegen/expr.rs::codegen_service_call_helper(...)` is the most relevant existing pattern for “invoke something elsewhere and wait for a typed reply”.

### 3) Router / HTTP runtime

**Files:**
- `compiler/mesh-rt/src/http/router.rs`
- `compiler/mesh-rt/src/http/server.rs`

**Current role:**
- `router.rs` stores both `handler_fn` and `handler_env` in `RouteEntry`, but the public registration functions (`mesh_http_route`, `mesh_http_route_get`, etc.) currently always store a **null env** and only accept `handler_fn`.
- `server.rs::call_handler(...)` already knows how to call either a bare handler or an env-bearing handler.

**S03 implication:**
- This is the concrete split point between the two architectural options:
  - generated bare shims: router ABI stays as-is
  - env-bearing routes: router ABI changes here first
- If the planner wants to keep the M032 closure limitation alive for now, leave this ABI alone.

### 4) Existing clustered execution planning seam

**Files:**
- `compiler/mesh-pkg/src/manifest.rs`
- `compiler/meshc/src/main.rs`
- `compiler/mesh-codegen/src/declared.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/meshc/src/cluster.rs`

**Current role:**
- `mesh-pkg` builds the clustered export surface for **public work functions** and **generated service call/cast handlers**.
- `meshc/src/main.rs` validates clustered declarations, turns them into `ClusteredExecutionMetadata`, and prepares declared runtime registrations.
- `declared.rs` turns planned clustered entries into runtime registrations and wrapper symbols.
- `node.rs` / `continuity.rs` own the truthful runtime behavior and continuity records keyed by runtime name + replication count.
- `meshc cluster continuity` already exposes `declared_handler_runtime_name` and `replication_count` truthfully.

**S03 implication:**
- S03 should reuse this exact truth surface.
- What does **not** exist yet is any representation of a **clustered HTTP route wrapper** in this plan.
- The planner needs to decide where synthetic route-wrapper clustered metadata is created:
  - extend the existing planning surface to synthesize clustered route entries, or
  - bypass mesh-pkg planning and have lowering/codegen emit equivalent declared-handler registrations directly.

The first option is more coherent if the project wants diagnostics and CLI truth to stay uniform. The second may be smaller if the route wrapper is purely a lowering concern.

### 5) Response-returning transport precedent

**Files:**
- `compiler/mesh-rt/src/actor/service.rs`
- `compiler/mesh-codegen/src/codegen/expr.rs` (`codegen_service_call_helper`)
- `compiler/mesh-codegen/src/mir/lower.rs` (service helper generation)

**Current role:**
- Service call/reply already implements “send work to another actor and block for a reply”.
- The runtime reply format is simple and synchronous.

**S03 implication:**
- This is the closest honest starting point for a clustered route boundary that must return a `Response` immediately.
- Continuity submit/status surfaces alone are insufficient for HTTP response delivery.

### 6) Real dogfood / proof surfaces

**Files:**
- `mesher/ingestion/routes.mpl`
- `reference-backend/api/router.mpl`
- `compiler/meshc/tests/e2e_stdlib.rs`
- `compiler/meshc/tests/e2e_m047_s02.rs`
- `compiler/meshc/tests/support/m046_route_free.rs`

**Current role:**
- `mesher/ingestion/routes.mpl` is the most explicit current dogfood comment: “Handlers are bare functions (HTTP routing does not support closures).”
- `reference-backend/api/router.mpl` is the simpler real router-chain pattern.
- `e2e_stdlib.rs` already has live server/request harnesses and the retained bare-vs-closure proofs.
- `e2e_m047_s02.rs` + `support/m046_route_free.rs` already know how to build temp clustered apps, spawn them, query cluster status/continuity, and retain artifacts.

**S03 implication:**
- There is enough existing harness code to prove clustered HTTP routes without inventing a new testing stack.

## Natural task boundaries

### T1 — Type surface and misuse diagnostics

**Likely files:**
- `compiler/mesh-typeck/src/builtins.rs`
- `compiler/mesh-typeck/src/infer.rs`
- tests near existing HTTP typing coverage

**Goal:**
Add `HTTP.clustered(handler)` and `HTTP.clustered(N, handler)` to the `HTTP` module surface with truthful misuse errors.

**Decision to make here:**
- Is the wrapper represented as a normal function type or a compiler-only sentinel type?

**Why this is first:**
- It pins down the syntax/typing contract before the planner commits to a runtime design.

### T2 — Lowering and clustered metadata/planning seam

**Likely files:**
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/meshc/src/main.rs`
- `compiler/mesh-pkg/src/manifest.rs` (if route wrappers are synthesized into the shared clustered plan)
- possibly `compiler/mesh-codegen/src/declared.rs`

**Goal:**
Translate `HTTP.clustered(...)` into the same runtime-name + replication-count execution model S02 already proved.

**Key question:**
- Where do route-wrapper clustered entries live before codegen?

**Do not do:**
- invent route-local replication metadata separate from S02’s declared-handler registry

### T3 — Runtime execution path for `Request -> Response`

**Likely files:**
- `compiler/mesh-codegen/src/codegen/expr.rs`
- `compiler/mesh-rt/src/http/router.rs` / `server.rs` (only if ABI changes)
- `compiler/mesh-rt/src/actor/service.rs` or a new similar transport path
- `compiler/mesh-rt/src/dist/node.rs` / continuity runtime if route shims still register as declared handlers

**Goal:**
Make the clustered route wrapper actually return a live HTTP `Response` while preserving clustered truth surfaces.

**Scope guidance:**
- Prefer a generated route shim + reply transport.
- Avoid broad generic route-closure support unless the team explicitly accepts that as part of S03.

### T4 — Proof rails and retained-limit guards

**Likely files:**
- `compiler/meshc/tests/e2e_m047_s03.rs` (new)
- `compiler/meshc/tests/e2e_stdlib.rs` (keep or adapt closure-limit control)
- `compiler/meshc/tests/support/m046_route_free.rs` (reused helpers)

**Goal:**
Prove the new wrapper live, not compile-only, and keep the pre-existing closure-limit story truthful unless deliberately retired.

## Key constraints and risks

### 1) A library-level closure wrapper is not honest today

Evidence chain:
- `mesher/ingestion/routes.mpl` still carries the real keep-site comment.
- `compiler/meshc/tests/e2e_stdlib.rs::e2e_m032_route_closure_runtime_failure` proves compile-pass/live-fail.
- `compiler/mesh-codegen/src/codegen/expr.rs` explicitly avoids changing route handler ABI by expanding closures when the target intrinsic does not expect the extra env pointer.

**Planning consequence:**
Do not decompose S03 as “add an ordinary `HTTP.clustered` function in Mesh stdlib source and call it a day.” That would recreate the retained M032 failure mode.

### 2) The current declared work runtime is not a response transport

Evidence chain:
- `mesh_continuity_submit_declared_work(...)` submits continuity work and returns submit decisions.
- `submit_declared_work(...)` in `node.rs` spawns work keyed by `request_key` / `attempt_id`.
- Nothing in that seam returns the underlying handler’s `Response` to the HTTP ingress path.

**Planning consequence:**
S03 needs a response-returning seam. The cleanest precedent is service call/reply, not continuity status polling.

### 3) Route ABI widening is a bigger product choice than the slice strictly requires

Because `server.rs` already knows how to invoke env-bearing handlers and `router.rs` already stores `handler_env`, the repo could widen `mesh_http_route*` to accept `(fn_ptr, env_ptr)` and thereby solve more than S03.

**Planning consequence:**
If the planner wants the smallest slice, keep the ABI unchanged and use generated bare wrappers.

### 4) Explicit counts must remain truthful even if unsupported

S02 already established the rule: `replication_count=3` must remain visible and durably rejected, not silently clipped.

**Planning consequence:**
Any route-wrapper path that accepts `HTTP.clustered(3, handler)` must preserve the same continuity/CLI truth behavior.

## Verification plan

### Minimum proof stack

1. **Type / compile rails**
   - add new tests for `HTTP.clustered(...)` typing and misuse
   - if route-wrapper lowering emits synthetic wrappers or IR markers, add focused codegen/unit rails there

2. **Live HTTP route proof**
   - add `compiler/meshc/tests/e2e_m047_s03.rs`
   - use real HTTP requests through a temp-built clustered app
   - do not accept compile-only proof for clustered routes

3. **Cluster truth proof**
   - reuse `meshc cluster continuity --json` / human output from the M047/S02 helpers
   - prove the wrapped route handler surfaces a real `declared_handler_runtime_name` and `replication_count`

4. **Explicit-count proof**
   - prove bare wrapper => `replication_count=2`
   - prove explicit wrapper count survives as `replication_count=3` and, if still unsupported at runtime, remains durably rejected rather than downgraded

5. **Retained-limit control**
   - keep `cargo test -p meshc --test e2e_stdlib e2e_m032_route_bare_handler_control -- --nocapture`
   - keep `cargo test -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture`
   - unless S03 intentionally broadens route closure support and updates that product contract in the same slice

### Good starting command set for execution planning

- `cargo test -p meshc --test e2e_stdlib e2e_m032_route_bare_handler_control -- --nocapture`
- `cargo test -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture`
- `cargo test -p meshc --test e2e_m047_s02 -- --nocapture`
- likely new targeted rails in:
  - `mesh-typeck`
  - `mesh-codegen`
  - `mesh-rt`
  - `compiler/meshc/tests/e2e_m047_s03.rs`

## Resume notes

- I stopped before reading deeper into `collect_source_cluster_declarations(...)`; from `main.rs` and the validated clustered export-surface code, it is already clear that current clustered planning is source-declaration driven and has no HTTP wrapper representation.
- The next honest step is not more repo mapping — it is an architectural choice between:
  1. **generated bare route shims** (recommended), or
  2. **widening route registration to `(fn_ptr, env_ptr)`**.
- That decision should happen at planning time first, because it determines whether S03 is a narrow route-wrapper slice or a broader HTTP route-closure/runtime ABI slice.