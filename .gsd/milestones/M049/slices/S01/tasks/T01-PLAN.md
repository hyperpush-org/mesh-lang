---
estimated_steps: 4
estimated_files: 4
skills_used:
  - rust-best-practices
---

# T01: Add typed todo-db selection to `meshc init`

**Slice:** S01 — Postgres starter contract
**Milestone:** M049

## Description

Add the typed DB-selection seam at the CLI boundary before the new Postgres content lands. This task should make `meshc init` fail closed on bad `--db` usage and route the `todo-api` template through a DB-aware scaffold entrypoint without disturbing the existing SQLite default path that S02 will replace later.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `clap` argument parsing and `meshc init` dispatch in `compiler/meshc/src/main.rs` | Exit non-zero with explicit `--db` / `--template todo-api` guidance and do not silently fall back to SQLite. | N/A — local parse and dispatch only. | Reject unsupported database values or conflicting flag combinations before any project directory is created. |
| Scaffold dispatch into `compiler/mesh-pkg/src/scaffold.rs` | Return an error before printing success or leaving a misleading partial tree. | N/A — local filesystem work only. | Refuse unknown database kinds instead of taking a default branch. |

## Load Profile

- **Shared resources**: local project-directory creation, temp test directories, and the scaffold dispatcher.
- **Per-operation cost**: one CLI parse plus one generated project tree write.
- **10x breakpoint**: duplicate-directory collisions and repeated tempdir churn show up first; there is no long-lived shared runtime resource in this task.

## Negative Tests

- **Malformed inputs**: unknown `--db` values, `--db postgres` without `--template todo-api`, and unsupported template/db combinations.
- **Error paths**: `--clustered --template todo-api --db postgres`, rerunning `meshc init` into an existing directory, and any branch that would otherwise silently ignore `--db`.
- **Boundary conditions**: omitting `--db` must preserve the existing SQLite todo template and existing clustered/non-template init flows.

## Steps

1. Add typed todo-db parsing and validation to `compiler/meshc/src/main.rs` so `--db` only works with `--template todo-api`, `--clustered` conflicts are explicit, and unknown values fail before project generation.
2. Route the DB kind through `compiler/mesh-pkg/src/lib.rs` and `compiler/mesh-pkg/src/scaffold.rs` so the todo scaffold uses a typed selector instead of stringly branching.
3. Add or update `compiler/meshc/tests/tooling_e2e.rs` coverage for bad flag combinations and the preserved no-flag SQLite path.
4. Keep the old SQLite scaffold contract green while this seam is introduced; do not broaden docs or M047 runtime helpers in this task.

## Must-Haves

- [ ] `meshc init --template todo-api --db postgres <name>` reaches a dedicated Postgres scaffold path instead of falling through to SQLite.
- [ ] `--db` without `--template todo-api`, unknown DB values, and `--clustered --template todo-api --db ...` fail with explicit non-zero errors.
- [ ] The existing no-flag `meshc init --template todo-api <name>` SQLite path and its current tests remain green.

## Verification

- `cargo test -p meshc --test tooling_e2e test_init_todo_template_db_ -- --nocapture`
- `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`
- `cargo test -p mesh-pkg m047_s05_scaffold_todo_api_project_ -- --nocapture`

## Observability Impact

- Signals added/changed: init-time flag errors distinguish unsupported db values, missing `--template todo-api`, and `--clustered` conflicts.
- How a future agent inspects this: run the new `tooling_e2e` db-flag tests and invoke invalid `meshc init` flag combinations locally.
- Failure state exposed: explicit non-zero stderr messages instead of silent precedence or default-to-SQLite fallbacks.

## Inputs

- `compiler/meshc/src/main.rs` — current `meshc init` CLI parsing and dispatch.
- `compiler/mesh-pkg/src/lib.rs` — scaffold exports used by the CLI boundary.
- `compiler/mesh-pkg/src/scaffold.rs` — current SQLite-only todo scaffold entrypoint.
- `compiler/meshc/tests/tooling_e2e.rs` — existing init contract coverage to extend without regressing.

## Expected Output

- `compiler/meshc/src/main.rs` — typed `--db` parsing and explicit invalid-combination errors.
- `compiler/mesh-pkg/src/lib.rs` — DB-aware todo scaffold export surface.
- `compiler/mesh-pkg/src/scaffold.rs` — typed todo scaffold dispatcher ready for SQLite/Postgres specialization.
- `compiler/meshc/tests/tooling_e2e.rs` — CLI contract tests for `--db` success/failure and preserved SQLite defaults.
