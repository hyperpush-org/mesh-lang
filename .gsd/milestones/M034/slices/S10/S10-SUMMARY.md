---
id: S10
parent: M034
milestone: M034
provides:
  - Monotonic registry latest-version truth for metadata/search callers, target-aware Windows MSVC runtime/linker selection in `mesh-codegen`, and refreshed hosted evidence showing the release tag lane as the sole remaining blocker.
requires:
  - slice: S09
    provides: the approved rollout-target mechanism plus freshness-aware remote-evidence surfaces anchored to one concrete SHA
affects:
  - S11
key_files:
  - registry/src/db/packages.rs
  - registry/src/routes/metadata.rs
  - registry/src/routes/search.rs
  - compiler/mesh-codegen/src/link.rs
  - compiler/mesh-codegen/src/lib.rs
  - scripts/verify-m034-s03.ps1
  - .tmp/m034-s05/verify/remote-runs.json
  - .tmp/m034-s09/rollout/workflow-status.json
  - .gsd/PROJECT.md
key_decisions:
  - D104: Recompute `packages.latest_version` from committed `versions` rows under a per-package row lock so package-level latest cannot move backward under overlapping publishes.
  - D105: Use target-aware linker selection and pass the resolved runtime static library path directly to the linker; keep Unix on `cc` + `libmesh_rt.a` and use `clang(.exe)` + `mesh_rt.lib` on Windows MSVC.
  - D106: Prefer `gh run rerun --failed` on the existing correct-head workflow run before dispatching a new run, so hosted blocker evidence stays attributable to the already-approved rollout target.
patterns_established:
  - Derive mutable package latest pointers from committed version rows instead of request order when concurrent publishes can race.
  - Treat hosted workflow freshness (`headSha` matches target) as a separate signal from hosted workflow health so a red run on the correct SHA stays attributable instead of being misread as staleness.
  - When platform-specific smoke fails, keep the verifier’s phase logs structured enough to point directly at the failing command and artifact paths.
observability_surfaces:
  - `.tmp/m034-s05/verify/remote-runs.json` now records expected vs observed `headSha` plus per-workflow status so stale-green runs cannot masquerade as current proof.
  - `.tmp/m034-s09/rollout/workflow-status.json` provides the concise authoritative-success / release-failure view for the current rollout target.
  - `scripts/verify-m034-s03.ps1` command logs now record display text, exit code, and stdout/stderr artifact paths for staged Windows smoke commands.
  - `.tmp/m034-s03/windows/verify/run/07-hello-build.log` remains the first trustworthy local diagnostic for Windows installed-compiler regressions.
drill_down_paths:
  - .gsd/milestones/M034/slices/S10/tasks/T01-SUMMARY.md
  - .gsd/milestones/M034/slices/S10/tasks/T02-SUMMARY.md
  - .gsd/milestones/M034/slices/S10/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-27T21:33:13.228Z
blocker_discovered: false
---

# S10: Hosted verification blocker remediation

**Registry latest-version ordering and Windows MSVC runtime linking were repaired locally, and the refreshed hosted evidence now shows `authoritative-verification.yml` green while the release tag lane remains the only live blocker.**

## What Happened

S10 retired two real local blockers instead of papering over them. First, the registry stopped trusting request/update order for `packages.latest_version`: publish now re-derives package latest from committed `versions` rows under a per-package lock, while metadata and search/list fail closed if the latest-version join goes missing. That moved the source of truth to the data that actually commits and gave the package-manager proof path monotonic latest semantics even under overlapping publishes.

Second, `mesh-codegen` stopped assuming Unix linker/runtime rules everywhere. The linker path is now target-aware, preserving direct `libmesh_rt.a` linking on Unix-like hosts and switching Windows MSVC to explicit `mesh_rt.lib` discovery plus `clang(.exe)`. The staged PowerShell verifier was tightened at the same time so hosted Windows failures retain command text, exit code, and stdout/stderr artifact paths instead of collapsing into a thin phase failure.

The slice did not fully clear the hosted blocker the roadmap hoped to retire, but it did make that blocker honest. Focused local verification passed for the registry race repair, the linker/runtime fix, the staged PowerShell helper, and the release-workflow contract. Then the remote-evidence refresh was replayed against the current rollout target `e59f18203a30951af5288791bf9aed5b53a24a2a` with expected-failure wrappers that only pass if the blocker shape is exact. The resulting state is now precise: `authoritative-verification.yml` is green on the rollout SHA, `release.yml` is still red on that same SHA, and the remaining failure is concentrated in the Windows staged installer smoke path rather than in registry latest drift or stale workflow freshness.

