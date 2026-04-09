# S02: SQLite local starter contract — UAT

**Milestone:** M049
**Written:** 2026-04-03T00:42:49.381Z

# S02: SQLite local starter contract — UAT

**Milestone:** M049
**Written:** 2026-04-03

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: this slice changed generator output, a live local runtime contract, retained historical compatibility rails, public docs, and Mesh skill text. The honest closeout needs both artifact-driven contract checks and one real generated SQLite runtime exercise.

## Preconditions

- Run from the repo root.
- Rust/Cargo toolchain is available.
- `meshc` test targets build successfully in this checkout.
- Docker is available if you want to replay the retained historical M047 wrapper exactly as shipped.
- No other process is using the temporary HTTP ports chosen by the e2e harness.

## Smoke Test

Run the live SQLite slice rail:

```bash
cargo test -p meshc --test e2e_m049_s02 -- --nocapture
```

**Expected:** the target reports `2 passed`, and a fresh `.tmp/m049-s02/todo-api-sqlite-runtime-truth-*` plus `.tmp/m049-s02/todo-api-sqlite-bad-db-path-*` bundle exists with generated-project snapshots, build/test logs, runtime stdout/stderr, raw HTTP responses, and bad-path connect-error artifacts.

## Test Cases

### 1. Historical clustered SQLite Todo proof stays green behind the committed fixture

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m047_s05 -- --nocapture
   ```
2. Run:
   ```bash
   bash scripts/verify-m047-s05.sh
   ```
3. Inspect `.tmp/m047-s05/verify/status.txt`, `.tmp/m047-s05/verify/current-phase.txt`, and `.tmp/m047-s05/verify/latest-proof-bundle.txt`.
4. **Expected:** the Rust rail passes, the wrapper exits 0, `status.txt` is `ok`, `current-phase.txt` is `complete`, and retained artifacts contain `init.log` entries with `source=fixture-copy` plus `fixture_root_relative=scripts/fixtures/m047-s05-clustered-todo`.

### 2. The public SQLite scaffold generates a local-only file set

1. Run:
   ```bash
   cargo test -p mesh-pkg m049_s02_sqlite_scaffold_ -- --nocapture
   ```
2. Generate a starter manually:
   ```bash
   meshc init --template todo-api --db sqlite todo_api
   ```
3. Inspect the generated project tree and README.
4. **Expected:** the generated project includes `config.mpl`, `api/health.mpl`, `storage/todos.mpl`, `tests/config.test.mpl`, and `tests/storage.test.mpl`; it does not include `work.mpl`; README text describes SQLite as the honest local starter and does not claim `HTTP.clustered(...)`, `meshc cluster`, or clustered/operator durability.

### 3. CLI/init guidance teaches the explicit SQLite/Postgres split

1. Run:
   ```bash
   cargo test -p meshc --test tooling_e2e test_init_todo_template_db_sqlite_ -- --nocapture
   ```
2. Open `compiler/meshc/src/main.rs` error/help output by invoking the relevant init command manually if needed.
3. **Expected:** the tooling rail passes, SQLite guidance points at `meshc init --template todo-api --db sqlite`, Postgres guidance points at `meshc init --template todo-api --db postgres`, and `meshc init --clustered` remains the minimal clustered scaffold instead of being conflated with the SQLite starter.

### 4. The generated SQLite starter boots locally and reports local `/health`

1. Generate a fresh starter:
   ```bash
   meshc init --template todo-api --db sqlite todo_api
   cd todo_api
   meshc test .
   meshc build .
   ```
2. Start the binary with a writable local DB path:
   ```bash
   PORT=8080 TODO_DB_PATH=./todo.sqlite3 TODO_RATE_LIMIT_MAX_REQUESTS=3 ./todo_api
   ```
3. In another shell, fetch:
   ```bash
   curl -s http://127.0.0.1:8080/health
   ```
4. **Expected:** `/health` returns JSON with `status:"ok"`, `mode:"local"`, `db_backend:"sqlite"`, `storage_mode:"single-node"`, the configured `db_path`, and the configured rate-limit values. It must not expose `clustered_handler`, `migration_strategy`, `node_name`, or other clustered metadata.

### 5. CRUD, rate limit, and restart persistence are real on the generated local starter

1. Start the generated binary as in test case 4.
2. Confirm empty state:
   ```bash
   curl -s http://127.0.0.1:8080/todos
   ```
3. Create a todo:
   ```bash
   curl -s -X POST http://127.0.0.1:8080/todos -H 'content-type: application/json' -d '{"title":"write summary"}'
   ```
4. Fetch the list and the created record, then toggle completion with the generated PUT route.
5. Hit the write path repeatedly until the rate limit triggers.
6. Stop the process, restart it with the same `TODO_DB_PATH`, and fetch `/todos` again.
7. **Expected:** empty list becomes one persisted todo, malformed or missing IDs return the documented 400/404 rails, repeated writes eventually return HTTP 429, and the restarted process still serves the previously created todo from the same `todo.sqlite3` file.

### 6. Bad `TODO_DB_PATH` fails closed before HTTP starts

1. Generate/build the starter.
2. Start it with `TODO_DB_PATH` pointing at a directory path instead of a database file.
3. Try to fetch `/health`.
4. Inspect stdout/stderr or the retained bad-path bundle from `cargo test -p meshc --test e2e_m049_s02 -- --nocapture`.
5. **Expected:** startup logs include `[todo-api] Database init failed: unable to open database file`, the process does not expose `/health`, and the retained artifact includes a `*.connect-error.txt` file confirming the health endpoint stayed unreachable.

### 7. Public docs and Mesh skills keep the starter split honest

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m047_s06 -- --nocapture
   npm --prefix website run build
   node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs
   node --test scripts/tests/verify-m048-s05-contract.test.mjs
   ```
