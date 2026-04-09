---
id: T06
parent: S08
milestone: M034
provides: []
requires: []
affects: []
key_files: [".tmp/m034-s05/verify/remote-runs.json", ".tmp/m034-s05/verify/remote-evidence.log", ".tmp/m034-s05/verify/status.txt", ".gsd/milestones/M034/slices/S08/tasks/T06-SUMMARY.md"]
key_decisions: ["Do not run `scripts/verify-m034-s06-remote-evidence.sh first-green` while the stop-after `remote-evidence` preflight is still red; preserve the label for the first all-green hosted bundle."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Reran `node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs` and it passed. Ran `bash -c 'set -euo pipefail; test -f .env; set -a; source .env; set +a; VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh'`; it failed in the `remote-evidence` phase and refreshed `.tmp/m034-s05/verify/remote-runs.json` / `remote-evidence.log` with the current hosted blocker state. Verified `.tmp/m034-s06/evidence/first-green/` still does not exist, so the reserved final label remains unused."
completed_at: 2026-03-27T17:08:34.059Z
blocker_discovered: true
---

# T06: Reconfirmed `first-green` is still unused and captured a fresh remote-evidence blocker showing `deploy-services.yml` and `release.yml` remain red on `v0.1.0`.

> Reconfirmed `first-green` is still unused and captured a fresh remote-evidence blocker showing `deploy-services.yml` and `release.yml` remain red on `v0.1.0`.

## What Happened
---
id: T06
parent: S08
milestone: M034
key_files:
  - .tmp/m034-s05/verify/remote-runs.json
  - .tmp/m034-s05/verify/remote-evidence.log
  - .tmp/m034-s05/verify/status.txt
  - .gsd/milestones/M034/slices/S08/tasks/T06-SUMMARY.md
key_decisions:
  - Do not run `scripts/verify-m034-s06-remote-evidence.sh first-green` while the stop-after `remote-evidence` preflight is still red; preserve the label for the first all-green hosted bundle.
duration: ""
verification_result: mixed
completed_at: 2026-03-27T17:08:34.061Z
blocker_discovered: true
---

# T06: Reconfirmed `first-green` is still unused and captured a fresh remote-evidence blocker showing `deploy-services.yml` and `release.yml` remain red on `v0.1.0`.

**Reconfirmed `first-green` is still unused and captured a fresh remote-evidence blocker showing `deploy-services.yml` and `release.yml` remain red on `v0.1.0`.**

## What Happened

Re-read the T06 contract, slice plan, wrapper, and contract tests first so the reserved `first-green` label would not be spent accidentally. Confirmed `.tmp/m034-s06/evidence/first-green/` is still absent. Because the saved T05 rollout snapshot was still red, reran the S05/S06 contract tests and then used the safe `VERIFY_M034_S05_STOP_AFTER=remote-evidence` path on `scripts/verify-m034-s05.sh` instead of calling `scripts/verify-m034-s06-remote-evidence.sh first-green`. That refreshed `.tmp/m034-s05/verify/remote-runs.json` and `.tmp/m034-s05/verify/remote-evidence.log` without consuming any archive label. The hosted preflight still fails in `remote-evidence`: `deploy.yml`, `authoritative-verification.yml`, `extension-release-proof.yml`, and `publish-extension.yml` are green, but `deploy-services.yml` and `release.yml` still conclude `completed/failure` on `v0.1.0`. Stopped there, left `first-green` untouched, and documented the blocker for slice replanning.

## Verification

Reran `node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs` and it passed. Ran `bash -c 'set -euo pipefail; test -f .env; set -a; source .env; set +a; VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh'`; it failed in the `remote-evidence` phase and refreshed `.tmp/m034-s05/verify/remote-runs.json` / `remote-evidence.log` with the current hosted blocker state. Verified `.tmp/m034-s06/evidence/first-green/` still does not exist, so the reserved final label remains unused.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs` | 0 | ✅ pass | 1394ms |
| 2 | `bash -c 'set -euo pipefail; test -f .env; set -a; source .env; set +a; VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh'` | 1 | ❌ fail | 105413ms |
| 3 | `python3 - <<'PY' ... check Path('.tmp/m034-s06/evidence/first-green').exists()` | 0 | ✅ pass | 0ms |


## Deviations

Did not run `bash scripts/verify-m034-s06-remote-evidence.sh first-green` because the safe stop-after preflight was still red; consuming the reserved label would have created a dishonest or unusable final archive. This is a task-blocking upstream rollout issue, not a local wrapper-contract issue.

## Known Issues

GitHub still reports `deploy-services.yml` as `completed/failure` on `v0.1.0`, and `release.yml` is also `completed/failure` on `v0.1.0`. Until those hosted candidate-tag workflows go green, T06 cannot truthfully create `.tmp/m034-s06/evidence/first-green/`.

## Files Created/Modified

- `.tmp/m034-s05/verify/remote-runs.json`
- `.tmp/m034-s05/verify/remote-evidence.log`
- `.tmp/m034-s05/verify/status.txt`
- `.gsd/milestones/M034/slices/S08/tasks/T06-SUMMARY.md`


## Deviations
Did not run `bash scripts/verify-m034-s06-remote-evidence.sh first-green` because the safe stop-after preflight was still red; consuming the reserved label would have created a dishonest or unusable final archive. This is a task-blocking upstream rollout issue, not a local wrapper-contract issue.

## Known Issues
GitHub still reports `deploy-services.yml` as `completed/failure` on `v0.1.0`, and `release.yml` is also `completed/failure` on `v0.1.0`. Until those hosted candidate-tag workflows go green, T06 cannot truthfully create `.tmp/m034-s06/evidence/first-green/`.
