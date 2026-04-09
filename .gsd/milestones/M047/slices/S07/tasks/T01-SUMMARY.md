---
id: T01
parent: S07
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-typeck/src/infer.rs", "compiler/mesh-typeck/src/unify.rs", "compiler/mesh-typeck/src/lib.rs", "compiler/mesh-typeck/src/error.rs", "compiler/mesh-typeck/src/diagnostics.rs", "compiler/mesh-lsp/src/analysis.rs", "compiler/mesh-typeck/tests/http_clustered_routes.rs", ".gsd/milestones/M047/slices/S07/tasks/T01-SUMMARY.md"]
key_decisions: ["D299: Keep `HTTP.clustered(handler)` typed as the underlying handler function and carry clustered-route truth in explicit wrapper metadata plus a final route-slot misuse sweep.", "Record imported bare-handler origins and qualified-module origins during typecheck so later lowering can keep real runtime names like `Api.Todos.handle_list_todos` instead of guessing local-module names."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the task-local verification bar with `cargo test -p mesh-typeck m047_s07 -- --nocapture` and `cargo test -p mesh-lsp m047_s07 -- --nocapture`. Also ran the slice-level commands to record honest intermediate-task status: `mesh-codegen` and `mesh-rt` `m047_s07` filters currently match 0 tests, `meshc --test e2e_stdlib e2e_m032_route_ -- --nocapture` passed, and `meshc --test e2e_m047_s07 -- --nocapture` failed because that target has not been added yet."
completed_at: 2026-04-01T23:37:26.110Z
blocker_discovered: false
---

# T01: Added compiler-known HTTP.clustered wrapper typing, metadata, and source-local typecheck/LSP diagnostics for clustered HTTP routes.

> Added compiler-known HTTP.clustered wrapper typing, metadata, and source-local typecheck/LSP diagnostics for clustered HTTP routes.

## What Happened
---
id: T01
parent: S07
milestone: M047
key_files:
  - compiler/mesh-typeck/src/infer.rs
  - compiler/mesh-typeck/src/unify.rs
  - compiler/mesh-typeck/src/lib.rs
  - compiler/mesh-typeck/src/error.rs
  - compiler/mesh-typeck/src/diagnostics.rs
  - compiler/mesh-lsp/src/analysis.rs
  - compiler/mesh-typeck/tests/http_clustered_routes.rs
  - .gsd/milestones/M047/slices/S07/tasks/T01-SUMMARY.md
key_decisions:
  - D299: Keep `HTTP.clustered(handler)` typed as the underlying handler function and carry clustered-route truth in explicit wrapper metadata plus a final route-slot misuse sweep.
  - Record imported bare-handler origins and qualified-module origins during typecheck so later lowering can keep real runtime names like `Api.Todos.handle_list_todos` instead of guessing local-module names.
duration: ""
verification_result: mixed
completed_at: 2026-04-01T23:37:26.111Z
blocker_discovered: false
---

# T01: Added compiler-known HTTP.clustered wrapper typing, metadata, and source-local typecheck/LSP diagnostics for clustered HTTP routes.

**Added compiler-known HTTP.clustered wrapper typing, metadata, and source-local typecheck/LSP diagnostics for clustered HTTP routes.**

## What Happened

Implemented compiler-known `HTTP.clustered(...)` handling in `mesh-typeck` by intercepting wrapper calls, validating only public top-level bare refs or imported-module-qualified refs, recording clustered-route metadata keyed by wrapper call span, preserving imported defining-module origins for later lowering, and enforcing a final route-slot misuse sweep. Threaded new wrapper-specific errors through human diagnostics and LSP span projection, then added focused `m047_s07` tests covering direct and pipe route forms, default versus explicit counts, imported bare origins, non-route-position misuse, private handlers, closure misuse, conflicting counts, and LSP range anchoring.

## Verification

Passed the task-local verification bar with `cargo test -p mesh-typeck m047_s07 -- --nocapture` and `cargo test -p mesh-lsp m047_s07 -- --nocapture`. Also ran the slice-level commands to record honest intermediate-task status: `mesh-codegen` and `mesh-rt` `m047_s07` filters currently match 0 tests, `meshc --test e2e_stdlib e2e_m032_route_ -- --nocapture` passed, and `meshc --test e2e_m047_s07 -- --nocapture` failed because that target has not been added yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-typeck m047_s07 -- --nocapture` | 0 | ✅ pass | 11800ms |
| 2 | `cargo test -p mesh-lsp m047_s07 -- --nocapture` | 0 | ✅ pass | 4000ms |
| 3 | `cargo test -p mesh-codegen m047_s07 -- --nocapture` | 0 | ✅ pass | 44600ms |
| 4 | `cargo test -p mesh-rt m047_s07 -- --nocapture` | 0 | ✅ pass | 88200ms |
| 5 | `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` | 101 | ❌ fail | 44400ms |
| 6 | `cargo test -p meshc --test e2e_stdlib e2e_m032_route_ -- --nocapture` | 0 | ✅ pass | 136400ms |


## Deviations

Did not modify compiler/mesh-typeck/src/builtins.rs because, in this tree, the module-qualified HTTP stdlib surface used by `HTTP.clustered(...)` lives in `compiler/mesh-typeck/src/infer.rs::stdlib_modules()` plus call-site interception. The honest local implementation seam was therefore `infer.rs`, not a separate builtin registry edit.

## Known Issues

`cargo test -p meshc --test e2e_m047_s07 -- --nocapture` still fails because the `e2e_m047_s07` target does not exist yet; that proof belongs to T04. The `m047_s07` filter also matches 0 tests in `mesh-codegen` and `mesh-rt` until downstream tasks add those focused rails.

## Files Created/Modified

- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-typeck/src/unify.rs`
- `compiler/mesh-typeck/src/lib.rs`
- `compiler/mesh-typeck/src/error.rs`
- `compiler/mesh-typeck/src/diagnostics.rs`
- `compiler/mesh-lsp/src/analysis.rs`
- `compiler/mesh-typeck/tests/http_clustered_routes.rs`
- `.gsd/milestones/M047/slices/S07/tasks/T01-SUMMARY.md`


## Deviations
Did not modify compiler/mesh-typeck/src/builtins.rs because, in this tree, the module-qualified HTTP stdlib surface used by `HTTP.clustered(...)` lives in `compiler/mesh-typeck/src/infer.rs::stdlib_modules()` plus call-site interception. The honest local implementation seam was therefore `infer.rs`, not a separate builtin registry edit.

## Known Issues
`cargo test -p meshc --test e2e_m047_s07 -- --nocapture` still fails because the `e2e_m047_s07` target does not exist yet; that proof belongs to T04. The `m047_s07` filter also matches 0 tests in `mesh-codegen` and `mesh-rt` until downstream tasks add those focused rails.
