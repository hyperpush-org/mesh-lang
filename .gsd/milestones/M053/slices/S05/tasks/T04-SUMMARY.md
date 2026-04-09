---
id: T04
parent: S05
milestone: M053
provides: []
requires: []
affects: []
key_files: ["scripts/verify-m053-s01.sh", "scripts/verify-m053-s02.sh", "compiler/meshc/tests/e2e_m053_s01.rs", "compiler/meshc/tests/e2e_m053_s02.rs", ".gsd/milestones/M053/slices/S05/tasks/T04-SUMMARY.md"]
key_decisions: ["Normalize CARGO_HOME/CARGO_TARGET_DIR to absolute repo-root paths inside the starter-proof wrappers so nested generated-project commands resolve the same target tree.", "Retain nested S01 verifier logs under .tmp/m053-s02/verify/upstream-m053-s01-verify so hosted S02 failures preserve decisive inner Rust/test output."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified node --test scripts/tests/verify-m053-s03-contract.test.mjs passed, both shell wrappers parse cleanly with bash -n, the pre-fix cold S01 rail truthfully failed at the remaining relative-path staged-deploy seam, and a focused absolute-path cold-target probe of cargo test -p meshc --test e2e_m053_s01 m053_s01_postgres_staged_deploy_bundle_boots_serves_crud_and_answers_cluster_inspection -- --nocapture --test-threads=1 passed. Full S01/S02 and hosted remote-main verification remain pending."
completed_at: 2026-04-06T00:04:22.419Z
blocker_discovered: false
---

# T04: Patched the starter-proof wrappers to normalize cargo paths and retain nested S01 logs, and proved the staged deploy rail with an absolute cold target path.

> Patched the starter-proof wrappers to normalize cargo paths and retain nested S01 logs, and proved the staged deploy rail with an absolute cold target path.

## What Happened
---
id: T04
parent: S05
milestone: M053
key_files:
  - scripts/verify-m053-s01.sh
  - scripts/verify-m053-s02.sh
  - compiler/meshc/tests/e2e_m053_s01.rs
  - compiler/meshc/tests/e2e_m053_s02.rs
  - .gsd/milestones/M053/slices/S05/tasks/T04-SUMMARY.md
key_decisions:
  - Normalize CARGO_HOME/CARGO_TARGET_DIR to absolute repo-root paths inside the starter-proof wrappers so nested generated-project commands resolve the same target tree.
  - Retain nested S01 verifier logs under .tmp/m053-s02/verify/upstream-m053-s01-verify so hosted S02 failures preserve decisive inner Rust/test output.
duration: ""
verification_result: mixed
completed_at: 2026-04-06T00:04:22.420Z
blocker_discovered: false
---

# T04: Patched the starter-proof wrappers to normalize cargo paths and retain nested S01 logs, and proved the staged deploy rail with an absolute cold target path.

**Patched the starter-proof wrappers to normalize cargo paths and retain nested S01 logs, and proved the staged deploy rail with an absolute cold target path.**

## What Happened

Patched scripts/verify-m053-s01.sh and scripts/verify-m053-s02.sh so starter-proof runs normalize relative CARGO_HOME/CARGO_TARGET_DIR values to absolute repo-root paths before nested generated-project commands run, added the intended mesh-rt prebuild phase to S01, and made S02 retain nested .tmp/m053-s01/verify logs under .tmp/m053-s02/verify/upstream-m053-s01-verify. Updated compiler/meshc/tests/e2e_m053_s01.rs and compiler/meshc/tests/e2e_m053_s02.rs to lock those verifier-surface changes in place. Verified the hosted-contract node test still passed and confirmed with a focused absolute-path cold-target probe that m053_s01_postgres_staged_deploy_bundle_boots_serves_crud_and_answers_cluster_inspection now passes. Did not complete the full end-to-end closeout before timeout: full bash scripts/verify-m053-s01.sh and scripts/verify-m053-s02.sh were not rerun after the final absolute-path fix, no remote main update was performed, and bash scripts/verify-m053-s03.sh was not replayed on a repaired hosted SHA.

## Verification

Verified node --test scripts/tests/verify-m053-s03-contract.test.mjs passed, both shell wrappers parse cleanly with bash -n, the pre-fix cold S01 rail truthfully failed at the remaining relative-path staged-deploy seam, and a focused absolute-path cold-target probe of cargo test -p meshc --test e2e_m053_s01 m053_s01_postgres_staged_deploy_bundle_boots_serves_crud_and_answers_cluster_inspection -- --nocapture --test-threads=1 passed. Full S01/S02 and hosted remote-main verification remain pending.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m053-s03-contract.test.mjs` | 0 | ✅ pass | 12728ms |
| 2 | `env DATABASE_URL='postgres://postgres:postgres@127.0.0.1:5432/postgres' CARGO_HOME='.tmp/m053-s05/t04-cold-cargo-home-r2' CARGO_TARGET_DIR='.tmp/m053-s05/t04-cold-target-r2' bash scripts/verify-m053-s01.sh` | 1 | ❌ fail | 307700ms |
| 3 | `env DATABASE_URL='postgres://postgres:postgres@127.0.0.1:5432/postgres' CARGO_HOME="$PWD/.tmp/m053-s05/t04-cold-cargo-home-r2" CARGO_TARGET_DIR="$PWD/.tmp/m053-s05/t04-cold-target-r2" cargo test -p meshc --test e2e_m053_s01 m053_s01_postgres_staged_deploy_bundle_boots_serves_crud_and_answers_cluster_inspection -- --nocapture --test-threads=1` | 0 | ✅ pass | 151320ms |
| 4 | `bash -n scripts/verify-m053-s01.sh && bash -n scripts/verify-m053-s02.sh` | 0 | ✅ pass | 100ms |


## Deviations

Because the unit hit the hard timeout, I stopped after landing the local repair and proving the repaired staged-deploy seam with a focused cold-target probe instead of forcing an incomplete hosted mainline closeout. Remote main was not updated, fresh authoritative/deploy workflow evidence was not captured, and the final assembled verifier was not replayed on a repaired SHA.

## Known Issues

Full bash scripts/verify-m053-s01.sh and DATABASE_URL='postgres://postgres:postgres@127.0.0.1:5432/postgres' bash scripts/verify-m053-s02.sh were not rerun after the final absolute-path normalization. Remote main remains at c6d31bf495fd43a19e96a8becebbd3f7426c4bd7 with no repair commit pushed, so authoritative-verification.yml / deploy-services.yml were not rechecked on a repaired SHA and bash scripts/verify-m053-s03.sh was not replayed after the final local fix.

## Files Created/Modified

- `scripts/verify-m053-s01.sh`
- `scripts/verify-m053-s02.sh`
- `compiler/meshc/tests/e2e_m053_s01.rs`
- `compiler/meshc/tests/e2e_m053_s02.rs`
- `.gsd/milestones/M053/slices/S05/tasks/T04-SUMMARY.md`


## Deviations
Because the unit hit the hard timeout, I stopped after landing the local repair and proving the repaired staged-deploy seam with a focused cold-target probe instead of forcing an incomplete hosted mainline closeout. Remote main was not updated, fresh authoritative/deploy workflow evidence was not captured, and the final assembled verifier was not replayed on a repaired SHA.

## Known Issues
Full bash scripts/verify-m053-s01.sh and DATABASE_URL='postgres://postgres:postgres@127.0.0.1:5432/postgres' bash scripts/verify-m053-s02.sh were not rerun after the final absolute-path normalization. Remote main remains at c6d31bf495fd43a19e96a8becebbd3f7426c4bd7 with no repair commit pushed, so authoritative-verification.yml / deploy-services.yml were not rechecked on a repaired SHA and bash scripts/verify-m053-s03.sh was not replayed after the final local fix.
