---
estimated_steps: 3
estimated_files: 4
skills_used:
  - test
---

# T03: Assemble the S03 verifier and retain a fail-closed failover bundle

Finish slice closeout with an authoritative, flattened verifier that replays the prerequisite runtime rails, the scaffold contract rails, and the new S03 e2e, then copies a fresh proof bundle into `.tmp/m045-s03/verify/` and fail-closes on zero-test, malformed pointer, or missing artifact shape.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Upstream runtime/scaffold regression commands replayed by `scripts/verify-m045-s03.sh` | Stop on the first red prerequisite and keep per-phase logs; do not continue to the S03 rail with stale assumptions. | Bound every command and fail with the captured log instead of hanging the verifier. | Treat missing `running N test` lines or malformed command output as verifier drift, not success. |
| Fresh `.tmp/m045-s03` artifact discovery and copy logic | Fail closed if no fresh bundle is produced or if the pointer/manifest targets the wrong directory. | N/A — copy/manifest checks are local and synchronous. | Reject missing `scenario-meta.json`, required JSON evidence files, or missing node logs rather than claiming the failover proof was retained. |

## Load Profile

- **Shared resources**: cargo test output, `.tmp/m045-s03` artifact roots, copied verifier bundles, and per-phase logs under `.tmp/m045-s03/verify/`.
- **Per-operation cost**: replay of focused runtime rails plus one full `e2e_m045_s03` run and artifact-copy validation.
- **10x breakpoint**: stale artifact roots and long-running test replays fail before throughput does; the verifier must make freshness and bundle shape explicit.

## Negative Tests

- **Malformed inputs**: zero-test filters, malformed `latest-proof-bundle.txt`, missing `scenario-meta.json`, and missing pre-kill/post-kill/post-rejoin evidence files.
- **Error paths**: prerequisite green rail goes red, the S03 e2e never emits a fresh artifact directory, or the copied bundle points at the wrong source.
- **Boundary conditions**: multiple old `.tmp/m045-s03` directories exist, the verifier still selects only the fresh replay output, and phase files stay truthful on early failure.

## Steps

1. Add `scripts/verify-m045-s03.sh` with phase/state files and direct replays of the focused prerequisites: runtime auto-promotion/recovery rails, the protected M044 S04 failover rail, the scaffold init contract, the S02 runtime-completion rail, and the new S03 e2e.
2. Snapshot and copy the fresh `.tmp/m045-s03` artifact directories into `.tmp/m045-s03/verify/retained-m045-s03-artifacts/`, then assert the retained bundle contains `scenario-meta.json`, pre-kill continuity/status JSON, post-promotion diagnostics/continuity JSON, post-rejoin status JSON, and node stdout/stderr logs.
3. Fail closed on zero-test drift, missing bundle freshness, malformed pointer files, or incomplete retained evidence so later slices can trust the verifier output without rerunning the cluster immediately.

## Must-Haves

- [ ] `scripts/verify-m045-s03.sh` is the authoritative local stopping condition for S03.
- [ ] The verifier retains one fresh failover bundle and checks its shape before returning green.
- [ ] Early failures leave enough per-phase output behind to distinguish prerequisite regressions from S03-only proof drift.

## Verification

- `bash scripts/verify-m045-s03.sh`

## Inputs

- `scripts/verify-m045-s02.sh`
- `compiler/meshc/tests/e2e_m045_s03.rs`
- `compiler/meshc/tests/e2e_m044_s04.rs`
- `compiler/meshc/tests/tooling_e2e.rs`

## Expected Output

- `scripts/verify-m045-s03.sh`
- `compiler/meshc/tests/e2e_m045_s03.rs`

## Verification

bash scripts/verify-m045-s03.sh

## Observability Impact

- Signals added/changed: `.tmp/m045-s03/verify/phase-report.txt`, `status.txt`, `current-phase.txt`, `latest-proof-bundle.txt`, copied manifest logs, and the retained failover bundle.
- How a future agent inspects this: run `bash scripts/verify-m045-s03.sh` and inspect `.tmp/m045-s03/verify/`.
- Failure state exposed: zero-test drift, stale or malformed bundle pointers, and missing pre-kill/post-kill/post-rejoin evidence are each called out separately.
