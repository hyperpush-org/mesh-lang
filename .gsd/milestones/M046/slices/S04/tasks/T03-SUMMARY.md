---
id: T03
parent: S04
milestone: M046
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/support/mod.rs", "compiler/meshc/tests/support/m046_route_free.rs", "compiler/meshc/tests/e2e_m046_s03.rs", "compiler/meshc/tests/e2e_m046_s04.rs", "scripts/verify-m046-s03.sh", "scripts/verify-m046-s04.sh", ".gsd/KNOWLEDGE.md", ".gsd/DECISIONS.md"]
key_decisions: ["D249: share the route-free package build/spawn/CLI harness through compiler/meshc/tests/support/m046_route_free.rs with temp output metadata and preflighted --output parents.", "Use the live completed S03 T04 wording as the regression-plan contract so the tiny-cluster rail proves current state rather than stale historical override text."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the shared S03 regression rail after helper extraction and live plan-text correction, passed the new S04 helper negatives/contract/smoke/startup target, and passed the exact slice verification command including bash scripts/verify-m046-s04.sh with retained bundle checks. The final passing verification command was: cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_ -- --nocapture && cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture && bash scripts/verify-m046-s04.sh."
completed_at: 2026-03-31T22:55:18.631Z
blocker_discovered: false
---

# T03: Added shared route-free package e2e support and a packaged cluster-proof CLI/runtime proof rail with retained .tmp/m046-s04 evidence.

> Added shared route-free package e2e support and a packaged cluster-proof CLI/runtime proof rail with retained .tmp/m046-s04 evidence.

## What Happened
---
id: T03
parent: S04
milestone: M046
key_files:
  - compiler/meshc/tests/support/mod.rs
  - compiler/meshc/tests/support/m046_route_free.rs
  - compiler/meshc/tests/e2e_m046_s03.rs
  - compiler/meshc/tests/e2e_m046_s04.rs
  - scripts/verify-m046-s03.sh
  - scripts/verify-m046-s04.sh
  - .gsd/KNOWLEDGE.md
  - .gsd/DECISIONS.md
key_decisions:
  - D249: share the route-free package build/spawn/CLI harness through compiler/meshc/tests/support/m046_route_free.rs with temp output metadata and preflighted --output parents.
  - Use the live completed S03 T04 wording as the regression-plan contract so the tiny-cluster rail proves current state rather than stale historical override text.
duration: ""
verification_result: passed
completed_at: 2026-03-31T22:55:18.633Z
blocker_discovered: false
---

# T03: Added shared route-free package e2e support and a packaged cluster-proof CLI/runtime proof rail with retained .tmp/m046-s04 evidence.

**Added shared route-free package e2e support and a packaged cluster-proof CLI/runtime proof rail with retained .tmp/m046-s04 evidence.**

## What Happened

Extracted the reusable M046 route-free build/spawn/CLI JSON harness into compiler/meshc/tests/support/m046_route_free.rs and exposed it via compiler/meshc/tests/support/mod.rs. Rewired compiler/meshc/tests/e2e_m046_s03.rs to import that shared helper layer and corrected its stale plan/verifier text assertions to the live completed T04 wording so the tiny-cluster regression rail proved current runtime behavior again. Added compiler/meshc/tests/e2e_m046_s04.rs with helper negative tests, a route-free package contract rail, a temp-output build plus package smoke rail, and a two-node packaged startup proof for cluster-proof that retains package sources, build metadata, tracked-binary snapshots, CLI JSON/human output, and node stdout/stderr under .tmp/m046-s04/.... Added scripts/verify-m046-s04.sh as the direct slice verifier that replays the shared S03 regression, cluster-proof smoke commands, the focused S04 e2e target, and bundle-copy/shape checks under .tmp/m046-s04/verify. Recorded the shared-harness pattern as D249 and added the temp-build/README drift gotchas to .gsd/KNOWLEDGE.md.

## Verification

Passed the shared S03 regression rail after helper extraction and live plan-text correction, passed the new S04 helper negatives/contract/smoke/startup target, and passed the exact slice verification command including bash scripts/verify-m046-s04.sh with retained bundle checks. The final passing verification command was: cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_ -- --nocapture && cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture && bash scripts/verify-m046-s04.sh.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_ -- --nocapture` | 0 | ✅ pass | 20500ms |
| 2 | `cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture` | 0 | ✅ pass | 23300ms |
| 3 | `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_ -- --nocapture && cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture && bash scripts/verify-m046-s04.sh` | 0 | ✅ pass | 195200ms |


## Deviations

Adjusted the carried S03 plan/verifier text assertions to the live completed T04 wording because the repo had already moved past the earlier override-task phrasing, and narrowed two brittle S04 verifier guards so they matched the real README/source contract instead of false-positive substring or ordering assumptions.

## Known Issues

compiler/meshc/tests/support/m046_route_free.rs still emits a dead-code warning in the S04 test binary for wait_for_diagnostics_matching because that generic waiter is currently consumed by the shared S03 rail only. No runtime or verifier behavior is blocked by the warning.

## Files Created/Modified

- `compiler/meshc/tests/support/mod.rs`
- `compiler/meshc/tests/support/m046_route_free.rs`
- `compiler/meshc/tests/e2e_m046_s03.rs`
- `compiler/meshc/tests/e2e_m046_s04.rs`
- `scripts/verify-m046-s03.sh`
- `scripts/verify-m046-s04.sh`
- `.gsd/KNOWLEDGE.md`
- `.gsd/DECISIONS.md`


## Deviations
Adjusted the carried S03 plan/verifier text assertions to the live completed T04 wording because the repo had already moved past the earlier override-task phrasing, and narrowed two brittle S04 verifier guards so they matched the real README/source contract instead of false-positive substring or ordering assumptions.

## Known Issues
compiler/meshc/tests/support/m046_route_free.rs still emits a dead-code warning in the S04 test binary for wait_for_diagnostics_matching because that generic waiter is currently consumed by the shared S03 rail only. No runtime or verifier behavior is blocked by the warning.
