---
estimated_steps: 4
estimated_files: 5
skills_used:
  - github-workflows
  - debug-like-expert
---

# T03: Rerun hosted release smoke and refresh rollout evidence

**Slice:** S12 — Windows release-smoke remediation and final green closeout
**Milestone:** M034

## Description

After the local installed-build repair is green, rerun the approved hosted release lane and refresh the authoritative stop-after `remote-evidence` bundle. This task is the hosted handoff. It must not mutate GitHub state until the user explicitly confirms the outward action, and it must keep any remaining hosted failure attributable by preserving fresh run IDs, head SHAs, and downloaded diagnostics.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Hosted `release.yml` on `v0.1.0` | Stop with fresh diagnostics and keep the task red until the approved hosted lane is actually green. | Preserve the latest run ID, failing job, and downloaded diagnostics; do not infer success from local state. | Treat missing `headSha`, wrong ref, or stale `remote-runs.json` data as evidence drift that invalidates the rerun. |
| `scripts/verify-m034-s05.sh` stop-after remote-evidence replay | Fail closed on the first red hosted phase and keep the fresh verify root. | Keep the run blocking and authoritative. | Treat malformed `remote-runs.json`, phase markers, or diag-download bundles as verification drift. |
| User-confirmed outward action | Do nothing outward until confirmation is explicit. | Keep the task pending instead of mutating refs or workflows implicitly. | Treat ambiguous approval or stale approved ref info as a blocker. |

## Load Profile

- **Shared resources**: GitHub-hosted workflow capacity, `.tmp/m034-s05/verify/remote-runs.json`, and downloaded release-smoke diagnostics.
- **Per-operation cost**: one hosted rerun plus one stop-after remote-evidence replay and optional artifact download.
- **10x breakpoint**: repeated reruns without refreshed summaries make it easy to confuse stale green history with current rollout truth, so each run must rewrite the remote-evidence artifacts atomically.

## Negative Tests

- **Malformed inputs**: wrong or missing approved ref, stale run ID, missing downloaded diagnostics artifact, or `remote-runs.json` that points at the wrong head SHA.
- **Error paths**: hosted release smoke still fails, stop-after remote-evidence replay stays red, or approval is not granted.
- **Boundary conditions**: release lane goes green immediately, release lane stays red with a new diagnostics bundle, and release lane is green but on the wrong ref/head SHA.

## Steps

1. Summarize the exact hosted action to take, the approved `v0.1.0` ref it will touch, and the local repair proof it depends on; get explicit user confirmation before any outward mutation.
2. After approval, rerun the hosted release lane on the approved ref and preserve the returned run ID(s) and job URLs in `.tmp/m034-s12/t03/hosted-rollout-summary.json`.
3. Refresh `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh` from a clean verify root so `.tmp/m034-s05/verify/remote-runs.json` becomes the canonical hosted state.
4. If the hosted lane is still red, download the fresh Windows diagnostics, record the unpacked file list in `.tmp/m034-s12/t03/diag-download-manifest.json`, and preserve the refreshed `07-hello-build` log instead of guessing at the remaining blocker.

## Must-Haves

- [ ] No GitHub mutation happens without explicit user confirmation.
- [ ] Hosted release-smoke status is refreshed in `.tmp/m034-s05/verify/remote-runs.json` for the approved ref/head SHA.
- [ ] A fresh hosted-rollout summary records the run ID, ref, head SHA, and outcome.
- [ ] If hosted release smoke is still red, the new diagnostics bundle is preserved locally for the next loop.

## Verification

- `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh`
- `test -s .tmp/m034-s12/t03/hosted-rollout-summary.json`

## Observability Impact

- Signals added/changed: refreshed hosted run IDs, head SHAs, remote-evidence phase markers, and optional downloaded Windows diagnostics.
- How a future agent inspects this: read `.tmp/m034-s05/verify/remote-runs.json`, `.tmp/m034-s05/verify/phase-report.txt`, and `.tmp/m034-s12/t03/hosted-rollout-summary.json` before deciding whether closeout can proceed.
- Failure state exposed: any remaining hosted blocker stays attributable to a concrete run and artifact bundle instead of to stale local assumptions.

## Inputs

- `scripts/verify-m034-s05.sh` — canonical stop-after remote-evidence verifier.
- `scripts/verify-m034-s06-remote-evidence.sh` — closeout archive helper that T04 will use after T03 succeeds.
- `.github/workflows/release.yml` — hosted lane being rerun.
- `.tmp/m034-s12/t02/local-repair-summary.json` — local repair proof that justifies the hosted rerun.
- `.tmp/m034-s11/t03/diag-download/windows/verify/run/07-hello-build.log` — prior hosted blocker surface to compare against if the lane stays red.

## Expected Output

- `.tmp/m034-s05/verify/remote-runs.json` — refreshed hosted workflow status on the approved ref.
- `.tmp/m034-s12/t03/hosted-rollout-summary.json` — local summary of the hosted rerun, run IDs, head SHA, and outcome.
- `.tmp/m034-s12/t03/diag-download-manifest.json` — manifest describing whether fresh diagnostics were downloaded and which files were unpacked.
- `.tmp/m034-s12/t03/diag-download/windows/verify/run/07-hello-build.log` — refreshed hosted crash log when the release lane remains red.
