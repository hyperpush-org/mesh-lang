# M028 / S01 — Research

**Date:** 2026-03-23

## Summary

This slice should **not** promote `mesher/` directly into the canonical backend proof path. `mesher/` is the strongest in-repo donor for patterns, but it is already a broad product app: 35 `.mpl` files, ~4.7k lines of Mesh code, WebSocket + clustering + alerting + retention + frontend, and checked-in build artifacts (`mesher/mesher`, `mesher/mesher.ll`, `mesher/output`). It proves the language is already being dogfooded, but it is too incidental and too wide to serve as the milestone’s boring, auditable backend baseline.

The best S01 outcome is a **new top-level reference backend package** (for example `reference-backend/`) that reuses Mesher’s real patterns but narrows the domain to one honest loop: **HTTP endpoint → Postgres row → migration-managed schema → periodic background worker updates that row**. Following the `debug-like-expert` rule “VERIFY, DON’T ASSUME”, the current repo proves pieces in isolation (HTTP runtime, SQLite runtime, migration scaffold generation, Job/supervisor fixtures) but not one small integrated backend path. S01 should create that proof target; S02 can then harden it with automated end-to-end verification.

This slice directly supports the milestone-scoped contract from the roadmap/context (especially R001, R002, and R009, while setting up R003/R004/R005/R006/R008/R010). The repo-level `REQUIREMENTS.md` compact view is still dominated by older product requirements, so the planner should treat the M028 roadmap/context as the authoritative requirement source for this slice.

## Recommendation

Create a **new, minimal Postgres-first backend package** instead of extending Mesher in place.

Recommended shape:
- package root: `reference-backend/` (or similarly explicit top-level package)
- startup contract: `DATABASE_URL`, `PORT`, optional worker interval env var
- HTTP routes:
  - `GET /health`
  - `POST /jobs` (insert a pending record)
  - `GET /jobs/:id` (read current status)
- DB shape: one durable `jobs` table with status + timestamps
- background job shape: **timer-driven actor/service** that periodically finds pending work and marks it processed
- migrations: real `migrations/*.mpl` files using `Migration.*` plus raw SQL escape hatch only when needed

Why this approach:
- Postgres is the only honest golden path for **API + DB + migrations + jobs** today because `meshc migrate` is Postgres-only (`compiler/meshc/src/migrate.rs:252`, `:333`, `:397`).
- Mesher already shows the right integration order to copy: pool open → runtime DB prep → service/job startup → `HTTP.serve` (`mesher/main.mpl:84-157`).
- Mesher’s timer-recursive actors are the repo’s real background-job idiom; `Job.async` exists, but it is a one-shot helper, not the best canonical “backend jobs” story (`mesher/ingestion/pipeline.mpl:90`, `:118`, `:227`; `compiler/meshc/tests/e2e_concurrency_stdlib.rs:200`).
- Keeping the reference backend separate avoids dragging S01 into Mesher’s clustering, WS, alerting, retention, and frontend scope.

Also follow the `test` skill rule “MATCH EXISTING PATTERNS”: reuse the existing Rust compile/run harnesses in `compiler/meshc/tests/*` instead of inventing a new verification stack. S01 should define the package and canonical commands; S02 should promote that package into deeper Rust e2e coverage.

## Implementation Landscape

### Key Files

