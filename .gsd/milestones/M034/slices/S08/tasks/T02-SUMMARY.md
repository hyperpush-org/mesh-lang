---
id: T02
parent: S08
milestone: M034
provides: []
requires: []
affects: []
key_files: [".tmp/m034-s08/tag-rollout/tag-refs.txt", ".tmp/m034-s08/tag-rollout/workflow-status.json", ".tmp/m034-s08/tag-rollout/release-v0.1.0-view.json", ".tmp/m034-s08/tag-rollout/release-v0.1.0-log-failed.txt", ".tmp/m034-s08/tag-rollout/deploy-services-v0.1.0-view.json", ".tmp/m034-s08/tag-rollout/deploy-services-v0.1.0-log-failed.txt", ".tmp/m034-s08/tag-rollout/publish-extension-ext-v0.3.0-view.json", ".tmp/m034-s08/tag-rollout/rollout_monitor.py", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Treat `origin/main` SHA `6979a4a17221af8e39200b574aa2209ad54bc983` as the rollout target because local `HEAD` had advanced to task-artifact commits after the hosted-green main push.", "Use `gh api repos/.../git/refs` to create the remote candidate tags after explicit approval instead of local git commands.", "Mark the slice blocked once durable hosted evidence showed `release.yml` and `deploy-services.yml` red, because T03 cannot truthfully archive `first-green` under those conditions."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified that both remote tag refs exist on the intended rollout SHA and that the saved hosted-run snapshots are structurally complete. Then ran the task-plan success gate exactly as written: the tag-ref check passed, and the success-only workflow assertion failed because `release.yml` remained red. Also ran a read-only assertion over `workflow-status.json` to confirm the settled hosted truth: `release.yml` and `deploy-services.yml` failed on `v0.1.0`, while `publish-extension.yml` succeeded on `ext-v0.3.0`, all on SHA `6979a4a17221af8e39200b574aa2209ad54bc983`."
completed_at: 2026-03-27T16:32:22.813Z
blocker_discovered: true
---

# T02: Created the candidate tags on the rolled-out SHA and captured the hosted blockers preventing a truthful first-green archive.

> Created the candidate tags on the rolled-out SHA and captured the hosted blockers preventing a truthful first-green archive.

## What Happened
---
id: T02
parent: S08
milestone: M034
key_files:
  - .tmp/m034-s08/tag-rollout/tag-refs.txt
  - .tmp/m034-s08/tag-rollout/workflow-status.json
  - .tmp/m034-s08/tag-rollout/release-v0.1.0-view.json
  - .tmp/m034-s08/tag-rollout/release-v0.1.0-log-failed.txt
  - .tmp/m034-s08/tag-rollout/deploy-services-v0.1.0-view.json
  - .tmp/m034-s08/tag-rollout/deploy-services-v0.1.0-log-failed.txt
  - .tmp/m034-s08/tag-rollout/publish-extension-ext-v0.3.0-view.json
  - .tmp/m034-s08/tag-rollout/rollout_monitor.py
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Treat `origin/main` SHA `6979a4a17221af8e39200b574aa2209ad54bc983` as the rollout target because local `HEAD` had advanced to task-artifact commits after the hosted-green main push.
  - Use `gh api repos/.../git/refs` to create the remote candidate tags after explicit approval instead of local git commands.
  - Mark the slice blocked once durable hosted evidence showed `release.yml` and `deploy-services.yml` red, because T03 cannot truthfully archive `first-green` under those conditions.
duration: ""
verification_result: mixed
completed_at: 2026-03-27T16:32:22.815Z
blocker_discovered: true
---

# T02: Created the candidate tags on the rolled-out SHA and captured the hosted blockers preventing a truthful first-green archive.

**Created the candidate tags on the rolled-out SHA and captured the hosted blockers preventing a truthful first-green archive.**

## What Happened

Derived the candidate tags from the Mesh compiler/package and VS Code extension version files, confirmed that remote `main` was still on the hosted-green rollout commit `6979a4a17221af8e39200b574aa2209ad54bc983`, and obtained explicit approval before mutating any remote refs. Created `v0.1.0` and `ext-v0.3.0` on that SHA through `gh api repos/snowdamiz/mesh-lang/git/refs`, then wrote a task-local rollout monitor that persisted the remote ref snapshot, per-workflow `gh run view` payloads, compact workflow status, and failed job logs under `.tmp/m034-s08/tag-rollout/`. The tag push triggered the expected hosted workflows on the correct refs and SHA. `publish-extension.yml` finished green on `ext-v0.3.0`, while `deploy-services.yml` and `release.yml` finished red on `v0.1.0`; their failed logs show the concrete blockers now recorded in `.gsd/KNOWLEDGE.md`. Because T03 is defined to claim `first-green` only after T02 leaves all required tag-triggered workflows green, the remaining slice plan is now blocked and needs replanning.

## Verification

Verified that both remote tag refs exist on the intended rollout SHA and that the saved hosted-run snapshots are structurally complete. Then ran the task-plan success gate exactly as written: the tag-ref check passed, and the success-only workflow assertion failed because `release.yml` remained red. Also ran a read-only assertion over `workflow-status.json` to confirm the settled hosted truth: `release.yml` and `deploy-services.yml` failed on `v0.1.0`, while `publish-extension.yml` succeeded on `ext-v0.3.0`, all on SHA `6979a4a17221af8e39200b574aa2209ad54bc983`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -c 'set -euo pipefail; test -s .tmp/m034-s08/tag-rollout/tag-refs.txt; grep -F "refs/tags/v0.1.0" .tmp/m034-s08/tag-rollout/tag-refs.txt; grep -F "refs/tags/ext-v0.3.0" .tmp/m034-s08/tag-rollout/tag-refs.txt'` | 0 | ✅ pass | 28ms |
| 2 | `python3 - <<'PY' ... assert release.yml / deploy-services.yml / publish-extension.yml are all completed success on the expected refs ... PY` | 1 | ❌ fail | 134ms |
| 3 | `python3 - <<'PY' ... assert workflow-status.json records release=failed, deploy-services=failed, publish-extension=success on SHA 6979a4a17221af8e39200b574aa2209ad54bc983 ... PY` | 0 | ✅ pass | 95ms |


## Deviations

Used `gh api repos/.../git/refs` instead of local `git tag` / `git push` commands so the approved outward mutation could happen without local git CLI usage. The outward action stayed equivalent: both candidate tags were created on the approved rollout SHA.

## Known Issues

`deploy-services.yml` is red on `v0.1.0` because the `packages-website` runtime Docker layer reruns `npm install --omit=dev --ignore-scripts` and fails with a Vite / Svelte peer-dependency `ERESOLVE` conflict. `release.yml` is red on `v0.1.0` because multiple `Verify release assets (...)` jobs fail: Unix installer smoke builds cannot find `libmesh_rt.a`, macOS checksum generation assumes `sha256sum`, and the Windows checksum step has broken PowerShell `Select-Object -First 1,` syntax. T03 cannot truthfully claim `.tmp/m034-s06/evidence/first-green/` until those hosted regressions are fixed.

## Files Created/Modified

- `.tmp/m034-s08/tag-rollout/tag-refs.txt`
- `.tmp/m034-s08/tag-rollout/workflow-status.json`
- `.tmp/m034-s08/tag-rollout/release-v0.1.0-view.json`
- `.tmp/m034-s08/tag-rollout/release-v0.1.0-log-failed.txt`
- `.tmp/m034-s08/tag-rollout/deploy-services-v0.1.0-view.json`
- `.tmp/m034-s08/tag-rollout/deploy-services-v0.1.0-log-failed.txt`
- `.tmp/m034-s08/tag-rollout/publish-extension-ext-v0.3.0-view.json`
- `.tmp/m034-s08/tag-rollout/rollout_monitor.py`
- `.gsd/KNOWLEDGE.md`


## Deviations
Used `gh api repos/.../git/refs` instead of local `git tag` / `git push` commands so the approved outward mutation could happen without local git CLI usage. The outward action stayed equivalent: both candidate tags were created on the approved rollout SHA.

## Known Issues
`deploy-services.yml` is red on `v0.1.0` because the `packages-website` runtime Docker layer reruns `npm install --omit=dev --ignore-scripts` and fails with a Vite / Svelte peer-dependency `ERESOLVE` conflict. `release.yml` is red on `v0.1.0` because multiple `Verify release assets (...)` jobs fail: Unix installer smoke builds cannot find `libmesh_rt.a`, macOS checksum generation assumes `sha256sum`, and the Windows checksum step has broken PowerShell `Select-Object -First 1,` syntax. T03 cannot truthfully claim `.tmp/m034-s06/evidence/first-green/` until those hosted regressions are fixed.
