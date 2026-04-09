---
id: T04
parent: S01
milestone: M045
provides: []
requires: []
affects: []
key_files: ["cluster-proof/main.mpl", "cluster-proof/config.mpl", "cluster-proof/tests/config.test.mpl", "cluster-proof/docker-entrypoint.sh", "compiler/meshc/tests/e2e_m045_s01.rs", "compiler/meshc/tests/e2e_m044_s05.rs", "scripts/verify-m045-s01.sh", "cluster-proof/README.md"]
key_decisions: ["D214: cluster-proof Mesh code now delegates clustered bootstrap to `Node.start_from_env()`, while `cluster-proof/docker-entrypoint.sh` only preflights continuity topology and defers bootstrap validation/errors to the runtime helper."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed all three planned verification commands: `cargo run -q -p meshc -- build cluster-proof`, `cargo run -q -p meshc -- test cluster-proof/tests`, and `bash scripts/verify-m045-s01.sh`. The assembled verifier also replayed `cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture`, `bash scripts/verify-m044-s03.sh`, and `cargo test -p meshc --test e2e_m044_s05 m044_s05_public_contract_ -- --nocapture` without zero-test or artifact-drift failures."
completed_at: 2026-03-30T18:57:57.520Z
blocker_discovered: false
---

# T04: Moved `cluster-proof` onto `Node.start_from_env()` and added the assembled `verify-m045-s01.sh` acceptance rail.

> Moved `cluster-proof` onto `Node.start_from_env()` and added the assembled `verify-m045-s01.sh` acceptance rail.

## What Happened
---
id: T04
parent: S01
milestone: M045
key_files:
  - cluster-proof/main.mpl
  - cluster-proof/config.mpl
  - cluster-proof/tests/config.test.mpl
  - cluster-proof/docker-entrypoint.sh
  - compiler/meshc/tests/e2e_m045_s01.rs
  - compiler/meshc/tests/e2e_m044_s05.rs
  - scripts/verify-m045-s01.sh
  - cluster-proof/README.md
key_decisions:
  - D214: cluster-proof Mesh code now delegates clustered bootstrap to `Node.start_from_env()`, while `cluster-proof/docker-entrypoint.sh` only preflights continuity topology and defers bootstrap validation/errors to the runtime helper.
duration: ""
verification_result: passed
completed_at: 2026-03-30T18:57:57.523Z
blocker_discovered: false
---

# T04: Moved `cluster-proof` onto `Node.start_from_env()` and added the assembled `verify-m045-s01.sh` acceptance rail.

**Moved `cluster-proof` onto `Node.start_from_env()` and added the assembled `verify-m045-s01.sh` acceptance rail.**

## What Happened

Rewrote `cluster-proof/main.mpl` to consume `Node.start_from_env()` directly and stop hand-rolling cluster mode detection, identity resolution, and direct `Node.start(...)` orchestration in Mesh code. Shrunk `cluster-proof/config.mpl` to the remaining continuity and durability helpers, switched the runtime mode lookup for that layer to `Node.self()`, trimmed `cluster-proof/tests/config.test.mpl` to the reduced helper surface, and simplified `cluster-proof/docker-entrypoint.sh` so it only keeps the continuity preflight the runtime helper does not yet validate. Extended `compiler/meshc/tests/e2e_m045_s01.rs` with `cluster-proof` source/package coverage, added a protected M044 source-contract test in `compiler/meshc/tests/e2e_m044_s05.rs`, added `scripts/verify-m045-s01.sh` as the assembled fail-closed acceptance rail, and updated `cluster-proof/README.md` to state the new bootstrap boundary explicitly.

## Verification

Passed all three planned verification commands: `cargo run -q -p meshc -- build cluster-proof`, `cargo run -q -p meshc -- test cluster-proof/tests`, and `bash scripts/verify-m045-s01.sh`. The assembled verifier also replayed `cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture`, `bash scripts/verify-m044-s03.sh`, and `cargo test -p meshc --test e2e_m044_s05 m044_s05_public_contract_ -- --nocapture` without zero-test or artifact-drift failures.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 7790ms |
| 2 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 9351ms |
| 3 | `bash scripts/verify-m045-s01.sh` | 0 | ✅ pass | 451184ms |


## Deviations

Also updated `cluster-proof/README.md` so the public runbook states the new `Node.start_from_env()` bootstrap boundary explicitly. That was not listed in the expected outputs but keeps the protected public-contract docs truthful.

## Known Issues

None.

## Files Created/Modified

- `cluster-proof/main.mpl`
- `cluster-proof/config.mpl`
- `cluster-proof/tests/config.test.mpl`
- `cluster-proof/docker-entrypoint.sh`
- `compiler/meshc/tests/e2e_m045_s01.rs`
- `compiler/meshc/tests/e2e_m044_s05.rs`
- `scripts/verify-m045-s01.sh`
- `cluster-proof/README.md`


## Deviations
Also updated `cluster-proof/README.md` so the public runbook states the new `Node.start_from_env()` bootstrap boundary explicitly. That was not listed in the expected outputs but keeps the protected public-contract docs truthful.

## Known Issues
None.
