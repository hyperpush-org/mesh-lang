---
id: T03
parent: S03
milestone: M043
provides: []
requires: []
affects: []
key_files: ["cluster-proof/docker-entrypoint.sh", "cluster-proof/tests/config.test.mpl", "scripts/verify-m043-s03.sh", ".gsd/milestones/M043/slices/S03/tasks/T03-SUMMARY.md"]
key_decisions: ["Mirror the existing continuity role/epoch validation in the Docker entrypoint so contradictory env exits non-zero before the compiled package can log a config error and return success.", "Probe verifier-side misconfiguration with a temporary env file outside `.tmp/m043-s03/verify/` and assert both the early config error and the absence of runtime-start/cookie leakage in retained logs."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran `cargo run -q -p meshc -- test cluster-proof/tests` and `bash scripts/verify-m043-s03.sh`. The config suite passed with the expanded continuity matrix. The packaged verifier finished with `verify-m043-s03: ok`, `status.txt=ok`, and `current-phase.txt=complete`, including the new entrypoint-misconfig phase and the retained same-image Docker artifact checks."
completed_at: 2026-03-29T11:12:19.786Z
blocker_discovered: false
---

# T03: Made the same-image entrypoint fail closed on bad continuity env and added a packaged misconfiguration proof.

> Made the same-image entrypoint fail closed on bad continuity env and added a packaged misconfiguration proof.

## What Happened
---
id: T03
parent: S03
milestone: M043
key_files:
  - cluster-proof/docker-entrypoint.sh
  - cluster-proof/tests/config.test.mpl
  - scripts/verify-m043-s03.sh
  - .gsd/milestones/M043/slices/S03/tasks/T03-SUMMARY.md
key_decisions:
  - Mirror the existing continuity role/epoch validation in the Docker entrypoint so contradictory env exits non-zero before the compiled package can log a config error and return success.
  - Probe verifier-side misconfiguration with a temporary env file outside `.tmp/m043-s03/verify/` and assert both the early config error and the absence of runtime-start/cookie leakage in retained logs.
duration: ""
verification_result: passed
completed_at: 2026-03-29T11:12:19.788Z
blocker_discovered: false
---

# T03: Made the same-image entrypoint fail closed on bad continuity env and added a packaged misconfiguration proof.

**Made the same-image entrypoint fail closed on bad continuity env and added a packaged misconfiguration proof.**

## What Happened

Tightened cluster-proof/docker-entrypoint.sh so contradictory continuity role/epoch env now fails at the shell boundary with the same concrete error text as the Mesh config surface instead of deferring to the compiled binary. The entrypoint now preserves standalone mode, HOSTNAME-derived identity fallback, Fly identity fallback, and the valid primary/standby/stale-primary rails while rejecting blank role/epoch values, invalid roles, missing cluster-mode role input, and standby promotion epochs above 0. Updated cluster-proof/tests/config.test.mpl to cover explicit standby epoch 0, explicit stale-primary restart env (primary + epoch 0), and epoch-without-role cluster input. Extended scripts/verify-m043-s03.sh with an entrypoint-misconfig phase that runs the same-image container under contradictory continuity env, asserts the early config error, fail-closes if runtime startup occurs, and keeps the temporary env file outside the retained verifier artifacts so cookie material is not preserved there.

## Verification

Ran `cargo run -q -p meshc -- test cluster-proof/tests` and `bash scripts/verify-m043-s03.sh`. The config suite passed with the expanded continuity matrix. The packaged verifier finished with `verify-m043-s03: ok`, `status.txt=ok`, and `current-phase.txt=complete`, including the new entrypoint-misconfig phase and the retained same-image Docker artifact checks.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 9397ms |
| 2 | `bash scripts/verify-m043-s03.sh` | 0 | ✅ pass | 178100ms |


## Deviations

None. `cluster-proof/config.mpl` already contained the necessary topology validation, so no Mesh-side implementation change was needed there.

## Known Issues

None.

## Files Created/Modified

- `cluster-proof/docker-entrypoint.sh`
- `cluster-proof/tests/config.test.mpl`
- `scripts/verify-m043-s03.sh`
- `.gsd/milestones/M043/slices/S03/tasks/T03-SUMMARY.md`


## Deviations
None. `cluster-proof/config.mpl` already contained the necessary topology validation, so no Mesh-side implementation change was needed there.

## Known Issues
None.
