# S05 Research — Simple clustered Todo scaffold

## Summary

S05 splits cleanly into two very different jobs:

1. **Scaffold/product work that is already feasible on today’s tree**: a SQLite-backed Todo API, actor-backed rate limiting, a registry/service layer, JSON handlers, and a complete Dockerfile.
2. **Route-level clustering, which is still blocked**: the roadmap wants `HTTP.clustered(...)` route syntax, but S03 closed with that wrapper still unimplemented, and the current runtime still executes matched HTTP handlers by direct `call_handler(...)` without consulting clustered declared-handler metadata.

So the key planning decision is not cosmetic. Either:

- **S05 includes the missing clustered-route implementation seam** before claiming clustered Todo routes, or
- **S05 is explicitly re-scoped** to ship a Todo scaffold with ordinary HTTP routes plus route-free `@cluster` work elsewhere in the app.

There is no honest cheap workaround in the current code: decorating a route handler with `@cluster` alone will not make `HTTP.on_get(...)` or `HTTP.on_post(...)` run clustered.

## Requirements Focus

Primary requirement ownership/support for this slice:

- **R104** — scaffold a simple SQLite Todo API with several routes, actors, rate limiting, and a complete Dockerfile.
- **R105** — keep the scaffold low-boilerplate and starter-grade rather than proof-app shaped.
- **R106** — keep the public clustered story coherent instead of introducing a fourth special scaffold contract.

Blocked dependency requirements that still matter here:

- **R100 / R101** — route-local clustering through `HTTP.clustered(...)`, with the route handler as the clustered boundary.
- **R099** — clustering remains a general function capability, not an HTTP-only hack.

Research conclusion: **R104/R105 are implementable now; the clustered-route part of R104 still depends on the missing S03 feature.**

## Skills Discovered

- **`multi-stage-dockerfile`** — already installed and directly relevant.
  - Relevant guidance used here: use a builder/runtime split, exact base image tags, minimal runtime contents, non-root runtime user, and a scaffold-owned `.dockerignore`.
- **`sqlite-database-expert`** — installed during this research.
  - Relevant guidance used here:
    - always use parameterized SQL (`?` placeholders) rather than string interpolation,
    - run schema/setup work transactionally,
    - enable SQLite safety/perf pragmas explicitly (`PRAGMA foreign_keys = ON`, WAL mode) because runtime open does not do it for you,
    - keep database access behind a stable seam instead of scattering raw SQL through handlers.

## Current Implementation Landscape

### 1. `meshc init --clustered` is currently a tiny route-free scaffold, not a server app

`compiler/mesh-pkg/src/scaffold.rs` only emits:

- `mesh.toml`
- `main.mpl`
- `work.mpl`
- `README.md`

and the generated clustered app is deliberately tiny and route-free:

- `main.mpl` only calls `Node.start_from_env()` and logs bootstrap status.
- `work.mpl` is only:
  - `@cluster pub fn execute_declared_work(_request_key :: String, _attempt_id :: String) -> Int do`
  - `1 + 1`
- README explicitly bans HTTP surfaces and `HTTP.clustered(...)`.

The current scaffold tests in both `compiler/mesh-pkg/src/scaffold.rs` and `compiler/meshc/tests/tooling_e2e.rs` assert:

- no `HTTP.serve(`
- no `/health`
- no `/work`
- no `HTTP.clustered(`
- no helper/manifest clustering leftovers

That means **replacing** `meshc init --clustered` with a Todo API would immediately break the S04 cutover contract and all the historical equal-surface rails that depend on it.

### 2. The CLI only has one clustered scaffold switch today

`compiler/meshc/src/main.rs` exposes:

- `meshc init <name>`
- `meshc init --clustered <name>`

There is no template flag or subcommand family yet.

If S05 wants to preserve the S04 route-free public contract **and** add a Todo API starter, the natural CLI seam is a new template selector or new scaffold mode, not an in-place mutation of the current `--clustered` behavior.

### 3. Route-local clustering is still genuinely absent

Evidence from the current tree:

- `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` still fails with `no test target named 'e2e_m047_s03'`.
- `rg -n "HTTP\.clustered|http_clustered" compiler/mesh-typeck/src compiler/mesh-codegen/src compiler/mesh-rt/src compiler/meshc/tests -g '*.rs'` only finds a negative assertion in `compiler/meshc/tests/tooling_e2e.rs`.
- `compiler/mesh-typeck/src/builtins.rs` only exposes:
  - `HTTP.router`
  - `HTTP.route`
  - `HTTP.on_get/on_post/on_put/on_delete`
  - `HTTP.use`
- `compiler/mesh-rt/src/http/server.rs` still matches a route and directly executes:
  - `call_handler(handler_fn, handler_env, req_ptr)`

That runtime path never consults declared-handler clustering metadata. So even if a handler function carried `@cluster`, the current HTTP server path would still call it locally like an ordinary handler.

### 4. The existing HTTP surface is already good enough for the non-clustered parts of the Todo scaffold

Current usable pieces:

