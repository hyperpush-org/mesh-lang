# S06: Docs, migration, and assembled proof closeout

**Goal:** Close M047 on one truthful assembled proof surface by fixing the built-package SQLite execute seam, proving the generated Todo starter natively and inside Docker, and finishing the public source-first `@cluster` docs/migration story without claiming `HTTP.clustered(...)` already exists.
**Demo:** After this: After this: one assembled verifier can regenerate the scaffold, build its Docker image, exercise the Todo API and clustered routes, and replay docs/migration/proof checks on the new model end to end.

## Tasks
- [x] **T01: Fixed generic Result scalar payload boxing for SQLite helper rewraps and added a dedicated built-package regression rail.** — ## Description

The red blocker below S06 is not Todo-specific: a Mesh package built with `meshc build` can open SQLite but fails on `Sqlite.execute(...)` with `bad parameter or other API misuse`. Fix that AOT/runtime seam first and add a dedicated built-package regression so later native and Docker Todo proof is trustworthy.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| built-package SQLite lowering between `compiler/mesh-codegen` and `compiler/mesh-rt` | fail the focused regression with retained binary/log artifacts; do not hide the issue behind Todo-specific retries or docs changes | N/A — this is a local build/run path, but the regression test should keep bounded process waits | treat malformed params/result payloads as a failing runtime/ABI contract, not as successful zero-row writes |
| runtime SQLite execute/query semantics | keep `Sqlite.execute` and `Sqlite.query` consistent for empty/non-empty param lists or fail a named test; do not silently special-case only the scaffold SQL strings | N/A | malformed SQLite error text is still failure evidence; do not downgrade to generic success |

## Load Profile

- **Shared resources**: `mesh_sqlite_execute`, AOT lowering, and retained `.tmp` test artifact directories.
- **Per-operation cost**: one `meshc build` plus one emitted-binary execution in the focused regression.
- **10x breakpoint**: pointer/ABI drift and malformed result payloads fail long before throughput matters.

## Negative Tests

- **Malformed inputs**: empty param list `[]`, non-empty param list with placeholders, and placeholder-count mismatches in the focused regression fixture.
- **Error paths**: a built binary that can `Sqlite.open` but not `Sqlite.execute` must fail with a named regression instead of hanging or being masked by Todo-specific retries.
- **Boundary conditions**: `CREATE TABLE`, `INSERT`, and read-back/terminal success all work in a built binary, not just via `compile_and_run`.

## Steps

1. Compare the green in-process `compile_and_run` SQLite path with a minimal built-package repro to identify where params/result pointers diverge in AOT lowering.
2. Repair the lowering/runtime seam in `compiler/mesh-codegen` and/or `compiler/mesh-rt` so `mesh_sqlite_execute` handles built binaries with empty and non-empty param lists correctly.
3. Add a focused `compiler/meshc/tests/e2e_sqlite_built_package.rs` regression that builds a tiny package, runs the emitted binary, and asserts `CREATE TABLE`, `INSERT`, and read-back/terminal success while retaining artifacts on failure.
4. Keep the failure surface narrow and honest: do not add Todo-specific workarounds or silent fallbacks.

## Must-Haves

- [ ] A package emitted by `meshc build` can execute `Sqlite.execute(db, ..., [])` and parameterized writes successfully.
- [ ] The fix is covered by a focused built-package regression, not only by the Todo scaffold rail.
- [ ] The task does not hide the issue behind scaffold-only retries, docs edits, or generic “bad parameter” suppression.

## Verification

- `cargo test -p meshc --test e2e_sqlite_built_package -- --nocapture`

## Observability Impact

- Signals added/changed: focused AOT SQLite regression artifacts and any preserved runtime SQLite error text.
- How a future agent inspects this: rerun the dedicated test file and inspect the retained built binary/log artifacts under `.tmp`.
- Failure state exposed: built-package pointer/ABI drift fails as a named regression instead of first surfacing deep inside the Todo scaffold.
  - Estimate: 3h
  - Files: compiler/mesh-rt/src/db/sqlite.rs, compiler/mesh-codegen/src/mir/lower.rs, compiler/mesh-codegen/src/codegen/intrinsics.rs, compiler/meshc/tests/e2e_sqlite_built_package.rs
  - Verify: cargo test -p meshc --test e2e_sqlite_built_package -- --nocapture
- [x] **T02: Extended the Todo scaffold proof to build a Linux `output`, run the generated image end to end, and retain container failure artifacts.** — ## Description

With built-package SQLite truth restored, extend the existing Todo harness from host-only runtime truth to actual container runtime proof. Keep the generated app on ordinary `HTTP.on_*` routes and route-free `@cluster` startup work; this task proves the documented Docker story rather than inventing new product behavior.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Docker build/run of the generated Todo starter | fail with retained build logs, `docker inspect`, and container stdout/stderr; do not fall back to host-only proof | stop the container, write a timeout artifact, and fail the named e2e rail | reject malformed `/health` or CRUD JSON instead of treating it as success |
| generated scaffold Docker/README contract | keep the documented env/volume/port shape aligned with what the e2e proves, or fail the harness explicitly | N/A | treat contradictory docs or container defaults as contract drift, not optional polish |

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
  - Estimate: 3h
  - Files: compiler/meshc/tests/support/m047_todo_scaffold.rs, compiler/meshc/tests/e2e_m047_s05.rs, compiler/mesh-pkg/src/scaffold.rs
  - Verify: cargo test -p meshc --test e2e_m047_s05 -- --nocapture && bash scripts/verify-m047-s05.sh
