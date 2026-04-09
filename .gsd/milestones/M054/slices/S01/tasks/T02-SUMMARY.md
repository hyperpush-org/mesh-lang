---
id: T02
parent: S01
milestone: M054
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-pkg/src/scaffold.rs", "examples/todo-postgres/README.md", "compiler/meshc/tests/e2e_m054_s01.rs", "scripts/verify-m054-s01.sh", "scripts/tests/verify-m054-s01-contract.test.mjs"]
key_decisions: ["Re-materialize `examples/todo-postgres` from the scaffold template instead of hand-editing the committed README so the public starter stays generator-truthful.", "Make `scripts/verify-m054-s01.sh` republish the fresh M054 ingress artifacts plus the staged bundle under `.tmp/m054-s01/proof-bundles/` instead of depending on ad-hoc local temp paths."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified that `bash -n scripts/verify-m054-s01.sh`, `node --test scripts/tests/verify-m054-s01-contract.test.mjs`, `cargo test -p mesh-pkg m049_s01_postgres_scaffold_ -- --nocapture`, and `cargo test -p meshc --test e2e_m047_s04 m047_s04_example_readmes_define_the_public_postgres_vs_sqlite_split -- --nocapture` all pass. Rebuilt `meshc` and re-materialized `examples/` so the committed Postgres README matches the scaffold template. The red seam remains `DATABASE_URL=<redacted> cargo test -p meshc --test e2e_m054_s01 -- --nocapture`, which leaves the same standby-first ingress failure bundle and makes `DATABASE_URL=<redacted> bash scripts/verify-m054-s01.sh` fail in phase `m054-s01-public-ingress-e2e`."
completed_at: 2026-04-06T05:58:25.215Z
blocker_discovered: false
---

# T02: Aligned the Postgres starter README and added the M054 contract/verifier rails, but the standby-first public-ingress e2e still fails on the clustered route transport.

> Aligned the Postgres starter README and added the M054 contract/verifier rails, but the standby-first public-ingress e2e still fails on the clustered route transport.

## What Happened
---
id: T02
parent: S01
milestone: M054
key_files:
  - compiler/mesh-pkg/src/scaffold.rs
  - examples/todo-postgres/README.md
  - compiler/meshc/tests/e2e_m054_s01.rs
  - scripts/verify-m054-s01.sh
  - scripts/tests/verify-m054-s01-contract.test.mjs
key_decisions:
  - Re-materialize `examples/todo-postgres` from the scaffold template instead of hand-editing the committed README so the public starter stays generator-truthful.
  - Make `scripts/verify-m054-s01.sh` republish the fresh M054 ingress artifacts plus the staged bundle under `.tmp/m054-s01/proof-bundles/` instead of depending on ad-hoc local temp paths.
duration: ""
verification_result: mixed
completed_at: 2026-04-06T05:58:25.220Z
blocker_discovered: false
---

# T02: Aligned the Postgres starter README and added the M054 contract/verifier rails, but the standby-first public-ingress e2e still fails on the clustered route transport.

**Aligned the Postgres starter README and added the M054 contract/verifier rails, but the standby-first public-ingress e2e still fails on the clustered route transport.**

## What Happened

Updated `compiler/mesh-pkg/src/scaffold.rs` so the generated Postgres starter README now states the bounded one-public-URL contract: one public app URL may front multiple nodes, `meshc cluster` remains the inspection path for ingress/owner/replica/execution truth, SQLite stays the local-only branch, and the starter does not promise frontend-aware node selection or a Fly-specific product contract. Rebuilt `meshc`, re-materialized `examples/todo-postgres/README.md`, added `scripts/tests/verify-m054-s01-contract.test.mjs`, and added `scripts/verify-m054-s01.sh` to replay the scaffold/unit/materializer/starter-boundary/runtime rails and republish retained M054 artifacts plus the staged bundle under `.tmp/m054-s01/proof-bundles/` on success. During runtime replay I fixed the truncated-backend negative assertion and started narrowing the pre-route public-ingress harness by removing the standby `cluster status` probe, but the main `e2e_m054_s01` rail still fails on the first standby-routed `GET /todos` with `clustered_http_route_reply_read_failed:peer closed connection without sending TLS close_notify`, and the assembled wrapper still stops on that phase.

## Verification

Verified that `bash -n scripts/verify-m054-s01.sh`, `node --test scripts/tests/verify-m054-s01-contract.test.mjs`, `cargo test -p mesh-pkg m049_s01_postgres_scaffold_ -- --nocapture`, and `cargo test -p meshc --test e2e_m047_s04 m047_s04_example_readmes_define_the_public_postgres_vs_sqlite_split -- --nocapture` all pass. Rebuilt `meshc` and re-materialized `examples/` so the committed Postgres README matches the scaffold template. The red seam remains `DATABASE_URL=<redacted> cargo test -p meshc --test e2e_m054_s01 -- --nocapture`, which leaves the same standby-first ingress failure bundle and makes `DATABASE_URL=<redacted> bash scripts/verify-m054-s01.sh` fail in phase `m054-s01-public-ingress-e2e`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -n scripts/verify-m054-s01.sh` | 0 | ✅ pass | 100ms |
| 2 | `node --test scripts/tests/verify-m054-s01-contract.test.mjs` | 0 | ✅ pass | 682ms |
| 3 | `cargo test -p mesh-pkg m049_s01_postgres_scaffold_ -- --nocapture` | 0 | ✅ pass | 26000ms |
| 4 | `cargo test -p meshc --test e2e_m047_s04 m047_s04_example_readmes_define_the_public_postgres_vs_sqlite_split -- --nocapture` | 0 | ✅ pass | 11500ms |
| 5 | `DATABASE_URL=<redacted> cargo test -p meshc --test e2e_m054_s01 -- --nocapture` | 101 | ❌ fail | 14000ms |
| 6 | `DATABASE_URL=<redacted> bash scripts/verify-m054-s01.sh` | 1 | ❌ fail | 34700ms |


## Deviations

Used a disposable local Docker Postgres instance because the repo did not provide a ready DATABASE_URL. Added `cargo build -q -p meshc` as a verifier preflight so the materializer parity check runs against the current scaffold binary instead of a stale `target/debug/meshc`.

## Known Issues

`cargo test -p meshc --test e2e_m054_s01 -- --nocapture` is still red on the first standby-first public `GET /todos`, and the primary node still records no matching route execution when the standby reports `clustered_http_route_reply_read_failed:peer closed connection without sending TLS close_notify`. `scripts/verify-m054-s01.sh` therefore still fails at `m054-s01-public-ingress-e2e`. The disposable local Postgres container used during this context was stopped during wrap-up, so the next context must recreate a disposable admin URL before replaying the failing rail.

## Files Created/Modified

- `compiler/mesh-pkg/src/scaffold.rs`
- `examples/todo-postgres/README.md`
- `compiler/meshc/tests/e2e_m054_s01.rs`
- `scripts/verify-m054-s01.sh`
- `scripts/tests/verify-m054-s01-contract.test.mjs`


## Deviations
Used a disposable local Docker Postgres instance because the repo did not provide a ready DATABASE_URL. Added `cargo build -q -p meshc` as a verifier preflight so the materializer parity check runs against the current scaffold binary instead of a stale `target/debug/meshc`.

## Known Issues
`cargo test -p meshc --test e2e_m054_s01 -- --nocapture` is still red on the first standby-first public `GET /todos`, and the primary node still records no matching route execution when the standby reports `clustered_http_route_reply_read_failed:peer closed connection without sending TLS close_notify`. `scripts/verify-m054-s01.sh` therefore still fails at `m054-s01-public-ingress-e2e`. The disposable local Postgres container used during this context was stopped during wrap-up, so the next context must recreate a disposable admin URL before replaying the failing rail.
