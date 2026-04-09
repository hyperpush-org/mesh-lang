---
id: S03
parent: M045
milestone: M045
provides:
  - A scaffold-first failover proof rail (`bash scripts/verify-m045-s03.sh`) that proves pre-kill mirrored pending truth, automatic promotion/recovery, and stale-primary rejoin on the same tiny example surface.
  - A clustered scaffold that stays small and runtime-owned while still supporting both the single-node S02 path and the destructive two-node failover path.
requires:
  - slice: S01
    provides: `Node.start_from_env()` plus typed `BootstrapStatus`, which the scaffold-first clustered example and its logs consume as the runtime-owned bootstrap seam.
  - slice: S02
    provides: The tiny clustered scaffold contract and the two-node runtime-completion/remote-owner proof surfaces that S03 replays before the destructive failover rail.
affects:
  - S04
  - S05
key_files:
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/tests/e2e_m045_s03.rs
  - scripts/verify-m045-s03.sh
  - .gsd/PROJECT.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Use `Node.list()` in the clustered scaffold to derive `required_replica_count`, so one scaffold stays truthful in both single-node clustered runs and two-node failover runs without manual operator/config surfaces.
  - Keep a small fixed `Timer.sleep(250)` in the generated declared-work handler so the local failover rail can actually kill a primary-owned mirrored pending request before it completes, while avoiding a scaffold-specific env knob.
  - Treat the rejoin proof for the selected request as post-rejoin continuity + stale-primary duplicate-submit truth, and use diagnostics/logs only for generic `fenced_rejoin` occurrence because the restarted primary’s diagnostics buffer can key on a different stale request.
patterns_established:
  - Use a peer-aware replica-count helper (`Node.list()`) to keep one scaffold truthful across both local-only clustered runs and two-node failover runs without reintroducing app-owned operator surfaces.
  - For fast failover transitions, derive the recovered attempt ID from durable diagnostics entries (`automatic_recovery` / `recovery_rollover`) instead of relying on a transient pending continuity snapshot.
  - Make the slice-level verifier snapshot `.tmp` before replay, copy only fresh artifact directories, and fail closed on bundle-pointer or file-shape drift so later slices can trust retained evidence without rerunning the cluster immediately.
observability_surfaces:
  - `meshc cluster status <node> --json` for membership/authority promotion truth before kill, after promotion, and after rejoin.
  - `meshc cluster continuity <node> <request_key> --json` on ingress/standby/rejoined-primary to prove the selected request’s pre-kill pending, post-kill completed, and post-rejoin fenced truth.
  - `meshc cluster diagnostics <node> --json` plus retained primary/standby stdout/stderr logs and `.tmp/m045-s03/verify/retained-m045-s03-artifacts/` for automatic promotion/recovery and rejoin debugging.
drill_down_paths:
  - .gsd/milestones/M045/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M045/slices/S03/tasks/T02-SUMMARY.md
  - .gsd/milestones/M045/slices/S03/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-30T23:14:32.436Z
blocker_discovered: false
---

# S03: Tiny Example Failover Truth

**The tiny scaffold-first clustered example now survives primary loss on the same two-node surface, and S03 ships an authoritative verifier that retains runtime-owned failover evidence end to end.**

## What Happened

S03 finished the failover half of the scaffold-first clustered example instead of falling back to `cluster-proof` as the “real” proof app. The slice added `scripts/verify-m045-s03.sh` as the authoritative local stopping condition, replaying the M044/S04 failover prerequisite, the clustered scaffold init contract, the S02 runtime-completion rail, and the new S03 e2e before copying one fresh `.tmp/m045-s03/scaffold-failover-runtime-truth-*` bundle into `.tmp/m045-s03/verify/retained-m045-s03-artifacts/` and fail-closing on stale/malformed evidence. To make the scaffold itself truthful on both single-node and two-node runs, the generated submit path now asks for one replica only when `Node.list()` reports a connected peer, while the generated declared-work handler sleeps briefly so the primary-owned mirrored pending state survives long enough for the destructive failover proof to kill the owner. The S03 e2e now matches the real standby-side pre-kill record (`cluster_role=standby`), derives recovered attempt IDs from the runtime diagnostics surface instead of racing a transient post-kill pending CLI snapshot, kills the primary immediately after the standby mirror is observed, and proves stale-primary rejoin through post-rejoin continuity and duplicate-submit truth on the selected request.