- `mesher/main.mpl` — best donor for the real startup order: open pool, create runtime partitions, start services/pipeline, then serve HTTP (`mesher/main.mpl:84-157`). Do **not** use it as the canonical reference backend directly; it is too broad.
- `mesher/ingestion/pipeline.mpl` — best donor for long-running background work patterns: timer-recursive actors, process registration, and service startup (`mesher/ingestion/pipeline.mpl:81-95`, `:227-235`, `:393-440`). Also shows wiring debt to avoid copying blindly.
- `mesher/services/writer.mpl` — clearest batch-writer/background-service example, including the `flush_ticker` actor (`mesher/services/writer.mpl:101-145`). Important: `flush_ticker` is defined here but not spawned from `start_pipeline`.
- `mesher/storage/schema.mpl` — shows that some DB boot work currently happens at runtime after migrations (`mesher/storage/schema.mpl:1-36`). Useful if the reference backend needs startup prep distinct from schema migration.
- `mesher/migrations/20260216120000_create_initial_schema.mpl` — strongest real migration example. Demonstrates the actual seam between `Migration.create_table` and raw `Pool.execute(...)` for Postgres-specific DDL (`mesher/migrations/20260216120000_create_initial_schema.mpl:5-133`).
- `mesher/migrations/20260226000000_seed_default_org.mpl` — best example of a seed migration plus a manual smoke path (`mesher/migrations/20260226000000_seed_default_org.mpl:1-49`).
- `compiler/meshc/src/main.rs` — authoritative CLI contract. `meshc build` expects a **project directory containing `main.mpl`**, not a single file (`compiler/meshc/src/main.rs:40-133`, `:307-344`).
- `compiler/meshc/src/migrate.rs` — authoritative migration contract: `migrations/` discovery, `_mesh_migrations` tracking table, `DATABASE_URL` requirement, synthetic temp-project execution (`compiler/meshc/src/migrate.rs:36-68`, `:74`, `:107-145`, `:252-447`).
- `compiler/mesh-rt/src/db/migration.rs` — confirms the migration DSL is intentionally limited and depends on `mesh_pool_execute` for actual DDL execution (`compiler/mesh-rt/src/db/migration.rs:1-220`).
- `compiler/mesh-rt/src/http/server.rs` — confirms the HTTP runtime is a real blocking server with actor-per-connection handling and crash isolation (`compiler/mesh-rt/src/http/server.rs:462-590`).
- `compiler/mesh-rt/src/actor/job.rs` — documents `Job.async` / `Job.await`; useful for isolated async work, but weaker than timer actors as the canonical background-worker story (`compiler/mesh-rt/src/actor/job.rs:1-171`).
- `compiler/meshc/tests/e2e_stdlib.rs` — current real HTTP and DB proof surface. `e2e_http_server_runtime` and `e2e_sqlite` already exercise compile → run → verify (`compiler/meshc/tests/e2e_stdlib.rs:774`, `:1664`, `:1786`).
- `compiler/meshc/tests/e2e_concurrency_stdlib.rs` — isolated `Job.async` proof (`compiler/meshc/tests/e2e_concurrency_stdlib.rs:200`).
- `compiler/meshc/tests/e2e_supervisors.rs` — isolated supervision proof (`compiler/meshc/tests/e2e_supervisors.rs:149`).
- `compiler/meshc/tests/e2e.rs` — contains reusable multi-file project helpers; best future home for on-disk reference-backend compile/run helpers (`compiler/meshc/tests/e2e.rs:1565`, `:1677`).
- `compiler/mesh-codegen/src/link.rs` — important verification constraint: compile/run e2e tests need a discoverable `libmesh_rt.a` (`compiler/mesh-codegen/src/link.rs:76-108`).
- `README.md` — currently not authoritative for the canonical backend path. It still shows a toy web server and an outdated `meshc build hello.mpl` command (`README.md:83`, `:87-103`).
- `reference-backend/mesh.toml` — **new** file to create for the canonical package shape. Even though `meshc build` only requires `main.mpl`, `meshc deps` expects `mesh.toml` (`compiler/meshc/src/main.rs:633-656`).
- `reference-backend/main.mpl` — **new** startup entrypoint. Keep this much smaller than `mesher/main.mpl`: env-driven pool open, worker start, route registration, `HTTP.serve`.
- `reference-backend/migrations/*.mpl` — **new** real migrations for the reference schema.
- `reference-backend/api/*.mpl`, `reference-backend/storage/*.mpl`, `reference-backend/jobs/*.mpl` — **new** narrow modules for handlers, queries, and the periodic worker.

### Build Order

1. **Lock the package shape and startup contract first.**
   - Create the new top-level package and decide the exact env contract (`DATABASE_URL`, `PORT`, worker interval).
   - Include `mesh.toml` even if S01 does not need dependencies yet; this keeps the package compatible with `meshc init` / `meshc deps` expectations.

2. **Define the minimal persisted record and migrations before HTTP.**
   - Recommended record: a `jobs` table with `id`, `payload`, `status`, `attempts`, `created_at`, `processed_at`.
   - This gives the planner one durable shape that both HTTP and the worker share.
   - Start with plain Postgres queries / `Pool.execute` / `Repo.query_raw`; do not spend S01 on ORM breadth.

3. **Wire the HTTP path second.**
   - Add `/health` plus one create/read flow (`POST /jobs`, `GET /jobs/:id`).
   - This is the smallest honest DB-backed API surface.

4. **Wire the periodic worker third.**
   - Use Mesher’s timer-recursive actor pattern, not a synthetic one-shot demo.
   - The worker should make an observable DB state transition (for example `pending -> processed`).
   - This is what turns the package into a real “API + DB + migrations + background jobs” backend instead of just CRUD plus a toy async helper.

5. **Only after the package exists, define canonical commands and smoke expectations.**
   - S01 should produce the exact build/run/migrate/smoke commands.
   - S02 can then turn those commands into stronger automated proof.

### Verification Approach

For S01, verify the **real command contract**, not just file existence.

Current repo signals already confirmed during research:
- `cargo test -p meshc e2e_migrate_generate_creates_file -- --nocapture` **passes**.
- `cargo build -p mesh-rt && cargo test -p meshc e2e_http_server_runtime -- --nocapture` **passes**.
- `cargo build -p mesh-rt && cargo test -p meshc e2e_job_async_await -- --nocapture` still **fails** in the runtime step with `binary execution failed with exit code None` and empty stdout/stderr, so the isolated `Job.async` proof is currently weaker than the HTTP + migration-scaffold proof.
- Without a prebuilt runtime static library, compile/run e2e tests can fail with `Could not locate libmesh_rt.a` (`compiler/mesh-codegen/src/link.rs:100-108`).

