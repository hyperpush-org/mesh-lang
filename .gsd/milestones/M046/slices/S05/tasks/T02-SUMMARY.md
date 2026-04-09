---
id: T02
parent: S05
milestone: M046
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/support/m046_route_free.rs", "compiler/meshc/tests/e2e_m046_s05.rs", "compiler/meshc/tests/e2e_m044_s03.rs", "compiler/meshc/tests/e2e_m045_s01.rs", "compiler/meshc/tests/e2e_m045_s02.rs", "compiler/meshc/tests/e2e_m045_s03.rs"]
key_decisions: ["Recorded D254 to keep the live generated-scaffold runtime proof centralized in e2e_m046_s05 while older scaffold rails shrink to route-free contract/delegation guards.", "Used support/m046_route_free.rs::init_clustered_project plus retained generated-project trees as the shared scaffold setup seam for route-free tests."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the exact task-plan verification chain and it passed end to end: cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture; cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture; cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture; cargo test -p meshc --test e2e_m045_s03 m045_s03_ -- --nocapture; cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture. The new e2e_m046_s05 live proof retained generated-project source, build metadata, cluster status/continuity/diagnostics JSON, and node logs under .tmp/m046-s05/...."
completed_at: 2026-04-01T00:58:38.185Z
blocker_discovered: false
---

# T02: Added the authoritative route-free scaffold equal-surface rail, moved generated-scaffold setup into the shared harness, and narrowed the older scaffold regressions so they fail on drift without reviving deleted HTTP routes.

> Added the authoritative route-free scaffold equal-surface rail, moved generated-scaffold setup into the shared harness, and narrowed the older scaffold regressions so they fail on drift without reviving deleted HTTP routes.

## What Happened
---
id: T02
parent: S05
milestone: M046
key_files:
  - compiler/meshc/tests/support/m046_route_free.rs
  - compiler/meshc/tests/e2e_m046_s05.rs
  - compiler/meshc/tests/e2e_m044_s03.rs
  - compiler/meshc/tests/e2e_m045_s01.rs
  - compiler/meshc/tests/e2e_m045_s02.rs
  - compiler/meshc/tests/e2e_m045_s03.rs
key_decisions:
  - Recorded D254 to keep the live generated-scaffold runtime proof centralized in e2e_m046_s05 while older scaffold rails shrink to route-free contract/delegation guards.
  - Used support/m046_route_free.rs::init_clustered_project plus retained generated-project trees as the shared scaffold setup seam for route-free tests.
duration: ""
verification_result: passed
completed_at: 2026-04-01T00:58:38.187Z
blocker_discovered: false
---

# T02: Added the authoritative route-free scaffold equal-surface rail, moved generated-scaffold setup into the shared harness, and narrowed the older scaffold regressions so they fail on drift without reviving deleted HTTP routes.

**Added the authoritative route-free scaffold equal-surface rail, moved generated-scaffold setup into the shared harness, and narrowed the older scaffold regressions so they fail on drift without reviving deleted HTTP routes.**

## What Happened

Extended compiler/meshc/tests/support/m046_route_free.rs with shared clustered-scaffold init/archive helpers, then added compiler/meshc/tests/e2e_m046_s05.rs as the new authoritative equal-surface rail. That rail proves generated scaffold parity against the route-free proof-package work contract, builds the generated scaffold to a temp output path, boots it on two nodes, and inspects startup only through meshc cluster status, continuity list, single-record continuity lookup, and diagnostics while retaining .tmp/m046-s05/... artifacts. Rewrote the historical scaffold-era rails so they no longer depend on deleted /health, /work, or app-owned continuity glue: e2e_m044_s03 now guards the route-free scaffold source/build contract, e2e_m045_s01 now pins the current scaffold and cluster-proof bootstrap/package contract, e2e_m045_s02 now proves scaffold startup completion through CLI-only continuity surfaces and route-free parity checks, and e2e_m045_s03 now keeps lightweight failover helper/delegation guards around the new shared S05 proof seam.

## Verification

Ran the exact task-plan verification chain and it passed end to end: cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture; cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture; cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture; cargo test -p meshc --test e2e_m045_s03 m045_s03_ -- --nocapture; cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture. The new e2e_m046_s05 live proof retained generated-project source, build metadata, cluster status/continuity/diagnostics JSON, and node logs under .tmp/m046-s05/....

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture` | 0 | ✅ pass | 7746ms |
| 2 | `cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture` | 0 | ✅ pass | 10815ms |
| 3 | `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture` | 0 | ✅ pass | 10267ms |
| 4 | `cargo test -p meshc --test e2e_m045_s03 m045_s03_ -- --nocapture` | 0 | ✅ pass | 6132ms |
| 5 | `cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture` | 0 | ✅ pass | 12314ms |


## Deviations

Narrowed e2e_m045_s03 from a direct live scaffold failover replay to a fail-closed helper/delegation contract around the new S05 authoritative rail, which the task plan allowed once one truthful shared runtime seam existed.

## Known Issues

Historical files still emit some unused-code warnings because they now act as narrower contract rails around the shared S05 proof seam. No failing verification remains for this task.

## Files Created/Modified

- `compiler/meshc/tests/support/m046_route_free.rs`
- `compiler/meshc/tests/e2e_m046_s05.rs`
- `compiler/meshc/tests/e2e_m044_s03.rs`
- `compiler/meshc/tests/e2e_m045_s01.rs`
- `compiler/meshc/tests/e2e_m045_s02.rs`
- `compiler/meshc/tests/e2e_m045_s03.rs`


## Deviations
Narrowed e2e_m045_s03 from a direct live scaffold failover replay to a fail-closed helper/delegation contract around the new S05 authoritative rail, which the task plan allowed once one truthful shared runtime seam existed.

## Known Issues
Historical files still emit some unused-code warnings because they now act as narrower contract rails around the shared S05 proof seam. No failing verification remains for this task.
