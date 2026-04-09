---
estimated_steps: 4
estimated_files: 3
skills_used:
  - debug-like-expert
  - test
---

# T01: Add a runtime-owned correlation header to clustered HTTP responses

**Slice:** S02 — Clustered HTTP request correlation
**Milestone:** M054

## Description

Land the single runtime seam this slice depends on. The clustered HTTP server already generates the continuity request key before dispatch; expose that key as an operator-facing response header on both successful and runtime-generated rejection responses, preserve any handler-supplied headers, and prove direct continuity lookup at the lower clustered-route rail without widening routing semantics.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| clustered HTTP response construction / handler header merge | Fail closed and keep the existing route response body/status; do not ship a path that drops app headers or hides rejection reasons. | Treat hung route execution or direct-record polling as a runtime regression, archive the last response/continuity artifacts, and stop. | Reject responses missing the correlation header or returning header values that do not match continuity lookup. |
| continuity direct lookup via existing `meshc cluster` surfaces | Stop the low-level rail if the returned header value cannot retrieve exactly one record on both nodes. | Bound lookup polling and archive the last continuity JSON rather than falling back to before/after diff. | Fail closed if the returned record is missing `request_key`, `declared_handler_runtime_name`, `phase`, or `result`. |

## Load Profile

- **Shared resources**: continuity registry, clustered-route request sequence, response-header map allocation, and the dual-node route proof harness.
- **Per-operation cost**: one extra response header and one direct continuity lookup per verified request; runtime work itself stays the same.
- **10x breakpoint**: continuity record churn and proof polling before the header injection overhead matters.

## Negative Tests

- **Malformed inputs**: empty runtime name/payload identity generation stays rejected, and malformed response/header parsing in the e2e fails closed.
- **Error paths**: unsupported replication count `503` still returns the correlation header plus a rejected continuity record.
- **Boundary conditions**: app-supplied headers survive alongside the new header, and repeated same-runtime requests get unique keys without exact numeric suffix assertions.

## Steps

1. Update `compiler/mesh-rt/src/http/server.rs` so `clustered_route_response_from_request(...)` attaches `X-Mesh-Continuity-Request-Key` after request-key generation on success and on runtime-generated rejection responses where a request key exists.
2. Add or extend `mesh-rt` unit coverage in `compiler/mesh-rt/src/http/server.rs` for header injection, handler-header preservation, and rejection-path correlation without asserting specific numeric request-id suffixes.
3. Update `compiler/meshc/tests/e2e_m047_s07.rs` to read the response header from raw HTTP, use it for direct `meshc cluster continuity <node> <request-key> --json` lookups on both nodes, and keep the old diff-helper unit coverage as separate guardrails.
4. Retain raw HTTP plus continuity artifacts so a missing or mismatched header is diagnosable without reopening the runtime manually.

## Must-Haves

- [ ] Clustered HTTP success responses include `X-Mesh-Continuity-Request-Key`.
- [ ] Runtime-generated `503` rejection responses for clustered routes still include the same correlation key when the runtime created a continuity record.
- [ ] Existing handler headers survive beside the new correlation header.
- [ ] The low-level clustered-route e2e uses the response header to fetch the single continuity record directly on both nodes.

## Verification

- `cargo test -p mesh-rt m054_s02_ -- --nocapture`
- `cargo test -p meshc --test e2e_m047_s07 -- --nocapture`

## Observability Impact

- Signals added/changed: `X-Mesh-Continuity-Request-Key` on clustered HTTP responses and retained raw `.http` artifacts that expose it.
- How a future agent inspects this: replay `compiler/meshc/tests/e2e_m047_s07.rs`, then compare the saved raw responses with `meshc cluster continuity <node> <request-key> --json` output for the same key.
- Failure state exposed: header missing, handler headers dropped, or direct continuity lookup mismatching the returned key.

## Inputs

- `compiler/mesh-rt/src/http/server.rs` — clustered HTTP identity generation and response assembly seam.
- `compiler/meshc/tests/e2e_m047_s07.rs` — low-level two-node clustered HTTP proof that currently diffs continuity lists.
- `compiler/meshc/src/cluster.rs` — existing direct continuity record lookup contract the new header should target.

## Expected Output

- `compiler/mesh-rt/src/http/server.rs` — runtime-owned correlation header injection and unit tests.
- `compiler/meshc/tests/e2e_m047_s07.rs` — low-level route proof updated to consume the new response header directly.
