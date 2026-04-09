---
id: S03
parent: M039
milestone: M039
provides:
  - A canonical local S03 replay wrapper (`scripts/verify-m039-s03.sh`) that proves degrade, continued local service, same-identity rejoin, and remote-routing recovery with copied evidence manifests.
  - Restart-safe request correlation and run-numbered proof artifacts for `cluster-proof`, so downstream operator-path work can reuse one truthful continuity contract instead of inventing a new one.
requires:
  - slice: S01
    provides: DNS discovery, stable node identity, truthful `/membership`, and the S01 fail-closed replay pattern.
  - slice: S02
    provides: The `/work` ingress-vs-execution proof surface, truthful one-node fallback behavior, and the S02 replay-wrapper artifact pattern.
affects:
  - S04
key_files:
  - cluster-proof/work.mpl
  - cluster-proof/tests/work.test.mpl
  - compiler/meshc/tests/e2e_m039_s02.rs
  - compiler/meshc/tests/e2e_m039_s03.rs
  - scripts/verify-m039-s03.sh
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - D137: keep request correlation ingress-owned via `work dispatched ...` logs plus run-numbered peer logs/artifacts instead of cross-node actor arguments through rejoin.
  - D138: treat `scripts/verify-m039-s03.sh` as the canonical local S03 replay wrapper, replaying cluster-proof tests/build plus S01 and S02 before fail-closing on malformed or skipped continuity evidence.
  - D139: R048 is now validated by the passing S03 wrapper and the preserved pre-loss/degraded/post-rejoin evidence bundle.
patterns_established:
  - Keep request correlation ingress-owned when distributed actor payload transport is not restart-safe; pair response JSON with ingress dispatch logs and run-numbered peer execution logs instead of widening the HTTP contract.
  - For continuity slices, replay earlier slice verifiers inside the new wrapper before the new proof phases, so later acceptance cannot hide drift in prerequisite discovery or routing contracts.
  - Snapshot temp artifact directories before each proof phase and copy only the new directories into a stable verify bundle; this preserves restart evidence while avoiding stale-directory false positives.
