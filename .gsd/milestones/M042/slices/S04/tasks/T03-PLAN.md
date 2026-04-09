---
estimated_steps: 4
estimated_files: 8
skills_used: []
---

# T03: Retire the inherited continuity-proof regressions and make the packaged Docker/operator replay authoritative.

1. Reproduce the three blocker paths separately: the inherited M039 remote `/work` routing crash, the unstable `verify-m042-s03.sh` replay, and the packaged Docker keyed phase returning `503 replica_required_unavailable` after bring-up.
2. Trace those failures to their real shared seam (runtime continuity admission/replica truth, cluster-proof placement/submit wiring, or verifier assumptions) and fix the root causes without widening the Mesh-facing `Continuity.*` contract or rewriting M039 history.
3. Tighten the M039/M042 helper and wrapper scripts only where the truthful contract changed or an old prerequisite assumption was wrong; keep fail-closed phase, zero-test, and malformed-artifact behavior.
4. End with a green local replay that proves packaged keyed continuity through the one-image two-container rail and preserves the read-only Fly wrapper/help contract.

## Inputs

- `.gsd/milestones/M042/slices/S04/tasks/T02-SUMMARY.md`
- `.tmp/m042-s04/task-t02-verification/summary.json`
- `scripts/lib/m039_cluster_proof.sh`
- `scripts/lib/m042_cluster_proof.sh`
- `scripts/verify-m039-s04.sh`
- `scripts/verify-m042-s03.sh`
- `scripts/verify-m042-s04.sh`

## Expected Output

- `Green `scripts/verify-m039-s04.sh`, `scripts/verify-m042-s03.sh`, and `scripts/verify-m042-s04.sh` replays with copied artifact bundles under `.tmp/m039-s04/verify/` and `.tmp/m042-s04/verify/``
- `A stable packaged keyed continuity proof that no longer reports `replica_required_unavailable` after healthy two-container bring-up`
- `An unchanged read-only `scripts/verify-m042-s04-fly.sh --help` contract that still truthfully scopes the Fly lane`

## Verification

bash scripts/verify-m039-s04.sh && bash scripts/verify-m042-s03.sh && bash scripts/verify-m042-s04.sh && bash scripts/verify-m042-s04-fly.sh --help
