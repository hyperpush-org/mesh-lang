---
estimated_steps: 23
estimated_files: 5
skills_used:
  - multi-stage-dockerfile
  - debug-like-expert
---

# T02: Assemble the fail-closed packaged verifier

Wrap the new same-image harness in the established M043 closeout pattern: replay the prior authority rails first, then validate copied Docker artifacts instead of trusting an exit code.

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

## Inputs

- `compiler/meshc/tests/e2e_m043_s03.rs`
- `scripts/lib/m043_cluster_proof.sh`
- `scripts/verify-m043-s02.sh`
- `scripts/verify-m042-s04.sh`

## Expected Output

- `scripts/verify-m043-s03.sh`
- `.tmp/m043-s03/verify/phase-report.txt`
- `.tmp/m043-s03/verify/full-contract.log`

## Verification

bash scripts/verify-m043-s03.sh

## Observability Impact

Introduces the canonical `.tmp/m043-s03/verify/` bundle with phase/state files, copied manifests, and log-backed assertions so future agents can localize whether drift came from prerequisites, Docker startup, promotion, or stale-primary fencing.
