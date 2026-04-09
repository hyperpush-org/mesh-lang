---
estimated_steps: 4
estimated_files: 3
skills_used:
  - multi-stage-dockerfile
  - test
  - debug-like-expert
---

# T02: Extend the Todo scaffold harness to prove container runtime truth

**Slice:** S06 — Docs, migration, and assembled proof closeout
**Milestone:** M047

## Description

With built-package SQLite truth restored, extend the existing Todo harness from host-only runtime truth to actual container runtime proof. Keep the generated app on ordinary `HTTP.on_*` routes and route-free `@cluster` startup work; this task proves the documented Docker story rather than inventing new product behavior.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Docker build/run of the generated Todo starter | Fail with retained build logs, `docker inspect`, and container stdout/stderr; do not fall back to host-only proof. | Stop the container, write a timeout artifact, and fail the named e2e rail. | Reject malformed `/health` or CRUD JSON instead of treating it as success. |
| generated scaffold Docker/README contract | Keep the documented env/volume/port shape aligned with what the e2e proves, or fail the harness explicitly. | N/A | Treat contradictory docs or container defaults as contract drift, not optional polish. |

## Load Profile

- **Shared resources**: Docker image cache, host ports, SQLite files/volumes, and retained `.tmp/m047-s05` artifacts.
- **Per-operation cost**: one scaffold generation, one native build/run, one `docker build`, one `docker run`, and a small number of HTTP requests.
- **10x breakpoint**: port collisions, readiness waits, and missing retained logs fail before throughput matters.

## Negative Tests

- **Malformed inputs**: missing published port, broken `TODO_DB_PATH`, malformed `/health` JSON, and CRUD responses with the wrong status code.
- **Error paths**: container never reaches `/health`, containerized writes fail after native proof succeeds, or artifact capture drops the logs needed to debug the failure.
- **Boundary conditions**: native runtime truth stays green, the container exposes the same `clustered_handler` metadata, and the proof still uses ordinary `HTTP.on_*` routes rather than claiming `HTTP.clustered(...)` exists.

## Steps

1. Reuse the container lifecycle pattern from `compiler/meshc/tests/e2e_m043_s03.rs` inside `compiler/meshc/tests/support/m047_todo_scaffold.rs`: start, wait for published port, capture logs/inspect, stop, and cleanup.
2. Extend `compiler/meshc/tests/e2e_m047_s05.rs` so the generated Todo project still passes native runtime truth and then also `docker build` + `docker run`, `/health`, and one CRUD route inside the container with retained artifacts.
3. If the runtime proof exposes a real mismatch in the generated Dockerfile/README/env contract, adjust `compiler/mesh-pkg/src/scaffold.rs` so the documented Docker run path matches what the test proves; keep the prebuilt-`output` binary model and ordinary `HTTP.on_*` routes.
4. Keep the non-goal explicit in code/comments/tests: do not add or imply `HTTP.clustered(...)`.

## Must-Haves

- [ ] The generated Todo image boots and reaches `/health` inside a container.
- [ ] At least one real CRUD path succeeds against the containerized app, not just the host binary.
- [ ] Native runtime proof, container proof, and retained artifacts all stay coherent on the same generated project.

## Verification

- `cargo test -p meshc --test e2e_m047_s05 -- --nocapture`
- `bash scripts/verify-m047-s05.sh`

## Observability Impact

- Signals added/changed: retained container stdout/stderr, `docker inspect` JSON, and containerized `/health`/CRUD snapshots in the Todo scaffold artifact bundle.
- How a future agent inspects this: rerun `e2e_m047_s05` or `bash scripts/verify-m047-s05.sh`, then open the retained `.tmp/m047-s05/todo-scaffold-runtime-truth-*` bundle.
- Failure state exposed: container boot/readiness drift is visible separately from native SQLite/runtime drift.

## Inputs

- `compiler/meshc/tests/support/m047_todo_scaffold.rs` — current native-runtime and docker-build harness.
- `compiler/meshc/tests/e2e_m047_s05.rs` — current end-to-end Todo scaffold truth rail.
- `compiler/meshc/tests/e2e_m043_s03.rs` — existing container lifecycle pattern to reuse.
- `compiler/mesh-pkg/src/scaffold.rs` — generated Dockerfile/README contract for the `todo-api` template.

## Expected Output

- `compiler/meshc/tests/support/m047_todo_scaffold.rs` — container lifecycle helpers with retained logs and inspect artifacts.
- `compiler/meshc/tests/e2e_m047_s05.rs` — native plus container runtime assertions for the generated Todo starter.
- `compiler/mesh-pkg/src/scaffold.rs` — Docker/README/env contract updates only if the runtime proof exposes a real mismatch.