observability_surfaces:
  - `/membership` JSON on each node, showing truthful `self`, `peers`, and `membership` before loss, during self-only degrade, and after rejoin.
  - `/work` JSON with `request_id`, `ingress_node`, `target_node`, `execution_node`, `routed_remotely`, and `fell_back_locally`, which now distinguishes `work-0`, `work-1`, and `work-2` across one cluster lifetime.
  - Ingress-side `[cluster-proof] work dispatched ...` log lines in `node-a-run1.stdout.log`, which preserve request correlation across peer loss and same-identity restart.
  - Execution-side `[cluster-proof] work executed execution=...` log lines in run-numbered peer logs (`node-b-run1.stdout.log`, `node-b-run2.stdout.log`) so restart evidence is not clobbered.
  - `.tmp/m039-s03/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, `05-s03-degrade-artifacts.txt`, and `06-s03-rejoin-artifacts.txt`, which make verifier health and copied evidence explicit for later debugging.
drill_down_paths:
  - .gsd/milestones/M039/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M039/slices/S03/tasks/T02-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-28T12:16:44.428Z
blocker_discovered: false
---

# S03: Single-Cluster Failure, Safe Degrade, and Rejoin

**Local `cluster-proof` now proves safe self-only degrade after peer loss, clean same-identity rejoin, and restored remote routing with restart-safe evidence and one authoritative S03 verifier.**

## What Happened

S03 finished the local single-cluster continuity story for `cluster-proof` by proving that one node can die, the survivor shrinks to truthful self-only membership, new `/work` continues locally, and the same node identity can rejoin cleanly without manual repair. T01 replaced the fixed request token with an ingress-owned monotonic request-id service, updated the proof-app tests and existing S02 harness to parse integer tokens instead of assuming every request is `work-0`, and added `compiler/meshc/tests/e2e_m039_s03.rs` with two live two-node proofs: one for safe degrade after peer loss and one for same-identity rejoin with remote-routing recovery. Those tests preserve pre-loss, degraded, and post-rejoin membership/work JSON plus run-numbered node stdout/stderr logs so the second incarnation cannot overwrite the first crashed-node evidence. During proofing, cross-node actor arguments proved untrustworthy through rejoin — integer request tokens reappeared as `work-0` on the restarted peer and string request ids crashed the peer in `mesh-rt` — so the slice locked correlation to ingress-side `work dispatched ...` logs and treated run-numbered peer logs plus `*-work.json` artifacts as the durable remote-execution truth surface. T02 then packaged the contract into `scripts/verify-m039-s03.sh`, a fail-closed replay wrapper that reruns `cluster-proof/tests`, `meshc build cluster-proof`, `scripts/verify-m039-s01.sh`, and `scripts/verify-m039-s02.sh` before the named S03 degrade/rejoin filters, checks that each named Cargo filter actually ran tests, and copies only the newly created S03 proof directories into `.tmp/m039-s03/verify/` with manifest validation and JSON-shape checks. The result is one authoritative local acceptance surface for M039/S03 plus one durable evidence bundle that downstream slices can inspect without rerunning the whole cluster by hand.

## Verification

Verified the full slice contract with the authoritative local replay surface: `bash scripts/verify-m039-s03.sh` completed successfully, replayed `cluster-proof/tests`, `meshc build cluster-proof`, `scripts/verify-m039-s01.sh`, `scripts/verify-m039-s02.sh`, and the two named `e2e_m039_s03` proofs, then wrote `status.txt=ok`, `current-phase.txt=complete`, and a passed `phase-report.txt`. Read the copied evidence bundle to confirm the runtime story itself, not just the wrapper exit code: the degrade artifacts show two-node pre-loss membership, self-only degraded membership, `work-0` remote execution before loss, and `work-1` local fallback after loss; the rejoin artifacts show both nodes in membership again plus `work-2` routed remotely after same-identity restart. Confirmed the observability surfaces directly in `node-a-run1.stdout.log` and `node-b-run2.stdout.log`, where ingress-side `work dispatched ...` and execution-side `work executed ...` lines match the preserved `request_id` and node identity evidence.

## Requirements Advanced

- R046 — The preserved pre-loss, degraded, and post-rejoin `/membership` artifacts now prove that membership shrinks to self-only after peer loss and grows back to both nodes on same-identity rejoin in the local proof environment.
- R047 — The continuity replay now proves that runtime-native internal balancing recovers after rejoin: `/work` routes remotely before loss, falls back locally during degrade, and returns to remote execution after the peer restarts.

## Requirements Validated

- R048 — `bash scripts/verify-m039-s03.sh` passes after replaying cluster-proof tests/build plus the S01 and S02 wrappers, and the preserved `.tmp/m039-s03/verify/` bundle shows the full contract: two-node pre-loss membership, truthful self-only degraded membership, successful degraded `/work` local fallback with `work-1`, same-identity rejoin, and restored remote `/work` routing with `work-2` without manual repair.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None.

## Known Limitations

This proof is still a local two-node `cluster-proof` continuity story, not the one-image/Fly operator proof that S04 owns. Cross-node actor arguments remain an unsafe channel for rich request correlation through same-identity restart: integer tokens can reset unexpectedly on the restarted peer and string payloads still crash the peer in current `mesh-rt` transport handling, so correlation must stay ingress-owned until that runtime seam is repaired.

## Follow-ups

S04 should carry the same continuity contract through the one-image operator path and Fly-backed proof environment using `scripts/verify-m039-s03.sh` as the local baseline. If later M040/M041 work needs richer cross-node request metadata than scalar tokens, the runtime transport layer needs real distributed string/struct serialization instead of reusing the current raw-value spawn/send path.

## Files Created/Modified

- `cluster-proof/work.mpl` — Replaced the fixed `work-0` token path with ingress-owned monotonic request correlation while preserving the narrow `/work` contract and ingress-side dispatch logging.
- `cluster-proof/tests/work.test.mpl` — Updated the Mesh proof-app tests to accept deterministic incrementing request ids instead of a permanently fixed token.
- `compiler/meshc/tests/e2e_m039_s02.rs` — Adjusted the S02 routing proof to validate parsed request-id tokens under the new monotonic correlation contract.
- `compiler/meshc/tests/e2e_m039_s03.rs` — Added the live two-node degrade and same-identity rejoin harness, per-phase membership/work artifacts, and run-numbered node logs for continuity proof.
- `scripts/verify-m039-s03.sh` — Added the fail-closed canonical S03 replay wrapper with prerequisite replays, phase markers, non-zero test-count checks, and copied degrade/rejoin manifests.
- `.gsd/KNOWLEDGE.md` — Recorded the authoritative S03 replay gate and the rejoin-safe request-correlation rule for future slices.
- `.gsd/PROJECT.md` — Updated current project state to reflect that M039 now proves local single-cluster degrade and same-identity rejoin continuity.
