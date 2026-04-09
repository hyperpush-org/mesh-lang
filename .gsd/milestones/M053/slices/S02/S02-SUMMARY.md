---
id: S02
parent: M053
milestone: M053
provides:
  - A reusable staged two-node Postgres starter helper that boots primary and standby from one generated bundle and archives dual-node operator and HTTP evidence.
  - An authoritative generated-starter failover rail that proves real CRUD/read continuity, owner-loss promotion/recovery, stale-primary fencing, and fenced rejoin truth.
  - A retained slice verifier (`bash scripts/verify-m053-s02.sh`) that replays S01 first and publishes a failover proof bundle under `.tmp/m053-s02/`.
requires:
  - slice: S01
    provides: starter-owned staged deploy bundle, staged migration/apply/smoke scripts, and the retained deploy-truth verifier consumed by the S02 failover replay
affects:
  - S03
  - S04
key_files:
  - compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs
  - compiler/meshc/tests/support/m053_todo_postgres_deploy.rs
  - compiler/meshc/tests/e2e_m053_s02.rs
  - scripts/verify-m053-s02.sh
  - .gsd/PROJECT.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D403: keep S02 on a host-native two-node staged replay of the generated Postgres starter instead of widening the starter contract or falling back to a proof-app path.
  - D405: accept `replication_health=unavailable` as truthful on the promoted node during owner-loss and fenced-rejoin windows when role/epoch/membership and continuity diagnostics already prove recovery.
  - D404: treat R122 as validated by the assembled S01+S02 generated-starter deploy/failover proof chain.
patterns_established:
  - Keep clustered failover proof starter-owned: stage once, boot two nodes from the same retained bundle, and archive per-node operator plus HTTP artifacts instead of adding app-owned failover glue.
  - Treat `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics` as a combined truth surface; promoted-node health alone is not enough to judge failover correctness.
  - Retain only fresh helper/fail-closed/failover artifact directories and a copied staged bundle inside one pointer-driven proof bundle so downstream slices can consume the exact replay output without rebuilding it.
observability_surfaces:
  - .tmp/m053-s02/verify/phase-report.txt
  - .tmp/m053-s02/verify/status.txt
  - .tmp/m053-s02/verify/current-phase.txt
  - .tmp/m053-s02/verify/full-contract.log
  - .tmp/m053-s02/verify/m053-s02-failover-e2e.test-count.log
  - .tmp/m053-s02/verify/latest-proof-bundle.txt
  - .tmp/m053-s02/proof-bundles/retained-failover-proof-*/retained-m053-s02-artifacts/
drill_down_paths:
  - .gsd/milestones/M053/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M053/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M053/slices/S02/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-05T20:15:41.466Z
blocker_discovered: false
---

# S02: Generated Postgres starter proves clustered failover truth

**The generated Postgres Todo starter now has a truthful two-node staged failover proof, plus an assembled verifier that retains fresh operator and HTTP evidence for downstream hosted-chain work.**

## What Happened

S02 closed the gap between S01’s staged deploy truth and a real clustered failover proof for the generated Postgres Todo starter. The slice extended the reusable staged-helper seam so one generated bundle plus one shared Postgres database can boot paired primary and standby runtimes, archive per-node `/health` plus `meshc cluster status|continuity|diagnostics` evidence, and retain HTTP request/response snapshots without adding failover logic to starter source. The helper contract stayed generated-starter-first: `examples/todo-postgres/README.md` remains bounded to operator commands and route surfaces, while `examples/todo-postgres/work.mpl` still contains only ordinary clustered starter code rather than timer sleeps or env-driven failover glue.

The authoritative e2e rail now generates a fresh Postgres starter, stages the deploy bundle outside the repo root, applies the staged SQL, boots two staged processes against one shared database, seeds state through real starter routes, and proves clustered `GET /todos` continuity under the real `Api.Todos.handle_list_todos` runtime name. It then uses the runtime-owned `MESH_STARTUP_WORK_DELAY_MS` seam to hold the startup record pending long enough to kill the primary, proving `automatic_promotion`, `automatic_recovery`, `recovery_rollover`, completed continuity on the promoted standby, stale-primary fencing, and fenced rejoin through retained operator JSON, per-node logs, and post-failover HTTP reads/writes/deletes.

