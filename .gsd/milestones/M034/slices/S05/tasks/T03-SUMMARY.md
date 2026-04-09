---
id: T03
parent: S05
milestone: M034
provides: []
requires: []
affects: []
key_files: ["scripts/verify-m034-s05.sh", ".gsd/DECISIONS.md", ".gsd/milestones/M034/slices/S05/tasks/T03-SUMMARY.md"]
key_decisions: ["Added a dedicated `public-http` phase before S01 so stale public release surfaces fail the assembled verifier before any live publish/install work runs.", "Proved public docs pages through normalized rendered HTML text plus exact installer body diffs instead of homepage-only reachability or raw HTML grep."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the wrapper in layers. `bash -n scripts/verify-m034-s05.sh` passed. The full assembled command `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh` ran through the local/docs/workflow/S02-S04 phases and failed at the intended `public-http` boundary on a real stale deploy surface, which is the honest intermediate-task result. `.tmp/m034-s05/verify/current-phase.txt`, `status.txt`, `phase-report.txt`, `public-http.log`, and `public-install-sh.diff` were all emitted and inspected to confirm the observability contract."
completed_at: 2026-03-27T02:53:30.729Z
blocker_discovered: false
---

# T03: Added the canonical S05 release-assembly verifier and made it fail on the first stale public install surface.

> Added the canonical S05 release-assembly verifier and made it fail on the first stale public install surface.

## What Happened
---
id: T03
parent: S05
milestone: M034
key_files:
  - scripts/verify-m034-s05.sh
  - .gsd/DECISIONS.md
  - .gsd/milestones/M034/slices/S05/tasks/T03-SUMMARY.md
key_decisions:
  - Added a dedicated `public-http` phase before S01 so stale public release surfaces fail the assembled verifier before any live publish/install work runs.
  - Proved public docs pages through normalized rendered HTML text plus exact installer body diffs instead of homepage-only reachability or raw HTML grep.
duration: ""
verification_result: mixed
completed_at: 2026-03-27T02:53:30.733Z
blocker_discovered: false
---

# T03: Added the canonical S05 release-assembly verifier and made it fail on the first stale public install surface.

**Added the canonical S05 release-assembly verifier and made it fail on the first stale public install surface.**

## What Happened

Created `scripts/verify-m034-s05.sh` as the canonical serial S05 acceptance command. The wrapper now resets and owns `.tmp/m034-s05/verify/`, records current phase plus phase history, writes final status, preserves per-phase logs, and stops at the first failing phase. It reuses the existing S05/S02/S03/S04 verifiers unchanged, adds local source-truth and built-docs truth phases over README/docs/installers/extension metadata, and adds an exact public HTTP phase that captures per-URL bodies/headers/status files before S01’s live proof. During execution I fixed two wrapper-side issues: the verify root was not being initialized before phase 1, and the S03 artifact post-check targeted the wrong upstream log filename. After those corrections, the assembled run passed through prereqs, workflow verifiers, docs build, local docs truth, built docs truth, S02, S03, S04 extension, and S04 workflows, then failed honestly at `public-http` because the deployed `https://meshlang.dev/install.sh` body is older than `website/docs/public/install.sh`. I also recorded D091 so downstream work extends the same public-truth contract.

## Verification

Verified the wrapper in layers. `bash -n scripts/verify-m034-s05.sh` passed. The full assembled command `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh` ran through the local/docs/workflow/S02-S04 phases and failed at the intended `public-http` boundary on a real stale deploy surface, which is the honest intermediate-task result. `.tmp/m034-s05/verify/current-phase.txt`, `status.txt`, `phase-report.txt`, `public-http.log`, and `public-install-sh.diff` were all emitted and inspected to confirm the observability contract.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -n scripts/verify-m034-s05.sh` | 0 | ✅ pass | 32ms |
| 2 | `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh` | 1 | ❌ fail | 85900ms |
| 3 | `test -f .tmp/m034-s05/verify/current-phase.txt` | 0 | ✅ pass | 20ms |
| 4 | `test -f .tmp/m034-s05/verify/status.txt && rg -n '^ok$' .tmp/m034-s05/verify/status.txt` | 1 | ❌ fail | 71ms |


## Deviations

Adjusted one local artifact assertion after reading the real S03 verifier output: the wrapper now checks `.tmp/m034-s03/verify/run/06-install-good.log` instead of a nonexistent `09-install.log`. Otherwise the implementation followed the task plan.

## Known Issues

The full assembled verifier currently fails at `public-http` because deployed `https://meshlang.dev/install.sh` is stale relative to `website/docs/public/install.sh`; `.tmp/m034-s05/verify/public-install-sh.diff` captures the exact drift, and the wrapper intentionally stops before S01 live publish/install when public truth is already red.

## Files Created/Modified

- `scripts/verify-m034-s05.sh`
- `.gsd/DECISIONS.md`
- `.gsd/milestones/M034/slices/S05/tasks/T03-SUMMARY.md`


## Deviations
Adjusted one local artifact assertion after reading the real S03 verifier output: the wrapper now checks `.tmp/m034-s03/verify/run/06-install-good.log` instead of a nonexistent `09-install.log`. Otherwise the implementation followed the task plan.

## Known Issues
The full assembled verifier currently fails at `public-http` because deployed `https://meshlang.dev/install.sh` is stale relative to `website/docs/public/install.sh`; `.tmp/m034-s05/verify/public-install-sh.diff` captures the exact drift, and the wrapper intentionally stops before S01 live publish/install when public truth is already red.
