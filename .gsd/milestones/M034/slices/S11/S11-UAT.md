# S11: First-green archive and final assembly closeout — UAT

**Milestone:** M034
**Written:** 2026-03-27T22:48:14.662Z

# S11: First-green archive and final assembly closeout — UAT

**Milestone:** M034
**Written:** 2026-03-27

## UAT Type

- UAT mode: artifact-driven
- Why this mode is sufficient: S11 did not ship a new runtime surface; it verified closeout truth by replaying the canonical wrapper and inspecting the generated verifier artifacts plus hosted diagnostics.

## Preconditions

- Worktree is the repo root with the S11 evidence present under `.tmp/m034-s11/` and `.tmp/m034-s05/verify/`.
- GitHub CLI access is available for the stop-after replay to refresh hosted workflow evidence.
- The approved rollout refs remain `main` at `e59f18203a30951af5288791bf9aed5b53a24a2a`, binary tag `v0.1.0` at `1e83ea930fdfd346b9e56659dc50d2f759ec5da2`, and extension tag `ext-v0.3.0` at `8e6d49dacc4f4cd64824b032078ae45aabfe9635`.

## Smoke Test

Run `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh` and confirm the wrapper stops at `remote-evidence` with `.tmp/m034-s05/verify/current-phase.txt` equal to `remote-evidence` and `.tmp/m034-s05/verify/status.txt` equal to `failed`.

## Test Cases

### 1. Local release-workflow contract is still green

1. Run `bash scripts/verify-m034-s02-workflows.sh`.
2. Run `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1`.
3. Run `bash -n scripts/verify-m034-s03.sh`.
4. **Expected:** all three commands exit 0, proving the repo-owned release workflow contract and PowerShell smoke helper are still aligned locally.

### 2. Stop-after remote-evidence replay isolates the remaining hosted blocker

1. Remove or ignore any stale conclusions from earlier verify roots.
2. Run `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh`.
3. Inspect `.tmp/m034-s05/verify/status.txt`, `.tmp/m034-s05/verify/current-phase.txt`, and `.tmp/m034-s05/verify/phase-report.txt`.
4. Inspect `.tmp/m034-s05/verify/remote-runs.json`.
5. **Expected:** `status.txt` is `failed`, `current-phase.txt` is `remote-evidence`, `phase-report.txt` shows all earlier local phases passed before `remote-evidence` failed, and `remote-runs.json` shows five workflows with `status: ok` while `release.yml` is the only workflow with `status: failed` on `v0.1.0` SHA `1e83ea930fdfd346b9e56659dc50d2f759ec5da2`.

### 3. `first-green` is still unavailable because the hosted Windows release smoke is red

1. Check for `.tmp/m034-s06/evidence/first-green/manifest.json`.
2. Open `.tmp/m034-s11/t03/final-assembly-summary.json`.
3. Open `.tmp/m034-s11/t03/diag-download/windows/verify/run/07-hello-build.log`.
4. **Expected:** the manifest file is absent, the blocker summary reports `closeoutReady: false`, and `07-hello-build.log` contains `exit_code: -1073741819` for the installed `meshc.exe` smoke build.

## Edge Cases

### Hosted workflow status is fresh but still red

1. Inspect `.tmp/m034-s05/verify/remote-runs.json` for `freshnessStatus` and `headShaMatchesExpected` on the `release.yml` entry.
2. **Expected:** freshness stays `ok` and the observed SHA matches the expected tag SHA even though the workflow status is `failed`, proving the blocker is the hosted Windows build behavior rather than stale ref propagation.

### Do not spend `first-green` on a red replay

1. Confirm `.tmp/m034-s06/evidence/first-green/manifest.json` is absent before any rerun.
2. Confirm the stop-after replay is still red at `remote-evidence`.
3. **Expected:** no `first-green` archive is created while the release lane remains red.

## Failure Signals

- `bash scripts/verify-m034-s02-workflows.sh` or `scripts/tests/verify-m034-s03-last-exitcode.ps1` fails locally.
- `.tmp/m034-s05/verify/phase-report.txt` no longer reaches `remote-evidence` after earlier local phases pass.
- `.tmp/m034-s05/verify/remote-runs.json` shows additional red workflows beyond `release.yml`, or `release.yml` points at the wrong SHA/ref.
- `07-hello-build.log` no longer exists in the downloaded diagnostics bundle, which would remove the only authoritative crash detail for the Windows blocker.

## Requirements Proved By This UAT

- R007 — The delivery proof surface remains trustworthy enough to separate repo-local workflow truth from the remaining hosted release-lane failure.

## Not Proven By This UAT

- It does not prove that `first-green` has been captured.
- It does not prove that the full `.env`-backed `bash scripts/verify-m034-s05.sh` replay is green end to end.
- It does not prove the root cause fix for the hosted Windows `meshc.exe` access violation; it only proves that the blocker is real and correctly localized.

## Notes for Tester

Treat S11 as a blocked closeout slice, not a green milestone finish. The truthful success condition for this UAT is that the evidence matches the current blocker story exactly. Do not fabricate a local full-replay success or spend the `first-green` label until `release.yml` on `v0.1.0` turns green.
