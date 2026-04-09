---
estimated_steps: 4
estimated_files: 6
skills_used:
  - rust-best-practices
  - test
---

# T02: Lower clustered route wrappers onto the shared declared-handler registration seam

**Slice:** S07 — Clustered HTTP route wrapper completion
**Milestone:** M047

## Description

Once typecheck captures truthful wrapper metadata, lower it through the same runtime-name and replication-count registry used by ordinary clustered functions. Generate bare route shims that keep the public handler signature `fn(Request) -> Response`, preserve the real handler runtime name, and keep startup-work registration unchanged.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| MIR lowering of route-wrapper metadata | fail codegen with a named lowering error; do not silently emit ordinary local routes when wrapper metadata is missing | N/A | reject inconsistent shim plans or conflicting counts instead of guessing |
| declared-handler registration emission | keep route handlers on the shared runtime-name and replication-count registry or fail before LLVM emission succeeds | N/A | malformed runtime names or missing lowered symbols are test failures, not fallback-to-local behavior |
| startup-work filtering | keep clustered routes out of startup registration or fail a focused assertion; do not auto-start HTTP handlers | N/A | a route handler appearing in startup registrations is a contract failure |

## Load Profile

- **Shared resources**: typecheck metadata maps, merged MIR, declared-handler planning, and LLVM registration markers.
- **Per-operation cost**: one lowering/codegen pass plus focused unit assertions.
- **10x breakpoint**: duplicate shim generation and symbol drift fail before throughput matters.

## Negative Tests

- **Malformed inputs**: missing lowered symbols, conflicting duplicate counts for the same handler, and wrapper metadata that does not resolve to a concrete route call.
- **Error paths**: generated shims without declared-handler registration, route handlers leaking into startup registration, or runtime names collapsing to shim-only identifiers.
- **Boundary conditions**: default and explicit counts both reach the declared-handler registry, identical repeated wrappers dedupe cleanly, and direct/piped route forms lower the same way.

## Steps

1. Extend `PreparedBuild` and the route-lowering seam so wrapper metadata from typecheck reaches MIR lowering without teaching `mesh-pkg::collect_source_cluster_declarations(...)` to parse route wrappers.
2. Teach MIR lowering and/or HTTP route call lowering to replace `HTTP.clustered(...)` with generated bare route shims that preserve the actual handler runtime name and fail closed on conflicting count reuse.
3. Add a route-capable declared-handler kind and registration path so explicit/default counts reach `mesh_register_declared_handler`, while `prepare_startup_work_registrations` continues to filter startup-only work.
4. Add focused `mesh-codegen` tests for shim generation, runtime-name/count markers, conflict handling, missing lowered symbols, and route handlers excluded from startup registration.

## Must-Haves

- [ ] Clustered routes reuse the ordinary declared-handler runtime-name and replication-count seam.
- [ ] Generated route shims keep the public handler signature while preserving the real handler identity.
- [ ] Clustered HTTP handlers are never added to startup-work registration.

## Verification

- `cargo test -p mesh-codegen m047_s07 -- --nocapture`

## Inputs

- `compiler/mesh-typeck/src/infer.rs` — wrapper metadata and imported-origin capture from T01.
- `compiler/mesh-typeck/src/lib.rs` — `TypeckResult` fields that hand wrapper metadata to lowering.
- `compiler/meshc/src/main.rs` — current `PreparedBuild` and declared-handler plan plumbing.
- `compiler/mesh-codegen/src/mir/lower.rs` — MIR lowering seam that already consumes typecheck metadata maps.
- `compiler/mesh-codegen/src/declared.rs` — declared-handler wrapper planning and startup filtering.
- `compiler/mesh-codegen/src/codegen/mod.rs` — LLVM registration emission for declared handlers.
- `compiler/mesh-codegen/src/codegen/expr.rs` — current plain-fn HTTP route ABI constraints.
- `compiler/mesh-pkg/src/manifest.rs` — route-wrapper non-goal boundary; do not teach manifest collection to parse `HTTP.clustered(...)`.

## Expected Output

- `compiler/meshc/src/main.rs` — route-wrapper metadata carried into declared-handler planning.
- `compiler/mesh-codegen/src/mir/lower.rs` — generated clustered route shim lowering and wrapper replacement.
- `compiler/mesh-codegen/src/declared.rs` — route-capable declared-handler planning that preserves counts and skips startup registration.
- `compiler/mesh-codegen/src/codegen/mod.rs` — LLVM registration emission for clustered route handlers.
- `compiler/mesh-codegen/src/codegen/expr.rs` — route-call lowering that still honors the plain fn-pointer ABI.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — any new route registration intrinsics needed by the lowered wrapper path.
