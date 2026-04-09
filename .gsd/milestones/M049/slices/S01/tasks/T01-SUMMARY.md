---
id: T01
parent: S01
milestone: M049
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/src/main.rs", "compiler/mesh-pkg/src/scaffold.rs", "compiler/mesh-pkg/src/lib.rs", "compiler/meshc/tests/tooling_e2e.rs", "compiler/meshc/tests/e2e_m049_s01.rs", "compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs", "compiler/meshc/tests/support/mod.rs", ".gsd/milestones/M049/slices/S01/tasks/T01-SUMMARY.md"]
key_decisions: ["Use a typed `TodoApiDatabase` seam in `mesh-pkg` and keep the Postgres branch fail-closed until the real starter content lands.", "Validate init flag combinations before project-directory creation and map the CLI enum to the scaffold enum at the boundary."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Replayed the task rail and the slice rail after the final formatting pass. `cargo test -p mesh-pkg m049_s01_ -- --nocapture`, `cargo test -p mesh-pkg m047_s05_scaffold_todo_api_project_ -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_todo_template_db_ -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_todo_template_postgres_ -- --nocapture`, `cargo test -p meshc --test e2e_m049_s01 -- --nocapture`, and `node --test scripts/tests/verify-m048-s05-contract.test.mjs` all passed."
completed_at: 2026-04-02T20:51:32.572Z
blocker_discovered: false
---

# T01: Added typed `--db` init validation and a DB-aware todo-api scaffold seam without regressing the existing SQLite starter.

> Added typed `--db` init validation and a DB-aware todo-api scaffold seam without regressing the existing SQLite starter.

## What Happened
---
id: T01
parent: S01
milestone: M049
key_files:
  - compiler/meshc/src/main.rs
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/mesh-pkg/src/lib.rs
  - compiler/meshc/tests/tooling_e2e.rs
  - compiler/meshc/tests/e2e_m049_s01.rs
  - compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs
  - compiler/meshc/tests/support/mod.rs
  - .gsd/milestones/M049/slices/S01/tasks/T01-SUMMARY.md
key_decisions:
  - Use a typed `TodoApiDatabase` seam in `mesh-pkg` and keep the Postgres branch fail-closed until the real starter content lands.
  - Validate init flag combinations before project-directory creation and map the CLI enum to the scaffold enum at the boundary.
duration: ""
verification_result: passed
completed_at: 2026-04-02T20:51:32.578Z
blocker_discovered: false
---

# T01: Added typed `--db` init validation and a DB-aware todo-api scaffold seam without regressing the existing SQLite starter.

**Added typed `--db` init validation and a DB-aware todo-api scaffold seam without regressing the existing SQLite starter.**

## What Happened

Added a typed `InitTodoDb` CLI enum and `run_init_command` / `resolve_init_target` validation path in `compiler/meshc/src/main.rs` so `meshc init` rejects unsupported `--db` usage, explicit clustered/template conflicts, and unknown DB values before creating a project directory. Introduced `mesh_pkg::TodoApiDatabase` plus `scaffold_todo_api_project_with_db(...)` in `compiler/mesh-pkg/src/scaffold.rs`, kept `scaffold_todo_api_project(...)` as the SQLite-default wrapper for backward compatibility, and made the dedicated Postgres branch fail closed with an explicit not-implemented error instead of silently falling through to the SQLite scaffold. Extended `compiler/meshc/tests/tooling_e2e.rs` with DB-flag coverage, added the new `compiler/meshc/tests/e2e_m049_s01.rs` slice rail, and added `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` plus the support-module registration so later S01 tasks can extend a single shared scaffold harness.

## Verification

Replayed the task rail and the slice rail after the final formatting pass. `cargo test -p mesh-pkg m049_s01_ -- --nocapture`, `cargo test -p mesh-pkg m047_s05_scaffold_todo_api_project_ -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_todo_template_db_ -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_todo_template_postgres_ -- --nocapture`, `cargo test -p meshc --test e2e_m049_s01 -- --nocapture`, and `node --test scripts/tests/verify-m048-s05-contract.test.mjs` all passed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-pkg m049_s01_ -- --nocapture` | 0 | ✅ pass | 11977ms |
| 2 | `cargo test -p mesh-pkg m047_s05_scaffold_todo_api_project_ -- --nocapture` | 0 | ✅ pass | 1060ms |
| 3 | `cargo test -p meshc --test tooling_e2e test_init_todo_template_db_ -- --nocapture` | 0 | ✅ pass | 30424ms |
| 4 | `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture` | 0 | ✅ pass | 9080ms |
| 5 | `cargo test -p meshc --test tooling_e2e test_init_todo_template_postgres_ -- --nocapture` | 0 | ✅ pass | 8165ms |
| 6 | `cargo test -p meshc --test e2e_m049_s01 -- --nocapture` | 0 | ✅ pass | 12329ms |
| 7 | `node --test scripts/tests/verify-m048-s05-contract.test.mjs` | 0 | ✅ pass | 1806ms |


## Deviations

Added the slice-owned `compiler/meshc/tests/e2e_m049_s01.rs` target and its small support helper in T01 so the slice verification target exists from the first task instead of appearing later as an empty or missing rail.

## Known Issues

`meshc init --template todo-api --db postgres <name>` intentionally fails closed with an explicit not-implemented error until later S01 tasks land the real Postgres starter content.

## Files Created/Modified

- `compiler/meshc/src/main.rs`
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/mesh-pkg/src/lib.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/e2e_m049_s01.rs`
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`
- `compiler/meshc/tests/support/mod.rs`
- `.gsd/milestones/M049/slices/S01/tasks/T01-SUMMARY.md`


## Deviations
Added the slice-owned `compiler/meshc/tests/e2e_m049_s01.rs` target and its small support helper in T01 so the slice verification target exists from the first task instead of appearing later as an empty or missing rail.

## Known Issues
`meshc init --template todo-api --db postgres <name>` intentionally fails closed with an explicit not-implemented error until later S01 tasks land the real Postgres starter content.
