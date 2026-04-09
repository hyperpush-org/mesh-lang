---
id: S04
parent: M044
milestone: M044
provides:
  - Bounded automatic promotion and automatic recovery for declared clustered work on the same-image local failover rail.
  - An authoritative `scripts/verify-m044-s04.sh` acceptance command with retained proof-bundle recording and docs-truth checks.
  - A public auto-only failover story: no manual authority mutation surface, stale-primary fencing on rejoin, and read-only operator inspection.
requires:
  - slice: S02
    provides: Runtime-owned declared-handler execution, continuity admission, and manifest-declared clustered handler registration.
  - slice: S03
    provides: Read-only operator inspection surfaces, clustered scaffold groundwork, and the public clustered-app/operator framing that S04 closes over.
affects:
  - S05
key_files:
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-codegen/src/declared.rs
  - cluster-proof/work_continuity.mpl
  - compiler/mesh-typeck/src/error.rs
  - compiler/mesh-typeck/src/infer.rs
  - compiler/mesh-typeck/src/builtins.rs
  - compiler/mesh-codegen/src/mir/lower.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-rt/src/lib.rs
  - compiler/mesh-lsp/src/analysis.rs
  - compiler/meshc/tests/e2e_m044_s04.rs
  - compiler/meshc/tests/e2e_m044_s01.rs
  - cluster-proof/README.md
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/distributed/index.md
  - README.md
  - scripts/verify-m044-s04.sh
  - .gsd/PROJECT.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D204 — remove the Mesh-visible `Continuity.promote()` surface and reject stale manual calls with an explicit automatic-only diagnostic while keeping runtime promotion internal-only.
  - D205 — register manifest-declared work handlers through generated actor-style `__declared_work_*` wrappers instead of the plain typed Mesh function symbol so runtime-owned declared work uses the correct actor-entry ABI.
  - Keep the assembled S04 acceptance command fail-closed on zero-test runs, stale manual failover wording, and missing retained failover artifacts via `scripts/verify-m044-s04.sh`.
patterns_established:
  - Manifest-declared work handlers that the runtime will spawn must register actor-style wrapper symbols plus matching `__actor_<wrapper>_body` deserialization bodies; registering the raw typed Mesh function corrupts args at first execution.
  - For failover closeout work, the authoritative rail should replay the named runtime/e2e filters, assert non-zero test execution, and record the latest destructive artifact bundle path instead of trusting exit codes alone.
  - When public failover docs change, make the docs-truth sweep part of the slice verifier so stale operator folklore cannot stay green after the code contract moves.
observability_surfaces:
  - `[mesh-rt continuity] transition=automatic_promotion ...`, `automatic_recovery ...`, and `automatic_*_rejected ...` stderr diagnostics.
  - `GET /membership` and `GET /work/:request_key` runtime-backed status truth in `cluster-proof`.
  - Retained same-image failover bundle under `.tmp/m044-s04/continuity-api-failover-promotion-rejoin-*/`.
  - `bash scripts/verify-m044-s04.sh` plus `.tmp/m044-s04/verify/status.txt`, `current-phase.txt`, and `latest-proof-bundle.txt`.
  - Read-only operator inspection via `meshc cluster status --json`, `meshc cluster continuity --json`, and `meshc cluster diagnostics --json` from S03.
drill_down_paths:
  - .gsd/milestones/M044/slices/S04/tasks/T01-SUMMARY.md
  - .gsd/milestones/M044/slices/S04/tasks/T02-SUMMARY.md
  - .gsd/milestones/M044/slices/S04/tasks/T03-SUMMARY.md
  - .gsd/milestones/M044/slices/S04/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-30T05:20:00.686Z
blocker_discovered: false
---

# S04: Bounded Automatic Promotion

**Bounded automatic promotion now promotes the mirrored standby, auto-resumes declared clustered work without a second submit, fences stale-primary rejoin, and ships with an authoritative S04 acceptance rail plus auto-only docs.**