2. **Expected:** all four commands pass. README, VitePress docs, and the Mesh skills all describe SQLite as the honest local starter, Postgres as the fuller shared/deployable starter, and `meshc init --clustered` as the minimal clustered scaffold. The retained contract tests must fail closed if generic `meshc init --template todo-api` or clustered-SQLite wording comes back.

## Edge Cases

### Restart with the same SQLite file

1. Create a todo with `TODO_DB_PATH=./todo.sqlite3`.
2. Stop and restart the binary with the same `TODO_DB_PATH`.
3. **Expected:** the todo still exists after restart, proving local persistence rather than in-memory behavior.

### Empty or broken DB path

1. Run with `TODO_DB_PATH` empty or pointing at a directory.
2. **Expected:** startup fails closed with a config/database-init error and no reachable `/health` endpoint.

### Malformed todo ID and blank title

1. Request `GET /todos/abc` or submit a blank title on `POST /todos`.
2. **Expected:** the generated app returns the documented 400 rail instead of crashing or silently coercing the bad input.

## Failure Signals

- `cargo test -p meshc --test e2e_m047_s05 -- --nocapture` or `bash scripts/verify-m047-s05.sh` goes red, especially on fixture provenance or retained bundle-shape checks.
- `cargo test -p meshc --test e2e_m049_s02 -- --nocapture` fails to produce fresh `.tmp/m049-s02/...` bundles.
- `/health` exposes clustered fields or omits `mode=local` / `db_backend=sqlite` / `storage_mode=single-node`.
- Runtime logs do not show local config/schema/runtime startup markers, or the bad-path run still exposes `/health`.
- Docs or skill contract tests pass while README/site/skill copy drifts back to generic todo-api or clustered-SQLite language.

## Requirements Proved By This UAT

- R115 — the Todo scaffold now supports the SQLite half of the dual-db starter contract with current Mesh patterns, and the local runtime behavior is proven rather than inferred.
- R122 — the SQLite starter now stays explicitly local/single-node and no longer implies clustered/shared durability.

## Not Proven By This UAT

- Generated `/examples` parity; that belongs to M049/S03.
- Retirement of `tiny-cluster/` / `cluster-proof/` as public onboarding surfaces; that belongs to M049/S04.
- A clustered deploy proof for the Postgres starter; this slice only proves the local SQLite contract and preserves the historical M047 clustered SQLite rail behind fixtures.

## Notes for Tester

The current generated `tests/storage.test.mpl` path is intentionally only part of the compile/import-surface proof. If you want real behavioral confidence, trust `cargo test -p meshc --test e2e_m049_s02 -- --nocapture` and the retained `.tmp/m049-s02/...` runtime bundles rather than trying to extend the generated package tests by hand. For the historical clustered Todo story, trust the retained fixture-backed M047 rails; do not treat the public SQLite starter as a hidden clustered mode.
