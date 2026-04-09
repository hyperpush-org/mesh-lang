# T02: 114-compile-run-and-end-to-end-verification 02

**Slice:** S10 — **Milestone:** M021

## Description

Perform smoke-test verification of all HTTP API endpoint domains and the WebSocket endpoint against a running Mesher instance, confirming the ORM rewrite produces correct responses and the EventProcessor service call SIGSEGV is not present.

Purpose: After compilation and startup are verified (Plan 01), this plan exercises all the rewritten query paths end-to-end via real HTTP requests.
Output: Documented HTTP response codes and payloads for each domain, WebSocket upgrade confirmation, SIGSEGV status resolved.

## Must-Haves

- [ ] "At least one HTTP endpoint in each domain (org/project/issue CRUD, search, dashboard, alerts, retention/settings) returns a 2xx JSON response with expected data"
- [ ] "The WebSocket endpoint at :8081 accepts a connection and completes the HTTP upgrade (101 Switching Protocols)"
- [ ] "The EventProcessor service call path (POST /api/v1/events with valid API key) does not crash with SIGSEGV on the first request"
- [ ] "All HTTP endpoints that previously used raw SQL now route through ORM query paths without runtime panics or unexpected 500 errors"
