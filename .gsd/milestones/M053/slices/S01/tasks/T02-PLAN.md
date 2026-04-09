---
estimated_steps: 4
estimated_files: 8
skills_used:
  - test
  - bash-scripting
  - flyio-cli-public
---

# T02: Prove the staged Postgres starter bundle boots, serves CRUD, and answers cluster inspection

**Slice:** S01 — Generated Postgres starter owns staged deploy truth
**Milestone:** M053

## Description

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

## Observability Impact

- Signals added/changed: retained staged runtime stdout/stderr/meta logs, redacted DB metadata, HTTP response snapshots, and cluster status/continuity/diagnostics JSON.
- How a future agent inspects this: open the latest `.tmp/m053-s01/...` bundle, then replay `cargo test -p meshc --test e2e_m053_s01 -- --nocapture`.
- Failure state exposed: staged apply/smoke exit codes, readiness timeouts, last cluster snapshot, and secret-redaction failures.

## Inputs

- `compiler/meshc/tests/support/mod.rs` — test support module exports that need the new staged deploy harness.
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` — existing Postgres starter runtime helpers and redaction utilities.
- `compiler/meshc/tests/e2e_m049_s01.rs` — current Postgres source-tree runtime truth rail to reuse without duplicating logic.
- `examples/todo-postgres/scripts/stage-deploy.sh` — generated starter bundle staging script from T01.
- `examples/todo-postgres/scripts/apply-deploy-migrations.sh` — generated starter staged SQL apply script from T01.
- `examples/todo-postgres/scripts/deploy-smoke.sh` — generated starter deploy smoke script from T01.
- `examples/todo-postgres/deploy/todo-postgres.up.sql` — generated starter deploy SQL artifact from T01.

## Expected Output

- `compiler/meshc/tests/support/mod.rs` — exports the new M053 staged deploy support module.
- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs` — starter-specific staged deploy/test harness helpers.
- `compiler/meshc/tests/e2e_m053_s01.rs` — real staged deploy replay with CRUD, cluster inspection, fail-closed paths, and retained artifacts.
