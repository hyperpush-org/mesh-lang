---
id: T03
parent: S07
milestone: M034
provides: []
requires: []
affects: []
key_files: [".tmp/m034-s05/verify/status.txt", ".tmp/m034-s05/verify/current-phase.txt", ".tmp/m034-s05/verify/phase-report.txt", ".tmp/m034-s05/verify/remote-evidence.log", ".tmp/m034-s05/verify/remote-runs.json", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M034/slices/S07/tasks/T03-SUMMARY.md"]
key_decisions: ["Reproduced the blocker through the unmodified canonical S05 verifier instead of bypassing it with ad hoc hosted checks.", "Treated the missing first-green bundle and stale remote workflow graph as a plan-invalidating dependency gap, not as a local public-http or S01 regression."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the exact canonical replay command from the task plan after a bounded `--stop-after remote-evidence` reproduction. Both replays failed at `remote-evidence` after all pre-remote local phases passed. Fresh hosted probes confirmed the blocker is still the remote rollout state: `main` is only at `8d3e76a65986e7bd5e00d107f9e11c1923ecd0ab`, `deploy.yml` is green on that SHA but still lacks the stronger `Verify public docs contract` step in the recorded hosted run, `authoritative-verification.yml` is absent from the remote default branch, and there are still no push runs for `v0.1.0` or `ext-v0.3.0`. Fresh post-run checks confirmed the S05 replay artifacts are not in the required final state (`status != ok`, `current-phase != complete`, no `remote-evidence/public-http/s01-live-proof` passes, and empty `public-http.log`)."
completed_at: 2026-03-27T08:04:52.637Z
blocker_discovered: true
---

# T03: Reproduced the canonical S05 replay blocker: `remote-evidence` is still red because remote `main` and the candidate tags are not fully rolled out.

> Reproduced the canonical S05 replay blocker: `remote-evidence` is still red because remote `main` and the candidate tags are not fully rolled out.

## What Happened
---
id: T03
parent: S07
milestone: M034
key_files:
  - .tmp/m034-s05/verify/status.txt
  - .tmp/m034-s05/verify/current-phase.txt
  - .tmp/m034-s05/verify/phase-report.txt
  - .tmp/m034-s05/verify/remote-evidence.log
  - .tmp/m034-s05/verify/remote-runs.json
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M034/slices/S07/tasks/T03-SUMMARY.md
key_decisions:
  - Reproduced the blocker through the unmodified canonical S05 verifier instead of bypassing it with ad hoc hosted checks.
  - Treated the missing first-green bundle and stale remote workflow graph as a plan-invalidating dependency gap, not as a local public-http or S01 regression.
duration: ""
verification_result: mixed
completed_at: 2026-03-27T08:04:52.639Z
blocker_discovered: true
---

# T03: Reproduced the canonical S05 replay blocker: `remote-evidence` is still red because remote `main` and the candidate tags are not fully rolled out.

**Reproduced the canonical S05 replay blocker: `remote-evidence` is still red because remote `main` and the candidate tags are not fully rolled out.**

## What Happened

Read the task contract, prior S07 summaries, and the canonical `scripts/verify-m034-s05.sh` / `scripts/verify-m034-s01.sh` entrypoints before touching the replay. Confirmed `.env` exists without printing it, then checked the named T03 input state and found that `.tmp/m034-s06/evidence/first-green/remote-runs.json` is still absent. Refreshed hosted truth directly: `origin/main` is still `8d3e76a65986e7bd5e00d107f9e11c1923ecd0ab`, `deploy.yml` has one green push run on that SHA, `authoritative-verification.yml` is still missing from the remote default branch, and `release.yml`, `deploy-services.yml`, and `publish-extension.yml` still have no push runs for `v0.1.0` / `ext-v0.3.0`. Reproduced the blocker through the canonical verifier twice: first with `bash scripts/verify-m034-s05.sh --stop-after remote-evidence` to confirm the failure boundary without paying for a doomed live publish/install cycle, then with the exact task-plan command `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh`. Both runs passed every local prerequisite phase and then failed at `remote-evidence`. The fresh canonical artifacts now show `status.txt=failed`, `current-phase.txt=remote-evidence`, `phase-report.txt` ending at `remote-evidence\tfailed`, and an empty `public-http.log` because the replay never reached `public-http`. I also documented the stale-artifact gotcha in `.gsd/KNOWLEDGE.md`: an older `.tmp/m034-s01/verify/*/package-version.txt` can still exist even when the current replay never reached `s01-live-proof`. This is a genuine dependency blocker rather than a local verifier bug, so I marked `blockerDiscovered: true` to trigger a truthful replan from the remaining remote-rollout gap.

## Verification

Ran the exact canonical replay command from the task plan after a bounded `--stop-after remote-evidence` reproduction. Both replays failed at `remote-evidence` after all pre-remote local phases passed. Fresh hosted probes confirmed the blocker is still the remote rollout state: `main` is only at `8d3e76a65986e7bd5e00d107f9e11c1923ecd0ab`, `deploy.yml` is green on that SHA but still lacks the stronger `Verify public docs contract` step in the recorded hosted run, `authoritative-verification.yml` is absent from the remote default branch, and there are still no push runs for `v0.1.0` or `ext-v0.3.0`. Fresh post-run checks confirmed the S05 replay artifacts are not in the required final state (`status != ok`, `current-phase != complete`, no `remote-evidence/public-http/s01-live-proof` passes, and empty `public-http.log`).

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh` | 1 | ❌ fail | 212200ms |
| 2 | `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh --stop-after remote-evidence` | 1 | ❌ fail | 209900ms |
| 3 | `gh api repos/snowdamiz/mesh-lang/branches/main --jq '.commit.sha'` | 0 | ✅ pass | 615ms |
| 4 | `gh run list -R snowdamiz/mesh-lang --workflow deploy.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url` | 0 | ✅ pass | 568ms |
| 5 | `gh run list -R snowdamiz/mesh-lang --workflow authoritative-verification.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url` | 1 | ❌ fail | 369ms |
| 6 | `gh run list -R snowdamiz/mesh-lang --workflow release.yml --event push --branch v0.1.0 --limit 1 --json databaseId,status,conclusion,headSha,url` | 0 | ❌ fail | 623ms |
| 7 | `gh run list -R snowdamiz/mesh-lang --workflow deploy-services.yml --event push --branch v0.1.0 --limit 1 --json databaseId,status,conclusion,headSha,url` | 0 | ❌ fail | 557ms |
| 8 | `gh run list -R snowdamiz/mesh-lang --workflow publish-extension.yml --event push --branch ext-v0.3.0 --limit 1 --json databaseId,status,conclusion,headSha,url` | 0 | ❌ fail | 640ms |
| 9 | `grep -Fx 'ok' .tmp/m034-s05/verify/status.txt` | 1 | ❌ fail | 24ms |
| 10 | `grep -Fx 'complete' .tmp/m034-s05/verify/current-phase.txt` | 1 | ❌ fail | 28ms |
| 11 | `grep -Fx 'remote-evidence	passed' .tmp/m034-s05/verify/phase-report.txt` | 1 | ❌ fail | 23ms |
| 12 | `grep -Fx 'public-http	passed' .tmp/m034-s05/verify/phase-report.txt` | 1 | ❌ fail | 20ms |
| 13 | `grep -Fx 's01-live-proof	passed' .tmp/m034-s05/verify/phase-report.txt` | 1 | ❌ fail | 19ms |
| 14 | `test -s .tmp/m034-s05/verify/public-http.log` | 1 | ❌ fail | 90ms |
| 15 | `find .tmp/m034-s01/verify -mindepth 2 -maxdepth 2 -name package-version.txt | grep -q .` | 0 | ✅ pass | 76ms |
| 16 | `test -f .tmp/m034-s06/evidence/first-green/remote-runs.json` | 1 | ❌ fail | 13ms |


## Deviations

The task plan assumed `.tmp/m034-s06/evidence/first-green/remote-runs.json` already existed and that T03 could focus on the final replay itself. In local reality that bundle is absent because the current workflow graph has not yet landed on remote `main` and the `v0.1.0` / `ext-v0.3.0` tags still have no truthful push runs. I therefore documented the blocker through the canonical S05 verifier instead of inventing a final replay success or weakening the acceptance script.

## Known Issues

Remote rollout is still incomplete: `origin/main` remains at `8d3e76a65986e7bd5e00d107f9e11c1923ecd0ab`; `deploy.yml` is green there but still reflects the older hosted step graph; `authoritative-verification.yml` is absent on the remote default branch; and there are no push runs yet for `release.yml`, `deploy-services.yml`, or `publish-extension.yml` on `v0.1.0` / `ext-v0.3.0`. Until those prerequisites are fixed, `remote-evidence` will stay red and the canonical replay cannot honestly reach `public-http` or `s01-live-proof`.

## Files Created/Modified

- `.tmp/m034-s05/verify/status.txt`
- `.tmp/m034-s05/verify/current-phase.txt`
- `.tmp/m034-s05/verify/phase-report.txt`
- `.tmp/m034-s05/verify/remote-evidence.log`
- `.tmp/m034-s05/verify/remote-runs.json`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M034/slices/S07/tasks/T03-SUMMARY.md`


## Deviations
The task plan assumed `.tmp/m034-s06/evidence/first-green/remote-runs.json` already existed and that T03 could focus on the final replay itself. In local reality that bundle is absent because the current workflow graph has not yet landed on remote `main` and the `v0.1.0` / `ext-v0.3.0` tags still have no truthful push runs. I therefore documented the blocker through the canonical S05 verifier instead of inventing a final replay success or weakening the acceptance script.

## Known Issues
Remote rollout is still incomplete: `origin/main` remains at `8d3e76a65986e7bd5e00d107f9e11c1923ecd0ab`; `deploy.yml` is green there but still reflects the older hosted step graph; `authoritative-verification.yml` is absent on the remote default branch; and there are no push runs yet for `release.yml`, `deploy-services.yml`, or `publish-extension.yml` on `v0.1.0` / `ext-v0.3.0`. Until those prerequisites are fixed, `remote-evidence` will stay red and the canonical replay cannot honestly reach `public-http` or `s01-live-proof`.