Closeout also finished the explicit next step left open by task execution: `scripts/verify-m053-s02.sh` is now the slice-owned retained verifier. It replays `bash scripts/verify-m053-s01.sh` first, reruns the full S02 e2e rail, copies only the fresh helper/fail-closed/failover artifact directories plus the staged bundle into `.tmp/m053-s02/proof-bundles/retained-failover-proof-*`, publishes `.tmp/m053-s02/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, and `latest-proof-bundle.txt`, and fails closed on missing bundles, stale pointers, missing retained artifacts, or leaked `DATABASE_URL` text. The resulting proof surface is ready for downstream hosted-chain and docs slices to consume directly instead of reconstructing the failover setup from scratch.

## Verification

Using a disposable local Docker Postgres exported as `DATABASE_URL`, the slice-level verification commands all passed:
- `cargo test -p meshc --test e2e_m053_s02 m053_s02_staged_postgres_helper_ -- --nocapture`
- `cargo test -p meshc --test e2e_m053_s02 -- --nocapture`
- `bash scripts/verify-m053-s02.sh`

The assembled verifier replayed `bash scripts/verify-m053-s01.sh` first and finished with:
- `.tmp/m053-s02/verify/status.txt` = `ok`
- `.tmp/m053-s02/verify/current-phase.txt` = `complete`
- `.tmp/m053-s02/verify/phase-report.txt` showing passed phases for `m053-s02-db-env-preflight`, `m053-s01-contract`, `m053-s02-failover-e2e`, `m053-s02-retain-artifacts`, `m053-s02-retain-staged-bundle`, `m053-s02-redaction-drift`, and `m053-s02-bundle-shape`
- `.tmp/m053-s02/verify/latest-proof-bundle.txt` pointing at a retained bundle that contains the copied helper/fail-closed/failover proof dirs, README/work snapshots, and the staged deploy bundle copy

## Requirements Advanced

- R122 — Completed the missing two-node clustered failover half of the generated-starter proof on top of S01’s staged deploy truth and turned it into one retained verifier surface for downstream hosted-chain work.

## Requirements Validated

- R122 — Using a disposable local Docker Postgres exported as `DATABASE_URL`, `cargo test -p meshc --test e2e_m053_s02 m053_s02_staged_postgres_helper_ -- --nocapture`, `cargo test -p meshc --test e2e_m053_s02 -- --nocapture`, and `bash scripts/verify-m053-s02.sh` all passed. The retained `.tmp/m053-s02/` bundle proves dual-node operator truth, real CRUD/read snapshots, automatic promotion/recovery, stale-primary fencing, and fenced rejoin while keeping SQLite explicitly local-only.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Closeout finished the retained S02 verifier wrapper instead of leaving it as a later follow-up: `scripts/verify-m053-s02.sh` now exists and is the slice-owned assembled replay surface. During replay, the destructive rail also proved that the promoted node can truthfully report `replication_health=unavailable` after owner loss and after fenced rejoin even when role/epoch/membership plus continuity diagnostics are correct, so the S02 acceptance rail was widened to accept that truthful runtime output instead of forcing only `local_only` or `healthy`.

## Known Limitations

The local verifier still requires `DATABASE_URL`; for closeout this was satisfied with a disposable local Docker Postgres rather than a shared external database. The retained S02 bundle is host-native and local-first; S03 still needs to wire `bash scripts/verify-m053-s02.sh` into the normal hosted release/deploy chain, and S04 still needs to align public docs/reference wording with the bounded starter contract. `gsd_requirement_update` also rejected `R122` as not found in the requirements DB during closeout, so D404 plus this slice summary are the current visible truth for requirement status until that DB mismatch is repaired.

## Follow-ups

S03 should call `bash scripts/verify-m053-s02.sh` as the serious starter failover prerequisite and fail the hosted release/deploy chain if `status.txt`, `phase-report.txt`, or `latest-proof-bundle.txt` drift. S04 should keep the starter README/work surfaces bounded and reference the retained proof bundle rather than moving failover prose or startup-delay mechanics into the public starter contract. If the runtime later changes promoted-node replication-health semantics, update both `compiler/meshc/tests/e2e_m053_s02.rs` and `scripts/verify-m053-s02.sh` together so the direct rail and assembled verifier stay truthful.

## Files Created/Modified

- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs` — Extended the reusable Postgres starter runtime/database helper seam that the staged S02 deploy/failover helpers build on.
- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs` — Added paired primary/standby staged runtime configs, dual-node health/status/continuity/diagnostics helpers, staged bundle retention, and per-node HTTP/operator artifact capture for the generated Postgres starter.
- `compiler/meshc/tests/e2e_m053_s02.rs` — Added helper-contract coverage, the authoritative destructive failover rail, bounded README/work assertions, and truthful promoted-node health expectations.
- `scripts/verify-m053-s02.sh` — Added the fail-closed assembled verifier that replays S01 first, reruns the full S02 rail, copies only fresh helper/fail-closed/failover artifacts, snapshots the staged bundle, and enforces redaction plus bundle shape.
- `.gsd/PROJECT.md` — Updated project state to reflect that M053/S02 is now complete and that the retained failover verifier exists.
- `.gsd/KNOWLEDGE.md` — Captured the promoted-node health nuance, local Docker Postgres replay pattern, and current R122 requirement-DB mismatch for future agents.
