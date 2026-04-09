---
estimated_steps: 3
estimated_files: 3
skills_used: []
---

# T01: Rebased the Todo scaffold onto explicit HTTP.clustered(1, ...) read routes with typed handlers and updated the fast contract rails.

Why: Rebaseline the generated Todo starter so the public scaffold no longer treats `HTTP.clustered(...)` as unshipped while keeping the route-free `@cluster` work surface canonical.
Do: Update the scaffold generator to wrap selected read routes with `HTTP.clustered(1, ...)`, add explicit `(Request) -> Response` signatures to wrapped handlers, and refresh generated README/source-contract expectations in the fast scaffold rails without clustering mutating routes.
Done when: `meshc init --template todo-api` emits typed clustered read routes, the generated README explains the truthful starter choice, and the fast scaffold rails assert the new contract.

## Inputs

- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`

## Expected Output

- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`

## Verification

cargo test -p mesh-pkg m047_s05 -- --nocapture && cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/mesh-pkg/src/scaffold.rs` route/README generator | fail the scaffold/unit rails and do not ship mixed local/wrapper wording | N/A | treat malformed generated router text or README claims as contract failures, not as fallback to ordinary local routes |
| `HTTP.clustered(1, ...)` handler type contract | require explicit `(Request) -> Response` signatures or fail compile/test rails | N/A | malformed handler signatures are a hard failure, not something the scaffold should guess around |
| fast scaffold expectations in `tooling_e2e` / `e2e_m047_s05` | fail exact assertions until all stale "not shipped" markers are removed | N/A | contradictory expectations are a red drift signal, not optional cleanup |

## Load Profile

- **Shared resources**: scaffold text templates, generated router/readme snippets, and fast content-assertion rails.
- **Per-operation cost**: one scaffold generation plus file-content assertions; runtime cost is trivial.
- **10x breakpoint**: duplicated stale snippets drift before any throughput concern appears.

## Negative Tests

- **Malformed inputs**: generated read handlers missing `Request`/`Response` annotations, missing explicit count `1`, or wrapper syntax appearing on non-read routes.
- **Error paths**: stale "HTTP.clustered(...) is still not shipped" wording survives in the generated README or the generator emits mixed wrapper/local route shapes.
- **Boundary conditions**: `GET /health` stays local, selected `GET /todos` routes are wrapped, and write routes continue to use the local actor-backed limiter path.
