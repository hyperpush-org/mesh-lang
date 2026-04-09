---
id: T03
parent: S03
milestone: M049
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/support/m049_todo_examples.rs", "compiler/meshc/tests/support/mod.rs", "compiler/meshc/tests/e2e_m049_s03.rs", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Retain example parity artifacts by importing `materializeExamples(...)` with `keepTemp: true` from Rust support instead of widening the public CLI surface.", "Reuse the existing SQLite/Postgres scaffold helpers for `meshc test` and `meshc build --output` so the committed examples inherit the same capture, timeout, and out-of-tree binary checks as the generated-starter rails."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "`node scripts/tests/verify-m049-s03-materialize-examples.mjs --check` passed and confirmed both committed examples still match fresh public-CLI output. `cargo test -p meshc --test e2e_m049_s03 -- --nocapture` passed all five tests, covering parity success, missing-root failure, named missing/extra/changed drift reporting, and SQLite/Postgres `meshc test` / `meshc build --output` proof. A final artifact-presence check confirmed the retained `.tmp/m049-s03` bundles contain generated/target snapshots, diff reports, meshc test logs, and build metadata at the promised paths."
completed_at: 2026-04-03T01:34:19.851Z
blocker_discovered: false
---

# T03: Added retained parity, `meshc test`, and `meshc build --output` rails for `examples/todo-sqlite` and `examples/todo-postgres`.

> Added retained parity, `meshc test`, and `meshc build --output` rails for `examples/todo-sqlite` and `examples/todo-postgres`.

## What Happened
---
id: T03
parent: S03
milestone: M049
key_files:
  - compiler/meshc/tests/support/m049_todo_examples.rs
  - compiler/meshc/tests/support/mod.rs
  - compiler/meshc/tests/e2e_m049_s03.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Retain example parity artifacts by importing `materializeExamples(...)` with `keepTemp: true` from Rust support instead of widening the public CLI surface.
  - Reuse the existing SQLite/Postgres scaffold helpers for `meshc test` and `meshc build --output` so the committed examples inherit the same capture, timeout, and out-of-tree binary checks as the generated-starter rails.
duration: ""
verification_result: passed
completed_at: 2026-04-03T01:34:19.852Z
blocker_discovered: false
---

# T03: Added retained parity, `meshc test`, and `meshc build --output` rails for `examples/todo-sqlite` and `examples/todo-postgres`.

**Added retained parity, `meshc test`, and `meshc build --output` rails for `examples/todo-sqlite` and `examples/todo-postgres`.**

## What Happened

Added `compiler/meshc/tests/support/m049_todo_examples.rs` as the slice-owned harness for the committed example trees. It wraps the T01 materializer through a Node module import so the rail can keep `keepTemp: true`, then archives generated-vs-target trees, manifests, diff reports, and the retained temp session under `.tmp/m049-s03/...` instead of losing them to the public CLI’s normal success cleanup. The same helper reuses the existing SQLite/Postgres scaffold build/test support so the committed examples are exercised through the same `meshc test` and `meshc build --output` seams as the generated starters, including temp-log capture and explicit out-of-tree binary checks.

Added `compiler/meshc/tests/e2e_m049_s03.rs` with five named tests: one happy-path parity rail, two negative parity rails (missing example root plus named missing/extra/changed drift reporting), and one build/test rail per example. The positive parity test also asserts the intentional SQLite/Postgres file-set split stays intact (`tests/storage.test.mpl` only on SQLite; `work.mpl`, `.env.example`, and the migration only on Postgres). The SQLite build/test rail proves generated config/storage tests stay green and that `meshc build --output` does not emit repo-tree binaries. The Postgres rail proves generated config tests stay green, preserves the runtime-owned `startup::Work.sync_todos` marker in `meshc test` output, and keeps the output binary under `.tmp/m049-s03/...` rather than under `examples/`.

I also appended the non-obvious harness rule to `.gsd/KNOWLEDGE.md`: the public materializer CLI cleans temp state on success, so retained parity artifacts must come from importing `materializeExamples(...)` with `keepTemp: true`, not by wrapping `node ... --check` and hoping the session survives.

## Verification

`node scripts/tests/verify-m049-s03-materialize-examples.mjs --check` passed and confirmed both committed examples still match fresh public-CLI output. `cargo test -p meshc --test e2e_m049_s03 -- --nocapture` passed all five tests, covering parity success, missing-root failure, named missing/extra/changed drift reporting, and SQLite/Postgres `meshc test` / `meshc build --output` proof. A final artifact-presence check confirmed the retained `.tmp/m049-s03` bundles contain generated/target snapshots, diff reports, meshc test logs, and build metadata at the promised paths.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check` | 0 | ✅ pass | 1275ms |
| 2 | `cargo test -p meshc --test e2e_m049_s03 -- --nocapture` | 0 | ✅ pass | 19662ms |
| 3 | `python3 - <<'PY' ... artifact-bundle presence check for .tmp/m049-s03 parity/build outputs ... PY` | 0 | ✅ pass | 1ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `compiler/meshc/tests/support/m049_todo_examples.rs`
- `compiler/meshc/tests/support/mod.rs`
- `compiler/meshc/tests/e2e_m049_s03.rs`
- `.gsd/KNOWLEDGE.md`


## Deviations
None.

## Known Issues
None.
