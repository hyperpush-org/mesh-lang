# S01: Generated Postgres starter owns staged deploy truth

**Goal:** Make the generated Postgres Todo starter own a truthful staged deploy bundle and local production-like replay so the serious starter path is starter-generated, starter-tested, and ready for later hosted-chain integration.
**Demo:** After this: Generate a fresh Postgres Todo starter, stage a deploy bundle outside the source tree, run the staged artifact against PostgreSQL, exercise CRUD plus `meshc cluster` inspection against that running starter, and retain starter-owned evidence.

## Tasks
- [x] **T01: Generated Postgres starter-owned staged deploy assets and regenerated examples/todo-postgres from the updated scaffold.** — ## Description

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
  - Estimate: 1 context window
  - Files: compiler/mesh-pkg/src/scaffold.rs, scripts/tests/verify-m049-s03-materialize-examples.mjs, compiler/meshc/tests/e2e_m049_s03.rs, compiler/meshc/tests/support/m049_todo_examples.rs, examples/todo-postgres/README.md, examples/todo-postgres/scripts/stage-deploy.sh, examples/todo-postgres/scripts/apply-deploy-migrations.sh, examples/todo-postgres/scripts/deploy-smoke.sh, examples/todo-postgres/deploy/todo-postgres.up.sql
  - Verify: cargo test -p mesh-pkg m049_s01_postgres_scaffold_ -- --nocapture && cargo test -p meshc --test e2e_m049_s03 -- --nocapture && node scripts/tests/verify-m049-s03-materialize-examples.mjs --check
- [x] **T02: Added a staged Postgres starter deploy harness and e2e rail for external bundle boot, CRUD, and cluster inspection.** — ## Description

Add a starter-specific staged deploy proof harness that materializes the generated Postgres starter, stages a deploy bundle outside the source tree, applies the staged schema artifact, boots the staged binary against PostgreSQL, exercises CRUD, and inspects runtime-owned cluster state through Mesh CLI surfaces. Reuse M049 Postgres starter helpers where they still fit, but keep staged-bundle logic isolated from the source-tree runtime rail. Keep the core proof Fly-independent; if a remote Postgres host is needed during validation, provision a temporary Fly-managed database and destroy it before the task closes.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| PostgreSQL provisioning | Fail the test with the isolated DB/bundle artifacts preserved and stop before debugging starter code. | Treat slow DB creation/migration as infrastructure failure; keep the bundle and DB metadata artifacts for inspection. | Fail closed if DB metadata or SQL-apply output is malformed instead of silently retrying. |
| staged apply/smoke scripts | Stop on the first non-zero exit, retain stdout/stderr/meta logs, and inspect bundle contents before changing the harness. | Treat a hung apply/smoke step as a script contract bug; kill the staged process and retain timeout markers. | Fail closed if JSON/health output is malformed or missing required fields. |
| `meshc cluster` inspection commands | Treat command failure as proof-surface drift and archive the command output next to the running starter logs. | Time-box polling and preserve the last status/continuity/diagnostics snapshot for debugging. | Fail closed if returned JSON cannot be parsed into the starter-owned runtime truth contract. |

## Load Profile

- **Shared resources**: one isolated Postgres database, one staged bundle directory outside the repo, one running staged starter process, and one cluster listener port.
- **Per-operation cost**: one staged build, one staged SQL apply, one runtime boot, CRUD requests, and repeated `meshc cluster` polls.
- **10x breakpoint**: DB/provisioning latency and repeated status/continuity polling, not application data volume.

## Negative Tests

- **Malformed inputs**: missing `DATABASE_URL`, malformed `BASE_URL`, invalid bundle path, and malformed todo payloads.
- **Error paths**: unmigrated or failed SQL apply, starter boot before readiness, cluster CLI against a non-ready node, and secret leakage into retained artifacts.
- **Boundary conditions**: empty todo list before first create, missing todo ID, malformed UUID, and continuity list with only the runtime-owned startup record.

## Steps

1. Add a starter-specific support module under `compiler/meshc/tests/support/` for staged bundle creation, redacted artifact capture, cluster inspection helpers, and teardown.
2. Add `compiler/meshc/tests/e2e_m053_s01.rs` with a real staged deploy happy-path replay: build starter, stage bundle outside the repo, apply SQL, boot staged binary in clustered mode, exercise `/health` + CRUD, and inspect `meshc cluster status|continuity|diagnostics`.
3. Cover at least one fail-closed path in the same target (for example bad env, malformed bundle path, or malformed cluster output handling) so the deploy proof does not only validate the happy path.
4. Retain the staged bundle pointer, redacted DB metadata, runtime logs, HTTP snapshots, and cluster JSON snapshots under `.tmp/m053-s01/` for later slices.

