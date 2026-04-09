# S01: Postgres starter contract — UAT

**Milestone:** M049
**Written:** 2026-04-02T22:06:42.974Z

# S01 UAT — Postgres starter contract

## Preconditions
- `meshc` is available from this repo build or an equivalent installed toolchain.
- You have a reachable PostgreSQL instance and can export a disposable `DATABASE_URL`.
- Docker is running if you want to use a disposable local Postgres container.
- A free local HTTP port is available (examples below use `18080`).

## Test Case 1 — Generate the Postgres starter
1. Run `meshc init --template todo-api --db postgres todo-starter`.
2. Verify the generated project contains `config.mpl`, `.env.example`, `migrations/20260402120000_create_todos.mpl`, `tests/config.test.mpl`, `storage/todos.mpl`, and the Postgres-aware `README.md` / `Dockerfile`.
3. Open `main.mpl` and confirm it references `DATABASE_URL` and `Pool.open(...)`, not `TODO_DB_PATH`, `ensure_schema`, or inline `CREATE TABLE IF NOT EXISTS` DDL.
4. Open `api/health.mpl` and confirm it reports `db_backend : "postgres"` and `migration_strategy : "meshc migrate"`.

**Expected outcome:** The generated app is clearly the Postgres starter, not a renamed SQLite scaffold, and its runtime/config files expose the serious migration-first contract honestly.

## Test Case 2 — Happy-path migrate, test, build, boot, and CRUD
1. If you need a disposable local database, start one. Example:
   ```bash
   docker run -d --rm --name todo-postgres \
     -e POSTGRES_USER=mesh \
     -e POSTGRES_PASSWORD=<password> \
     -e POSTGRES_DB=todo_api \
     -p 127.0.0.1:5432:5432 \
     postgres:16
   export DATABASE_URL=postgres://mesh:<password>@127.0.0.1:5432/todo_api
   ```
2. Run `meshc migrate todo-starter up`.
3. Run `meshc test todo-starter`.
4. Run `meshc build todo-starter`.
5. Start the binary: `PORT=18080 TODO_RATE_LIMIT_WINDOW_SECONDS=60 TODO_RATE_LIMIT_MAX_REQUESTS=5 ./todo-starter`.
6. `GET /health`.
7. `POST /todos` with body `{"title":"Buy milk"}`.
8. `GET /todos`.
9. `GET /todos/<id>` using the ID from the create response.
10. `PUT /todos/<id>`.
11. `DELETE /todos/<id>`.
12. `GET /todos/<id>` again.

**Expected outcome:**
- `GET /health` returns HTTP 200 with JSON containing `status="ok"`, `db_backend="postgres"`, `migration_strategy="meshc migrate"`, `clustered_handler="Work.sync_todos"`, and the configured rate-limit values.
- `POST /todos` returns HTTP 201 with a non-empty `id`, `title="Buy milk"`, and `completed=false`.
- `GET /todos` includes the created todo.
- `GET /todos/<id>` returns the same record.
- `PUT /todos/<id>` toggles `completed` to `true`.
- `DELETE /todos/<id>` returns `{"status":"deleted","id":"<same id>"}`.
- The final `GET /todos/<id>` returns HTTP 404 with `{"error":"todo not found"}`.

## Test Case 3 — Missing `DATABASE_URL` fails closed
1. Keep the built `todo-starter` binary from Test Case 2.
2. Unset `DATABASE_URL`.
3. Run the binary with `PORT`, rate-limit env, and optional clustered-runtime env set, but without `DATABASE_URL`.

**Expected outcome:**
- The process logs `[todo-api] Config error: Missing required environment variable DATABASE_URL`.
- The process exits promptly on its own.
- The logs do **not** contain `[todo-api] Runtime ready` or `[todo-api] HTTP server starting on :...`.

## Test Case 4 — Unmigrated database returns an explicit JSON error
1. Create a fresh empty Postgres database.
2. Export `DATABASE_URL` to that fresh database.
3. Start the built `todo-starter` binary **without** running `meshc migrate`.
4. `GET /health`.
5. `GET /todos`.

**Expected outcome:**
- `GET /health` still returns HTTP 200 with `status="ok"` because the starter can open the pool and start the server.
- `GET /todos` returns HTTP 500 with a JSON `error` string containing `relation "todos" does not exist`.
- Neither the logs nor the HTTP payload echo the raw `DATABASE_URL`.

## Test Case 5 — Retained M048 public tooling/docs contract stays green
1. From the repo root, run `node --test scripts/tests/verify-m048-s05-contract.test.mjs`.

**Expected outcome:** The retained M048 contract test reports 4 passing assertions and 0 failures, confirming that the Postgres starter/runtime wording change did not reintroduce stale tooling or editor claims.

## Edge Cases / Cleanup
- If you used the disposable Docker database, stop/remove it after the test.
- If any step fails, keep the generated project plus runtime logs and raw HTTP responses; the slice-owned regression surface is `cargo test -p meshc --test e2e_m049_s01 -- --nocapture`.
