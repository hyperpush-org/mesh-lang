# Slice Summary — S08: Final Proof Surface Reconciliation

## Status
- **State:** done
- **Roadmap checkbox:** checked
- **Why:** the promoted README/docs/UAT/validation/requirements surfaces now all cite the same green recovery-aware `reference-backend/` proof path, and the full slice verification list reran green in this worktree.

## What this slice actually delivered

### 1. The public proof surface now points at one canonical recovery-aware runbook
S08 finished the last public truth-surface cleanup rather than adding new runtime behavior.

What now agrees publicly:
- `reference-backend/README.md` has an explicit **Supervision and recovery** section
- `website/docs/docs/production-backend-proof/index.md` promotes the same named proof commands instead of a narrower or divergent subset
- `reference-backend/scripts/verify-production-proof-surface.sh` enforces the shared command list and recovery vocabulary mechanically

The public contract now names the same green proof set for:
- worker-crash recovery
- restart visibility in `/health`
- whole-process restart recovery
- migration status/apply truth
- staged deploy artifact smoke

It also promotes the recovery fields evaluators are supposed to read from `/health`, including:
- `restart_count`
- `last_exit_reason`
- `recovered_jobs`
- `last_recovery_at`
- `recovery_active`

### 2. The stale internal closure artifacts were rewritten to inherit S07 instead of contradicting it
S05 and S06 were still dangerous because future readers could land on stale closure artifacts and get a different story than the green S07 proof surface.

S08 rewrote:
- `.gsd/milestones/M028/slices/S05/S05-SUMMARY.md`
- `.gsd/milestones/M028/slices/S05/S05-UAT.md`
- `.gsd/milestones/M028/slices/S06/S06-SUMMARY.md`
- `.gsd/milestones/M028/slices/S06/S06-UAT.md`

What changed in practice:
- S05 is now described as the recovery groundwork slice whose final acceptance is the green S07 proof set
- S06 is now described as an honest production-proof/docs slice that depends on the now-green recovery-aware backend path
- both UAT files now reuse the canonical S07 recovery-aware command order instead of preserving slice-local historical debug scripts
- the stale-claim grep sweep can pass without contradictory wording left in those artifacts

### 3. Milestone validation and requirement truth were sealed against the same evidence
S08 closed the milestone honestly by updating the final closure surfaces only after rerunning the proof set.

The closure surfaces now agree:
- `.gsd/milestones/M028/M028-VALIDATION.md` has `verdict: pass`
- `.gsd/REQUIREMENTS.md` keeps R004 and R009 validated by S07’s runtime proof and records **R008 as validated by S08**
- `.gsd/PROJECT.md` now reflects M028 as a closed baseline milestone rather than an in-progress trust gap
- `.gsd/milestones/M028/M028-ROADMAP.md` now marks S08 complete

### 4. S08 established the final M028 proof-surface pattern
The important pattern is not “more docs.” It is that all promoted surfaces must point at the same named proof commands.

This slice established that:
- public proof drift should fail mechanically through `verify-production-proof-surface.sh`
- internal closure artifacts should inherit the canonical green command list rather than preserve old slice-local scripts
- milestone validation and requirement updates should happen only after a post-edit rerun proves the surfaces and runtime still agree
- the website install/build verification for this slice must be run **serially**, not in parallel, because overlapping `npm ci` and build can produce false missing-module failures while `node_modules` is being replaced

## Verification run by the closer

All slice-level verification passed in this closure run.

### Passing commands
- `bash reference-backend/scripts/verify-production-proof-surface.sh`
- `npm --prefix website ci`
- `npm --prefix website run build`
- `cargo run -p meshc -- build reference-backend`
- `cargo run -p meshc -- fmt --check reference-backend`
- `cargo run -p meshc -- test reference-backend`
- `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_crash_recovers_job -- --ignored --nocapture`
- `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_restart_is_visible_in_health -- --ignored --nocapture`
- `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_process_restart_recovers_inflight_job -- --ignored --nocapture`
- `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_migration_status_and_apply -- --ignored --nocapture`
- `set -a && source .env && set +a && cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_deploy_artifact_smoke -- --ignored --nocapture`
- `! rg -n "placeholder|partial / not done|current blocker|needs-remediation|R004.*still open|R009.*still open|replace this placeholder" .gsd/milestones/M028/M028-VALIDATION.md .gsd/milestones/M028/slices/S05/S05-SUMMARY.md .gsd/milestones/M028/slices/S05/S05-UAT.md .gsd/milestones/M028/slices/S06/S06-SUMMARY.md .gsd/milestones/M028/slices/S06/S06-UAT.md`
- the targeted Python gate asserting `R008` contains `Status: validated` and `M028-VALIDATION.md` contains `verdict: pass`

## Observability / diagnostics confirmed
- `reference-backend/scripts/verify-production-proof-surface.sh` is the fastest drift detector for the public proof hierarchy and passed in this closure run.
- The authoritative runtime recovery signals remain the S07 tests plus `/health` fields such as `restart_count`, `last_exit_reason`, `recovered_jobs`, `last_recovery_at`, and `recovery_active`; those surfaces were exercised again by the green ignored compiler e2e proofs.
- `.gsd/tmp/t03-verification-final/summary.json` and sibling logs remain a compact artifact trail for the post-edit full rerun used to seal milestone closure.

## Requirement impact
- **R008:** validated and now honestly tied to S08’s reconciled production-proof surface.
- **R004:** unchanged in ownership, still validated by S07 runtime recovery proof.
- **R009:** unchanged in ownership, still validated by the green end-to-end `reference-backend/` proof path, with S08 only reconciling the promoted surfaces.

## Decisions recorded
- **D025:** guard public proof drift with exact string checks over the shared recovery-aware command list and `/health` recovery vocabulary.
- **D026:** use the S07 recovery-aware UAT command order as the canonical acceptance script for reconciled S05/S06/S08 closure surfaces.

## Patterns established
- Final closure slices should not invent a second acceptance story; they should converge all surfaces onto the already-green proof path.
- Literal stale-claim sweeps are brittle by design; banned phrases must be absent literally, even in negated sentences.
- For authoritative website proof evidence in this repo, run `npm --prefix website ci` and `npm --prefix website run build` serially.
- Requirement and milestone closure text should trail proof, not lead it.

## Files that matter downstream
- `reference-backend/README.md`
- `website/docs/docs/production-backend-proof/index.md`
- `reference-backend/scripts/verify-production-proof-surface.sh`
- `.gsd/milestones/M028/slices/S05/S05-SUMMARY.md`
- `.gsd/milestones/M028/slices/S05/S05-UAT.md`
- `.gsd/milestones/M028/slices/S06/S06-SUMMARY.md`
- `.gsd/milestones/M028/slices/S06/S06-UAT.md`
- `.gsd/milestones/M028/M028-VALIDATION.md`
- `.gsd/REQUIREMENTS.md`
- `.gsd/KNOWLEDGE.md`
- `.gsd/PROJECT.md`

## What the reassess-roadmap agent should know
M028 is closed. There is no remaining truth-surface gap inside this milestone.

The important read on the repo after S08 is:
- the reference backend is green across build, fmt, test, migration, deploy smoke, worker-crash recovery, restart visibility, and whole-process restart recovery
- the public and internal proof surfaces now point at that same green command set instead of contradicting it
- the next planning problem is no longer “prove Mesh can do one real backend honestly”
- the next planning problem is “what post-baseline backend ergonomics, package trust, and differentiators matter most now that the baseline is credible?”
