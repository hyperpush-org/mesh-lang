---
estimated_steps: 24
estimated_files: 3
skills_used:
  - test
  - debug-like-expert
---

# T03: Add the S02 e2e harness and fail-closed verifier for rejected, mirrored, and degraded continuity truth

This slice needs its own proof rail because `scripts/verify-m042-s01.sh` still depends on the unrelated remote-owner completion crash. The new harness must prove the S02 contract only on stable paths and preserve enough artifacts to debug regressions later.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof` process startup / membership convergence in the Rust harness | Capture stdout/stderr and fail with the named readiness condition that was not met. | Time out with preserved artifacts instead of hanging or silently skipping tests. | Treat non-JSON HTTP bodies as contract failures and archive the raw response. |
| Slice verifier wrapper | Fail closed on the first missing proof phase or zero-test run; do not reuse the stale S01 full verifier. | Surface which phase stalled and preserve the partial artifact directory. | Reject malformed archived JSON / logs rather than claiming the slice is green. |

## Load Profile

- **Shared resources**: Ephemeral ports, spawned `cluster-proof` processes, `.tmp/m042-s02/...` artifact directories, and the repo-local `cluster-proof` build output.
- **Per-operation cost**: Building `mesh-rt` / `cluster-proof` once, then a small number of live HTTP polls per e2e case.
- **10x breakpoint**: Port allocation and process cleanup flake first; the harness must keep setup/teardown deterministic and archive enough logs to diagnose races.

## Negative Tests

- **Malformed inputs**: Missing or invalid request key body, non-JSON HTTP response, and zero-test command filters.
- **Error paths**: Single-node cluster-mode replica-backed submit rejected with stored status, replica node killed after mirrored admission, and harness timeout before status transitions.
- **Boundary conditions**: Local-owner with non-empty replica selection, duplicate retry of rejected record, and pending mirrored work observed before completion.

## Steps

1. Create `compiler/meshc/tests/e2e_m042_s02.rs` by reusing only the stable process / HTTP / artifact helpers from S01, not the remote-owner completion proof itself.
2. Add named e2e cases for single-node cluster-mode rejection, two-node local-owner/remote-replica mirrored admission, and post-loss `degraded_continuing` status.
3. Write `scripts/verify-m042-s02.sh` so it replays runtime tests, `cluster-proof` tests, the S01 standalone regression, and the new S02 target without calling the known-failing full S01 verifier.
4. Make the verifier fail closed on missing `running N test` evidence or missing artifact files so a skipped test target cannot look green.

## Must-Haves

- [ ] The repo contains a named `compiler/meshc/tests/e2e_m042_s02.rs` target covering rejected, mirrored, and degraded status truth.
- [ ] The new verifier replays only stable prerequisite proofs and stops before the unrelated remote-owner completion blocker.
- [ ] The harness archives per-node logs and response JSON under `.tmp/m042-s02/...` for later diagnosis.
- [ ] The verification surface checks that the intended tests actually ran, not just that Cargo exited 0.

## Inputs

- ``compiler/meshc/tests/e2e_m042_s01.rs``
- ``scripts/verify-m042-s01.sh``
- ``cluster-proof/work.mpl``
- ``compiler/mesh-rt/src/dist/continuity.rs``
- ``compiler/mesh-rt/src/dist/node.rs``

## Expected Output

- ``compiler/meshc/tests/e2e_m042_s02.rs``
- ``compiler/meshc/tests/e2e_m042_s01.rs``
- ``scripts/verify-m042-s02.sh``

## Verification

cargo test -p meshc --test e2e_m042_s02 -- --nocapture && bash scripts/verify-m042-s02.sh

## Observability Impact

Adds the canonical S02 artifact root and fail-closed verifier so later slices can inspect rejection, mirrored admission, and replica-loss degradation without rediscovering the stable harness path.
