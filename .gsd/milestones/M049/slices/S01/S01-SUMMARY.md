---
id: S01
parent: M049
milestone: M049
provides:
  - A real `meshc init --template todo-api --db postgres <name>` scaffold path with migrations, pool-backed runtime startup, helper-based CRUD, `.env.example`, and Docker packaging guidance.
  - A reusable M049 Postgres starter harness (`compiler/meshc/tests/e2e_m049_s01.rs` plus `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`) that proves generate/migrate/test/build/boot/CRUD/error behavior without leaking `DATABASE_URL`.
  - A typed DB-selection seam that preserves the existing SQLite starter while giving S02/S03/S04/S05 a stable place to add SQLite-local parity, generated `/examples`, and proof-surface retirement.
requires:
  []
affects:
  - S02
  - S03
  - S04
  - S05
key_files:
  - compiler/meshc/src/main.rs
  - compiler/mesh-pkg/src/lib.rs
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/tests/tooling_e2e.rs
  - compiler/meshc/tests/e2e_m049_s01.rs
  - compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs
  - compiler/meshc/tests/support/mod.rs
key_decisions:
  - Use a typed `TodoApiDatabase` seam in `mesh-pkg`, keep `scaffold_todo_api_project(...)` as the SQLite-default wrapper, and make invalid `--db` / `--template` / `--clustered` combinations fail before project creation.
  - Keep the Postgres starter migration-first via `meshc migrate . up` and keep `DATABASE_URL` out of the generated runtime registry and `/health` surface while still exposing usable bootstrap/config diagnostics.
  - Validate config and connect the PostgreSQL pool before calling `Node.start_from_env()` so missing-env and connection-failure paths fail closed instead of hanging on runtime keepalive actors.
patterns_established:
  - Additive dual-db scaffolding should keep the existing SQLite wrapper stable while new database-specific starter branches land behind a typed CLI/pkg selector.
  - Serious starter templates should keep schema creation in `migrations/` and use runtime `/health` plus explicit log lines for diagnostics instead of startup DDL or secret-bearing config echoes.
  - Live scaffold proof should generate the app into a temp workspace, provision a disposable backing service, exercise only the public runtime/HTTP surface, and retain redacted artifacts for both happy-path and failure-path assertions.
observability_surfaces:
  - `GET /health` returns `status`, `db_backend=postgres`, `migration_strategy=meshc migrate`, `clustered_handler=Work.sync_todos`, and the configured rate-limit window/max values.
  - The generated starter logs explicit config, bootstrap, pool-ready, registry-ready, runtime-ready, HTTP-start, config-error, and connect-failure lines without echoing `DATABASE_URL`.
  - The live M049 harness archives redacted build, migration, runtime, and raw HTTP artifacts for the happy path plus the unmigrated-database and missing-env failure paths.
  - Operational readiness: health signal = `GET /health` plus the `Runtime ready` / `HTTP server starting` log lines; failure signal = explicit `[todo-api] Config error: Missing required environment variable DATABASE_URL`, `[todo-api] PostgreSQL connect failed: ...`, or `GET /todos` returning a 500 JSON error on an unmigrated DB; recovery procedure = supply a valid `DATABASE_URL`, run `meshc migrate . up`, then restart the binary; monitoring gap = there is still no later-slice clustered deploy/operator proof for this starter.
drill_down_paths:
  - .gsd/milestones/M049/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M049/slices/S01/tasks/T02-SUMMARY.md
  - .gsd/milestones/M049/slices/S01/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-02T22:06:42.973Z
blocker_discovered: false
---

# S01: Postgres starter contract

**S01 shipped the typed dual-db init seam, a migration-first Postgres `todo-api` starter, and a live redacted runtime proof while keeping the legacy SQLite starter and retained M048 contract green.**

## What Happened

S01 turned `meshc init --template todo-api` into a typed dual-database surface instead of a single stale SQLite-only path. T01 added a typed `TodoApiDatabase` seam across `meshc` and `mesh-pkg`, preserved `scaffold_todo_api_project(...)` as the SQLite-default wrapper, and made invalid `--db` / `--template` / `--clustered` combinations fail before project creation. T02 filled the Postgres branch with a real starter: `config.mpl`, `.env.example`, migration-owned `migrations/`, pool-backed startup, helper-based storage using `Repo` / `Query` / `Pg`, a Postgres-aware README/Dockerfile, and a `/health` surface that exposes backend/migration/cluster-handler/rate-limit truth without leaking `DATABASE_URL`.

T03 completed the story with a dedicated live-runtime acceptance rail in `compiler/meshc/tests/e2e_m049_s01.rs` plus `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`. The harness generates an isolated starter, provisions per-run databases, runs `meshc migrate`, `meshc test`, and `meshc build`, boots the compiled binary, exercises `/health` plus CRUD/error paths, and archives redacted build/runtime/HTTP artifacts. Closeout exposed one last dishonest seam: the generated Postgres `main.mpl` logged a missing `DATABASE_URL` error but still hung because `Node.start_from_env()` had already spawned keepalive/startup actors. The scaffold now validates config and opens the pool before runtime bootstrap, preserving the happy-path bootstrap logs while making fail-closed startup behavior honest.