- HTTP routing: `HTTP.router()`, `HTTP.on_get/post/put/delete(...)`
- middleware: `HTTP.use(...)`
- request accessors: `Request.body`, `Request.param`, `Request.query`, `Request.header`
- JSON output via `json { ... }`
- custom headers via `HTTP.response_with_headers(...)`

Useful examples already in-tree:

- `reference-backend/api/router.mpl` — small route table pattern
- `reference-backend/api/jobs.mpl` — body/param handling and JSON responses
- `tests/e2e/stdlib_http_middleware.mpl` — middleware shape
- `mesher/api/helpers.mpl` — `Request.query` / `Request.param` helper pattern

So the Todo API routing itself is straightforward **without** new HTTP runtime work.

### 5. SQLite support exists, but the scaffold must own a few things explicitly

Current Mesh SQLite surface exists today:

- `Sqlite.open`
- `Sqlite.close`
- `Sqlite.execute`
- `Sqlite.query`
- `Sqlite.begin`
- `Sqlite.commit`
- `Sqlite.rollback`

Evidence:

- types are in `compiler/mesh-typeck/src/builtins.rs` / `infer.rs`
- lowering/runtime intrinsics are in `compiler/mesh-codegen/src/mir/lower.rs` and `compiler/mesh-rt/src/db/sqlite.rs`
- runtime examples exist in `tests/e2e/stdlib_sqlite.mpl` and `tests/e2e/sqlite_*.mpl`

Important constraint from `compiler/mesh-rt/src/db/sqlite.rs`:

- `mesh_sqlite_open` just opens the DB; it does **not** automatically enable WAL, foreign keys, or other pragmas.

So a serious scaffold should execute setup statements like:

- `PRAGMA foreign_keys = ON`
- `PRAGMA journal_mode = WAL`

immediately after opening the DB.

Also, the current SQLite API is a **single connection handle**, not a pool. That makes a DB-owning service/registry seam attractive even before clustering enters the picture.

### 6. Actor/service patterns for rate limiting and stateful seams already exist

Good existing patterns to copy:

- `mesher/services/rate_limiter.mpl`
  - service state
  - `CheckLimit` call
  - reset ticker actor
- `reference-backend/runtime/registry.mpl`
  - boot-time registry service + `Process.register(...)`
- `mesher/api/helpers.mpl`
  - global/process lookup split helper pattern

This makes the Todo scaffold’s internal shape fairly obvious:

- one registry or app-state service to hold the SQLite handle and long-lived PIDs
- one rate-limiter service with reset ticker
- optionally one Todo service actor for writes/notifications/background work

### 7. Docker is the second real design decision after route clustering

The current `cluster-proof/Dockerfile` works because it builds from the **repo root** and has access to `compiler/meshc` + `compiler/mesh-rt`.

A generated scaffold project will not have that context. So its Dockerfile cannot honestly assume `compiler/` is present.

The only public install surface already proven in this repo is the installer pair documented in:

- `README.md`
- `website/docs/public/install.sh`
- `website/docs/public/install.ps1`

The Unix installer supports:

- `--version VERSION`
- env overrides like `MESH_INSTALL_RELEASE_API_URL` and `MESH_INSTALL_RELEASE_BASE_URL`

That gives S05 a viable Docker strategy:

- builder stage installs `meshc`/`meshpkg` via the public installer,
- builder stage runs `meshc build . --output /tmp/<binary>`,
- runtime stage copies only the built binary,
- scaffold Dockerfile exposes both the HTTP port and cluster port if needed,
- verifier can override installer URLs to point at staged local assets instead of the live network.

This aligns with the installed Docker skill and avoids shipping a repo-only Dockerfile that would not work for real users.

## Recommendation

### Command / product recommendation

**Do not replace the current `meshc init --clustered` route-free scaffold in S05.**

That path is already the validated S04 public contract and is hard-coded into:

- `compiler/mesh-pkg/src/scaffold.rs` tests
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/e2e_m046_s05.rs`
- README/docs pages that explicitly teach the route-free equal-surface story

Instead, add the Todo API as a **new scaffold mode/template**.

Most plausible seam:

- extend `meshc init` with a template flag or new mode,
- keep `--clustered` as today’s tiny route-free contract,
- add a Todo-oriented scaffold beside it.

If product direction insists that the Todo API should become the primary clustered starter, that is no longer an S05-only scaffold task — it becomes an S04 contract rewrite plus docs/test migration across multiple historical rails.

### Runtime / architecture recommendation

Assuming S05 stays a scaffold slice rather than a route-wrapper slice:

- build the Todo API on **ordinary HTTP routes**,
- use actors/services for rate limiting and DB access,
- keep one explicit route-free `@cluster` function if the app still needs to demonstrate general clustered execution without claiming route-local clustering,
- be explicit in scaffold docs that clustered **routes** depend on future `HTTP.clustered(...)` work unless that feature is landed in the same slice.

If the user/product insists on real clustered routes now, then **Task 1 of S05 must be the missing S03 feature**. The scaffold should not fake it.

### SQLite recommendation

Use a small explicit DB seam, not raw `Sqlite.*` calls scattered through handlers.

Recommended app shape:

- `db.mpl` — open DB, apply PRAGMAs, create schema if missing
- `todo_store.mpl` — parameterized CRUD helpers around `Sqlite.execute/query`
- `runtime/registry.mpl` or `app_state.mpl` — registered DB/service state
- `services/rate_limiter.mpl` — copied/adapted rate limiter pattern
- `api/router.mpl` + `api/todos.mpl` — HTTP handlers only

That keeps SQL isolated and makes the starter look intentional rather than like pasted proof code.

### Docker recommendation

Generate a scaffold-local multi-stage Dockerfile that:

- installs Mesh tooling through the public installer in the builder stage,
- supports version/URL override args for local proof,
- builds the Mesh app binary in the builder stage,
- runs as non-root in the runtime stage,
- includes a scaffold-local `.dockerignore`.

## Natural Seams for Planning

### 1. Decide the public scaffold command surface first

Files:

- `compiler/meshc/src/main.rs`
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`