## Verification

Passed the local slice verifiers directly: `DATABASE_URL='postgres://postgres:postgres@127.0.0.1:55433/postgres' cargo test --manifest-path registry/Cargo.toml latest -- --nocapture`, `bash scripts/tests/verify-m034-s01-fetch-retry.sh`, `cargo test -p mesh-codegen link -- --nocapture`, `pwsh -NoProfile -File scripts/tests/verify-m034-s03-last-exitcode.ps1`, and `bash scripts/verify-m034-s02-workflows.sh` all succeeded. Then refreshed the hosted evidence with expected-failure wrappers that only pass if the blocker shape is exact: `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh` rebuilt `.tmp/m034-s05/verify/remote-runs.json` and confirmed `authoritative-verification.yml` is `ok` while `release.yml` is `failed` on rollout SHA `e59f18203a30951af5288791bf9aed5b53a24a2a`; `python3 .tmp/m034-s09/rollout/monitor_workflows.py` rebuilt `.tmp/m034-s09/rollout/workflow-status.json` and confirmed the same authoritative-success / release-failure split on that same SHA.

## Requirements Advanced

- R007 — S10 hardened the real registry publish/install trust path by making package-level `latest` monotonic across overlapping publishes and by keeping metadata/search aligned with committed version data instead of last-writer-wins package state.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

T03 invalidated the slice’s original closure assumption. The local code repairs landed and held under focused verification, but the hosted refresh did not converge to both lanes green. Instead, the refreshed evidence proved a narrower state: `authoritative-verification.yml` recovered, while `release.yml` remained red on the rollout target. By closeout, the active rollout target had also moved to the workflow-only synthetic commit `e59f18203a30951af5288791bf9aed5b53a24a2a`, so the final slice verification had to refresh remote evidence against that newer target instead of the earlier `8e6d49dacc4f4cd64824b032078ae45aabfe9635` state recorded in the task narrative.

## Known Limitations

`release.yml` is still red on the current rollout target `e59f18203a30951af5288791bf9aed5b53a24a2a`, specifically at the Windows staged installer smoke step (`installed meshc.exe build installer smoke fixture failed`). The current remote target is only a workflow-only synthetic commit, so a green `authoritative-verification.yml` run there is not enough evidence that the full local S10 code surface has been exercised on the release tag lane. `.tmp/m034-s06/evidence/first-green/` must remain absent until both lanes are green on the same target SHA.

## Follow-ups

1. Roll the full local S10 code surface onto the release tag lane instead of trusting the workflow-only synthetic commit evidence.
2. Re-run `release.yml` on the current rollout target and inspect the Windows staged installer smoke diagnostics first, especially `.tmp/m034-s03/windows/verify/run/07-hello-build.log` and the hosted failed-job log under `.tmp/m034-s10/release-artifact-rerun/` or the latest release failure bundle.
3. After the release lane is green on the same `headSha` as authoritative verification, rerun `VERIFY_M034_S05_STOP_AFTER=remote-evidence bash scripts/verify-m034-s05.sh` and capture `.tmp/m034-s06/evidence/first-green/` exactly once for S11.

## Files Created/Modified

- `registry/src/db/packages.rs` — Replaced last-writer-wins `latest_version` updates with monotonic recomputation from committed `versions` rows under a per-package lock.
- `registry/src/routes/metadata.rs` — Made package metadata fail closed when the latest-version join is missing and added latest-version regression coverage.
- `registry/src/routes/search.rs` — Aligned package list/search latest-version semantics with metadata and added malformed-latest join regression coverage.
- `compiler/mesh-codegen/src/link.rs` — Added target-aware linker/runtime discovery so Unix keeps direct `libmesh_rt.a` linking while Windows MSVC resolves `mesh_rt.lib` and `clang(.exe)` explicitly.
- `compiler/mesh-codegen/src/lib.rs` — Threaded the requested target triple into the codegen link path so `meshc build` uses the repaired linker behavior.
- `scripts/verify-m034-s03.ps1` — Persisted PowerShell staged-smoke command logs with display text, exit code, and stdout/stderr artifact paths so hosted Windows failures stay attributable.
- `.tmp/m034-s05/verify/remote-runs.json` — Refreshed canonical hosted-workflow evidence against the current rollout target and recorded expected-vs-observed `headSha` values for each lane.
- `.tmp/m034-s09/rollout/workflow-status.json` — Refreshed rollout workflow status so the authoritative-success / release-failure split is visible on the current target SHA.
