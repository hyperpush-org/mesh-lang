---
estimated_steps: 4
estimated_files: 3
skills_used:
  - rust-best-practices
  - SQLite Database Expert
  - test
---

# T03: Add a live SQLite starter acceptance harness for local CRUD, restart, and failure truth

**Slice:** S02 — SQLite local starter contract
**Milestone:** M049

## Description

Prove the rewritten starter operationally. Generate the SQLite starter into a temp workspace, run its generated package tests, build and boot it without any cluster env, and exercise local CRUD, restart persistence, and at least one explicit negative rail. This is the slice-owned proof that the new public SQLite contract is real and stable enough for S03 to snapshot.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Generated SQLite project plus `meshc test` / `meshc build` | Fail closed before runtime boot and archive the generated project plus stdout/stderr. | Bound every phase with explicit timeouts and retain timeout artifacts. | Reject missing generated tests or malformed scaffold output before claiming runtime truth. |
| Local HTTP startup, `/health`, and SQLite file-path behavior | Archive startup logs and the last health/HTTP snapshot; fail the test on bad `TODO_DB_PATH` or startup regression. | Timeout with retained logs and last response instead of hanging. | Reject malformed JSON, wrong status codes, or health payloads that still imply clustered behavior. |
| Restart/persistence proof across local file DB reuse | Preserve before/after HTTP snapshots and db-path artifacts so persistence drift is diagnosable. | Timeout waiting for restarted health. | Reject missing persisted todo state or silent resets to in-memory behavior. |

## Load Profile

- **Shared resources**: temp workspaces, loopback ports, on-disk SQLite files, spawned local processes, and `.tmp/m049-s02` artifacts.
- **Per-operation cost**: one scaffold generation, one `meshc test`, one build, two local binary launches, and a short CRUD/restart sequence.
- **10x breakpoint**: port collisions and temp-db churn show up before CPU; there should be no cluster transport or operator path in this harness.

## Negative Tests

- **Malformed inputs**: empty title, malformed todo id, and invalid/broken `TODO_DB_PATH` values.
- **Error paths**: startup failure on bad db path, 404/400 rails on invalid todo access, and any non-local `/health` story.
- **Boundary conditions**: empty list before first create, persistence across restart, and rate-limit behavior if the local contract keeps it enabled.

## Steps

1. Add `compiler/meshc/tests/support/m049_todo_sqlite_scaffold.rs` to generate the SQLite starter, control ports/db paths, run `meshc test` / `meshc build`, spawn the binary, and archive logs plus raw HTTP responses.
2. Add `compiler/meshc/tests/e2e_m049_s02.rs` covering happy-path local `/health` + CRUD, restart persistence, and at least one explicit negative rail.
3. Register the helper in `compiler/meshc/tests/support/mod.rs` and keep the harness cluster-free: no `MESH_*`, no `meshc cluster`, and `/health` must prove local mode.
4. Retain a stable `.tmp/m049-s02/...` bundle so S03 can snapshot the final SQLite scaffold output rather than re-deriving it.

## Must-Haves

- [ ] `cargo test -p meshc --test e2e_m049_s02 -- --nocapture` exercises generated package tests, build, live local runtime, restart persistence, and a diagnosable failure path.
- [ ] The runtime proof shows the SQLite starter working without cluster env or operator CLI surfaces.
- [ ] The retained artifact bundle is rich enough to debug bad db-path, malformed-id, or local health/runtime failures later.

## Verification

- `cargo test -p meshc --test e2e_m049_s02 -- --nocapture`

## Observability Impact

- Signals added/changed: `.tmp/m049-s02/...` captures generated-project snapshots, build/test logs, runtime stdout/stderr, raw HTTP exchanges, and timeout evidence for the local starter.
- How a future agent inspects this: rerun `cargo test -p meshc --test e2e_m049_s02 -- --nocapture` and inspect the retained artifact bundle.
- Failure state exposed: bad `TODO_DB_PATH`, malformed todo access, startup regressions, and persistence failures are archived as named artifacts instead of generic test panics.

## Inputs

- `compiler/mesh-pkg/src/scaffold.rs` — rewritten SQLite scaffold output that this harness must prove on the live runtime path.
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` — existing M049 scaffold-harness pattern to mirror for process control and artifact capture.
- `compiler/meshc/tests/e2e_m049_s01.rs` — reference for the current slice-owned live starter acceptance style.
- `compiler/meshc/tests/support/mod.rs` — support-module registry that must expose the new SQLite helper.

## Expected Output

- `compiler/meshc/tests/support/m049_todo_sqlite_scaffold.rs` — shared helper for generating, testing, building, running, and archiving the local SQLite starter.
- `compiler/meshc/tests/e2e_m049_s02.rs` — slice-owned live SQLite starter acceptance rail.
- `compiler/meshc/tests/support/mod.rs` — support registry updated to expose the new SQLite harness.
