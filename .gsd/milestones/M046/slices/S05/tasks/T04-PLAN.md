---
estimated_steps: 4
estimated_files: 4
skills_used:
  - rust-best-practices
  - test
---

# T04: Create the authoritative S05 verifier and repoint the historical closeout wrapper to it

Finish the slice with one fail-closed verifier that replays the route-free proof surfaces, scaffold alignment rails, and docs build, then make the old M045 closeout wrapper a thin alias to that truthful S05 rail.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m046-s05.sh` direct verifier | Fail on the first replay/build/docs error and keep phase/status/current-phase/latest-proof-bundle artifacts plus command logs. | Bound delegated verifier and test phases; do not report success if any replay hangs. | Treat missing phase files, missing retained bundles, or malformed artifact pointers as verifier failures. |
| `scripts/verify-m045-s05.sh` historical wrapper | Fail closed if it still delegates only to S04 or if it omits the new docs/scaffold alignment phases. | Let the delegated S05 timeout propagate instead of masking it as a historical no-op. | Treat missing retained S05 artifacts as alias drift. |
| `compiler/meshc/tests/e2e_m045_s05.rs` and `compiler/meshc/tests/e2e_m046_s05.rs` content guards | Fail fast when wrapper/script assertions drift away from the authoritative S05 phase names, retained bundle shape, or doc/scaffold scope. | N/A | Reject stale S04-only expectations and malformed verifier references as contract failures. |

## Load Profile

- **Shared resources**: delegated S03/S04 verifiers, fast scaffold tests, docs build, route-free runtime rails, and copied `.tmp/m046-s05/verify` artifacts.
- **Per-operation cost**: one full closeout replay including Rust tests, docs build, and artifact copy/shape checks.
- **10x breakpoint**: verifier artifact churn and replay timeouts will fail first; this task is about proof-surface integrity rather than runtime throughput.

## Negative Tests

- **Malformed inputs**: missing `status.txt`, `current-phase.txt`, `phase-report.txt`, `latest-proof-bundle.txt`, or copied retained bundles.
- **Error paths**: stale wrapper delegation to `verify-m046-s04.sh`, missing docs-build phase, or verifier scripts that stop checking scaffold/docs alignment.
- **Boundary conditions**: the historical alias may stay, but it must clearly depend on the authoritative S05 verifier and not revive deleted routeful/product-story checks.

## Steps

1. Add `scripts/verify-m046-s05.sh` as the direct equal-surface verifier: replay the focused scaffold tests, the route-free proof regressions, the new `e2e_m046_s05` rail, the docs build, and retained-artifact copy/shape checks.
2. Repoint `scripts/verify-m045-s05.sh` so it delegates to `scripts/verify-m046-s05.sh`, retains the delegated verify directory locally, and fails closed on missing S05 phase/bundle artifacts.
3. Update `compiler/meshc/tests/e2e_m045_s05.rs` (and `compiler/meshc/tests/e2e_m046_s05.rs` if needed) so the Rust-side contract assertions pin the new S05 verifier phases, retained artifacts, and docs/scaffold scope.
4. Preserve the S03/S04 retained bundle chain so S06 can assemble milestone-wide replay from one truthful closeout seam.

## Must-Haves

- [ ] `scripts/verify-m046-s05.sh` is the authoritative verifier for scaffold/docs/proof alignment.
- [ ] `scripts/verify-m045-s05.sh` becomes a thin historical alias that retains and checks S05 verifier artifacts instead of reasserting S04-only truth.
- [ ] Rust content-guard tests pin the new S05 verifier shape and fail on stale S04-only expectations.
- [ ] The verifier emits retained S05 phase/status/current-phase/latest-proof-bundle artifacts that future slices can inspect directly.

## Done When

- [ ] The direct S05 verifier passes and the historical alias passes only by delegating to it.
- [ ] The retained S05 verify directory is sufficient for a future agent to diagnose which replay phase drifted.

## Inputs

- `scripts/verify-m046-s03.sh`
- `scripts/verify-m046-s04.sh`
- `scripts/verify-m045-s05.sh`
- `compiler/meshc/tests/e2e_m045_s05.rs`
- `compiler/meshc/tests/e2e_m046_s05.rs`
- `compiler/meshc/tests/e2e_m046_s03.rs`
- `compiler/meshc/tests/e2e_m046_s04.rs`

## Expected Output

- `scripts/verify-m046-s05.sh`
- `scripts/verify-m045-s05.sh`
- `compiler/meshc/tests/e2e_m045_s05.rs`
- `compiler/meshc/tests/e2e_m046_s05.rs`

## Verification

cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture && bash scripts/verify-m046-s05.sh && bash scripts/verify-m045-s05.sh

## Observability Impact

- Signals added/changed: `.tmp/m046-s05/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, `full-contract.log`, and `latest-proof-bundle.txt` plus copied retained S03/S04/S05 artifacts.
- How a future agent inspects this: rerun `bash scripts/verify-m046-s05.sh` or inspect the retained verify directory and delegated bundle pointers.
- Failure state exposed: the exact failing phase, command log, missing retained file, or delegated verifier drift remains visible after failure.