Recommended S01 smoke contract for the new package:

```bash
cargo build -p mesh-rt
cargo run -p meshc -- migrate status reference-backend
DATABASE_URL=postgres://... cargo run -p meshc -- migrate up reference-backend
cargo run -p meshc -- build reference-backend
DATABASE_URL=postgres://... PORT=18080 ./reference-backend/reference-backend
```

Observable checks for S01:
- binary builds successfully
- migrations are discovered and apply cleanly
- service starts and binds HTTP port
- `GET /health` returns 200
- `POST /jobs` creates a row
- worker updates that row without manual intervention

Best future automation home (for S02):
- extend `compiler/meshc/tests/e2e.rs` or `e2e_stdlib.rs` with an **on-disk package fixture** or copied project-dir helper, rather than encoding the reference backend as one giant inline string.

## Don't Hand-Roll

| Problem | Existing Solution | Why Use It |
|---------|------------------|------------|
| Migration execution | `meshc migrate` in `compiler/meshc/src/migrate.rs` | It already owns discovery, tracking table management, and synthetic migration execution. |
| Periodic background work | Timer-recursive actors in `mesher/ingestion/pipeline.mpl` and `mesher/services/writer.mpl` | This is the repo’s real dogfooded backend-job pattern today. |
| Multi-file compile/run proof | `compiler/meshc/tests/e2e.rs:1677` (`compile_multifile_and_run`) and `compiler/meshc/src/test_runner.rs:1021` (`copy_project_sources_to_tmp`) | Reusing these keeps future proof aligned with existing compiler tests. |

## Constraints

- **Golden-path DB must be Postgres, not SQLite.** `meshc migrate` uses native Postgres calls and a `_mesh_migrations` table; there is no equivalent SQLite migration runner path (`compiler/meshc/src/migrate.rs:74`, `:252-447`).
- **`meshc build` is directory-based.** Any canonical commands or docs must pass a project directory containing `main.mpl`, not a single file path (`compiler/meshc/src/main.rs:307-344`; note the conflicting README example at `README.md:83`).
- **Migration DSL is not enough for all real schemas.** Mesher already needs raw SQL for extensions, partitioned tables, and advanced indexes (`mesher/migrations/20260216120000_create_initial_schema.mpl:6`, `:62`, `:91-123`).
- **`mesh.toml` is optional for build but required for dependency workflows.** A canonical backend should include it even if S01 has no dependencies (`compiler/meshc/src/main.rs:633-656`).
- **Repo requirement sources are currently split.** The global `REQUIREMENTS.md` is not the right truth source for M028 slice ownership; use the milestone roadmap/context.

## Common Pitfalls

- **Using `mesher/` as the reference backend** — it already includes frontend, clustering, WebSocket, alerting, retention, and other product concerns. It is a donor app, not the boring proof target.
- **Treating `Job.async` as the canonical background-job story** — it proves one-shot async work, but the dogfooded backend pattern is a long-running timer actor/service. Use that for S01.
- **Assuming the isolated `Job.async` e2e is already a stable trust anchor** — `cargo build -p mesh-rt && cargo test -p meshc e2e_job_async_await -- --nocapture` still failed during binary execution with no stdout/stderr, so planners should not lean on it as the primary golden-path proof.
- **Copying Mesher’s writer pattern without the ticker spawn** — `flush_ticker` exists in `mesher/services/writer.mpl:142-145`, but `start_pipeline` only starts `StorageWriter` and never spawns the ticker (`mesher/ingestion/pipeline.mpl:412`).
- **Assuming Mesher already has working restart orchestration** — `restart_all_services` exists (`mesher/ingestion/pipeline.mpl:355`) but is not wired from `health_checker`; do not copy that as “already proven supervision.”
- **Using compiler e2e tests without first building the runtime staticlib** — compile/run tests may fail until `libmesh_rt.a` exists in the expected target dir (`compiler/mesh-codegen/src/link.rs:76-108`).
- **Treating checked-in binaries as proof** — `mesher/mesher`, `mesher/mesher.ll`, and `mesher/output` exist in-tree; they are artifacts, not trustworthy verification.

## Open Risks

- The first real Postgres-backed reference package may expose gaps in `meshc migrate up/down/status` because current automated coverage is much stronger for scaffold generation than for live migration execution.
- If S01 tries to make the reference backend supervisor-heavy immediately, it will mix slice goals. The current repo proves supervision separately, but not yet as the canonical backend-job wiring.
- Choosing the wrong package location/name can create unnecessary downstream churn for S03/S04/S06. Decide the top-level path once and keep it stable.

## Skills Discovered

| Technology | Skill | Status |
|------------|-------|--------|
| Rust | `apollographql/skills@rust-best-practices` | available (`npx skills add apollographql/skills@rust-best-practices`) |
| PostgreSQL | `wshobson/agents@postgresql-table-design` | available (`npx skills add wshobson/agents@postgresql-table-design`) |
