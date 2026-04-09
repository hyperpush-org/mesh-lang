---
id: T04
parent: S03
milestone: M047
provides: []
requires: []
affects: []
key_files: [".gsd/milestones/M047/slices/S03/tasks/T04-SUMMARY.md", "compiler/mesh-typeck/src/builtins.rs", "compiler/mesh-typeck/src/infer.rs", "compiler/mesh-rt/src/http/server.rs"]
key_decisions: ["Treat the current T04 failure as a plan-invalidating upstream blocker: do not patch mesh-rt reply transport until `HTTP.clustered(...)` exists as a compiler-known route wrapper that lowers real clustered route shims and registrations."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the blocker directly by reading the compiler/typeck/runtime surfaces and replaying the named rails. `cargo test -p mesh-rt m047_s03 -- --nocapture` produced `running 0 tests`, `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` failed with `no test target named e2e_m047_s03`, and the HTTP stdlib/typeck sources still lack any `HTTP.clustered(...)` surface."
completed_at: 2026-04-01T08:21:59.078Z
blocker_discovered: true
---

# T04: Documented the M047/S03/T04 blocker: the current tree still lacks the `HTTP.clustered(...)` compiler/lowering seam that T04 depends on, so no runtime clustered-route execution work shipped.

> Documented the M047/S03/T04 blocker: the current tree still lacks the `HTTP.clustered(...)` compiler/lowering seam that T04 depends on, so no runtime clustered-route execution work shipped.

## What Happened
---
id: T04
parent: S03
milestone: M047
key_files:
  - .gsd/milestones/M047/slices/S03/tasks/T04-SUMMARY.md
  - compiler/mesh-typeck/src/builtins.rs
  - compiler/mesh-typeck/src/infer.rs
  - compiler/mesh-rt/src/http/server.rs
key_decisions:
  - Treat the current T04 failure as a plan-invalidating upstream blocker: do not patch mesh-rt reply transport until `HTTP.clustered(...)` exists as a compiler-known route wrapper that lowers real clustered route shims and registrations.
duration: ""
verification_result: mixed
completed_at: 2026-04-01T08:21:59.079Z
blocker_discovered: true
---

# T04: Documented the M047/S03/T04 blocker: the current tree still lacks the `HTTP.clustered(...)` compiler/lowering seam that T04 depends on, so no runtime clustered-route execution work shipped.

**Documented the M047/S03/T04 blocker: the current tree still lacks the `HTTP.clustered(...)` compiler/lowering seam that T04 depends on, so no runtime clustered-route execution work shipped.**

## What Happened

I verified the local tree against the T04 contract before patching runtime code. The blocker is upstream and concrete: `compiler/mesh-typeck/src/builtins.rs` and `compiler/mesh-typeck/src/infer.rs` still expose only the ordinary `HTTP.route` / `HTTP.on_*` surfaces and do not define `HTTP.clustered(...)`; repo searches over `compiler/mesh-codegen/` found no clustered HTTP route shim or route-wrapper symbols; and `compiler/mesh-rt/src/http/server.rs` still executes matched handlers directly through `call_handler(...)`. The planned verification rails also prove the gap: `cargo test -p mesh-rt m047_s03 -- --nocapture` built `mesh-rt` but ran 0 tests, and `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` failed because the target does not exist. Because T04 explicitly depends on generated route shims and declared-handler registrations from earlier tasks, the remaining slice plan is invalid against the current checkout until the missing T02/T03 compiler and lowering work actually lands.

## Verification

Verified the blocker directly by reading the compiler/typeck/runtime surfaces and replaying the named rails. `cargo test -p mesh-rt m047_s03 -- --nocapture` produced `running 0 tests`, `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` failed with `no test target named e2e_m047_s03`, and the HTTP stdlib/typeck sources still lack any `HTTP.clustered(...)` surface.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `rg -n "clustered" compiler/mesh-typeck/src compiler/mesh-codegen/src compiler/mesh-parser/src -g '*.rs'` | 0 | ✅ pass | 20ms |
| 2 | `cargo test -p mesh-rt m047_s03 -- --nocapture` | 0 | ❌ fail | 18050ms |
| 3 | `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` | 101 | ❌ fail | 820ms |


## Deviations

T04 assumed T03 had already landed clustered HTTP route shims and declared-handler registrations. In the current checkout that prerequisite compiler/lowering seam is still absent, so no runtime code changes were made.

## Known Issues

`HTTP.clustered(...)` is still missing from the compiler typing surfaces, there are no clustered HTTP route shim symbols under `compiler/mesh-codegen/`, `cargo test -p mesh-rt m047_s03 -- --nocapture` runs 0 tests, and `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` fails with `no test target named e2e_m047_s03`.

## Files Created/Modified

- `.gsd/milestones/M047/slices/S03/tasks/T04-SUMMARY.md`
- `compiler/mesh-typeck/src/builtins.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-rt/src/http/server.rs`


## Deviations
T04 assumed T03 had already landed clustered HTTP route shims and declared-handler registrations. In the current checkout that prerequisite compiler/lowering seam is still absent, so no runtime code changes were made.

## Known Issues
`HTTP.clustered(...)` is still missing from the compiler typing surfaces, there are no clustered HTTP route shim symbols under `compiler/mesh-codegen/`, `cargo test -p mesh-rt m047_s03 -- --nocapture` runs 0 tests, and `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` fails with `no test target named e2e_m047_s03`.
