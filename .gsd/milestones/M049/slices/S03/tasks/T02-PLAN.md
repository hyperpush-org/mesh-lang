---
estimated_steps: 4
estimated_files: 6
skills_used:
  - test
---

# T02: Check in the generated SQLite and Postgres example trees

Use the T01 materializer to create the tracked example apps as generated outputs, not hand-edited showcase projects. This task owns the committed trees only: `examples/todo-sqlite` and `examples/todo-postgres` should be generated with those exact project names, stay free of build artifacts, and stop at example content. It must not rewrite repo docs or retire proof apps; that boundary remains S04 work.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| T01 materializer/check script | Stop on any generation or drift failure; do not hand-fix generated files. | Bound the script call and keep its temp output path for diagnosis. | Reject partial writes or half-generated example dirs instead of normalizing them by hand. |
| Current scaffold templates | Regenerate from the public CLI and compare before accepting the tree. | N/A — local generation only. | Reject unexpected file additions/removals rather than silently checking them in. |
| Tracked example directories | Replace from staged temp output and keep repo paths free of build artifacts and secrets. | N/A — local filesystem work only. | Refuse dirty unexpected non-generated files or unsafe path shapes under `examples/`. |

## Load Profile

- **Shared resources**: repo example directories, temp generation roots, and the working tree diff.
- **Per-operation cost**: two scaffold generations and one recursive check replay.
- **10x breakpoint**: working tree churn and review noise show up before CPU; the task should keep the example trees generator-owned and boring.

## Negative Tests

- **Malformed inputs**: wrong project name, missing SQLite storage test, or missing Postgres migration / `.env.example` file.
- **Error paths**: extra hand-edited files under an example tree or partial generation output.
- **Boundary conditions**: SQLite and Postgres intentionally keep different file sets, and S03 must not add an `examples/README.md`, repo doc rewires, or proof-app deletions.

## Steps

1. Run the T01 materializer in write mode to generate `examples/todo-sqlite` and `examples/todo-postgres` through the public CLI with those exact project names.
2. Keep the checked-in trees generator-owned: no hand-maintained content, no build outputs, no `.env` secrets, and no extra S04 public-surface rewiring.
3. Confirm the mode-specific files remain intact (`tests/storage.test.mpl` only on SQLite; `work.mpl`, `.env.example`, and the migration file only on Postgres).
4. Rerun the materializer in check mode so the committed examples are exactly reproducible.

## Must-Haves

- [ ] `examples/todo-sqlite` and `examples/todo-postgres` exist as checked-in scaffold output, not hand-maintained variants.
- [ ] The checked-in trees use the exact project names that future docs can reference directly.
- [ ] No build artifacts, secrets, or S04 doc rewires land with the example trees.

## Inputs

- `scripts/tests/verify-m049-s03-materialize-examples.mjs` — the repo-owned regeneration/check seam created in T01.
- `compiler/mesh-pkg/src/scaffold.rs` — scaffold source of truth that the committed example trees must mirror.

## Expected Output

- `examples/todo-sqlite/mesh.toml` — checked-in SQLite example manifest generated with the final public contract.
- `examples/todo-sqlite/README.md` — checked-in SQLite example README with explicit local-only guidance.
- `examples/todo-sqlite/tests/storage.test.mpl` — SQLite-only generated package test proving the local scaffold tree stayed intact.
- `examples/todo-postgres/mesh.toml` — checked-in Postgres example manifest generated from the public scaffold.
- `examples/todo-postgres/README.md` — checked-in Postgres example README preserving the serious shared/starter guidance.
- `examples/todo-postgres/migrations/20260402120000_create_todos.mpl` — Postgres-only generated migration proving the example is the migration-first scaffold output.

## Verification

- `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`
