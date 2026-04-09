---
id: S02
parent: M054
milestone: M054
provides:
  - Direct response-header-to-continuity correlation for clustered HTTP requests
  - A serious-starter standby-first public-ingress proof bundle with mirrored continuity and diagnostics evidence for the same request
  - Generator-owned README and verifier wording for the bounded operator-facing lookup flow
requires:
  - slice: S01
    provides: one-public-URL starter ingress truth, the standby-first public-ingress harness, and the retained staged-bundle proof surface that S02 delegates unchanged
affects:
  - S03
key_files:
  - compiler/mesh-rt/src/http/server.rs
  - compiler/meshc/tests/e2e_m047_s07.rs
  - compiler/meshc/tests/e2e_m054_s02.rs
  - compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs
  - compiler/meshc/tests/support/m054_public_ingress.rs
  - compiler/mesh-pkg/src/scaffold.rs
  - examples/todo-postgres/README.md
  - scripts/verify-m054-s02.sh
  - scripts/tests/verify-m054-s02-contract.test.mjs
key_decisions:
  - Use a runtime-owned `X-Mesh-Continuity-Request-Key` response header instead of body fields or continuity-list diffing to hand off clustered HTTP request identity.
  - Keep continuity-list discovery for startup/manual inspection, but require direct request-key lookup for clustered HTTP response correlation.
  - Retain S01 unchanged inside the S02 proof bundle and copy the staged bundle so the assembled verifier stays self-contained without mutating older proof surfaces.
patterns_established:
  - Raw public HTTP transcript -> fail-closed response-header extraction -> direct `meshc cluster continuity` / diagnostics lookup is the canonical clustered HTTP correlation pattern.
  - Injecting the request key on runtime-generated rejection responses keeps observability consistent across success and 503 paths.
  - Assembled proof wrappers should copy delegated verify trees and staged bundles rather than sharing or rewriting older `.tmp/.../verify` directories.
observability_surfaces:
  - `X-Mesh-Continuity-Request-Key` on clustered HTTP responses and runtime-generated clustered 503s
  - `meshc cluster continuity <node> <request-key> --json` direct record lookup
  - request-scoped `meshc cluster diagnostics <node> --json` entries retained for the same request key
  - retained `.tmp/m054-s02/.../public-selected-list.request-key.json` and `selected-route-direct-{primary,standby}-{record,diagnostics}.json` artifacts
drill_down_paths:
  - .gsd/milestones/M054/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M054/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M054/slices/S02/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-06T15:55:45.085Z
blocker_discovered: false
---

# S02: Clustered HTTP request correlation

**Runtime-owned clustered HTTP responses now carry a continuity request key, and the serious Postgres starter can trace one public `GET /todos` directly to mirrored continuity and diagnostics records on both nodes.**

## What Happened

S02 replaced continuity-list diffing with a runtime-owned handoff for clustered HTTP.

T01 added `X-Mesh-Continuity-Request-Key` in `compiler/mesh-rt/src/http/server.rs`, keeping handler headers intact on successful clustered responses and on runtime-generated rejection responses where the runtime still created a continuity record. The lower clustered-route rail in `compiler/meshc/tests/e2e_m047_s07.rs` now reads that header from the raw HTTP response and goes straight to `meshc cluster continuity <node> <request-key> --json` on both nodes.

T02 carried the same seam into the serious starter. `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` now exposes a fail-closed raw-response header helper, `compiler/meshc/tests/support/m054_public_ingress.rs` builds a direct route-correlation summary from paired continuity and diagnostics lookups, and `compiler/meshc/tests/e2e_m054_s02.rs` stages a fresh two-node Postgres starter behind standby-first public ingress, extracts the selected response header, then proves that one public `GET /todos` maps to one completed continuity record and one diagnostics stream on both nodes. The retained `.tmp/m054-s02/...` bundle now includes the raw public response, extracted request-key artifacts, direct primary/standby record JSON, and direct primary/standby diagnostics JSON for the same request.

T03 aligned the starter-owned operator story and the assembled verifier around that same bounded seam. `compiler/mesh-pkg/src/scaffold.rs` and `examples/todo-postgres/README.md` now teach direct response-header correlation for clustered HTTP while explicitly leaving startup/manual discovery on the continuity-list path. `scripts/tests/verify-m054-s02-contract.test.mjs` guards that wording and the retained-bundle contract, and `scripts/verify-m054-s02.sh` delegates S01 unchanged, reruns the new S02 e2e, copies the retained S01 verify tree, and republishes one self-contained direct-correlation proof bundle under `.tmp/m054-s02/proof-bundles/` without mutating the older S01 diff-based bundle.

## Verification

Verified with the slice-plan commands and the assembled closeout rail:

