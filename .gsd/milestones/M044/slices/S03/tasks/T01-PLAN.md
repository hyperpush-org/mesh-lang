---
estimated_steps: 13
estimated_files: 5
skills_used: []
---

# T01: Added a runtime-owned operator query/diagnostics seam in mesh-rt for membership, authority, continuity, and recent failover state.

**Slice:** S03 — Built-in Operator Surfaces & Clustered Scaffold
**Milestone:** M044

## Description

S02 made declared clustered execution real, but the only truthful inspection surface is still proof-app HTTP plus stderr. This task makes operator truth runtime-owned by adding a dedicated read-only operator snapshot/query surface in `mesh-rt` that can answer membership, authority, per-key continuity status, and recent failover transitions without depending on app-authored routes or zero-record continuity state.

## Steps

1. Extract a dedicated operator seam under `compiler/mesh-rt/src/dist/` for structured operator data, keeping `node.rs` focused on transport/session mechanics and `continuity.rs` focused on record state transitions.
2. Add authenticated read-only query/reply support over the node transport plus safe Rust helpers that return normalized membership (self + peers), authority, continuity lookup/listing, and bounded recent failover diagnostics.
3. Retain diagnostics as structured runtime entries instead of stderr-only strings, but keep the existing log lines so current proofs and future debugging stay correlated.
4. Add runtime tests that cover zero-record authority/membership truth, bounded diagnostic retention/truncation, and malformed query frame rejection.

## Must-Haves

- [ ] `mesh-rt` exposes a safe read-only operator snapshot/query API without parsing app HTTP or raw FFI payloads in `meshc`.
- [ ] Membership snapshots include the local node plus connected peers and do not disappear when there are zero continuity records.
- [ ] Recent failover/continuity transitions are queryable as structured diagnostics with bounded retention and no cookie leakage.

## Inputs

- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/lib.rs`

## Expected Output

- `compiler/mesh-rt/src/dist/operator.rs`
- `compiler/mesh-rt/src/dist/mod.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/lib.rs`

## Verification

`cargo test -p mesh-rt operator_query_ -- --nocapture`
`cargo test -p mesh-rt operator_diagnostics_ -- --nocapture`

## Observability Impact

- Signals added/changed: structured operator snapshots plus bounded recent failover/continuity diagnostic entries alongside the existing `mesh-rt continuity` transition logs.
- How a future agent inspects this: call the new safe operator helpers or rerun the named `operator_query_` / `operator_diagnostics_` runtime tests.
- Failure state exposed: query kind, target node, timeout/malformed-frame reason, truncation/buffer status, and the latest request/attempt identifiers without secret values.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Node transport query frames in `compiler/mesh-rt/src/dist/node.rs` | Return a structured read-only query error; do not fall back to app HTTP. | Bound the query wait and include target/query kind in the error. | Reject the frame and surface decode failure without partially hydrating a snapshot. |
| Continuity diagnostic retention in `compiler/mesh-rt/src/dist/continuity.rs` | Keep the existing stderr transition logs and return an explicit empty/partial diagnostic bundle. | N/A — in-process buffer read. | Drop invalid entries and expose truncation/parse failure instead of panicking. |
| Safe Rust operator API exported through `compiler/mesh-rt/src/lib.rs` | Fail the caller with typed `Result` context rather than raw pointer decoding. | N/A — local API. | Reject missing or invalid payloads before they reach `meshc`. |

## Load Profile

- **Shared resources**: live node sessions, continuity registry reads, and bounded diagnostic storage.
- **Per-operation cost**: one snapshot read plus one read-only query/reply frame per remote inspection.
- **10x breakpoint**: diagnostic buffer churn or query serialization latency before CPU; retention must stay bounded instead of growing with cluster lifetime.

## Negative Tests

- **Malformed inputs**: invalid query kind, missing request key for per-key lookup, and bogus node names in query frames.
- **Error paths**: unreachable peer, auth mismatch, and zero-record authority snapshots.
- **Boundary conditions**: empty continuity set, truncated diagnostic ring buffer, and membership on a just-started node with only self plus zero peers.
