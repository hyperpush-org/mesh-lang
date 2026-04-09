---
estimated_steps: 5
estimated_files: 3
skills_used:
  - rust-best-practices
  - postgresql-database-engineering
  - test
---

# T03: Prove the generated Postgres starter on the live runtime path

**Slice:** S01 — Postgres starter contract
**Milestone:** M049

## Description

Close the loop with a live Postgres proof. The slice is only honest once the generated starter can create an isolated database, apply migrations, run package tests, build, boot, and serve CRUD requests while leaving enough retained evidence for a future agent to localize init, migrate, build, boot, or HTTP failures without exposing secrets.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Live Postgres database used by `compiler/meshc/tests/e2e_m049_s01.rs` | Fail the test with retained phase logs and redacted connection details; do not silently skip the serious-path proof. | Abort the phase, tear down child processes, and retain timeout artifacts so the failing seam is obvious. | Treat invalid DB responses or connection bootstrap drift as a hard test failure. |
| `meshc migrate`, `meshc test`, `meshc build`, and the generated starter binary | Stop at the first failing phase and archive stdout/stderr plus the generated project tree. | Kill the timed-out phase and record whether the failure happened during migrate, test, build, boot, or HTTP smoke. | Fail on missing success markers, malformed `/health`, or invalid CRUD JSON instead of inferring success from process liveness. |

## Load Profile

- **Shared resources**: isolated Postgres database/schema, temp project directory, HTTP port, retained artifact directory, and spawned starter process.
- **Per-operation cost**: one scaffold generation, one migration run, one package-test run, one build, one process boot, and a bounded CRUD/error-path HTTP sequence.
- **10x breakpoint**: port reuse, stale child processes, or database-name collisions fail first; the harness should allocate unique names and clean up aggressively.

## Negative Tests

- **Malformed inputs**: empty todo title, invalid JSON body, and malformed todo ids hitting the generated HTTP handlers.
- **Error paths**: missing `DATABASE_URL`, migration not yet applied or failing, boot-time config validation failure, and `/health` or CRUD returning malformed JSON.
- **Boundary conditions**: empty todo list before the first create, 404 on missing todo fetch/toggle/delete, and safe `/health` output that does not echo secrets.

## Steps

1. Add `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` to initialize `meshc init --template todo-api --db postgres`, create an isolated Postgres database, and manage phase-specific artifact capture.
2. Expose the helper from `compiler/meshc/tests/support/mod.rs` and build `compiler/meshc/tests/e2e_m049_s01.rs` around the full `meshc migrate <project> up` → `meshc test <project>` → `meshc build <project>` → boot → HTTP sequence.
3. Prove `/health`, empty-state list, create, fetch, toggle, and delete against the generated starter.
4. Add at least one diagnostic-path assertion for missing `DATABASE_URL` or equivalent startup failure, and keep raw connection data redacted from artifacts.
5. Re-run the M048 contract script if the CLI/help surface moved so the slice does not regress update/entrypoint/editor markers while landing the new Postgres path.

## Must-Haves

- [ ] The Postgres e2e creates an isolated DB and proves `meshc migrate <project> up`, `meshc test <project>`, `meshc build <project>`, boot, `/health`, and CRUD against the generated starter.
- [ ] At least one diagnostic-path assertion proves missing `DATABASE_URL` or equivalent startup failure is explicit and retained.
- [ ] HTTP negative cases cover empty list, bad input, and missing todo ids with stable expected responses.
- [ ] Retained artifacts do not print the raw `DATABASE_URL`.

## Verification

- `cargo test -p meshc --test e2e_m049_s01 -- --nocapture`
- `node --test scripts/tests/verify-m048-s05-contract.test.mjs`

## Observability Impact

- Signals added/changed: the dedicated e2e harness retains init, migrate, test, build, boot, and HTTP-phase artifacts for the generated Postgres starter.
- How a future agent inspects this: rerun `cargo test -p meshc --test e2e_m049_s01 -- --nocapture` and inspect the retained `.tmp/m049-s01/*` bundle.
- Failure state exposed: the exact failing phase and its stderr/response payload stay visible without printing secrets.

## Inputs

- `compiler/mesh-pkg/src/scaffold.rs` — generated Postgres starter content to prove end-to-end.
- `compiler/meshc/src/migrate.rs` — `meshc migrate <project> up/status` contract the new starter must satisfy.
- `compiler/meshc/tests/support/m047_todo_scaffold.rs` — prior scaffold-harness patterns to reuse selectively without pulling SQLite assumptions forward.
- `compiler/meshc/tests/support/mod.rs` — shared test-support export surface.
- `reference-backend/config.mpl` — canonical config-validation message pattern.
- `reference-backend/tests/config.test.mpl` — package-test shape to mirror in the runtime proof.
- `mesher/migrations/20260216120000_create_initial_schema.mpl` — migration helper usage reference for the generated Postgres project.

## Expected Output

- `compiler/meshc/tests/e2e_m049_s01.rs` — named live Postgres acceptance rail for the generated starter.
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` — helper module for project generation, DB setup, process control, and retained artifacts.
- `compiler/meshc/tests/support/mod.rs` — support-module export wiring for the new harness.