## Verification

Verified with `bash scripts/verify-m045-s03.sh` (green). The assembled rail replays `bash scripts/verify-m044-s04.sh`, `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`, `cargo test -p meshc --test e2e_m045_s02 m045_s02_scaffold_runtime_completion_ -- --nocapture`, and `cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture`, then copies one fresh `.tmp/m045-s03/scaffold-failover-runtime-truth-*` bundle into `.tmp/m045-s03/verify/retained-m045-s03-artifacts/` and checks its pointer and required file shape before returning green.

## Requirements Advanced

- R077 — Kept the primary example tiny while adding failover truth: the scaffold still exposes only `Node.start_from_env()`, `/health`, the submit route, and runtime CLI inspection surfaces instead of manual failover/operator endpoints.

## Requirements Validated

- R078 — `bash scripts/verify-m045-s03.sh` replays clustered init, the S02 runtime-completion rail, the M044/S04 failover prerequisite, and `cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture`, then retains a fresh failover bundle with pre-kill, post-kill, and post-rejoin runtime-owned evidence.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The original plan assumed the post-kill recovered attempt could be proven by catching a transient `phase=submitted` continuity snapshot on the promoted standby. On the current runtime that window was too short-lived, so the slice moved that proof to the durable `meshc cluster diagnostics --json` surface (`automatic_recovery` + `recovery_rollover`) and kept selected-request truth on the post-kill/post-rejoin continuity surfaces. The generated scaffold also needed a small fixed `Timer.sleep(250)` in `execute_declared_work(...)` so the primary-owned mirrored pending window stayed observable long enough for the destructive failover rail to kill the owner without adding a scaffold-specific env knob.

## Known Limitations

The generated scaffold still carries a fixed 250ms demo delay in `execute_declared_work(...)` to keep the primary-owned pending failover window observable on the current local runtime. The restarted primary’s diagnostics buffer does not reliably key `fenced_rejoin` to the request under test, so authoritative selected-request rejoin truth still comes from `meshc cluster continuity --json` on both nodes plus the stale-primary duplicate-submit response. S04/S05 still need to retire the remaining legacy example-side clustered residue and move the public teaching surface onto the tiny scaffold-first example.

## Follow-ups

1. In S04, remove the remaining legacy example-side clustered residue outside the scaffold-first path so `cluster-proof`-style mechanics no longer dominate the repo’s clustered teaching story.
2. In S05, make the public docs teach the scaffold-first clustered example and its verifier first, with deeper proof rails secondary.
3. Decide later whether the scaffold’s fixed 250ms demo delay should stay in generated source or move to a more neutral runtime/demo seam once the local failover proof can remain truthful without it.

## Files Created/Modified

- `compiler/mesh-pkg/src/scaffold.rs` — Changed the clustered scaffold so declared-work submits request one replica only when a peer is connected, added a brief fixed work delay, and updated the README contract to describe the runtime-owned failover demo surface.
- `compiler/meshc/tests/e2e_m045_s03.rs` — Finished the scaffold-first failover e2e: standby pre-kill truth now matches the real mirrored standby record shape, recovery attempt IDs come from diagnostics instead of a transient pending poll, the destructive primary kill happens immediately after selection, and rejoin proof relies on stable continuity/duplicate-submit truth for the selected request.
- `scripts/verify-m045-s03.sh` — Added the authoritative assembled verifier for S03, including prerequisite replays, fresh artifact capture, bundle-shape validation, and retained failover evidence under `.tmp/m045-s03/verify/`.
- `.gsd/PROJECT.md` — Refreshed the living project state to mark scaffold-first failover truth complete and narrowed the remaining milestone gap to S04/S05 cleanup and docs promotion.
- `.gsd/KNOWLEDGE.md` — Appended durable M045/S03 gotchas covering the scaffold failover window, the stable rejoin proof surface, and the current requirement-DB drift for M045 IDs.
