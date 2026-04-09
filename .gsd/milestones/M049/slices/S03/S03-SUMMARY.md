---
id: S03
parent: M049
milestone: M049
provides:
  - A public `examples/todo-sqlite` tree that matches fresh `meshc init --template todo-api --db sqlite todo-sqlite` output exactly.
  - A public `examples/todo-postgres` tree that matches fresh `meshc init --template todo-api --db postgres todo-postgres` output exactly.
  - One repo-owned materializer/parity seam that regenerates, diffs, tests, and builds both examples through the public CLI.
requires:
  - slice: S01
    provides: The Postgres todo-api scaffold contract, project naming, and serious clustered/deployable starter surface that S03 materialized into `examples/todo-postgres`.
  - slice: S02
    provides: The SQLite local-only todo-api scaffold contract and explicit database-specific wording split that S03 materialized into `examples/todo-sqlite` and the public materializer checks.
affects:
  - S04
  - S05
key_files:
  - scripts/tests/verify-m049-s03-materialize-examples.mjs
  - scripts/tests/verify-m049-s03-materialize-examples.test.mjs
  - examples/todo-sqlite/README.md
  - examples/todo-postgres/README.md
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/tests/tooling_e2e.rs
  - compiler/meshc/tests/support/m049_todo_examples.rs
  - compiler/meshc/tests/e2e_m049_s03.rs
key_decisions:
  - Keep `/examples/todo-sqlite` and `/examples/todo-postgres` generator-owned and refresh them only through the public `meshc init` materializer, not by hand.
  - Reuse the same public materializer command in the Rust closeout rail so example refresh, drift detection, and parity proof stay on one seam.
  - Keep `--check` and `--write` asymmetric: check mode names missing/extra/changed drift, while write mode refuses malformed partial targets before overwriting anything.
patterns_established:
  - Public example trees under `examples/` should be treated as committed scaffold output, not as a second showcase implementation that drifts from `meshc init`.
  - Slice-owned parity rails should archive generated and target manifests plus temp-session pointers under `.tmp/<slice>/...` so later slices can debug drift without re-running ad hoc generation steps.
  - Database-specific differences between generated starters should be asserted explicitly (required and forbidden paths) instead of normalized away in the examples layer.
observability_surfaces:
  - `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check` emits named validation/drift lines with `missing`, `extra`, and `changed` file lists.
  - `.tmp/m049-s03/todo-examples-parity-*` retains generated trees, target snapshots, manifests, prior-diff JSON, and retained-session pointers for public-CLI parity debugging.
  - `.tmp/m049-s03/todo-sqlite-test-build-*` and `.tmp/m049-s03/todo-postgres-test-build-*` retain `meshc test` logs, `meshc build --output` logs, and build metadata proving outputs stayed out of the tracked example trees.
drill_down_paths:
  - .gsd/milestones/M049/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M049/slices/S03/tasks/T02-SUMMARY.md
  - .gsd/milestones/M049/slices/S03/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-03T01:38:38.351Z
blocker_discovered: false
---

# S03: Generated `/examples` from scaffold output

**Checked in generator-owned `examples/todo-sqlite` and `examples/todo-postgres` trees plus one public materializer/parity rail that keeps them mechanically aligned with `meshc init` output and proves both examples still test and build.**

## What Happened

S03 finished the generated-example half of the M049 reset instead of adding another hand-maintained showcase layer.

T01 tightened the SQLite public scaffold wording to the explicit `meshc init --template todo-api --db sqlite` banner and added one repo-owned materializer at `scripts/tests/verify-m049-s03-materialize-examples.mjs`. That command generates both example trees through the public `meshc init` CLI, supports `--write` and `--check`, refuses unsafe or symlinked target roots, rejects malformed partial target trees before overwriting anything, and reports named `missing` / `extra` / `changed` drift in check mode. The companion Node test file proves those red paths and keeps the generator seam honest without touching tracked examples.

T02 used that public materializer seam to check in `examples/todo-sqlite` and `examples/todo-postgres` as exact scaffold output with those exact project names. The examples keep the intentional database-specific file-set split instead of smoothing it away: SQLite retains `tests/storage.test.mpl` and omits `work.mpl`, `.env.example`, and `migrations/`; Postgres retains `work.mpl`, `.env.example`, and `migrations/20260402120000_create_todos.mpl` and omits the SQLite storage test. Both trees stay generator-owned and free of build artifacts or secrets.

