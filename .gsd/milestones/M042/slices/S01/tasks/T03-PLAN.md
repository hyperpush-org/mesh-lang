---
estimated_steps: 1
estimated_files: 5
skills_used: []
---

# T03: Rewrite cluster-proof as a thin Continuity consumer and close the healthy-path proof

Replace the app-authored request registry, replica-prepare logic, and keyed status ownership inside `cluster-proof` with calls into the runtime-native `Continuity` API while preserving the existing `/work` HTTP contract. Update the proof surfaces so standalone and healthy two-node cluster runs prove the new ownership boundary and keep the M040 semantic guarantees intact.

## Inputs

- `cluster-proof/main.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/tests/work.test.mpl`
- `compiler/meshc/tests/e2e_m040_s01.rs`
- `compiler/meshc/tests/e2e_m039_s03.rs`

## Expected Output

- `cluster-proof/work.mpl`
- `cluster-proof/main.mpl`
- `cluster-proof/tests/work.test.mpl`
- `compiler/meshc/tests/e2e_m042_s01.rs`
- `scripts/verify-m042-s01.sh`

## Verification

cargo run -q -p meshc -- test cluster-proof/tests && cargo test -p meshc --test e2e_m042_s01 -- --nocapture && bash scripts/verify-m042-s01.sh

## Observability Impact

Reconciles proof-app logging and verifier artifacts with the new runtime boundary so healthy-path regressions remain localizable through `/work` JSON, stdout transition logs, and archived verifier evidence.
