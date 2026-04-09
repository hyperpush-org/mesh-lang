# S04: Thin cluster-proof consumer and truthful operator/docs rail

**Goal:** Finish M042 by making `cluster-proof` visibly a thin consumer of the runtime-native continuity API, preserving the one-image Docker/Fly operator rail, and aligning the repo’s runbooks/docs/verifiers with the runtime-owned continuity truth instead of the older app-authored continuity story.
**Demo:** After this: After this slice, cluster-proof is visibly just a thin consumer over the runtime-native continuity API, and the repo’s Docker/Fly/operator/docs surfaces truthfully show the runtime-owned capability instead of app-authored continuity machinery.

## Tasks
- [x] **T01: Split `cluster-proof` into shared placement, legacy probe, and runtime continuity modules without changing the keyed submit/status contract.** — Make the proof app visibly thin before touching operator scripts or docs.

## Why

`cluster-proof/work.mpl` already consumes the runtime-native continuity API, but the file still hides that fact by mixing legacy route proof, placement helpers, keyed submit/status HTTP translation, and work-execution plumbing in one module. S04 should leave a reader with an obvious seam: Mesh owns placement and HTTP adaptation, the runtime owns continuity state and recovery truth.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof` handler split across `main.mpl` and `work.mpl` | Fail closed at compile/test time; do not leave routes pointing at stale helpers. | Not applicable beyond ordinary test/build timeout; the refactor should not add polling. | Keep invalid runtime JSON on the existing parse-failure path instead of inventing fallback continuity state. |
| Runtime-owned `Continuity.submit/status/mark_completed` contract | Preserve the current API and log/status mapping exactly; do not widen or shadow it in Mesh code. | Keep the existing runtime timeout/error payload behavior. | Treat malformed continuity JSON as an explicit failure payload, not as a legacy-probe success path. |

## Load Profile

- **Shared resources**: `cluster-proof` route handlers, runtime continuity JSON parsing, and keyed-work log volume.
- **Per-operation cost**: one local module dispatch plus the existing runtime continuity calls; this task should stay structurally neutral at runtime.
- **10x breakpoint**: confusion and regressions show up first in handler wiring and log/status mismatches, not in raw performance.

## Negative Tests

- **Malformed inputs**: invalid submit JSON, blank payloads, malformed request keys, and invalid continuity JSON from the runtime parser seam.
- **Error paths**: rejected continuity records, duplicate same-key submits, conflict responses, and invalid target selection still map to the right HTTP payloads after the split.
- **Boundary conditions**: single-node fallback, deterministic multi-node placement, legacy `GET /work` probe behavior, and owner-loss submit downgrading still behave exactly as before.

## Steps

1. Split the legacy `GET /work` probe helpers from the keyed continuity submit/status helpers so the exported route handlers and worker helpers read as two distinct concerns.
2. Keep placement and runtime `Continuity.*` calls in Mesh, but do not move continuity semantics, dedupe, or recovery logic out of `mesh-rt`.
3. Update `cluster-proof/main.mpl` and `cluster-proof/tests/work.test.mpl` so legacy probe and keyed continuity seams are exercised separately.
4. Preserve the current log/status surfaces and fail closed on any parser or handler mismatch.

## Must-Haves

- [ ] `cluster-proof` code reads as placement plus `Continuity.*` adaptation, not as app-authored continuity orchestration.
- [ ] Legacy `GET /work` remains available but clearly isolated from keyed submit/status code.
- [ ] No new Mesh-side continuity state machine or recovery shim is introduced.
- [ ] `cluster-proof` tests and build stay green after the refactor.
  - Estimate: 90m
  - Files: cluster-proof/work.mpl, cluster-proof/main.mpl, cluster-proof/tests/work.test.mpl, cluster-proof/WorkLegacy.mpl, cluster-proof/WorkContinuity.mpl
  - Verify: cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof
- [x] **T02: Added the M042 packaged operator verifier scripts and read-only Fly help rail, but authoritative local replay is still blocked by inherited M039/M042 verifier regressions.** — Close the operator proof rail without rewriting M039 history.

## Why

S03 already proved the runtime-owned owner-loss contract locally, but the packaged operator rail still only proves the older M039 routing story. This task should add M042 wrappers that reuse the same one-image Docker/Fly path, replay the stable local destructive continuity authority, and keep the Fly lane read-only.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Repo-root Docker image and two-container local proof scripts | Fail the phase that drifted and archive the build/container logs; do not silently fall back to a partial proof. | Preserve phase reports and copied artifacts instead of hanging on container readiness or continuity polling. | Fail closed on malformed `/membership`, `GET /work`, `POST /work`, or `GET /work/:request_key` JSON. |
| Read-only Fly contract helpers | Keep help/syntax and validation local and fail before any live call when required inputs are absent. | Read-only live checks should time out with a named phase and artifact root. | Reject malformed config/status/probe payloads instead of weakening the Fly contract. |

## Load Profile

- **Shared resources**: Docker image cache, local container ports/network, `.tmp/m039-s04/...` and `.tmp/m042-s04/...` artifact roots, and any repo-local `cluster-proof` build outputs.
- **Per-operation cost**: one repo-root image build plus a small number of HTTP probes and artifact copies; the Fly wrapper should remain lightweight unless explicitly run live.
- **10x breakpoint**: container cleanup, phase-artifact clarity, and accidental script overlap fail before raw throughput does.

## Negative Tests

- **Malformed inputs**: missing Fly env, malformed base URLs, malformed keyed continuity JSON, and zero-test/empty-artifact regressions.
- **Error paths**: packaged cluster degrade/rejoin drift, keyed submit/status mismatch after the Docker packaging path, and stale help/live-contract wording that implies mutating Fly verification.
- **Boundary conditions**: repo-root Docker context, two-node convergence, legacy routing still remote under health, keyed continuity status remains truthful after packaged submit, and `--help` remains the safe non-live Fly path.

## Steps

1. Reuse or extend the existing `scripts/lib/m039_cluster_proof.sh` helpers so M042 can assert packaged keyed submit/status payloads without duplicating the baseline membership/work checks.
2. Add `scripts/verify-m042-s04.sh` as the authoritative local packaged operator wrapper: replay the stable S03 local continuity rail, build the repo-root image, stand up the same one-image two-container runtime, and archive `/membership`, `GET /work`, `POST /work`, and `GET /work/:request_key` proof artifacts.
3. Add `scripts/verify-m042-s04-fly.sh` as a read-only wrapper/help contract that keeps the live Fly lane honest about what it can inspect without mutating remote state.
4. Keep `scripts/verify-m039-s04*.sh` replayable as the validated baseline instead of overwriting their historical scope.

## Must-Haves

- [ ] The packaged local wrapper proves runtime-owned keyed continuity through the one-image Docker path without claiming exactly-once semantics.
- [ ] The Fly lane remains read-only and explicitly scoped as sanity/config/log/probe truth, not destructive recovery authority.
- [ ] M039 baseline verifiers still run as the prior validated rail.
- [ ] The new M042 artifact root makes the first failing phase obvious from logs and JSON alone.
  - Estimate: 2h
  - Files: scripts/lib/m039_cluster_proof.sh, scripts/lib/m042_cluster_proof.sh, scripts/verify-m039-s04.sh, scripts/verify-m039-s04-fly.sh, scripts/verify-m042-s03.sh, scripts/verify-m042-s04.sh, scripts/verify-m042-s04-fly.sh, cluster-proof/Dockerfile
  - Verify: bash scripts/verify-m039-s04.sh && bash scripts/verify-m042-s04.sh && bash scripts/verify-m042-s04-fly.sh --help
  - Blocker: `bash scripts/verify-m039-s04.sh` still fails in the inherited `e2e_m039_s02_routes_work_to_peer_and_logs_execution` path, where remote `/work` routing regresses and the peer crashes in `compiler/mesh-rt/src/string.rs:104:21`. `bash scripts/verify-m042-s03.sh` is not stable in the current checkout, so the default `bash scripts/verify-m042-s04.sh` path fails in its S03 prerequisite replay. Even with the debug skips enabled, the packaged keyed phase still hits `503 replica_required_unavailable` during remote-owner search after Docker bring-up.
- [x] **T03: Confirmed the remaining T03 blockers: remote `Node.spawn` string-arg transport still breaks the inherited M039 legacy `/work` path, and remote-owner keyed submits still reject with `replica_required_unavailable` while `verify-m042-s03.sh` now passes.** — 1. Reproduce the three blocker paths separately: the inherited M039 remote `/work` routing crash, the unstable `verify-m042-s03.sh` replay, and the packaged Docker keyed phase returning `503 replica_required_unavailable` after bring-up.
2. Trace those failures to their real shared seam (runtime continuity admission/replica truth, cluster-proof placement/submit wiring, or verifier assumptions) and fix the root causes without widening the Mesh-facing `Continuity.*` contract or rewriting M039 history.
3. Tighten the M039/M042 helper and wrapper scripts only where the truthful contract changed or an old prerequisite assumption was wrong; keep fail-closed phase, zero-test, and malformed-artifact behavior.
4. End with a green local replay that proves packaged keyed continuity through the one-image two-container rail and preserves the read-only Fly wrapper/help contract.
  - Estimate: 3h
  - Files: compiler/mesh-rt/src/string.rs, cluster-proof/main.mpl, cluster-proof/work.mpl, scripts/lib/m039_cluster_proof.sh, scripts/lib/m042_cluster_proof.sh, scripts/verify-m039-s04.sh, scripts/verify-m042-s03.sh, scripts/verify-m042-s04.sh
  - Verify: bash scripts/verify-m039-s04.sh && bash scripts/verify-m042-s03.sh && bash scripts/verify-m042-s04.sh && bash scripts/verify-m042-s04-fly.sh --help
  - Blocker: `compiler/mesh-rt/src/dist/node.rs` still handles remote spawn args as raw bytes/pointers, which leaves the inherited M039 remote `/work` proof red when `execute_work(request_key, attempt_id)` is spawned on a peer. `cluster-proof/work_continuity.mpl` still submits remote-owner keyed work from the ingress node, and the runtime continuity prepare path treats `replica_node == self` as unavailable, which keeps remote-owner keyed submits red in both the non-Docker and packaged rails. `compiler/meshc/tests/e2e_m039_s02.rs` and `compiler/meshc/tests/e2e_m042_s01.rs` are currently failing against local reality for the reasons above.
- [x] **T04: Rewrote the distributed proof runbook and public docs around the runtime-owned continuity contract, added the M042 proof-surface verifier, and aligned the Fly help rail with the same local-authority story.** — 1. Update `cluster-proof/README.md` to document the thin-consumer shape, the legacy `GET /work` probe versus keyed `POST /work` / `GET /work/:request_key`, the small env contract, and the local-authority versus read-only-Fly proof split.
2. Update `website/docs/docs/distributed-proof/index.md`, `website/docs/docs/distributed/index.md`, and repo-level entry points so the public story matches the verified M042 rail, including the runtime `Continuity` API semantics and the authoritative command set from T03.
3. Add or update the proof-surface verifier and VitePress wiring so the runbook, proof page, distributed guide, and README mechanically agree on links, commands, and wording, with no exactly-once or process-state-migration claims.
4. Build the docs serially after the proof-surface verifier passes.
  - Estimate: 90m
  - Files: cluster-proof/README.md, website/docs/docs/distributed-proof/index.md, website/docs/docs/distributed/index.md, README.md, website/docs/.vitepress/config.mts, scripts/verify-m042-s04-proof-surface.sh
  - Verify: bash scripts/verify-m042-s04-proof-surface.sh && bash scripts/verify-m042-s04-fly.sh --help && npm --prefix website run build
