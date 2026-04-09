---
id: T03
parent: S05
milestone: M053
provides: []
requires: []
affects: []
key_files: [".tmp/m053-s05/starter-proof-repro/root-cause.md", ".tmp/m053-s05/starter-proof-repro/ci-failure-classification.json", ".tmp/m053-s05/starter-proof-repro/recheck-cold-run.json", ".tmp/m053-s05/starter-proof-repro/recheck-control-run.json", ".tmp/m053-s05/starter-proof-repro/logs/recheck-m053-s01-example-e2e.log", ".tmp/m053-s05/starter-proof-repro/logs/recheck-e2e-m049-s03-after-mesh-rt.stdout.log", ".gsd/milestones/M053/slices/S05/tasks/T03-SUMMARY.md"]
key_decisions: ["Classify the starter-proof failure as environment drift: a cold isolated target dir lacked libmesh_rt.a, so the nested e2e rail failed before any product assertion fired.", "Preserve the raw nested Rust/test logs in .tmp/m053-s05/starter-proof-repro/ and treat the wrapper's 'expected success within <N>s' message as non-authoritative for timeout classification."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Reproduced the hosted-red path locally with a cold wrapper replay of bash scripts/verify-m053-s01.sh, confirmed the nested e2e_m049_s03 log contains 'Could not locate Mesh runtime static library ... Run `cargo build -p mesh-rt` first.', then reran the exact nested test after a same-target cargo build -p mesh-rt control and confirmed it passed with 'running 5 tests' and 'test result: ok'. Ran the task-plan artifact verification exactly as written and also asserted that the retained replay metadata shows the cold wrapper failed while the single-variable control passed."
completed_at: 2026-04-05T23:30:17.127Z
blocker_discovered: false
---

# T03: Reproduced the authoritative starter-proof failure as a missing mesh-rt prebuild in cold target dirs and retained CI-grade inner logs.

> Reproduced the authoritative starter-proof failure as a missing mesh-rt prebuild in cold target dirs and retained CI-grade inner logs.

## What Happened
---
id: T03
parent: S05
milestone: M053
key_files:
  - .tmp/m053-s05/starter-proof-repro/root-cause.md
  - .tmp/m053-s05/starter-proof-repro/ci-failure-classification.json
  - .tmp/m053-s05/starter-proof-repro/recheck-cold-run.json
  - .tmp/m053-s05/starter-proof-repro/recheck-control-run.json
  - .tmp/m053-s05/starter-proof-repro/logs/recheck-m053-s01-example-e2e.log
  - .tmp/m053-s05/starter-proof-repro/logs/recheck-e2e-m049-s03-after-mesh-rt.stdout.log
  - .gsd/milestones/M053/slices/S05/tasks/T03-SUMMARY.md
key_decisions:
  - Classify the starter-proof failure as environment drift: a cold isolated target dir lacked libmesh_rt.a, so the nested e2e rail failed before any product assertion fired.
  - Preserve the raw nested Rust/test logs in .tmp/m053-s05/starter-proof-repro/ and treat the wrapper's 'expected success within <N>s' message as non-authoritative for timeout classification.
duration: ""
verification_result: passed
completed_at: 2026-04-05T23:30:17.129Z
blocker_discovered: false
---

# T03: Reproduced the authoritative starter-proof failure as a missing mesh-rt prebuild in cold target dirs and retained CI-grade inner logs.

**Reproduced the authoritative starter-proof failure as a missing mesh-rt prebuild in cold target dirs and retained CI-grade inner logs.**

## What Happened

Started from the retained T02 hosted diagnostics and the shipped main SHA, then re-ran the failing starter-proof path locally in a clean CI-like environment instead of trusting the truncated uploaded artifact. Because the repo-root .env no longer exposed DATABASE_URL, I used the same runner-local Postgres shape already captured in the retained workflow metadata and ran bash scripts/verify-m053-s01.sh with a brand-new CARGO_HOME and CARGO_TARGET_DIR under .tmp/m053-s05/starter-proof-repro/. That replay failed at the same S01 phase as hosted CI, but the preserved inner Rust log showed the actual error was not a timeout and not a stable product assertion in e2e_m049_s03: the nested meshc test <project> path could not find libmesh_rt.a in the isolated target dir. I then changed one variable only: prebuilt mesh-rt into that same isolated target dir and reran the exact nested failing command cargo test -p meshc --test e2e_m049_s03 -- --nocapture. The prebuild passed, and the rerun passed with 5 tests green. That fail→single-change→pass sequence rules out a stable product assertion for this replay and classifies the hosted failure as environment/setup drift. Finally, I refreshed .tmp/m053-s05/starter-proof-repro/root-cause.md and .tmp/m053-s05/starter-proof-repro/ci-failure-classification.json so they point at the fresh recheck logs, name the exact failing command, and call out the current hosted diagnostic-retention gap: the uploaded artifact preserves only wrapper-level output and the first 220 lines of the nested log, which makes non-timeout failures look like compile-budget timeouts.

## Verification

Reproduced the hosted-red path locally with a cold wrapper replay of bash scripts/verify-m053-s01.sh, confirmed the nested e2e_m049_s03 log contains 'Could not locate Mesh runtime static library ... Run `cargo build -p mesh-rt` first.', then reran the exact nested test after a same-target cargo build -p mesh-rt control and confirmed it passed with 'running 5 tests' and 'test result: ok'. Ran the task-plan artifact verification exactly as written and also asserted that the retained replay metadata shows the cold wrapper failed while the single-variable control passed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `env DATABASE_URL=<workflow-like> CARGO_HOME=.tmp/m053-s05/starter-proof-repro/recheck-cargo-home CARGO_TARGET_DIR=.tmp/m053-s05/starter-proof-repro/recheck-target bash scripts/verify-m053-s01.sh` | 1 | ✅ pass (expected reproduced hosted failure; inner log retained) | 216560ms |
| 2 | `env DATABASE_URL=<workflow-like> CARGO_HOME=.tmp/m053-s05/starter-proof-repro/recheck-cargo-home CARGO_TARGET_DIR=.tmp/m053-s05/starter-proof-repro/recheck-target cargo build -p mesh-rt` | 0 | ✅ pass | 39770ms |
| 3 | `env DATABASE_URL=<workflow-like> CARGO_HOME=.tmp/m053-s05/starter-proof-repro/recheck-cargo-home CARGO_TARGET_DIR=.tmp/m053-s05/starter-proof-repro/recheck-target cargo test -p meshc --test e2e_m049_s03 -- --nocapture` | 0 | ✅ pass | 38780ms |
| 4 | `test -s .tmp/m053-s05/starter-proof-repro/root-cause.md && python3 - <<'PY' ...` | 0 | ✅ pass | 1ms |
| 5 | `python3 - <<'PY' ... assert cold replay failed, primary log contains missing libmesh_rt, and control rerun passed ... PY` | 0 | ✅ pass | 1ms |


## Deviations

The plan allowed rerunning the nested S01/S02 entrypoints or the targeted cargo test as needed. I did not rerun the full S02 failover wrapper because the failure had already been isolated to the nested S01 e2e_m049_s03 path and the repo-root .env no longer supplied DATABASE_URL. Instead I used the workflow-like Postgres settings preserved by the retained hosted metadata, reproduced the exact failing S01 wrapper path with fresh cargo caches, and then narrowed to the targeted nested test for the control probe. This was a local execution adaptation, not a plan-invalidating blocker.

## Known Issues

Hosted authoritative-starter-failover-proof.yml is still red on the shipped SHA until T04 bakes the environment/log-retention repair into the workflow/script path. The uploaded starter-proof diagnostics still do not preserve the raw nested log tail, and the wrapper's 'expected success within <N>s' message still collapses real command failures into timeout-shaped wording.

## Files Created/Modified

- `.tmp/m053-s05/starter-proof-repro/root-cause.md`
- `.tmp/m053-s05/starter-proof-repro/ci-failure-classification.json`
- `.tmp/m053-s05/starter-proof-repro/recheck-cold-run.json`
- `.tmp/m053-s05/starter-proof-repro/recheck-control-run.json`
- `.tmp/m053-s05/starter-proof-repro/logs/recheck-m053-s01-example-e2e.log`
- `.tmp/m053-s05/starter-proof-repro/logs/recheck-e2e-m049-s03-after-mesh-rt.stdout.log`
- `.gsd/milestones/M053/slices/S05/tasks/T03-SUMMARY.md`


## Deviations
The plan allowed rerunning the nested S01/S02 entrypoints or the targeted cargo test as needed. I did not rerun the full S02 failover wrapper because the failure had already been isolated to the nested S01 e2e_m049_s03 path and the repo-root .env no longer supplied DATABASE_URL. Instead I used the workflow-like Postgres settings preserved by the retained hosted metadata, reproduced the exact failing S01 wrapper path with fresh cargo caches, and then narrowed to the targeted nested test for the control probe. This was a local execution adaptation, not a plan-invalidating blocker.

## Known Issues
Hosted authoritative-starter-failover-proof.yml is still red on the shipped SHA until T04 bakes the environment/log-retention repair into the workflow/script path. The uploaded starter-proof diagnostics still do not preserve the raw nested log tail, and the wrapper's 'expected success within <N>s' message still collapses real command failures into timeout-shaped wording.
