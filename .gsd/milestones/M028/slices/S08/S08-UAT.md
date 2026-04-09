# S08 UAT — Final Proof Surface Reconciliation

**Milestone:** M028  
**Slice:** S08  
**Scope:** verify that the promoted README/docs/UAT/validation/requirements surfaces all point at the same green recovery-aware `reference-backend/` proof path and that the proof path reruns cleanly in this worktree.

## Preconditions
1. Run from repo root: `/Users/sn0w/Documents/dev/mesh-lang/.gsd/worktrees/M028`
2. `.env` contains a working `DATABASE_URL`
3. Postgres is reachable from this worktree
4. No stale long-lived `reference-backend` process is occupying harness-selected ports
5. Run website install/build serially, not in parallel:
   - `npm --prefix website ci`
   - `npm --prefix website run build`
6. Use the repo-root `.env` for ignored compiler e2e commands:
   - `set -a && source .env && set +a`

## Test Case 1 — Public proof surfaces advertise the same recovery-aware contract

### Goal
Prove that the package README, public proof page, and proof-surface verifier all describe the same canonical recovery-aware backend proof path.

### Steps
1. Run:
   - `bash reference-backend/scripts/verify-production-proof-surface.sh`
2. Inspect the public surfaces:
   - `reference-backend/README.md`
   - `website/docs/docs/production-backend-proof/index.md`

### Expected outcomes
1. The verifier exits `0`.
2. `reference-backend/README.md` contains a `## Supervision and recovery` runbook section.
3. The README and proof page both name the same authoritative runtime proof commands in substance:
   - `e2e_reference_backend_worker_crash_recovers_job`
   - `e2e_reference_backend_worker_restart_is_visible_in_health`
   - `e2e_reference_backend_process_restart_recovers_inflight_job`
   - `e2e_reference_backend_migration_status_and_apply`
   - `e2e_reference_backend_deploy_artifact_smoke`
4. The public surfaces call out the recovery fields evaluators should read from `/health`, including `restart_count`, `last_exit_reason`, `recovered_jobs`, `last_recovery_at`, and `recovery_active`.
5. The proof page routes readers back to the real package runbook rather than inventing a second independent command story.

## Test Case 2 — Website build still promotes the canonical proof page

### Goal
Prove that the promoted docs surface is buildable and that the production-backend proof page remains part of the published site.

### Steps
1. Run:
   - `npm --prefix website ci`
2. Run:
   - `npm --prefix website run build`

### Expected outcomes
1. Both commands exit `0`.
2. The build completes without proof-surface routing or content errors.
3. The only acceptable website-build noise is the existing large-chunk warning; it must not block the build.
4. The production proof page remains part of the built site and is still the canonical public backend proof route.

## Test Case 3 — The full green recovery-aware backend proof set still passes

### Goal
Prove that S08 is not a docs-only paper-over by rerunning the same green `reference-backend/` command set the promoted surfaces point at.

### Steps
1. Run baseline backend checks:
   - `cargo run -p meshc -- build reference-backend`
   - `cargo run -p meshc -- fmt --check reference-backend`
   - `cargo run -p meshc -- test reference-backend`
2. Run the ignored compiler e2e recovery/deploy proofs:
   - `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_crash_recovers_job -- --ignored --nocapture`
   - `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_restart_is_visible_in_health -- --ignored --nocapture`
   - `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_process_restart_recovers_inflight_job -- --ignored --nocapture`
   - `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_migration_status_and_apply -- --ignored --nocapture`
   - `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_deploy_artifact_smoke -- --ignored --nocapture`

### Expected outcomes
1. Every command exits `0`.
2. The recovery proofs still observe the intended degraded/recovering behavior before final healthy settlement.
3. `/health` restart metadata remains coherent across the proofs:
   - `restart_count` increments on restart
   - `last_exit_reason` is populated
   - `recovered_jobs` reflects reclaimed work
   - `recovery_active` is true during recovery and false after final settlement
4. The migration proof still succeeds against the canonical Mesh migration source.
5. The deploy smoke proof still succeeds against the staged artifact path.

## Test Case 4 — Internal closure artifacts no longer contradict the green proof surface

### Goal
Prove that stale S05/S06/M028 closure surfaces no longer contain red-era wording or placeholder content.

