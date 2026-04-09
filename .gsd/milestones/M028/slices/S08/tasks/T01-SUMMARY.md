---
id: T01
parent: S08
milestone: M028
provides:
  - Recovery-aware public proof contract for the reference backend surface
key_files:
  - reference-backend/README.md
  - website/docs/docs/production-backend-proof/index.md
  - reference-backend/scripts/verify-production-proof-surface.sh
key_decisions:
  - Guard public proof drift with exact string checks over the shared S07 command list and recovery-field vocabulary.
patterns_established:
  - Public proof docs and the verifier must promote the same named recovery-aware commands, not overlapping but divergent summaries.
observability_surfaces:
  - reference-backend/scripts/verify-production-proof-surface.sh
  - reference-backend/README.md
  - website/docs/docs/production-backend-proof/index.md
duration: 39m
verification_result: passed
completed_at: 2026-03-24T00:03:31-04:00
blocker_discovered: false
---

# T01: Reconcile the public runbook and proof guard

**Aligned the reference-backend runbook, proof page, and verifier on the green recovery-aware S07 command set.**

## What Happened

I expanded `reference-backend/README.md` with an explicit **Supervision and recovery** runbook section that now names the authoritative S07 proof commands, explains the recovery-relevant `/health` fields (`restart_count`, `last_exit_reason`, `recovered_jobs`, `last_recovery_*`, `recovery_active`), and distinguishes worker-crash recovery from whole-process restart recovery.

I then rewrote `website/docs/docs/production-backend-proof/index.md` so the public proof page promotes the same canonical command list instead of a narrower subset, while still staying shorter than the README. The page now calls out migration truth, deploy smoke, worker-crash recovery, restart visibility, and whole-process restart recovery explicitly.

Finally, I strengthened `reference-backend/scripts/verify-production-proof-surface.sh` so it no longer just checks routing links and stale phrases. It now enforces the recovery runbook wording, the recovery signal vocabulary, and the shared authoritative command list across the README and public proof page. That turns public proof drift into a named verifier failure instead of a manual-reading problem.

## Verification

Task-level verification passed:
- `bash reference-backend/scripts/verify-production-proof-surface.sh`
- `rg -n "Supervision and recovery|restart_count|last_exit_reason|recovered_jobs|process restart" reference-backend/README.md`
- `rg -n "e2e_reference_backend_worker_crash_recovers_job|e2e_reference_backend_worker_restart_is_visible_in_health|e2e_reference_backend_process_restart_recovers_inflight_job|e2e_reference_backend_migration_status_and_apply|e2e_reference_backend_deploy_artifact_smoke" website/docs/docs/production-backend-proof/index.md reference-backend/scripts/verify-production-proof-surface.sh`

Because this task changed public docs, I also ran the relevant website slice checks and confirmed they pass:
- `npm --prefix website ci`
- `npm --prefix website run build`

The heavier slice-level backend/runtime commands remain for later slice tasks; this is an intermediate task, so only partial slice verification is expected at this point.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash reference-backend/scripts/verify-production-proof-surface.sh` | 0 | ✅ pass | 1.30s |
| 2 | `rg -n "Supervision and recovery|restart_count|last_exit_reason|recovered_jobs|process restart" reference-backend/README.md` | 0 | ✅ pass | 0.03s |
| 3 | `rg -n "e2e_reference_backend_worker_crash_recovers_job|e2e_reference_backend_worker_restart_is_visible_in_health|e2e_reference_backend_process_restart_recovers_inflight_job|e2e_reference_backend_migration_status_and_apply|e2e_reference_backend_deploy_artifact_smoke" website/docs/docs/production-backend-proof/index.md reference-backend/scripts/verify-production-proof-surface.sh` | 0 | ✅ pass | 0.03s |
| 4 | `npm --prefix website ci` | 0 | ✅ pass | 18.54s |
| 5 | `npm --prefix website run build` | 0 | ✅ pass | 36.86s |

## Diagnostics

Future agents can inspect this work by rerunning `bash reference-backend/scripts/verify-production-proof-surface.sh`. If it fails, the error now points at the missing proof command or missing recovery wording directly.

For human inspection:
- `reference-backend/README.md` contains the authoritative runbook under `## Supervision and recovery`.
- `website/docs/docs/production-backend-proof/index.md` contains the promoted public proof list.
- `reference-backend/scripts/verify-production-proof-surface.sh` contains the canonical command array and recovery wording checks.

## Deviations

I ran `npm --prefix website ci` and `npm --prefix website run build` during T01 even though the task plan only required the proof-surface verifier and targeted `rg` checks, because this task changed promoted website docs and the slice verification list includes website build health.

## Known Issues

`npm --prefix website run build` still emits the existing Vite/Rollup chunk-size warning for the docs site, but the build exits 0 and this task did not change bundling behavior.

## Files Created/Modified

- `reference-backend/README.md` — replaced the stale proof-target tail with a recovery-aware supervision runbook and the canonical public S07 command list.
- `website/docs/docs/production-backend-proof/index.md` — aligned the public proof page to the same migration/deploy/recovery command set and `/health` interpretation.
- `reference-backend/scripts/verify-production-proof-surface.sh` — upgraded the verifier to enforce recovery wording and exact shared proof commands across the public surfaces.
- `.gsd/milestones/M028/slices/S08/S08-PLAN.md` — marked T01 complete.
- `.gsd/milestones/M028/slices/S08/tasks/T01-SUMMARY.md` — recorded execution narrative and verification evidence.
