# S03: Single-Cluster Failure, Safe Degrade, and Rejoin

**Goal:** Prove single-cluster continuity on `cluster-proof`: after one node dies the survivor reports truthful self-only membership and still serves new `/work`, and when the same node identity restarts the cluster reconverges and remote routing resumes without manual repair.
**Demo:** After this: After this: killing and restarting a node shows safe degrade, truthful membership updates, continued service for new work, and clean rejoin without manual repair.

## Tasks
- [x] **T01: Added ingress-owned request correlation and live S03 degrade/rejoin continuity proofs for cluster-proof.** — Add the live S03 proof surface by making repeated `/work` calls distinguishable across one cluster lifetime and by creating a restart-safe Rust harness that preserves pre-loss, degraded, and post-rejoin evidence without clobbering node logs.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof` child processes and mesh runtime discovery | stop the phase, keep stdout/stderr paths in the panic, and preserve the phase artifact dir for postmortem | kill the child after the bounded wait, keep the partial artifact dir, and fail the phase with context | treat unexpected early exits or inconsistent node names as a proof failure rather than synthesizing continuity |
| `/membership` and `/work` HTTP endpoints | retry until the phase deadline, then fail with the last raw response or socket error | fail the phase after the bounded wait and keep the last raw body plus log paths | write the raw body to the phase artifact file and fail on the missing field immediately |

## Load Profile

- **Shared resources**: two child processes, dual-stack cluster ports, per-phase artifact directories, and copied stdout/stderr logs.
- **Per-operation cost**: one build/test prerequisite, repeated membership polls, and three `/work` calls in the rejoin lifetime.
- **10x breakpoint**: artifact/log collisions and longer convergence waits before CPU; the harness must isolate incarnations and keep bounded polling.

## Negative Tests

- **Malformed inputs**: missing `self`, `membership`, `request_id`, or `execution_node` fields must fail the harness with the raw body path preserved.
- **Error paths**: dead-peer degrade, reconnect lag, and unexpected early child exit must fail with per-incarnation logs rather than hanging.
- **Boundary conditions**: self-only membership after loss, same-identity restart, and remote route truth before loss plus after rejoin.

## Steps

1. Replace the hardcoded request token in `cluster-proof/work.mpl` with a narrow monotonic/token generator so multiple `/work` calls in one cluster lifetime emit distinct `request_id` values without widening the HTTP contract.
2. Update `cluster-proof/tests/work.test.mpl` and the request-id expectation in `compiler/meshc/tests/e2e_m039_s02.rs` so request correlation stays deterministic but no longer assumes every call is `work-0`.
3. Add `compiler/meshc/tests/e2e_m039_s03.rs` using the S01/S02 spawn/kill/membership patterns, but preserve phase-specific work artifacts and incarnation-specific node log filenames so restart evidence survives.
4. Prove `e2e_m039_s03_degrades_safely_and_serves_locally_after_peer_loss` and `e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair` against real child processes, same-identity restarts, and eventual convergence rather than instant reconnect.
5. If the live proof exposes a real reconnect defect, repair only the smallest proof-blocking seam instead of widening `cluster-proof` into a coordinator or operator abstraction.

## Must-Haves

- [ ] Multiple `/work` calls in one cluster lifetime produce distinct `request_id` values that the logs and artifacts can correlate.
- [ ] The S03 harness leaves durable pre-loss, degraded, and post-rejoin evidence without overwriting the first crashed-node logs.
- [ ] The two named S03 tests prove truthful membership shrinkage/rejoin and truthful routing fallback/recovery on a real two-node cluster.
  - Estimate: 2h
  - Files: cluster-proof/work.mpl, cluster-proof/tests/work.test.mpl, compiler/meshc/tests/e2e_m039_s02.rs, compiler/meshc/tests/e2e_m039_s03.rs
  - Verify: cargo run -q -p meshc -- test cluster-proof/tests
cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_falls_back_locally_without_peers -- --nocapture
cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_degrades_safely_and_serves_locally_after_peer_loss -- --nocapture
cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair -- --nocapture
- [x] **T02: Added the fail-closed S03 continuity verifier with copied degrade/rejoin evidence manifests.** — Add the canonical local S03 acceptance wrapper so the distributed continuity story replays from known-good prerequisites and fails closed with a stable evidence bundle that later agents can inspect without rerunning a long multi-node proof.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m039-s01.sh` / `scripts/verify-m039-s02.sh` prerequisite replays | stop immediately, copy the failing phase log, and refuse to claim S03 passed on a broken base contract | fail the phase with the prerequisite log excerpt and current-phase marker | treat missing phase-report or zero-test evidence as verifier drift, not as a flaky prerequisite |
| named `e2e_m039_s03` cargo filters and copied artifact dirs | fail closed with the exact cargo log and missing artifact path | kill the command after the bounded wait, preserve the partial `.tmp/m039-s03` tree, and mark the phase failed | reject zero-test filters, missing phase manifests, or malformed copied JSON/log bundles |

## Load Profile

- **Shared resources**: multiple cargo invocations, copied `.tmp/m039-s03` directories, and per-phase log files.
- **Per-operation cost**: one test pass, one build, two prerequisite verifier replays, two named S03 test invocations, and artifact-copy validation.
- **10x breakpoint**: wall-clock time and artifact sprawl before CPU; the wrapper must bound timeouts and only copy the new S03 phase directories it created.

## Negative Tests

- **Malformed inputs**: missing `phase-report.txt`, `status.txt`, `current-phase.txt`, or per-phase artifact files must fail the wrapper.
- **Error paths**: a named cargo filter that runs 0 tests, prerequisite verifier drift, or missing copied node logs must mark the phase failed.
- **Boundary conditions**: separate pre-loss/degraded/post-rejoin artifact groups, repeated reruns against an existing `.tmp/m039-s03/verify`, and phase-specific timeout handling.

## Steps

1. Add `scripts/verify-m039-s03.sh` in the S02 wrapper pattern with `status.txt`, `current-phase.txt`, `phase-report.txt`, bounded `cargo` timeouts, and full-contract logging under `.tmp/m039-s03/verify/`.
2. Replay `cluster-proof/tests`, `meshc build cluster-proof`, `scripts/verify-m039-s01.sh`, and `scripts/verify-m039-s02.sh` before any new S03 checks so continuity proof cannot hide earlier regression.
3. Run the two named `e2e_m039_s03` filters with non-zero test-count checks and copy stable pre-loss/degraded/post-rejoin artifacts plus per-incarnation logs/manifests into `.tmp/m039-s03/verify/`.
4. Fail closed on missing prerequisite phase reports, zero-test filters, missing artifacts, or malformed copied evidence, and finish only when `bash scripts/verify-m039-s03.sh` succeeds from a clean run.

## Must-Haves

- [ ] `scripts/verify-m039-s03.sh` is the authoritative local S03 replay surface and refuses false-green runs when a named filter drifts or a prerequisite contract is broken.
- [ ] `.tmp/m039-s03/verify/` preserves phase-by-phase logs, manifests, and copied per-incarnation node evidence for degrade and rejoin debugging.
- [ ] A passing wrapper proves the full local chain: proof app tests, build, S01, S02, and both S03 continuity filters.
  - Estimate: 1h
  - Files: scripts/verify-m039-s03.sh, scripts/verify-m039-s01.sh, scripts/verify-m039-s02.sh, compiler/meshc/tests/e2e_m039_s03.rs
  - Verify: bash scripts/verify-m039-s03.sh
