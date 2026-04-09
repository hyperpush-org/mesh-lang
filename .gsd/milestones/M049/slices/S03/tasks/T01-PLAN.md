---
estimated_steps: 4
estimated_files: 4
skills_used:
  - rust-best-practices
  - test
---

# T01: Lock the SQLite banner and add a public-CLI example materializer

Before checking in `/examples`, stabilize the generator inputs and the regeneration seam. Tighten the SQLite README banner to the explicit `--db sqlite` command, then add a repo-owned materializer/check script that generates both example trees through the public `meshc init` CLI into temp space. The script must support `--write` and `--check` modes, fail closed on unsafe target paths or partial output, and have a small Node test so future refreshes do not depend on ad hoc copy steps.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Public `meshc init` CLI | Stop and surface stdout/stderr from generation; do not write partial example trees. | Bound each generation call and fail with the command plus temp path if it hangs. | Reject generated trees missing expected root files instead of diffing partial output. |
| `compiler/mesh-pkg/src/scaffold.rs` and `compiler/meshc/tests/tooling_e2e.rs` wording assertions | Fail tests before any example refresh lands. | N/A — local test execution only. | Treat generic SQLite wording or mixed local/clustered guidance as contract drift. |
| Local filesystem target paths | Refuse to write outside `examples/todo-sqlite` and `examples/todo-postgres`. | N/A — local filesystem work only. | Reject symlinks, partial target trees, or unexpected target roots instead of overwriting blindly. |

## Load Profile

- **Shared resources**: temp workspaces, repo example roots, and the local `meshc` binary path used by the materializer.
- **Per-operation cost**: two `meshc init` runs plus recursive manifest/diff generation.
- **10x breakpoint**: repeated scaffold generation and filesystem churn show up before CPU; the script should leave no partial temp trees behind.

## Negative Tests

- **Malformed inputs**: unknown mode/flag, invalid `MESHC_BIN` / `--meshc-bin`, and missing expected scaffold files.
- **Error paths**: generic SQLite README wording surviving, generation failure, or an unsafe write target outside the two example roots.
- **Boundary conditions**: check mode on a temp examples root, write mode replacing an existing example tree atomically, and the intentional SQLite/Postgres file-set differences.

## Steps

1. Update `compiler/mesh-pkg/src/scaffold.rs` so the SQLite README banner uses the explicit `meshc init --template todo-api --db sqlite` form and extend nearby scaffold assertions to pin that wording.
2. Add `scripts/tests/verify-m049-s03-materialize-examples.mjs` with `--write` and `--check` modes, public-CLI generation, temp staging, allowed-target validation, and exact recursive diff output.
3. Add `scripts/tests/verify-m049-s03-materialize-examples.test.mjs` covering temp-root write/check behavior and drift detection without touching the tracked repo examples.
4. Extend `compiler/meshc/tests/tooling_e2e.rs` so the SQLite init path fails closed on stale generic wording.

## Must-Haves

- [ ] SQLite public scaffold surfaces use explicit `--db sqlite` wording.
- [ ] One repo-owned script can regenerate or check both example trees through `meshc init`, not private scaffold internals.
- [ ] Script red paths name missing/extra/changed files and refuse unsafe target paths.

## Inputs

- `compiler/mesh-pkg/src/scaffold.rs` — current SQLite and Postgres todo-api scaffold templates that will feed the checked-in examples.
- `compiler/meshc/tests/tooling_e2e.rs` — existing CLI/init contract rail that should pin the explicit SQLite wording.
- `scripts/tests/verify-m036-s02-materialize-corpus.mjs` — repo-owned materializer pattern to mirror for temp staging, manifest output, and fail-closed errors.

## Expected Output

- `compiler/mesh-pkg/src/scaffold.rs` — SQLite README text and scaffold assertions updated to the explicit `--db sqlite` contract.
- `compiler/meshc/tests/tooling_e2e.rs` — tooling rail updated to reject stale generic SQLite init wording.
- `scripts/tests/verify-m049-s03-materialize-examples.mjs` — repo-owned example materializer/check command using the public CLI.
- `scripts/tests/verify-m049-s03-materialize-examples.test.mjs` — Node test covering write/check behavior and drift reporting.

## Verification

- `cargo test -p mesh-pkg m049_s02_sqlite_scaffold_ -- --nocapture`
- `cargo test -p meshc --test tooling_e2e test_init_todo_template_db_sqlite_ -- --nocapture`
- `node --test scripts/tests/verify-m049-s03-materialize-examples.test.mjs`

## Observability Impact

- Signals added/changed: the materializer emits deterministic manifest/diff output and preserves generation command context on failure.
- How a future agent inspects this: rerun `node --test scripts/tests/verify-m049-s03-materialize-examples.test.mjs` or invoke the script with `--check`.
- Failure state exposed: missing/extra/changed files and rejected target paths are named explicitly instead of collapsing into one generic exception.
