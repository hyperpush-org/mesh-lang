---
estimated_steps: 24
estimated_files: 6
skills_used:
  - multi-stage-dockerfile
---

# T01: Build the same-image Docker failover harness

Implement the new M043 packaged regression where the `cluster-proof` binary runs from one Docker image in two roles and the operator truth is observed through runtime-owned HTTP/log surfaces, not shell-authored state.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Docker daemon / image build | Fail closed with the captured command log and any container inspect output already produced. | Stop containers, archive partial stdout/stderr, and mark the timed-out phase in the artifact bundle. | N/A |
| `cluster-proof` HTTP surfaces | Archive the raw HTTP response for the failing phase and stop the scenario. | Kill the container pair, retain logs, and report which phase never converged. | Reject missing or mistyped authority fields before continuing to the next phase. |

## Load Profile

- **Shared resources**: Docker daemon, local bridge network, ephemeral host ports, and the repo-local image cache.
- **Per-operation cost**: one image build plus a two-container scenario with repeated `/membership`, `/promote`, and `/work/:request_key` reads.
- **10x breakpoint**: image rebuild time and local Docker/network resource exhaustion fail before runtime continuity logic changes.

## Negative Tests

- **Malformed inputs**: request-key candidates that do not place owner=`primary` and replica=`standby` must be rejected instead of being treated as proof.
- **Error paths**: missing authority fields or malformed JSON from `/membership` or `/work/:request_key` must fail the harness with retained raw responses.
- **Boundary conditions**: restarting the old primary on its stale `primary` / epoch `0` env must stay in the proof, and the harness must fail if that node executes or completes the promoted attempt.

## Steps

1. Add `compiler/meshc/tests/e2e_m043_s03.rs` with Docker lifecycle helpers that build the repo-root image, start `primary` and `standby` containers on one bridge network, and assert hostname-derived node names.
2. Reuse or extract the M043 placement/artifact logic so the harness can deterministically choose a request key whose owner is `primary` and replica is `standby`, then retain `scenario-meta.json`, raw HTTP, and stdout/stderr logs.
3. Prove the destructive story in Docker: mirrored pending truth, primary kill, degraded standby truth, `/promote`, retry rollover, completion on promoted standby, and fenced stale-primary rejoin.
4. Keep the harness fail-closed on stale-primary execution/completion after rejoin and on any missing retained artifact that later verifier phases depend on.

## Must-Haves

- [ ] The same repo-root image serves both roles; do not introduce a role-specific image or manual peer list.
- [ ] Assertions use Docker hostname identities (`primary@primary:4370`, `standby@standby:4370`-style names), not the loopback names from the compiler-only harness.
- [ ] The old primary restarts with its original stale env instead of a repaired standby config.
- [ ] The artifact directory is non-empty and contains scenario metadata plus per-phase JSON and container logs.

## Inputs

- `compiler/meshc/tests/e2e_m043_s01.rs`
- `compiler/meshc/tests/e2e_m043_s02.rs`
- `scripts/lib/m043_cluster_proof.sh`
- `cluster-proof/Dockerfile`
- `cluster-proof/docker-entrypoint.sh`

## Expected Output

- `compiler/meshc/tests/e2e_m043_s03.rs`
- `scripts/lib/m043_cluster_proof.sh`
- `.tmp/m043-s03/continuity-api-same-image-failover/scenario-meta.json`

## Verification

cargo test -p meshc --test e2e_m043_s03 -- --nocapture

## Observability Impact

Adds a retained same-image artifact directory with `scenario-meta.json`, raw HTTP snapshots, and per-container stdout/stderr logs so later verifier phases can diagnose placement drift, promotion drift, or stale-primary execution.
