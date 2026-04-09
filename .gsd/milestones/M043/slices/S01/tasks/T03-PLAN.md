---
estimated_steps: 4
estimated_files: 3
skills_used:
  - test
  - debug-like-expert
---

# T03: Add destructive primary→standby proof harness and fail-closed verifier

**Slice:** S01 — Primary→Standby Runtime Replication and Role Truth
**Milestone:** M043

## Description

Prove the slice on a real multi-cluster path. This task adds the first M043 destructive proof rail: boot a primary cluster and a standby cluster, submit keyed work on the primary, wait for mirrored standby truth to appear, and archive the raw JSON and per-node logs that explain what happened if replication drifts.

The verifier should stay local and destructive, like the M042 rail, but its contract is different: S01 proves live standby mirroring and authority/status truth before promotion. It must not accidentally claim that failover already works.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Multi-cluster `cluster-proof` harness in `compiler/meshc/tests/e2e_m043_s01.rs` | Capture stdout/stderr and fail with the named readiness or mirror phase that did not converge. | Preserve pre-timeout artifacts and fail closed instead of hanging or skipping the hard part. | Archive raw HTTP bodies and fail when membership or keyed status payloads are not valid JSON. |
| Shared assertion helpers in `scripts/lib/m043_cluster_proof.sh` | Stop at the first contract drift and keep the raw artifact paths in the failure output. | Report which phase never converged and retain the last fetched payloads. | Reject missing fields or wrong role/epoch/health shapes instead of treating partial JSON as success. |
| Wrapper verifier in `scripts/verify-m043-s01.sh` | Replay prerequisites before the slice-specific proof and stop on the first failing phase. | Fail if any named test filter runs 0 tests or if required artifacts are missing. | Reject malformed phase manifests or copied logs instead of claiming a green replay. |

## Load Profile

- **Shared resources**: ephemeral ports, multiple `cluster-proof` processes, `.tmp/m043-s01/...` artifact directories, and the repo-local `cluster-proof` build output.
- **Per-operation cost**: one `mesh-rt`/`cluster-proof` build reuse plus a small number of destructive multi-cluster scenarios with HTTP polling and retained artifact copies.
- **10x breakpoint**: port cleanup, process teardown, and replication convergence polling will flake before raw CPU usage does.

## Negative Tests

- **Malformed inputs**: malformed membership or keyed-status JSON, missing role/epoch/health fields, and verifier manifests with missing copied artifacts.
- **Error paths**: standby never receives mirrored truth, replication health degrades before the mirror is visible, and named test filters run 0 tests.
- **Boundary conditions**: epoch `0` mirrored standby truth, healthy primary submit with no promotion signal, and repeated status polling against both clusters.

## Steps

1. Create `compiler/meshc/tests/e2e_m043_s01.rs` by reusing the stable process/artifact patterns from the M042 continuity harness while booting separate primary and standby clusters.
2. Add named scenarios that prove primary submit plus standby-side mirrored status, including role, promotion epoch, and replication-health assertions on both membership and keyed continuity surfaces.
3. Write `scripts/lib/m043_cluster_proof.sh` helpers for raw JSON assertions and artifact copying that match the new S01 contract.
4. Write `scripts/verify-m043-s01.sh` so it replays the runtime/package prerequisites, runs the named e2e target, fails closed on zero-test filters or missing artifacts, and preserves `.tmp/m043-s01/verify/` evidence for future slices.

## Must-Haves

- [ ] The repo contains a named `compiler/meshc/tests/e2e_m043_s01.rs` target that proves mirrored standby truth before any promotion work exists.
- [ ] The verifier explicitly checks role, promotion epoch, and replication-health fields instead of inferring them from generic success responses.
- [ ] `scripts/verify-m043-s01.sh` fail-closes on zero-test filters, malformed payloads, and missing retained artifacts.
- [ ] The retained evidence makes the first failing mirror/health phase obvious from JSON and logs alone.

## Verification

- `cargo test -p meshc --test e2e_m043_s01 -- --nocapture`
- `bash scripts/verify-m043-s01.sh`

## Observability Impact

- Signals added/changed: phase reports, copied raw HTTP responses, and per-node stdout/stderr artifacts for primary and standby clusters.
- How a future agent inspects this: read `.tmp/m043-s01/verify/phase-report.txt` plus the retained membership/status JSON and node logs.
- Failure state exposed: missing mirror convergence, wrong role/epoch/health payloads, and harness cleanup/readiness failures are preserved without rerunning the slice.

## Inputs

- `compiler/meshc/tests/e2e_m042_s03.rs` — prior destructive continuity harness patterns for process control, polling, and retained artifacts.
- `scripts/lib/m042_cluster_proof.sh` — current keyed payload/status assertion helpers to adapt for the S01 contract.
- `scripts/verify-m042-s03.sh` — fail-closed wrapper structure and phase-report discipline.
- `compiler/mesh-rt/src/dist/continuity.rs` — new primary/standby authority metadata from T01.
- `cluster-proof/main.mpl` — operator-visible membership and keyed-status endpoints from T02.
- `cluster-proof/work_continuity.mpl` — thin continuity consumer used by the keyed status surface.

## Expected Output

- `compiler/meshc/tests/e2e_m043_s01.rs` — multi-cluster e2e proof for primary submit and mirrored standby truth.
- `scripts/lib/m043_cluster_proof.sh` — reusable JSON/assertion helpers for M043 retained artifacts.
- `scripts/verify-m043-s01.sh` — fail-closed local verifier that replays prerequisites and archives `.tmp/m043-s01/verify/` evidence.
