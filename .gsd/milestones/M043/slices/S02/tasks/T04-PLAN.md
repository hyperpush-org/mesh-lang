---
estimated_steps: 4
estimated_files: 3
skills_used:
  - test
  - debug-like-expert
---

# T04: Add destructive failover and fenced-rejoin proof rails

**Slice:** S02 — Standby Promotion and Stale-Primary Fencing
**Milestone:** M043

## Description

Close the slice with real failover evidence. Extend the M043 harness and verifier helpers so the repo proves the full sequence: mirrored standby state exists, the primary dies, the standby is explicitly promoted, surviving keyed work completes through the promoted authority, and the restarted old primary stays fenced or deposed at the newer epoch.

The verifier must fail closed on named test counts, copied-artifact manifests, and stale proof drift. It should replay the S01 verifier and the targeted M042 rejoin regression before asserting the new S02 contract.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Multi-process failover harness in `compiler/meshc/tests/e2e_m043_s02.rs` | Capture stdout and stderr plus raw HTTP artifacts and fail with the named phase that drifted. | Preserve pre-timeout artifacts and fail closed instead of hanging or skipping the hard part. | Archive raw bodies and fail if membership, promotion, or keyed status payloads are not valid JSON. |
| Shared assertion helpers in `scripts/lib/m043_cluster_proof.sh` | Stop at the first contract drift and keep raw artifact paths in the failure output. | Report which failover phase never converged and retain the last fetched payloads. | Reject missing fields or wrong role, epoch, or execution-node shapes instead of accepting partial JSON. |
| Wrapper verifier in `scripts/verify-m043-s02.sh` | Replay prerequisites before the slice-specific proof and stop on the first failing phase. | Fail if any named test filter runs 0 tests or if required artifacts are missing. | Reject malformed manifests or copied log bundles instead of claiming a green replay. |

## Load Profile

- **Shared resources**: ephemeral ports, multiple `cluster-proof` processes, `.tmp/m043-s02/...` artifact directories, and the shared `cluster-proof` build output.
- **Per-operation cost**: one destructive multi-node scenario with process start, kill, restart, HTTP polling, artifact copy, and phase-report checks.
- **10x breakpoint**: process teardown, port reuse, and convergence polling will flake before raw CPU or memory does.

## Negative Tests

- **Malformed inputs**: malformed promotion responses, missing role or epoch fields in copied JSON, and copied manifests that omit required retained artifacts.
- **Error paths**: standby never converges to promoted truth, surviving work never completes after primary loss, old primary regains authority on rejoin, and named test filters run 0 tests.
- **Boundary conditions**: first promotion to epoch `1`, same-identity restart of the old primary, repeated status polling across promote and rejoin, and targeted M042 rejoin regression replay.

## Steps

1. Extend `compiler/meshc/tests/e2e_m043_s02.rs` from the API proof into destructive failover scenarios that boot primary and standby nodes, kill the primary, promote the standby, complete surviving keyed work, and restart the old primary.
2. Update `scripts/lib/m043_cluster_proof.sh` so the retained-artifact helpers can assert promotion responses, post-failover membership truth, completed work on the promoted standby, and fenced rejoin state on the old primary.
3. Write `scripts/verify-m043-s02.sh` so it replays `bash scripts/verify-m043-s01.sh`, the targeted `e2e_m042_s03` rejoin regression, the full `e2e_m043_s02` target, and copied-artifact validation under `.tmp/m043-s02/verify/`.
4. Fail closed on zero-test filters, malformed manifests, or missing artifacts, and keep phase reports plus copied logs explicit enough that the first failing failover step is obvious from the verifier output.

## Must-Haves

- [ ] `compiler/meshc/tests/e2e_m043_s02.rs` proves real primary loss, explicit standby promotion, surviving-work completion, and fenced same-identity rejoin.
- [ ] The verifier replays the S01 mirrored-truth baseline and the targeted M042 rejoin regression before claiming the S02 failover contract is green.
- [ ] `scripts/verify-m043-s02.sh` fails closed on zero-test filters, malformed payloads, and missing retained artifacts.
- [ ] `.tmp/m043-s02/verify/` preserves enough JSON and log evidence to localize the first failing failover phase without rerunning the slice.

## Verification

- `cargo test -p meshc --test e2e_m043_s02 -- --nocapture`
- `cargo test -p meshc --test e2e_m042_s03 continuity_api_same_identity_rejoin_preserves_newer_attempt_truth -- --nocapture`
- `bash scripts/verify-m043-s02.sh`

## Observability Impact

- Signals added/changed: failover phase reports, copied promotion and status JSON, per-node stdout and stderr, and verifier assertions for promoted execution and fenced rejoin.
- How a future agent inspects this: read `.tmp/m043-s02/verify/phase-report.txt`, the copied JSON artifacts, and the per-node logs retained by the verifier.
- Failure state exposed: missing promotion convergence, wrong execution node after failover, stale-primary authority regain, and harness cleanup drift remain visible without another destructive replay.

## Inputs

- `compiler/meshc/tests/e2e_m043_s02.rs` — compiler-facing API proof from T02 that now needs destructive failover scenarios.
- `compiler/meshc/tests/e2e_m043_s01.rs` — mirrored-standby harness patterns and retained artifact structure.
- `compiler/meshc/tests/e2e_m042_s03.rs` — targeted same-identity rejoin regression that must remain green.
- `scripts/lib/m043_cluster_proof.sh` — existing M043 JSON assertion and artifact-copy helpers.
- `scripts/verify-m043-s01.sh` — S01 fail-closed wrapper and phase-report discipline to replay first.
- `cluster-proof/main.mpl` — operator-visible routes, including the explicit promotion surface from T03.
- `cluster-proof/work_continuity.mpl` — promoted status truth and surviving-work completion behavior consumed by the harness.

## Expected Output

- `compiler/meshc/tests/e2e_m043_s02.rs` — destructive failover scenarios plus compiler-facing API coverage for S02.
- `scripts/lib/m043_cluster_proof.sh` — reusable assertion helpers for promotion, failover, and fenced rejoin artifacts.
- `scripts/verify-m043-s02.sh` — fail-closed local verifier that replays S01, the targeted M042 regression, the S02 e2e target, and copied-artifact validation.