Question to settle immediately:

- new template/mode alongside current `--clustered`, or
- destructive replacement of current `--clustered`

This decision affects every later task.

### 2. Refactor scaffold generation before writing the Todo app strings

`compiler/mesh-pkg/src/scaffold.rs` is currently manageable because it writes four tiny files. A Todo API scaffold will likely need many files.

Natural refactor seam:

- introduce helper(s) that return a file map / template set,
- keep the current route-free scaffold intact,
- add a second template set for the Todo scaffold.

Without that refactor, the file will turn into one long string-literal dump and become hard to verify.

### 3. Build the Todo app package shape independently of route clustering

Likely generated files:

- `mesh.toml`
- `main.mpl`
- `README.md`
- `Dockerfile`
- `.dockerignore`
- `api/router.mpl`
- `api/todos.mpl`
- `runtime/registry.mpl` or `app_state.mpl`
- `services/rate_limiter.mpl`
- `storage/db.mpl`
- `storage/todos.mpl`
- maybe `types/todo.mpl`

This is the main app-design task.

### 4. Add a scaffold-specific contract/proof rail

Likely files:

- `compiler/meshc/tests/e2e_m047_s05.rs` (new)
- maybe `compiler/meshc/tests/support/m047_todo_scaffold.rs` (new helper if runtime exercise is nontrivial)
- `scripts/verify-m047-s05.sh` (new)

This should prove:

- generation shape
- buildability of the generated app
- route/API behavior
- Docker build proof
- and, only if implemented, clustered-route truth

### 5. Keep docs-heavy public migration out of S05 unless required by the command-surface choice

If S05 adds a new template without replacing `meshc init --clustered`, docs churn can stay relatively small until S06.

If S05 replaces the primary clustered scaffold, then README/docs/test rails from S04 must move in the same slice.

## Verification Plan

### Minimum scaffold-generation proof

- new mesh-pkg unit test(s) for the new scaffold generator
- new tooling e2e for the new `meshc init ...` invocation
- assert generated file set and exact key markers

### Minimum app/runtime proof

At least one generated-project e2e should:

- run `meshc init <new-mode> ...`
- `meshc build` the generated project
- run the binary
- hit the Todo routes over HTTP
- verify CRUD + rate-limit behavior

### SQLite-specific proof

Because the runtime does not auto-configure SQLite, the scaffold proof should explicitly confirm:

- schema initialization succeeds on first boot,
- repeated boot does not destroy the DB,
- parameterized CRUD paths work,
- user-input SQL is passed via params, not string interpolation.

### Docker proof

The scaffold verifier should actually build the generated Docker image, not just grep the Dockerfile.

Recommended proof shape:

- stage local release assets / installer override endpoints,
- `docker build` the generated scaffold directory,
- run the resulting image with required env,
- hit at least one HTTP route.

### Blocker proof for clustered routes

If the slice still claims clustered route syntax, proof must include real `HTTP.clustered(...)` behavior. Static text is not enough.

Until that feature lands, the blocker signals remain:

- `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` -> missing target
- no `HTTP.clustered` symbols in typeck/codegen/runtime
- runtime HTTP server direct `call_handler(...)` execution path

## Concrete Risks / Watchpoints

- **Biggest risk:** accidentally turning S05 into a silent rewrite of the validated S04 `meshc init --clustered` contract.
- **Second risk:** generating a Dockerfile that only works inside this repo by assuming `compiler/` exists in the build context.
- **Third risk:** using SQLite directly from many handlers without a stable registry/service seam, which will make the starter noisy and hard to extend.
- **Fourth risk:** claiming clustered routes by documentation only while the runtime still routes directly to local handlers.
- **Fifth risk:** forgetting scaffold-local `.dockerignore`; the repo root `.dockerignore` is repo-specific and not suitable for a generated standalone project.

## Best Next Step for the Planner

Before decomposing implementation tasks, decide one question explicitly:

**Is S05 allowed to add a new Todo scaffold mode while preserving the existing route-free `meshc init --clustered` contract, or must it replace that contract?**

Everything else flows from that answer.
