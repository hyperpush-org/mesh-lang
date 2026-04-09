---
id: T02
parent: S04
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-pkg/src/scaffold.rs", "compiler/meshc/tests/tooling_e2e.rs"]
key_decisions: ["Kept runtime-owned handler identity by preserving the clustered function name `execute_declared_work` and deleting `declared_work_runtime_name()` instead of introducing a second compatibility seam.", "Tightened clustered scaffold verification at both the mesh-pkg and meshc layers so source/readme drift fails during generation tests rather than later runtime rails."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran broadened clustered scaffold filters to prove both the happy path and the new collision/rerun negative paths, then replayed the task-plan’s exact verification commands. `cargo test -p mesh-pkg scaffold_clustered -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_clustered -- --nocapture`, `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture`, and `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture` all passed."
completed_at: 2026-04-01T09:40:30.960Z
blocker_discovered: false
---

# T02: Switched `meshc init --clustered` to emit `@cluster` source-first work declarations and tightened scaffold contract tests.

> Switched `meshc init --clustered` to emit `@cluster` source-first work declarations and tightened scaffold contract tests.

## What Happened
---
id: T02
parent: S04
milestone: M047
key_files:
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/tests/tooling_e2e.rs
key_decisions:
  - Kept runtime-owned handler identity by preserving the clustered function name `execute_declared_work` and deleting `declared_work_runtime_name()` instead of introducing a second compatibility seam.
  - Tightened clustered scaffold verification at both the mesh-pkg and meshc layers so source/readme drift fails during generation tests rather than later runtime rails.
duration: ""
verification_result: passed
completed_at: 2026-04-01T09:40:30.961Z
blocker_discovered: false
---

# T02: Switched `meshc init --clustered` to emit `@cluster` source-first work declarations and tightened scaffold contract tests.

**Switched `meshc init --clustered` to emit `@cluster` source-first work declarations and tightened scaffold contract tests.**

## What Happened

Rewrote the clustered scaffold template so generated `work.mpl` now contains only `@cluster pub fn execute_declared_work(...)` with the visible `1 + 1` body and no `declared_work_runtime_name()` helper or `clustered(work)` marker. Updated the generated README to teach the route-free source-first contract explicitly: `mesh.toml` stays package-only, `main.mpl` still boots only through `Node.start_from_env()`, runtime inspection remains CLI-owned, and the stable runtime handler name comes from the function name `execute_declared_work` rather than from a helper seam. Tightened both scaffold-layer and CLI-layer contract tests to assert the new positive markers and the absence of legacy or publicly-unshipped surfaces, and added clustered existing-directory failure coverage so rerunning `meshc init --clustered` fails cleanly with an explicit collision message instead of mutating an existing project.

## Verification

Ran broadened clustered scaffold filters to prove both the happy path and the new collision/rerun negative paths, then replayed the task-plan’s exact verification commands. `cargo test -p mesh-pkg scaffold_clustered -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_clustered -- --nocapture`, `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture`, and `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture` all passed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-pkg scaffold_clustered -- --nocapture` | 0 | ✅ pass | 3670ms |
| 2 | `cargo test -p meshc --test tooling_e2e test_init_clustered -- --nocapture` | 0 | ✅ pass | 11380ms |
| 3 | `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture` | 0 | ✅ pass | 470ms |
| 4 | `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture` | 0 | ✅ pass | 5100ms |


## Deviations

Added explicit clustered existing-directory/rerun tests alongside the planned happy-path verification so the cutover also proves the required failure mode instead of relying on the shared non-clustered collision coverage.

## Known Issues

A pre-existing `LegacyCompatValid` dead-code warning from `compiler/mesh-parser/src/parser/items.rs` still appears during the cargo test runs. It did not affect the scaffold or CLI contract verification in this task.

## Files Created/Modified

- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`


## Deviations
Added explicit clustered existing-directory/rerun tests alongside the planned happy-path verification so the cutover also proves the required failure mode instead of relying on the shared non-clustered collision coverage.

## Known Issues
A pre-existing `LegacyCompatValid` dead-code warning from `compiler/mesh-parser/src/parser/items.rs` still appears during the cargo test runs. It did not affect the scaffold or CLI contract verification in this task.
