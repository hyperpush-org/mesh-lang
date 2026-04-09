---
estimated_steps: 4
estimated_files: 4
skills_used:
  - debug-like-expert
  - postgresql-database-engineering
  - test
---

# T02: Harden the staged Postgres failover rail around the startup window

- Why: S05 already proved the hosted red is not a missing mirror transport seam; the retained bundle shows standby saw mirrored pending state, then primary completed startup before the kill. The local S02 rail must now fail closed on that timing invariant instead of passing by luck.
- Do: Tighten `compiler/meshc/tests/e2e_m053_s02.rs` around the startup diagnostics and retained bundle shape so the generated-starter failover proof records the configured `pending_window_ms`, rejects an owner-completed-before-kill run, and keeps `.tmp/m053-s02/verify/` as the starter-owned proof bundle that Task 3 will ship.
- Done when: the authoritative S02 e2e and assembled `bash scripts/verify-m053-s02.sh` both go green only when the configured startup window is visible in diagnostics and the owner-loss/promotion/recovery path remains truthful.

## Steps

1. Use the hosted red bundle under `.tmp/m053-s05/remote-auth-24014506220/artifacts/authoritative-starter-failover-proof-diagnostics/` as comparison evidence for the exact failure shape: mirrored pending exists, `primary-run1.combined.log` later reaches `startup_completed`, and post-kill standby never promotes.
2. Update `compiler/meshc/tests/e2e_m053_s02.rs` (and a small helper in `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs` if it simplifies the assertion) so the local failover rail explicitly checks startup diagnostics metadata for the configured `pending_window_ms` and fails if `startup_completed` beats the forced owner stop.
3. Re-run the targeted e2e and assembled `bash scripts/verify-m053-s02.sh` against a disposable Postgres URL so `.tmp/m053-s02/verify/` becomes the retained local proof bundle for hosted closeout.

## Must-Haves

- [ ] `compiler/meshc/tests/e2e_m053_s02.rs` fails when runtime-owned startup diagnostics fall back to 2500ms instead of the configured delay.
- [ ] The destructive rail fails when the owner reaches `startup_completed` before the forced stop or when standby never records `automatic_promotion` / `automatic_recovery`.
- [ ] The green local replay still proves real starter HTTP CRUD/read behavior through `.tmp/m053-s02/verify/` without widening SQLite, packages, or docs scope.

## Inputs

- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/meshc/tests/e2e_m053_s02.rs`
- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs`
- `scripts/verify-m053-s02.sh`
- `.tmp/m053-s05/remote-auth-24014506220/artifacts/authoritative-starter-failover-proof-diagnostics/staged-postgres-failover-runtime-truth-1775437858534365102/pre-kill-continuity-standby.json`
- `.tmp/m053-s05/remote-auth-24014506220/artifacts/authoritative-starter-failover-proof-diagnostics/staged-postgres-failover-runtime-truth-1775437858534365102/primary-run1.combined.log`
- `.tmp/m053-s05/remote-auth-24014506220/artifacts/authoritative-starter-failover-proof-diagnostics/staged-postgres-failover-runtime-truth-1775437858534365102/post-kill-status-standby.timeout.txt`

## Expected Output

- `compiler/meshc/tests/e2e_m053_s02.rs`
- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs`
- `.tmp/m053-s02/verify/status.txt`
- `.tmp/m053-s02/verify/latest-proof-bundle.txt`

## Verification

DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_m053_s02 m053_s02_staged_postgres_failover_proves_clustered_http_and_runtime_recovery -- --nocapture && DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash scripts/verify-m053-s02.sh

## Observability Impact

- Signals added/changed: the staged failover rail now records and asserts `startup_dispatch_window` metadata alongside `automatic_promotion`, `automatic_recovery`, and the retained request-key/attempt-id bundle.
- How a future agent inspects this: read `.tmp/m053-s02/verify/latest-proof-bundle.txt`, then compare `pre-kill-continuity-standby.json`, `primary-run1.combined.log`, `post-kill-diagnostics-standby.json`, and `post-kill-status-standby.timeout.txt` in the copied proof bundle.
- Failure state exposed: whether mirrored pending existed, which `pending_window_ms` the runtime actually used, whether `startup_completed` beat the kill, and which promotion/recovery transition went missing.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Runtime diagnostics in `.tmp/m053-s02/...` and `meshc cluster diagnostics` | Stop the rail and preserve the latest bundle instead of guessing whether the owner finished too early. | Treat waiting past the configured pending window as proof drift; keep the last status/diagnostics snapshots and fail closed. | Fail closed if diagnostics omit `pending_window_ms`, `request_key`, `attempt_id`, or the promotion/recovery transitions. |
| Shared Postgres starter cluster (`primary`, `standby`, real CRUD routes) | Stop on the first unexpected HTTP/operator mismatch and keep per-node logs plus JSON snapshots. | Treat slow HTTP or cluster convergence as a runtime/harness regression, not as a reason to widen SQLite or add starter sleeps. | Fail closed if response JSON, continuity records, or retained bundle pointers are malformed. |
| Retained hosted failure bundle under `.tmp/m053-s05/remote-auth-24014506220/...` | Use it only as comparison evidence; if it is missing, rederive the expectation from the live local rail instead of weakening assertions. | N/A — once local tests start, the live local rail owns the truth. | Fail closed if the retained hosted artifacts no longer demonstrate the expected “mirrored pending then completed-before-kill” shape for the same scenario. |

## Load Profile

- **Shared resources**: one staged binary reused by two nodes, one shared Postgres database, `.tmp/m053-s02/` artifact space, and repeated operator polling.
- **Per-operation cost**: full staged starter boot, one destructive failover, CRUD/read replay, and copied retained bundle evidence.
- **10x breakpoint**: startup pending-window timing and operator polling churn on slower runners, not todo volume.

## Negative Tests

- **Malformed inputs**: missing `DATABASE_URL`, malformed todo payload/body, absent request key, and stale bundle pointers.
- **Error paths**: `startup_completed` before owner stop, standby never records `automatic_promotion` or `automatic_recovery`, and malformed diagnostics JSON.
- **Boundary conditions**: exactly one mirrored pending startup record, configured delay `20000`, empty todo list before seeding, and the stale-primary same-key guard after rejoin.
