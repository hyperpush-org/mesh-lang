---
id: S01
parent: M053
milestone: M053
provides:
  - A generator-owned Postgres starter deploy contract: staged bundle, staged schema artifact, and staged smoke helpers.
  - A green staged deploy e2e rail (`cargo test -p meshc --test e2e_m053_s01 -- --nocapture`) with redacted artifacts proving `/health`, CRUD, startup continuity, route continuity, diagnostics, and cluster status against a running staged starter.
  - One retained wrapper surface (`bash scripts/verify-m053-s01.sh`) that downstream CI/deploy slices can call without re-implementing starter deploy logic.
requires:
  []
affects:
  - S02
  - S03
  - S04
key_files:
  - compiler/mesh-pkg/src/scaffold.rs
  - examples/todo-postgres/main.mpl
  - examples/todo-postgres/README.md
  - examples/todo-postgres/scripts/stage-deploy.sh
  - examples/todo-postgres/scripts/apply-deploy-migrations.sh
  - examples/todo-postgres/scripts/deploy-smoke.sh
  - examples/todo-postgres/deploy/todo-postgres.up.sql
  - compiler/meshc/tests/support/m053_todo_postgres_deploy.rs
  - compiler/meshc/tests/e2e_m053_s01.rs
  - scripts/verify-m053-s01.sh
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-rt/src/http/server.rs
key_decisions:
  - D400: the Postgres todo starter's serious deploy handoff is a starter-owned staged bundle (`deploy/<package>.up.sql` plus stage/apply/smoke scripts), not a Fly-first or packages-site-owned contract.
  - D401: staged deploy rails must prebuild `mesh-rt` before shelling out to the public `meshc build` path so staged binaries cannot silently pick up a stale `target/debug/libmesh_rt.a`.
  - D402: long-lived clustered HTTP starters trigger startup work once when `mesh_http_serve` / `mesh_http_serve_tls` starts listening, while the existing main-wrapper trigger remains for route-free apps behind the `STARTUP_WORK_TRIGGERED` guard.
patterns_established:
  - Starter-generated serious deploy paths should be proven through an external staged bundle (`deploy` artifact + stage/apply/smoke scripts) instead of through source-tree binaries or hosted-environment assumptions.
  - Long-lived clustered HTTP starters need runtime-owned startup triggering at HTTP server start, guarded so the existing route-free main-wrapper trigger stays truthful and duplicate startup submissions cannot occur.
  - Any test harness that shells out to the public `meshc build` path after runtime changes must prebuild `mesh-rt` first or the staged binary can contradict the in-process Rust test binary.
observability_surfaces:
  - .tmp/m053-s01/verify/status.txt
  - .tmp/m053-s01/verify/current-phase.txt
  - .tmp/m053-s01/verify/phase-report.txt
  - .tmp/m053-s01/verify/full-contract.log
  - .tmp/m053-s01/verify/latest-proof-bundle.txt
  - .tmp/m053-s01/proof-bundles/retained-starter-deploy-1775415092594704000/retained-m053-s01-artifacts/todo-postgres-staged-deploy-truth-1775415079977578000/health.json
  - .tmp/m053-s01/proof-bundles/retained-starter-deploy-1775415092594704000/retained-m053-s01-artifacts/todo-postgres-staged-deploy-truth-1775415079977578000/cluster-status.json
  - .tmp/m053-s01/proof-bundles/retained-starter-deploy-1775415092594704000/retained-m053-s01-artifacts/todo-postgres-staged-deploy-truth-1775415079977578000/cluster-continuity-startup-list.json
  - .tmp/m053-s01/proof-bundles/retained-starter-deploy-1775415092594704000/retained-m053-s01-artifacts/todo-postgres-staged-deploy-truth-1775415079977578000/cluster-continuity-startup-record.json
  - .tmp/m053-s01/proof-bundles/retained-starter-deploy-1775415092594704000/retained-m053-s01-artifacts/todo-postgres-staged-deploy-truth-1775415079977578000/cluster-continuity-route-record.json
  - .tmp/m053-s01/proof-bundles/retained-starter-deploy-1775415092594704000/retained-m053-s01-artifacts/todo-postgres-staged-deploy-truth-1775415079977578000/cluster-diagnostics.json
drill_down_paths:
  - .gsd/milestones/M053/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M053/slices/S01/tasks/T02-SUMMARY.md
  - .gsd/milestones/M053/slices/S01/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-05T18:56:02.784Z
blocker_discovered: false
---

# S01: Generated Postgres starter owns staged deploy truth

**Generated Postgres starters now own a real staged deploy bundle and a retained, green staged PostgreSQL replay with runtime-owned cluster inspection.**

## What Happened

S01 turned the generated Postgres todo starter into a truthful starter-owned deploy surface instead of a README-only promise. On the generator side, the scaffold now emits `deploy/<package>.up.sql` plus starter-owned `stage-deploy.sh`, `apply-deploy-migrations.sh`, and `deploy-smoke.sh`, and `examples/todo-postgres/` still matches that public CLI output mechanically. On the proof side, the slice added a dedicated staged deploy harness that builds a fresh starter, stages the bundle outside the repo tree, applies the staged SQL artifact, boots the staged binary against PostgreSQL, exercises `/health` plus real CRUD, and archives redacted `meshc cluster status|continuity|diagnostics` evidence. Closing the slice required fixing two runtime/proof seams that the task summaries had only isolated: clustered startup work for long-lived HTTP starters could not wait for `mesh_main` to return, and the staged shell-out path could quietly link a stale `libmesh_rt.a`. The final slice state resolves both problems and publishes one retained proof bundle plus one slice-owned wrapper (`bash scripts/verify-m053-s01.sh`) that downstream hosted-chain work can consume directly.

