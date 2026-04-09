---
estimated_steps: 3
estimated_files: 1
skills_used: []
---

# T04: Add the canonical S02 verifier and phase ledger

1. Add `scripts/verify-m039-s02.sh` as the canonical local replay wrapper for the slice.
2. Run `cluster-proof/tests`, rebuild `cluster-proof`, recheck S01 convergence explicitly, and then run the named S02 e2e filters so routing proof never hides a broken cluster bootstrap.
3. Preserve per-phase logs and per-node artifacts under `.tmp/m039-s02/verify/`, and fail closed if any named filter runs zero tests, any phase stalls, or any node log is missing.

## Inputs

- `compiler/meshc/tests/e2e_m039_s02.rs`
- `scripts/verify-m039-s01.sh`

## Expected Output

- `scripts/verify-m039-s02.sh`
- `Canonical phase-report and verifier artifacts under .tmp/m039-s02/verify/`

## Verification

bash scripts/verify-m039-s02.sh

## Observability Impact

Adds one fail-closed replay surface with phase logs and node-artifact pointers so future agents can distinguish runtime regressions from harness drift quickly.
