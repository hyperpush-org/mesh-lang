---
id: S11
parent: M034
milestone: M034
provides:
  - A durable blocker ledger showing that M034 closeout is held only by the hosted Windows release-smoke crash on `v0.1.0`
  - An authoritative hosted-workflow status snapshot proving that five of six rollout lanes are green on the approved refs
requires:
  - slice: S10
    provides: The approved rollout refs and local Windows release-lane fixes that already turned the other hosted lanes green before S11 revalidated the remaining blocker.
affects:
  []
key_files:
  - .tmp/m034-s11/t01/verification-summary.json
  - .tmp/m034-s05/verify/remote-runs.json
  - .tmp/m034-s05/verify/phase-report.txt
  - .tmp/m034-s11/t03/diag-download/windows/verify/run/07-hello-build.log
  - .tmp/m034-s11/t03/final-assembly-summary.json
  - .gsd/DECISIONS.md
key_decisions:
  - Left the local release workflow and PowerShell smoke verifier unchanged because replayed proofs showed the intended Windows workflow patch was already present.
  - Stopped before the full `.env`-backed S05 replay once `first-green` was still missing, instead of manufacturing stale closeout evidence from an earlier verify root.
  - Treat the uploaded Windows diagnostics artifact (`07-hello-build.log`) as the authoritative blocker surface for the remaining hosted `release.yml` failure.
patterns_established:
  - Separate repo-local workflow-contract verification from hosted-rollout freshness so remote failures are not misdiagnosed as local drift.
  - Use `VERIFY_M034_S05_STOP_AFTER=remote-evidence` plus artifact-level diagnostics before attempting the full public-surface replay when closeout depends on hosted workflow truth.
observability_surfaces:
  - `.tmp/m034-s05/verify/remote-runs.json` for per-workflow freshness and success/failure on the approved refs
  - `.tmp/m034-s05/verify/status.txt`, `.tmp/m034-s05/verify/current-phase.txt`, and `.tmp/m034-s05/verify/phase-report.txt` for truthful stop-after gate state
  - `.tmp/m034-s11/t03/diag-download/windows/verify/run/07-hello-build.log` for the actionable Windows release-smoke crash evidence
drill_down_paths:
  - .gsd/milestones/M034/slices/S11/tasks/T01-SUMMARY.md
  - .gsd/milestones/M034/slices/S11/tasks/T02-SUMMARY.md
  - .gsd/milestones/M034/slices/S11/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-27T22:48:14.660Z
blocker_discovered: false
---

# S11: First-green archive and final assembly closeout

**Rebuilt the S11 closeout evidence, confirmed that five hosted rollout lanes are green on the approved refs, and isolated the remaining M034 blocker to the hosted Windows release-smoke crash that still prevents `first-green` capture and the final S05 replay.**

## What Happened

S11 started by rechecking the repo-owned release/workflow contract before blaming hosted rollout. T01 reran the local workflow and PowerShell smoke proofs and confirmed the intended Windows release-lane patch was already present locally, so the remaining drift was not in `.github/workflows/release.yml`, `scripts/verify-m034-s02-workflows.sh`, or `scripts/verify-m034-s03.ps1`.

The hosted handoff that T02 was supposed to provide was not actually there: `first-green` was still absent, and T02 never produced a durable local summary. T03 therefore rebuilt the authoritative stop-after replay from a clean verify root with `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh` instead of trusting stale partial artifacts. That replay showed the current rollout state clearly: `deploy.yml` and `authoritative-verification.yml` are green on `main` SHA `e59f18203a30951af5288791bf9aed5b53a24a2a`, `deploy-services.yml` is green on binary tag `v0.1.0` SHA `1e83ea930fdfd346b9e56659dc50d2f759ec5da2`, and the extension proof/publish lanes are green on `ext-v0.3.0` SHA `8e6d49dacc4f4cd64824b032078ae45aabfe9635`. The only remaining red lane is `release.yml` on `v0.1.0`.

