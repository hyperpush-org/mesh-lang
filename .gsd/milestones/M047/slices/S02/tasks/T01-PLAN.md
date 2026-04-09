---
estimated_steps: 4
estimated_files: 5
skills_used:
  - rust-best-practices
  - llvm
---

# T01: Thread replication-count metadata into declared-handler registration

Carry S01's source-resolved `replication_count` through the compiler/codegen/runtime registration seam so the runtime can look up clustered-function counts by runtime name instead of guessing from startup heuristics.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/meshc/src/main.rs` -> `compiler/mesh-codegen/src/declared.rs` declared-handler plan seam | Fail the build with an explicit missing-count / missing-lowered-symbol error instead of emitting a handler with implicit count `0`. | Not applicable beyond test-time command failure; treat a hung codegen test as a failing rail. | Reject mismatched handler metadata in unit tests and keep runtime registration absent rather than silently defaulting. |
| `compiler/mesh-codegen/src/codegen/mod.rs` -> `compiler/mesh-rt/src/dist/node.rs` registration ABI | Refuse to emit startup/declared registrations when the runtime signature and codegen payload drift. | Not applicable beyond compile/test failure. | Fail LLVM marker tests when the emitted intrinsic arguments do not match the runtime contract. |

## Load Profile

- **Shared resources**: declared-handler registry and startup registration list.
- **Per-operation cost**: one extra integer field carried per declared handler / startup registration.
- **10x breakpoint**: registration drift or duplicate-name collisions, not CPU cost; the hot-path overhead should stay trivial.

## Negative Tests

- **Malformed inputs**: missing lowered handler symbols or missing runtime names fail before registration.
- **Error paths**: declared-handler and startup registration tests fail if count metadata is omitted from the emitted/runtime ABI.
- **Boundary conditions**: bare `@cluster` carries default `2`, explicit `@cluster(N)` preserves `N`, and service handlers do not accidentally become startup-work registrations.

## Steps

1. Extend `DeclaredHandlerPlanEntry`, `DeclaredRuntimeRegistration`, and the meshc planning seam to carry the resolved replication count from `ClusteredExecutionMetadata`.
2. Update LLVM/runtime registration plumbing so declared handlers register runtime name, executable symbol, and replication count together.
3. Store the registered count in the runtime declared-handler registry while keeping the startup registry name-only.
4. Add `m047_s02` unit coverage around codegen/runtime registration markers and declared-handler metadata lookup.

## Must-Haves

- [ ] Bare `@cluster` reaches runtime registration as count `2`.
- [ ] Explicit `@cluster(N)` reaches runtime registration without being clipped or dropped.
- [ ] The runtime can resolve replication-count metadata by declared-handler runtime name without inventing a second startup-only registry.

## Inputs

- ``compiler/meshc/src/main.rs``
- ``compiler/mesh-codegen/src/declared.rs``
- ``compiler/mesh-codegen/src/codegen/mod.rs``
- ``compiler/mesh-codegen/src/codegen/intrinsics.rs``
- ``compiler/mesh-rt/src/dist/node.rs``

## Expected Output

- ``compiler/meshc/src/main.rs``
- ``compiler/mesh-codegen/src/declared.rs``
- ``compiler/mesh-codegen/src/codegen/mod.rs``
- ``compiler/mesh-codegen/src/codegen/intrinsics.rs``
- ``compiler/mesh-rt/src/dist/node.rs``

## Verification

cargo test -p mesh-codegen m047_s02 -- --nocapture && cargo test -p mesh-rt m047_s02 -- --nocapture

## Observability Impact

Registration-time failures should become visible through explicit codegen/runtime tests and emitted LLVM markers that include the declared runtime name plus replication count.
