# S03: Generated `/examples` from scaffold output — UAT

**Milestone:** M049
**Written:** 2026-04-03T01:38:38.361Z

# S03: Generated `/examples` from scaffold output — UAT

**Milestone:** M049
**Written:** 2026-04-02

## UAT Type

- UAT mode: artifact-driven
- Why this mode is sufficient: S03 ships committed generated example trees plus repo-owned CLI/test/build rails. The truth surface is whether fresh public `meshc init` output matches the checked-in trees exactly and whether those checked-in trees still pass `meshc test` and `meshc build --output` without hand edits or repo-tree pollution.

## Preconditions

- The repo is in `/Users/sn0w/Documents/dev/mesh-lang`.
- `target/debug/meshc` exists (build it first if needed).
- `examples/todo-sqlite` and `examples/todo-postgres` are present in the working tree.
- `.tmp/` is writable for retained verification artifacts.

## Smoke Test

1. Run `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`.
2. Confirm the command prints one `phase=check` line for `todo-sqlite`, one for `todo-postgres`, and a final `phase=materialize mode=check result=pass` line.
3. **Expected:** Both examples match fresh public `meshc init` output exactly with no missing/extra/changed files reported.

## Test Cases

### 1. SQLite example stays generator-owned and local-only

1. Open `examples/todo-sqlite/mesh.toml` and confirm the package name is `todo-sqlite`.
2. Confirm `examples/todo-sqlite/tests/storage.test.mpl` exists.
3. Confirm `examples/todo-sqlite/work.mpl`, `examples/todo-sqlite/.env.example`, and `examples/todo-sqlite/migrations/` do **not** exist.
4. Run `meshc test examples/todo-sqlite`.
5. Run `meshc build examples/todo-sqlite --output .tmp/m049-s03/uat/todo-sqlite/output`.
6. **Expected:** `meshc test` reports the generated SQLite config and storage tests as passing, `meshc build --output` succeeds, and the binary is written under `.tmp/m049-s03/uat/todo-sqlite/output` instead of inside `examples/todo-sqlite/`.

### 2. Postgres example stays generator-owned and clustered-aware

1. Open `examples/todo-postgres/mesh.toml` and confirm the package name is `todo-postgres`.
2. Confirm `examples/todo-postgres/work.mpl`, `examples/todo-postgres/.env.example`, and `examples/todo-postgres/migrations/20260402120000_create_todos.mpl` exist.
3. Confirm `examples/todo-postgres/tests/storage.test.mpl` does **not** exist.
4. Run `meshc test examples/todo-postgres`.
5. Run `meshc build examples/todo-postgres --output .tmp/m049-s03/uat/todo-postgres/output`.
6. **Expected:** `meshc test` reports the generated Postgres config tests as passing, retains the runtime-owned startup marker `startup::Work.sync_todos` in output, and `meshc build --output` succeeds without writing build outputs back into `examples/todo-postgres/`.

### 3. Public parity rail fails closed on drift and names the exact files

1. Copy `examples/` to `.tmp/m049-s03/uat/drift/examples`.
2. Edit `.tmp/m049-s03/uat/drift/examples/todo-sqlite/mesh.toml` so the package name is no longer `todo-sqlite`.
3. Delete `.tmp/m049-s03/uat/drift/examples/todo-sqlite/tests/storage.test.mpl`.
4. Delete `.tmp/m049-s03/uat/drift/examples/todo-postgres/migrations/20260402120000_create_todos.mpl`.
5. Add `.tmp/m049-s03/uat/drift/examples/todo-postgres/HAND_EDITED.txt`.
6. Run `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check --examples-root .tmp/m049-s03/uat/drift/examples --meshc-bin target/debug/meshc`.
7. **Expected:** The command exits non-zero and names `changed=mesh.toml`, `missing=tests/storage.test.mpl`, `missing=migrations/20260402120000_create_todos.mpl`, and `extra=HAND_EDITED.txt` instead of silently normalizing the drift.

## Edge Cases

### Write mode refuses malformed partial targets before overwriting

1. Create `.tmp/m049-s03/uat/partial/examples/todo-sqlite/README.md` with placeholder text and do not create the rest of the scaffold tree.
2. Run `node scripts/tests/verify-m049-s03-materialize-examples.mjs --write --examples-root .tmp/m049-s03/uat/partial/examples --meshc-bin target/debug/meshc`.
3. **Expected:** The command exits non-zero with a validation failure naming the malformed partial target, and the placeholder `README.md` remains untouched.

### Success-path parity check keeps no temp session unless explicitly retained

1. Run `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`.
2. Confirm it succeeds.
3. **Expected:** The public check command does not leave a retained materializer temp session behind on success; retained generated-vs-target artifacts come from the Rust `e2e_m049_s03` rail or an explicit `keepTemp` import path.

## Failure Signals

- `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check` exits non-zero or reports any non-`-` `missing`, `extra`, or `changed` values for either example.
- `meshc test examples/todo-sqlite` or `meshc test examples/todo-postgres` reports `COMPILE ERROR` or stops printing the expected generated test names.
- `meshc build --output` writes an output binary back into `examples/todo-sqlite/` or `examples/todo-postgres/` instead of the requested `.tmp` path.
- The SQLite example gains `work.mpl`, `.env.example`, or `migrations/`, or the Postgres example loses `work.mpl`, `.env.example`, or the migration file.

## Requirements Proved By This UAT

- R116 — proves the repo now ships generator-owned `examples/todo-sqlite` and `examples/todo-postgres` trees under a stable `/examples` surface and that they stay mechanically aligned with public `meshc init` output. This UAT does not yet prove proof-app retirement; that remains S04.

## Not Proven By This UAT

- Removal of `tiny-cluster/`, `cluster-proof/`, or other older onboarding surfaces from the public story.
- The final one-command M049 assembled replay that combines scaffold/example truth, proof-app retirement, and M048 non-regression rails.

## Notes for Tester

Use the public materializer and the retained `.tmp/m049-s03/...` bundles as the debugging starting point; do not hand-edit the example trees to make parity pass. If a parity check goes red, inspect the named drift lines first, then the latest retained `todo-examples-parity-*`, `todo-sqlite-test-build-*`, and `todo-postgres-test-build-*` bundles before touching scaffold templates or the checked-in `examples/` trees.
