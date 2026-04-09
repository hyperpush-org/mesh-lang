---
id: T02
parent: S04
milestone: M042
provides: []
requires: []
affects: []
key_files: ["scripts/lib/m042_cluster_proof.sh", "scripts/verify-m042-s04.sh", "scripts/verify-m042-s04-fly.sh", ".gsd/milestones/M042/slices/S04/tasks/T02-SUMMARY.md"]
key_decisions: ["Reused the M039 phase/assert helper library for status tracking and legacy membership/work checks, and isolated the M042 keyed submit/status contract in a dedicated helper library instead of forking the old assertions.", "Kept the default `verify-m042-s04.sh` path fail-closed on the upstream S03 prerequisite, but added explicit debug-only skip envs so the new packaged phases can still be isolated when inherited M039/M042 rails are already red."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the required slice-level verification commands under a timed harness and wrote exact results to `.tmp/m042-s04/task-t02-verification/summary.json`. `bash scripts/verify-m042-s04-fly.sh --help` passed. `bash scripts/verify-m039-s04.sh` and `bash scripts/verify-m042-s04.sh` both failed closed against inherited upstream regressions before this task could claim a green packaged proof. Additional debug reruns with `M042_S04_SKIP_S03=1` and `M042_S04_SKIP_LEGACY_WORK=1` confirmed that the new packaged wrapper reaches the Docker/keyed phases and archives artifacts, but the underlying cluster continuity rails are still red."
completed_at: 2026-03-29T02:05:14.378Z
blocker_discovered: true
---

# T02: Added the M042 packaged operator verifier scripts and read-only Fly help rail, but authoritative local replay is still blocked by inherited M039/M042 verifier regressions.

> Added the M042 packaged operator verifier scripts and read-only Fly help rail, but authoritative local replay is still blocked by inherited M039/M042 verifier regressions.

## What Happened
---
id: T02
parent: S04
milestone: M042
key_files:
  - scripts/lib/m042_cluster_proof.sh
  - scripts/verify-m042-s04.sh
  - scripts/verify-m042-s04-fly.sh
  - .gsd/milestones/M042/slices/S04/tasks/T02-SUMMARY.md
key_decisions:
  - Reused the M039 phase/assert helper library for status tracking and legacy membership/work checks, and isolated the M042 keyed submit/status contract in a dedicated helper library instead of forking the old assertions.
  - Kept the default `verify-m042-s04.sh` path fail-closed on the upstream S03 prerequisite, but added explicit debug-only skip envs so the new packaged phases can still be isolated when inherited M039/M042 rails are already red.
duration: ""
verification_result: mixed
completed_at: 2026-03-29T02:05:14.386Z
blocker_discovered: true
---

# T02: Added the M042 packaged operator verifier scripts and read-only Fly help rail, but authoritative local replay is still blocked by inherited M039/M042 verifier regressions.

**Added the M042 packaged operator verifier scripts and read-only Fly help rail, but authoritative local replay is still blocked by inherited M039/M042 verifier regressions.**

## What Happened

Added `scripts/lib/m042_cluster_proof.sh` with keyed HTTP/JSON helpers, `scripts/verify-m042-s04.sh` for the packaged Docker/operator rail, and `scripts/verify-m042-s04-fly.sh` for the read-only Fly contract. The local wrapper now creates its own `.tmp/m042-s04/verify/` bundle, replays the M042 S03 authority by default, builds the repo-root image, brings up the one-image two-container cluster, and archives phase artifacts. I also added explicit debug-only skip envs to isolate the packaged phases when inherited prerequisite rails are already red. Local reality diverged from the task plan’s assumption that the old rails were green: the preserved M039 baseline still fails in the legacy remote-route/runtime-crash path, `verify-m042-s03.sh` is unstable in the current checkout, and the isolated packaged keyed run still hits `replica_required_unavailable` on remote-owner search after Docker bring-up. The wrapper and helper code shipped, but the authoritative end-to-end proof is still blocked by those inherited regressions.

## Verification

Ran the required slice-level verification commands under a timed harness and wrote exact results to `.tmp/m042-s04/task-t02-verification/summary.json`. `bash scripts/verify-m042-s04-fly.sh --help` passed. `bash scripts/verify-m039-s04.sh` and `bash scripts/verify-m042-s04.sh` both failed closed against inherited upstream regressions before this task could claim a green packaged proof. Additional debug reruns with `M042_S04_SKIP_S03=1` and `M042_S04_SKIP_LEGACY_WORK=1` confirmed that the new packaged wrapper reaches the Docker/keyed phases and archives artifacts, but the underlying cluster continuity rails are still red.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash scripts/verify-m039-s04.sh` | 1 | ❌ fail | 134790ms |
| 2 | `bash scripts/verify-m042-s04.sh` | 1 | ❌ fail | 147989ms |
| 3 | `bash scripts/verify-m042-s04-fly.sh --help` | 0 | ✅ pass | 60ms |


## Deviations

Added debug-only skip envs (`M042_S04_SKIP_S03=1` and `M042_S04_SKIP_LEGACY_WORK=1`) to `verify-m042-s04.sh` so the new packaged phases can be isolated when inherited prerequisite rails are already failing. The default path still fail-closes on the upstream prerequisites.

## Known Issues

`bash scripts/verify-m039-s04.sh` still fails in the inherited `e2e_m039_s02_routes_work_to_peer_and_logs_execution` path, where remote `/work` routing regresses and the peer crashes in `compiler/mesh-rt/src/string.rs:104:21`. `bash scripts/verify-m042-s03.sh` is not stable in the current checkout, so the default `bash scripts/verify-m042-s04.sh` path fails in its S03 prerequisite replay. Even with the debug skips enabled, the packaged keyed phase still hits `503 replica_required_unavailable` during remote-owner search after Docker bring-up.

## Files Created/Modified

- `scripts/lib/m042_cluster_proof.sh`
- `scripts/verify-m042-s04.sh`
- `scripts/verify-m042-s04-fly.sh`
- `.gsd/milestones/M042/slices/S04/tasks/T02-SUMMARY.md`


## Deviations
Added debug-only skip envs (`M042_S04_SKIP_S03=1` and `M042_S04_SKIP_LEGACY_WORK=1`) to `verify-m042-s04.sh` so the new packaged phases can be isolated when inherited prerequisite rails are already failing. The default path still fail-closes on the upstream prerequisites.

## Known Issues
`bash scripts/verify-m039-s04.sh` still fails in the inherited `e2e_m039_s02_routes_work_to_peer_and_logs_execution` path, where remote `/work` routing regresses and the peer crashes in `compiler/mesh-rt/src/string.rs:104:21`. `bash scripts/verify-m042-s03.sh` is not stable in the current checkout, so the default `bash scripts/verify-m042-s04.sh` path fails in its S03 prerequisite replay. Even with the debug skips enabled, the packaged keyed phase still hits `503 replica_required_unavailable` during remote-owner search after Docker bring-up.
