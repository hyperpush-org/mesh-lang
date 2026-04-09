# M053 / S02 Research — Generated Postgres starter proves clustered failover truth

**Date:** 2026-04-04  
**Status:** Ready for planning

## Summary

S02 directly owns **R122** and supports **R115**, **R116**, **R117**, and **R120**. The core job is not new docs or hosted wiring; it is extending S01’s generated-starter staged deploy proof into a **two-node clustered replay with visible owner-loss/failover truth** while keeping SQLite local-only and keeping the public starter contract portable-first.

The repo already has the right building blocks:

- **S01** proved the generated Postgres starter can stage a deploy bundle outside the source tree and answer `meshc cluster status|continuity|diagnostics` from that running staged binary.
- **Route-free clustered rails** already know how to boot two nodes, poll runtime-owned CLI surfaces, and retain bundled artifacts.
- **Todo/HTTP clustered rails** already know how to assert clustered route continuity and optional same-image/Docker execution.

What is still missing is a helper and e2e rail that combine those pieces for the **generated Postgres starter itself**.

The main technical risk is timing: the current starter’s clustered surfaces are fast. `work.mpl` is just `1 + 1`, and `GET /todos` is a quick shared-DB read. A destructive owner-loss proof needs a stable pending window. Per repo knowledge, the truthful seam for this class of proof is the **runtime-owned delay window** used for startup failover, not re-introducing app-owned `Timer.sleep(...)` into the starter source.

## Requirements Focus

### Directly owned

- **R122** — generated Postgres starter must get real clustered deploy/failover proof while SQLite stays explicitly local.

### Supported by this slice

- **R115** — keeps the dual-db split honest by proving serious clustered behavior only on Postgres.
- **R116** — keeps the proof generated-starter-first rather than reviving retained proof apps as the product surface.
- **R117** — keeps evaluator-facing docs bounded by proving the starter in code/tests instead of stuffing failover prose into the starter README.
- **R120** — strengthens one coherent public story by making the Postgres starter’s serious path materially true.

## Skills Discovered

Already-installed skills directly relevant to this slice:

- **postgresql-database-engineering** — useful constraint: S02 is shared-DB runtime failover proof, not PostgreSQL replication/HA work.
- **flyio-cli-public** — relevant as a boundary rule: prefer read-only Fly actions first and do not make S02 depend on live Fly mutation.
- **github-workflows** — relevant later for S03 hosted evidence wiring; not a primary S02 implementation surface.
- **multi-stage-dockerfile** — relevant only if S02 needs a same-image/container replay; keep runtime images copy-only if used.

No additional skill installs were needed. All core technologies for this slice already have installed skills.

## Implementation Landscape

### 1. Generated Postgres starter contract is already narrow and explicit

Primary files:

- `compiler/mesh-pkg/src/scaffold.rs`
- `examples/todo-postgres/main.mpl`
- `examples/todo-postgres/work.mpl`
- `examples/todo-postgres/api/router.mpl`
- `examples/todo-postgres/api/todos.mpl`
- `examples/todo-postgres/api/health.mpl`
- `examples/todo-postgres/storage/todos.mpl`
- `examples/todo-postgres/README.md`

What the starter actually is right now:

- `main.mpl` validates config, opens the PostgreSQL pool, then boots through `Node.start_from_env()` before serving HTTP.
- `work.mpl` declares only `@cluster pub fn sync_todos() -> Int do 1 + 1 end`.
- `api/router.mpl` clusters only the **read** routes:
  - `GET /todos` → `HTTP.clustered(1, handle_list_todos)`
  - `GET /todos/:id` → `HTTP.clustered(1, handle_get_todo)`
- `/health` and all mutating routes remain local.
- `api/health.mpl` reports `db_backend`, `migration_strategy`, `clustered_handler`, and rate-limit config, but **not** runtime authority truth.
- `storage/todos.mpl` uses shared Postgres CRUD via `Repo.*`, so two starter nodes can truthfully observe the same underlying state without any SQLite-style local persistence story.

Important constraint from `compiler/mesh-pkg/src/scaffold.rs` and its tests:

