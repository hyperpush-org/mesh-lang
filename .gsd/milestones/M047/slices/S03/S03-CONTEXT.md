# M047 / S03 wrap-up note

## Status
- Verification is still red.
- `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` still fails because the target does not exist.
- No shippable S03 code landed in this wrap-up unit.
- I started a route-wrapper implementation, then reverted the partial edits so the tree is not left half-migrated.

## Concrete resume seam
The route-wrapper feature is still missing in three places at once:

1. **Typecheck acceptance**
   - `compiler/mesh-typeck/src/infer.rs::infer_call(...)`
   - Minimal acceptance path: recognize `HTTP.clustered(handler)` and `HTTP.clustered(N, handler)` as wrapper-shaped calls that typecheck to the wrapped handler type `Fn(Request) -> Response`.
   - Do **not** rely on generic builtin lowering alone; the wrapper is a compiler-known route marker.

2. **Route lowering**
   - `compiler/mesh-codegen/src/mir/lower.rs::lower_call_expr(...)`
   - Intercept `HTTP.route(...)` / `HTTP.on_get(...)` / `HTTP.on_post(...)` / `HTTP.on_put(...)` / `HTTP.on_delete(...)` when the handler argument is a wrapper call.
   - That is the clean seam for deriving a deterministic runtime name, preserving default vs explicit replication count, and routing the call to clustered-route-specific runtime intrinsics instead of the plain `mesh_http_route_*` path.

3. **Runtime HTTP execution**
   - `compiler/mesh-rt/src/http/router.rs`
   - `compiler/mesh-rt/src/http/server.rs`
   - The HTTP server thread cannot simply reuse `mesh_node_spawn` / `submit_declared_work(...)` as-is, because the remote-spawn path requires actor context.
   - The safest first truthful implementation is a **runtime-owned clustered route entry** on the router plus a server-side continuity submission/completion path for the matched request.
   - That keeps the route boundary visible in continuity/diagnostic surfaces without pretending the existing actor-only spawn path already works for HTTP threads.

## Design notes from the aborted attempt
- The least invasive architecture is:
  - add clustered-route metadata to router entries,
  - add dedicated runtime intrinsics like `mesh_http_route_clustered_*`,
  - let lowering swap to those intrinsics only when the handler argument is `HTTP.clustered(...)`.
- This keeps plain route handlers on the existing direct `call_handler(...)` path.
- It also avoids broadening generic route-closure support by accident; the M032 route-closure control rails still need to stay red/unchanged unless the runtime ABI work is deliberate.

## Immediate next steps
1. Add the typechecker wrapper acceptance in `infer_call(...)`.
2. Add clustered-route runtime intrinsic declarations in codegen.
3. Add router metadata + server dispatch support in `mesh-rt`.
4. Add `compiler/meshc/tests/e2e_m047_s03.rs` with:
   - one live clustered-route happy-path rail,
   - one explicit-count truth rail,
   - continuity/diagnostic assertions,
   - replay of the M032 route-closure control behavior.

## Why I stopped here
The partial implementation touched both compiler and runtime seams and would have left the tree in a broken state. I reverted those edits instead of shipping an unverified half-step.
