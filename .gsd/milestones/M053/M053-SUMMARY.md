---
id: M053
title: "Deploy Truth for Scaffolds & Packages Surface"
status: complete
completed_at: 2026-04-06T04:07:21.971Z
key_decisions:
  - D400 — keep the Postgres starter’s serious handoff as a starter-owned staged deploy bundle instead of a Fly-first or hosted-only contract
  - D401 — force a fresh mesh-rt build before staged bundle creation whenever the proof path shells out through public meshc build
  - D402 — trigger pending startup work once when clustered HTTP servers start listening, guarded so route-free apps keep their existing proof seam
  - D403 — prove clustered failover on the generated Postgres starter itself through a host-native two-node staged replay
  - D405 — accept promoted-node replication_health=unavailable when stronger role/epoch/membership/continuity signals already prove truthful recovery
  - D406 — bind hosted starter failover proof and packages deploy/public-surface proof into one fail-closed evidence chain
  - D407 — host the serious starter failover proof in a dedicated reusable GitHub Actions workflow with runner-local Postgres
  - D408 — derive the hosted verification repo slug from origin by default, with an explicit override for unusual remotes
  - D416 — treat R121 as validated only once fresh main/tag workflow evidence converges on one shipped SHA
key_files:
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-rt/src/http/server.rs
  - compiler/meshc/tests/support/m053_todo_postgres_deploy.rs
  - compiler/meshc/tests/e2e_m053_s01.rs
  - compiler/meshc/tests/e2e_m053_s02.rs
  - .github/workflows/authoritative-starter-failover-proof.yml
  - .github/workflows/authoritative-verification.yml
  - .github/workflows/release.yml
  - scripts/verify-m053-s01.sh
  - scripts/verify-m053-s02.sh
  - scripts/verify-m053-s03.sh
  - scripts/verify-m053-s04.sh
  - scripts/tests/verify-m053-s03-contract.test.mjs
  - examples/todo-postgres/README.md
  - website/docs/docs/distributed-proof/index.md
lessons_learned:
  - Long-lived clustered HTTP starters need a runtime-owned startup trigger at HTTP server start; waiting for mesh_main to return is not a truthful contract.
  - Hosted failover diagnostics need explicit pending-window evidence; wrapper wording alone is not enough to distinguish a runtime regression from a timeout-shaped failure.
  - Synthetic hosted rollout commits must include every newly tracked fixture directory referenced by tests, or clean runners will fail even when the local tree is green.
  - The visible REQUIREMENTS contract can lag the GSD requirements DB; milestone closeout must treat the checked-in requirement file plus retained proof bundles as the source of truth until the DB is repaired.
---

# M053: Deploy Truth for Scaffolds & Packages Surface

**M053 made the generated Postgres Todo starter the truthful deployable clustered path, carried that proof through hosted release/deploy evidence, and kept SQLite local-only plus Fly reference-only in the public contract.**

## What Happened

M053 started by making the generated Postgres Todo starter own a real staged deploy handoff instead of a README promise. S01 moved the serious deploy contract into generator-owned assets (`deploy/<package>.up.sql`, `scripts/stage-deploy.sh`, `scripts/apply-deploy-migrations.sh`, and `scripts/deploy-smoke.sh`), added runtime-owned startup triggering for long-lived clustered HTTP servers, and retained staged PostgreSQL/operator evidence under `.tmp/m053-s01/verify/`.

S02 reused that same generated bundle to prove the serious clustered path locally: two staged starter nodes ran against PostgreSQL, real CRUD and clustered reads stayed truthful through live HTTP routes, `meshc cluster status|continuity|diagnostics` evidence was retained, and the destructive rail proved automatic promotion/recovery, stale-primary fencing, and fenced rejoin without widening the starter source surface. That closed R122 at the product level.