## Must-Haves

- [ ] The staged Postgres starter bundle is built outside the source tree and stays source-clean.
- [ ] The staged binary proves `/health`, real CRUD, and `meshc cluster` inspection against the running starter.
- [ ] Retained artifacts are redacted and point to the staged bundle/evidence needed for later hosted-chain integration.

## Verification

- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_m053_s01 -- --nocapture`
- Confirm the target runs more than 0 tests and leaves a retained `.tmp/m053-s01/...` artifact bundle with staged runtime + cluster JSON evidence.
  - Estimate: 1 context window
  - Files: compiler/meshc/tests/support/mod.rs, compiler/meshc/tests/support/m053_todo_postgres_deploy.rs, compiler/meshc/tests/e2e_m053_s01.rs, compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs, examples/todo-postgres/scripts/stage-deploy.sh, examples/todo-postgres/scripts/apply-deploy-migrations.sh, examples/todo-postgres/scripts/deploy-smoke.sh, examples/todo-postgres/deploy/todo-postgres.up.sql
  - Verify: DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_m053_s01 -- --nocapture
- [x] **T03: Publish a retained starter deploy verifier surface for later hosted-chain wiring** — ## Description

Wrap the starter deploy proof into one fail-closed verifier surface that later hosted-chain work can call without re-implementing starter logic. The executor should replay the generator/example parity rail and the staged deploy e2e, copy the retained proof bundle under `.tmp/m053-s01/verify/`, and publish pointer/status/phase markers so S03 can wire this starter-owned command into the normal release/deploy chain alongside packages-site checks.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| prerequisite test commands | Fail the wrapper immediately and preserve the command log instead of continuing with stale evidence. | Kill the phase, record timeout context, and leave the last successful artifact pointer intact for debugging. | Fail closed if a named test filter runs 0 tests or emits malformed output. |
| retained bundle copy/pointer publishing | Stop if the source bundle is missing or under the repo root and log the bundle-shape mismatch. | Treat a hung copy/archive step as artifact corruption and abort the wrapper. | Fail closed if pointer, manifest, or status files do not match the retained bundle directory. |

## Load Profile

- **Shared resources**: `.tmp/m053-s01/verify/`, copied retained bundle directories, and prerequisite test logs.
- **Per-operation cost**: one scripted replay of prerequisite commands plus one retained bundle copy/shape check.
- **10x breakpoint**: artifact copy size and repeated full replays, not runtime throughput.

## Negative Tests

- **Malformed inputs**: missing `DATABASE_URL`, missing retained bundle path, malformed phase report, or bundle pointer under the repo root.
- **Error paths**: prerequisite command failure, 0-test filter, missing copied artifact, or leaked secret in retained logs.
- **Boundary conditions**: rerun with an existing `.tmp/m053-s01/verify/` tree, stale `latest-proof-bundle.txt`, and partial retained bundle copies.

## Steps

1. Create `scripts/verify-m053-s01.sh` to run the Postgres scaffold rail, example-parity rail, and staged deploy e2e in a fixed order with explicit phase logging and 0-test guards where needed.
2. Copy the retained staged-deploy artifact bundle into `.tmp/m053-s01/verify/`, publish `status.txt`, `current-phase.txt`, `phase-report.txt`, and `latest-proof-bundle.txt`, and fail closed if the copied bundle shape or redaction markers drift.
3. Keep the wrapper scoped to the starter deploy truth; do not pull packages-site or Fly-public-docs work forward from later slices.
4. Leave the verify surface ready for later CI/deploy-chain wiring by making the script the single starter-owned command S03 can call.

## Must-Haves

- [ ] `bash scripts/verify-m053-s01.sh` is the single retained starter deploy proof surface for S01.
- [ ] The verifier fail-closes on prerequisite failures, 0-test filters, missing bundle pointers, or redaction drift.
- [ ] `.tmp/m053-s01/verify/` contains stable phase/status/pointer files and a copied retained proof bundle for downstream slices.

## Verification

- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash scripts/verify-m053-s01.sh`
- Confirm `.tmp/m053-s01/verify/status.txt` is `ok`, `current-phase.txt` is `complete`, and `latest-proof-bundle.txt` points at the copied retained bundle.
  - Estimate: 1 context window
  - Files: scripts/verify-m053-s01.sh, compiler/meshc/tests/e2e_m053_s01.rs, scripts/tests/verify-m049-s03-materialize-examples.mjs
  - Verify: DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash scripts/verify-m053-s01.sh
