---
id: T01
parent: S01
milestone: M054
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/support/m054_public_ingress.rs", "compiler/meshc/tests/support/m053_todo_postgres_deploy.rs", "compiler/meshc/tests/support/mod.rs", "compiler/meshc/tests/e2e_m054_s01.rs", ".gsd/milestones/M054/slices/S01/tasks/T01-SUMMARY.md"]
key_decisions: ["Reuse the existing M053 staged Postgres helper seam and add only pair waiters plus a thin support-layer ingress harness.", "Derive the selected request summary from runtime-owned continuity records instead of proxy-side placement guesses."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified that `cargo test -p meshc --test e2e_m054_s01 --no-run` succeeds after the new ingress-harness and e2e changes. Brought up a disposable local Docker Postgres instance for runtime testing. The last full runtime replay before wrap-up timed out after exposing the standby-first public-route failure and the proxy/cleanup issues noted above; the code was patched, but the post-fix full rerun is still outstanding."
completed_at: 2026-04-06T05:38:24.091Z
blocker_discovered: false
---

# T01: Added a first-pass public-ingress harness and M054 staged Postgres proof rail; compile passes, full runtime proof still needs rerun.

> Added a first-pass public-ingress harness and M054 staged Postgres proof rail; compile passes, full runtime proof still needs rerun.

## What Happened
---
id: T01
parent: S01
milestone: M054
key_files:
  - compiler/meshc/tests/support/m054_public_ingress.rs
  - compiler/meshc/tests/support/m053_todo_postgres_deploy.rs
  - compiler/meshc/tests/support/mod.rs
  - compiler/meshc/tests/e2e_m054_s01.rs
  - .gsd/milestones/M054/slices/S01/tasks/T01-SUMMARY.md
key_decisions:
  - Reuse the existing M053 staged Postgres helper seam and add only pair waiters plus a thin support-layer ingress harness.
  - Derive the selected request summary from runtime-owned continuity records instead of proxy-side placement guesses.
duration: ""
verification_result: mixed
completed_at: 2026-04-06T05:38:24.093Z
blocker_discovered: false
---

# T01: Added a first-pass public-ingress harness and M054 staged Postgres proof rail; compile passes, full runtime proof still needs rerun.

**Added a first-pass public-ingress harness and M054 staged Postgres proof rail; compile passes, full runtime proof still needs rerun.**

## What Happened

Added the new public-ingress support module, extended the staged Postgres deploy helpers with pair waiters, and wrote the new `e2e_m054_s01` proof/negative-test target. The compile-only target now passes. During runtime verification I switched to a disposable local Docker Postgres per user direction and got one full failing replay that exposed concrete seams: accepted ingress sockets needed to be forced back to blocking mode, the standby-first public GET /todos needed to wait for startup continuity completion, and the one-shot fake backend needed teardown-safe cleanup. Those fixes are on disk, but the full post-fix runtime rerun still needs a fresh context.

## Verification

Verified that `cargo test -p meshc --test e2e_m054_s01 --no-run` succeeds after the new ingress-harness and e2e changes. Brought up a disposable local Docker Postgres instance for runtime testing. The last full runtime replay before wrap-up timed out after exposing the standby-first public-route failure and the proxy/cleanup issues noted above; the code was patched, but the post-fix full rerun is still outstanding.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m054_s01 --no-run` | 0 | ✅ pass | 14000ms |
| 2 | `docker run -d --rm --name mesh-m054-s01-pg-52027 -e POSTGRES_USER=mesh -e POSTGRES_PASSWORD=mesh -e POSTGRES_DB=postgres -p 127.0.0.1:52027:5432 postgres:16 (with pg_isready wait)` | 0 | ✅ pass | 5000ms |
| 3 | `DATABASE_URL=postgres://mesh:mesh@127.0.0.1:52027/postgres cargo test -p meshc --test e2e_m054_s01 -- --nocapture` | 124 | ❌ fail | 900000ms |


## Deviations

Used a disposable local Docker Postgres instance for verification instead of relying on a preconfigured DATABASE_URL. Added startup-completion stabilization before the standby-first public GET /todos after the first runtime replay exposed a startup-order sensitivity.

## Known Issues

The final post-fix rerun of `cargo test -p meshc --test e2e_m054_s01 -- --nocapture` did not happen before wrap-up, so the runtime proof is not fully revalidated yet. The last full runtime replay on the previous revision failed on the standby-first public route with `clustered_http_route_reply_read_failed:peer closed connection without sending TLS close_notify`; the current edits were made to address that and still need confirmation in fresh context. T02-owned verifier/docs work is still untouched.

## Files Created/Modified

- `compiler/meshc/tests/support/m054_public_ingress.rs`
- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs`
- `compiler/meshc/tests/support/mod.rs`
- `compiler/meshc/tests/e2e_m054_s01.rs`
- `.gsd/milestones/M054/slices/S01/tasks/T01-SUMMARY.md`


## Deviations
Used a disposable local Docker Postgres instance for verification instead of relying on a preconfigured DATABASE_URL. Added startup-completion stabilization before the standby-first public GET /todos after the first runtime replay exposed a startup-order sensitivity.

## Known Issues
The final post-fix rerun of `cargo test -p meshc --test e2e_m054_s01 -- --nocapture` did not happen before wrap-up, so the runtime proof is not fully revalidated yet. The last full runtime replay on the previous revision failed on the standby-first public route with `clustered_http_route_reply_read_failed:peer closed connection without sending TLS close_notify`; the current edits were made to address that and still need confirmation in fresh context. T02-owned verifier/docs work is still untouched.
