# S10: Compile Run And End To End Verification

**Goal:** Verify zero-error compilation of Mesher with the fully rewritten ORM query layer, confirm successful startup with PostgreSQL, run migrations, and confirm the MirType::Tuple SIGSEGV fix is active.
**Demo:** Verify zero-error compilation of Mesher with the fully rewritten ORM query layer, confirm successful startup with PostgreSQL, run migrations, and confirm the MirType::Tuple SIGSEGV fix is active.

## Must-Haves


## Tasks

- [x] **T01: 114-compile-run-and-end-to-end-verification 01** `est:30min`
  - Verify zero-error compilation of Mesher with the fully rewritten ORM query layer, confirm successful startup with PostgreSQL, run migrations, and confirm the MirType::Tuple SIGSEGV fix is active.

Purpose: Phase 113 completed the ORM rewrite. Phase 114 must prove the full build pipeline works end-to-end before HTTP/WS endpoint testing begins.
Output: A freshly compiled `mesher/mesher` binary, confirmed startup log, and documented SIGSEGV resolution status.
- [x] **T02: 114-compile-run-and-end-to-end-verification 02** `est:15min`
  - Perform smoke-test verification of all HTTP API endpoint domains and the WebSocket endpoint against a running Mesher instance, confirming the ORM rewrite produces correct responses and the EventProcessor service call SIGSEGV is not present.

Purpose: After compilation and startup are verified (Plan 01), this plan exercises all the rewritten query paths end-to-end via real HTTP requests.
Output: Documented HTTP response codes and payloads for each domain, WebSocket upgrade confirmation, SIGSEGV status resolved.

## Files Likely Touched

- `mesher/mesher`
- `SERVICE_CALL_SEGFAULT.md`
