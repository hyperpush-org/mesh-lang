---
estimated_steps: 4
estimated_files: 3
skills_used:
  - rust-best-practices
  - SQLite Database Expert
  - test
---

# T02: Rewrite the public SQLite todo-api scaffold as a local-only starter

**Slice:** S02 — SQLite local starter contract
**Milestone:** M049

## Description

Use S01’s typed DB seam to replace the public SQLite branch with an explicitly local contract. The point is not to fake Postgres parity or preserve the old clustered proof in disguise; it is to ship a truthful local starter that uses current Mesh SQLite patterns and gives S03 a stable output to snapshot.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/mesh-pkg/src/scaffold.rs` template strings and file list | Fail scaffold generation or static contract tests before a misleading mixed local/clustered project can ship. | N/A — local generation only. | Reject partial local/clustered hybrids (`work.mpl`, `HTTP.clustered(...)`, or `meshc cluster` guidance surviving in the SQLite branch). |
| `compiler/meshc/src/main.rs` init guidance and `tooling_e2e` assertions | Exit non-zero with explicit SQLite-local vs Postgres-clustered guidance and do not preserve the old clustered wording. | N/A — local CLI and test execution only. | Treat stale README/help text or missing generated tests as contract drift. |

## Load Profile

- **Shared resources**: generated project trees, local SQLite database files, and the default `meshc init` path.
- **Per-operation cost**: one scaffold write plus local `meshc test` / build proof on the generated project.
- **10x breakpoint**: repeated local DB init/test churn shows up before CPU; there should be no cluster transport or operator path left in this starter.

## Negative Tests

- **Malformed inputs**: empty title, malformed todo id, invalid positive-int env values, and broken `TODO_DB_PATH` handling.
- **Error paths**: stale `work.mpl`, `HTTP.clustered(...)`, `meshc cluster`, or `MESH_*` guidance surviving in the SQLite branch.
- **Boundary conditions**: empty todo list, `:memory:` or temp-path package tests, and the no-flag SQLite path staying the default while Postgres remains explicit.

## Steps

1. Rewrite the SQLite scaffold strings in `compiler/mesh-pkg/src/scaffold.rs` so the generated file set is local-only: remove `work.mpl`, localize `main.mpl` bootstrap, update `/health`, router, README, Dockerfile, and `.dockerignore`, and expose explicit SQLite-local markers.
2. Modernize the SQLite storage/type templates to stay on parameterized `Sqlite.*` calls and typed row conversion (`deriving(Row)` / `Todo.from_row(...)`) instead of manual `Map.get(...)` parsing.
3. Emit a small generated package-test surface (for example `tests/config.test.mpl` and `tests/storage.test.mpl`) so `meshc test <project>` proves the local contract.
4. Update `compiler/meshc/src/main.rs` guidance and `compiler/meshc/tests/tooling_e2e.rs` expectations to teach SQLite-local vs Postgres-clustered without reintroducing a shadow scaffold mode.

## Must-Haves

- [ ] The generated SQLite starter is explicitly single-node/local: no `work.mpl`, `Node.start_from_env()`, `HTTP.clustered(...)`, `clustered_handler`, `meshc cluster`, `MESH_*` env, or cluster-port Docker exposure.
- [ ] The generated SQLite project uses current Mesh-local SQLite patterns and includes real `.test.mpl` files.
- [ ] CLI/help and tooling tests point clustered/deployable guidance at `--db postgres` / `--clustered` while keeping SQLite as the honest local default.

## Verification

- `cargo test -p mesh-pkg m049_s02_sqlite_scaffold_ -- --nocapture`
- `cargo test -p meshc --test tooling_e2e test_init_todo_template_db_sqlite_ -- --nocapture`

## Observability Impact

- Signals added/changed: the generated SQLite starter logs and `/health` surface report local SQLite state instead of clustered-handler truth.
- How a future agent inspects this: run the SQLite scaffold/tooling tests or generate a project and inspect its README, health handler, and generated test files.
- Failure state exposed: broken db-path/config behavior and stale clustered markers are caught by static scaffold and CLI contract rails.

## Inputs

- `compiler/mesh-pkg/src/scaffold.rs` — current SQLite starter strings that still teach the clustered contract.
- `compiler/meshc/src/main.rs` — current `meshc init` messaging that still calls the SQLite starter the current clustered path.
- `compiler/meshc/tests/tooling_e2e.rs` — existing CLI/init assertions that must be inverted to the new local-first contract.
- `scripts/fixtures/m047-s05-clustered-todo/README.md` — retained historical wording kept separate from the public rewrite.

## Expected Output

- `compiler/mesh-pkg/src/scaffold.rs` — local-only SQLite scaffold strings plus generated package tests and updated local health/log/readme contract.
- `compiler/meshc/src/main.rs` — init guidance updated to separate SQLite-local from Postgres-clustered and minimal clustered scaffold flows.
- `compiler/meshc/tests/tooling_e2e.rs` — tooling contract tests proving the new local SQLite output and guidance.
