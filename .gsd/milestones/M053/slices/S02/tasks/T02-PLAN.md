---
estimated_steps: 4
estimated_files: 4
skills_used:
  - test
  - postgresql-database-engineering
  - debug-like-expert
---

# T02: Add the authoritative generated Postgres starter failover e2e rail

**Slice:** S02 — Generated Postgres starter proves clustered failover truth
**Milestone:** M053

## Description

Add the generated-starter-first two-node failover rail that turns the new helper into the actual S02 proof. The executor should generate a fresh Postgres starter, stage the deploy bundle outside the repo tree, apply the staged SQL, boot primary and standby against one shared database, prove real starter HTTP behavior, and then use the runtime-owned startup pending window to trigger owner loss and recovery with truthful operator artifacts.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| primary/standby cluster convergence | Stop on the first role/epoch/membership mismatch and preserve both node logs plus the last operator snapshots. | Treat stalled convergence or mirrored-state waits as runtime/helper regressions; archive the last status/continuity JSON rather than adding sleeps. | Fail closed if `meshc cluster` payloads omit required role, epoch, request-key, or replica-status fields. |
| runtime-owned startup pending window | Fail the rail if the selected startup record completes before the destructive step; keep the chosen node names, cluster port, and diagnostics bundle so the next agent can inspect the timing seam. | Time-box the pending-window wait and record the last diagnostics snapshot instead of looping forever. | Fail closed if diagnostics claim promotion/recovery without a matching continuity record transition. |
| real starter HTTP CRUD + clustered read route | Stop on the first unexpected status/body and preserve the exact request/response snapshots. | Treat slow responses as a starter/runtime regression and archive before/after HTTP state rather than masking it. | Fail closed if JSON bodies, todo IDs, or shared-state reads are malformed after failover. |

## Load Profile

- **Shared resources**: one staged binary reused by two nodes, one shared PostgreSQL database, real HTTP ports, and repeated operator polling during failover.
- **Per-operation cost**: full staged starter boot, CRUD seed traffic, clustered GET route proof, destructive failover, rejoin, and post-failover reads.
- **10x breakpoint**: the pending-window/failover timing seam and cluster polling churn, not todo volume.

## Negative Tests

- **Malformed inputs**: missing `DATABASE_URL`, bad todo payloads, malformed UUIDs, and stale-primary duplicate submits after rejoin.
- **Error paths**: owner finishes before kill, mirrored state never appears, operator JSON malformed, or promoted node never recovers the pending startup record.
- **Boundary conditions**: empty todo list before seeding, exactly one mirrored pending startup record, and post-rejoin stale-primary guard on the same request key.

## Steps

1. In `compiler/meshc/tests/e2e_m053_s02.rs`, generate a fresh Postgres starter, stage the bundle, apply staged migrations, and boot two staged processes with one shared cookie/seed/database while choosing the node/port shape that keeps the startup request primary-owned.
2. Seed shared state through the real starter routes, prove `GET /todos` continuity truth for `Api.Todos.handle_list_todos`, and assert the starter contract stays bounded by reading source/README markers instead of adding failover prose to the starter docs.
3. Set the runtime-owned startup delay seam, wait for a mirrored pending startup record, kill the owner, then prove `owner_lost`, `automatic_promotion`, `automatic_recovery`, stale-primary guard, and `fenced_rejoin` through `meshc cluster status|continuity|diagnostics`, per-node logs, and post-failover HTTP reads.
4. Retain a single clean bundle under `.tmp/m053-s02/` with scenario metadata, before/after HTTP snapshots, selected request keys, per-node operator JSON, and redacted logs that later verifier code can copy verbatim.

## Must-Haves

- [ ] The authoritative S02 Rust rail proves two-node staged Postgres starter CRUD/read behavior and shared-state continuity through real starter endpoints.
- [ ] The destructive proof uses the runtime-owned startup pending window rather than starter-source sleeps, and it shows truthful owner-loss/promotion/recovery/stale-primary/fenced-rejoin evidence.
- [ ] The proof remains generated-starter-first and keeps the starter README bounded instead of turning it into the failover contract.

## Inputs

- `compiler/meshc/tests/e2e_m046_s03.rs`
- `compiler/meshc/tests/support/m046_route_free.rs`
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`
- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs`
- `examples/todo-postgres/api/router.mpl`
- `examples/todo-postgres/work.mpl`
- `examples/todo-postgres/README.md`

## Expected Output

- `compiler/meshc/tests/e2e_m053_s02.rs`
- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs`
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`

## Verification

DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_m053_s02 -- --nocapture

## Observability Impact

- Signals added/changed: retained failover scenario metadata, before/after HTTP snapshots, per-node status/continuity/diagnostics JSON, and explicit promotion/recovery/fenced-rejoin log evidence.
- How a future agent inspects this: run `cargo test -p meshc --test e2e_m053_s02 -- --nocapture`, then follow the newest `.tmp/m053-s02/...` bundle into the saved operator/HTTP snapshots.
- Failure state exposed: selected request key, attempt ids, owner/replica role+epoch, last pending-window snapshot, post-failover stale-primary response, and rejoin diagnostics.
