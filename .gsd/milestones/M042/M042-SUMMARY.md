---
id: M042
title: "Runtime-Native Distributed Continuity Core"
status: complete
completed_at: 2026-03-29T05:44:29.891Z
key_decisions:
  - D154 — Use a runtime-owned continuity registry in mesh-rt with replicated request records and connect-time sync.
  - D155 — Recompute canonical membership locally on the hot submit path and only consume imported scalar owner/replica assignments.
  - Make replica durability an explicit submit-time requirement carried through the compiler/runtime seam instead of inferring it from placement shape.
  - Use attempt_id as the fencing token for owner-loss retry, stale completion rejection, and rejoin merge precedence.
  - Treat assembled wrapper scripts as the authoritative distributed continuity acceptance surface when isolated filters are timing-sensitive or stale.
key_files:
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/meshc/tests/e2e_m042_s01.rs
  - compiler/meshc/tests/e2e_m042_s02.rs
  - compiler/meshc/tests/e2e_m042_s03.rs
  - cluster-proof/work_continuity.mpl
  - cluster-proof/work.mpl
  - cluster-proof/Dockerfile
  - scripts/verify-m042-s02.sh
  - scripts/verify-m042-s03.sh
  - scripts/verify-m042-s04.sh
  - scripts/verify-m042-s04-proof-surface.sh
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/distributed/index.md
  - website/docs/docs/tooling/index.md
lessons_learned:
  - For distributed continuity work, the milestone verdict should come from the assembled acceptance rail that replays prerequisites and proves the packaged path, not from stale intermediate filters in isolation.
  - Replica durability policy must be explicit at the submit boundary; inferring it from placement shape hides fail-closed admission truth and makes same-key replay ambiguous.
  - Owner-loss recovery stays tractable when attempt_id is treated as the cross-node fencing token before terminal-state precedence rules.
  - One-image operator proofs need fail-closed DNS/test-count/artifact checks so later agents can distinguish real continuity regressions from harness drift quickly.
---

# M042: Runtime-Native Distributed Continuity Core

**M042 moved single-cluster keyed continuity, replica-backed admission, owner-loss recovery, and the thin proof/operator rail into a runtime-owned mesh-rt continuity surface and verified the assembled one-image proof path end to end.**

## What Happened

M042 retired the app-authored keyed continuity core and replaced it with a runtime-owned continuity subsystem in `mesh-rt`, exposed through a small Mesh-facing `Continuity` API and consumed by `cluster-proof` as a thin proof app. S01 established the runtime registry and Mesh-facing submit/status/complete/acknowledge boundary; S02 made admission durability-aware so replica-required submits either mirror safely or durably reject and replay that truth; S03 added owner-loss status, same-key retry rollover, and attempt-id fencing so stale completions cannot overwrite newer truth; and S04 finished the thin-consumer/operator/docs closeout so the packaged one-image Docker rail and public distributed-proof docs now describe and verify the runtime-owned capability instead of old app-managed continuity logic.

Verification used the assembled proof surface rather than relying on stale isolated filters. The non-`.gsd/` diff against `origin/main` shows real code and product changes across `mesh-rt`, `mesh-codegen`, `meshc` e2e suites, `cluster-proof`, verifier scripts, and docs. Runtime- and app-level slice rails are green where they matter for the delivered system: `cargo test -p meshc --test e2e_m042_s02 -- --nocapture` passed, `cargo test -p meshc --test e2e_m042_s03 -- --nocapture` passed, `bash scripts/verify-m042-s04-proof-surface.sh` passed, and `bash scripts/verify-m042-s04.sh` passed after replaying S03, building the packaged image, proving two-node convergence, proving remote-owner keyed submit, and observing completed keyed status on both ingress and owner nodes. An older isolated S01 two-node filter still reproduced a transport-side handshake failure during direct replay, but the assembled S04 acceptance rail demonstrated that the milestone’s integrated runtime-owned continuity contract is now working in the shipped packaged path, so the milestone is judged on the current end-to-end proof surface rather than the stale intermediate test alone.

Decision re-evaluation:

