---
estimated_steps: 4
estimated_files: 6
skills_used:
  - rust-best-practices
  - test
---

# T02: Expose explicit promotion and live authority readback through the Continuity API

**Slice:** S02 — Standby Promotion and Stale-Primary Fencing
**Milestone:** M043

## Description

Add the narrow Mesh-visible continuity surface that S02 needs: an explicit promote action and a read-only authority-status call backed directly by `mesh-rt`. The goal is to let Mesh code trigger operator-approved promotion and read current authority truth without re-deriving role, epoch, or health from env.

Keep the API boring and runtime-owned. The compiler, typechecker, codegen, and runtime-export seam should only forward the minimal new intrinsics needed by `cluster-proof`, and the task should add focused compiler-facing proof coverage so future slices can reuse the same surface.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Runtime C ABI surface in `compiler/mesh-rt/src/dist/continuity.rs` and `compiler/mesh-rt/src/lib.rs` | Return `Result`-style error payloads for invalid promotion or unavailable authority state; never panic. | Preserve existing authority truth and return an explicit timeout or unavailable error instead of blocking the caller. | Reject malformed runtime result payloads before they reach Mesh code. |
| Compiler/typechecker/codegen seam in `compiler/mesh-typeck/src/infer.rs`, `compiler/mesh-codegen/src/mir/lower.rs`, and `compiler/mesh-codegen/src/codegen/intrinsics.rs` | Fail compilation with a specific missing-intrinsic or type error instead of silently lowering to the wrong runtime call. | N/A — this seam is compile-time only. | Reject wrong arity or wrong result-shape calls at typecheck time. |
| API proof coverage in `compiler/meshc/tests/e2e_m043_s02.rs` | Fail closed if the new API cannot be invoked from Mesh source or returns the wrong JSON shape. | Preserve logs and temporary source artifacts that show which API call stalled. | Fail on missing fields or wrong result tags rather than accepting partial status data. |

## Load Profile

- **Shared resources**: runtime authority snapshot reads, promotion call serialization, and the compiler's intrinsic registry.
- **Per-operation cost**: one runtime call plus one result decode; compile-time cost is a small intrinsic and type-scheme addition.
- **10x breakpoint**: repeated authority-status polling will stress the runtime status encode path first, not the compiler seam.

## Negative Tests

- **Malformed inputs**: wrong-arity Mesh calls, invalid promotion attempts from the wrong authority state, and malformed result payloads.
- **Error paths**: repeated promotion, promotion while authority is already at the same or newer epoch, and authority-status reads when the runtime has not initialized continuity state yet.
- **Boundary conditions**: first promotion to epoch `1`, idempotent status reads before and after promotion, and failure payloads that preserve the current authoritative state.

## Steps

1. Add narrow runtime exports for promotion and authority-status readback in `compiler/mesh-rt/src/dist/continuity.rs` and re-export them from `compiler/mesh-rt/src/lib.rs`.
2. Wire the new calls through the built-in `Continuity` module in `compiler/mesh-typeck/src/infer.rs`, `compiler/mesh-codegen/src/mir/lower.rs`, and `compiler/mesh-codegen/src/codegen/intrinsics.rs`.
3. Add focused compiler-facing tests in `compiler/meshc/tests/e2e_m043_s02.rs` with names prefixed `continuity_api_` so the task-level verification filter stays truthful.
4. Verify that Mesh source can call the new API, repeated promotion and invalid-state paths fail explicitly, and authority-status reads return the runtime's current truth.

## Must-Haves

- [ ] Mesh code can call explicit continuity promotion and authority-status APIs backed directly by `mesh-rt`.
- [ ] The compiler seam rejects wrong-arity or wrong-shape uses instead of silently lowering to the wrong intrinsic.
- [ ] Runtime promotion and status calls return explicit `Result`-style success or error payloads without panic-driven control flow.
- [ ] Compiler-facing tests in `compiler/meshc/tests/e2e_m043_s02.rs` exercise real assertions and keep the `continuity_api_` verification filter honest.

## Verification

- `cargo test -p meshc --test e2e_m043_s02 continuity_api_ -- --nocapture`

## Observability Impact

- Signals added/changed: promotion result payloads and authority-status readback expose explicit runtime role, epoch, health, and failure reasons through one stable Mesh-facing contract.
- How a future agent inspects this: run the `continuity_api_` tests in `compiler/meshc/tests/e2e_m043_s02.rs` and inspect the temporary Mesh source plus stderr when the compiler or runtime seam drifts.
- Failure state exposed: missing intrinsic wiring, invalid promotion state, and malformed authority-status payloads fail at the API boundary instead of leaking into `cluster-proof` as stale config truth.

## Inputs

- `compiler/mesh-rt/src/dist/continuity.rs` — runtime authority and promotion core from T01 that needs a Mesh-visible C ABI surface.
- `compiler/mesh-rt/src/lib.rs` — current runtime re-export seam for continuity functions.
- `compiler/mesh-typeck/src/infer.rs` — built-in `Continuity` module signatures.
- `compiler/mesh-codegen/src/mir/lower.rs` — intrinsic name lowering for built-in module calls.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — declared runtime intrinsic signatures.
- `compiler/meshc/tests/e2e_m042_s03.rs` — existing continuity API test style and runtime proof patterns.
- `.gsd/milestones/M043/slices/S02/S02-RESEARCH.md` — rationale for explicit promotion and runtime-backed authority truth.

## Expected Output

- `compiler/mesh-rt/src/dist/continuity.rs` — narrow Mesh-visible promotion and authority-status exports over the new runtime authority core.
- `compiler/mesh-rt/src/lib.rs` — re-exports for the new continuity runtime entrypoints.
- `compiler/mesh-typeck/src/infer.rs` — built-in `Continuity` signatures updated for the new APIs.
- `compiler/mesh-codegen/src/mir/lower.rs` — lowering rules that map the new built-in calls to runtime intrinsics.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — declared runtime intrinsic signatures for promotion and authority-status.
- `compiler/meshc/tests/e2e_m043_s02.rs` — compiler-facing proof coverage for the new continuity APIs.