The slice therefore delivers a real `meshc init --template todo-api --db postgres <name>` path, keeps the existing SQLite starter working until S02 lands the explicit local-first story, and gives downstream slices a reusable generator/harness seam instead of another hand-maintained proof app.

## Verification

Verified the slice with the full task matrix: `cargo test -p mesh-pkg m047_s05_scaffold_todo_api_project_ -- --nocapture`, `cargo test -p mesh-pkg m049_s01_postgres_scaffold_ -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_todo_template_db_ -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_todo_template_postgres_ -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`, `set -a && . .tmp/m049-s01/local-postgres/connection.env && set +a && cargo test -p meshc --test e2e_m049_s01 -- --nocapture`, and `node --test scripts/tests/verify-m048-s05-contract.test.mjs`. The live rail passed all three runtime scenarios: migrate/test/build/boot/CRUD, unmigrated-database 500 JSON error, and missing-`DATABASE_URL` fail-closed shutdown. The retained M048 contract stayed green after the starter/runtime wording change.

## Requirements Advanced

- R122 — Established the Postgres half of the honest serious-starter split by shipping and live-testing the migration-first Postgres scaffold while leaving SQLite’s explicit local-first story to later slices.

## Requirements Validated

- R115 — Validated by `cargo test -p meshc --test tooling_e2e test_init_todo_template_db_ -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_todo_template_postgres_ -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`, `cargo test -p mesh-pkg m049_s01_postgres_scaffold_ -- --nocapture`, `cargo test -p mesh-pkg m047_s05_scaffold_todo_api_project_ -- --nocapture`, `set -a && . .tmp/m049-s01/local-postgres/connection.env && set +a && cargo test -p meshc --test e2e_m049_s01 -- --nocapture`, and `node --test scripts/tests/verify-m048-s05-contract.test.mjs`.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Closeout needed one extra scaffold fix beyond the task-plan happy path: the generated Postgres `main.mpl` still called `Node.start_from_env()` before validating `DATABASE_URL`, which left runtime keepalive actors alive after the config error and made the missing-env rail hang instead of fail closed. The slice also had to use a fresh loopback-only local Docker Postgres env file because the repo-root `.env` in this session was present but not shell-loadable.

## Known Limitations

S01 proves the Postgres starter locally through scaffold generation, migrations, runtime behavior, and explicit error surfaces, but it does not yet provide the later clustered deploy proof promised for the serious Postgres path. SQLite-local parity, generated `/examples`, and public proof-app retirement still belong to later M049 slices. The GSD requirements DB also rejected `R115` as `not found`, so the visible requirement truth for this slice currently lives in the checked-in `.gsd/REQUIREMENTS.md` plus decision D341 rather than in a rendered DB update.

## Follow-ups

S02 should land the SQLite-local starter contract on the same typed database seam. S03 should generate checked-in `/examples/todo-postgres` and `/examples/todo-sqlite` from scaffold output instead of maintaining another drifting app surface. S04/S05 should retire the old proof-app onboarding surfaces and assemble one retained verifier over the dual-db scaffold/example story. The requirements DB mismatch for `R115` also still needs repair so validated M049 requirements can be rendered from the DB instead of living only in the checked-in file plus decisions.

## Files Created/Modified

- `compiler/meshc/src/main.rs` — Added typed `--db` init parsing/validation and routed `todo-api` generation through the database-aware scaffold seam.
- `compiler/mesh-pkg/src/lib.rs` — Exported the typed todo database selector and database-aware scaffold entrypoint from `mesh-pkg`.
- `compiler/mesh-pkg/src/scaffold.rs` — Added the real Postgres `todo-api` starter files, migration-first runtime/storage templates, and the fail-closed boot ordering fix.
- `compiler/meshc/tests/tooling_e2e.rs` — Added CLI-level init/scaffold regression coverage for SQLite default, explicit `--db`, and invalid flag combinations.
- `compiler/meshc/tests/e2e_m049_s01.rs` — Added the slice-owned live Postgres starter acceptance rail covering happy path, unmigrated DB, and missing-`DATABASE_URL` behavior.
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` — Added the shared Postgres starter harness for project generation, per-run databases, process control, HTTP probes, and redacted artifact capture.
- `compiler/meshc/tests/support/mod.rs` — Registered the new M049 Postgres starter support helper for the `meshc` integration-test suite.
- `.gsd/PROJECT.md` — Recorded the new slice state so future agents can see that the Postgres starter contract is already green.
- `.gsd/KNOWLEDGE.md` — Captured the fail-closed clustered-starter boot-order lesson and the M049 requirements DB mismatch.
