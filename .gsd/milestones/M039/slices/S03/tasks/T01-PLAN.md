---
estimated_steps: 5
estimated_files: 4
skills_used:
  - test
  - debug-like-expert
---

# T01: Add restart-safe request correlation and live continuity e2e coverage

**Slice:** S03 — Single-Cluster Failure, Safe Degrade, and Rejoin
**Milestone:** M039

## Description

Add the live S03 continuity proof surface by making repeated `/work` calls distinguishable across one cluster lifetime and by creating a restart-safe Rust harness that preserves pre-loss, degraded, and post-rejoin evidence without clobbering node logs.

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

## Verification

- `cargo run -q -p meshc -- test cluster-proof/tests`
- `cargo test -p meshc --test e2e_m039_s02 e2e_m039_s02_falls_back_locally_without_peers -- --nocapture`
- `cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_degrades_safely_and_serves_locally_after_peer_loss -- --nocapture`
- `cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair -- --nocapture`

## Observability Impact

- Signals added/changed: distinct `request_id` values across repeated `/work` calls plus phase-specific and incarnation-specific artifact/log names.
- How a future agent inspects this: read `.tmp/m039-s03/...` work JSON files and the matching `node-*-runN.{stdout,stderr}.log` files.
- Failure state exposed: whether the proof failed during pre-loss routing, degraded fallback, membership reconvergence, or post-rejoin routing, and which request/node incarnation was involved.

## Inputs

- `cluster-proof/work.mpl` — existing routing and request-correlation helpers.
- `cluster-proof/tests/work.test.mpl` — current pure routing tests.
- `compiler/meshc/tests/e2e_m039_s01.rs` — existing node-loss harness patterns and child-process lifecycle helpers.
- `compiler/meshc/tests/e2e_m039_s02.rs` — existing routing harness patterns and current `request_id` expectation.

## Expected Output

- `cluster-proof/work.mpl` — restart-safe request-token generation for repeated `/work` calls.
- `cluster-proof/tests/work.test.mpl` — updated correlation helper coverage.
- `compiler/meshc/tests/e2e_m039_s02.rs` — S02 expectation aligned with non-constant request ids.
- `compiler/meshc/tests/e2e_m039_s03.rs` — new degrade/rejoin continuity proofs with stable artifacts.
