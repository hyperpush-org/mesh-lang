---
estimated_steps: 4
estimated_files: 5
skills_used:
  - rust-best-practices
  - debug-like-expert
  - test
---

# T01: Make declared-work wrappers remotely spawnable across nodes

**Slice:** S02 — Tiny End-to-End Clustered Example
**Milestone:** M045

## Description

Repair the runtime/codegen seam so a manifest-declared work wrapper generated as `__declared_work_*` can be found by the remote `mesh_node_spawn(...)` path without widening undeclared helpers into the public execution surface.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Generic remote function registry in `compiler/mesh-codegen/src/codegen/mod.rs` and `compiler/mesh-rt/src/dist/node.rs` | Fail the build/tests loudly on missing registration or lookup drift; do not fall back to local spawn. | N/A — registration and lookup are synchronous. | Reject unknown wrappers with an explicit runtime error instead of coercing to a different executable symbol. |
| Declared-handler planning in `compiler/mesh-codegen/src/declared.rs` and `compiler/meshc/tests/e2e_m044_s02.rs` | Keep manifest gating intact so undeclared locals remain absent from remote execution. | N/A — compile/e2e coverage is bounded. | Treat wrong wrapper/runtime-name pairing as a contract regression, not a best-effort alias. |

## Load Profile

- **Shared resources**: function registry, declared-handler registry, and remote node session state.
- **Per-operation cost**: one runtime registration plus one remote spawn lookup/dispatch.
- **10x breakpoint**: registry drift and session reconnect churn fail before throughput does; the seam must remain narrow and explicit.

## Negative Tests

- **Malformed inputs**: undeclared runtime target, missing wrapper symbol, and wrapper/runtime-name mismatch.
- **Error paths**: remote spawn rejection on missing function, reconnect/retry drift, and accidental widening of manifestless helpers.
- **Boundary conditions**: local-owner declared work still runs, remote-owner declared work no longer rejects, and service-declaration rails stay protected.

## Steps

1. Update the generated declared-work registration path so manifest-approved wrappers remain remote-spawnable even though the raw function name starts with `__`.
2. Keep the runtime registry/lookup surface explicit and fail closed when a runtime name or wrapper symbol is not manifest-approved.
3. Seed a dedicated `compiler/meshc/tests/e2e_m045_s02.rs` regression for the remote-owner seam while preserving the existing M044 declared-handler coverage.
4. Re-run the focused declared-handler rails to prove remote-owner submits stop failing at `function not found __declared_work_*`.

## Must-Haves

- [ ] Remote-owner declared-work submits can find the generated wrapper symbol on the owner node.
- [ ] Undeclared helpers remain absent from remote execution.
- [ ] Existing declared service/work coverage still protects the manifest gate.
- [ ] A new M045/S02 test file exists from the first task onward.

## Verification

- `cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture`
- `cargo test -p meshc --test e2e_m045_s02 m045_s02_declared_work_remote_spawn_ -- --nocapture`

## Observability Impact

- Signals added/changed: explicit remote declared-work registration/lookup behavior and preserved remote-spawn rejection diagnostics.
- How a future agent inspects this: focused `e2e_m044_s02` / `e2e_m045_s02` logs plus owner-node stderr when lookup fails.
- Failure state exposed: wrapper missing vs undeclared target drift stays distinguishable instead of collapsing into a silent fallback.

## Inputs

- `compiler/mesh-codegen/src/codegen/mod.rs` — current generic remote function registration loop.
- `compiler/mesh-codegen/src/declared.rs` — declared-work wrapper generation and runtime registration metadata.
- `compiler/mesh-rt/src/dist/node.rs` — remote spawn lookup/dispatch path.
- `compiler/meshc/tests/e2e_m044_s02.rs` — existing manifest-gating coverage that must stay green.

## Expected Output

- `compiler/mesh-codegen/src/codegen/mod.rs` — remote-spawn registration includes the manifest-approved declared wrapper seam.
- `compiler/mesh-codegen/src/declared.rs` — declared-work wrapper planning stays explicit and manifest-gated.
- `compiler/mesh-rt/src/dist/node.rs` — runtime lookup continues to fail closed while finding the approved wrapper.
- `compiler/meshc/tests/e2e_m044_s02.rs` — existing declared-handler rails remain truthful.
- `compiler/meshc/tests/e2e_m045_s02.rs` — new S02 regression file exists with a named remote-spawn test.
