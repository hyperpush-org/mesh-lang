---
id: T02
parent: S02
milestone: M045
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-rt/src/dist/node.rs", "compiler/mesh-rt/src/dist/continuity.rs", "compiler/mesh-codegen/src/codegen/expr.rs", "compiler/mesh-codegen/src/codegen/intrinsics.rs", "compiler/mesh-pkg/src/scaffold.rs", "compiler/meshc/tests/tooling_e2e.rs", "compiler/meshc/tests/e2e_m045_s01.rs", "compiler/meshc/tests/e2e_m045_s02.rs"]
key_decisions: ["Complete declared work in the generated declared-work actor wrapper via a dedicated runtime helper, so scaffolded apps do not need `Continuity.mark_completed(...)` glue.", "Shrink the clustered scaffold by inlining the manifest-declared runtime target string and keeping runtime truth on `meshc cluster status` / `meshc cluster continuity` instead of app-owned status or placement helpers."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the required task rails with `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture` and `cargo test -p meshc --test e2e_m045_s02 m045_s02_scaffold_runtime_completion_ -- --nocapture`. I also reran the existing remote-owner declared-work rail to prove the new wrapper-owned completion did not break the older cluster-proof path, rechecked the S01 scaffold/source contract, and ran the mesh-pkg scaffold unit covering the generated clustered project surface."
completed_at: 2026-03-30T20:25:26.875Z
blocker_discovered: false
---

# T02: Moved declared-work completion into the runtime/codegen seam and shrank the clustered scaffold to a tiny runtime-owned example.

> Moved declared-work completion into the runtime/codegen seam and shrank the clustered scaffold to a tiny runtime-owned example.

## What Happened
---
id: T02
parent: S02
milestone: M045
key_files:
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-codegen/src/codegen/expr.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/tests/tooling_e2e.rs
  - compiler/meshc/tests/e2e_m045_s01.rs
  - compiler/meshc/tests/e2e_m045_s02.rs
key_decisions:
  - Complete declared work in the generated declared-work actor wrapper via a dedicated runtime helper, so scaffolded apps do not need `Continuity.mark_completed(...)` glue.
  - Shrink the clustered scaffold by inlining the manifest-declared runtime target string and keeping runtime truth on `meshc cluster status` / `meshc cluster continuity` instead of app-owned status or placement helpers.
duration: ""
verification_result: mixed
completed_at: 2026-03-30T20:25:26.878Z
blocker_discovered: false
---

# T02: Moved declared-work completion into the runtime/codegen seam and shrank the clustered scaffold to a tiny runtime-owned example.

**Moved declared-work completion into the runtime/codegen seam and shrank the clustered scaffold to a tiny runtime-owned example.**

## What Happened

I attached declared-work completion to the generated actor-wrapper path instead of reintroducing app-side completion calls. The runtime now exposes a narrow declared-work completion helper that resolves the truthful execution node (`node_state().name` or the standalone fallback) and records completion through the existing continuity registry. Codegen calls that helper automatically after a `__declared_work_*` body returns, so successful declared work closes its continuity record without any scaffold-owned `Continuity.mark_completed(...)` glue.

With that seam in place, I rewrote the clustered scaffold to stay honestly small: `main.mpl` now submits directly to `"Work.execute_declared_work"`, `work.mpl` only defines the declared work body, and the README explicitly points users at runtime-owned `meshc cluster status` / `meshc cluster continuity` truth while stating that the generated work file does not call `Continuity.mark_completed(...)`.

I then tightened the contract rails around that shape. `tooling_e2e`, the mesh-pkg scaffold unit, and the M045/S01 scaffold/source contract now all reject leaked `declared_work_target`, `Continuity.mark_completed`, proof-app literals, and app-owned status helpers. In `e2e_m045_s02.rs` I added one source-contract test and one runtime completion test for the generated scaffold. The runtime test initializes a clustered scaffold project, builds it, boots a single runtime-owned node, submits `/work/:request_key`, waits for `meshc cluster continuity --json` to report `phase=completed`, and verifies the new continuity stderr surface. The first pass exposed a truthful CLI detail — `meshc cluster continuity --json` returns the record directly rather than an `ok` wrapper — so I fixed the test to match the real contract and reran the rail green.

## Verification

Verified the required task rails with `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture` and `cargo test -p meshc --test e2e_m045_s02 m045_s02_scaffold_runtime_completion_ -- --nocapture`. I also reran the existing remote-owner declared-work rail to prove the new wrapper-owned completion did not break the older cluster-proof path, rechecked the S01 scaffold/source contract, and ran the mesh-pkg scaffold unit covering the generated clustered project surface.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture` | 0 | ✅ pass | 35400ms |
| 2 | `cargo test -p meshc --test e2e_m045_s02 m045_s02_scaffold_runtime_completion_ -- --nocapture` | 101 | ❌ fail | 25700ms |
| 3 | `cargo test -p meshc --test e2e_m045_s02 m045_s02_scaffold_runtime_completion_ -- --nocapture` | 0 | ✅ pass | 20800ms |
| 4 | `cargo test -p meshc --test e2e_m045_s02 m045_s02_declared_work_remote_spawn_ -- --nocapture` | 0 | ✅ pass | 20100ms |
| 5 | `cargo test -p meshc --test e2e_m045_s01 m045_s01_bootstrap_api_scaffold_contract_uses_runtime_owned_bootstrap -- --nocapture` | 0 | ✅ pass | 13100ms |
| 6 | `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture` | 0 | ✅ pass | 8300ms |


## Deviations

None.

## Known Issues

`scripts/verify-m045-s02.sh` still does not exist; the assembled slice verifier remains T03 work.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/mesh-codegen/src/codegen/expr.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/e2e_m045_s01.rs`
- `compiler/meshc/tests/e2e_m045_s02.rs`


## Deviations
None.

## Known Issues
`scripts/verify-m045-s02.sh` still does not exist; the assembled slice verifier remains T03 work.