- the generated Postgres README is intentionally bounded
- it already describes the staged bundle and clustered reads
- it is **not** the place to dump Fly-first or failover-heavy prose
- existing generator tests in this file already pin the public wording tightly enough that S02 should avoid turning the starter README into a proof runbook

### 2. S01 already owns the starter deploy bundle and single-node operator seam

Primary files:

- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs`
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`
- `compiler/meshc/tests/e2e_m053_s01.rs`
- `scripts/verify-m053-s01.sh`

What exists now:

- `m053_todo_postgres_deploy.rs` wraps the generated starter path and already knows the two relevant runtime names:
  - startup runtime: `Work.sync_todos`
  - clustered route runtime: `Api.Todos.handle_list_todos`
- it stages the deploy bundle outside the repo tree, applies staged SQL, boots the staged binary, waits for `/health`, and captures single-node `meshc cluster` snapshots.
- it already reuses `m046_route_free` waiters for:
  - cluster status convergence
  - continuity list/record polling
  - diagnostics polling
- `m049_todo_postgres_scaffold.rs` already owns reusable pieces for:
  - `meshc init --template todo-api --db postgres`
  - isolated disposable Postgres DB creation
  - `meshc build` with `mesh-rt` prebuild
  - binary spawn/stop with redacted logs
  - raw HTTP request helpers
- `e2e_m053_s01.rs` is the authoritative single-node staged deploy proof.
- `scripts/verify-m053-s01.sh` is already the retained-wrapper surface S02 should replay rather than replace.

Implication:

S02 should extend the **same generated starter artifact path**. It should not pivot to `cluster-proof/`, Fly deploys, or a hand-curated fixture as its primary proof surface.

### 3. The reusable two-node cluster primitives already exist

Primary files:

- `compiler/meshc/tests/support/m046_route_free.rs`
- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `compiler/meshc/tests/e2e_m045_s02.rs`
- `compiler/meshc/tests/e2e_m046_s05.rs`

What each layer contributes:

#### `compiler/meshc/tests/support/m046_route_free.rs`

This is the generic cluster/runtime helper layer. It already provides:

- temp artifact roots and retained build metadata
- `meshc build` temp-output handling with fail-closed preflight
- route-free runtime spawn/stop helpers
- `meshc cluster status|continuity|diagnostics` polling helpers
- request-key diff helpers for continuity lists
- startup diagnostics waiters
- retained artifact writing conventions

This is the right base for any two-node Postgres starter proof that still wants runtime-owned CLI evidence.

#### `compiler/meshc/tests/support/m047_todo_scaffold.rs`

This is the route-based Todo helper layer. It already provides:

- Todo app runtime config + clustered env application
- health / CRUD / JSON snapshot helpers for route-based apps
- clustered route continuity helpers for Todo GET routes
- optional Dockerized `meshc cluster` queries from helper containers when host-published cluster ports are not the truthful operator surface
- same-image/container packaging helpers that preserve the "copy built artifact, don’t rebuild in runtime image" contract

This is the closest existing match to S02’s HTTP/read-route side.

#### `compiler/meshc/tests/e2e_m047_s05.rs`

This file is especially useful as a pattern source because it already proves:

- route-based Todo CRUD + `/health`
- clustered `GET /todos` continuity truth
- discovery of a new route request key after hitting the clustered GET route
- native and containerized route proof shapes

But it is still a **historical SQLite-backed fixture**. Use its mechanics, not its public status.

#### `compiler/meshc/tests/e2e_m045_s02.rs` and `compiler/meshc/tests/e2e_m046_s05.rs`

These are useful for the generated-app side:

- two-node startup membership convergence
- mirrored continuity record assertions across primary/standby
- diagnostics expectations (`startup_trigger`, `startup_dispatch_window`, `startup_completed`)
- retained artifact shapes for generated apps

But they are route-free clustered scaffolds, not Postgres starters with HTTP CRUD.

### 4. Existing verifiers already define the right wrapper pattern

Primary files:

- `scripts/verify-m053-s01.sh`
- `scripts/verify-m045-s02.sh`
- `scripts/verify-m043-s03.sh`

Common structure worth reusing:

- replay prerequisites explicitly
- run one authoritative e2e target
- copy only the **fresh** `.tmp/...` artifact dirs created by that replay
- publish `status.txt`, `current-phase.txt`, `phase-report.txt`, and `latest-proof-bundle.txt`
- fail closed on malformed retained-bundle shape

