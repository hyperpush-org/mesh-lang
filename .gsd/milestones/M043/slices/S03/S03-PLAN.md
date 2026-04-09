# S03: Same-Image Two-Cluster Operator Rail

**Goal:** Turn the shipped M043 failover contract into a repeatable one-image operator rail: same `cluster-proof` image, hostname-derived primary/standby identities, explicit promotion, retained Docker evidence, and no app-authored disaster-control logic.
**Demo:** After this: Using the same cluster-proof image and a small env surface, an operator can launch a primary cluster and standby cluster locally, run the packaged destructive failover verifier, and get retained artifacts that show replication, promotion, and fenced rejoin truth.

## Tasks
- [x] **T01: Added the same-image Docker failover e2e with retained artifacts and stale-primary fencing checks.** — Implement the new M043 packaged regression where the `cluster-proof` binary runs from one Docker image in two roles and the operator truth is observed through runtime-owned HTTP/log surfaces, not shell-authored state.

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
  - Estimate: 3h
  - Files: compiler/meshc/tests/e2e_m043_s03.rs, compiler/meshc/tests/e2e_m043_s01.rs, compiler/meshc/tests/e2e_m043_s02.rs, scripts/lib/m043_cluster_proof.sh, cluster-proof/Dockerfile, cluster-proof/docker-entrypoint.sh
  - Verify: cargo test -p meshc --test e2e_m043_s03 -- --nocapture
- [x] **T02: Added the fail-closed packaged same-image verifier that replays S02 and validates copied Docker failover artifacts from runtime-owned JSON and logs.** — Wrap the new same-image harness in the established M043 closeout pattern: replay the prior authority rails first, then validate copied Docker artifacts instead of trusting an exit code.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m043-s02.sh` and prerequisite cargo commands | Stop immediately and leave the failing prerequisite log in `.tmp/m043-s03/verify/`. | Mark the phase as failed and preserve the partial log for replay diagnosis. | N/A |
| Copied same-image artifact bundle | Fail closed on the first missing or empty required file and report the manifest/log path. | N/A | Reject malformed copied JSON or missing required keys before claiming the packaged contract passed. |

## Load Profile

- **Shared resources**: Cargo build cache, Docker image cache, local bridge network, and `.tmp/` artifact storage.
- **Per-operation cost**: prerequisite test/build replay plus one destructive same-image Docker run and artifact copy.
- **10x breakpoint**: build time, disk usage, and Docker cache churn fail before the failover assertions become logically different.

## Negative Tests

- **Malformed inputs**: copied manifests with missing `scenario-meta.json`, phase JSON, or container logs must fail verification immediately.
- **Error paths**: missing `running N test` evidence, a failing prerequisite verifier, or malformed retained JSON must stop the wrapper with the right artifact hint.
- **Boundary conditions**: the wrapper must fail if the old primary logs completion/execution of the promoted attempt or if the promoted standby never reaches epoch `1` authority truth.

## Steps

1. Create `scripts/verify-m043-s03.sh` using the S02 wrapper structure and the M042 Docker/artifact pattern where that still fits.
2. Replay `cargo test -p mesh-rt continuity -- --nocapture`, `cargo run -q -p meshc -- test cluster-proof/tests`, `cargo run -q -p meshc -- build cluster-proof`, and `bash scripts/verify-m043-s02.sh` before the new `cargo test -p meshc --test e2e_m043_s03 -- --nocapture` phase.
3. Copy the selected same-image artifact directory into `.tmp/m043-s03/verify/` and assert pre-failover, degraded, promoted, recovery-rollover, completion, and fenced-rejoin truth from preserved JSON/logs/manifests.
4. Emit `phase-report.txt`, `status.txt`, `current-phase.txt`, and `full-contract.log`, and fail when any copied artifact is missing, empty, malformed, or inconsistent with the runtime-owned truth.

## Must-Haves

- [ ] The wrapper reuses runtime-owned role/epoch/health truth and does not derive live authority from env after startup.
- [ ] Verification is based on copied artifacts and explicit JSON/log assertions, not just on a green cargo test exit code.
- [ ] `.tmp/m043-s03/verify/` contains enough retained manifests/logs/JSON to debug a failed packaged run without rerunning it.
  - Estimate: 2h
  - Files: scripts/verify-m043-s03.sh, scripts/lib/m043_cluster_proof.sh, scripts/verify-m043-s02.sh, scripts/verify-m042-s04.sh, compiler/meshc/tests/e2e_m043_s03.rs
  - Verify: bash scripts/verify-m043-s03.sh
- [x] **T03: Made the same-image entrypoint fail closed on bad continuity env and added a packaged misconfiguration proof.** — Make the same-image operator rail fail closed at the smallest honest boundary: valid primary/standby/stale-primary env should keep working, while contradictory continuity role/epoch input should fail before ambiguous runtime startup.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof` entrypoint/config startup | Emit a precise config error and stop before the package starts serving misleading operator surfaces. | N/A | Reject blank or contradictory role/epoch env instead of letting startup drift into a later failure. |
| `cluster-proof/tests/config.test.mpl` regression suite | Treat any failure as proof that the small-env contract changed unexpectedly and stop before reusing the verifier. | N/A | N/A |

## Negative Tests

- **Malformed inputs**: blank role, invalid role, blank epoch, non-integer epoch, and standby-with-epoch-1 startup must fail closed.
- **Error paths**: partial cluster identity env and contradictory continuity env must report a concrete config error without leaking secrets.
- **Boundary conditions**: valid primary, standby, and stale-primary restart env must still boot unchanged so the packaged verifier stays honest.

## Steps

1. Tighten `cluster-proof/docker-entrypoint.sh` so same-image cluster mode rejects missing or contradictory `MESH_CONTINUITY_ROLE` / `MESH_CONTINUITY_PROMOTION_EPOCH` combinations early while preserving standalone mode and the HOSTNAME/Fly identity fallback.
2. Update `cluster-proof/tests/config.test.mpl` (and `cluster-proof/config.mpl` only if needed) so valid primary/standby/stale-primary env and invalid role/epoch combinations stay covered.
3. Keep `scripts/verify-m043-s03.sh` aligned with the improved startup failure surface so operator misconfiguration points to the right log/artifact path without logging `CLUSTER_PROOF_COOKIE`.

## Must-Haves

- [ ] Valid primary, standby, and stale-primary restart flows still boot unchanged.
- [ ] Invalid continuity role/epoch env fails before ambiguous runtime startup.
- [ ] No verifier or entrypoint path prints `CLUSTER_PROOF_COOKIE` or similar secrets.
  - Estimate: 90m
  - Files: cluster-proof/docker-entrypoint.sh, cluster-proof/tests/config.test.mpl, cluster-proof/config.mpl, scripts/verify-m043-s03.sh
  - Verify: cargo run -q -p meshc -- test cluster-proof/tests && bash scripts/verify-m043-s03.sh
