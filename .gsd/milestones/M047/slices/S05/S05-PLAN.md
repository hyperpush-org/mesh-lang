# S05: Simple clustered Todo scaffold

**Goal:** Shift the public clustered-function contract to ordinary no-ceremony `@cluster` functions, rebaseline the existing route-free clustered scaffold/examples on that contract, and add an explicit SQLite Todo starter template with actor-backed rate limiting, Docker packaging, and native/Docker proof without pretending `HTTP.clustered(...)` already exists.
**Demo:** After this: After this: a new scaffold command generates a SQLite Todo API with several routes, actors, rate limiting, clustered route syntax, and a complete Dockerfile, and the result reads like a starting point.

## Tasks
- [x] **T01: Confirmed the leaked public continuity-arg seam and isolated the validator/wrapper changes needed for the next unit; no source changes landed in this unit.** — The user-facing clustered contract is wrong today: codegen still expects the declared work wrapper to expose `request_key` and `attempt_id`, which leaks runtime continuity plumbing into ordinary source. Remove that public ceremony first so later scaffold and Todo work can dogfood the right model instead of repainting the wrong one.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| clustered declaration validation / lowering | fail compile-time with a source-local diagnostic instead of silently generating a half-adapted wrapper | N/A | reject inconsistent clustered signatures instead of guessing at hidden metadata placement |
| declared work wrapper generation | keep continuity completion and runtime registration intact through an internal adapter, or fail loudly before codegen succeeds | N/A | malformed wrapper plans must not produce LLVM that compiles but drops completion/diagnostic state |
| runtime continuity inspection | preserve request-key / attempt-id truth in runtime-owned CLI surfaces even after the public function signature stops exposing them | bounded by existing e2e timeouts | malformed continuity records or missing metadata should fail the runtime proof rail, not degrade to a no-op |

## Load Profile

- **Shared resources**: compiler validation buffers, MIR/codegen wrapper generation, and route-free runtime continuity state.
- **Per-operation cost**: one compile plus one route-free runtime replay per proof case; no new external network dependency.
- **10x breakpoint**: wrapper/ABI drift and diagnostic spam will fail before throughput matters, so correctness and failure visibility dominate this task.

## Negative Tests

- **Malformed inputs**: stale clustered sources that still declare `request_key` / `attempt_id`, invalid decorator counts, and mixed legacy/no-ceremony fixtures.
- **Error paths**: ordinary `@cluster` functions with no public continuity args fail loudly if lowering cannot inject hidden metadata, and runtime continuity rails fail closed if the internal metadata disappears.
- **Boundary conditions**: `@cluster pub fn add() -> Int`, `@cluster(3) pub fn retry() -> Int`, and runtime continuity output that still reports request/attempt metadata all stay truthful together.

## Steps

1. Remove the public `(request_key, attempt_id)` assumption from clustered declaration validation and declared-work lowering so no-ceremony `@cluster` functions become the supported source contract.
2. Generate internal adapters that keep runtime continuity completion, request keys, and attempt IDs behind the wrapper instead of as user-authored parameters.
3. Add compiler/runtime regression rails proving ordinary no-ceremony `@cluster` functions build with generic runtime names while `meshc cluster continuity` still exposes the internal continuity metadata.

## Must-Haves

- [ ] A source-declared clustered function like `@cluster pub fn add() -> Int do ... end` is valid without public continuity args.
- [ ] Runtime-owned continuity completion and diagnostics still record request keys / attempt IDs internally after the public signature changes.
- [ ] Compiler/runtime proof rails fail closed if the adapter seam regresses.
  - Estimate: 4h
  - Files: compiler/mesh-typeck/src/infer.rs, compiler/mesh-codegen/src/declared.rs, compiler/mesh-codegen/src/codegen/expr.rs, compiler/mesh-codegen/src/mir/lower.rs, compiler/meshc/tests/e2e_m047_s01.rs, compiler/meshc/tests/e2e_m047_s02.rs
  - Verify: cargo test -p meshc --test e2e_m047_s01 -- --nocapture && cargo test -p meshc --test e2e_m047_s02 -- --nocapture
