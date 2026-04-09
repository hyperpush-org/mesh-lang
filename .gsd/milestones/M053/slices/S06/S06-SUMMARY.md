---
id: S06
parent: M053
milestone: M053
provides:
  - A fresh hosted closeout proving starter failover, packages deploy/public-surface proof, and release freshness all agree on shipped SHA `e5fb36a6fe7e9e56f3a608a608abbaaab6764167`.
  - A runtime-owned startup-window regression seam with unit coverage, fail-closed S02 assertions, and tracked hosted-red fixtures for future failover debugging.
requires:
  - slice: S02
    provides: The generated Postgres starter failover rail, staged deploy bundle contract, and retained `.tmp/m053-s02/` proof shape that S06 tightened instead of replacing.
  - slice: S05
    provides: Hosted starter/packages workflow wiring, remote-mutation planning, and the truthful red-state evidence that isolated the last blocker to the runtime-owned startup window.
affects:
  - M053 milestone validation/closeout
  - M054/S01 load-balancing truth work
key_files:
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/meshc/tests/e2e_m053_s02.rs
  - scripts/verify-m053-s02.sh
  - scripts/fixtures/m053-s02-hosted-failure-bundle/SOURCE.txt
  - .tmp/m053-s06/rollout/final-hosted-closeout.md
key_decisions:
  - Keep `MESH_STARTUP_WORK_DELAY_MS` as a runtime-owned positive-only override inside `startup_dispatch_window_ms(...)`, with a safe 2500ms fallback and zero-delay behavior preserved outside startup/replica-required requests.
  - Treat hosted-red regression data used by full-target CI as tracked `scripts/fixtures/...` input, not transient `.tmp/...` state.
  - Close the hosted starter/packages proof only when fresh `authoritative-verification.yml`, `deploy-services.yml`, and `release.yml` runs all agree on one shipped SHA and the `v0.1.0` release tag resolves through an annotated peeled ref.
patterns_established:
  - Keep timing control seams runtime-owned, then prove them twice: focused unit coverage near the runtime helper and fail-closed e2e assertions against the retained diagnostics payload.
  - When full-target hosted tests depend on prior red-state evidence, move that evidence into a tracked fixture under `scripts/fixtures/...` so clean runners can reproduce the contract honestly.
  - Close hosted rollout slices in stages: local green rail, fresh main workflows on the shipped SHA, annotated tag reroll, then final hosted verifier replay.
observability_surfaces:
  - `meshc cluster diagnostics` startup records now expose `startup_dispatch_window` metadata, including the effective `pending_window_ms` used by the staged failover rail.
  - `.tmp/m053-s02/verify/phase-report.txt` and the retained failover bundle capture pre-kill diagnostics, post-kill status/continuity, and rejoin artifacts for the generated starter.
  - `.tmp/m053-s03/verify/status.txt`, `current-phase.txt`, and `remote-runs.json` show whether main/tag hosted freshness is closed on one shipped SHA.
  - `.tmp/m053-s06/rollout/final-hosted-closeout.md` and `release-workflow.json` summarize the final main/tag alignment and annotated-tag release outcome.
drill_down_paths:
  - .gsd/milestones/M053/slices/S06/tasks/T01-SUMMARY.md
  - .gsd/milestones/M053/slices/S06/tasks/T02-SUMMARY.md
  - .gsd/milestones/M053/slices/S06/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-06T03:48:21.742Z
blocker_discovered: false
---

# S06: Hosted failover promotion truth and annotated tag reroll

**Closed the hosted starter/packages contract by restoring the runtime-owned startup pending window, making the staged failover rail fail closed on it, and aligning fresh main plus annotated-tag release evidence on one shipped SHA.**

## What Happened

S06 closed the last honest blocker in the M053 hosted chain rather than routing around it. The retained red bundle already showed that the staged Postgres starter mirrored pending startup state, so the failure was not missing mirror transport; the real seam was `compiler/mesh-rt/src/dist/node.rs::startup_dispatch_window_ms(...)` falling back to the 2500ms default on hosted Ubuntu because it ignored the runtime-owned `MESH_STARTUP_WORK_DELAY_MS` override that the staged starter already exported.

