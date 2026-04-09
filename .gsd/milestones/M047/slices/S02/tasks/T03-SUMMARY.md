---
id: T03
parent: S02
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/e2e_m047_s02.rs", ".gsd/milestones/M047/slices/S02/tasks/T03-SUMMARY.md"]
key_decisions: ["Kept the M047 proof local to a new meshc integration test and reused the shared `m046_route_free` helper without widening the helper surface.", "Archived emitted LLVM from the explicit output directory because `meshc build --emit-llvm --output ...` writes the `.ll` beside the output binary, not inside the temp project tree."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the new proof target with `cargo test -p meshc --test e2e_m047_s02 -- --nocapture`, then replayed `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture` to confirm the shared M046 route-free contract stayed green. The M047 rail itself checked LLVM registration markers, continuity JSON and human output, single-record continuity truth, startup diagnostics, explicit unsupported-fanout rejection, and retained proof artifacts under `.tmp/m047-s02/...`."
completed_at: 2026-04-01T07:24:57.673Z
blocker_discovered: false
---

# T03: Added M047 end-to-end coverage proving ordinary `@cluster` functions keep generic runtime names and truthful replication counts through LLVM registration and `meshc cluster continuity`.

> Added M047 end-to-end coverage proving ordinary `@cluster` functions keep generic runtime names and truthful replication counts through LLVM registration and `meshc cluster continuity`.

## What Happened
---
id: T03
parent: S02
milestone: M047
key_files:
  - compiler/meshc/tests/e2e_m047_s02.rs
  - .gsd/milestones/M047/slices/S02/tasks/T03-SUMMARY.md
key_decisions:
  - Kept the M047 proof local to a new meshc integration test and reused the shared `m046_route_free` helper without widening the helper surface.
  - Archived emitted LLVM from the explicit output directory because `meshc build --emit-llvm --output ...` writes the `.ll` beside the output binary, not inside the temp project tree.
duration: ""
verification_result: passed
completed_at: 2026-04-01T07:24:57.674Z
blocker_discovered: false
---

# T03: Added M047 end-to-end coverage proving ordinary `@cluster` functions keep generic runtime names and truthful replication counts through LLVM registration and `meshc cluster continuity`.

**Added M047 end-to-end coverage proving ordinary `@cluster` functions keep generic runtime names and truthful replication counts through LLVM registration and `meshc cluster continuity`.**

## What Happened

Added `compiler/meshc/tests/e2e_m047_s02.rs` as the end-to-end proof rail for ordinary source-declared clustered functions. The new fixture builds a temporary route-free Mesh app that uses `@cluster` and `@cluster(3)` instead of the legacy `clustered(work)` story, emits LLVM IR beside a temp binary, and archives the generated sources and build output under `.tmp/m047-s02/...`. The LLVM proof asserts that generic runtime names `Work.handle_submit` and `Work.handle_retry` survive into registration markers with replication counts `2` and `3`, and that the legacy `Work.execute_declared_work` name does not appear. The runtime proof starts the compiled binary through the shared M046 helper and verifies runtime-owned continuity JSON, human continuity output, single-record output, and cluster diagnostics: the default `@cluster` record surfaces `replication_count=2` and completes locally through the single-node startup carveout, while the explicit `@cluster(3)` record stays durable and rejected with `unsupported_replication_count:3`. The only adjustment during execution was correcting the emitted LLVM archive path after observing that `meshc build --emit-llvm --output ...` writes the `.ll` beside the output binary instead of under the temp project directory.

## Verification

Verified the new proof target with `cargo test -p meshc --test e2e_m047_s02 -- --nocapture`, then replayed `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture` to confirm the shared M046 route-free contract stayed green. The M047 rail itself checked LLVM registration markers, continuity JSON and human output, single-record continuity truth, startup diagnostics, explicit unsupported-fanout rejection, and retained proof artifacts under `.tmp/m047-s02/...`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m047_s02 -- --nocapture` | 0 | ✅ pass | 11487ms |
| 2 | `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture` | 0 | ✅ pass | 11734ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `compiler/meshc/tests/e2e_m047_s02.rs`
- `.gsd/milestones/M047/slices/S02/tasks/T03-SUMMARY.md`


## Deviations
None.

## Known Issues
None.
