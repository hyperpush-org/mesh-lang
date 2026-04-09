---
estimated_steps: 4
estimated_files: 6
skills_used:
  - rust-best-practices
  - test
---

# T02: Expose `Node.start_from_env()` through the compiler and add bootstrap e2e rails

**Slice:** S01 — Runtime-Owned Cluster Bootstrap
**Milestone:** M045

## Description

Wire the new bootstrap boundary through typeck, MIR lowering, intrinsic declarations, and codegen so Mesh code can call it directly, then prove the public API with a dedicated M045 compiler e2e target.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Node builtin registration across typeck / MIR / codegen | Fail the compile or tests loudly on missing symbol, wrong arity, or stale lowering. | N/A — compile-time plumbing is synchronous. | Reject field/shape drift rather than boxing status back into strings. |
| Runtime export / Mesh result layout | Stop the e2e target with the exact ABI mismatch; do not fall back to app-owned env parsing. | N/A — temp-project compile/run is bounded by Cargo. | Treat wrong payload boxing or field layout as a contract regression. |

## Load Profile

- **Shared resources**: compiler builtin tables, temp-project build output, and the `mesh-rt` staticlib freshness requirement.
- **Per-operation cost**: one runtime rebuild plus one temp-project compile/run per proof case.
- **10x breakpoint**: stale `mesh-rt` artifacts and repeated relink churn fail before performance matters; the task must keep the ABI surface narrow and explicit.

## Negative Tests

- **Malformed inputs**: wrong-arity calls, missing-field access on the status struct, and invalid use as an `Int` return.
- **Error paths**: malformed bootstrap env returned as `Err(String)` through Mesh code, and runtime bootstrap failure propagating without a string decode shim.
- **Boundary conditions**: standalone no-op startup, explicit `MESH_NODE_NAME`, and Fly identity without an explicit node name.

## Steps

1. Register the new Node builtin and bootstrap status type in `compiler/mesh-typeck` and `compiler/mesh-codegen` alongside the existing `Node.start(...)` primitive.
2. Export the runtime symbol and typed result layout so the generated LLVM calls the new bootstrap helper directly.
3. Add `compiler/meshc/tests/e2e_m045_s01.rs` with named `m045_s01_bootstrap_api_` coverage for standalone, explicit-node, Fly-identity, and malformed-env cases.
4. Keep the runtime build freshness hook or equivalent in place so temp-project linking cannot silently use stale `mesh-rt` symbols.

## Must-Haves

- [ ] Mesh code can call `Node.start_from_env()` and inspect typed status fields directly.
- [ ] Typeck, MIR, intrinsics, and runtime exports agree on arity, symbol name, and payload layout.
- [ ] `compiler/meshc/tests/e2e_m045_s01.rs` proves both happy-path and fail-closed bootstrap behavior.

## Verification

- `cargo test -p meshc --test e2e_m045_s01 m045_s01_bootstrap_api_ -- --nocapture`
- `cargo test -p meshc --test e2e_m045_s01 m045_s01_bootstrap_api_fail_closed_ -- --nocapture`

## Observability Impact

- Signals added/changed: typed Mesh-facing bootstrap success/error surface instead of app-owned env parsing and log-only decisions.
- How a future agent inspects this: named `m045_s01_bootstrap_api_` compiler e2e cases and compile-time errors for bad field/arity usage.
- Failure state exposed: exact symbol/arity/payload-layout drift at the compiler/runtime seam.

## Inputs

- `compiler/mesh-rt/src/dist/bootstrap.rs` — runtime bootstrap helper and status definitions from T01.
- `compiler/mesh-rt/src/dist/node.rs` — integrated runtime bootstrap entrypoint.
- `compiler/mesh-rt/src/lib.rs` — runtime export surface to extend.
- `compiler/mesh-typeck/src/infer.rs` — Node module typing surface.
- `compiler/mesh-typeck/src/builtins.rs` — builtin environment registration for new Node symbols/types.
- `compiler/mesh-codegen/src/mir/lower.rs` — builtin lowering map and typed struct registration.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — intrinsic declarations for the new runtime symbol.
- `compiler/mesh-codegen/src/codegen/expr.rs` — call lowering for the new Node helper.

## Expected Output

- `compiler/mesh-typeck/src/infer.rs` — typed `Node.start_from_env()` signature.
- `compiler/mesh-typeck/src/builtins.rs` — bootstrap status type and builtin registration.
- `compiler/mesh-codegen/src/mir/lower.rs` — lowering map and typed struct layout updates.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — runtime intrinsic declaration for the bootstrap helper.
- `compiler/mesh-codegen/src/codegen/expr.rs` — codegen path for `Node.start_from_env()`.
- `compiler/meshc/tests/e2e_m045_s01.rs` — dedicated compiler e2e target for the new public bootstrap API.
