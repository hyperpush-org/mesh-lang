---
estimated_steps: 4
estimated_files: 3
skills_used:
  - rust-best-practices
  - test
---

# T03: Add a slice-owned parity, test, and build rail for the committed examples

Prove the checked-in examples are not decorative. Add a dedicated `meshc` integration target that regenerates fresh examples through the public CLI, compares them byte-for-byte against the committed trees, runs `meshc test` on both examples, and builds both examples to `.tmp` outputs. Reuse the T01 materializer/check seam instead of copying generation logic again.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Materializer script plus public `meshc init` | Fail before any build/test claims and archive generated trees plus diff output. | Bound generation and command phases with retained logs. | Reject partial generation output instead of diffing nonsense. |
| Committed example trees | Fail with named missing/extra/changed file reports and retain committed-vs-generated snapshots. | N/A — local filesystem comparison only. | Reject silent normalization or ignored diffs. |
| `meshc test` / `meshc build --output` on the example dirs | Archive stdout/stderr and stop on the first failing example. | Bound each phase and retain timeout artifacts. | Reject wrong pass counts, missing binaries, or accidental repo-tree build outputs. |

## Load Profile

- **Shared resources**: temp workspaces, `.tmp/m049-s03` artifacts, the local `meshc` binary, and the static runtime archive used by `meshc build`.
- **Per-operation cost**: two fresh generations, two `meshc test` runs, and two `meshc build --output` runs.
- **10x breakpoint**: compile/build time and temp artifact churn appear before CPU; no live DB or network should be required.

## Negative Tests

- **Malformed inputs**: missing example root, extra hand-edited file, unexpected project name, or a missing Postgres migration / SQLite storage test.
- **Error paths**: check-mode diff, `meshc test` not reporting the expected pass count, or `meshc build --output` not emitting the requested binary.
- **Boundary conditions**: SQLite/Postgres divergent file sets stay intentional, and both examples must build from tracked dirs without writing outputs back into the repo tree.

## Steps

1. Add `compiler/meshc/tests/support/m049_todo_examples.rs` with helpers to run the materializer/check script, archive fresh generated trees and diff manifests, and reuse the existing scaffold helpers for `meshc test` / `meshc build --output` against the committed examples.
2. Register the helper in `compiler/meshc/tests/support/mod.rs`.
3. Add `compiler/meshc/tests/e2e_m049_s03.rs` with named tests covering exact SQLite/Postgres parity plus `meshc test` and `meshc build --output` for both `examples/` projects.
4. Retain `.tmp/m049-s03/...` artifacts containing generated trees, diff output, test logs, and build metadata so S04/S05 can debug example drift quickly.

## Must-Haves

- [ ] `cargo test -p meshc --test e2e_m049_s03 -- --nocapture` fails on any committed/generated drift in either example tree.
- [ ] The same target proves `meshc test` reports the expected generated-package pass counts for both examples.
- [ ] The same target proves both examples build through `meshc build --output <tmp>` without polluting the repo tree.

## Inputs

- `scripts/tests/verify-m049-s03-materialize-examples.mjs` — single source of truth for regenerating and diffing the example trees.
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` — existing helper seam for build/test artifact capture on scaffolded Postgres apps.
- `compiler/meshc/tests/support/m049_todo_sqlite_scaffold.rs` — existing helper seam for build/test artifact capture on scaffolded SQLite apps.
- `examples/todo-sqlite/mesh.toml` — committed SQLite example root to verify and build.
- `examples/todo-postgres/mesh.toml` — committed Postgres example root to verify and build.

## Expected Output

- `compiler/meshc/tests/support/m049_todo_examples.rs` — shared helper for materializer invocation, diff artifact capture, and example build/test reuse.
- `compiler/meshc/tests/support/mod.rs` — support registry updated to expose the new examples helper.
- `compiler/meshc/tests/e2e_m049_s03.rs` — slice-owned parity/build/test rail for both committed examples.

## Verification

- `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`
- `cargo test -p meshc --test e2e_m049_s03 -- --nocapture`

## Observability Impact

- Signals added/changed: `.tmp/m049-s03/...` stores generated trees, diff reports/manifests, `meshc test` stdout/stderr, build metadata, and binary output paths per example.
- How a future agent inspects this: rerun `cargo test -p meshc --test e2e_m049_s03 -- --nocapture` and inspect `.tmp/m049-s03/<scenario>/`.
- Failure state exposed: per-example drift, wrong pass counts, and build-output path issues are preserved as named artifacts instead of a single failing assert.
