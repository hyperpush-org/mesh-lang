---
id: T03
parent: S04
milestone: M049
provides: []
requires: []
affects: []
key_files: ["README.md", "compiler/mesh-pkg/src/scaffold.rs", "website/docs/docs/getting-started/clustered-example/index.md", "website/docs/docs/distributed/index.md", "website/docs/docs/distributed-proof/index.md", "website/docs/docs/tooling/index.md", "tools/skill/mesh/skills/clustering/SKILL.md", "scripts/tests/verify-m049-s04-onboarding-contract.test.mjs"]
key_decisions: ["Public clustered onboarding now points at `meshc init --clustered` plus the generated repo examples `examples/todo-postgres` and `examples/todo-sqlite`, while retained `tiny-cluster` and `cluster-proof` proof rails stay lower-level internal fixtures instead of first-contact public runbooks.", "The onboarding contract test extracts the clustered scaffold README template directly from `compiler/mesh-pkg/src/scaffold.rs` so stale public copy fails before generated scaffold wording can drift back toward deleted proof-app links."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-owned verification passed: `node --test scripts/tests/verify-m049-s04-onboarding-contract.test.mjs`, `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`, `node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs`, `node --test scripts/tests/verify-m048-s05-contract.test.mjs`, `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture`, and `npm --prefix website run build` all completed successfully. Slice-level verification is still partial, as expected for this intermediate task: `bash scripts/verify-m039-s01.sh` still fails because it shells `cargo run -q -p meshc -- build cluster-proof`, and `bash scripts/verify-m045-s02.sh` still fails because its bootstrap rail runs `e2e_m045_s01`, which still expects the deleted repo-root `cluster-proof` fixture and the older `@cluster pub fn execute_declared_work` scaffold contract."
completed_at: 2026-04-03T02:56:13.715Z
blocker_discovered: false
---

# T03: Repointed public clustered onboarding to the scaffold plus generated examples and added a fail-closed onboarding contract test.

> Repointed public clustered onboarding to the scaffold plus generated examples and added a fail-closed onboarding contract test.

## What Happened
---
id: T03
parent: S04
milestone: M049
key_files:
  - README.md
  - compiler/mesh-pkg/src/scaffold.rs
  - website/docs/docs/getting-started/clustered-example/index.md
  - website/docs/docs/distributed/index.md
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/tooling/index.md
  - tools/skill/mesh/skills/clustering/SKILL.md
  - scripts/tests/verify-m049-s04-onboarding-contract.test.mjs
key_decisions:
  - Public clustered onboarding now points at `meshc init --clustered` plus the generated repo examples `examples/todo-postgres` and `examples/todo-sqlite`, while retained `tiny-cluster` and `cluster-proof` proof rails stay lower-level internal fixtures instead of first-contact public runbooks.
  - The onboarding contract test extracts the clustered scaffold README template directly from `compiler/mesh-pkg/src/scaffold.rs` so stale public copy fails before generated scaffold wording can drift back toward deleted proof-app links.
duration: ""
verification_result: mixed
completed_at: 2026-04-03T02:56:13.717Z
blocker_discovered: false
---

# T03: Repointed public clustered onboarding to the scaffold plus generated examples and added a fail-closed onboarding contract test.

**Repointed public clustered onboarding to the scaffold plus generated examples and added a fail-closed onboarding contract test.**

## What Happened

Updated the public clustered story across the repo root README, the clustered scaffold README template in `compiler/mesh-pkg/src/scaffold.rs`, the clustered docs pages, the tooling docs, and the Mesh clustering skill so first-contact guidance now starts with `meshc init --clustered`, branches to `examples/todo-postgres` for the serious shared/deployable PostgreSQL starter, and branches to `examples/todo-sqlite` for the honest local single-node SQLite starter. I also kept `reference-backend` positioned as the deeper backend proof surface rather than a coequal clustered starter. On the docs-verifier side, `website/docs/docs/distributed-proof/index.md` now keeps the lower-level retained fixture rails explicit under `scripts/fixtures/clustered/` instead of pointing at deleted repo-root README runbooks.

Added `scripts/tests/verify-m049-s04-onboarding-contract.test.mjs` as a slice-owned fail-closed contract test. It verifies the scaffold/examples-first public story, rejects stale `tiny-cluster/README.md` and `cluster-proof/README.md` onboarding links, rejects unsplit `meshc init --template todo-api` wording, and catches distributed-proof regressions that drift retained fixture commands back to deleted repo-root paths. I also tightened the clustered scaffold unit test in `compiler/mesh-pkg/src/scaffold.rs` so the generated README now asserts the example-first follow-ons directly.

## Verification

Task-owned verification passed: `node --test scripts/tests/verify-m049-s04-onboarding-contract.test.mjs`, `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`, `node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs`, `node --test scripts/tests/verify-m048-s05-contract.test.mjs`, `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture`, and `npm --prefix website run build` all completed successfully. Slice-level verification is still partial, as expected for this intermediate task: `bash scripts/verify-m039-s01.sh` still fails because it shells `cargo run -q -p meshc -- build cluster-proof`, and `bash scripts/verify-m045-s02.sh` still fails because its bootstrap rail runs `e2e_m045_s01`, which still expects the deleted repo-root `cluster-proof` fixture and the older `@cluster pub fn execute_declared_work` scaffold contract.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m049-s04-onboarding-contract.test.mjs` | 0 | ✅ pass | 1495ms |
| 2 | `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check` | 0 | ✅ pass | 774ms |
| 3 | `node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs` | 0 | ✅ pass | 1698ms |
| 4 | `node --test scripts/tests/verify-m048-s05-contract.test.mjs` | 0 | ✅ pass | 1370ms |
| 5 | `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture` | 0 | ✅ pass | 11432ms |
| 6 | `npm --prefix website run build` | 0 | ✅ pass | 53343ms |
| 7 | `bash scripts/verify-m039-s01.sh` | 1 | ❌ fail | 87196ms |
| 8 | `bash scripts/verify-m045-s02.sh` | 1 | ❌ fail | 139993ms |


## Deviations

Added a targeted `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture` replay and updated the clustered scaffold unit test alongside the planned Node contract test, because the task changed generated Rust-owned README text and needed one direct scaffold guardrail in addition to the docs/content checks.

## Known Issues

`bash scripts/verify-m039-s01.sh` still shells `cargo run -q -p meshc -- build cluster-proof`, so it fails until the older direct bash verifier family is retargeted to the relocated fixture path in later S04 work. `bash scripts/verify-m045-s02.sh` still bootstraps through `cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture`, and that target still expects the deleted repo-root `cluster-proof` files plus the older `@cluster pub fn execute_declared_work` scaffold contract.

## Files Created/Modified

- `README.md`
- `compiler/mesh-pkg/src/scaffold.rs`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/tooling/index.md`
- `tools/skill/mesh/skills/clustering/SKILL.md`
- `scripts/tests/verify-m049-s04-onboarding-contract.test.mjs`


## Deviations
Added a targeted `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture` replay and updated the clustered scaffold unit test alongside the planned Node contract test, because the task changed generated Rust-owned README text and needed one direct scaffold guardrail in addition to the docs/content checks.

## Known Issues
`bash scripts/verify-m039-s01.sh` still shells `cargo run -q -p meshc -- build cluster-proof`, so it fails until the older direct bash verifier family is retargeted to the relocated fixture path in later S04 work. `bash scripts/verify-m045-s02.sh` still bootstraps through `cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture`, and that target still expects the deleted repo-root `cluster-proof` files plus the older `@cluster pub fn execute_declared_work` scaffold contract.
