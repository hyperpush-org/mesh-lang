# S05: Simple clustered Todo scaffold — UAT

**Milestone:** M047
**Written:** 2026-04-01T20:00:08.645Z

## UAT Type

- UAT mode: mixed (artifact inspection + live native runtime + Docker packaging build)
- Why this mode is sufficient: S05 changes public clustered source shape, generated scaffold output, live SQLite-backed HTTP behavior, and container packaging. The slice is only trustworthy if all four surfaces agree.

## Preconditions

- Run from the repo root.
- Rust workspace dependencies are installed.
- Docker is available locally.
- `npm --prefix website install` has already been satisfied for docs builds.
- For the Docker packaging case, build the Linux binary you intend to ship before `docker build`, because the Dockerfile copies the existing `./output` artifact into the image.

## Smoke Test

1. Run `cargo test -p mesh-pkg m047_s05 -- --nocapture`.
2. Run `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`.
3. Run `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`.
4. Run `bash scripts/verify-m047-s05.sh`.
5. **Expected:** all four commands pass, proving the generated scaffold shape, live native runtime behavior, Docker image build, and assembled verifier/doc replay together.

## Test Cases

### 1. Generate the Todo scaffold and inspect the source-first contract

1. In a temp directory, run `meshc init --template todo-api todo_api`.
2. Confirm the generated tree contains `mesh.toml`, `main.mpl`, `work.mpl`, `api/`, `runtime/`, `services/`, `storage/`, `types/`, `README.md`, `Dockerfile`, and `.dockerignore`.
3. Open `work.mpl`.
4. **Expected:** it contains `@cluster pub fn sync_todos()` and does **not** contain `execute_declared_work`, `request_key`, or `attempt_id`.
5. Open `README.md` and `Dockerfile`.
6. **Expected:** the README documents `GET /health`, `GET /todos`, `GET /todos/:id`, `POST /todos`, `PUT /todos/:id`, `DELETE /todos/:id`, the rate-limit env vars, runtime-owned `meshc cluster ...` inspection, and the `meshc build .` → `docker build` packaging sequence; the Dockerfile copies `output` into `/usr/local/bin/todo_api`.

### 2. Native runtime proof: health, CRUD, rate limiting, and restart persistence

1. In the generated project, run `meshc build .`.
2. Start the app:
   ```bash
   PORT=8080 \
   TODO_DB_PATH=todo.sqlite3 \
   TODO_RATE_LIMIT_WINDOW_SECONDS=1 \
   TODO_RATE_LIMIT_MAX_REQUESTS=2 \
   ./output
   ```
3. Run `curl -sS http://127.0.0.1:8080/health`.
4. **Expected:** HTTP 200 JSON containing `status: "ok"`, `clustered_handler: "Work.sync_todos"`, `db_path: "todo.sqlite3"`, `rate_limit_window_seconds: 1`, and `rate_limit_max_requests: 2`.
5. Create a todo:
   ```bash
   curl -sS -X POST http://127.0.0.1:8080/todos \
     -H 'Content-Type: application/json' \
     -d '{"title":"Buy milk"}'
   ```
6. **Expected:** HTTP 201 JSON with a string `id`, `title: "Buy milk"`, and `completed: false`.
7. List todos with `curl -sS http://127.0.0.1:8080/todos` and fetch the created id with `curl -sS http://127.0.0.1:8080/todos/<id>`.
8. **Expected:** both responses include the created todo and the same id.
9. Toggle completion:
   ```bash
   curl -sS -X PUT http://127.0.0.1:8080/todos/<id> \
     -H 'Content-Type: application/json' \
     -d '{}'
   ```
10. **Expected:** HTTP 200 JSON for the same todo id with `completed: true`.
11. Trigger the write limiter with one more immediate mutation:
   ```bash
   curl -sS -X POST http://127.0.0.1:8080/todos \
     -H 'Content-Type: application/json' \
     -d '{"title":"Second"}'
   ```
12. **Expected:** HTTP 429 JSON `{ "error": "rate limited" }`.
13. Wait at least 1 second, then create a second todo (`Walk dog`).
14. **Expected:** HTTP 201 and a second string id.
15. Stop the app, restart it with the same env, then run `curl -sS http://127.0.0.1:8080/todos`.
16. **Expected:** both todos are still present, the first todo is still `completed: true`, and the second todo still exists.
17. Delete the second todo:
   ```bash
   curl -sS -X DELETE http://127.0.0.1:8080/todos/<second-id>
   ```
18. **Expected:** HTTP 200 JSON `{ "status": "deleted", "id": "<second-id>" }`.
19. Re-run `curl -sS http://127.0.0.1:8080/todos`.
20. **Expected:** only the first todo remains.

### 3. Docker packaging proof

1. In the generated project, run `meshc build .`.
2. Run `docker build -t todo_api .`.
3. **Expected:** the image build succeeds, the Dockerfile copies `./output` into `/usr/local/bin/todo_api`, and the image exposes ports `8080` and `4370` with the documented `TODO_*` env defaults.

### 4. Public clustered wording stays source-first and honest

1. Open repo-root `README.md`, `website/docs/docs/tooling/index.md`, `tiny-cluster/work.mpl`, and `cluster-proof/work.mpl`.
2. **Expected:** these surfaces teach ordinary `@cluster` function names (`add()` / `sync_todos()`), omit `execute_declared_work(...)` as the public model, and stay explicit that `HTTP.clustered(...)` is still unshipped.

## Edge Cases

### Empty title is rejected

1. Run:
   ```bash
   curl -sS -X POST http://127.0.0.1:8080/todos \
     -H 'Content-Type: application/json' \
     -d '{"title":"   "}'
   ```
2. **Expected:** HTTP 400 JSON `{ "error": "title is required" }`.

### Missing todo id returns a not-found response

1. Run `curl -sS http://127.0.0.1:8080/todos/999999` and `curl -sS -X DELETE http://127.0.0.1:8080/todos/999999`.
2. **Expected:** the read and delete routes return a truthful not-found response rather than crashing the server.

### Rate-limit window resets cleanly

1. Exhaust the 2-write window as in Test Case 2.
2. Wait slightly longer than `TODO_RATE_LIMIT_WINDOW_SECONDS`.
3. Create another todo.
4. **Expected:** the next write succeeds and returns HTTP 201, proving the actor-backed limiter reset tick is active.

### Restart persistence uses the same SQLite file

1. Create/toggle todos, stop the app, and restart with the same `TODO_DB_PATH`.
2. **Expected:** `GET /todos` returns the same persisted rows after restart, proving the scaffold stores state in SQLite rather than in process memory.

## Notes for Tester

- The generated app is intentionally honest about clustered HTTP routes: the clustered function is `sync_todos()` in `work.mpl`, while the HTTP route handlers remain ordinary local handlers until `HTTP.clustered(...)` actually ships.
- The Dockerfile packages the already-built `./output` binary. If you are validating on macOS, build the Linux-target binary in the environment you plan to ship before relying on the produced image at runtime.
