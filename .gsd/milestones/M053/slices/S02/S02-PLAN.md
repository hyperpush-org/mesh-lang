# S02: Generated Postgres starter proves clustered failover truth

**Goal:** Extend the generated Postgres Todo starter from S01’s single-node staged deploy truth into a generated-starter-first two-node clustered failover proof that exercises real HTTP routes, real `meshc cluster` operator surfaces, and runtime-owned owner-loss recovery without widening SQLite’s explicitly local contract.
**Demo:** After this: Run the generated Postgres starter in a production-like clustered replay, hit its real endpoints, inspect `meshc cluster status|continuity|diagnostics`, trigger the named node-loss or failover path, and confirm the starter survives with truthful artifacts.

## Tasks
- [x] **T01: Added the two-node staged Postgres helper seam and the first M053/S02 helper-contract test target.** — ## Description

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
  - Estimate: 1 context window
  - Files: compiler/meshc/tests/support/mod.rs, compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs, compiler/meshc/tests/support/m053_todo_postgres_deploy.rs, compiler/meshc/tests/e2e_m053_s02.rs
  - Verify: DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_m053_s02 m053_s02_staged_postgres_helper_ -- --nocapture
- [x] **T02: Add the authoritative generated Postgres starter failover e2e rail** — ## Description

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
  - Estimate: 1 context window
  - Files: compiler/meshc/tests/e2e_m053_s02.rs, compiler/meshc/tests/support/m053_todo_postgres_deploy.rs, compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs, examples/todo-postgres/README.md
  - Verify: DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_m053_s02 -- --nocapture
- [x] **T03: Added the staged Postgres failover e2e rail in compile-green form and left the retained verifier wrapper as the next explicit step.** — ## Description

Wrap the new S02 proof into one fail-closed retained verifier surface that later hosted-chain work can call without reconstructing the failover setup. The executor should replay the S01 wrapper first, run the authoritative S02 Rust rail, copy only the fresh `.tmp/m053-s02/...` proof bundle into `.tmp/m053-s02/verify/`, and validate the retained bundle shape and redaction contract.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| prerequisite S01/S02 proof commands | Fail the wrapper immediately and preserve the command log instead of publishing stale evidence. | Kill the phase, record timeout context, and stop before copying old bundles. | Fail closed if a named test filter runs 0 tests or if the prerequisite wrapper/target emits malformed output. |
| retained bundle copy and manifest publication | Stop if the source bundle is missing, empty, or points at an old run; do not silently reuse prior evidence. | Treat a hung copy/archive step as artifact corruption and abort the wrapper. | Fail closed if bundle pointers, manifests, or required retained files do not match the copied directory. |
| secret-redaction and bundle-shape checks | Stop on the first leaked secret marker or missing required artifact path. | Treat a slow redaction scan as an artifact problem and abort. | Fail closed if required JSON/log paths are malformed or absent. |

## Load Profile

- **Shared resources**: `.tmp/m053-s02/verify/`, copied retained proof bundles, and prerequisite S01/S02 logs.
- **Per-operation cost**: one ordered replay of prerequisite commands plus one bundle-copy/shape/redaction audit.
- **10x breakpoint**: artifact-copy size and repeated full replays, not runtime throughput.

## Negative Tests

- **Malformed inputs**: missing `DATABASE_URL`, missing fresh `.tmp/m053-s02` bundle, malformed phase report, or stale bundle pointer.
- **Error paths**: S01 wrapper failure, S02 target failure, 0-test filter, missing copied artifact, or redaction drift.
- **Boundary conditions**: rerun with an existing `.tmp/m053-s02/verify/` tree, multiple fresh candidate bundles, and stale `latest-proof-bundle.txt` from a previous run.

## Steps

1. Create `scripts/verify-m053-s02.sh` to replay `bash scripts/verify-m053-s01.sh` before `cargo test -p meshc --test e2e_m053_s02 -- --nocapture`, with explicit phase logging, timeout handling, and named-test-count guards.
2. Snapshot the pre-run `.tmp/m053-s02` tree, copy only the fresh proof bundle(s) created by the S02 replay into `.tmp/m053-s02/verify/`, and publish `status.txt`, `current-phase.txt`, `phase-report.txt`, and `latest-proof-bundle.txt`.
3. Add retained-bundle checks for the specific starter-owned failover artifacts: per-node logs, before/after HTTP snapshots, cluster status/continuity/diagnostics JSON, scenario metadata, and redaction markers.
4. Keep the wrapper scoped to generated Postgres starter failover truth; do not pull packages-site or public-doc/Fly work forward from S03/S04.

## Must-Haves

- [ ] `bash scripts/verify-m053-s02.sh` is the single retained S02 failover proof surface and it replays S01 first.
- [ ] The verifier fail-closes on prerequisite failures, 0-test filters, stale/malformed bundle pointers, missing retained artifacts, or secret leakage.
- [ ] `.tmp/m053-s02/verify/` contains stable phase/status/pointer files and a copied retained proof bundle that downstream hosted-chain work can consume directly.
  - Estimate: 1 context window
  - Files: scripts/verify-m053-s02.sh, scripts/verify-m053-s01.sh, compiler/meshc/tests/e2e_m053_s02.rs
  - Verify: DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash scripts/verify-m053-s02.sh
