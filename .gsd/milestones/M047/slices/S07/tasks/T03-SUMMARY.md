---
id: T03
parent: S07
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-rt/src/http/router.rs", "compiler/mesh-rt/src/http/server.rs", "compiler/mesh-rt/src/dist/node.rs", "compiler/mesh-rt/src/dist/continuity.rs", "compiler/mesh-rt/src/dist/operator.rs", ".gsd/DECISIONS.md"]
key_decisions: ["D301: Reverse-look up declared-handler registrations by route shim fn pointer at router registration time and execute clustered HTTP routes over a dedicated transient request/response transport instead of widening generic declared-work spawn arg tags."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran `cargo test -p mesh-rt m047_s07 -- --nocapture` after formatting; the focused runtime rail passed with 8 tests covering request/response transport, clustered route rejection, default-count completion truth, and repeated-runtime-name operator inspection."
completed_at: 2026-04-02T00:27:06.856Z
blocker_discovered: false
---

# T03: Added real clustered HTTP route dispatch and truthful continuity/operator evidence for route handlers.

> Added real clustered HTTP route dispatch and truthful continuity/operator evidence for route handlers.

## What Happened
---
id: T03
parent: S07
milestone: M047
key_files:
  - compiler/mesh-rt/src/http/router.rs
  - compiler/mesh-rt/src/http/server.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-rt/src/dist/operator.rs
  - .gsd/DECISIONS.md
key_decisions:
  - D301: Reverse-look up declared-handler registrations by route shim fn pointer at router registration time and execute clustered HTTP routes over a dedicated transient request/response transport instead of widening generic declared-work spawn arg tags.
duration: ""
verification_result: passed
completed_at: 2026-04-02T00:27:06.856Z
blocker_discovered: false
---

# T03: Added real clustered HTTP route dispatch and truthful continuity/operator evidence for route handlers.

**Added real clustered HTTP route dispatch and truthful continuity/operator evidence for route handlers.**

## What Happened

I completed the runtime seam for clustered HTTP routes. Route registration now carries declared-handler runtime metadata, the HTTP server serializes `MeshHttpRequest` / `MeshHttpResponse` and routes clustered handlers through a fail-closed 503 path, and `dist/node.rs` now submits/completes continuity around the real route shim while executing requests through a dedicated transient request/response transport when ownership is remote. I also enriched continuity/operator diagnostics with phase/result, replication count, and declared-handler runtime-name metadata and added focused `m047_s07` tests for transport roundtrip, malformed transport rejection, default-count completion truth, unsupported explicit fanout rejection, and repeated-runtime-name inspection behavior.

## Verification

Ran `cargo test -p mesh-rt m047_s07 -- --nocapture` after formatting; the focused runtime rail passed with 8 tests covering request/response transport, clustered route rejection, default-count completion truth, and repeated-runtime-name operator inspection.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-rt m047_s07 -- --nocapture` | 0 | ✅ pass | 16300ms |


## Deviations

Did not change dist/operator.rs list ordering logic itself because the existing request-key/attempt-id sort already satisfied the repeated-runtime-name inspection requirement; I added the focused proof rail there instead.

## Known Issues

None.

## Files Created/Modified

- `compiler/mesh-rt/src/http/router.rs`
- `compiler/mesh-rt/src/http/server.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-rt/src/dist/operator.rs`
- `.gsd/DECISIONS.md`


## Deviations
Did not change dist/operator.rs list ordering logic itself because the existing request-key/attempt-id sort already satisfied the repeated-runtime-name inspection requirement; I added the focused proof rail there instead.

## Known Issues
None.
