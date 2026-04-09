---
id: T01
parent: S04
milestone: M042
provides: []
requires: []
affects: []
key_files: ["cluster-proof/work.mpl", "cluster-proof/work_legacy.mpl", "cluster-proof/work_continuity.mpl", "cluster-proof/main.mpl", "cluster-proof/tests/work.test.mpl", ".gsd/milestones/M042/slices/S04/tasks/T01-SUMMARY.md"]
key_decisions: ["Kept placement, node identity, and request-key validation in shared `Work`, while moving legacy probe HTTP behavior and runtime continuity HTTP adaptation into separate modules.", "Preserved the existing runtime-owned `Continuity.submit/status/mark_completed` contract and log/status mapping instead of introducing any Mesh-side continuity state machine."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-local verification passed with `cargo run -q -p meshc -- test cluster-proof/tests` and `cargo run -q -p meshc -- build cluster-proof`. Slice-level characterization also showed `npm --prefix website run build` passing, while `bash scripts/verify-m039-s04.sh` still fails in the inherited remote-routing/runtime-crash path and the M042 verifier scripts are not present yet, so those checks fail closed as expected for later tasks."
completed_at: 2026-03-29T01:31:06.575Z
blocker_discovered: false
---

# T01: Split `cluster-proof` into shared placement, legacy probe, and runtime continuity modules without changing the keyed submit/status contract.

> Split `cluster-proof` into shared placement, legacy probe, and runtime continuity modules without changing the keyed submit/status contract.

## What Happened
---
id: T01
parent: S04
milestone: M042
key_files:
  - cluster-proof/work.mpl
  - cluster-proof/work_legacy.mpl
  - cluster-proof/work_continuity.mpl
  - cluster-proof/main.mpl
  - cluster-proof/tests/work.test.mpl
  - .gsd/milestones/M042/slices/S04/tasks/T01-SUMMARY.md
key_decisions:
  - Kept placement, node identity, and request-key validation in shared `Work`, while moving legacy probe HTTP behavior and runtime continuity HTTP adaptation into separate modules.
  - Preserved the existing runtime-owned `Continuity.submit/status/mark_completed` contract and log/status mapping instead of introducing any Mesh-side continuity state machine.
duration: ""
verification_result: mixed
completed_at: 2026-03-29T01:31:06.578Z
blocker_discovered: false
---

# T01: Split `cluster-proof` into shared placement, legacy probe, and runtime continuity modules without changing the keyed submit/status contract.

**Split `cluster-proof` into shared placement, legacy probe, and runtime continuity modules without changing the keyed submit/status contract.**

## What Happened

Refactored `cluster-proof` so the mixed `work.mpl` module became a shared placement/type seam, moved the legacy `GET /work` probe into `work_legacy.mpl`, and moved the keyed submit/status runtime continuity adapter into `work_continuity.mpl`. Updated `main.mpl` to wire the legacy and keyed routes explicitly from separate modules, and updated `cluster-proof/tests/work.test.mpl` so placement helpers, legacy target-selection behavior, and keyed submit/status helpers are exercised through their split seams. Kept the runtime-owned continuity behavior unchanged: no new Mesh-side continuity state machine, dedupe layer, or recovery shim was introduced.

## Verification

Task-local verification passed with `cargo run -q -p meshc -- test cluster-proof/tests` and `cargo run -q -p meshc -- build cluster-proof`. Slice-level characterization also showed `npm --prefix website run build` passing, while `bash scripts/verify-m039-s04.sh` still fails in the inherited remote-routing/runtime-crash path and the M042 verifier scripts are not present yet, so those checks fail closed as expected for later tasks.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 10572ms |
| 2 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 6990ms |
| 3 | `bash scripts/verify-m039-s04.sh` | 1 | ❌ fail | 183351ms |
| 4 | `bash scripts/verify-m042-s04.sh` | 127 | ❌ fail | 42ms |
| 5 | `bash scripts/verify-m042-s04-fly.sh --help` | 127 | ❌ fail | 48ms |
| 6 | `bash scripts/verify-m042-s04-proof-surface.sh` | 127 | ❌ fail | 22ms |
| 7 | `npm --prefix website run build` | 0 | ✅ pass | 22201ms |


## Deviations

Used the repo’s existing snake_case Mesh file-path convention (`work_legacy.mpl`, `work_continuity.mpl`) instead of literal CamelCase filenames from the task plan while keeping the imported module names `WorkLegacy` and `WorkContinuity`.

## Known Issues

`bash scripts/verify-m039-s04.sh` still fails in the inherited `e2e_m039_s02_routes_work_to_peer_and_logs_execution` path, where the route response does not report remote routing and the peer crashes with `null pointer dereference occurred` at `compiler/mesh-rt/src/string.rs:104:21`. The M042 verifier scripts referenced by the slice plan do not exist yet and fail closed with `No such file or directory`; those are later-task gaps, not regressions from this refactor.

## Files Created/Modified

- `cluster-proof/work.mpl`
- `cluster-proof/work_legacy.mpl`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/main.mpl`
- `cluster-proof/tests/work.test.mpl`
- `.gsd/milestones/M042/slices/S04/tasks/T01-SUMMARY.md`


## Deviations
Used the repo’s existing snake_case Mesh file-path convention (`work_legacy.mpl`, `work_continuity.mpl`) instead of literal CamelCase filenames from the task plan while keeping the imported module names `WorkLegacy` and `WorkContinuity`.

## Known Issues
`bash scripts/verify-m039-s04.sh` still fails in the inherited `e2e_m039_s02_routes_work_to_peer_and_logs_execution` path, where the route response does not report remote routing and the peer crashes with `null pointer dereference occurred` at `compiler/mesh-rt/src/string.rs:104:21`. The M042 verifier scripts referenced by the slice plan do not exist yet and fail closed with `No such file or directory`; those are later-task gaps, not regressions from this refactor.
