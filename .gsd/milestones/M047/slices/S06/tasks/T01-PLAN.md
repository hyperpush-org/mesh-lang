---
estimated_steps: 4
estimated_files: 4
skills_used:
  - debug-like-expert
  - SQLite Database Expert
  - test
---

# T01: Repair the built-package SQLite execute seam and add a focused AOT regression rail

**Slice:** S06 — Docs, migration, and assembled proof closeout
**Milestone:** M047

## Description

The red blocker below S06 is not Todo-specific: a Mesh package built with `meshc build` can open SQLite but fails on `Sqlite.execute(...)` with `bad parameter or other API misuse`. Fix that AOT/runtime seam first and add a dedicated built-package regression so later native and Docker Todo proof is trustworthy.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| built-package SQLite lowering between `compiler/mesh-codegen` and `compiler/mesh-rt` | Fail the focused regression with retained binary/log artifacts; do not hide the issue behind Todo-specific retries or docs changes. | N/A — this is a local build/run path, but the regression test should keep bounded process waits. | Treat malformed params/result payloads as a failing runtime/ABI contract, not as successful zero-row writes. |
| runtime SQLite execute/query semantics | Keep `Sqlite.execute` and `Sqlite.query` consistent for empty/non-empty param lists or fail a named test; do not silently special-case only the scaffold SQL strings. | N/A | Malformed SQLite error text is still failure evidence; do not downgrade to generic success. |

## Load Profile

- **Shared resources**: `mesh_sqlite_execute`, AOT lowering, and retained `.tmp` test artifact directories.
- **Per-operation cost**: one `meshc build` plus one emitted-binary execution in the focused regression.
- **10x breakpoint**: pointer/ABI drift and malformed result payloads fail long before throughput matters.

## Negative Tests

- **Malformed inputs**: empty param list `[]`, non-empty param list with placeholders, and placeholder-count mismatches in the focused regression fixture.
- **Error paths**: a built binary that can `Sqlite.open` but not `Sqlite.execute` must fail with a named regression instead of hanging or being masked by Todo-specific retries.
- **Boundary conditions**: `CREATE TABLE`, `INSERT`, and read-back/terminal success all work in a built binary, not just via `compile_and_run`.

## Steps

1. Compare the green in-process `compile_and_run` SQLite path with a minimal built-package repro to identify where params/result pointers diverge in AOT lowering.
2. Repair the lowering/runtime seam in `compiler/mesh-codegen` and/or `compiler/mesh-rt` so `mesh_sqlite_execute` handles built binaries with empty and non-empty param lists correctly.
3. Add a focused `compiler/meshc/tests/e2e_sqlite_built_package.rs` regression that builds a tiny package, runs the emitted binary, and asserts `CREATE TABLE`, `INSERT`, and read-back/terminal success while retaining artifacts on failure.
4. Keep the failure surface narrow and honest: do not add Todo-specific workarounds or silent fallbacks.

## Must-Haves

- [ ] A package emitted by `meshc build` can execute `Sqlite.execute(db, ..., [])` and parameterized writes successfully.
- [ ] The fix is covered by a focused built-package regression, not only by the Todo scaffold rail.
- [ ] The task does not hide the issue behind scaffold-only retries, docs edits, or generic “bad parameter” suppression.

## Verification

- `cargo test -p meshc --test e2e_sqlite_built_package -- --nocapture`

## Observability Impact

- Signals added/changed: focused AOT SQLite regression artifacts and any preserved runtime SQLite error text.
- How a future agent inspects this: rerun the dedicated test file and inspect the retained built binary/log artifacts under `.tmp`.
- Failure state exposed: built-package pointer/ABI drift fails as a named regression instead of first surfacing deep inside the Todo scaffold.

## Inputs

- `compiler/mesh-rt/src/db/sqlite.rs` — runtime SQLite execute/query ABI implementation.
- `compiler/mesh-codegen/src/mir/lower.rs` — AOT lowering path that emits SQLite intrinsic calls.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — intrinsic signatures for SQLite execute/query.
- `compiler/meshc/tests/e2e.rs` — existing in-process SQLite coverage that currently misses the built-package seam.

## Expected Output

- `compiler/mesh-rt/src/db/sqlite.rs` — fixed execute path for built binaries.
- `compiler/mesh-codegen/src/mir/lower.rs` — corrected lowering/pointer handling for SQLite execute.
- `compiler/meshc/tests/e2e_sqlite_built_package.rs` — focused built-package regression with retained artifacts.