- `node --test scripts/tests/verify-m054-s02-contract.test.mjs` — passed; contract guards the starter README/verifier wording, retained bundle markers, S01 delegation, and redaction expectations.
- `cargo test -p mesh-rt m054_s02_ -- --nocapture` — passed; proved clustered success responses preserve handler headers while adding `X-Mesh-Continuity-Request-Key`, and runtime-generated 503 rejections keep the same correlation seam.
- `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` — passed; authoritative clustered-route rail now uses the response header for direct continuity lookup on both nodes.
- `DATABASE_URL=<local disposable postgres> cargo test -p meshc --test e2e_m054_s02 -- --nocapture` — passed; the new serious-starter rail retained `.tmp/m054-s02/staged-postgres-public-ingress-direct-correlation-1775490659489693000/` with `public-selected-list.request-key.{txt,json}`, `selected-route.summary.json`, `selected-route-direct-primary-record.json`, `selected-route-direct-standby-record.json`, and matching diagnostics artifacts.
- `DATABASE_URL=<local disposable postgres> bash scripts/verify-m054-s02.sh` — passed; `.tmp/m054-s02/verify/status.txt` is `ok`, `current-phase.txt` is `complete`, and `latest-proof-bundle.txt` points at `.tmp/m054-s02/proof-bundles/retained-direct-correlation-proof-1775490670703155000`.

Operational/diagnostic surfaces were also confirmed from retained evidence: the selected public standby-routed `GET /todos` response includes `X-Mesh-Continuity-Request-Key: http-route::Api.Todos.handle_list_todos::1`, `selected-route.summary.json` resolves that request to ingress=`todo-postgres-standby@[::1]:55139`, owner/execution=`todo-postgres-primary@127.0.0.1:55139`, and both `health-primary-health.json` / `health-standby-health.json` report `status: ok`.

## Requirements Advanced

- R123 — Adds the runtime-owned request-correlation follow-through the load-balancing story was missing: clustered HTTP responses now hand operators a direct request key, and both the low-level route rail and the serious starter prove direct continuity/diagnostics lookup on both nodes without continuity-list diffing.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None.

## Known Limitations

- Direct request correlation now exists for clustered HTTP responses, but startup records and general manual investigation still use continuity-list discovery plus diagnostics rather than a response-header seam.
- The response header is an operator/debugging handoff, not a client-routing or sticky-session contract.
- The public copy guardrails for homepage/distributed-proof/serious-starter docs still land in S03, so R123 advances here but is not fully validated yet.

## Follow-ups

- Use S03 to align homepage, distributed-proof docs, and serious-starter guidance with this bounded header-to-continuity lookup flow while keeping Fly, sticky-session, and frontend-aware routing claims out of the public contract.
- Keep future clustered HTTP proof work anchored on the retained S02 artifacts (`public-selected-list.request-key.{txt,json}` and `selected-route-direct-{primary,standby}-{record,diagnostics}.json`) instead of reviving continuity-list diffing for request-scoped debugging.

## Files Created/Modified

- `compiler/mesh-rt/src/http/server.rs` — Injected the runtime-owned clustered HTTP request-correlation header on successful replies and runtime-generated rejection replies while preserving handler headers.
- `compiler/meshc/tests/e2e_m047_s07.rs` — Reworked the authoritative clustered-route rail to extract the response header from raw HTTP and perform direct continuity lookup on both nodes.
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` — Added a fail-closed raw-response header extraction helper for starter and ingress rails.
- `compiler/meshc/tests/support/m054_public_ingress.rs` — Added direct route-correlation summary helpers that compose paired continuity and diagnostics lookups for the same public request.
- `compiler/meshc/tests/e2e_m054_s02.rs` — Added the serious Postgres starter rail that proves standby-first public ingress can trace one clustered `GET /todos` response directly to matching primary/standby continuity and diagnostics records.
- `compiler/mesh-pkg/src/scaffold.rs` — Updated generated Postgres starter README content to teach the direct response-header correlation flow while reserving continuity-list discovery for startup/manual inspection.
- `examples/todo-postgres/README.md` — Re-materialized the committed starter example so its operator guidance matches the generator-owned direct-correlation contract.
- `scripts/tests/verify-m054-s02-contract.test.mjs` — Guarded the direct-correlation docs/verifier wording, retained artifact markers, S01 delegation, and redaction expectations.
- `scripts/verify-m054-s02.sh` — Added the assembled S02 verifier that delegates S01, replays the new starter rail, and republishes one self-contained retained proof bundle.
- `.gsd/PROJECT.md` — Refreshed current-state milestone text so M054 now reflects both the S01 ingress truth and the S02 direct-correlation seam.
- `.gsd/KNOWLEDGE.md` — Recorded the authoritative retained-artifact debug seam for future M054/S03 and later clustered HTTP debugging.