## What Happened

S04 closed the failover contract at the runtime, compiler, proof-app, and public-doc layers. The runtime side now has green bounded-failover unit rails for `automatic_promotion_` and `automatic_recovery_`, and the destructive same-image `e2e_m044_s04` rail proves that a mirrored standby promotes itself only when the runtime can prove the one-primary/one-standby transition is safe, rolls a new fenced attempt, completes that attempt on the promoted standby without a second submit, and keeps the old primary fenced on rejoin. The public manual mutation seam is gone: stale Mesh code now fails closed with an explicit automatic-only diagnostic instead of lowering `Continuity.promote()`, and `cluster-proof` no longer teaches or exposes a manual authority-change route.

The last real blocker turned out not to be the continuity state machine but the declared-work execution ABI. `mesh_actor_spawn` expects an actor-entry wrapper (`extern "C" fn(*const u8)`), but declared work handlers were still being registered as plain typed Mesh functions. That corrupted declared-work string arguments on first execution and crashed `cluster-proof` inside `WorkContinuity__log_execution_started` / `mesh_string_concat` during both healthy mirrored submit and post-promotion recovery. The fix was to generate and register actor-style `__declared_work_*` wrapper symbols with matching `__actor_<wrapper>_body` deserialization bodies in `compiler/mesh-codegen/src/declared.rs`, so declared work now executes through the same ABI seam that ordinary actor entries already use. I also kept the recovery submit on an actor-context seam in `compiler/mesh-rt/src/dist/node.rs` and restored direct declared-work execution in `cluster-proof/work_continuity.mpl` so the proof app stays truthful on the recovered path.

With that seam repaired, T04 stopped being a documentation blocker and became a real closeout task again. `scripts/verify-m044-s04.sh` now replays the runtime unit rails, the destructive same-image `m044_s04_auto_promotion_` / `m044_s04_auto_resume_` rails, the manual-surface-disabled compiler checks, the `cluster-proof` build/tests, and a docs-truth sweep that refuses stale manual failover wording. The public docs (`README.md`, `cluster-proof/README.md`, `website/docs/docs/distributed/index.md`, and `website/docs/docs/distributed-proof/index.md`) now describe bounded automatic promotion, automatic recovery, stale-primary fencing, ambiguity rejection, and read-only Fly inspection instead of the old manual `/promote` story.

## Verification

Verified with the assembled acceptance rail and its underlying named proofs. `bash scripts/verify-m044-s04.sh` passed and wrote `.tmp/m044-s04/verify/status.txt=ok`, including non-zero test-count guards and a retained latest proof bundle pointer. That script replayed: `cargo test -p mesh-rt automatic_promotion_ -- --nocapture`, `cargo test -p mesh-rt automatic_recovery_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_resume_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s04 m044_s04_manual_surface_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s01 m044_s01_typed_continuity_ -- --nocapture`, `cargo run -q -p meshc -- build cluster-proof`, and `cargo run -q -p meshc -- test cluster-proof/tests`. I also reran `npm --prefix website run build`, which completed successfully after the distributed proof docs were updated. The retained failover bundle under `.tmp/m044-s04/continuity-api-failover-promotion-rejoin-1774847636876634000/` contains the concrete truth artifacts: `auto-recovery-pending-standby.json`, `auto-recovery-completed-standby.json`, `stale-guard-primary.json`, `standby-run1.stderr.log`, and `primary-run2.stderr.log`.

## Requirements Advanced

- R067 — Completed the auto-only bounded failover contract across runtime, compiler, proof-app, docs, and verifier surfaces so promotion is automatic-only, epoch/fence-based, and fail-closed on ambiguity.
- R068 — Completed the runtime-owned recovery path so declared clustered work survives primary loss through safe promotion, automatic attempt rollover, standby completion, and stale-primary fencing on rejoin.

## Requirements Validated