S02 should follow this style. It does **not** need a new verifier architecture.

## What Is Missing

### 1. No two-node generated Postgres starter helper exists yet

There is no helper today that boots **two instances of the staged Postgres starter** against one shared `DATABASE_URL`, then exposes both:

- route-based HTTP assertions
- runtime-owned `meshc cluster` assertions

`m053_todo_postgres_deploy.rs` is currently single-node. `m047_todo_scaffold.rs` is route-aware but SQLite/fixture-oriented. S02 needs the two merged.

### 2. No starter-owned destructive failover rail exists yet

There is currently no `e2e_m053_s02.rs`-style test target and no `verify-m053-s02.sh` wrapper.

### 3. The starter’s clustered surfaces are too fast for naive kill timing

This is the main technical risk.

Current fast surfaces:

- `work.mpl` startup handler: `1 + 1`
- `GET /todos` clustered read: simple shared-DB query

That means a kill-after-submit proof can easily miss the pending window unless the runtime or harness deliberately holds the request open long enough.

Per repo knowledge, the stable direction here is:

- **do not** add package-owned delay back into `work.mpl`
- **do not** turn the starter into a custom failover fixture
- use the existing **runtime-owned delay seam** for startup-work failover when a pending mirrored record is required

That makes startup continuity the best candidate for destructive failover proof, while regular HTTP CRUD and clustered GET routes remain the product-facing traffic proof.

## Recommendation

### 1. Keep S02 starter-owned and staged-bundle-first

Build the proof around the exact S01 handoff:

- generate fresh Postgres starter
- stage deploy bundle outside repo tree
- apply staged SQL artifact
- boot **two copies of the same staged binary** with shared `DATABASE_URL`
- use the same starter-owned scripts/artifacts as the contract

This matches R116 and avoids slipping back into proof-app-first validation.

### 2. Split S02 into two proof surfaces inside one e2e rail

#### A. HTTP/shared-state truth

Prove the running clustered starter can:

- boot both nodes against shared Postgres
- accept local write traffic on a live node
- serve clustered read traffic through `HTTP.clustered(1, ...)`
- retain route continuity records for `Api.Todos.handle_list_todos`
- keep serving shared-state reads after the failover event

Because storage is Postgres-backed, this is a real shared-state proof without implying database failover.

#### B. Failover/runtime truth

Use runtime-owned startup continuity as the destructive proof seam:

- hold the startup request in a stable pending/mirrored window via the existing runtime-owned delay seam
- kill the owner node during that window
- assert owner-loss / recovery / fenced-rejoin truth through `meshc cluster continuity` and `meshc cluster diagnostics`
- assert surviving-node behavior and stale-primary behavior explicitly

This avoids having to mutate the starter source into a slow custom route or sleeper fixture.

### 3. Prefer host-native staged replay before Docker

The slice title asks for production-like clustered failover truth, not a Docker-only contract.

Given S01 already stages a portable deploy bundle, the least-wrong S02 path is:

- **first:** two staged host processes + shared Postgres + host `meshc cluster` CLI
- **only if needed:** reuse `m047_todo_scaffold.rs` Docker helper patterns for same-image/container operator paths

This stays aligned with the portable public contract and respects the Fly skill’s boundary: use Fly/read-only later, not as the first implementation choice here.

### 4. Keep generated README changes minimal or zero in S02

Generated README/docs constraints are already tight in `scaffold.rs`.

S02 should focus on:

- helper code
- e2e rail
- retained verifier
- maybe bounded README contract assertions if absolutely necessary

It should **not** try to push full failover/Fly wording into `examples/todo-postgres/README.md`. That belongs to the later docs/reference slice.

## Natural Task Seams

### Task seam 1 — Two-node Postgres staged helper

Likely files:

- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs`
- possibly a new sibling helper such as `compiler/meshc/tests/support/m053_todo_postgres_failover.rs`
- maybe small shared reuse from `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`

Goal:

- extend the staged Postgres starter support from single-node to two-node
- spawn primary + standby with shared `DATABASE_URL`, shared cookie/seed, distinct node names/ports
- provide shared helpers for:
  - cluster membership convergence
  - route request-key discovery
  - continuity record polling
  - diagnostics polling
  - HTTP CRUD against either live node
  - retained artifact writing

### Task seam 2 — Authoritative S02 Rust e2e rail

Likely file:

- `compiler/meshc/tests/e2e_m053_s02.rs`

Goal:

- generate starter
- stage bundle
- migrate DB
- boot two nodes from staged artifact
- seed shared state through real routes
- prove clustered GET continuity
- trigger owner loss during pending startup continuity
- assert surviving runtime truth, HTTP truth, and stale-primary/fenced-rejoin truth
- retain a clean artifact bundle

### Task seam 3 — Retained wrapper/verifier

Likely file:

- `scripts/verify-m053-s02.sh`

Goal:

- replay `bash scripts/verify-m053-s01.sh` first
- run `cargo test -p meshc --test e2e_m053_s02 -- --nocapture`
- copy fresh `.tmp/m053-s02/...` artifacts into retained proof bundle
- publish bundle pointer + phase markers
- fail closed on malformed retained bundle shape

## Verification Targets

Most likely authoritative commands:

```bash
DATABASE_URL=postgres://... cargo test -p meshc --test e2e_m053_s02 -- --nocapture
DATABASE_URL=postgres://... bash scripts/verify-m053-s02.sh
```

The shell wrapper should probably replay at least:

```bash
bash scripts/verify-m053-s01.sh
DATABASE_URL=postgres://... cargo test -p meshc --test e2e_m053_s02 -- --nocapture
```

Expected retained markers should mirror the repo’s normal verifier shape:

- `.tmp/m053-s02/verify/status.txt`
- `.tmp/m053-s02/verify/current-phase.txt`
- `.tmp/m053-s02/verify/phase-report.txt`
- `.tmp/m053-s02/verify/full-contract.log`
- `.tmp/m053-s02/verify/latest-proof-bundle.txt`

Expected retained evidence inside the fresh proof bundle should include:

- two node stdout/stderr/combined logs
- cluster status JSON/log for both nodes before and after failover
- continuity list + single-record JSON/log snapshots
- diagnostics JSON/log snapshots
- HTTP snapshots before and after failover (`/health`, CRUD, clustered `GET /todos`)
- any chosen request-key / selected-scenario metadata

## Forward Intelligence / Risks

### 1. Do not turn this into Postgres HA work

The shared store is already Postgres-backed. S02 proves **Mesh runtime continuity and ownership over shared DB state**, not database replication or PostgreSQL failover. Keep those claims separate.

This matches the PostgreSQL skill guidance: replication/HA is a distinct subsystem. Don’t overclaim it because two app nodes share one DB.

### 2. `/health` is not the failover truth surface

`examples/todo-postgres/api/health.mpl` only reports app readiness/config and the clustered handler name. It does not report role/epoch/replica state.

For failover truth, the authoritative surfaces remain:

- `meshc cluster status`
- `meshc cluster continuity`
- `meshc cluster diagnostics`

### 3. The pending window is the slice’s hard problem

Without a deliberate pending window, the owner can finish before the destructive step lands.

Use the existing runtime-owned seam; do **not** put `Timer.sleep(...)` back into starter code. The generated starter should stay clean and source-first.

### 4. Keep public wording bounded until S04

`compiler/mesh-pkg/src/scaffold.rs` already embeds and tests the staged deploy contract. It is not currently shaped as a failover narrative. That is fine.

S02’s proof should make the starter more truthful in code and retained evidence first. Public wording alignment belongs later.

### 5. Same-image Docker proof remains the fallback reference, not the default plan

If host-native staged two-node replay proves unstable or insufficient, the best next reference point is the older same-image destructive rail and its verifier bundle shape:

- `compiler/meshc/tests/e2e_m043_s03.rs`
- `scripts/verify-m043-s03.sh`

But those are fallback mechanics, not the public/generated-starter contract.

## Resume Note

I stopped before reopening the full destructive failover bodies in:

- `compiler/meshc/tests/e2e_m046_s03.rs`
- `compiler/meshc/tests/e2e_m043_s03.rs`

If the planner needs the exact request-selection or stale-primary assertion sequence, those are the next files to read first. Everything else needed for planning the slice is already identified above.