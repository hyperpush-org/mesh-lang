---
estimated_steps: 4
estimated_files: 5
skills_used:
  - rust-best-practices
  - debug-like-expert
  - test
---

# T03: Add runtime clustered-route dispatch and continuity truth for request/response handlers

**Slice:** S07 — Clustered HTTP route wrapper completion
**Milestone:** M047

## Description

The HTTP runtime already knows how to build `MeshHttpRequest` and crack `MeshHttpResponse`; the missing seam is clustered execution. This task should bridge that boundary into continuity and declared-handler execution without widening generic closure routes or generic remote-spawn arg tags.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| clustered route dispatch from HTTP server | fail the request with an explicit 5xx/503 path plus continuity evidence; do not silently fall back to local direct invocation | bound request waits and fail the focused runtime/e2e rails with retained logs | reject malformed request/response encode-decode as a transport failure, not as an empty 200 |
| declared-handler continuity integration | keep `declared_handler_runtime_name`, `replication_count`, and rejection reasons truthful or fail the runtime tests | preserve current bounded continuity waits; do not add unbounded polling | malformed continuity records are a contract failure, not acceptable best effort |
| operator/inspection surfaces | route records must remain queryable even when multiple requests share one runtime name; do not depend on list order | N/A | malformed list ordering or missing request keys should fail the focused helper assertions |

## Load Profile

- **Shared resources**: HTTP server actors, continuity registry state, cluster membership, and route request/response serialization.
- **Per-operation cost**: one clustered HTTP request plus continuity submit/complete bookkeeping; unsupported higher fanout should reject durably before extra work starts.
- **10x breakpoint**: request-key generation, serialization correctness, and replica-availability checks fail before throughput matters.

## Negative Tests

- **Malformed inputs**: invalid or empty request-key/payload-hash generation, malformed encoded request/response payloads, and missing route metadata on dispatch.
- **Error paths**: unsupported explicit counts produce durable rejection plus HTTP failure, and clustered dispatch never degrades into a silent local-success path.
- **Boundary conditions**: default-count route success, repeated requests against one runtime name, and request/response roundtrip fidelity all stay truthful together.

## Steps

1. Extend route entries and HTTP server dispatch so a route can carry clustered runtime metadata and submit/complete a declared handler around the actual route handler invocation instead of always calling `call_handler` directly.
2. Implement request-key and payload-hash generation plus the route request/response transport that serializes `MeshHttpRequest`, runs the real handler as the clustered boundary, and returns `MeshHttpResponse` without widening generic `spawn_declared_work` arg tags.
3. Surface route execution outcome through continuity and operator diagnostics with request key, handler runtime name, count, phase/result, and explicit `unsupported_replication_count:3` rejection while keeping request bodies out of continuity.
4. Add focused `mesh-rt` tests for request/response roundtrip, successful default-count route completion, rejected explicit-count route flow, and repeated runtime-name inspection behavior.

## Must-Haves

- [ ] The HTTP route handler, not a downstream helper, is the clustered boundary.
- [ ] Runtime continuity truth for clustered routes reuses the declared-handler registry and exposes the same runtime-name/count fields as ordinary clustered work.
- [ ] Unsupported explicit fanout returns an HTTP failure contract with a durable continuity rejection reason.

## Verification

- `cargo test -p mesh-rt m047_s07 -- --nocapture`

## Observability Impact

- Signals added/changed: clustered HTTP route continuity records/logs, request-key-to-route-handler correlation, and durable rejection reasons for unsupported fanout.
- How a future agent inspects this: rerun `cargo test -p mesh-rt m047_s07 -- --nocapture` and inspect continuity/operator snapshots plus retained request/response assertions.
- Failure state exposed: transport encode/decode drift, replica-availability rejection, and repeated-runtime-name list behavior become explicit instead of hiding behind generic HTTP failures.

## Inputs

- `compiler/mesh-codegen/src/mir/lower.rs` — generated clustered route shims and runtime metadata from T02.
- `compiler/mesh-codegen/src/declared.rs` — declared-handler registration shape the runtime must honor.
- `compiler/mesh-rt/src/http/router.rs` — current route-entry storage and matching logic.
- `compiler/mesh-rt/src/http/server.rs` — request construction, handler invocation, and response extraction seam.
- `compiler/mesh-rt/src/dist/node.rs` — declared-handler registry lookup plus current submit/spawn limits.
- `compiler/mesh-rt/src/dist/continuity.rs` — continuity record truth surface.
- `compiler/mesh-rt/src/dist/operator.rs` — continuity list/query ordering behavior for repeated runtime names.

## Expected Output

- `compiler/mesh-rt/src/http/router.rs` — route entries that can carry clustered handler metadata.
- `compiler/mesh-rt/src/http/server.rs` — clustered route dispatch path that submits, executes, and returns `MeshHttpResponse`.
- `compiler/mesh-rt/src/dist/node.rs` — route-capable declared-handler execution that still reuses runtime-name/count truth.
- `compiler/mesh-rt/src/dist/continuity.rs` — truthful route continuity records and rejection handling.
- `compiler/mesh-rt/src/dist/operator.rs` — inspection helpers that stay usable when clustered routes create repeated records for one runtime name.