To avoid stale milestone-closeout claims, T03 stopped before the `.env`-backed full `scripts/verify-m034-s05.sh` replay because the prerequisite archive `.tmp/m034-s06/evidence/first-green/manifest.json` still does not exist. It then downloaded the hosted Windows diagnostics artifact for `release.yml` run `23669185030` and anchored the blocker to the only trustworthy low-level signal: `.tmp/m034-s11/t03/diag-download/windows/verify/run/07-hello-build.log`, which records `exit_code: -1073741819` during `installed meshc.exe build installer smoke fixture` with empty stdout/stderr. The slice therefore produced a truthful closeout ledger, not the intended first-green archive: local workflow truth is green, rollout freshness is mostly green, and the remaining work is concentrated in the installed Windows compiler path on the release tag lane.

## Verification

Passed repo-local release/workflow contract verification with `bash scripts/verify-m034-s02-workflows.sh`, `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1`, `bash -n scripts/verify-m034-s03.sh`, and `node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs`. Replayed `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh` from a clean verify root and confirmed it fails truthfully at `remote-evidence`, with `.tmp/m034-s05/verify/status.txt = failed`, `.tmp/m034-s05/verify/current-phase.txt = remote-evidence`, and `.tmp/m034-s05/verify/phase-report.txt` showing all earlier local phases passed before the hosted gate failed. Confirmed `.tmp/m034-s06/evidence/first-green/manifest.json` is absent. Verified the hosted blocker via `.tmp/m034-s11/t03/diag-download/windows/verify/run/07-hello-build.log`, which records `exit_code: -1073741819` for the installed Windows `meshc.exe` smoke build.

## Requirements Advanced

- R007 — Preserved the delivery-proof contract by proving that the remaining confidence gap is a hosted Windows release-smoke failure, not repo-local workflow drift or stale rollout refs.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

T01 required no source edits because the local workflow patch was already present. T02 did not produce the planned hosted-rollout handoff or `first-green` archive. T03 therefore deviated from the written plan by stopping before the full `.env`-backed S05 replay and converting the task into blocker capture once the required `first-green` precondition failed.

## Known Limitations

`.tmp/m034-s06/evidence/first-green/manifest.json` is still missing, so the one-shot archive has not been spent and the final `bash scripts/verify-m034-s05.sh` assembly replay cannot be claimed. The blocking hosted failure is still `release.yml` run `23669185030` on `v0.1.0` SHA `1e83ea930fdfd346b9e56659dc50d2f759ec5da2`, specifically `Verify release assets (x86_64-pc-windows-msvc)` crashing during `installed meshc.exe build installer smoke fixture` with access-violation exit code `-1073741819`.

## Follow-ups

Investigate and fix the installed Windows `meshc.exe build` crash on the release-tag smoke path, using the downloaded diagnostics artifact as the starting point. After that fix lands on the approved rollout ref, rerun `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh` until every hosted lane is green, claim `.tmp/m034-s06/evidence/first-green/` exactly once, and only then run the full `.env`-backed `bash scripts/verify-m034-s05.sh` replay for milestone revalidation.

## Files Created/Modified

- `.tmp/m034-s11/t01/verification-summary.json` — Captured fresh local proof that the release workflow contract and PowerShell smoke helper already match the intended repo-owned state.
- `.tmp/m034-s05/verify/remote-runs.json` — Refreshed the authoritative hosted-workflow snapshot showing five lanes green and `release.yml` still red on the approved tag ref.
- `.tmp/m034-s05/verify/phase-report.txt` — Recorded the stop-after replay phase sequence, proving that the wrapper fails at `remote-evidence` after all earlier local phases pass.
- `.tmp/m034-s11/t03/diag-download/windows/verify/run/07-hello-build.log` — Preserved the actionable hosted Windows installer-smoke crash log with `exit_code: -1073741819`.
- `.tmp/m034-s11/t03/final-assembly-summary.json` — Saved the blocker-oriented closeout summary tying the missing `first-green` archive to the still-red `release.yml` lane.
- `.gsd/DECISIONS.md` — Appended the S11 closeout decision to fail closed at `remote-evidence` and treat the Windows diagnostics artifact as authoritative.
