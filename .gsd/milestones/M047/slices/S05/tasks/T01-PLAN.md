---
estimated_steps: 23
estimated_files: 6
skills_used: []
---

# T01: Confirmed the leaked public continuity-arg seam and isolated the validator/wrapper changes needed for the next unit; no source changes landed in this unit.

The user-facing clustered contract is wrong today: codegen still expects the declared work wrapper to expose `request_key` and `attempt_id`, which leaks runtime continuity plumbing into ordinary source. Remove that public ceremony first so later scaffold and Todo work can dogfood the right model instead of repainting the wrong one.

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

## Inputs

- ``compiler/mesh-typeck/src/infer.rs``
- ``compiler/mesh-codegen/src/declared.rs``
- ``compiler/mesh-codegen/src/codegen/expr.rs``
- ``compiler/mesh-codegen/src/mir/lower.rs``
- ``compiler/meshc/tests/e2e_m047_s01.rs``
- ``compiler/meshc/tests/e2e_m047_s02.rs``

## Expected Output

- ``compiler/mesh-typeck/src/infer.rs``
- ``compiler/mesh-codegen/src/declared.rs``
- ``compiler/mesh-codegen/src/codegen/expr.rs``
- ``compiler/mesh-codegen/src/mir/lower.rs``
- ``compiler/meshc/tests/e2e_m047_s01.rs``
- ``compiler/meshc/tests/e2e_m047_s02.rs``

## Verification

cargo test -p meshc --test e2e_m047_s01 -- --nocapture && cargo test -p meshc --test e2e_m047_s02 -- --nocapture

## Observability Impact

- Signals added/changed: clustered runtime continuity should keep exposing request-key / attempt-id metadata even when the public function signature is now no-ceremony.
- How a future agent inspects this: replay `e2e_m047_s01` / `e2e_m047_s02` and read the retained continuity / diagnostics output.
- Failure state exposed: wrapper-signature drift should show up as compile-time or route-free runtime proof failures instead of silently dropping continuity completion.
