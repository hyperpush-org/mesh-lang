---
estimated_steps: 4
estimated_files: 2
skills_used:
  - rust-testing
  - test
---

# T02: Implement the authoritative S06 assembled verifier and repoint the historical alias chain

Create the final S06 shell verifier as the truthful M046 closeout rail, then make the historical alias layer depend on that rail instead of pretending S05 is still the top of the stack.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m046-s06.sh` direct verifier | Fail on the first delegated replay, targeted truth-rail, artifact-copy, or bundle-shape error and keep phase/status/current-phase/latest-proof-bundle artifacts plus command logs. | Bound delegated verifier and targeted test phases; do not report success if any replay hangs. | Treat missing retained verify files, missing copied bundles, or malformed bundle pointers as verifier failures. |
| `scripts/verify-m045-s05.sh` historical wrapper | Fail closed if it still delegates to S05 or if it omits the S06 phase and retained-bundle checks. | Let the delegated S06 timeout propagate instead of masking it as a historical no-op. | Treat missing retained S06 artifacts as alias drift. |
| Existing S03/S04/S05 verifiers and targeted S03/S04 rails | Stop immediately if any lower rail regresses; do not add a second runtime harness or a compensating app-owned control surface. | Preserve the lower-rail timeout/failure semantics and artifact paths inside the S06 retained bundle. | Reject stale route/status/timing seams or missing `meshc cluster` truth artifacts as proof failure. |

## Load Profile

- **Shared resources**: delegated S05 replay, targeted S03/S04 runtime tests, copied `.tmp/m046-s03`, `.tmp/m046-s04`, `.tmp/m046-s05`, and `.tmp/m046-s06` artifact trees.
- **Per-operation cost**: one full equal-surface replay plus targeted startup/failover/package truth reruns and retained-bundle assembly.
- **10x breakpoint**: verifier runtime and artifact churn will fail first; this task is about proof-surface integrity rather than throughput.

## Negative Tests

- **Malformed inputs**: missing `status.txt`, `current-phase.txt`, `phase-report.txt`, `latest-proof-bundle.txt`, or copied retained bundles from delegated rails.
- **Error paths**: stale wrapper delegation to `verify-m046-s05.sh`, missing targeted S03/S04 truth phases, or an S06 verifier that stops checking retained bundle shape.
- **Boundary conditions**: S05 may stay as the equal-surface subrail, but S06 must be the only direct closeout seam and must not reintroduce app-owned status/submit/timing checks.

## Steps

1. Add `scripts/verify-m046-s06.sh` as the direct M046 closeout verifier rooted at `.tmp/m046-s06/verify/`: replay `scripts/verify-m046-s05.sh`, retain the delegated S05 verify directory, rerun the targeted S03 local startup/failover truth rails and the targeted S04 packaged startup truth rail, and copy the fresh retained artifacts under one S06-owned bundle root.
2. Make the S06 verifier publish `status.txt`, `current-phase.txt`, `phase-report.txt`, `full-contract.log`, and `latest-proof-bundle.txt`, and fail closed if any delegated phase, targeted replay, or copied artifact bundle is missing or malformed.
3. Repoint `scripts/verify-m045-s05.sh` so it delegates to `scripts/verify-m046-s06.sh`, retains the delegated verify directory locally, and checks the S06 phase/bundle contract instead of the old S05 boundary.
4. Keep the final assembled verifier route-free: reuse the existing runtime-owned `meshc cluster status|continuity|diagnostics` proof rails and do not add any app-owned status route, submit route, or timing helper to make S06 pass.

## Must-Haves

- [ ] `scripts/verify-m046-s06.sh` is the authoritative closeout verifier for M046 and owns `.tmp/m046-s06/verify/`.
- [ ] The S06 verifier wraps S05, reruns targeted S03/S04 truth rails, and publishes a retained `latest-proof-bundle.txt` pointer for downstream diagnosis.
- [ ] `scripts/verify-m045-s05.sh` becomes a thin historical alias that only passes by delegating to S06.
- [ ] The assembled verifier fails closed on missing retained files, malformed bundle pointers, stale routeful drift, or lower-rail regressions.

## Done When

- [ ] `bash scripts/verify-m046-s06.sh` passes and leaves a diagnosable retained S06 bundle chain.
- [ ] `bash scripts/verify-m045-s05.sh` passes only by delegating to the S06 rail.

## Inputs

- `scripts/verify-m046-s03.sh`
- `scripts/verify-m046-s04.sh`
- `scripts/verify-m046-s05.sh`
- `scripts/verify-m045-s05.sh`
- `compiler/meshc/tests/e2e_m046_s03.rs`
- `compiler/meshc/tests/e2e_m046_s04.rs`
- `compiler/meshc/tests/e2e_m046_s06.rs`

## Expected Output

- `scripts/verify-m046-s06.sh`
- `scripts/verify-m045-s05.sh`

## Verification

cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture && cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture && bash scripts/verify-m046-s06.sh && bash scripts/verify-m045-s05.sh

## Observability Impact

Adds the final `.tmp/m046-s06/verify/` phase/status/current-phase/full-contract/latest-proof-bundle surfaces plus copied delegated S05 verification state and retained S03/S04/S06 bundles so a future agent can localize whether failure came from wrapper delegation, targeted runtime truth, or bundle assembly.
