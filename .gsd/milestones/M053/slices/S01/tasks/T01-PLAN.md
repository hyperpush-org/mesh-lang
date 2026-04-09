---
estimated_steps: 4
estimated_files: 9
skills_used:
  - bash-scripting
  - test
---

# T01: Generate Postgres starter deploy-kit assets and keep example parity honest

**Slice:** S01 — Generated Postgres starter owns staged deploy truth
**Milestone:** M053

## Description

Teach the generated Postgres Todo starter to emit a real staged deploy kit instead of stopping at local runtime + Docker guidance. The executor should add starter-owned deploy assets, update the Postgres README contract, and refresh the generator/example parity rails together so a fresh scaffold and the committed `examples/todo-postgres/` tree stay identical.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `meshc init --template todo-api --db postgres` generation | Fail the scaffold/unit rail and compare the generated temp tree against the committed example before editing more files. | Treat a hung generation/materialization run as a generator regression; stop and inspect the temp tree instead of hand-editing examples. | Fail closed if the generated tree omits any required staged-deploy file or README marker. |
| example materializer parity | Stop on the first missing/extra/changed file report and refresh the generator + committed example in the same task. | Treat a stuck parity run as a temp-tree or file-walk bug; do not bypass it with manual copies. | Fail closed if manifests or required-path definitions drift from the generated starter tree. |

## Load Profile

- **Shared resources**: `examples/todo-postgres/`, scaffold temp trees, and the materializer temp session directory.
- **Per-operation cost**: one scaffold generation, one example-tree diff, and one committed example refresh.
- **10x breakpoint**: temp-file churn and parity diff noise, not runtime load.

## Negative Tests

- **Malformed inputs**: missing bundle dir arg, non-directory bundle target, or missing generated deploy file markers.
- **Error paths**: generator/example drift, stale README markers, or Fly/SQLite wording leaking into the Postgres starter contract.
- **Boundary conditions**: partial example tree, missing required staged file, or committed example containing extra hand-edited files.

## Steps

1. Extend `compiler/mesh-pkg/src/scaffold.rs` so the Postgres starter generates starter-owned staged deploy assets (`scripts/` + `deploy/`) and README/runbook text that stays portable-first.
2. Update scaffold unit assertions to prove the new file set, staged-deploy markers, and the continued absence of Fly-first or SQLite-only drift in the Postgres starter.
3. Refresh the example materializer definitions and the committed `examples/todo-postgres/` tree so public example parity still comes from generation instead of hand edits.
4. Keep the SQLite starter untouched and explicitly local-only while the Postgres starter grows the serious deploy-kit surface.

## Must-Haves

- [ ] The generated Postgres starter includes starter-owned stage/apply/smoke assets plus a deploy SQL artifact.
- [ ] The generated Postgres README describes the staged bundle flow, keeps Fly as a non-contract concern, and does not reintroduce SQLite/local-only wording.
- [ ] `examples/todo-postgres/` and the materializer contract both match the new generated file set.

## Verification

- `cargo test -p mesh-pkg m049_s01_postgres_scaffold_ -- --nocapture`
- `cargo test -p meshc --test e2e_m049_s03 -- --nocapture`
- `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`

## Observability Impact

- Signals added/changed: phase-tagged starter deploy-script markers (`[stage-deploy]`, `[deploy-apply]`, `[deploy-smoke]`) and README contract markers for staged deploy.
- How a future agent inspects this: compare a fresh scaffold to `examples/todo-postgres/` with the materializer check and scaffold unit tests.
- Failure state exposed: missing required generated paths, stale README wording, and broken deploy-script markers fail through scaffold/materializer rails.

## Inputs

- `compiler/mesh-pkg/src/scaffold.rs` — current Postgres starter generator and scaffold assertions.
- `scripts/tests/verify-m049-s03-materialize-examples.mjs` — generator-owned example parity contract.
- `compiler/meshc/tests/e2e_m049_s03.rs` — example parity and build/test replay.
- `compiler/meshc/tests/support/m049_todo_examples.rs` — Rust helpers that mirror the example file-set contract.
- `examples/todo-postgres/README.md` — committed public example surface that must stay generator-owned.

## Expected Output

- `compiler/mesh-pkg/src/scaffold.rs` — Postgres starter generation emits staged deploy assets and updated README text.
- `scripts/tests/verify-m049-s03-materialize-examples.mjs` — required-path list includes the new Postgres deploy-kit files.
- `compiler/meshc/tests/e2e_m049_s03.rs` — parity/build-test rails stay aligned with the new example tree.
- `compiler/meshc/tests/support/m049_todo_examples.rs` — support assertions understand the expanded Postgres example shape.
- `examples/todo-postgres/README.md` — committed example README documents the staged bundle contract.
- `examples/todo-postgres/scripts/stage-deploy.sh` — committed starter bundle staging script.
- `examples/todo-postgres/scripts/apply-deploy-migrations.sh` — committed starter staged SQL apply script.
- `examples/todo-postgres/scripts/deploy-smoke.sh` — committed starter deploy smoke script.
- `examples/todo-postgres/deploy/todo-postgres.up.sql` — committed starter deploy SQL artifact.
