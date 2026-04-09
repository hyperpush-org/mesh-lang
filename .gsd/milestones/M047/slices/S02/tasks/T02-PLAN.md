---
estimated_steps: 4
estimated_files: 7
skills_used:
  - rust-best-practices
  - rust-testing
---

# T02: Apply replication-count semantics to continuity records and operator truth

Make the runtime use the registered count honestly: derive required replicas from the declared-handler metadata, preserve the requested count in continuity state and operator surfaces, and fail closed when the current single-replica runtime cannot satisfy a requested replication factor.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Runtime continuity submit/merge/recovery paths in `compiler/mesh-rt/src/dist/continuity.rs` and `compiler/mesh-rt/src/dist/node.rs` | Reject the request with an explicit reason and keep the record queryable instead of silently degrading the count. | Keep the existing startup/recovery timeout path explicit and record the count-bearing rejection/timeout in diagnostics. | Treat malformed count/runtime metadata as invalid continuity state and reject before dispatch. |
| Operator serialization + CLI rendering in `compiler/mesh-rt/src/dist/operator.rs` and `compiler/meshc/src/cluster.rs` | Fail the query/CLI rail with a concrete field-mismatch error rather than dropping the new count field. | Surface the same operator timeout as today; do not invent a fallback record without count truth. | Reject decode drift between runtime/operator/CLI structs instead of silently omitting `replication_count`. |

## Load Profile

- **Shared resources**: continuity registry, operator query buffers, startup/recovery dispatch path.
- **Per-operation cost**: one extra count field carried through record encode/decode and a small amount of validation per submit/recovery.
- **10x breakpoint**: topology/count mismatch handling and record serialization drift, not raw CPU time.

## Negative Tests

- **Malformed inputs**: negative/oversized requested counts and missing runtime-name metadata reject explicitly.
- **Error paths**: unsupported replication factors or insufficient topology fail closed with durable continuity truth instead of local success.
- **Boundary conditions**: bare `@cluster` maps to replication count `2`, single-node clustered startup stays valid with count-aware semantics, and explicit count truth survives direct submit plus automatic recovery surfaces.

## Steps

1. Add `replication_count` to continuity record / Mesh continuity payload structs and update encode/decode, typeck builtins, and MIR builtin struct definitions together.
2. Derive `required_replica_count` from the registered declared-handler count for startup, direct submit, and automatic recovery instead of the current hardcoded `0`/`1` behavior.
3. Fail closed for unsupported requested replication factors or topologies instead of pretending the old single-replica runtime honored them.
4. Extend operator and `meshc cluster continuity` JSON/human output to render runtime name plus replication count, then add `m047_s02` runtime tests covering default count, explicit count preservation, and rejection paths.

## Must-Haves

- [ ] Runtime continuity records preserve requested replication count for ordinary clustered functions.
- [ ] Startup, direct submit, and recovery all derive replica requirements from declared-handler metadata instead of hardcoded defaults.
- [ ] Unsupported replication factors/topologies reject explicitly and stay visible through continuity/operator truth.

## Inputs

- ``compiler/mesh-rt/src/dist/continuity.rs``
- ``compiler/mesh-rt/src/dist/node.rs``
- ``compiler/mesh-rt/src/dist/operator.rs``
- ``compiler/meshc/src/cluster.rs``
- ``compiler/mesh-typeck/src/builtins.rs``
- ``compiler/mesh-typeck/src/infer.rs``
- ``compiler/mesh-codegen/src/mir/lower.rs``
- ``compiler/meshc/src/main.rs``
- ``compiler/mesh-codegen/src/declared.rs``

## Expected Output

- ``compiler/mesh-rt/src/dist/continuity.rs``
- ``compiler/mesh-rt/src/dist/node.rs``
- ``compiler/mesh-rt/src/dist/operator.rs``
- ``compiler/meshc/src/cluster.rs``
- ``compiler/mesh-typeck/src/builtins.rs``
- ``compiler/mesh-typeck/src/infer.rs``
- ``compiler/mesh-codegen/src/mir/lower.rs``

## Verification

cargo test -p mesh-rt m047_s02 -- --nocapture

## Observability Impact

`meshc cluster continuity` and runtime diagnostics should expose replication count, derived required replicas, and explicit rejection reasons so failures localize to count semantics instead of generic continuity drift.