S03 and S04 then turned the local proof into a public/hosted contract. The serious failover proof now lives in its own reusable GitHub Actions workflow with runner-local Postgres and is required by both `authoritative-verification.yml` and `release.yml`; the hosted verifier binds that starter proof to `deploy-services.yml` packages/public-surface evidence and fails closed on stale refs or missing caller runs. In parallel, README, Getting Started, Clustered Example, Tooling, Distributed Proof, and retained Fly reference surfaces were rewritten to describe the shipped boundaries honestly: SQLite stays explicitly local-only, the generated Postgres starter is the serious deployable/shared path, and Fly remains a retained reference proof environment rather than the public contract.

S05 and S06 closed the last hosted gap. S05 improved diagnostic fidelity so hosted failures stopped looking like wrapper noise. S06 then fixed the real blocker: on hosted Ubuntu, `compiler/mesh-rt/src/dist/node.rs::startup_dispatch_window_ms(...)` ignored `MESH_STARTUP_WORK_DELAY_MS` and collapsed the startup failover pending window back to the default, which let startup work complete before the forced owner stop. The repair, the tracked hosted-red fixture, and the corrected synthetic rollout ship-set landed on shipped SHA `e5fb36a6fe7e9e56f3a608a608abbaaab6764167`; `authoritative-verification.yml`, `deploy-services.yml`, and `release.yml` all went green on that SHA; and `v0.1.0` was rerolled as an annotated tag so the hosted freshness check now resolves `refs/tags/v0.1.0^{}` truthfully.

## Decision Re-evaluation

| Decision | Outcome in delivered system | Revisit next milestone? |
| --- | --- | --- |
| D400 — starter-owned staged deploy bundle | Held. The staged bundle became the stable seam consumed by local proof, hosted proof, and docs. | No |
| D401 — prebuild `mesh-rt` before staged bundle creation | Held. Without it, staged binaries can contradict the in-process test binary. | No |
| D402 — trigger startup work from HTTP server start with idempotent guard | Held. This is the correct runtime seam for long-lived clustered HTTP starters. | No |
| D403 — prove failover on the generated starter, not a proof app | Held. The retained S02 rail stayed on the staged generated starter all the way through failover/rejoin. | No |
| D405 — accept promoted-node `replication_health=unavailable` when stronger failover evidence is present | Held. The runtime can be truthful with that health value during owner-loss and fenced-rejoin windows. | Revisit only if runtime health semantics are intentionally normalized later. |
| D406 — keep hosted starter proof and packages drift in one assembled evidence chain | Held. The hosted verifier now closes the starter/packages contract end to end. | No |
| D407 — give the starter failover proof its own reusable workflow with local Postgres | Held. That kept the hosted proof explicit, reusable, and secret-free. | No |
| D408 — derive the hosted repo slug from `origin` by default | Held. It avoided stale-owner ambiguity after the repo move. | No |
| D416 — treat R121 as validated only when fresh main/tag workflow evidence converges on one shipped SHA | Held. The final hosted closeout met that exact bar. | No |

## Success Criteria Results

- **S01 staged deploy truth remains green — MET.** Evidence: the S01 summary records a green staged deploy replay, `.tmp/m053-s01/verify/status.txt = ok`, `current-phase.txt = complete`, and retained staged PostgreSQL/operator artifacts proving the generated Postgres starter can stage, run, and surface `meshc cluster` truth from deploy artifacts.
- **S02 local failover truth remains green and fail-closed on the runtime-owned startup window — MET.** Evidence: the S02 summary records green helper + full-target + wrapper replays; the validation checklist confirms the retained bundle now proves `startup_dispatch_window.pending_window_ms` before owner stop; and the previous verification run included `cargo test -p mesh-rt startup_work_dispatch_window_ -- --nocapture`, the full `e2e_m053_s02` target, and `bash scripts/verify-m053-s02.sh`.
- **S03 workflow wiring and hosted verifier surfacing are truthful — MET.** Evidence: S03 delivered `.github/workflows/authoritative-starter-failover-proof.yml`, updated `authoritative-verification.yml` / `release.yml`, and a green `scripts/verify-m053-s03.sh`; the validation artifact records fresh exact main/tag workflow evidence rather than wrapper-only signals.
- **S04 docs and retained Fly reference surfaces stay aligned with the shipped contract — MET.** Evidence: the validation checklist marks S04 green, and `.tmp/m053-s04/verify/status.txt = ok` remains the retained docs/reference proof that SQLite stays local-only, Postgres stays the serious deployable starter, and Fly remains reference-only.
- **S05 and S06 close the hosted starter/packages contract end to end — MET.** Evidence: the validation checklist records `.tmp/m053-s03/verify/status.txt = ok`, `current-phase.txt = complete`, and fresh successful `authoritative-verification.yml`, `deploy-services.yml`, and `release.yml` runs on shipped SHA `e5fb36a6fe7e9e56f3a608a608abbaaab6764167`, with the annotated `v0.1.0` reroll making `refs/tags/v0.1.0^{}` resolve truthfully.

