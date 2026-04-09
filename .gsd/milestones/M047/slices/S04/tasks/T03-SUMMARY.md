---
id: T03
parent: S04
milestone: M047
provides: []
requires: []
affects: []
key_files: ["tiny-cluster/work.mpl", "tiny-cluster/tests/work.test.mpl", "tiny-cluster/README.md", "cluster-proof/work.mpl", "cluster-proof/tests/work.test.mpl", "cluster-proof/README.md", "tiny-cluster-prefered/mesh.toml"]
key_decisions: ["Kept `tiny-cluster/work.mpl` and `cluster-proof/work.mpl` byte-for-byte aligned with the scaffold's `@cluster pub fn execute_declared_work(...)` source so later parity rails keep one clustered work contract.", "Kept the package smoke tests fail-closed on legacy tokens and route text instead of documenting old markers as omitted, because these package READMEs are part of the public contract surface."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the task verification command from the slice plan exactly: `cargo run -q -p meshc -- test tiny-cluster/tests && cargo run -q -p meshc -- build tiny-cluster && cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof`. The passing replay proved the tiny-cluster smoke suite (3 assertions), tiny-cluster build, cluster-proof smoke suite (5 assertions including packaging/deleted-helper guards), and cluster-proof build all completed successfully. As an intermediate task, I ran the T03 slice rail only; later S04 task rails were not run yet."
completed_at: 2026-04-01T09:47:47.219Z
blocker_discovered: false
---

# T03: Migrated tiny-cluster and cluster-proof to the @cluster source-first contract and hardened their package smoke tests against legacy markers.

> Migrated tiny-cluster and cluster-proof to the @cluster source-first contract and hardened their package smoke tests against legacy markers.

## What Happened
---
id: T03
parent: S04
milestone: M047
key_files:
  - tiny-cluster/work.mpl
  - tiny-cluster/tests/work.test.mpl
  - tiny-cluster/README.md
  - cluster-proof/work.mpl
  - cluster-proof/tests/work.test.mpl
  - cluster-proof/README.md
  - tiny-cluster-prefered/mesh.toml
key_decisions:
  - Kept `tiny-cluster/work.mpl` and `cluster-proof/work.mpl` byte-for-byte aligned with the scaffold's `@cluster pub fn execute_declared_work(...)` source so later parity rails keep one clustered work contract.
  - Kept the package smoke tests fail-closed on legacy tokens and route text instead of documenting old markers as omitted, because these package READMEs are part of the public contract surface.
duration: ""
verification_result: passed
completed_at: 2026-04-01T09:47:47.220Z
blocker_discovered: false
---

# T03: Migrated tiny-cluster and cluster-proof to the @cluster source-first contract and hardened their package smoke tests against legacy markers.

**Migrated tiny-cluster and cluster-proof to the @cluster source-first contract and hardened their package smoke tests against legacy markers.**

## What Happened

Rewrote `tiny-cluster/work.mpl` and `cluster-proof/work.mpl` to the shared scaffold-style `@cluster pub fn execute_declared_work(...) -> Int do 1 + 1 end` form, removing both `clustered(work)` and `declared_work_runtime_name()` while preserving the stable runtime-owned handler name through the function name itself. Updated both package smoke suites to import only `execute_declared_work`, prove the visible `1 + 1` behavior, fail if `clustered(work)`, `declared_work_runtime_name`, or manifest `[cluster]` text reappears, and keep the route-free/runtime-owned negative guards for helper-managed continuity seams and package-owned routes. Rewrote both READMEs to teach the same source-first route-free contract as the clustered scaffold, pinning `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics` as the operator-facing inspection surface. Cleaned `tiny-cluster-prefered/mesh.toml` down to a package-only manifest and renamed its package entry to `tiny-cluster-prefered` so stale manifest clustering and the duplicate canonical package name stop competing with the current dogfood examples. The only debug turn was a failed first replay caused by a literal `/status` substring still present in the tiny-cluster README; I removed that wording and reran the exact smoke rail to green.

## Verification

Ran the task verification command from the slice plan exactly: `cargo run -q -p meshc -- test tiny-cluster/tests && cargo run -q -p meshc -- build tiny-cluster && cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof`. The passing replay proved the tiny-cluster smoke suite (3 assertions), tiny-cluster build, cluster-proof smoke suite (5 assertions including packaging/deleted-helper guards), and cluster-proof build all completed successfully. As an intermediate task, I ran the T03 slice rail only; later S04 task rails were not run yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- test tiny-cluster/tests && cargo run -q -p meshc -- build tiny-cluster && cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 22400ms |


## Deviations

None.

## Known Issues

Verification still emits the pre-existing Rust warning that `compiler/mesh-parser/src/parser/items.rs::ClusteredDeclPrefixState::LegacyCompatValid` is dead code. It did not affect the package rails and was not part of this task.

## Files Created/Modified

- `tiny-cluster/work.mpl`
- `tiny-cluster/tests/work.test.mpl`
- `tiny-cluster/README.md`
- `cluster-proof/work.mpl`
- `cluster-proof/tests/work.test.mpl`
- `cluster-proof/README.md`
- `tiny-cluster-prefered/mesh.toml`


## Deviations
None.

## Known Issues
Verification still emits the pre-existing Rust warning that `compiler/mesh-parser/src/parser/items.rs::ClusteredDeclPrefixState::LegacyCompatValid` is dead code. It did not affect the package rails and was not part of this task.
