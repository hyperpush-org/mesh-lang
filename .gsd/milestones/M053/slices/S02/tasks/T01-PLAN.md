---
estimated_steps: 4
estimated_files: 4
skills_used:
  - test
  - postgresql-database-engineering
  - debug-like-expert
---

# T01: Extend the staged Postgres starter harness for two-node clustered replay

**Slice:** S02 — Generated Postgres starter proves clustered failover truth
**Milestone:** M053

## Description

Extend the S01 staged starter helper from a single-node deploy replay into a two-node clustered harness that can boot primary and standby processes from the same staged bundle, talk to one shared PostgreSQL database, and archive dual-node operator/HTTP evidence without reintroducing app-owned delay logic into the starter source. This task should create the reusable helper seam that the destructive failover rail will consume.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| staged starter runtime env (`DATABASE_URL`, cluster cookie/name/seed, startup-delay env) | Fail the helper-contract test immediately and archive the rejected config instead of guessing at defaults. | Treat hung startup as helper/runtime drift; stop the processes, retain logs, and inspect the last readiness snapshot. | Fail closed if env-derived node names, operator JSON, or retained metadata cannot be parsed into the expected cluster contract. |
| shared PostgreSQL provisioning and migrations | Stop on the first DB/bootstrap failure and preserve the isolated DB + bundle artifacts for the later e2e rail. | Time-box DB/bootstrap setup and retain the last migration/runtime logs instead of retrying blindly. | Fail closed if staged apply output or DB metadata is malformed instead of normalizing it. |
| reused operator waiters from `m046_route_free` / route-based request-key helpers | Stop when a reused waiter no longer matches the staged Postgres starter contract; do not paper over it with ad-hoc sleeps. | Treat polling timeouts as proof-surface drift and retain the last status/continuity/diagnostics JSON. | Fail closed if returned JSON omits required request-key, role, epoch, or continuity fields. |

## Load Profile

- **Shared resources**: one shared PostgreSQL database, two staged starter processes, cluster listener ports, and retained artifact directories under `.tmp/m053-s02/`.
- **Per-operation cost**: one staged bundle boot per node, repeated `meshc cluster` polls, and shared HTTP request snapshots.
- **10x breakpoint**: port/config drift and operator polling noise, not data volume.

## Negative Tests

- **Malformed inputs**: missing `DATABASE_URL`, malformed node name/seed, invalid staged bundle pointer, and invalid startup-delay configuration.
- **Error paths**: bootstrap before readiness, cluster CLI against a non-ready node, malformed operator JSON, or redaction leakage in retained artifacts.
- **Boundary conditions**: mirrored startup record absent, primary-owned startup selection mismatch, and continuity lists that only contain the startup runtime before route traffic begins.

## Steps

1. Extend `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` and `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs` so the staged Postgres starter can derive paired primary/standby runtime configs, including optional `MESH_STARTUP_WORK_DELAY_MS`, shared `DATABASE_URL`, and starter-owned artifact paths.
2. Reuse `compiler/meshc/tests/support/m046_route_free.rs` cluster waiters and the older route/request-key helpers to add dual-node staged spawn/stop, membership convergence, continuity snapshot, diagnostics snapshot, and per-node HTTP/archive helpers; keep the default operator path host-native and staged-bundle-first rather than Docker-first.
3. Add helper-contract coverage in `compiler/meshc/tests/e2e_m053_s02.rs` that proves the new helper chooses a primary-owned startup request, keeps the starter README bounded, and rejects malformed staged bundle / cluster env states fail-closed.
4. Keep SQLite untouched and keep the starter source clean: the helper may use the runtime-owned startup delay seam, but it must not add `Timer.sleep(...)` or other app-owned failover glue to `examples/todo-postgres/work.mpl`.

## Must-Haves

- [ ] The staged Postgres starter helper can boot and inspect a two-node cluster from one generated staged bundle and one shared PostgreSQL database.
- [ ] The helper exposes retained per-node status/continuity/diagnostics snapshots plus HTTP/request-key artifacts that a later destructive replay can reuse.
- [ ] The helper contract stays generated-starter-first, keeps the starter README bounded, and does not widen SQLite or starter-source responsibilities.

## Inputs

- `compiler/meshc/tests/support/mod.rs`
- `compiler/meshc/tests/support/m046_route_free.rs`
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`
- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs`
- `examples/todo-postgres/README.md`
- `examples/todo-postgres/work.mpl`

## Expected Output

- `compiler/meshc/tests/support/mod.rs`
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`
- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs`
- `compiler/meshc/tests/e2e_m053_s02.rs`

## Verification

DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_m053_s02 m053_s02_staged_postgres_helper_ -- --nocapture

## Observability Impact

- Signals added/changed: dual-node staged runtime logs, paired status/continuity/diagnostics JSON snapshots, request-key selection metadata, and helper-level timeout artifacts.
- How a future agent inspects this: replay the helper-contract filter in `compiler/meshc/tests/e2e_m053_s02.rs`, then open the latest `.tmp/m053-s02/...` helper artifact directory.
- Failure state exposed: rejected env/config, last readiness snapshot, missing mirrored startup record, and malformed operator JSON evidence.
