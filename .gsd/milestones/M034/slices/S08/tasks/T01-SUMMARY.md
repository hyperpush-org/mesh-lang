---
id: T01
parent: S08
milestone: M034
provides: []
requires: []
affects: []
key_files: [".tmp/m034-s06/evidence/s08-prepush/manifest.json", ".tmp/m034-s06/evidence/s08-prepush/remote-runs.json", ".tmp/m034-s06/evidence/s08-prepush/status.txt", ".gsd/milestones/M034/slices/S08/tasks/T01-SUMMARY.md"]
key_decisions: ["Treat the existing `.tmp/m034-s06/evidence/v0.1.0/` directory as non-authoritative because it lacks the archive-helper contract files.", "Reserve `first-green` for the single final green capture and use `s08-prepush` as the disposable red baseline label."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the archive-helper contract with `node --test scripts/tests/verify-m034-s06-contract.test.mjs`, then ran the slice-level contract suite with `node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs`. Executed the real `.env`-backed pre-push replay with `bash scripts/verify-m034-s06-remote-evidence.sh s08-prepush` wrapped so the expected red result still yields exit code 0 for the task gate. Confirmed via Python assertions that `.tmp/m034-s06/evidence/s08-prepush/manifest.json`, `remote-runs.json`, and `status.txt` exist, `status.txt` is `failed`, the manifest stops after `remote-evidence` with non-zero S05 exit, the stale `v0.1.0` directory still lacks authoritative bundle files, and `first-green` remains absent."
completed_at: 2026-03-27T16:03:45.851Z
blocker_discovered: false
---

# T01: Captured a fresh s08-prepush red hosted-evidence bundle, kept first-green unused, and marked the stale v0.1.0 directory as incomplete noise.

> Captured a fresh s08-prepush red hosted-evidence bundle, kept first-green unused, and marked the stale v0.1.0 directory as incomplete noise.

## What Happened
---
id: T01
parent: S08
milestone: M034
key_files:
  - .tmp/m034-s06/evidence/s08-prepush/manifest.json
  - .tmp/m034-s06/evidence/s08-prepush/remote-runs.json
  - .tmp/m034-s06/evidence/s08-prepush/status.txt
  - .gsd/milestones/M034/slices/S08/tasks/T01-SUMMARY.md
key_decisions:
  - Treat the existing `.tmp/m034-s06/evidence/v0.1.0/` directory as non-authoritative because it lacks the archive-helper contract files.
  - Reserve `first-green` for the single final green capture and use `s08-prepush` as the disposable red baseline label.
duration: ""
verification_result: passed
completed_at: 2026-03-27T16:03:45.858Z
blocker_discovered: false
---

# T01: Captured a fresh s08-prepush red hosted-evidence bundle, kept first-green unused, and marked the stale v0.1.0 directory as incomplete noise.

**Captured a fresh s08-prepush red hosted-evidence bundle, kept first-green unused, and marked the stale v0.1.0 directory as incomplete noise.**

## What Happened

Inspected `.tmp/m034-s06/evidence/v0.1.0/` and confirmed it is missing the archive-helper contract files (`manifest.json`, `status.txt`, `current-phase.txt`, `phase-report.txt`, `remote-runs.json`), so it cannot be treated as authoritative proof. Confirmed `.tmp/m034-s06/evidence/first-green/` does not exist and left it reserved for the final green capture. Ran the repo-owned stop-after wrapper with repo-root `.env` loaded to create `.tmp/m034-s06/evidence/s08-prepush/`. The resulting bundle failed at `remote-evidence` as expected before any tag push, with `manifest.json` showing `stopAfterPhase=remote-evidence`, `s05ExitCode=1`, and remote-run summaries that keep the missing tag-scoped hosted runs explicit for `v0.1.0` and `ext-v0.3.0`. No wrapper or contract-test repair was needed.

## Verification

Verified the archive-helper contract with `node --test scripts/tests/verify-m034-s06-contract.test.mjs`, then ran the slice-level contract suite with `node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs`. Executed the real `.env`-backed pre-push replay with `bash scripts/verify-m034-s06-remote-evidence.sh s08-prepush` wrapped so the expected red result still yields exit code 0 for the task gate. Confirmed via Python assertions that `.tmp/m034-s06/evidence/s08-prepush/manifest.json`, `remote-runs.json`, and `status.txt` exist, `status.txt` is `failed`, the manifest stops after `remote-evidence` with non-zero S05 exit, the stale `v0.1.0` directory still lacks authoritative bundle files, and `first-green` remains absent.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m034-s06-contract.test.mjs` | 0 | ✅ pass | 1097ms |
| 2 | `node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs` | 0 | ✅ pass | 1104ms |
| 3 | `bash -c 'set -euo pipefail; test -f .env; set -a; source .env; set +a; rm -rf .tmp/m034-s06/evidence/s08-prepush; if bash scripts/verify-m034-s06-remote-evidence.sh s08-prepush; then echo "expected pre-push bundle to stay red before tags exist" >&2; exit 1; fi'` | 0 | ✅ pass | 108812ms |
| 4 | `python3 - <<'PY' ... validate stale v0.1.0 incompleteness, s08-prepush archive files, manifest stop-after state, and absent first-green ... PY` | 0 | ✅ pass | 125ms |


## Deviations

None.

## Known Issues

Hosted push-tag workflow runs for `v0.1.0` and `ext-v0.3.0` are still absent, which is the expected pre-push baseline for T01 rather than a plan-invalidating blocker.

## Files Created/Modified

- `.tmp/m034-s06/evidence/s08-prepush/manifest.json`
- `.tmp/m034-s06/evidence/s08-prepush/remote-runs.json`
- `.tmp/m034-s06/evidence/s08-prepush/status.txt`
- `.gsd/milestones/M034/slices/S08/tasks/T01-SUMMARY.md`


## Deviations
None.

## Known Issues
Hosted push-tag workflow runs for `v0.1.0` and `ext-v0.3.0` are still absent, which is the expected pre-push baseline for T01 rather than a plan-invalidating blocker.
