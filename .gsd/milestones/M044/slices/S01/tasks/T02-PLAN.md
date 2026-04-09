---
estimated_steps: 31
estimated_files: 7
skills_used: []
---

# T02: Replace stringly Continuity results with typed builtins across typeck, MIR, and runtime

---
estimated_steps: 4
estimated_files: 7
skills_used:
  - rust-best-practices
  - test
---

Move the public continuity API onto the existing typed-builtin pattern so app code can field-access authority and record values directly, and later clustered-handler work can build on a real Mesh-facing contract instead of JSON shims.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Typechecker builtin-module and struct registration | Fail compilation with a type error instead of silently leaving `Continuity.*` as `String ! String`. | N/A — compile-time only. | Reject partial struct registration that would let some fields type-check and others drift. |
| MIR/intrinsic/runtime ABI alignment | Stop at compile or runtime tests with the exact symbol/layout mismatch; do not paper over it with string re-encoding. | Fail the e2e run and keep the build/run logs. | Treat wrong payload boxing or field layout as an ABI regression, not a decode fallback. |
| `mesh-rt` staticlib freshness in compiler e2e | Rebuild `mesh-rt` before linking temp projects so stale symbols cannot fake a compiler/runtime failure. | N/A — one explicit build step. | Reject missing/new continuity symbols as stale-artifact drift rather than changing tests to match the stale library. |

## Load Profile

- **Shared resources**: type maps, MIR struct tables, runtime result boxing, and temp-project compiler e2e builds.
- **Per-operation cost**: one `mesh-rt` build plus typed compile/run checks over the new builtin structs.
- **10x breakpoint**: ABI and payload-layout mismatches break before performance matters; the expensive failure mode is repeated temp-project rebuild/link churn.

## Negative Tests

- **Malformed inputs**: wrong arity, wrong field name, and wrong result-shape usage of `Continuity.*` values.
- **Error paths**: runtime promotion rejection and request-key-not-found must still surface as `Err(String)` without decode wrappers.
- **Boundary conditions**: nested typed payloads (`submit().record.*`), completion/ack record updates, and authority/status reads in both primary and standby roles.

## Steps

1. Register builtin continuity struct shapes and typed `Continuity.*` signatures in `compiler/mesh-typeck/src/infer.rs`, following the existing `HttpResponse` precedent instead of inventing a new ad hoc path.
2. Mirror those structs and result payload shapes in `compiler/mesh-codegen/src/mir/lower.rs` and `compiler/mesh-codegen/src/codegen/intrinsics.rs` so field access and lowering agree on layout.
3. Change `compiler/mesh-rt/src/dist/continuity.rs` and `compiler/mesh-rt/src/lib.rs` to return typed Mesh payloads directly from the exported continuity functions rather than JSON strings.
4. Extend `compiler/meshc/tests/e2e_m044_s01.rs` with `typed_continuity_` coverage and either retire or rewrite the old stringly continuity assertions in `compiler/meshc/tests/e2e_m043_s02.rs` so the workspace no longer depends on JSON decoding as the public contract.

## Must-Haves

- [ ] `Continuity.authority_status()`, `status()`, `submit()`, `promote()`, `mark_completed()`, and `acknowledge_replica()` all expose typed Mesh results.
- [ ] The typed continuity structs are registered in both typeck and MIR, with matching runtime payload boxing/layout.
- [ ] Compiler e2e coverage proves both the happy path and compile-time failures for bad typed API usage.

## Inputs

- ``compiler/mesh-typeck/src/infer.rs` — current `Continuity` signatures and builtin struct registration seam.`
- ``compiler/mesh-codegen/src/mir/lower.rs` — current builtin struct layout registration and continuity intrinsic mapping.`
- ``compiler/mesh-codegen/src/codegen/intrinsics.rs` — current stringly continuity runtime declarations.`
- ``compiler/mesh-rt/src/dist/continuity.rs` — typed runtime structs currently serialized to JSON for Mesh-facing calls.`
- ``compiler/meshc/tests/e2e_m044_s01.rs` — manifest/declaration harness from T01 to extend with typed continuity proofs.`
- ``compiler/meshc/tests/e2e_m043_s02.rs` — old stringly continuity test rail that must stop defining the public contract.`

## Expected Output

- ``compiler/mesh-typeck/src/infer.rs` — typed builtin continuity signatures and struct registration.`
- ``compiler/mesh-codegen/src/mir/lower.rs` — continuity struct layouts and lowered typed intrinsic mapping.`
- ``compiler/mesh-codegen/src/codegen/intrinsics.rs` — runtime declarations matching typed continuity payloads.`
- ``compiler/mesh-rt/src/dist/continuity.rs` — typed Mesh-facing continuity exports instead of JSON string wrappers.`
- ``compiler/mesh-rt/src/lib.rs` — continuity export surface aligned with the new ABI.`
- ``compiler/meshc/tests/e2e_m044_s01.rs` — typed continuity compile/runtime coverage with stable prefixes.`
- ``compiler/meshc/tests/e2e_m043_s02.rs` — old stringly assertions removed or narrowed so M043 stays about disaster continuity, not the deprecated API.`

## Verification

`cargo test -p meshc --test e2e_m044_s01 typed_continuity_ -- --nocapture`
`cargo test -p meshc --test e2e_m044_s01 continuity_compile_fail_ -- --nocapture`

## Observability Impact

- Signals added/changed: typed compiler diagnostics for wrong `Continuity.*` field/arity usage and preserved build/run logs for ABI regressions.
- How a future agent inspects this: rerun the `typed_continuity_` / `continuity_compile_fail_` tests and inspect the emitted temp-project build logs.
- Failure state exposed: whether the break is in type registration, MIR lowering, runtime symbol shape, or stale `mesh-rt` artifacts.
