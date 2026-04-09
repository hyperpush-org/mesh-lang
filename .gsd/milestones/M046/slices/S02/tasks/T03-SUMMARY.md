---
id: T03
parent: S02
milestone: M046
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/src/cluster.rs", "compiler/mesh-rt/src/dist/continuity.rs", "compiler/meshc/tests/e2e_m046_s02.rs", "compiler/meshc/tests/e2e_m044_s03.rs"]
key_decisions: ["Exposed the continuity runtime identity through a read-only `ContinuityRecord` accessor and surfaced the exact `declared_handler_runtime_name` key on all CLI continuity renderers instead of inventing a second startup-only status field."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the new S02 CLI rail with `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_ -- --nocapture`, which passed its route-free startup-work discovery and explicit failure tests. Replayed the retained operator rail with `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`, which also passed after adding the new continuity JSON assertion."
completed_at: 2026-03-31T18:25:12.406Z
blocker_discovered: false
---

# T03: Surfaced declared runtime names on cluster continuity output and proved route-free startup discovery through the CLI.

> Surfaced declared runtime names on cluster continuity output and proved route-free startup discovery through the CLI.

## What Happened
---
id: T03
parent: S02
milestone: M046
key_files:
  - compiler/meshc/src/cluster.rs
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/meshc/tests/e2e_m046_s02.rs
  - compiler/meshc/tests/e2e_m044_s03.rs
key_decisions:
  - Exposed the continuity runtime identity through a read-only `ContinuityRecord` accessor and surfaced the exact `declared_handler_runtime_name` key on all CLI continuity renderers instead of inventing a second startup-only status field.
duration: ""
verification_result: passed
completed_at: 2026-03-31T18:25:12.407Z
blocker_discovered: false
---

# T03: Surfaced declared runtime names on cluster continuity output and proved route-free startup discovery through the CLI.

**Surfaced declared runtime names on cluster continuity output and proved route-free startup discovery through the CLI.**

## What Happened

Added `declared_handler_runtime_name` to every `meshc cluster continuity` surface by exposing the existing runtime field through a read-only `ContinuityRecord` accessor and threading it through list JSON, single-record JSON, human-readable list output, and human-readable single-record output. Strengthened the retained M044 operator proof to assert the new JSON field on a live declared-work record. Added two `m046_s02_cli_` rails that boot a route-free temporary clustered app through `Node.start_from_env()` only, let runtime-owned startup work auto-run, discover the resulting continuity records from list mode by runtime name, and then inspect the exact record through both JSON and human-readable output. The second CLI rail verifies fail-closed behavior for an empty request-key lookup and a wrong-cookie continuity query.

## Verification

Verified the new S02 CLI rail with `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_ -- --nocapture`, which passed its route-free startup-work discovery and explicit failure tests. Replayed the retained operator rail with `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`, which also passed after adding the new continuity JSON assertion.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_ -- --nocapture` | 0 | ✅ pass | 187900ms |
| 2 | `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture` | 0 | ✅ pass | 23300ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `compiler/meshc/src/cluster.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`
- `compiler/meshc/tests/e2e_m046_s02.rs`
- `compiler/meshc/tests/e2e_m044_s03.rs`


## Deviations
None.

## Known Issues
None.
