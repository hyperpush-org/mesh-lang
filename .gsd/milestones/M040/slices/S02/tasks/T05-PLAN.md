---
estimated_steps: 4
estimated_files: 3
skills_used:
  - test
  - best-practices
---

# T05: Ship the slice-local verifier and evidence bundle for replica-backed continuity

**Slice:** S02 — Replica-Backed Admission, Fail-Closed Policy, and Owner-Loss Convergence
**Milestone:** M040

## Description

S02 needs a named operator-facing proof surface, but this slice should not absorb S03’s broader docs/Fly migration. Add a new repo-root verifier that replays the keyed baseline, exercises the real two-node replica-backed continuity flow, and archives the evidence future slices will diff instead of rediscovering.

Keep the helper story additive. Reuse patterns from the M039 verifier library if they fit, but prefer a new `scripts/lib/m040_cluster_proof.sh` helper over mutating the old M039 proof surface in ways that could blur historical evidence. The result should leave a stable `.tmp/m040-s02/verify/` bundle containing pre-loss, degraded, and post-owner-loss JSON plus the per-node logs needed to debug failures quickly.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m040-s02.sh` phase orchestration | fail the verifier immediately with a clear phase/report path | treat timeouts as red; do not silently continue to later phases | fail if JSON bodies or log files do not match the expected shape |
| `compiler/meshc/tests/e2e_m040_s02.rs` evidence contract | keep the script aligned with the live Rust proof instead of re-implementing different expectations | stop on the first failing phase and surface the preserved artifact directory | reject missing/partial artifact output instead of papering over it |
| helper library reuse (`scripts/lib/m040_cluster_proof.sh`) | keep helper changes additive so M039 proof surfaces stay stable | N/A — helper loading is synchronous | refuse silent helper drift that changes phase semantics without updating verifier assertions |

## Load Profile

- **Shared resources**: local cluster processes, verifier artifact directory, temporary ports, and phase-report/log writes.
- **Per-operation cost**: one clustered proof run with repeated HTTP polling, JSON captures, and per-node log copies.
- **10x breakpoint**: parallel verifier runs would mainly stress port allocation and artifact churn, not CPU.

## Negative Tests

- **Malformed inputs**: missing or malformed JSON artifact files, absent node logs, and incorrect status/rejection payloads.
- **Error paths**: degraded submit unexpectedly returns success, owner-loss continuation never reaches completion, and artifact-shape drift between the Rust e2e and shell verifier.
- **Boundary conditions**: pre-loss accepted request, degraded new submit rejection, and same-key continuation finishing exactly once after owner loss.

## Steps

1. Add `scripts/lib/m040_cluster_proof.sh` with only the helper logic needed for the new keyed continuity verifier.
2. Write `scripts/verify-m040-s02.sh` so it replays the keyed baseline, runs the two-node continuity flow, and records phase-by-phase JSON/log artifacts under `.tmp/m040-s02/verify/`.
3. Assert the artifact shape explicitly: pre-loss submit/status truth, degraded rejection, post-owner-loss continuation, and copied per-node stdout/stderr logs.
4. Re-run the verifier until it is a trustworthy named acceptance surface for the slice without rewriting the older M039 docs/Fly proofs.

## Must-Haves

- [ ] The verifier leaves a stable `.tmp/m040-s02/verify/` bundle with phase report, keyed JSON snapshots, and per-node logs.
- [ ] The verifier proves acceptance-after-replica-ack, degraded new-submit rejection, and same-key convergent completion after owner loss.
- [ ] Helper reuse stays additive; M039 proof surfaces are not silently repurposed into S02’s contract.

## Verification

- Run the new slice-local proof surface from repo root.
- `bash scripts/verify-m040-s02.sh`

## Observability Impact

- Signals added/changed: phase-report entries, verifier status/current-phase files, and copied node stdout/stderr logs for each proof phase.
- How a future agent inspects this: open `.tmp/m040-s02/verify/phase-report.txt`, the per-phase JSON artifacts, and the archived node logs.
- Failure state exposed: the failing phase, artifact path, and missing/incorrect evidence are all explicit instead of buried in one long shell log.

## Inputs

- `compiler/meshc/tests/e2e_m040_s02.rs` — live clustered continuity proof and artifact contract to mirror.
- `scripts/lib/m039_cluster_proof.sh` — existing verifier helper patterns to reuse or copy narrowly.
- `scripts/verify-m040-s01.sh` — baseline keyed verifier to replay before the clustered continuity phases.

## Expected Output

- `scripts/lib/m040_cluster_proof.sh` — additive helper library for the keyed continuity verifier.
- `scripts/verify-m040-s02.sh` — repo-root verifier for the new clustered continuity contract.
- `compiler/meshc/tests/e2e_m040_s02.rs` — any small evidence-path alignment updates needed so the Rust harness and shell verifier agree on artifact truth.
