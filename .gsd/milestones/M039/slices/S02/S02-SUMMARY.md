---
id: S02
parent: M039
milestone: M039
provides:
  - A typed `GET /work` proof surface in `cluster-proof` that distinguishes ingress node, target node, execution node, `routed_remotely`, and truthful local fallback while leaving `/membership` unchanged.
  - A real dual-stack localhost e2e harness (`e2e_m039_s02`) that proves remote routing in both ingress directions and truthful self-only fallback without inventing peers.
  - A canonical local replay wrapper (`scripts/verify-m039-s02.sh`) that replays S01 first and preserves stable routing artifacts for downstream debugging.
requires:
  - slice: S01
    provides: Local DNS discovery, truthful membership from `Node.self()` plus `Node.list()`, and the fail-closed `scripts/verify-m039-s01.sh` prerequisite replay surface.
affects:
  - S03
  - S04
key_files:
  - cluster-proof/work.mpl
  - cluster-proof/main.mpl
  - cluster-proof/tests/work.test.mpl
  - compiler/meshc/tests/e2e_m039_s02.rs
  - scripts/verify-m039-s02.sh
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - D132: Keep the proof app narrow by using a read-only `cluster-proof` work endpoint that deterministically prefers a non-self peer and leaves `/membership` as the separate truth surface.
  - D134: Within current Mesh runtime limits, prove internal routing through a direct spawn-and-return `/work` path and use the execution-node log as the second proof signal instead of waiting for a coordinator reply inside the HTTP path.
  - D133: Treat `scripts/verify-m039-s02.sh` as the authoritative local S02 acceptance surface, replay S01 first, and preserve copied per-node route/log artifacts for postmortem inspection.
patterns_established:
  - For internal-routing proofs, pair the HTTP route body with the execution-node stdout log instead of treating one signal as sufficient.
  - Replay the prerequisite distributed slice verifier before the dependent slice so routing proof cannot hide earlier discovery or membership drift.
  - When the current runtime makes the two-node proof startup-order-sensitive, prove each ingress direction in its own fresh cluster lifetime rather than forcing a flaky symmetric run.
observability_surfaces:
  - `[cluster-proof] work services ready ...` startup log showing the result-registry name and local node identity.
  - `[cluster-proof] work executed request_id=... execution=...` on the execution node as the second proof signal for `/work`.
  - `[cluster-proof] work failed ...`, `work malformed result ...`, and `work timeout ...` log paths for future routing failures even though the current slice only exercises the happy remote and local-fallback paths.
  - `.tmp/m039-s02/verify/phase-report.txt` as the authoritative verifier phase ledger.
  - `.tmp/m039-s02/verify/04-s02-remote-route-artifacts/*/node-*-work.json` plus matching node stdout/stderr logs as durable two-node route evidence.
  - `.tmp/m039-s02/verify/05-s02-local-fallback-artifacts/*/node-solo-work.json` plus stdout/stderr logs as durable single-node fallback evidence.
drill_down_paths:
  - .gsd/milestones/M039/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M039/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M039/slices/S02/tasks/T03-SUMMARY.md
  - .gsd/milestones/M039/slices/S02/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-28T11:24:45.498Z
blocker_discovered: false
---

# S02: Native Cluster Work Routing Proof App

**`cluster-proof` now proves runtime-native internal balancing by returning truthful ingress/target/execution data from `/work`, preserving `/membership` as the separate cluster-truth surface, and replaying that contract through a fail-closed local verifier.**

## What Happened

S02 finished the cluster-proof balancing story by turning the earlier T01/T02 runtime mismatch into a smaller but honest proof surface. `cluster-proof/work.mpl` now computes deterministic peer-preferred target selection from live membership, keeps `/membership` untouched as the separate cluster-truth surface, and answers `GET /work` with a typed payload that names `ingress_node`, `target_node`, `execution_node`, `routed_remotely`, `fell_back_locally`, `timed_out`, and `error`. When a peer exists, the route dispatches work through direct `Node.spawn`; when no peer exists, it falls back locally. The response returns immediately with the truthful routing decision, while the execution node emits `[cluster-proof] work executed request_id=... execution=...` so the proof has an app-visible second signal instead of pretending the HTTP handler observed remote completion itself.

The slice also closed the proof gap around that route. `cluster-proof/tests/work.test.mpl` now covers the pure selection helpers so membership-order and fallback behavior are pinned before live runtime tests run. `compiler/meshc/tests/e2e_m039_s02.rs` reuses the S01 harness patterns to boot real `cluster-proof` nodes on dual-stack localhost, assert `/membership` convergence first, hit `/work` on node A and node B directly in separate cluster lifecycles, and preserve route bodies and per-node stdout/stderr under `.tmp/m039-s02/`. The harness fails closed on malformed JSON, missing fields, early exits, or missing execution-log evidence.

Finally, S02 added the canonical local replay wrapper in `scripts/verify-m039-s02.sh`. That script runs `cluster-proof/tests`, rebuilds `cluster-proof`, replays the full S01 verifier as a prerequisite, then runs the two named S02 filters with bounded timeouts, explicit non-zero test-count checks, and stable copied artifacts under `.tmp/m039-s02/verify/`. The preserved evidence now shows the exact property this slice was meant to prove: a request can enter one node, report that ingress node in the HTTP body, and still show another node as both the chosen target and the execution node, while the single-node path stays truthful about local fallback.

