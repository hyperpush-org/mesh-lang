---
id: T02
parent: S07
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-codegen/src/declared.rs", "compiler/mesh-codegen/src/mir/lower.rs", "compiler/mesh-codegen/src/codegen/mod.rs", "compiler/mesh-codegen/src/lib.rs", "compiler/meshc/src/main.rs", ".gsd/DECISIONS.md", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M047/slices/S07/tasks/T02-SUMMARY.md"]
key_decisions: ["D300: Lower `HTTP.clustered(...)` directly to deterministic `__declared_route_<runtime_name>` bare shims and derive route declared-handler plan entries from typecheck metadata instead of teaching manifest/source clustered-declaration collection about route wrappers."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed `cargo test -p mesh-codegen m047_s07 -- --nocapture` and reran the slice verification set for honest intermediate-task status. `mesh-typeck`, `mesh-lsp`, `mesh-codegen`, and the retained `e2e_m032_route_*` controls passed; `mesh-rt m047_s07` still matches 0 tests because T03 owns the runtime rails; `meshc --test e2e_m047_s07` still fails because that T04 target does not exist yet."
completed_at: 2026-04-02T00:01:06.587Z
blocker_discovered: false
---

# T02: Lowered clustered HTTP route wrappers into deterministic route shims and shared declared-handler registration.

> Lowered clustered HTTP route wrappers into deterministic route shims and shared declared-handler registration.

## What Happened
---
id: T02
parent: S07
milestone: M047
key_files:
  - compiler/mesh-codegen/src/declared.rs
  - compiler/mesh-codegen/src/mir/lower.rs
  - compiler/mesh-codegen/src/codegen/mod.rs
  - compiler/mesh-codegen/src/lib.rs
  - compiler/meshc/src/main.rs
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M047/slices/S07/tasks/T02-SUMMARY.md
key_decisions:
  - D300: Lower `HTTP.clustered(...)` directly to deterministic `__declared_route_<runtime_name>` bare shims and derive route declared-handler plan entries from typecheck metadata instead of teaching manifest/source clustered-declaration collection about route wrappers.
duration: ""
verification_result: mixed
completed_at: 2026-04-02T00:01:06.588Z
blocker_discovered: false
---

# T02: Lowered clustered HTTP route wrappers into deterministic route shims and shared declared-handler registration.

**Lowered clustered HTTP route wrappers into deterministic route shims and shared declared-handler registration.**

## What Happened

Added a route-capable declared-handler seam for clustered HTTP wrappers. `mesh-codegen` now prepares deterministic route declared-handler plan entries from typecheck wrapper metadata, rejects conflicting replication counts, validates bare route shim signatures, and keeps startup registration filtered to `Work` only. MIR lowering now intercepts `HTTP.clustered(...)` call ranges directly, rewrites them to deterministic `__declared_route_<runtime_name>` shims that preserve the public `fn(Request) -> Response` ABI, and fails closed if wrapper metadata or handler lowering drift. `meshc` now carries those route plan entries through build prep, includes shim symbols in merge reachability roots so monomorphization cannot prune them, and appends the route entries to the shared declared-handler plan beside work and service handlers. Focused `m047_s07` tests now cover dedupe/conflict handling, direct and pipe route forms, imported bare-handler identity, startup-registration exclusion, and LLVM registration markers.

## Verification

Passed `cargo test -p mesh-codegen m047_s07 -- --nocapture` and reran the slice verification set for honest intermediate-task status. `mesh-typeck`, `mesh-lsp`, `mesh-codegen`, and the retained `e2e_m032_route_*` controls passed; `mesh-rt m047_s07` still matches 0 tests because T03 owns the runtime rails; `meshc --test e2e_m047_s07` still fails because that T04 target does not exist yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-typeck m047_s07 -- --nocapture` | 0 | ✅ pass | 29818ms |
| 2 | `cargo test -p mesh-lsp m047_s07 -- --nocapture` | 0 | ✅ pass | 23340ms |
| 3 | `cargo test -p mesh-codegen m047_s07 -- --nocapture` | 0 | ✅ pass | 1686ms |
| 4 | `cargo test -p mesh-rt m047_s07 -- --nocapture` | 0 | ✅ pass | 15077ms |
| 5 | `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` | 101 | ❌ fail | 776ms |
| 6 | `cargo test -p meshc --test e2e_stdlib e2e_m032_route_ -- --nocapture` | 0 | ✅ pass | 51327ms |


## Deviations

Did not modify `compiler/mesh-codegen/src/codegen/expr.rs` or `compiler/mesh-codegen/src/codegen/intrinsics.rs` because the existing plain fn-pointer ABI guard in `codegen_call(...)` already keeps `mesh_http_route_*` handler arguments unexpanded, and no new runtime intrinsics were required.

## Known Issues

`cargo test -p meshc --test e2e_m047_s07 -- --nocapture` still fails because the `e2e_m047_s07` target has not been added yet; that proof belongs to T04. `cargo test -p mesh-rt m047_s07 -- --nocapture` still runs 0 tests because the runtime continuity and HTTP dispatch rails belong to T03.

## Files Created/Modified

- `compiler/mesh-codegen/src/declared.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/mod.rs`
- `compiler/mesh-codegen/src/lib.rs`
- `compiler/meshc/src/main.rs`
- `.gsd/DECISIONS.md`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M047/slices/S07/tasks/T02-SUMMARY.md`


## Deviations
Did not modify `compiler/mesh-codegen/src/codegen/expr.rs` or `compiler/mesh-codegen/src/codegen/intrinsics.rs` because the existing plain fn-pointer ABI guard in `codegen_call(...)` already keeps `mesh_http_route_*` handler arguments unexpanded, and no new runtime intrinsics were required.

## Known Issues
`cargo test -p meshc --test e2e_m047_s07 -- --nocapture` still fails because the `e2e_m047_s07` target has not been added yet; that proof belongs to T04. `cargo test -p mesh-rt m047_s07 -- --nocapture` still runs 0 tests because the runtime continuity and HTTP dispatch rails belong to T03.