## Definition of Done Results

- [x] **All planned slices are complete.** The roadmap marks S01-S06 complete, and the validation delivery audit records every slice as **Pass**.
- [x] **All slice summaries exist.** Files exist for every slice summary and UAT pair under `.gsd/milestones/M053/slices/S01` through `S06`; the completion check found all twelve files on disk.
- [x] **The milestone produced real code and contract changes, not only planning artifacts.** `git diff --stat HEAD $(git merge-base HEAD origin/main) -- ':!.gsd/'` returned a non-empty diff across runtime, scaffold, workflow, docs, verifier, and test files.
- [x] **Cross-slice integration is closed.** The validation artifact records S01 -> S02, S02 -> S03/S05/S06, S03 -> S05, and S05 -> S06 as closed with no remaining delivery-path mismatch.
- [x] **Operational / integration / UAT classes are met.** `M053-VALIDATION.md` now contains explicit `Contract — MET`, `Integration — MET`, `Operational — MET`, and `UAT — MET` markers that match the roadmap’s verification contract.
- [x] **Horizontal checklist review is complete.** No horizontal checklist heading/items are present in `M053-ROADMAP.md`, so there were no extra checklist items to verify beyond the milestone contract itself.

## Requirement Outcomes

- **R121: active -> validated.** Evidence: D416 records the validation decision; `.tmp/m053-s03/verify/status.txt = ok`; `current-phase.txt = complete`; and `remote-runs.json` shows fresh successful `authoritative-verification.yml`, `deploy-services.yml`, and `release.yml` runs aligned on shipped SHA `e5fb36a6fe7e9e56f3a608a608abbaaab6764167`. This proves the packages site now lives inside the normal hosted CI/deploy contract rather than outside it.
- **R122: active -> validated.** Evidence: D404 records the validation decision; the S02 summary records green local helper/full-target/wrapper replays; `bash scripts/verify-m053-s02.sh` is green; and the retained S02 bundle proves the staged generated Postgres starter survives dual-node failover/rejoin honestly while SQLite remains explicitly local-only. S03-S06 extend that same starter proof into hosted release/deploy evidence on one shipped SHA.
- **No requirement was deferred, blocked, or moved out of scope during milestone closeout.**
- **Bookkeeping caveat:** the checked-in `.gsd/REQUIREMENTS.md` still showed R121/R122 as `active` before milestone closeout because the GSD requirements DB lags the visible M053 requirement family. The status transitions above are still supported by milestone evidence and should be projected into the visible requirement contract during closeout.

## Deviations

The milestone needed two remediation slices beyond the initial local-proof/docs wave: S05 improved hosted diagnostic fidelity, and S06 fixed the real hosted standby-promotion blocker plus the annotated release-tag freshness gap. S02 also widened the proof expectation to accept truthful promoted-node `replication_health=unavailable` during owner-loss and fenced-rejoin windows instead of forcing only `local_only` or `healthy`.

## Follow-ups

Repair the GSD requirements DB so R121/R122 can be projected as validated without manual visible-file sync. Carry the startup-window diagnostic seam forward if M054 changes clustered failover timing or load-balancing behavior. Keep future starter/docs work bounded to the shipped SQLite-local / Postgres-deployable / Fly-reference-only contract.
