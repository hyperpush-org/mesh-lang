---
id: T01
parent: S04
milestone: M049
provides: []
requires: []
affects: []
key_files: ["scripts/fixtures/clustered/tiny-cluster/mesh.toml", "scripts/fixtures/clustered/tiny-cluster/main.mpl", "scripts/fixtures/clustered/tiny-cluster/work.mpl", "scripts/fixtures/clustered/tiny-cluster/README.md", "scripts/fixtures/clustered/tiny-cluster/tests/work.test.mpl", "compiler/meshc/tests/support/m046_route_free.rs", "compiler/meshc/tests/e2e_m046_s03.rs", ".gsd/KNOWLEDGE.md"]
key_decisions: ["The shared `m046_route_free` support module now owns tiny-cluster fixture discovery and validates required files plus package identity before tests consume the path."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-owned verification passed with `cargo run -q -p meshc -- test scripts/fixtures/clustered/tiny-cluster/tests`, `cargo run -q -p meshc -- build scripts/fixtures/clustered/tiny-cluster`, `cargo test -p meshc --test e2e_m046_s03 -- --nocapture`, and a structural check confirming the repo-root `tiny-cluster/` directory is gone while the relocated fixture remains and the updated Rust seam no longer hardcodes `repo_root().join("tiny-cluster")`. I also replayed the full slice verification set into `.tmp/m049-s04/t01-slice-verification/results.json`: `e2e_m046_s03`, `e2e_m046_s04`, `e2e_m045_s02`, the materialize check, both M048 contract tests, and the website build passed; the remaining failures are later-task drift around old repo-root `tiny-cluster` / `cluster-proof` paths or still-missing onboarding-contract coverage."
completed_at: 2026-04-03T02:22:40.277Z
blocker_discovered: false
---

# T01: Relocated tiny-cluster into scripts/fixtures/clustered/tiny-cluster and switched the retained M046 route-free rail to the fixture path.

> Relocated tiny-cluster into scripts/fixtures/clustered/tiny-cluster and switched the retained M046 route-free rail to the fixture path.

## What Happened
---
id: T01
parent: S04
milestone: M049
key_files:
  - scripts/fixtures/clustered/tiny-cluster/mesh.toml
  - scripts/fixtures/clustered/tiny-cluster/main.mpl
  - scripts/fixtures/clustered/tiny-cluster/work.mpl
  - scripts/fixtures/clustered/tiny-cluster/README.md
  - scripts/fixtures/clustered/tiny-cluster/tests/work.test.mpl
  - compiler/meshc/tests/support/m046_route_free.rs
  - compiler/meshc/tests/e2e_m046_s03.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - The shared `m046_route_free` support module now owns tiny-cluster fixture discovery and validates required files plus package identity before tests consume the path.
duration: ""
verification_result: mixed
completed_at: 2026-04-03T02:22:40.278Z
blocker_discovered: false
---

# T01: Relocated tiny-cluster into scripts/fixtures/clustered/tiny-cluster and switched the retained M046 route-free rail to the fixture path.

**Relocated tiny-cluster into scripts/fixtures/clustered/tiny-cluster and switched the retained M046 route-free rail to the fixture path.**

## What Happened

Copied the source-only `tiny-cluster` package into `scripts/fixtures/clustered/tiny-cluster/`, updated the fixture-local README and package smoke test to use the relocated path, and kept the package/runtime/log identity as `tiny-cluster`. Added `TINY_CLUSTER_FIXTURE_ROOT_RELATIVE`, a required-file list, a manifest/package-name validator, and a fail-closed `tiny_cluster_fixture_root()` helper in `compiler/meshc/tests/support/m046_route_free.rs`, then retargeted `compiler/meshc/tests/e2e_m046_s03.rs` to that helper and added a negative test that removes `work.mpl` from a copied fixture tree to prove the helper reports the broken fixture before runtime proof starts. While rerunning `e2e_m046_s03`, fixed a stale helper-only request-key literal that still expected `startup::Work.execute_declared_work` even though the file’s live runtime-owned startup key is `startup::Work.add`. After the moved fixture and retained rail were green, removed the repo-root `tiny-cluster/` directory, cleaned the in-place built binary so the fixture ends in source-only state, and recorded the repo-relative `File.read(...)` fallback gotcha in `.gsd/KNOWLEDGE.md`.

## Verification

Task-owned verification passed with `cargo run -q -p meshc -- test scripts/fixtures/clustered/tiny-cluster/tests`, `cargo run -q -p meshc -- build scripts/fixtures/clustered/tiny-cluster`, `cargo test -p meshc --test e2e_m046_s03 -- --nocapture`, and a structural check confirming the repo-root `tiny-cluster/` directory is gone while the relocated fixture remains and the updated Rust seam no longer hardcodes `repo_root().join("tiny-cluster")`. I also replayed the full slice verification set into `.tmp/m049-s04/t01-slice-verification/results.json`: `e2e_m046_s03`, `e2e_m046_s04`, `e2e_m045_s02`, the materialize check, both M048 contract tests, and the website build passed; the remaining failures are later-task drift around old repo-root `tiny-cluster` / `cluster-proof` paths or still-missing onboarding-contract coverage.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- test scripts/fixtures/clustered/tiny-cluster/tests` | 0 | ✅ pass | 9869ms |
| 2 | `cargo run -q -p meshc -- build scripts/fixtures/clustered/tiny-cluster` | 0 | ✅ pass | 7808ms |
| 3 | `cargo test -p meshc --test e2e_m046_s03 -- --nocapture` | 0 | ✅ pass | 21690ms |
| 4 | `test ! -e tiny-cluster && test -d scripts/fixtures/clustered/tiny-cluster && ! rg -n 'repo_root\(\)\.join\("tiny-cluster"\)' compiler/meshc/tests/e2e_m046_s03.rs compiler/meshc/tests/support/m046_route_free.rs` | 0 | ✅ pass | 63ms |
| 5 | `cargo test -p meshc --test e2e_m046_s03 -- --nocapture` | 0 | ✅ pass | 14705ms |
| 6 | `cargo test -p meshc --test e2e_m046_s04 -- --nocapture` | 0 | ✅ pass | 36648ms |
| 7 | `cargo test -p meshc --test e2e_m045_s01 -- --nocapture` | 101 | ❌ fail | 17449ms |
| 8 | `cargo test -p meshc --test e2e_m045_s02 -- --nocapture` | 0 | ✅ pass | 20941ms |
| 9 | `cargo test -p meshc --test e2e_m046_s05 -- --nocapture` | 101 | ❌ fail | 16658ms |
| 10 | `cargo test -p meshc --test e2e_m047_s04 -- --nocapture` | 101 | ❌ fail | 8374ms |
| 11 | `node --test scripts/tests/verify-m049-s04-onboarding-contract.test.mjs` | 1 | ❌ fail | 410ms |
| 12 | `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check` | 0 | ✅ pass | 5216ms |
| 13 | `node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs` | 0 | ✅ pass | 893ms |
| 14 | `node --test scripts/tests/verify-m048-s05-contract.test.mjs` | 0 | ✅ pass | 998ms |
| 15 | `npm --prefix website run build` | 0 | ✅ pass | 45339ms |
| 16 | `bash scripts/verify-m039-s01.sh` | 1 | ❌ fail | 83472ms |
| 17 | `bash scripts/verify-m045-s02.sh` | 1 | ❌ fail | 62820ms |
| 18 | `bash scripts/verify-m047-s04.sh` | 1 | ❌ fail | 1231ms |
| 19 | `bash scripts/verify-m047-s05.sh` | 1 | ❌ fail | 2602ms |


## Deviations

Updated two stale request-key literals inside `compiler/meshc/tests/e2e_m046_s03.rs` from `startup::Work.execute_declared_work` to the live `startup::Work.add` key while rerunning the retained target. This was outside the written relocation steps but inside the owned test file, and the target could not pass truthfully without it.

## Known Issues

- `compiler/meshc/tests/e2e_m046_s05.rs`, `compiler/meshc/tests/e2e_m047_s04.rs`, `scripts/verify-m047-s04.sh`, and `scripts/verify-m047-s05.sh` still depend on the deleted repo-root `tiny-cluster` paths; later S04 tasks need to move those to the fixture/helper layer.
- `compiler/meshc/tests/e2e_m045_s01.rs` and the wrapper path inside `scripts/verify-m045-s02.sh` still expect the older `execute_declared_work` clustered-work shape in `cluster-proof`/scaffold surfaces.
- `scripts/tests/verify-m049-s04-onboarding-contract.test.mjs` is referenced by the slice plan but does not exist in the tree yet.
- `bash scripts/verify-m039-s01.sh` remains red on the retained `cluster-proof` bootstrap env contract (`MESH_CLUSTER_COOKIE is required when discovery or identity env is set`); that rail is outside the tiny-cluster relocation seam.

## Files Created/Modified

- `scripts/fixtures/clustered/tiny-cluster/mesh.toml`
- `scripts/fixtures/clustered/tiny-cluster/main.mpl`
- `scripts/fixtures/clustered/tiny-cluster/work.mpl`
- `scripts/fixtures/clustered/tiny-cluster/README.md`
- `scripts/fixtures/clustered/tiny-cluster/tests/work.test.mpl`
- `compiler/meshc/tests/support/m046_route_free.rs`
- `compiler/meshc/tests/e2e_m046_s03.rs`
- `.gsd/KNOWLEDGE.md`


## Deviations
Updated two stale request-key literals inside `compiler/meshc/tests/e2e_m046_s03.rs` from `startup::Work.execute_declared_work` to the live `startup::Work.add` key while rerunning the retained target. This was outside the written relocation steps but inside the owned test file, and the target could not pass truthfully without it.

## Known Issues
- `compiler/meshc/tests/e2e_m046_s05.rs`, `compiler/meshc/tests/e2e_m047_s04.rs`, `scripts/verify-m047-s04.sh`, and `scripts/verify-m047-s05.sh` still depend on the deleted repo-root `tiny-cluster` paths; later S04 tasks need to move those to the fixture/helper layer.
- `compiler/meshc/tests/e2e_m045_s01.rs` and the wrapper path inside `scripts/verify-m045-s02.sh` still expect the older `execute_declared_work` clustered-work shape in `cluster-proof`/scaffold surfaces.
- `scripts/tests/verify-m049-s04-onboarding-contract.test.mjs` is referenced by the slice plan but does not exist in the tree yet.
- `bash scripts/verify-m039-s01.sh` remains red on the retained `cluster-proof` bootstrap env contract (`MESH_CLUSTER_COOKIE is required when discovery or identity env is set`); that rail is outside the tiny-cluster relocation seam.