## Verification

Plan-level verification passed exactly as written:

- `cargo test -p mesh-pkg m049_s01_postgres_scaffold_ -- --nocapture`
- `cargo test -p meshc --test e2e_m049_s03 -- --nocapture`
- `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`
- `DATABASE_URL=postgres://postgres:postgres@127.0.0.1:61918/postgres cargo test -p meshc --test e2e_m053_s01 -- --nocapture`
- `DATABASE_URL=postgres://postgres:postgres@127.0.0.1:61918/postgres bash scripts/verify-m053-s01.sh`

The final slice-owned wrapper replay is green and published:

- `.tmp/m053-s01/verify/status.txt` = `ok`
- `.tmp/m053-s01/verify/current-phase.txt` = `complete`
- `.tmp/m053-s01/verify/latest-proof-bundle.txt` -> `.tmp/m053-s01/proof-bundles/retained-starter-deploy-1775415092594704000`
- `.tmp/m053-s01/verify/phase-report.txt` shows every phase passed, including staged deploy e2e, retained artifact copy, staged bundle copy, redaction drift, and bundle-shape checks.

### Operational Readiness

- **Health signal:** `GET /health` on the staged binary returns `status=ok`, `db_backend=postgres`, `migration_strategy=meshc migrate`, and `clustered_handler=Work.sync_todos`; cluster status/continuity/diagnostics JSON under the retained proof bundle provide the runtime-owned operator view.
- **Failure signal:** explicit fail-closed artifacts are retained for invalid bundle paths, missing `DATABASE_URL`, malformed `BASE_URL`, and cluster CLI against a non-ready node; the wrapper stops immediately and records the failing phase plus log path.
- **Recovery procedure:** rerun `bash scripts/verify-m053-s01.sh` with a disposable PostgreSQL instance and inspect `.tmp/m053-s01/verify/full-contract.log`, then the retained proof bundle pointer. If staged runtime behavior changed, rebuild `mesh-rt` first and regenerate the staged bundle through the harness instead of reusing old staged binaries.
- **Monitoring gaps:** this slice does not yet cover multi-node failover, hosted deploy-chain evidence, or packages-site integration; those remain S02-S04 follow-on work.

## Requirements Advanced

- R122 — S01 gives the Postgres starter a truthful staged deploy bundle, single-node clustered runtime replay, and operator-surface evidence bundle that S02 can extend into the full multi-node clustered deploy proof without widening SQLite’s explicitly local contract.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

T02 and T03 originally stopped with a partial closeout because the staged starter registered startup work but never submitted it while `main` blocked in `HTTP.serve(...)`, and the staged bundle shell-out path could still link a stale `target/debug/libmesh_rt.a`. S01 closed both seams inside the slice instead of deferring them: the runtime now triggers startup work idempotently when HTTP servers start listening, and the staged deploy harness forces a fresh `cargo build -p mesh-rt` before staging binaries. The slice scope did not change.

## Known Limitations

This slice proves the generated Postgres starter through a single-node staged deploy replay against PostgreSQL, with startup continuity, route continuity, diagnostics, and CRUD truth. It does not yet prove multi-node failover/rejoin, hosted-chain/packages-site integration, or the public docs/Fly contract; those remain S02-S04 work. SQLite remains the explicitly local starter and was not widened by this slice.

## Follow-ups

S02 should reuse `.tmp/m053-s01/verify/latest-proof-bundle.txt` as the bootstrap evidence seam, then extend the starter proof from single-node staged deploy truth to a real two-node/failover replay. S03 should call `bash scripts/verify-m053-s01.sh` directly instead of rebuilding starter deploy logic inside CI wiring. S04 still needs the public docs/Fly/reference assets to match the shipped starter contract.

## Files Created/Modified

- `compiler/mesh-pkg/src/scaffold.rs` — Extended the Postgres starter generator with starter-owned staged deploy assets, the blocking HTTP starter shape, and the serious deploy README contract.
- `examples/todo-postgres/main.mpl` — Kept the checked-in Postgres example in generator parity with the scaffolded starter, including the staged deploy contract and blocking HTTP main shape.
- `examples/todo-postgres/README.md` — Published the staged deploy README/runbook and deploy helpers that the starter now owns.
- `examples/todo-postgres/scripts/stage-deploy.sh` — Stages an external deploy bundle with the built starter binary plus deploy assets, and now stays truthful when runtime behavior changes by relying on the harness-side mesh-rt prebuild.
- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs` — Materializes the staged deploy bundle, applies staged SQL, boots the staged binary, inspects cluster state, and archives redacted evidence.
- `compiler/meshc/tests/e2e_m053_s01.rs` — Added the staged deploy e2e rail plus retained verifier contract assertions, and updated the startup continuity expectation to match declared replication metadata.
- `scripts/verify-m053-s01.sh` — Added the slice-owned retained verifier wrapper that replays scaffold, example parity, staged deploy proof, and retained bundle-shape/redaction checks.
- `compiler/mesh-rt/src/dist/node.rs` — Made startup work idempotent and retained a reset seam for runtime tests.
- `compiler/mesh-rt/src/http/server.rs` — Triggers pending startup work when long-lived HTTP servers start listening so clustered HTTP starters do not depend on `mesh_main` returning.
- `.gsd/PROJECT.md` — Refreshed project state and current-operating notes for the now-complete M053/S01 slice.
- `.gsd/KNOWLEDGE.md` — Replaced obsolete M053/S01 investigation notes with the durable runtime-trigger and staged-staticlib rules.
