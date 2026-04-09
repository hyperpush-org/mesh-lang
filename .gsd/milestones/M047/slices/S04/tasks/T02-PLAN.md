---
estimated_steps: 4
estimated_files: 2
skills_used:
  - test
---

# T02: Move the clustered scaffold generator and CLI contract to @cluster

Update the generated clustered scaffold so `meshc init --clustered` emits the source-first contract the repo now expects to dogfood. Keep the route-free `main.mpl` bootstrap path, keep the visible work body minimal, and preserve runtime-name continuity by keeping the function name `execute_declared_work` instead of keeping the old helper.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| scaffold file generation | fail the init command instead of creating a half-migrated project tree | N/A | never emit mixed `@cluster` + legacy helper output |
| scaffold contract tests | fail loudly on text drift rather than letting generated files silently regress | N/A | treat mismatched generated content as contract breakage, not a soft warning |
| runtime-name continuity assumption | keep the function name stable so downstream route-free rails still see `Work.execute_declared_work` without a helper | N/A | do not invent a second runtime-name helper surface while migrating the syntax |

## Load Profile

- **Shared resources**: temp project directories and textual scaffold fixtures asserted by unit/tooling tests.
- **Per-operation cost**: one project generation plus a handful of file reads/assertions; no long-lived runtime state.
- **10x breakpoint**: contract drift across generated files and tests breaks faster than performance does; correctness matters more than throughput here.

## Negative Tests

- **Malformed inputs**: existing project directory collisions and clustered init reruns still fail cleanly.
- **Error paths**: generated `mesh.toml` must stay package-only and generated `work.mpl` must omit the old helper/legacy syntax completely.
- **Boundary conditions**: generated README still describes runtime-owned inspection, generated `main.mpl` stays route-free, and generated `work.mpl` preserves `1 + 1` plus `execute_declared_work` naming.

## Steps

1. Rewrite the clustered scaffold template in `scaffold.rs` to emit `@cluster pub fn execute_declared_work(...)` with no `declared_work_runtime_name()` helper and no manifest clustering text.
2. Update the generated README copy so it teaches the new source-first route-free contract and preserves the runtime-owned inspection commands.
3. Update mesh-pkg scaffold tests and `tooling_e2e` expectations so `meshc init --clustered` becomes the canonical source-first generator surface.
4. Keep the scaffold contract intentionally narrow: no `HTTP.clustered(...)`, no app-owned routes, and no extra helper seams added during the migration.

## Must-Haves

- [ ] `meshc init --clustered` generates `@cluster` source, not `clustered(work)` or `declared_work_runtime_name()`.
- [ ] Generated scaffold output still keeps `main.mpl` route-free and runtime-owned via `Node.start_from_env()`.
- [ ] Generated README text matches the new source-first model without claiming the missing HTTP route wrapper exists.

## Inputs

- ``compiler/mesh-pkg/src/scaffold.rs``
- ``compiler/meshc/tests/tooling_e2e.rs``

## Expected Output

- ``compiler/mesh-pkg/src/scaffold.rs``
- ``compiler/meshc/tests/tooling_e2e.rs``

## Verification

cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture && cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture
