---
estimated_steps: 4
estimated_files: 8
skills_used:
  - test
---

# T03: Retire manual `Continuity.promote()` from Mesh/compiler surfaces

**Slice:** S04 — Bounded Automatic Promotion
**Milestone:** M044

## Description

D185 and R067 make the failover control mode auto-only. Even with runtime auto-promotion working, leaving `Continuity.promote()` callable from Mesh code would keep a manual override seam alive. This task removes that public control surface while preserving the internal Rust authority-transition helper the runtime needs for bounded automatic promotion.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Mesh builtin/typechecker surface in `compiler/mesh-typeck/src/` | Fail compile with a clear unsupported/manual-surface diagnostic. | N/A — compile-time path. | Reject malformed promote calls at compile time instead of lowering them. |
| Codegen/runtime export wiring in `compiler/mesh-codegen/src/` and `compiler/mesh-rt/src/lib.rs` | Remove the manual promote entrypoint cleanly; do not leave a dead intrinsic or unresolved symbol. | N/A — compile/link path. | Treat stale promote references as compile/link failures, not runtime surprises. |
| Historical compiler/e2e coverage | Update or replace expectations so the repo does not keep green tests for a contract that no longer exists. | N/A — test path. | Fail closed on stale manual-promotion assertions. |

## Load Profile

- **Shared resources**: compiler builtin registry, codegen intrinsic table, and runtime export surface.
- **Per-operation cost**: trivial compile-time symbol resolution; the important risk is surface drift, not throughput.
- **10x breakpoint**: N/A — correctness task rather than a hot runtime path.

## Negative Tests

- **Malformed inputs**: wrong-arity `Continuity.promote(...)`, promote calls in ordinary Mesh code, and stale manual-promotion proof fixtures.
- **Error paths**: removed intrinsic or export still referenced by generated code, or old tests still expecting a successful manual promote result.
- **Boundary conditions**: `Continuity.authority_status()` still works, but any manual promote call fails closed with a clear diagnostic.

## Steps

1. Remove the Mesh-visible `Continuity.promote()` builtin, lowering, intrinsic, and export path while keeping the internal Rust promotion helper the runtime auto-promotion logic uses.
2. Update compile and e2e coverage so manual promotion attempts now fail closed explicitly and `Continuity.authority_status()` remains the read-only Mesh-visible authority seam.
3. Audit stale tests and compatibility fixtures that still assume manual promotion succeeds, and retarget them to the new auto-only contract.
4. Keep the failure surface specific enough that app authors learn the operator contract changed instead of seeing a generic unresolved symbol or parse failure.

## Must-Haves

- [ ] Mesh application code can no longer change authority manually through `Continuity.promote()`.
- [ ] The runtime still retains the internal authority-transition seam needed by T01 and T02 auto-promotion logic.
- [ ] Compiler, runtime, and test surfaces fail closed with a clear manual-promotion-disabled diagnostic instead of stale success or dead-symbol errors.

## Verification

- `cargo test -p meshc --test e2e_m044_s04 m044_s04_manual_surface_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s01 m044_s01_typed_continuity_ -- --nocapture`
- `cargo test -p meshc --test e2e_m043_s02 continuity_api_ -- --nocapture`

## Observability Impact

- Signals added/changed: explicit compiler/test rejection for manual promotion attempts, while `Continuity.authority_status()` and runtime/operator diagnostics remain the supported inspection surfaces.
- How a future agent inspects this: run the named compiler/e2e filters and confirm the diagnostic message names the auto-only failover contract rather than an unrelated symbol error.
- Failure state exposed: where the stale manual surface still leaked into type checking, lowering, or runtime export wiring.

## Inputs

- `compiler/mesh-typeck/src/infer.rs` — builtin typing surface for Mesh continuity APIs.
- `compiler/mesh-typeck/src/builtins.rs` — builtin registration for Mesh continuity modules.
- `compiler/mesh-codegen/src/mir/lower.rs` — builtin lowering/intrinsic dispatch.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — declared LLVM/runtime intrinsic definitions.
- `compiler/mesh-rt/src/dist/continuity.rs` — current exported manual promote entrypoint.
- `compiler/mesh-rt/src/lib.rs` — public runtime export list.
- `compiler/meshc/tests/e2e_m044_s01.rs` — existing typed continuity API coverage.
- `compiler/meshc/tests/e2e_m043_s02.rs` — historical manual-promotion e2e coverage that must stop passing dishonestly.

## Expected Output

- `compiler/mesh-typeck/src/infer.rs` — no Mesh-visible promote builtin.
- `compiler/mesh-typeck/src/builtins.rs` — auto-only continuity surface registration.
- `compiler/mesh-codegen/src/mir/lower.rs` — no manual promote lowering path.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — no manual promote intrinsic in generated code.
- `compiler/mesh-rt/src/dist/continuity.rs` — internal-only promotion helper retained for runtime use.
- `compiler/mesh-rt/src/lib.rs` — manual promote export removed from the public surface.
- `compiler/meshc/tests/e2e_m044_s01.rs` — compile-fail/read-only authority assertions updated.
- `compiler/meshc/tests/e2e_m043_s02.rs` — stale historical expectations retargeted or removed.