## Verification

Re-ran the canonical slice verifier and the slice-level contract passed. `bash scripts/verify-m039-s02.sh` completed successfully, replayed `cluster-proof/tests`, rebuilt `cluster-proof`, replayed the full S01 verifier as a prerequisite, and then ran the two named `e2e_m039_s02` filters with non-zero test counts. The preserved artifacts under `.tmp/m039-s02/verify/` confirm both contract shapes: the remote two-node path returns distinct `ingress_node` and `execution_node` values with `routed_remotely=true`, and the single-node path returns truthful self-only fallback with `routed_remotely=false` and `fell_back_locally=true`. I also checked the copied execution-node stdout logs and the route-body JSON artifacts directly; both surfaces agreed on the matching `request_id` and execution node.

### Operational Readiness
- **Health signal:** `.tmp/m039-s02/verify/phase-report.txt` records all five phases as passed, and the current happy-path route artifacts show `ok=true`, `timed_out=false`, and a matching execution-node stdout line for the returned `request_id`.
- **Failure signal:** the verifier fail-closes on a broken S01 prerequisite, missing `running N test` evidence, malformed `node-*-work.json`, missing copied node logs, or absent execution-log proof. At runtime, `cluster-proof` also emits `work failed`, `work malformed result`, and `work timeout` log lines for future non-happy-path routing failures.
- **Recovery procedure:** rerun `bash scripts/verify-m039-s01.sh` first, then rerun `bash scripts/verify-m039-s02.sh`. If the wrapper still fails, inspect the newest `node-*-work.json` body and the matching execution-node stdout log in `.tmp/m039-s02/verify/` before changing the route logic; if the failure is directional, preserve the per-direction startup order from the current harness.
- **Monitoring gaps:** request correlation is still limited to the stable `work-0` token, and the current slice does not yet prove routing continuity through node loss or rejoin. Those are real gaps for S03 rather than missing documentation.

## Requirements Advanced

None.

## Requirements Validated

- R047 — Validated by `bash scripts/verify-m039-s02.sh`, which replays `scripts/verify-m039-s01.sh`, then proves remote routing with `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_routes_work_to_peer_and_logs_execution -- --nocapture` and single-node truthful fallback with `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_falls_back_locally_without_peers -- --nocapture`. Durable evidence lives under `.tmp/m039-s02/verify/` as `node-*-work.json` route bodies plus execution-node stdout logs with the matching `request_id`.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Did not keep the original coordinator/result-registry wait design from T01/T02. The final route is a smaller direct spawn-and-return proof surface because ordinary HTTP handlers are not actor contexts and distributed spawn/send payloads are still scalar-only. The two-node proof also uses two separate cluster lifecycles so each ingress direction is proven with its ingress node started first.

## Known Limitations

The current distributed request-correlation token is still the normalized `work-0` value rather than a unique per-request id because spawned worker argument transport is not yet a trustworthy correlation channel. The route proves internal balancing by pairing the `/work` response with the execution-node log, but it does not yet prove continued routing through node loss or rejoin; S03 owns that failure-path expansion. The canonical two-node proof is also startup-order-sensitive, so each ingress direction is proven in its own cluster lifetime.

## Follow-ups

S03 should reuse the current `/work` proof body plus execution-log pairing under node-loss and rejoin scenarios instead of inventing a new routing harness. It should start by replaying `scripts/verify-m039-s01.sh` and `scripts/verify-m039-s02.sh`, then extend the e2e surface to prove continued work acceptance, truthful degraded membership, and clean post-rejoin recovery. S04 should keep `scripts/verify-m039-s02.sh` as the local contract input when it builds the one-image/Fly operator path and public docs truth surfaces.

## Files Created/Modified

- `cluster-proof/work.mpl` — Implements deterministic peer-preferred work routing, the typed `/work` response payload, direct remote/local spawn dispatch, and the execution-log proof surface.
- `cluster-proof/main.mpl` — Keeps `/membership` as the separate truthful membership diagnostic surface while wiring the new `/work` handler into the proof app startup.
- `cluster-proof/tests/work.test.mpl` — Covers the pure routing-selection helper behavior and the truthful local-fallback rule for the proof module.
- `compiler/meshc/tests/e2e_m039_s02.rs` — Adds the S02 Rust harness that starts real `cluster-proof` nodes, probes `/membership` and `/work`, preserves route-body artifacts, and asserts execution-node log evidence.
- `scripts/verify-m039-s02.sh` — Adds the canonical local S02 verifier that replays S01 first, fail-closes on zero-test filters, and copies stable per-node artifacts under `.tmp/m039-s02/verify/`.
- `.gsd/KNOWLEDGE.md` — Records the scalar-only transport limit, handler-context limit, `work-0` correlation constraint, and the authoritative S02 verifier/artifact layout for future M039 slices.
- `.gsd/PROJECT.md` — Refreshes the living project state so M039 now includes proven local discovery, truthful membership, and runtime-native work routing.