- [x] **T03: Added the final S06 closeout rail, finished the public source-first `@cluster` docs story, and retained the delegated S05 proof bundle under one assembled verifier surface.** — ## Description

With runtime truth green, finish the final user-facing closeout: docs must teach one source-first `@cluster` story, preserve the route-free canonical surfaces, present the Todo template as the fuller starter, and make `scripts/verify-m047-s06.sh` the final assembled authority that wraps rather than replaces S04/S05 subrails.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| docs/readmes/verifier-contract strings | fail exact file-path assertions; do not leave mixed S04/S05/S06 authority or stale migration wording | N/A | treat contradictory or missing markers as contract drift, not optional docs polish |
| delegated `scripts/verify-m047-s05.sh` replay and retained artifact handoff | stop at the first red prerequisite and keep copied bundle/log pointers; do not write into `.tmp/m047-s05/verify` from the S06 wrapper | bound every replayed command and fail with the captured phase log | reject missing `status.txt`, `phase-report.txt`, malformed bundle pointers, or zero-test filters |

## Load Profile

- **Shared resources**: README/VitePress pages, `.tmp/m047-s05/verify`, new `.tmp/m047-s06/verify`, and the docs build.
- **Per-operation cost**: one contract test, one assembled verifier replay, and one docs build.
- **10x breakpoint**: string drift and bundle-shape mistakes fail before runtime throughput; clarity of authority and artifact ownership matters most.

## Negative Tests

- **Malformed inputs**: stale `execute_declared_work` / `Work.execute_declared_work` public docs, missing `verify-m047-s06.sh` references, docs that claim `HTTP.clustered(...)`, or wrapper scripts that write into `.tmp/m047-s05/`.
- **Error paths**: delegated S05 rail goes red, retained S05 artifact bundle is missing/malformed, docs build fails, or zero-test `m047_s06_` filters pass without running.
- **Boundary conditions**: S04 remains the authoritative cutover rail, S05 remains the delegated Todo/runtime subrail, and S06 becomes the final closeout rail while route-free `@cluster` surfaces stay canonical.

## Steps

1. Update `README.md`, `website/docs/docs/tooling/index.md`, `website/docs/docs/getting-started/clustered-example/index.md`, `website/docs/docs/distributed-proof/index.md`, and `website/docs/docs/distributed/index.md` so they teach the two-layer story: route-free canonical surfaces plus the fuller Todo starter, explicit migration from `clustered(work)` / `[cluster]` / helper-shaped names to ordinary verbs, and explicit `HTTP.clustered(...)` non-goal.
2. Add `compiler/meshc/tests/e2e_m047_s06.rs` to fail-close on the final docs/verifier authority: S04 cutover rail retained, S05 lower-level Todo subrail retained, S06 final closeout rail present, helper-shaped public names removed, and no docs claim route-local clustering shipped.
3. Add `scripts/verify-m047-s06.sh` as the final wrapper: replay `bash scripts/verify-m047-s05.sh`, copy `.tmp/m047-s05/verify` into `.tmp/m047-s06/verify/retained-m047-s05-verify`, run the S06 contract test and docs build, write its own phase/status/bundle files, and keep all artifact ownership under `.tmp/m047-s06/`.
4. Keep historical truth additive, not replacement: S04 still documents cutover, S05 still documents Todo proof, and S06 closes the milestone without reopening legacy or overclaiming `HTTP.clustered(...)`.

## Must-Haves

- [ ] Public docs/readmes show one source-first `@cluster` story with route-free canonical surfaces and the Todo starter as the fuller example.
- [ ] Migration guidance explicitly covers `clustered(work)`, `[cluster]`, and helper-shaped names like `execute_declared_work(...)` / `Work.execute_declared_work`.
- [ ] `scripts/verify-m047-s06.sh` owns the final closeout bundle under `.tmp/m047-s06/verify` while retaining delegated S05 evidence and failing closed on malformed handoff.
- [ ] `compiler/meshc/tests/e2e_m047_s06.rs` turns stale authority or `HTTP.clustered(...)` claims into named failures.

## Verification

- `cargo test -p meshc --test e2e_m047_s06 -- --nocapture`
- `bash scripts/verify-m047-s06.sh`

## Observability Impact

- Signals added/changed: `.tmp/m047-s06/verify/{status.txt,current-phase.txt,phase-report.txt,full-contract.log,latest-proof-bundle.txt}` plus retained S05 verify bundle pointers.
- How a future agent inspects this: rerun the S06 contract test or wrapper, inspect phase logs, then follow retained S05 artifacts into native/container runtime evidence.
- Failure state exposed: stale doc authority, missing migration language, zero-test drift, and malformed retained bundle handoff fail as named phases instead of generic milestone-closeout drift.
  - Estimate: 3h
  - Files: README.md, website/docs/docs/tooling/index.md, website/docs/docs/getting-started/clustered-example/index.md, website/docs/docs/distributed-proof/index.md, website/docs/docs/distributed/index.md, compiler/meshc/tests/e2e_m047_s06.rs, scripts/verify-m047-s06.sh
  - Verify: cargo test -p meshc --test e2e_m047_s06 -- --nocapture && bash scripts/verify-m047-s06.sh
