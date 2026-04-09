---
id: T04
parent: S07
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/e2e_m047_s07.rs", "compiler/meshc/tests/support/m046_route_free.rs", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M047/slices/S07/tasks/T04-SUMMARY.md"]
key_decisions: ["Track clustered HTTP route continuity by before/after request-key diff instead of list order so repeated runtime names remain inspectable.", "Keep repeated-success requests on one ingress node in the proof rail because current clustered HTTP request keys are node-local and cross-ingress repeats collide as duplicate submissions."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` and `cargo test -p meshc --test e2e_stdlib e2e_m032_route_ -- --nocapture` on the final file state."
completed_at: 2026-04-02T00:43:46.042Z
blocker_discovered: false
---

# T04: Added a two-node clustered HTTP route e2e that proves live success/rejection continuity and preserves the M032 route guardrails.

> Added a two-node clustered HTTP route e2e that proves live success/rejection continuity and preserves the M032 route guardrails.

## What Happened
---
id: T04
parent: S07
milestone: M047
key_files:
  - compiler/meshc/tests/e2e_m047_s07.rs
  - compiler/meshc/tests/support/m046_route_free.rs
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M047/slices/S07/tasks/T04-SUMMARY.md
key_decisions:
  - Track clustered HTTP route continuity by before/after request-key diff instead of list order so repeated runtime names remain inspectable.
  - Keep repeated-success requests on one ingress node in the proof rail because current clustered HTTP request keys are node-local and cross-ingress repeats collide as duplicate submissions.
duration: ""
verification_result: passed
completed_at: 2026-04-02T00:43:46.044Z
blocker_discovered: false
---

# T04: Added a two-node clustered HTTP route e2e that proves live success/rejection continuity and preserves the M032 route guardrails.

**Added a two-node clustered HTTP route e2e that proves live success/rejection continuity and preserves the M032 route guardrails.**

## What Happened

Added a dedicated two-node clustered HTTP route proof rail in `compiler/meshc/tests/e2e_m047_s07.rs` that builds a temporary multi-module package, boots a live two-node cluster, waits for both HTTP servers, sends real success and unsupported-count requests, and validates continuity by before/after request-key diff instead of list order. Extended `compiler/meshc/tests/support/m046_route_free.rs` with request-key-aware continuity lookup/diff helpers so repeated runtime names can be inspected truthfully, and recorded the node-local request-key collision gotcha in `.gsd/KNOWLEDGE.md`.

## Verification

Passed `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` and `cargo test -p meshc --test e2e_stdlib e2e_m032_route_ -- --nocapture` on the final file state.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` | 0 | ✅ pass | 16500ms |
| 2 | `cargo test -p meshc --test e2e_stdlib e2e_m032_route_ -- --nocapture` | 0 | ✅ pass | 10000ms |


## Deviations

The success route kept the imported bare-handler proof, but the explicit-count rejection route uses the module-qualified `Todos.handle_retry_todos` form because this tree’s current typecheck surface rejected the imported-bare explicit-count fixture shape even though the same runtime-name/count/rejection seam lowered and executed correctly through the module-qualified reference. I also kept the repeated-success requests on one ingress node and exercised the second HTTP server through `/health` plus the unsupported-count route, because current clustered HTTP request keys are node-local and cross-ingress repeats for the same runtime name collide as duplicate submissions.

## Known Issues

Cross-ingress repeated clustered HTTP requests for the same runtime name can still collide on `request_key` and fail with `declared_handler_submit_rejected:duplicate`; this task documented that behavior in `.gsd/KNOWLEDGE.md` but did not change runtime identity generation.

## Files Created/Modified

- `compiler/meshc/tests/e2e_m047_s07.rs`
- `compiler/meshc/tests/support/m046_route_free.rs`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M047/slices/S07/tasks/T04-SUMMARY.md`


## Deviations
The success route kept the imported bare-handler proof, but the explicit-count rejection route uses the module-qualified `Todos.handle_retry_todos` form because this tree’s current typecheck surface rejected the imported-bare explicit-count fixture shape even though the same runtime-name/count/rejection seam lowered and executed correctly through the module-qualified reference. I also kept the repeated-success requests on one ingress node and exercised the second HTTP server through `/health` plus the unsupported-count route, because current clustered HTTP request keys are node-local and cross-ingress repeats for the same runtime name collide as duplicate submissions.

## Known Issues
Cross-ingress repeated clustered HTTP requests for the same runtime name can still collide on `request_key` and fail with `declared_handler_submit_rejected:duplicate`; this task documented that behavior in `.gsd/KNOWLEDGE.md` but did not change runtime identity generation.