- [x] **T02: Confirmed that the route-free scaffold/example cutover is blocked because the current compiler still rejects zero-ceremony `@cluster` work before any scaffold or package rebaseline can land.** — Once the public contract changes, the existing route-free clustered surfaces cannot keep teaching `execute_declared_work(request_key, attempt_id)`. Rebaseline the default scaffold, repo-owned examples, and historical route-free exact-string rails on the no-ceremony model so later Todo work extends the corrected surface instead of preserving the old one in parallel.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| route-free scaffold/example source parity | fail unit/tooling/package rails instead of letting scaffold, `tiny-cluster/`, and `cluster-proof/` drift apart | N/A | reject stale `execute_declared_work` / helper wording instead of treating it as a compatible alias |
| historical route-free e2e harness | preserve retained artifacts and runtime-owned CLI inspection while changing the source contract | bounded by existing harness timeouts | malformed bundle pointers or exact-string expectations should fail closed |
| README/runbook wording | keep continuity metadata described as runtime-owned inspection state, not public function arguments | N/A | reject contradictory docs that teach both models at once |

## Load Profile

- **Shared resources**: temp scaffold projects, route-free package build outputs, retained `.tmp` bundles, and exact-string docs/tests.
- **Per-operation cost**: one scaffold generation plus route-free package/test replays; no heavy external services.
- **10x breakpoint**: textual contract drift across scaffold/packages/tests is the first failure mode, not runtime throughput.

## Negative Tests

- **Malformed inputs**: stale generated/package sources containing `execute_declared_work`, `request_key`, `attempt_id`, `clustered(work)`, or manifest clustering text.
- **Error paths**: historical route-free rails must still fail explicitly on missing retained artifacts or malformed continuity output after the contract change.
- **Boundary conditions**: the default scaffold, `tiny-cluster/`, and `cluster-proof/` all dogfood the same no-ceremony `@cluster` source contract while keeping runtime-owned `meshc cluster` inspection.

## Steps

1. Rewrite `meshc init --clustered`, `tiny-cluster/`, and `cluster-proof/` to dogfood ordinary no-ceremony `@cluster` functions with generic runtime names instead of scaffold-owned `execute_declared_work(...)` ceremony.
2. Update shared route-free harnesses and historical exact-string rails so they assert the new contract while preserving runtime-owned CLI continuity/diagnostic inspection and retained artifact behavior.
3. Refresh package/tooling/readme assertions so the route-free public story stays coherent before the Todo template is introduced.

## Must-Haves

- [ ] The default route-free clustered scaffold and repo-owned example packages all use the same no-ceremony `@cluster` contract.
- [ ] Historical route-free rails still localize failures honestly after the source contract changes.
- [ ] Route-free docs/tests keep continuity metadata as runtime-owned inspection truth rather than public function parameters.
  - Estimate: 4h
  - Files: compiler/mesh-pkg/src/scaffold.rs, compiler/meshc/tests/tooling_e2e.rs, tiny-cluster/work.mpl, cluster-proof/work.mpl, compiler/meshc/tests/support/m046_route_free.rs, compiler/meshc/tests/e2e_m046_s05.rs
  - Verify: cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture && cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture && cargo run -q -p meshc -- test tiny-cluster/tests && cargo run -q -p meshc -- test cluster-proof/tests && cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture
  - Blocker: Zero-ceremony `@cluster` work is still unsupported in the current tree; even a minimal `@cluster pub fn add() -> Int` build fails in declared-work wrapper codegen. Because of that missing capability, the route-free scaffold, example packages, package smoke tests, and equal-surface rails still advertise the stale `execute_declared_work(request_key, attempt_id)` contract.
- [x] **T03: Stopped at the declared-work wrapper seam under the context-budget warning; no compiler/runtime source changes landed in this unit.** — Implement the prerequisite compiler/runtime seam that T02 proved is still missing. Remove the public `(request_key, attempt_id)` requirement from declared-work validation/lowering for ordinary `@cluster` functions, generate internal adapters that receive runtime continuity metadata and invoke the user-authored function without exposing those args in source, and keep runtime-owned continuity completion/diagnostic surfaces truthful. Extend the M047 compiler/runtime rails so a minimal `@cluster pub fn add() -> Int` build passes while `meshc cluster continuity` still reports internal request/attempt metadata.
  - Estimate: 5h
  - Files: compiler/mesh-typeck/src/infer.rs, compiler/mesh-codegen/src/declared.rs, compiler/mesh-codegen/src/codegen/expr.rs, compiler/mesh-codegen/src/mir/lower.rs, compiler/meshc/tests/e2e_m047_s01.rs, compiler/meshc/tests/e2e_m047_s02.rs
  - Verify: cargo test -p meshc --test e2e_m047_s01 -- --nocapture && cargo test -p meshc --test e2e_m047_s02 -- --nocapture
