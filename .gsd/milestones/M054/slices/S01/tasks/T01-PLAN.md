---
estimated_steps: 4
estimated_files: 4
skills_used:
  - test
  - postgresql-database-engineering
  - debug-like-expert
---

# T01: Prove the staged Postgres starter through one public ingress URL

**Slice:** S01 — One-public-URL starter ingress truth
**Milestone:** M054

## Description

Build the first honest one-public-URL proof on top of the existing serious starter and runtime surfaces. Reuse the M053 staged Postgres starter helpers, put a thin public-ingress harness in front of the two-node bundle, run the real starter smoke against that one URL, then retain the same clustered `GET /todos` request's ingress, owner, replica, and execution truth from `meshc cluster continuity` and diagnostics instead of inventing new starter-owned placement logic.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| thin public-ingress harness / reverse proxy | Stop the rail immediately, retain proxy startup logs and selected target metadata, and do not fall back to direct node requests. | Treat an unready or hung ingress listener as harness drift; stop both starter nodes and archive the last proxy-state snapshot. | Fail closed if the proxy returns malformed status lines, truncated bodies, or ambiguous target-selection metadata. |
| staged two-node Postgres starter + shared database | Stop on the first staged boot, health, or shared-DB failure and preserve the staged bundle, runtime configs, and node logs. | Time-box cluster convergence and health waits, then archive the last status/continuity/diagnostics observations instead of adding sleeps. | Fail closed if public HTTP JSON or per-node operator JSON omits required fields for the selected request. |
| continuity-list diffing for the clustered `GET /todos` request | Abort if the before/after diff cannot isolate one new route request key or if the two nodes disagree about that request. | Treat polling exhaustion as proof-surface drift and retain the final before/after continuity snapshots. | Fail closed if the selected record is missing `ingress_node`, `owner_node`, `replica_node`, `execution_node`, or redaction markers. |

## Load Profile

- **Shared resources**: one public ingress listener, two staged starter HTTP listeners, one shared PostgreSQL database, cluster ports, and retained artifact directories under `.tmp/m054-s01/`.
- **Per-operation cost**: one public CRUD smoke flow, one clustered `GET /todos` selection, and repeated before/after `meshc cluster continuity|diagnostics` polls on both nodes.
- **10x breakpoint**: proxy queueing and continuity polling churn before database volume or Todo payload size.

## Negative Tests

- **Malformed inputs**: invalid `BASE_URL` or proxy target config, non-JSON public responses, truncated proxy output, and continuity records missing required route fields.
- **Error paths**: proxy forward failure, staged node never reaching ready health, `meshc cluster` auth/parse failure, and retained artifact redaction leaks.
- **Boundary conditions**: first clustered `GET /todos` after startup, empty list before create, standby-first ingress through the public URL, and identical route request keys mirrored across both nodes.

## Steps

1. Add `compiler/meshc/tests/support/m054_public_ingress.rs` and register it in `compiler/meshc/tests/support/mod.rs`; implement a small public-ingress harness that fronts the staged primary/standby starter with one local URL, archives forwarded request/response metadata, and never falls back to direct node requests silently.
2. Extend `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs` only where needed so T01 can boot the existing staged two-node starter, call `examples/todo-postgres/scripts/deploy-smoke.sh` against the public URL, and capture before/after continuity snapshots from both nodes without changing the starter's runtime contract.
3. Add `compiler/meshc/tests/e2e_m054_s01.rs` to stage a fresh Postgres starter, apply migrations, boot two nodes plus the public ingress harness, run CRUD through the public URL, then isolate one new clustered `GET /todos` request by diffing continuity lists and verify the retained summary records `ingress_node`, `owner_node`, `replica_node`, and `execution_node` for that same request.
4. Fail closed on malformed public HTTP/operator JSON and secret leakage: archive the last public transcript, proxy log, and continuity snapshots under `.tmp/m054-s01/...`, and keep `DATABASE_URL`/cluster cookie redaction checks in the rail.

## Must-Haves

- [ ] The task proves a staged two-node Postgres starter can serve real CRUD traffic through one public URL instead of direct per-node ports.
- [ ] The retained proof bundle includes the same route request's public HTTP transcript plus runtime-owned `ingress_node`, `owner_node`, `replica_node`, and `execution_node` truth.
- [ ] The harness and rail fail closed on malformed proxy/operator data and never leak `DATABASE_URL` or cluster cookies.

## Verification

- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_m054_s01 -- --nocapture`
- The retained bundle created by that rail contains a redacted continuity summary that ties one public `GET /todos` request to the selected route record.

## Observability Impact

- Signals added/changed: retained public-ingress request summaries, proxy/backend logs, before/after continuity snapshots, and per-node diagnostics/record JSON for the selected route request.
- How a future agent inspects this: replay `compiler/meshc/tests/e2e_m054_s01.rs`, then open the newest `.tmp/m054-s01/...` proof bundle and compare the public HTTP transcript with the continuity summary JSON.
- Failure state exposed: last public response, proxy-forward target, continuity diff miss, malformed operator JSON, and secret-redaction failures.

## Inputs

- `compiler/meshc/tests/support/mod.rs` — current support-module export list for meshc integration tests.
- `compiler/meshc/tests/support/m046_route_free.rs` — shared cluster waiters, JSON parsers, and retained-artifact helpers.
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` — staged starter runtime/bootstrap helpers and isolated Postgres DB setup.
- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs` — existing staged Postgres deploy/failover helper seam that T01 should reuse rather than replace.
- `examples/todo-postgres/scripts/deploy-smoke.sh` — the real public starter smoke script that must keep working against one public base URL.
- `examples/todo-postgres/README.md` — current serious-starter contract wording to keep bounded while the new proof rail lands.

## Expected Output

- `compiler/meshc/tests/e2e_m054_s01.rs` — the authoritative Rust rail for one-public-URL staged starter ingress truth.
- `compiler/meshc/tests/support/m054_public_ingress.rs` — thin public-ingress harness and retained-artifact helpers for the new rail.
- `compiler/meshc/tests/support/mod.rs` — support-module export updated so the new harness is available to the e2e target.
- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs` — staged Postgres helper seam extended just enough for public-ingress replay and retained summary capture.
