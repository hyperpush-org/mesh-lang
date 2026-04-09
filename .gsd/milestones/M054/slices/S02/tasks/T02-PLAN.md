---
estimated_steps: 4
estimated_files: 4
skills_used:
  - debug-like-expert
  - postgresql-database-engineering
  - test
---

# T02: Prove direct request correlation on the serious starter public-ingress rail

**Slice:** S02 — Clustered HTTP request correlation
**Milestone:** M054

## Description

Add the serious-starter rail that consumes the runtime header instead of continuity diffing. Keep S01 intact as the historical diff-based proof, but add a new S02 e2e and small support helpers that extract the selected public response header, jump straight into primary/standby direct continuity and diagnostics lookups, and retain a self-contained direct-correlation bundle.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| public ingress harness + staged two-node Postgres starter | Stop on the first staged boot, health, or ingress-forwarding failure and preserve runtime config, node logs, and the last ingress snapshot. | Time-box cluster convergence and selected-request waits, then archive the last public request transcript and status/diagnostics output instead of adding sleeps. | Fail closed if the selected public response omits the correlation header or if the retained raw HTTP transcript cannot be parsed deterministically. |
| direct continuity + diagnostics lookup on both nodes | Abort if the primary/standby direct lookups disagree about the request key or completed record. | Treat polling exhaustion as a proof-surface regression and retain the final lookup JSON on both nodes. | Fail closed if the record or diagnostics entries for the returned key omit required fields or disappear on one node. |

## Load Profile

- **Shared resources**: public ingress harness, two-node staged starter, disposable PostgreSQL database, and retained bundles under `.tmp/m054-s02/`.
- **Per-operation cost**: one staged deploy/apply flow, one selected public `GET /todos`, and paired direct continuity plus diagnostics polls on both nodes.
- **10x breakpoint**: cluster convergence, diagnostics polling, and database startup time before any response-header parsing cost matters.

## Negative Tests

- **Malformed inputs**: public responses missing `X-Mesh-Continuity-Request-Key`, empty header values, or malformed raw HTTP transcripts.
- **Error paths**: primary/standby direct lookup disagreement, diagnostics missing entries for the returned key, and staged runtime readiness failures.
- **Boundary conditions**: the first public standby-targeted `GET /todos` after startup, empty-list response before create, and one public request mapping to exactly one completed continuity record on both nodes.

## Steps

1. Extend `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` or `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs` with a small fail-closed HTTP response-header helper so the staged/public-ingress rails can read `X-Mesh-Continuity-Request-Key` from saved raw responses without inventing a second HTTP abstraction.
2. Add `compiler/meshc/tests/e2e_m054_s02.rs` as a new rail that stages the serious Postgres starter behind the existing public ingress harness, extracts the selected public response header, and goes directly to `wait_for_continuity_record_completed_pair(...)` plus `wait_for_request_diagnostics_pair(...)`.
3. Retain S02-specific artifacts that show the raw public response, extracted request-key JSON, direct primary/standby continuity record JSON, and request-scoped diagnostics entries, while leaving S01's diff-based retained bundle shape untouched.
4. Fail closed on missing or mismatched headers or lookup drift instead of silently diffing continuity lists again.

## Must-Haves

- [ ] The serious starter gets a dedicated S02 e2e rail instead of mutating S01's retained diff-based proof.
- [ ] The S02 rail extracts one request key from the selected public response and uses it for direct continuity lookup on both nodes.
- [ ] The retained S02 bundle includes raw public HTTP, extracted correlation data, direct record JSON, and request-scoped diagnostics JSON for the same request.
- [ ] Missing or malformed response headers or primary/standby lookup drift fail closed.

## Verification

- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_m054_s02 -- --nocapture`
- The new `.tmp/m054-s02/...` bundle contains `public-selected-list.http`, an extracted request-key artifact, and direct primary/standby continuity-record artifacts for the same key.

## Observability Impact

- Signals added/changed: direct request-key artifact for the selected public request plus paired direct continuity and diagnostics JSON keyed by that value.
- How a future agent inspects this: replay `compiler/meshc/tests/e2e_m054_s02.rs`, then open the newest `.tmp/m054-s02/...` bundle and compare the response-header artifact with the paired primary/standby record JSON.
- Failure state exposed: header extraction failure, primary/standby record disagreement, and request-scoped diagnostics gaps for the selected public request.

## Inputs

- `compiler/meshc/tests/e2e_m054_s01.rs` — current one-public-URL rail that still uses continuity diffing and must stay intact.
- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs` — staged two-node starter helpers and direct-record/diagnostics waiters to reuse.
- `compiler/meshc/tests/support/m054_public_ingress.rs` — public-ingress request transcript capture with raw response bytes.
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` — shared raw HTTP response helper that can expose header parsing without a new abstraction.

## Expected Output

- `compiler/meshc/tests/e2e_m054_s02.rs` — serious-starter direct-correlation rail and retained bundle.
- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs` — staged deploy support extended for direct request-key extraction or paired lookup helpers as needed.
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` — small shared HTTP response-header parsing/helper seam.
