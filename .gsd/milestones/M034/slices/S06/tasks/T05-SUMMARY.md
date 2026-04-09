---
id: T05
parent: S06
milestone: M034
provides: []
requires: []
affects: []
key_files: ["scripts/verify-m034-s05.sh", "scripts/tests/verify-m034-s06-contract.test.mjs", ".tmp/m034-s05/verify/remote-runs.json", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M034/slices/S06/tasks/T05-SUMMARY.md"]
key_decisions: ["Derive reusable `extension-release-proof.yml` evidence from the `publish-extension.yml` caller run instead of querying the reusable workflow as a standalone push workflow.", "Treat the missing rollout commit on GitHub and stale remote `main` as a plan-invalidating blocker for `ext-v0.3.0` push and `first-green` archival."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "`bash -n scripts/verify-m034-s05.sh` and `node --test scripts/tests/verify-m034-s06-contract.test.mjs` both passed after the verifier change. Direct GitHub checks proved the blocker remains upstream of this task: remote `main` still points at `5ddf3b2dce17abe08e1188d9b46e575d83525b50`, the local rollout SHA is unknown to GitHub, `publish-extension.yml` still has no `ext-v0.3.0` push run, and direct `extension-release-proof.yml` listing still 404s because it is not a standalone hosted workflow surface. `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh` reran the real verifier and failed again at `remote-evidence`, with fresh artifacts written under `.tmp/m034-s05/verify/`."
completed_at: 2026-03-27T05:49:43.215Z
blocker_discovered: true
---

# T05: Retargeted S05 extension-proof polling to the publish workflow surface and confirmed the hosted rollout is still blocked before `remote-evidence`.

> Retargeted S05 extension-proof polling to the publish workflow surface and confirmed the hosted rollout is still blocked before `remote-evidence`.

## What Happened
---
id: T05
parent: S06
milestone: M034
key_files:
  - scripts/verify-m034-s05.sh
  - scripts/tests/verify-m034-s06-contract.test.mjs
  - .tmp/m034-s05/verify/remote-runs.json
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M034/slices/S06/tasks/T05-SUMMARY.md
key_decisions:
  - Derive reusable `extension-release-proof.yml` evidence from the `publish-extension.yml` caller run instead of querying the reusable workflow as a standalone push workflow.
  - Treat the missing rollout commit on GitHub and stale remote `main` as a plan-invalidating blocker for `ext-v0.3.0` push and `first-green` archival.
duration: ""
verification_result: mixed
completed_at: 2026-03-27T05:49:43.216Z
blocker_discovered: true
---

# T05: Retargeted S05 extension-proof polling to the publish workflow surface and confirmed the hosted rollout is still blocked before `remote-evidence`.

**Retargeted S05 extension-proof polling to the publish workflow surface and confirmed the hosted rollout is still blocked before `remote-evidence`.**

## What Happened

Confirmed `tools/editors/vscode-mesh/package.json` still yields `ext-v0.3.0`, then updated the S05 remote-evidence verifier so the reusable `extension-release-proof.yml` lane is observed through the real `publish-extension.yml` caller run and reusable-workflow-prefixed job names are accepted. Added a contract test that pins that query surface. Re-ran the authoritative S05 verifier in `remote-evidence` stop-after mode; all local phases passed through `s04-workflows`, but the run still failed at `remote-evidence` because GitHub is serving the pre-rollout branch graph: remote `main` remains on the old SHA, `authoritative-verification.yml` is absent from the remote default branch, and there are still no `push` runs for `v0.1.0` or `ext-v0.3.0`. I also confirmed the local rollout commit is not present on GitHub, so this task cannot truthfully create/push the extension tag or spend the reserved `first-green` archive label.

## Verification

`bash -n scripts/verify-m034-s05.sh` and `node --test scripts/tests/verify-m034-s06-contract.test.mjs` both passed after the verifier change. Direct GitHub checks proved the blocker remains upstream of this task: remote `main` still points at `5ddf3b2dce17abe08e1188d9b46e575d83525b50`, the local rollout SHA is unknown to GitHub, `publish-extension.yml` still has no `ext-v0.3.0` push run, and direct `extension-release-proof.yml` listing still 404s because it is not a standalone hosted workflow surface. `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh` reran the real verifier and failed again at `remote-evidence`, with fresh artifacts written under `.tmp/m034-s05/verify/`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -n scripts/verify-m034-s05.sh` | 0 | ✅ pass | 23ms |
| 2 | `node --test scripts/tests/verify-m034-s06-contract.test.mjs` | 0 | ✅ pass | 1408ms |
| 3 | `gh api repos/snowdamiz/mesh-lang/branches/main --jq .commit.sha` | 0 | ❌ fail | 713ms |
| 4 | `gh api repos/snowdamiz/mesh-lang/commits/6428dca29064e4c0e8ab54d210d2fe475e0b9f68 --jq .sha` | 1 | ❌ fail | 590ms |
| 5 | `gh run list -R snowdamiz/mesh-lang --workflow publish-extension.yml --event push --branch ext-v0.3.0 --limit 1 --json databaseId,status,conclusion,headSha,url` | 0 | ❌ fail | 649ms |
| 6 | `gh run list -R snowdamiz/mesh-lang --workflow extension-release-proof.yml --event push --branch ext-v0.3.0 --limit 1 --json databaseId,status,conclusion,headSha,url` | 1 | ❌ fail | 380ms |
| 7 | `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh` | 1 | ❌ fail | 166100ms |


## Deviations

Did not create or push `ext-v0.3.0`, did not wait on hosted extension runs, and did not archive `.tmp/m034-s06/evidence/first-green/`. Remote reality invalidated that part of the plan: the rollout SHA `6428dca29064e4c0e8ab54d210d2fe475e0b9f68` is not present on GitHub, remote `main` is still stale, and spending the reserved `first-green` label on a red hosted state would be dishonest.

## Known Issues

Remote `main` still resolves to `5ddf3b2dce17abe08e1188d9b46e575d83525b50`, `authoritative-verification.yml` is still absent from the remote default branch, there are still no hosted `push` runs for `v0.1.0` or `ext-v0.3.0`, and GitHub still does not know the local rollout commit `6428dca29064e4c0e8ab54d210d2fe475e0b9f68`.

## Files Created/Modified

- `scripts/verify-m034-s05.sh`
- `scripts/tests/verify-m034-s06-contract.test.mjs`
- `.tmp/m034-s05/verify/remote-runs.json`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M034/slices/S06/tasks/T05-SUMMARY.md`


## Deviations
Did not create or push `ext-v0.3.0`, did not wait on hosted extension runs, and did not archive `.tmp/m034-s06/evidence/first-green/`. Remote reality invalidated that part of the plan: the rollout SHA `6428dca29064e4c0e8ab54d210d2fe475e0b9f68` is not present on GitHub, remote `main` is still stale, and spending the reserved `first-green` label on a red hosted state would be dishonest.

## Known Issues
Remote `main` still resolves to `5ddf3b2dce17abe08e1188d9b46e575d83525b50`, `authoritative-verification.yml` is still absent from the remote default branch, there are still no hosted `push` runs for `v0.1.0` or `ext-v0.3.0`, and GitHub still does not know the local rollout commit `6428dca29064e4c0e8ab54d210d2fe475e0b9f68`.