T03 added `compiler/meshc/tests/support/m049_todo_examples.rs` and the dedicated `compiler/meshc/tests/e2e_m049_s03.rs` rail. That closeout target reuses the materializer instead of duplicating generation logic, archives generated and target manifests plus retained temp-session pointers under `.tmp/m049-s03/`, proves drift failures name the exact missing/extra/changed files, and verifies that both checked-in examples still pass `meshc test` and `meshc build --output` without writing outputs back into the repo tree.

The result is one stable public example surface under `/examples` and one stable regeneration/proof seam that later slices can point docs and wrappers at instead of inventing a second story.

## Verification

Passed all slice-plan verification rails:

- `cargo test -p mesh-pkg m049_s02_sqlite_scaffold_ -- --nocapture`
- `cargo test -p meshc --test tooling_e2e test_init_todo_template_db_sqlite_ -- --nocapture`
- `node --test scripts/tests/verify-m049-s03-materialize-examples.test.mjs`
- `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`
- `cargo test -p meshc --test e2e_m049_s03 -- --nocapture`

Results:
- SQLite scaffold wording stayed pinned to the explicit `--db sqlite` banner and the related tooling assertions remained green.
- The public materializer check reported both tracked examples as exact matches with mode-specific fingerprints and no missing/extra/changed files.
- The Node test suite proved write-mode replacement, check-mode drift reporting, unsafe-root rejection, malformed partial-target rejection, and surfaced `meshc init` stdout/stderr on generation failure.
- `e2e_m049_s03` proved both tracked examples match fresh public-CLI output, fail closed on missing roots or hand-edited drift, pass `meshc test`, and build via `meshc build --output` into `.tmp/m049-s03/...` instead of polluting `examples/`.

Retained diagnostics now live under `.tmp/m049-s03/todo-examples-parity-*`, `.tmp/m049-s03/todo-sqlite-test-build-*`, and `.tmp/m049-s03/todo-postgres-test-build-*`.

## Requirements Advanced

- R116 — S03 established the generator-owned `/examples/todo-sqlite` and `/examples/todo-postgres` surface and the mechanical parity rail that proves they stay aligned with `meshc init`, giving S04 a real example-first target to point public onboarding at.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None.

## Known Limitations

S03 delivers the generated `/examples` surface and its parity/build rails, but it does not yet retire the older top-level proof-app onboarding surfaces or rewire every public reference to point at `/examples`. That replacement work remains S04, and the assembled one-command reset replay remains S05.

## Follow-ups

S04 should remove `tiny-cluster/` and `cluster-proof/` as public onboarding projects and repoint public references to `examples/todo-sqlite` and `examples/todo-postgres` without breaking the retained internal fixture/proof rails. S05 should assemble the materializer check and `e2e_m049_s03` proof into the final reset wrapper alongside proof-app-retirement and M048 non-regression checks.

## Files Created/Modified

- `scripts/tests/verify-m049-s03-materialize-examples.mjs` — Added the public `meshc init`-driven materializer/check command with safe target validation, atomic writes, and named drift reporting.
- `scripts/tests/verify-m049-s03-materialize-examples.test.mjs` — Added repo-owned Node tests that prove write-mode replacement, check-mode drift output, and failure handling without mutating tracked examples.
- `compiler/mesh-pkg/src/scaffold.rs` — Pinned the SQLite generated README banner to the explicit `--db sqlite` command so the scaffold wording stays database-specific.
- `compiler/meshc/tests/tooling_e2e.rs` — Strengthened CLI/tooling assertions so stale generic SQLite banner wording fails closed.
- `examples/todo-sqlite/README.md` — Checked in the generated local SQLite example under its exact scaffold project name and local-only contract.
- `examples/todo-postgres/README.md` — Checked in the generated PostgreSQL example under its exact scaffold project name and clustered/runtime-aware contract.
- `compiler/meshc/tests/support/m049_todo_examples.rs` — Added shared helpers to run the materializer, archive retained parity artifacts, and verify `meshc test` / `meshc build --output` against the checked-in examples.
- `compiler/meshc/tests/e2e_m049_s03.rs` — Added the authoritative slice rail that proves exact example parity plus out-of-tree test/build behavior for both tracked examples.