Task 1 restored that override in `mesh-rt` with positive-only parsing and focused unit coverage for the default path, invalid-value fallback, and the zero-delay behavior that should still hold for non-startup or replica-free requests. Task 2 tightened `compiler/meshc/tests/e2e_m053_s02.rs` and the assembled `scripts/verify-m053-s02.sh` rail so the staged two-node Postgres proof now fails closed unless pre-kill diagnostics show the configured `startup_dispatch_window.pending_window_ms` and no `startup_completed` before the forced owner stop. That kept the fix where it belongs: in the runtime and its diagnostics, not in starter-owned sleeps or wrapper-only timing hacks.

Task 3 carried that repair through the hosted evidence chain. The slice moved the hosted-red comparison data out of `.tmp/m053-s05/...` and into the tracked fixture `scripts/fixtures/m053-s02-hosted-failure-bundle/`, fixed the synthetic rollout helper so it ships tracked fixture files alongside the code and verifier changes that reference them, advanced remote `main` to `e5fb36a6fe7e9e56f3a608a608abbaaab6764167`, rerolled `v0.1.0` as an annotated tag object, and then waited for fresh `authoritative-verification.yml`, `deploy-services.yml`, and `release.yml` runs to finish green on that same SHA before re-running the hosted verifier.

The slice leaves two durable truth surfaces for downstream work: starter-owned local failover evidence under `.tmp/m053-s02/verify/` and hosted release/deploy freshness evidence under `.tmp/m053-s03/verify/` plus `.tmp/m053-s06/rollout/`. Taken together, they close the starter/packages contract on one shipped SHA instead of treating local green rails and hosted green rails as separate stories.

## Verification

Passed:
- `cargo test -p mesh-rt startup_work_dispatch_window_ -- --nocapture`
- `DATABASE_URL=postgres://postgres:postgres@127.0.0.1:55432/postgres cargo test -p meshc --test e2e_m053_s02 m053_s02_staged_postgres_failover_proves_clustered_http_and_runtime_recovery -- --nocapture`
- `DATABASE_URL=postgres://postgres:postgres@127.0.0.1:55432/postgres bash scripts/verify-m053-s02.sh`
- `bash scripts/verify-m034-s02-workflows.sh`
- `node --test scripts/tests/verify-m053-s03-contract.test.mjs`
- `GH_TOKEN=<redacted> bash scripts/verify-m053-s03.sh`

The hosted verifier now reports `.tmp/m053-s03/verify/status.txt = ok` and `.tmp/m053-s03/verify/current-phase.txt = complete`. `.tmp/m053-s03/verify/remote-runs.json` shows fresh successful `authoritative-verification.yml` (24017044531), `deploy-services.yml` (24017044515), and `release.yml` (24017289518) runs aligned on shipped SHA `e5fb36a6fe7e9e56f3a608a608abbaaab6764167`, with the release check resolving the annotated `v0.1.0` peeled target.

## Requirements Advanced

- R122 — Carried the generated Postgres starter failover proof from the local staged replay into fresh hosted main/tag evidence on the shipped SHA, so the serious clustered starter story is now proven end to end instead of only locally.

## Requirements Validated

- R121 — `bash scripts/verify-m034-s02-workflows.sh`, `node --test scripts/tests/verify-m053-s03-contract.test.mjs`, and `GH_TOKEN=<redacted> bash scripts/verify-m053-s03.sh` all passed; `.tmp/m053-s03/verify/status.txt` is `ok`; and `remote-runs.json` shows fresh successful `authoritative-verification.yml` (24017044531), `deploy-services.yml` (24017044515), and `release.yml` (24017289518) runs aligned on shipped SHA `e5fb36a6fe7e9e56f3a608a608abbaaab6764167` with the annotated `v0.1.0` release freshness check resolving.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Operational Readiness

