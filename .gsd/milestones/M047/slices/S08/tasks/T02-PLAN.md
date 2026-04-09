---
estimated_steps: 3
estimated_files: 4
skills_used: []
---

# T02: Added single-node clustered-route proof helpers for the Todo scaffold and captured the Docker host-handshake blocker that still prevents S05 closeout.

Why: S08 is only done when the starter's native and Docker proof rails exercise the shipped wrapper end to end instead of merely generating the syntax.
Do: Extend the Todo scaffold support harness to boot the generated app in single-node cluster mode natively and inside Docker, publish/query the cluster port, reuse bounded continuity helpers, and rebaseline `e2e_m047_s05` / `verify-m047-s05.sh` so wrapped read routes prove truthful continuity while existing CRUD/rate-limit/persistence coverage stays green.
Done when: the lower-level starter rail shows `HTTP.clustered(1, ...)` read-route success plus `declared_handler_runtime_name`, `replication_count=1`, `replication_health=local_only`, and Docker/native proof artifacts.

## Inputs

- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
- `compiler/meshc/tests/support/m046_route_free.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `scripts/verify-m047-s05.sh`

## Expected Output

- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
- `compiler/meshc/tests/support/m046_route_free.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `scripts/verify-m047-s05.sh`

## Verification

cargo test -p meshc --test e2e_m047_s05 -- --nocapture && cargo test -p meshc --test e2e_m047_s07 -- --nocapture && bash scripts/verify-m047-s05.sh

## Observability Impact

Adds native/Docker cluster-status and continuity snapshots plus retained log/inspect artifacts for wrapped starter routes, so missing request keys, cluster-port drift, and local-only fallback are diagnosable from `.tmp/m047-s05/...` evidence.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| native single-node cluster-mode boot for the generated Todo app | fail with retained stdout/stderr and env artifacts; do not silently fall back to non-cluster mode | keep bounded health/continuity waits and archive the last observation before failing | malformed `/health` or `meshc cluster` JSON is a hard failure, not an empty success |
| Docker cluster-port publication and container lifecycle | fail with retained `docker inspect`, stdout/stderr, and port snapshots; do not treat unpublished cluster ports as optional | stop the container, archive the timeout snapshot, and fail the task rail | malformed inspect/port output is failure evidence, not a best-effort skip |
| continuity/status polling via shared helpers | require a new request key plus the expected runtime/count fields before passing; do not trust list order or immediate snapshots | keep bounded polling and retain the last list/record snapshot | malformed continuity records fail the rail instead of degrading to string-matching heuristics |

## Load Profile

- **Shared resources**: cluster ports, SQLite files, Docker containers, continuity registry state, and retained `.tmp/m047-s05` artifacts.
- **Per-operation cost**: one generated project, one native clustered boot, one Docker build/run, a small number of HTTP requests, and a small number of `meshc cluster` queries.
- **10x breakpoint**: port collisions, readiness waits, and continuity polling fail before throughput matters.

## Negative Tests

- **Malformed inputs**: missing `MESH_*` env, unpublished cluster port, stale request key discovery, or empty continuity lists.
- **Error paths**: the wrapped `GET /todos` route returns 5xx/503, continuity never reaches `completed`, or Docker boots without usable cluster inspection.
- **Boundary conditions**: single-node `HTTP.clustered(1, ...)` records `replication_health=local_only` with `fell_back_locally=true`, and the existing CRUD/rate-limit/persistence checks remain green on the same starter.
