---
id: T01
parent: S06
milestone: M034
provides: []
requires: []
affects: []
key_files: ["scripts/verify-m034-s05.sh", "scripts/verify-m034-s06-remote-evidence.sh", "scripts/tests/verify-m034-s06-contract.test.mjs", ".tmp/m034-s06/evidence/preflight/manifest.json", ".tmp/m034-s06/evidence/preflight/remote-runs.json", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Fail the S06 archive helper closed on label reuse so deterministic labels like `preflight` and `first-green` cannot be silently overwritten.", "Use the canonical S05 verifier in a `remote-evidence` stop-after mode instead of building a parallel hosted-evidence codepath."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified shell syntax for both scripts, ran the new `node:test` contract suite, reran the inherited S05/S02/S04 workflow verifiers, captured the real hosted-red `preflight` archive bundle under `.tmp/m034-s06/evidence/preflight/`, and confirmed the canonical full S05 checkpoint still truthfully fails at `remote-evidence` rather than moving on to `public-http`."
completed_at: 2026-03-27T04:12:27.277Z
blocker_discovered: false
---

# T01: Added a stop-after remote-evidence mode to the S05 verifier, shipped the S06 archive helper, and captured the hosted-red preflight bundle.

> Added a stop-after remote-evidence mode to the S05 verifier, shipped the S06 archive helper, and captured the hosted-red preflight bundle.

## What Happened
---
id: T01
parent: S06
milestone: M034
key_files:
  - scripts/verify-m034-s05.sh
  - scripts/verify-m034-s06-remote-evidence.sh
  - scripts/tests/verify-m034-s06-contract.test.mjs
  - .tmp/m034-s06/evidence/preflight/manifest.json
  - .tmp/m034-s06/evidence/preflight/remote-runs.json
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Fail the S06 archive helper closed on label reuse so deterministic labels like `preflight` and `first-green` cannot be silently overwritten.
  - Use the canonical S05 verifier in a `remote-evidence` stop-after mode instead of building a parallel hosted-evidence codepath.
duration: ""
verification_result: mixed
completed_at: 2026-03-27T04:12:27.279Z
blocker_discovered: false
---

# T01: Added a stop-after remote-evidence mode to the S05 verifier, shipped the S06 archive helper, and captured the hosted-red preflight bundle.

**Added a stop-after remote-evidence mode to the S05 verifier, shipped the S06 archive helper, and captured the hosted-red preflight bundle.**

## What Happened

Extended `scripts/verify-m034-s05.sh` with an explicit `remote-evidence` stop-after contract exposed through both `VERIFY_M034_S05_STOP_AFTER=remote-evidence` and `--stop-after remote-evidence`, so the canonical verifier can halt cleanly before `public-http` and `s01-live-proof`. Added `scripts/verify-m034-s06-remote-evidence.sh` to run that mode, fail closed on invalid or reused labels, verify the expected phase boundary, archive the full `.tmp/m034-s05/verify/` bundle into `.tmp/m034-s06/evidence/<label>/`, and write a deterministic `manifest.json` with git refs, candidate tags, remote run summaries, and copied contents. Added `scripts/tests/verify-m034-s06-contract.test.mjs` to pin the S05 boundary and the S06 archive helper’s red-evidence, missing-artifact, and overwrite-refusal behavior. Ran the real helper with `preflight` and captured the truthful hosted-red baseline for downstream rollout tasks.

## Verification

Verified shell syntax for both scripts, ran the new `node:test` contract suite, reran the inherited S05/S02/S04 workflow verifiers, captured the real hosted-red `preflight` archive bundle under `.tmp/m034-s06/evidence/preflight/`, and confirmed the canonical full S05 checkpoint still truthfully fails at `remote-evidence` rather than moving on to `public-http`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -n scripts/verify-m034-s05.sh` | 0 | ✅ pass | 57ms |
| 2 | `bash -n scripts/verify-m034-s06-remote-evidence.sh` | 0 | ✅ pass | 38ms |
| 3 | `node --test scripts/tests/verify-m034-s06-contract.test.mjs` | 0 | ✅ pass | 1282ms |
| 4 | `bash scripts/verify-m034-s05-workflows.sh` | 0 | ✅ pass | 794ms |
| 5 | `bash scripts/verify-m034-s02-workflows.sh` | 0 | ✅ pass | 1254ms |
| 6 | `bash scripts/verify-m034-s04-workflows.sh` | 0 | ✅ pass | 756ms |
| 7 | `bash scripts/verify-m034-s06-remote-evidence.sh preflight` | 1 | ✅ pass | 126200ms |
| 8 | `test -f .tmp/m034-s06/evidence/preflight/remote-runs.json && test -f .tmp/m034-s06/evidence/preflight/candidate-tags.json && test -f .tmp/m034-s06/evidence/preflight/manifest.json` | 0 | ✅ pass | 24ms |
| 9 | `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh || test "$(cat .tmp/m034-s05/verify/failed-phase.txt)" = "public-http"` | 1 | ❌ fail | 138000ms |
| 10 | `grep -Fx 'remote-evidence	passed' .tmp/m034-s05/verify/phase-report.txt` | 1 | ❌ fail | 26ms |


## Deviations

Did not consume the slice-level `first-green` label during T01. The new helper intentionally fails closed on label reuse, so spending `first-green` before T04 would block the first all-green archive capture. Used the task-scoped `preflight` label from the task plan instead.

## Known Issues

Remote hosted rollout evidence is still red: `deploy.yml` on remote `main` is missing the expected `Verify public docs contract` step, `authoritative-verification.yml` and `extension-release-proof.yml` are not present remotely yet, and there are no current `v0.1.0` / `ext-v0.3.0` hosted runs. The canonical full S05 verifier still stops at `remote-evidence`; later slice tasks must push the updated workflow graph and tags before the slice-level `public-http` checkpoint can turn green.

## Files Created/Modified

- `scripts/verify-m034-s05.sh`
- `scripts/verify-m034-s06-remote-evidence.sh`
- `scripts/tests/verify-m034-s06-contract.test.mjs`
- `.tmp/m034-s06/evidence/preflight/manifest.json`
- `.tmp/m034-s06/evidence/preflight/remote-runs.json`
- `.gsd/KNOWLEDGE.md`


## Deviations
Did not consume the slice-level `first-green` label during T01. The new helper intentionally fails closed on label reuse, so spending `first-green` before T04 would block the first all-green archive capture. Used the task-scoped `preflight` label from the task plan instead.

## Known Issues
Remote hosted rollout evidence is still red: `deploy.yml` on remote `main` is missing the expected `Verify public docs contract` step, `authoritative-verification.yml` and `extension-release-proof.yml` are not present remotely yet, and there are no current `v0.1.0` / `ext-v0.3.0` hosted runs. The canonical full S05 verifier still stops at `remote-evidence`; later slice tasks must push the updated workflow graph and tags before the slice-level `public-http` checkpoint can turn green.