- [x] **T04: Confirmed that T04 is still blocked because the no-ceremony `@cluster` wrapper seam has not landed, so the route-free scaffold/example cutover cannot be verified honestly.** — Once T03 lands, rewrite the existing route-free clustered public surfaces to dogfood the corrected no-ceremony `@cluster` model. Update `meshc init --clustered`, `tiny-cluster/`, `cluster-proof/`, and the shared route-free harnesses/tests so they stop teaching `execute_declared_work(request_key, attempt_id)` or `Work.execute_declared_work` as the public source contract while preserving runtime-owned `meshc cluster status|continuity|diagnostics` inspection and retained-artifact behavior.
  - Estimate: 4h
  - Files: compiler/mesh-pkg/src/scaffold.rs, compiler/meshc/tests/tooling_e2e.rs, tiny-cluster/work.mpl, cluster-proof/work.mpl, compiler/meshc/tests/support/m046_route_free.rs, compiler/meshc/tests/e2e_m046_s05.rs
  - Verify: cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture && cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture && cargo run -q -p meshc -- test tiny-cluster/tests && cargo run -q -p meshc -- test cluster-proof/tests && cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture
  - Blocker: Zero-ceremony `@cluster` declared work is still unsupported in the current tree. As long as `@cluster pub fn add() -> Int` fails in declared-work wrapper codegen, the route-free scaffold, `tiny-cluster/`, `cluster-proof/`, and their exact-string rails cannot be rebaselined honestly to the intended public contract.
- [x] **T05: Documented that the Todo starter is still blocked by missing no-ceremony `@cluster` support and zero-test verifier filters; no scaffold source changes landed.** — Extend `meshc init` with an explicit Todo template selector (for example `--template todo-api`) without changing the corrected route-free `--clustered` default. Generate a small multi-file starter with SQLite persistence, several HTTP routes, actor-backed rate limiting, clustered work on the no-ceremony `@cluster` contract, a README, and a standalone Dockerfile/.dockerignore that use the public Mesh install/build story rather than repo-only assumptions.
  - Estimate: 5h
  - Files: compiler/meshc/src/main.rs, compiler/mesh-pkg/src/scaffold.rs, compiler/mesh-pkg/src/lib.rs, compiler/meshc/tests/tooling_e2e.rs
  - Verify: cargo test -p mesh-pkg m047_s05 -- --nocapture && cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture
  - Blocker: The upstream blocker from T03/T04 is still unresolved: declared-work wrapper generation still requires public `request_key` and `attempt_id` arguments, so an opt-in Todo scaffold built on the intended contract would be false. There is also no current T05 proof rail in `mesh-pkg` or `tooling_e2e`; both planned filters return success with zero matched tests.
- [x] **T06: Prove the Todo starter end to end and refresh public wording** — Add the generated-project helper, named `e2e_m047_s05` rail, and assembled verifier for the Todo starter. Generate the template, build and boot it with retained logs/SQLite artifacts, exercise CRUD + rate-limit + restart-persistence behavior through real HTTP requests, build the Docker image, and update targeted docs/help so the no-ceremony `@cluster` story and Todo template are discoverable without claiming `HTTP.clustered(...)` already exists.
  - Estimate: 4h
  - Files: compiler/meshc/tests/support/m047_todo_scaffold.rs, compiler/meshc/tests/e2e_m047_s05.rs, scripts/verify-m047-s05.sh, README.md, website/docs/docs/getting-started/clustered-example/index.md
  - Verify: cargo test -p meshc --test e2e_m047_s05 -- --nocapture && bash scripts/verify-m047-s05.sh && npm --prefix website run build
