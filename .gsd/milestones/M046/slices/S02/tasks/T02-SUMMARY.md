---
id: T02
parent: S02
milestone: M046
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-rt/src/dist/node.rs", "compiler/mesh-rt/src/lib.rs", ".gsd/DECISIONS.md", ".gsd/KNOWLEDGE.md"]
key_decisions: ["D236: Derive startup request identity from the declared runtime name, require peer stability only after a peer is actually observed, and scope the runtime-owned keepalive actor to clustered startup-work apps."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran `cargo test -p mesh-rt startup_work_ -- --nocapture` after formatting. The rail now runs four focused runtime tests and passes: registration dedupe plus stable identity, single-node/no-peer convergence, peer-flap timeout, and clustered keepalive-trigger interaction."
completed_at: 2026-03-31T18:04:37.020Z
blocker_discovered: false
---

# T02: Added runtime-owned startup submission, bounded convergence waiting, and clustered route-free keepalive for declared startup work.

> Added runtime-owned startup submission, bounded convergence waiting, and clustered route-free keepalive for declared startup work.

## What Happened
---
id: T02
parent: S02
milestone: M046
key_files:
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-rt/src/lib.rs
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D236: Derive startup request identity from the declared runtime name, require peer stability only after a peer is actually observed, and scope the runtime-owned keepalive actor to clustered startup-work apps.
duration: ""
verification_result: passed
completed_at: 2026-03-31T18:04:37.022Z
blocker_discovered: false
---

# T02: Added runtime-owned startup submission, bounded convergence waiting, and clustered route-free keepalive for declared startup work.

**Added runtime-owned startup submission, bounded convergence waiting, and clustered route-free keepalive for declared startup work.**

## What Happened

Implemented the runtime-owned startup-work path in `compiler/mesh-rt/src/dist/node.rs`. Startup registrations now derive a deterministic runtime-owned request key and payload hash from the declared runtime name, reject blank or duplicate registrations explicitly, emit startup diagnostics, and trigger runtime-owned startup actors after `mesh_main` returns. The startup actor waits boundedly for cluster convergence, only upgrades to replica-required submission after a peer has actually been observed, submits through `submit_declared_work(...)`, and then watches continuity truth until the logical startup run completes, is rejected, or is fenced. Cluster-mode startup work now also spawns a runtime-owned keepalive actor so route-free clustered apps remain inspectable through `meshc cluster ...` surfaces even when startup work rejects early. Added focused `startup_work_` runtime tests for registration dedupe, deterministic identity, convergence timeout behavior, and keepalive-trigger interaction, then recorded the resulting runtime decision and knowledge entry for downstream tasks.

## Verification

Ran `cargo test -p mesh-rt startup_work_ -- --nocapture` after formatting. The rail now runs four focused runtime tests and passes: registration dedupe plus stable identity, single-node/no-peer convergence, peer-flap timeout, and clustered keepalive-trigger interaction.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-rt startup_work_ -- --nocapture` | 0 | ✅ pass | 19630ms |


## Deviations

Used the existing actor spawn/timer surfaces in `compiler/mesh-rt/src/dist/node.rs` for route-free keepalive instead of introducing a new primitive in `compiler/mesh-rt/src/actor/mod.rs`.

## Known Issues

The `cargo test -p mesh-rt startup_work_ -- --nocapture` rail still emits unrelated pre-existing warnings from `compiler/mesh-rt/src/actor/scheduler.rs`, `compiler/mesh-rt/src/actor/service.rs`, and `compiler/mesh-rt/src/iter.rs`.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/mesh-rt/src/lib.rs`
- `.gsd/DECISIONS.md`
- `.gsd/KNOWLEDGE.md`


## Deviations
Used the existing actor spawn/timer surfaces in `compiler/mesh-rt/src/dist/node.rs` for route-free keepalive instead of introducing a new primitive in `compiler/mesh-rt/src/actor/mod.rs`.

## Known Issues
The `cargo test -p mesh-rt startup_work_ -- --nocapture` rail still emits unrelated pre-existing warnings from `compiler/mesh-rt/src/actor/scheduler.rs`, `compiler/mesh-rt/src/actor/service.rs`, and `compiler/mesh-rt/src/iter.rs`.
