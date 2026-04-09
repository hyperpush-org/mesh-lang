---
estimated_steps: 4
estimated_files: 3
skills_used:
  - rust-best-practices
  - postgresql-database-engineering
---

# T02: Generate the migration-first Postgres starter package

**Slice:** S01 — Postgres starter contract
**Milestone:** M049

## Description

Turn the new selector seam into a truthful Postgres starter. The generated project must use `DATABASE_URL`, pool-backed startup, `migrations/`, and helper-backed CRUD instead of copying the SQLite starter with renamed variables, and it must keep the README/Docker story honest about what S01 has and has not proven yet.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Postgres scaffold emitters inside `compiler/mesh-pkg/src/scaffold.rs` | Fail generation/tests rather than emitting a half-SQLite, half-Postgres starter. | N/A — template rendering is local string assembly. | Treat stale SQLite markers or missing migration/config files as test failures, not acceptable drift. |
| Public helper contract used by generated code (`Migration`, `Repo`, `Query`, `Pg`) | Keep the starter on the proven helper surface and stop before introducing raw-SQL or startup-DDL regressions. | N/A — compile/test-time contract only. | Update the generated files and assertions together so mismatched helper usage is caught by scaffold tests. |

## Load Profile

- **Shared resources**: scaffold file writer, generated project tree, and shared template strings inside `compiler/mesh-pkg/src/scaffold.rs`.
- **Per-operation cost**: one generated project directory plus content-assertion reads across the emitted files.
- **10x breakpoint**: template drift across duplicated builders appears before runtime load does, so common/shared emitters should stay centralized instead of splitting into independent SQLite/Postgres monoliths.

## Negative Tests

- **Malformed inputs**: missing `DATABASE_URL` helper text, invalid positive-int helper messages, and absent Postgres-only migration/config files in the generated tree.
- **Error paths**: generated README/Dockerfile/health output must not mention `TODO_DB_PATH`, `*.sqlite3`, startup schema creation, or already-proven deploy/failover behavior.
- **Boundary conditions**: the default SQLite starter remains unchanged while `--db postgres` adds the new package shape and contract text.

## Steps

1. Split shared vs Postgres-specific todo scaffold emitters in `compiler/mesh-pkg/src/scaffold.rs`, keeping the existing SQLite branch untouched for S02.
2. Generate the Postgres-specific starter files: `config.mpl`, `storage/todos.mpl`, `migrations/<timestamp>_create_todos.mpl`, `tests/config.test.mpl`, `.env.example`, and the Postgres README/Dockerfile contract.
3. Use `DATABASE_URL`, `Pool.open`, and `Migration` / `Repo` / `Query` / `Pg` helpers in the Postgres variant; do not reintroduce startup DDL or SQLite env names.
4. Add generator and `tooling_e2e` assertions that the Postgres file set and contract text are correct while the legacy SQLite file set remains green.

## Must-Haves

- [ ] The generated Postgres starter uses `DATABASE_URL`, pool startup, and migration-owned schema creation.
- [ ] The emitted Postgres project includes config helpers, a migration, package tests, Postgres storage, and an honest README/Docker/env sample.
- [ ] The Postgres template omits `TODO_DB_PATH`, `*.sqlite3`, `ensure_schema(...)`, and fake deploy/failover claims.
- [ ] The current SQLite scaffold text and tests remain unchanged except for the typed selector seam.

## Verification

- `cargo test -p mesh-pkg m049_s01_postgres_scaffold_ -- --nocapture`
- `cargo test -p mesh-pkg m047_s05_scaffold_todo_api_project_ -- --nocapture`
- `cargo test -p meshc --test tooling_e2e test_init_todo_template_postgres_ -- --nocapture`
- `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`

## Observability Impact

- Signals added/changed: the generated starter now has canonical config-validation messages, migration-first startup logging, and a safe `/health` contract that reports mode/status without leaking `DATABASE_URL`.
- How a future agent inspects this: scaffold the project, inspect generated `README.md`, `config.mpl`, `migrations/`, and rerun the new `m049_s01_postgres_scaffold_*` assertions.
- Failure state exposed: SQLite leftovers, missing migration files, or secret leakage fail static scaffold tests before runtime.

## Inputs

- `compiler/mesh-pkg/src/scaffold.rs` — current SQLite-only todo scaffold emitter to split.
- `compiler/mesh-pkg/src/lib.rs` — export surface for the DB-aware scaffold entrypoint.
- `compiler/meshc/tests/tooling_e2e.rs` — existing init tests plus the new Postgres project-shape checks.
- `reference-backend/config.mpl` — canonical env-key/error-message helper pattern.
- `reference-backend/tests/config.test.mpl` — starter-sized package test style to mirror.
- `mesher/migrations/20260216120000_create_initial_schema.mpl` — migration helper usage for PG-specific schema creation.
- `mesher/storage/queries.mpl` — `Repo` / `Query` helper usage to keep CRUD on the proven helper surface.

## Expected Output

- `compiler/mesh-pkg/src/scaffold.rs` — Postgres-specific scaffold emitters and honest generated file content.
- `compiler/mesh-pkg/src/lib.rs` — stable export surface for the typed todo scaffold entrypoint.
- `compiler/meshc/tests/tooling_e2e.rs` — static CLI/project-shape assertions for the generated Postgres starter and preserved SQLite starter.
