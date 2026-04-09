---
estimated_steps: 24
estimated_files: 2
skills_used:
  - test
  - debug-like-expert
---

# T03: Add the S03 owner-loss e2e harness and fail-closed verifier rail

Finish the slice with a destructive proof rail that exercises the real owner-loss contract. Reuse the stable local-owner placement search from S02 and the kill/restart artifact discipline from M039/S03 so the proof stays about continuity recovery, not the unrelated remote-spawn crash.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Spawned `cluster-proof` processes and membership convergence in the Rust harness | Capture stdout/stderr and fail with the named readiness or phase that did not converge. | Time out with preserved pre-loss/degraded/retry/rejoin artifacts instead of hanging or silently skipping the hard part. | Archive raw HTTP bodies and fail closed on non-JSON or invalid continuity payloads. |
| S03 verifier wrapper | Replay prerequisites before the slice-specific target and stop on the first drift. | Fail if any named test filter runs 0 tests or required artifact bundle is missing. | Reject malformed JSON manifests or copied logs instead of claiming the slice passed. |

## Load Profile

- **Shared resources**: Ephemeral ports, spawned `cluster-proof` processes, `.tmp/m042-s03/...` artifact directories, and the repo-local `cluster-proof` build output.
- **Per-operation cost**: Building `mesh-rt` / `cluster-proof` once, then running a small number of destructive two-node scenarios with HTTP polling and process restarts.
- **10x breakpoint**: Port reuse, process cleanup, and late stale-completion races will flake first; the harness must keep setup/teardown deterministic and archive enough evidence to debug the first failing phase.

## Negative Tests

- **Malformed inputs**: Non-JSON status responses, zero-test command filters, and malformed copied artifact manifests.
- **Error paths**: Owner loss while the request is pending, same-key retry on the surviving node, stale completion from the old attempt, and same-identity rejoin with stale replicated state.
- **Boundary conditions**: Stable local-owner selection before owner loss, rollover from one attempt to the next on the same request key, and rejoin after the retry has already completed.

## Steps

1. Create `compiler/meshc/tests/e2e_m042_s03.rs` by combining the stable HTTP/artifact helpers from `e2e_m042_s02.rs` with the kill/restart patterns from `compiler/meshc/tests/e2e_m039_s03.rs`.
2. Add named scenarios for surviving-node status after owner loss, same-key retry rolling a new `attempt_id`, stale-completion rejection, and same-identity rejoin preserving the newer attempt as authoritative.
3. Write `scripts/verify-m042-s03.sh` so it replays runtime continuity tests, `cluster-proof` tests, `bash scripts/verify-m042-s02.sh`, and the named `e2e_m042_s03` target while fail-closing on missing `running N test` evidence or missing artifact bundles.
4. Preserve copied proof bundles under `.tmp/m042-s03/verify/` for pre-loss, degraded, retry-rollover, stale-completion, and post-rejoin phases with per-node logs and raw HTTP captures.

## Must-Haves

- [ ] The repo contains a named `compiler/meshc/tests/e2e_m042_s03.rs` target proving owner-loss status serving, retry rollover, stale-completion safety, and rejoin truth.
- [ ] `scripts/verify-m042-s03.sh` replays the stable prerequisites, fails closed on zero-test filters, and archives copied evidence for each destructive phase.
- [ ] The proof stays on the stable local-owner rail instead of reopening the unrelated remote-owner execution crash.
- [ ] The preserved artifacts make the first failing phase obvious from logs and JSON alone.

## Inputs

- ``compiler/meshc/tests/e2e_m042_s02.rs``
- ``compiler/meshc/tests/e2e_m042_s01.rs``
- ``compiler/meshc/tests/e2e_m039_s03.rs``
- ``scripts/verify-m042-s02.sh``
- ``scripts/verify-m039-s03.sh``
- ``cluster-proof/work.mpl``
- ``compiler/mesh-rt/src/dist/continuity.rs``
- ``compiler/mesh-rt/src/dist/node.rs``

## Expected Output

- ``compiler/meshc/tests/e2e_m042_s03.rs``
- ``scripts/verify-m042-s03.sh``

## Verification

cargo test -p meshc --test e2e_m042_s03 -- --nocapture && bash scripts/verify-m042-s03.sh

## Observability Impact

Creates the canonical S03 artifact root and copied per-phase evidence so future agents can localize owner-loss, retry, stale-completion, or rejoin failures without rebuilding the harness by hand.