- **Health signal**: The local failover contract is healthy when pre-kill `meshc cluster diagnostics` show the configured `startup_dispatch_window.pending_window_ms`, `.tmp/m053-s02/verify/status.txt` is `ok`, and `.tmp/m053-s03/verify/status.txt` is `ok` with `remote-runs.json` aligning fresh successful main/tag workflows on `e5fb36a6fe7e9e56f3a608a608abbaaab6764167`.
- **Failure signal**: `automatic_promotion_rejected:no_mirrored_state`, `startup_completed` landing before the forced owner stop, `pending_window_ms` falling back to 2500 in pre-kill diagnostics, or main/tag workflow SHA drift in `.tmp/m053-s03/verify/remote-runs.json`.
- **Recovery**: Fix the runtime-owned dispatch-window seam in `compiler/mesh-rt/src/dist/node.rs`, rerun the local S02 proof rails against a disposable Docker Postgres, then rerun `bash scripts/verify-m053-s03.sh` after hosted workflows settle.
- **Monitoring gaps**: There is no separate continuous alert channel beyond GitHub Actions plus verifier replays, and the requirements DB can lag the visible validated state for the M053 requirement family.

## Deviations

None.

## Known Limitations

- The GSD requirements DB still does not project the M053 requirement statuses cleanly, so `REQUIREMENTS.md` can continue to show R121/R122 as active until that DB repair lands even though D404 and D416 record the validation evidence.
- Hosted regressions are detected through replayable verifier/workflow evidence rather than a separate continuous alert channel; between replays, drift still relies on GitHub Actions failures and manual inspection of the retained bundles.

## Follow-ups

- Run M053 milestone validation and completion now that all six slices are done.
- Repair the GSD requirements DB mapping for the M053 requirement family so the rendered requirements projection can reflect D404 and D416 automatically.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/node.rs` — Restored positive-only runtime parsing of `MESH_STARTUP_WORK_DELAY_MS` for startup dispatch windows and added focused unit coverage around default, invalid-value fallback, and non-startup behavior.
- `compiler/meshc/tests/e2e_m053_s02.rs` — Made the staged Postgres failover rail fail closed on pre-kill startup-window diagnostics and repointed the hosted-red comparison to a tracked fixture bundle.
- `scripts/verify-m053-s02.sh` — Replayed the assembled starter failover verifier and retained the S02 proof bundle shape used by the hosted chain.
- `scripts/fixtures/m053-s02-hosted-failure-bundle/SOURCE.txt` — Tracked clean-runner hosted-red regression data so full `e2e_m053_s02` targets no longer depend on prior `.tmp` history.
- `.tmp/m053-s06/rollout/final-hosted-closeout.md` — Recorded the final one-SHA hosted closeout after fresh main and release workflows turned green.

## Forward Intelligence

### What the next slice should know
- If the hosted starter failover proof goes red again, inspect the retained pre-kill diagnostics and `startup_dispatch_window.pending_window_ms` first; this slice proved the last blocker was timing truth, not missing promotion plumbing.
- Local S02 replays do not need a shared external database. A disposable Docker Postgres on a high local port is enough because the tests create isolated databases per run.

### What's fragile
- Synthetic rollout ship-sets — leaving out tracked `scripts/fixtures/...` regression data can make clean-runner CI fail even when the local worktree is green.
- Hosted freshness depends on main/tag workflow convergence. A locally green failover rail is not enough if `remote-runs.json` shows different SHAs or an unpeeled tag.

### Authoritative diagnostics
- `.tmp/m053-s03/verify/remote-runs.json` — the truthful source for whether `authoritative-verification.yml`, `deploy-services.yml`, and `release.yml` all point at the same shipped SHA.
- `.tmp/m053-s02/verify/` pre-kill diagnostics and continuity/status artifacts — the fastest way to tell whether the runtime actually held the startup window open long enough for owner-loss promotion.
- `.tmp/m053-s06/rollout/final-hosted-closeout.md` — the final stitched narrative tying the runtime fix, synthetic rollout fix, and annotated tag reroll together.

### What assumptions changed
- The earlier working assumption was that hosted red meant the standby promotion/mirrored-state transport was still broken. The final root cause was narrower: `startup_dispatch_window_ms(...)` ignored the runtime-owned `MESH_STARTUP_WORK_DELAY_MS` override and let startup complete before the forced owner stop.