### Steps
1. Run:
   - `! rg -n "placeholder|partial / not done|current blocker|needs-remediation|R004.*still open|R009.*still open|replace this placeholder" .gsd/milestones/M028/M028-VALIDATION.md .gsd/milestones/M028/slices/S05/S05-SUMMARY.md .gsd/milestones/M028/slices/S05/S05-UAT.md .gsd/milestones/M028/slices/S06/S06-SUMMARY.md .gsd/milestones/M028/slices/S06/S06-UAT.md`
2. Inspect the reconciled closure artifacts:
   - `.gsd/milestones/M028/slices/S05/S05-SUMMARY.md`
   - `.gsd/milestones/M028/slices/S05/S05-UAT.md`
   - `.gsd/milestones/M028/slices/S06/S06-SUMMARY.md`
   - `.gsd/milestones/M028/slices/S06/S06-UAT.md`
   - `.gsd/milestones/M028/M028-VALIDATION.md`

### Expected outcomes
1. The stale-claim sweep exits `0`.
2. S05 and S06 closure artifacts now route acceptance through the green S07 command set rather than through historical partial/debug scripts.
3. `M028-VALIDATION.md` carries `verdict: pass` and describes S08 as the final proof-surface reconciliation slice.
4. None of the inspected artifacts contain placeholder wording or still-red recovery claims.

## Test Case 5 — Requirement truth matches the sealed milestone state

### Goal
Prove that requirement tracking now reflects the reconciled green proof surface rather than a pending closure state.

### Steps
1. Run:
   - `python3 - <<'PY'
from pathlib import Path
req = Path('.gsd/REQUIREMENTS.md').read_text()
val = Path('.gsd/milestones/M028/M028-VALIDATION.md').read_text()
section = req.split('### R008 —', 1)[1].split('\n### ', 1)[0]
assert 'Status: validated' in section, 'R008 is not marked validated'
assert 'Validated by M028/S08' in section, 'R008 validation source is not S08'
assert 'verdict: pass' in val, 'M028 validation verdict is not pass'
print('requirement gate ok')
PY`
2. Inspect:
   - `.gsd/REQUIREMENTS.md`
   - `.gsd/milestones/M028/M028-VALIDATION.md`

### Expected outcomes
1. The Python gate exits `0`.
2. `R008` is marked `validated` and cites S08’s reconciled production-proof surface.
3. `R004` and `R009` remain validated by the runtime proof path rather than being silently reassigned to a docs-only slice.
4. Requirement tracking and milestone validation now tell the same closure story.

## Edge Cases to watch while running the script

### Edge Case A — False website failures caused by parallel execution
If `npm --prefix website run build` reports missing modules like `minisearch` or `@rollup/rollup-darwin-arm64` while `npm ci` is still running, treat that as a harness mistake. Re-run website install and build serially before concluding the slice regressed.

### Edge Case B — Public docs are green but runtime proof is red
If the proof-surface verifier and website build pass while any ignored compiler e2e proof fails, treat that as a real backend regression, not as closure. S08 only passes if both docs and runtime commands are green.

### Edge Case C — Runtime proof is green but closure artifacts still drift
If the backend commands pass but the stale-claim sweep or requirement gate fails, treat that as truth-surface drift. Fix the closure artifacts, then rerun the full gate.

### Edge Case D — R008 is validated but validation text still implies pending work
If `.gsd/REQUIREMENTS.md` says validated but `M028-VALIDATION.md` still implies remediation, the milestone is not sealed honestly. Both surfaces must agree.

## Minimal acceptance checklist
- [ ] proof-surface verifier passes
- [ ] website install passes
- [ ] website build passes
- [ ] `meshc build` passes for `reference-backend`
- [ ] `meshc fmt --check` passes for `reference-backend`
- [ ] `meshc test` passes for `reference-backend`
- [ ] worker-crash recovery proof passes
- [ ] restart-visibility proof passes
- [ ] whole-process restart proof passes
- [ ] migration status/apply proof passes
- [ ] staged deploy artifact smoke passes
- [ ] stale-claim sweep passes
- [ ] `R008` is validated by S08
- [ ] `M028-VALIDATION.md` has `verdict: pass`
- [ ] public proof page, package runbook, verifier, internal closure artifacts, milestone validation, and requirements all point at the same green recovery-aware proof path

## Failure signals
- proof-surface verifier exits non-zero
- website build fails after a serial install/build run
- any ignored `e2e_reference_backend` proof exits non-zero
- stale phrases are found in S05/S06/M028 closure artifacts
- `R008` is not marked validated by S08
- `M028-VALIDATION.md` does not contain `verdict: pass`
- README/proof page/verifier cite different command lists or different `/health` recovery fields
