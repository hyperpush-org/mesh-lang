# S01: Generated Postgres starter owns staged deploy truth — UAT

**Milestone:** M053
**Written:** 2026-04-05T18:56:02.784Z

# S01: Generated Postgres starter owns staged deploy truth — UAT

**Milestone:** M053
**Written:** 2026-04-05

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: This slice changes a generated starter, a staged deploy bundle, runtime startup behavior, and a retained verifier surface. The truthful acceptance path therefore needs both artifact inspection and a live PostgreSQL-backed replay of the staged binary.

## Preconditions

- Docker is available locally.
- A disposable PostgreSQL instance is reachable, for example:
  - `docker run --rm --name mesh-m053-s01-pg -e POSTGRES_PASSWORD=postgres -e POSTGRES_USER=postgres -e POSTGRES_DB=postgres -p 61918:5432 postgres:16`
- `DATABASE_URL` points at that instance, for example:
  - `export DATABASE_URL=postgres://postgres:postgres@127.0.0.1:61918/postgres`
- The repo is built from the current working tree.

## Smoke Test

Run the slice-owned wrapper:

1. `DATABASE_URL=$DATABASE_URL bash scripts/verify-m053-s01.sh`
2. **Expected:** the command exits 0, `.tmp/m053-s01/verify/status.txt` contains `ok`, `.tmp/m053-s01/verify/current-phase.txt` contains `complete`, and `.tmp/m053-s01/verify/latest-proof-bundle.txt` points at a retained proof bundle under `.tmp/m053-s01/proof-bundles/`.

## Test Cases

### 1. Generated Postgres starter still matches the public scaffold contract

1. Run `cargo test -p mesh-pkg m049_s01_postgres_scaffold_ -- --nocapture`.
2. Run `cargo test -p meshc --test e2e_m049_s03 -- --nocapture`.
3. Run `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`.
4. **Expected:** the scaffold emits the Postgres staged deploy file set, `examples/todo-postgres/` stays in parity with the public CLI output, and the example/test/build rails stay green.

### 2. Stage a deploy bundle outside the repo and inspect the staged layout

1. Generate a fresh starter with `meshc init --template todo-api --db postgres todo-postgres` in a temp workspace.
2. Run `bash scripts/stage-deploy.sh <bundle-dir>` from that generated project.
3. Inspect `<bundle-dir>`.
4. **Expected:** `<bundle-dir>` contains exactly `todo-postgres`, `todo-postgres.up.sql`, `apply-deploy-migrations.sh`, and `deploy-smoke.sh`; the generated source tree does not gain an in-place binary; the stage script logs `[stage-deploy] staged layout` and `[stage-deploy] bundle ready dir=...`.

### 3. Apply staged schema, boot the staged binary, and inspect runtime-owned cluster truth

1. Run `DATABASE_URL=$DATABASE_URL bash <bundle-dir>/apply-deploy-migrations.sh <bundle-dir>/todo-postgres.up.sql`.
2. Start the staged binary with clustered env:
   - `DATABASE_URL=$DATABASE_URL PORT=<port> TODO_RATE_LIMIT_WINDOW_SECONDS=60 TODO_RATE_LIMIT_MAX_REQUESTS=20 MESH_CLUSTER_COOKIE=dev-cookie MESH_NODE_NAME=todo-postgres@127.0.0.1:<cluster-port> MESH_DISCOVERY_SEED=127.0.0.1 MESH_CLUSTER_PORT=<cluster-port> MESH_CONTINUITY_ROLE=primary MESH_CONTINUITY_PROMOTION_EPOCH=0 <bundle-dir>/todo-postgres`
3. Query `GET /health`.
4. Run:
   - `meshc cluster status todo-postgres@127.0.0.1:<cluster-port> --json`
   - `meshc cluster continuity todo-postgres@127.0.0.1:<cluster-port> --json`
   - `meshc cluster diagnostics todo-postgres@127.0.0.1:<cluster-port> --json`
5. **Expected:** `/health` returns `status=ok`, `db_backend=postgres`, `migration_strategy=meshc migrate`, and `clustered_handler=Work.sync_todos` without leaking `DATABASE_URL`; cluster status reports the node as `primary`; cluster continuity includes a completed `startup::Work.sync_todos` record plus its route-owned records after traffic; diagnostics include `startup_trigger` and `startup_completed`.

### 4. Exercise real CRUD through the staged smoke helper

1. With the staged binary still running, run `DATABASE_URL=$DATABASE_URL PORT=<port> BASE_URL=http://127.0.0.1:<port> bash <bundle-dir>/deploy-smoke.sh`.
2. Query `GET /todos` before and after the smoke run.
3. **Expected:** the smoke helper logs a green health check, create/toggle/delete steps, and `CRUD smoke passed`; `GET /todos` is empty before the smoke and empty again afterward; retained artifacts include `todos-empty.json`, `todos-after-smoke.json`, `cluster-continuity-route-record.json`, and `cluster-diagnostics.json`.

## Edge Cases

### Invalid staged bundle path fails closed

1. Point `bash scripts/stage-deploy.sh` at a file instead of a directory.
2. **Expected:** the script exits non-zero with `[stage-deploy] bundle path exists but is not a directory:` and the retained invalid-path artifact bundle contains `stage-deploy-invalid-path.stdout.log` / `.stderr.log`.

### Missing `DATABASE_URL` or malformed `BASE_URL` fails closed

1. Run the staged apply helper without `DATABASE_URL`.
2. Run the staged smoke helper with `BASE_URL=127.0.0.1:8080`.
3. Run `meshc cluster status <node> --json` against a non-ready node.
4. **Expected:** each surface fails non-zero with its explicit error, and the retained fail-closed artifact bundle contains `deploy-apply-missing-database-url.*`, `deploy-smoke-malformed-base-url.*`, and `cluster-status-not-ready.log`.

## Failure Signals

- `scripts/verify-m053-s01.sh` exits non-zero or leaves `status.txt=failed`.
- `.tmp/m053-s01/verify/latest-proof-bundle.txt` is empty, missing, or points somewhere outside `.tmp/m053-s01/proof-bundles/`.
- The staged bundle contains repo source files, omits the staged SQL/script set, or writes a binary back into the source tree.
- `/health` leaks `DATABASE_URL`, omits `clustered_handler`, or never becomes ready.
- `meshc cluster continuity --json` omits the `startup::Work.sync_todos` record or the route continuity record after `GET /todos` traffic.
- Retained bundle-shape or redaction scans fail.

## Requirements Proved By This UAT

- none — this slice advances R122 by proving the serious starter’s staged deploy seam and retained operator evidence, but S02 still owns the full clustered deploy/failover validation.

## Not Proven By This UAT

- Multi-node failover, rejoin, or owner-loss recovery for the generated Postgres starter.
- Hosted CI/deploy-chain integration and packages-website deployment evidence.
- Public docs/Fly contract alignment for the starter and packages story.

## Notes for Tester

- The canonical retained evidence surface is `.tmp/m053-s01/verify/latest-proof-bundle.txt`.
- The wrapper already copies the fresh proof bundle and scans it for `DATABASE_URL` leakage; inspect the copied bundle first if a later slice reports starter deploy drift.
- When the runtime changes, rerun the wrapper rather than reusing an older staged bundle so `run_stage_deploy_script(...)` can refresh `libmesh_rt.a` before the shell-out build.