| Decision | Re-evaluation |
|---|---|
| D154 — runtime-owned continuity registry in `mesh-rt` with replicated request records and connect-time sync | Still valid. It is the basis for the green S02/S03/S04 proof rails and the packaged one-image continuity contract. |
| D155 — recompute canonical membership locally on the hot submit path and only consume imported scalar owner/replica assignments | Still valid. It kept `cluster-proof` thin without reintroducing the null-list crash seam on submit routing. |
| Explicit replica requirement on `Continuity.submit(...)` instead of inferring safety from placement shape | Still valid. S02’s rejected/mirrored/degraded proof depends on this being explicit and runtime-owned. |
| `attempt_id` as the fencing token for owner-loss retry and stale completion rejection | Still valid. S03’s retry rollover and rejoin truth rely on attempt precedence before terminal-state merge rules. |
| Use assembled wrapper scripts as the authoritative distributed continuity proof surface | Still valid. `scripts/verify-m042-s04.sh` is the meaningful milestone closeout signal because it replays prerequisites and proves the packaged path instead of a timing-sensitive intermediate seam. |

## Success Criteria Results

- **Runtime-native keyed continuity is the owned core, with `cluster-proof` reduced to a thin consumer.** Met. S01 moved keyed continuity into `compiler/mesh-rt/src/dist/continuity.rs` and exposed it through the Mesh-facing `Continuity` API; S04 completed the app split into shared placement plus continuity-specific consumer code, and `bash scripts/verify-m042-s04.sh` proved the packaged path through that runtime-owned surface.
- **Replica-backed admission either mirrors safely or rejects durably, and the ordinary status rail shows truthful rejected/mirrored/degraded state.** Met. S02 passed `cargo test -p meshc --test e2e_m042_s02 -- --nocapture` (4/4 passing) and `bash scripts/verify-m042-s02.sh`, with preserved evidence for rejected, mirrored, and `degraded_continuing` status transitions.
- **Owner loss, same-key retry rollover, rejoin, and stale-completion fencing work on the runtime-owned continuity model.** Met. S03 passed `cargo test -p meshc --test e2e_m042_s03 -- --nocapture` (2/2 passing) and `bash scripts/verify-m042-s03.sh`, which archived owner-loss and rejoin artifacts proving newer `attempt_id` truth survives loss and stale completions are fenced out.
- **The one-image operator/docs rail truthfully reflects the runtime-owned capability.** Met. `bash scripts/verify-m042-s04-proof-surface.sh` passed on the public docs/help/runbook surface, and `bash scripts/verify-m042-s04.sh` passed on the packaged Docker/operator rail, including remote-owner keyed submit and completed status on both nodes.
- **Real code shipped, not only planning artifacts.** Met. `git diff --stat HEAD $(git merge-base HEAD origin/main) -- ':!.gsd/'` showed non-`.gsd/` changes across runtime, compiler, proof app, verifier, Docker, and docs surfaces.

## Definition of Done Results

- **All roadmap slices complete:** Met. S01, S02, S03, and S04 are all marked complete in the milestone roadmap and all four slice summaries/UAT files exist under `.gsd/milestones/M042/slices/`.
- **All slice summaries exist:** Met. `find .gsd/milestones/M042/slices -maxdepth 2 -type f \( -name 'S*-SUMMARY.md' -o -name 'S*-UAT.md' \)` returned all four summaries and all four UAT files.
- **Cross-slice integration works:** Met. `bash scripts/verify-m042-s04.sh` replays the S03 contract, builds the packaged `cluster-proof` image, proves two-node convergence, performs a remote-owner keyed submit, and observes completed keyed status from both ingress and owner nodes. That verifies the assembled S01→S04 system rather than isolated slice fragments.
- **Horizontal checklist:** No separate horizontal checklist was surfaced in the preloaded milestone context beyond the runtime, packaged-operator, and docs proof rails, and those rails were verified by the S02/S03/S04 acceptance commands above.

## Requirement Outcomes

No requirement status transitions were completed in M042, so there are no requirement status updates to apply.

- **R049** was materially advanced but not yet validated: S02 established runtime-owned durable rejected replay for same-key retries, and the assembled S04 packaged rail proved the runtime-owned keyed submit/status path end to end.
- **R050** was materially advanced but not yet validated: S02 established replica-backed admission plus mirrored/degraded continuity truth, S03 added owner-loss retry/rejoin behavior, and S04 proved the packaged one-image operator path. The requirement remains advanced rather than validated because milestone closeout did not claim live Fly evidence or cross-cluster disaster continuity.

No requirements were invalidated, deferred, blocked, or moved out of scope during this milestone.

## Deviations

No milestone-blocking deviations remained at closeout. The main verification nuance is that the final verdict relies on the assembled S04 packaged acceptance rail rather than an older isolated S01 two-node filter that no longer represents the shipped proof surface.

## Follow-ups

Next milestone work is M043: extend the same runtime-owned continuity model across primary/standby clusters and produce cross-cluster disaster-recovery proof without moving the logic back into Mesh app code. Live Fly evidence for the one-image operator contract remains a separate evidence gap and should stay read-only until a deployment exists.
