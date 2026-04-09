---
id: T02
parent: S03
milestone: M046
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/e2e_m046_s03.rs", ".gsd/milestones/M046/slices/S03/tasks/T02-SUMMARY.md"]
key_decisions: ["Build `tiny-cluster/` directly from the repo path and archive package sources into `.tmp/m046-s03/...` before node launch instead of recreating temp source fixtures.", "Treat `meshc cluster continuity` list/single-record output plus `meshc cluster diagnostics` as the startup source of truth, while retaining the last CLI JSON/logs and node stdout/stderr for failure diagnosis."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the slice-level `tiny-cluster` package checks and both focused S03 e2e filters. `cargo run -q -p meshc -- build tiny-cluster`, `cargo run -q -p meshc -- test tiny-cluster/tests`, `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_ -- --nocapture`, and `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_startup_ -- --nocapture` all passed. I also formatted `compiler/meshc/tests/e2e_m046_s03.rs` directly with `rustfmt` because Rust LSP tooling is not available in this workspace."
completed_at: 2026-03-31T20:25:23.526Z
blocker_discovered: false
---

# T02: Added a repo-backed `tiny-cluster` e2e rail that proves startup/status truth through Mesh CLI surfaces.

> Added a repo-backed `tiny-cluster` e2e rail that proves startup/status truth through Mesh CLI surfaces.

## What Happened
---
id: T02
parent: S03
milestone: M046
key_files:
  - compiler/meshc/tests/e2e_m046_s03.rs
  - .gsd/milestones/M046/slices/S03/tasks/T02-SUMMARY.md
key_decisions:
  - Build `tiny-cluster/` directly from the repo path and archive package sources into `.tmp/m046-s03/...` before node launch instead of recreating temp source fixtures.
  - Treat `meshc cluster continuity` list/single-record output plus `meshc cluster diagnostics` as the startup source of truth, while retaining the last CLI JSON/logs and node stdout/stderr for failure diagnosis.
duration: ""
verification_result: passed
completed_at: 2026-03-31T20:25:23.534Z
blocker_discovered: false
---

# T02: Added a repo-backed `tiny-cluster` e2e rail that proves startup/status truth through Mesh CLI surfaces.

**Added a repo-backed `tiny-cluster` e2e rail that proves startup/status truth through Mesh CLI surfaces.**

## What Happened

Added `compiler/meshc/tests/e2e_m046_s03.rs` as the S03 real-package rail. The new test file reads the repo-owned `tiny-cluster/` package from disk, copies `mesh.toml`, `main.mpl`, `work.mpl`, `README.md`, and `tests/work.test.mpl` into `.tmp/m046-s03/...` bundles, and fails closed if the package drifts back toward `[cluster]`, `HTTP.serve(...)`, `/work`, `/status`, `/health`, or explicit continuity control flow. It also proves the package still exposes exactly one `Node.start_from_env()` bootstrap seam and a visible `1 + 1` declared-work body.

On the runtime side, the new startup rail builds the real repo package to an artifact-local binary, boots two nodes with no HTTP routes, waits for runtime-owned cluster membership, discovers the deterministic startup record by `declared_handler_runtime_name == "Work.execute_declared_work"` from `meshc cluster continuity` list mode on both nodes, then inspects the completed record through single-record continuity output and diagnostics. The proof asserts one logical startup record, mirrored completion on both nodes, and runtime-owned `startup_trigger` / `startup_completed` diagnostics, while retaining build logs, CLI JSON/log snapshots, scenario metadata, and per-node stdout/stderr under `.tmp/m046-s03/...`.

## Verification

Ran the slice-level `tiny-cluster` package checks and both focused S03 e2e filters. `cargo run -q -p meshc -- build tiny-cluster`, `cargo run -q -p meshc -- test tiny-cluster/tests`, `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_ -- --nocapture`, and `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_startup_ -- --nocapture` all passed. I also formatted `compiler/meshc/tests/e2e_m046_s03.rs` directly with `rustfmt` because Rust LSP tooling is not available in this workspace.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- build tiny-cluster` | 0 | ✅ pass | 7210ms |
| 2 | `cargo run -q -p meshc -- test tiny-cluster/tests` | 0 | ✅ pass | 7510ms |
| 3 | `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_ -- --nocapture` | 0 | ✅ pass | 14690ms |
| 4 | `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_startup_ -- --nocapture` | 0 | ✅ pass | 16100ms |


## Deviations

None.

## Known Issues

`cargo fmt --check --all` currently reports pre-existing formatting drift in `compiler/meshc/tests/e2e_m046_s02.rs`. I formatted only `compiler/meshc/tests/e2e_m046_s03.rs` to avoid unrelated churn in this task.

## Files Created/Modified

- `compiler/meshc/tests/e2e_m046_s03.rs`
- `.gsd/milestones/M046/slices/S03/tasks/T02-SUMMARY.md`


## Deviations
None.

## Known Issues
`cargo fmt --check --all` currently reports pre-existing formatting drift in `compiler/meshc/tests/e2e_m046_s02.rs`. I formatted only `compiler/meshc/tests/e2e_m046_s03.rs` to avoid unrelated churn in this task.
