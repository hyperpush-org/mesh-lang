---
id: S03
parent: M043
milestone: M043
provides:
  - A same-image two-cluster Docker failover proof rail for `cluster-proof` using hostname-derived `primary@primary:4370` / `standby@standby:4370` identities.
  - A fail-closed packaged verifier that replays the prior continuity rails and validates copied same-image JSON/log artifacts.
  - An early continuity-env startup fence in the image entrypoint so contradictory role/epoch input never reaches ambiguous runtime startup.
requires:
  - slice: S02
    provides: Runtime-owned promotion, authority-status readback, same-key attempt rollover, and stale-primary fencing that the same-image operator rail packages and replays.
affects:
  - S04
key_files:
  - compiler/meshc/tests/e2e_m043_s03.rs
  - scripts/verify-m043-s03.sh
  - cluster-proof/docker-entrypoint.sh
  - cluster-proof/tests/config.test.mpl
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - Keep the same-image Docker proof strict on ownership, epoch, hostname-derived node names, and stale-primary fencing, but allow the promoted standby's post-rejoin `replication_health` to be either `local_only` or `healthy` because the runtime truth varies with timing.
  - Select the retained same-image proof directory by `scenario-meta.json` plus required-file shape instead of a loose artifact-name prefix, because the full `e2e_m043_s03` target also creates a malformed-response guard bundle.
  - Mirror the continuity role/epoch validation in `cluster-proof/docker-entrypoint.sh` so contradictory same-image continuity env exits non-zero before ambiguous runtime startup.
patterns_established:
  - Pair destructive Docker continuity e2es with a copied-artifact verifier that selects proof bundles by required-file shape and asserts runtime-owned JSON/log truth instead of trusting command exit codes.
  - Keep same-image node identity derived from Docker hostname in the image entrypoint and keep continuity role/epoch validation at that same boundary so the operator rail stays thin and fail-closed.
  - Replay prior authoritative continuity rails before packaging-level assertions so the packaged verifier composes existing proof surfaces instead of becoming a parallel source of truth.
