---
id: T01
parent: S03
milestone: M049
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-pkg/src/scaffold.rs", "compiler/meshc/tests/tooling_e2e.rs", "scripts/tests/verify-m049-s03-materialize-examples.mjs", "scripts/tests/verify-m049-s03-materialize-examples.test.mjs", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Keep the SQLite starter README banner pinned to the explicit `meshc init --template todo-api --db sqlite` command in both scaffold and CLI rails.", "Keep the new materializer asymmetric on purpose: `--check` reports named drift, while `--write` rejects malformed partial targets before replacing anything."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "`cargo test -p mesh-pkg m049_s02_sqlite_scaffold_ -- --nocapture` passed (3 tests) and `cargo test -p meshc --test tooling_e2e test_init_todo_template_db_sqlite_ -- --nocapture` passed (5 tests), proving the scaffold and CLI wording contract. `node --test scripts/tests/verify-m049-s03-materialize-examples.test.mjs` passed (5 tests), proving the new materializer against temp-root write/check, drift, unsafe-root, invalid-binary, and generation-failure cases. Slice-level status was also recorded: `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check` currently fails with named missing `examples/todo-sqlite` and `examples/todo-postgres` roots because T02 has not materialized them yet, and `cargo test -p meshc --test e2e_m049_s03 -- --nocapture` fails because the T03 test target does not exist yet."
completed_at: 2026-04-03T01:19:02.431Z
blocker_discovered: false
---

# T01: Locked the SQLite scaffold banner to `--db sqlite` and added a repo-owned public-CLI example materializer/check script with temp-root tests.

> Locked the SQLite scaffold banner to `--db sqlite` and added a repo-owned public-CLI example materializer/check script with temp-root tests.

## What Happened
---
id: T01
parent: S03
milestone: M049
key_files:
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/tests/tooling_e2e.rs
  - scripts/tests/verify-m049-s03-materialize-examples.mjs
  - scripts/tests/verify-m049-s03-materialize-examples.test.mjs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Keep the SQLite starter README banner pinned to the explicit `meshc init --template todo-api --db sqlite` command in both scaffold and CLI rails.
  - Keep the new materializer asymmetric on purpose: `--check` reports named drift, while `--write` rejects malformed partial targets before replacing anything.
duration: ""
verification_result: mixed
completed_at: 2026-04-03T01:19:02.432Z
blocker_discovered: false
---

# T01: Locked the SQLite scaffold banner to `--db sqlite` and added a repo-owned public-CLI example materializer/check script with temp-root tests.

**Locked the SQLite scaffold banner to `--db sqlite` and added a repo-owned public-CLI example materializer/check script with temp-root tests.**

## What Happened

Updated the SQLite todo scaffold README banner to the explicit `meshc init --template todo-api --db sqlite` command, then tightened both the scaffold-unit and tooling integration assertions so generic wording now fails closed. Added `scripts/tests/verify-m049-s03-materialize-examples.mjs` as the repo-owned public-CLI regeneration seam for `todo-sqlite` and `todo-postgres`, with temp generation, target validation, deterministic manifest/diff output, `--write`/`--check` modes, atomic replacement, and preserved temp-path diagnostics on failure. Added `scripts/tests/verify-m049-s03-materialize-examples.test.mjs` to cover temp-root write/check behavior, drift reporting, unsafe roots, partial targets, invalid binary overrides, and generation failure propagation without touching tracked repo example paths. Recorded the intentional `--check` versus `--write` safety split in project knowledge for follow-on tasks.

## Verification

`cargo test -p mesh-pkg m049_s02_sqlite_scaffold_ -- --nocapture` passed (3 tests) and `cargo test -p meshc --test tooling_e2e test_init_todo_template_db_sqlite_ -- --nocapture` passed (5 tests), proving the scaffold and CLI wording contract. `node --test scripts/tests/verify-m049-s03-materialize-examples.test.mjs` passed (5 tests), proving the new materializer against temp-root write/check, drift, unsafe-root, invalid-binary, and generation-failure cases. Slice-level status was also recorded: `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check` currently fails with named missing `examples/todo-sqlite` and `examples/todo-postgres` roots because T02 has not materialized them yet, and `cargo test -p meshc --test e2e_m049_s03 -- --nocapture` fails because the T03 test target does not exist yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-pkg m049_s02_sqlite_scaffold_ -- --nocapture` | 0 | ✅ pass | 39200ms |
| 2 | `cargo test -p meshc --test tooling_e2e test_init_todo_template_db_sqlite_ -- --nocapture` | 0 | ✅ pass | 39100ms |
| 3 | `node --test scripts/tests/verify-m049-s03-materialize-examples.test.mjs` | 0 | ✅ pass | 8819ms |
| 4 | `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check` | 1 | ❌ fail | 724ms |
| 5 | `cargo test -p meshc --test e2e_m049_s03 -- --nocapture` | 101 | ❌ fail | 459ms |


## Deviations

None.

## Known Issues

None. The remaining red slice-level checks are expected pre-T02/T03 state: tracked `examples/` trees do not exist yet, and the `e2e_m049_s03` target has not been added yet.

## Files Created/Modified

- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `scripts/tests/verify-m049-s03-materialize-examples.mjs`
- `scripts/tests/verify-m049-s03-materialize-examples.test.mjs`
- `.gsd/KNOWLEDGE.md`


## Deviations
None.

## Known Issues
None. The remaining red slice-level checks are expected pre-T02/T03 state: tracked `examples/` trees do not exist yet, and the `e2e_m049_s03` target has not been added yet.