- R067 — Validated by `cargo test -p mesh-rt automatic_promotion_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s04 m044_s04_manual_surface_ -- --nocapture`, and the assembled `bash scripts/verify-m044-s04.sh` acceptance rail.
- R068 — Validated by `cargo test -p mesh-rt automatic_recovery_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_resume_ -- --nocapture`, and the retained failover bundle recorded by `bash scripts/verify-m044-s04.sh`.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None. The blocker recorded during T04 was retired inside the planned slice scope once the declared-work registry ABI mismatch was fixed.

## Known Limitations

S04 proves the bounded auto-promotion contract on the same-image local rail and keeps the Fly surface read-only. It does not add active-active writes, multi-standby quorum semantics, or destructive hosted failover proof. `cluster-proof` is still the proof consumer rather than the final generic clustered-app story; the full dogfood rewrite onto the standard scaffold remains in S05.

## Follow-ups

S05 should retire or redirect the old M043 proof-surface wording/verifier surfaces so there is one canonical public clustered failover story. S05 should also finish rewriting `cluster-proof` onto the generic clustered-app standard introduced in S03 instead of keeping proof-app-specific packaging/runbook language as the deepest operator path.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/node.rs` — Finished the runtime-owned bounded failover seam, including automatic promotion/recovery orchestration and actor-context recovery submission.
- `compiler/mesh-codegen/src/declared.rs` — Generated and registered actor-style declared-work wrappers so manifest-declared work uses the correct actor-entry ABI when spawned by the runtime.
- `cluster-proof/work_continuity.mpl` — Kept declared work execution on the truthful direct path used by the runtime-owned handler rail after the wrapper fix.
- `compiler/mesh-typeck/src/error.rs` — Added the explicit automatic-only diagnostic for stale `Continuity.promote()` calls.
- `compiler/mesh-typeck/src/infer.rs` — Removed the public Mesh-visible promotion builtin from type inference while keeping read-only authority access.
- `compiler/mesh-typeck/src/builtins.rs` — Dropped the stale public promotion builtin from the compiler-visible continuity surface.
- `compiler/mesh-codegen/src/mir/lower.rs` — Kept manual promotion lowering out of the generated runtime surface while preserving read-only authority status lowering.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — Removed the stale public promotion intrinsic path and kept the runtime-owned continuity calls aligned with the auto-only contract.
- `compiler/mesh-rt/src/dist/continuity.rs` — Provided the bounded promotion/recovery state transitions and declared-handler metadata carried through continuity records.
- `compiler/mesh-rt/src/lib.rs` — Removed the stale exported manual promotion ABI while keeping internal runtime promotion support.
- `compiler/mesh-lsp/src/analysis.rs` — Kept editor diagnostics aligned with the explicit automatic-only promotion error.
- `compiler/meshc/tests/e2e_m044_s04.rs` — Proved the destructive same-image bounded auto-promotion/auto-resume/stale-primary-fencing contract and retained artifact bundle.
- `compiler/meshc/tests/e2e_m044_s01.rs` — Kept typed continuity authority status and manual-surface-disabled expectations honest after S04 changes.
- `cluster-proof/README.md` — Rewrote the proof-app runbook to the auto-only bounded failover contract and authoritative S04 verifier.
- `website/docs/docs/distributed-proof/index.md` — Updated the public distributed proof page to bounded automatic promotion, automatic recovery, and stale-primary fencing.
- `website/docs/docs/distributed/index.md` — Retargeted the distributed guide’s proof note to the S04 auto-only failover story.
- `README.md` — Updated the repo root distributed-proof pointer to the bounded automatic promotion contract.
- `scripts/verify-m044-s04.sh` — Added the authoritative fail-closed S04 acceptance command with non-zero test guards, docs-truth checks, and retained proof-bundle recording.
- `.gsd/PROJECT.md` — Updated current project state from the stale S04 blocker to the completed bounded auto-promotion contract.
- `.gsd/KNOWLEDGE.md` — Recorded the declared-work wrapper ABI fix as the key lesson for future clustered-handler work.