observability_surfaces:
  - `.tmp/m043-s03/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, and `full-contract.log` provide the packaged verifier's authoritative phase and failure surface.
  - `.tmp/m043-s03/verify/05-same-image-artifacts/.../scenario-meta.json`, `*membership*.json`, `*status*.json`, and per-container `stdout`/`stderr` logs preserve the runtime-owned failover truth for postmortem inspection.
  - `.tmp/m043-s03/verify/04a-invalid-continuity.log` preserves the fail-closed same-image misconfiguration probe without leaking cookie material.
drill_down_paths:
  - .gsd/milestones/M043/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M043/slices/S03/tasks/T02-SUMMARY.md
  - .gsd/milestones/M043/slices/S03/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-29T11:26:44.517Z
blocker_discovered: false
---

# S03: Same-Image Two-Cluster Operator Rail

**Packaged the M043 failover contract into a same-image Docker operator rail with retained proof artifacts and fail-closed continuity-env startup.**

## What Happened

S03 turned the runtime-owned primary→standby failover from S01/S02 into a repeatable operator path. T01 added `compiler/meshc/tests/e2e_m043_s03.rs`, which builds one repo-root `cluster-proof` image, runs primary and standby from that same image with hostname-derived node identity, chooses a request key whose owner is `primary` and replica is `standby`, and retains scenario metadata, raw HTTP, sanitized Docker inspect output, and per-container stdout/stderr under `.tmp/m043-s03/`. The destructive sequence proves mirrored pending truth, primary loss, degraded standby truth, explicit `/promote`, same-key retry rollover from `attempt-0` to `attempt-1`, completion on the promoted standby, and fenced stale-primary rejoin on the original stale env.

T02 wrapped that rail in `scripts/verify-m043-s03.sh`. The wrapper replays the prerequisite continuity rails (`mesh-rt` continuity, `cluster-proof` tests/build, and `bash scripts/verify-m043-s02.sh`), runs the full `e2e_m043_s03` target so the negative guards stay inside the packaged contract, copies the real same-image artifact bundle under `.tmp/m043-s03/verify/05-same-image-artifacts/`, and fail-closes on missing, empty, or malformed artifacts plus any stale-primary execution/completion drift. It selects the real proof bundle by `scenario-meta.json` and required-file shape because the full test target also emits a malformed-response guard bundle.

T03 tightened `cluster-proof/docker-entrypoint.sh` and `cluster-proof/tests/config.test.mpl` so blank or contradictory continuity role/epoch input fails at the image boundary instead of drifting into ambiguous runtime startup. Valid primary, standby, and stale-primary restart env still boot unchanged. The packaged verifier now includes an entrypoint-misconfig phase that proves the early config error and rejects any runtime-start log line or cookie leakage in the retained misconfiguration log.

## Verification

I reran `cargo test -p meshc --test e2e_m043_s03 -- --nocapture`, which passed all 3 tests in 24.15s. I then reran `bash scripts/verify-m043-s03.sh`, which completed green and replayed `cargo test -p mesh-rt continuity -- --nocapture`, `cargo run -q -p meshc -- test cluster-proof/tests`, `cargo run -q -p meshc -- build cluster-proof`, `bash scripts/verify-m043-s02.sh`, and the full `e2e_m043_s03` target before validating copied artifacts.

The retained verifier state is healthy: `.tmp/m043-s03/verify/status.txt` is `ok`, `.tmp/m043-s03/verify/current-phase.txt` is `complete`, and `.tmp/m043-s03/verify/phase-report.txt` shows every phase passed (`runtime-continuity`, `cluster-proof-tests`, `build-cluster-proof`, `s02-contract`, `same-image-contract`, `entrypoint-misconfig`, and `same-image-artifacts`). `.tmp/m043-s03/verify/05-same-image-bundle-values.txt` records the concrete failover key and attempt rollover (`same-image-failover-key-0`, `attempt-0` → `attempt-1`, `primary@primary:4370`, `standby@standby:4370`). The copied bundle's `scenario-meta.json`, `promoted-membership-standby.json`, `membership-primary-run2.json`, `post-rejoin-primary-status.json`, and `stale-guard-primary.json` show epoch-1 promotion on standby plus fenced/deposed rejoin on the old primary. `.tmp/m043-s03/verify/04a-invalid-continuity.log` shows the fail-closed startup error: `[cluster-proof] Config error: Invalid continuity topology: standby role requires promotion epoch 0 before promotion`.

## Requirements Advanced

- R052 — S03 proves the local same-image Docker operator rail with one `cluster-proof` image, a small continuity env surface, hostname-derived node identity, retained failover artifacts, and fail-closed misconfiguration checks. The Fly/public-contract half remains for S04.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None.

## Known Limitations

This slice proves the same-image operator rail locally in Docker only. Public README/help/docs reconciliation and the read-only Fly proof surface remain S04 work. Also, after rejoin the promoted standby's `replication_health` can settle as either `local_only` or `healthy`; downstream checks should stay strict on ownership, epoch, and stale-primary fencing fields instead of treating one post-rejoin health string as the only green state.

## Follow-ups

S04 should align the cluster-proof README, distributed-proof docs, help/verifier text, and read-only Fly checks with `bash scripts/verify-m043-s03.sh` and the retained artifact contract so the public proof surface describes the shipped same-image failover rail exactly.

## Files Created/Modified

- `compiler/meshc/tests/e2e_m043_s03.rs` — Adds the same-image Docker failover e2e, retained artifact bundle generation, placement guards, and stale-primary fencing assertions.
- `scripts/verify-m043-s03.sh` — Adds the canonical packaged verifier that replays prior rails, runs the full same-image target, copies the real proof bundle, checks runtime-owned failover truth from retained artifacts, and probes entrypoint misconfiguration.
- `cluster-proof/docker-entrypoint.sh` — Fails closed on blank or contradictory continuity role/epoch input while preserving valid primary, standby, stale-primary, hostname-fallback, and Fly-startup paths.
- `cluster-proof/tests/config.test.mpl` — Extends config coverage for valid standby/stale-primary env and invalid cluster-mode role/epoch combinations.
- `.gsd/KNOWLEDGE.md` — Records the post-rejoin replication-health allowance, artifact-bundle selection rule, and early entrypoint-validation seam for future continuity work.
- `.gsd/PROJECT.md` — Refreshes the current-state record so M043/S03 is represented as complete and only S04/public-proof/Fly work remains.
