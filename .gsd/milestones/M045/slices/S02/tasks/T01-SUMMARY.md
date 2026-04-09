---
id: T01
parent: S02
milestone: M045
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-codegen/src/codegen/mod.rs", "compiler/mesh-rt/src/dist/node.rs", "compiler/meshc/tests/e2e_m044_s02.rs", "compiler/meshc/tests/e2e_m045_s02.rs", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M045/slices/S02/tasks/T01-SUMMARY.md"]
key_decisions: ["Register only manifest-approved declared wrapper executable symbols for remote spawn; keep other compiler-internal `__*` helpers hidden.", "Prove remote execution from the submit response's actual `owner_node`, then require completed status on both ingress and owner nodes plus owner-node execution logs."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the existing declared-work rail with `cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture`, the new focused remote-owner regression with `cargo test -p meshc --test e2e_m045_s02 m045_s02_declared_work_remote_spawn_ -- --nocapture`, and the current slice-level checks that already exist: `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture` and `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture`. The assembled slice verifier `bash scripts/verify-m045-s02.sh` still fails closed because the script does not exist yet; that is expected until T03 lands."
completed_at: 2026-03-30T20:03:05.522Z
blocker_discovered: false
---

# T01: Registered manifest-approved declared wrapper symbols for remote spawn and added a two-node remote-owner completion regression.

> Registered manifest-approved declared wrapper symbols for remote spawn and added a two-node remote-owner completion regression.

## What Happened
---
id: T01
parent: S02
milestone: M045
key_files:
  - compiler/mesh-codegen/src/codegen/mod.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/meshc/tests/e2e_m044_s02.rs
  - compiler/meshc/tests/e2e_m045_s02.rs
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M045/slices/S02/tasks/T01-SUMMARY.md
key_decisions:
  - Register only manifest-approved declared wrapper executable symbols for remote spawn; keep other compiler-internal `__*` helpers hidden.
  - Prove remote execution from the submit response's actual `owner_node`, then require completed status on both ingress and owner nodes plus owner-node execution logs.
duration: ""
verification_result: mixed
completed_at: 2026-03-30T20:03:05.525Z
blocker_discovered: false
---

# T01: Registered manifest-approved declared wrapper symbols for remote spawn and added a two-node remote-owner completion regression.

**Registered manifest-approved declared wrapper symbols for remote spawn and added a two-node remote-owner completion regression.**

## What Happened

I traced the declared-handler seam from `prepare_declared_runtime_handlers(...)` through startup registration and the remote `DIST_SPAWN` handler. The functional change is in codegen: startup remote-spawn registration still excludes ordinary compiler-internal `__*` symbols, but now explicitly includes manifest-approved declared wrapper executable symbols so the owner node can resolve the `__declared_work_*` name that `submit_declared_work(...)` dispatches remotely. On the runtime side I kept lookup fail-closed and improved the rejection diagnostic when a declared executable exists in the manifest-approved registry but is missing from the remote-spawn registry. I then tightened the existing M044 LLVM regression so declared work/service wrappers must remain remote-spawnable while raw internal helpers stay hidden, and added `compiler/meshc/tests/e2e_m045_s02.rs` with a focused two-node `cluster-proof` regression. That new rail uses the current clustered bootstrap env contract, retries until the runtime actually reports `owner_node != ingress_node`, and then requires completed continuity truth on both ingress and owner nodes plus an owner-node execution log, while asserting the absence of `declared_work_remote_spawn_failed` and `function not found __declared_work_` in the retained logs.

## Verification

Verified the existing declared-work rail with `cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture`, the new focused remote-owner regression with `cargo test -p meshc --test e2e_m045_s02 m045_s02_declared_work_remote_spawn_ -- --nocapture`, and the current slice-level checks that already exist: `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture` and `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture`. The assembled slice verifier `bash scripts/verify-m045-s02.sh` still fails closed because the script does not exist yet; that is expected until T03 lands.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture` | 0 | ✅ pass | 46700ms |
| 2 | `cargo test -p meshc --test e2e_m045_s02 m045_s02_declared_work_remote_spawn_ -- --nocapture` | 0 | ✅ pass | 22900ms |
| 3 | `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture` | 0 | ✅ pass | 14000ms |
| 4 | `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture` | 0 | ✅ pass | 22300ms |
| 5 | `bash scripts/verify-m045-s02.sh` | 127 | ❌ fail | 22ms |


## Deviations

None.

## Known Issues

`scripts/verify-m045-s02.sh` does not exist yet, so the assembled slice-level verifier still fails closed with `No such file or directory`. That verifier belongs to T03.

## Files Created/Modified

- `compiler/mesh-codegen/src/codegen/mod.rs`
- `compiler/mesh-rt/src/dist/node.rs`
- `compiler/meshc/tests/e2e_m044_s02.rs`
- `compiler/meshc/tests/e2e_m045_s02.rs`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M045/slices/S02/tasks/T01-SUMMARY.md`


## Deviations
None.

## Known Issues
`scripts/verify-m045-s02.sh` does not exist yet, so the assembled slice-level verifier still fails closed with `No such file or directory`. That verifier belongs to T03.
