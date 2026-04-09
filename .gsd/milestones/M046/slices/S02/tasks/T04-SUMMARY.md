---
id: T04
parent: S02
milestone: M046
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/e2e_m046_s02.rs", "/Users/sn0w/Documents/dev/mesh-lang/.gsd/milestones/M046/slices/S02/tasks/T04-SUMMARY.md"]
key_decisions: ["Kept the new S02 proof on the source-level `clustered(work)` surface with a package-only `mesh.toml`, then proved dedupe and completion from CLI continuity/diagnostics instead of reintroducing app routes or submit glue."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran a focused new-test replay first, then the full `e2e_m046_s02` target and the retained M044 rails from the task plan. All passed: `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_tiny_route_free_startup_dedupes_on_two_nodes -- --nocapture`, `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture`, and `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`."
completed_at: 2026-03-31T18:38:39.844Z
blocker_discovered: false
---

# T04: Added a dual-node source-level startup proof that boots route-free nodes, auto-runs trivial clustered work, and verifies deduped completion plus diagnostics entirely through `meshc cluster ...`.

> Added a dual-node source-level startup proof that boots route-free nodes, auto-runs trivial clustered work, and verifies deduped completion plus diagnostics entirely through `meshc cluster ...`.

## What Happened
---
id: T04
parent: S02
milestone: M046
key_files:
  - compiler/meshc/tests/e2e_m046_s02.rs
  - /Users/sn0w/Documents/dev/mesh-lang/.gsd/milestones/M046/slices/S02/tasks/T04-SUMMARY.md
key_decisions:
  - Kept the new S02 proof on the source-level `clustered(work)` surface with a package-only `mesh.toml`, then proved dedupe and completion from CLI continuity/diagnostics instead of reintroducing app routes or submit glue.
duration: ""
verification_result: passed
completed_at: 2026-03-31T18:38:39.857Z
blocker_discovered: false
---

# T04: Added a dual-node source-level startup proof that boots route-free nodes, auto-runs trivial clustered work, and verifies deduped completion plus diagnostics entirely through `meshc cluster ...`.

**Added a dual-node source-level startup proof that boots route-free nodes, auto-runs trivial clustered work, and verifies deduped completion plus diagnostics entirely through `meshc cluster ...`.**

## What Happened

Extended `compiler/meshc/tests/e2e_m046_s02.rs` with a tiny route-free fixture that uses only `Node.start_from_env()` plus a source-level `clustered(work)` declaration whose body is the trivial `1 + 1`. Added small helpers for dual-stack cluster port selection, labeled route-free process logs, generic `meshc cluster status` membership waiting, single-record continuity polling, and diagnostics polling. The new rail builds a temp project, archives `mesh.toml`/`main.mpl`/`work.mpl`/`build.log` into `.tmp/m046-s02/...`, boots primary and standby nodes from the same binary, proves a single continuity record exists for `Work.execute_declared_work`, verifies completion from both nodes, and requires startup diagnostics to show `startup_trigger` plus `startup_completed` without any startup rejection or convergence-timeout fallback. Updated the earlier single-node route-free tests to use the refactored labeled spawn helper so the full M046 S02 target still passes.

## Verification

Ran a focused new-test replay first, then the full `e2e_m046_s02` target and the retained M044 rails from the task plan. All passed: `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_tiny_route_free_startup_dedupes_on_two_nodes -- --nocapture`, `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture`, and `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_tiny_route_free_startup_dedupes_on_two_nodes -- --nocapture` | 0 | ✅ pass | 26500ms |
| 2 | `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture` | 0 | ✅ pass | 15500ms |
| 3 | `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture` | 0 | ✅ pass | 23100ms |
| 4 | `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture` | 0 | ✅ pass | 26600ms |
| 5 | `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture` | 0 | ✅ pass | 14800ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `compiler/meshc/tests/e2e_m046_s02.rs`
- `/Users/sn0w/Documents/dev/mesh-lang/.gsd/milestones/M046/slices/S02/tasks/T04-SUMMARY.md`


## Deviations
None.

## Known Issues
None.
