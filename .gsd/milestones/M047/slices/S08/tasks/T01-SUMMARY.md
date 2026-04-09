---
id: T01
parent: S08
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-pkg/src/scaffold.rs", "compiler/meshc/tests/tooling_e2e.rs", "compiler/meshc/tests/e2e_m047_s05.rs"]
key_decisions: ["Keep work.mpl route-free and limit wrapper adoption to selected GET /todos routes using explicit HTTP.clustered(1, ...) while GET /health and mutating routes stay local.", "Make the fast scaffold rails assert both presence of explicit-count clustered read routes and absence of wrappers on local health/write routes so contract drift fails closed early."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-level verification passed with cargo test -p mesh-pkg m047_s05 -- --nocapture and cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture. Broader slice verification also passed with cargo test -p meshc --test e2e_m047_s05 -- --nocapture, cargo test -p meshc --test e2e_m047_s07 -- --nocapture, cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture, bash scripts/verify-m047-s05.sh, bash scripts/verify-m047-s06.sh, and npm --prefix website run build. The longer S05/S07 rails exercised expected fail-closed negative-path panics inside caught test assertions and still passed overall."
completed_at: 2026-04-02T01:56:48.846Z
blocker_discovered: false
---

# T01: Rebased the Todo scaffold onto explicit HTTP.clustered(1, ...) read routes with typed handlers and updated the fast contract rails.

> Rebased the Todo scaffold onto explicit HTTP.clustered(1, ...) read routes with typed handlers and updated the fast contract rails.

## What Happened
---
id: T01
parent: S08
milestone: M047
key_files:
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/tests/tooling_e2e.rs
  - compiler/meshc/tests/e2e_m047_s05.rs
key_decisions:
  - Keep work.mpl route-free and limit wrapper adoption to selected GET /todos routes using explicit HTTP.clustered(1, ...) while GET /health and mutating routes stay local.
  - Make the fast scaffold rails assert both presence of explicit-count clustered read routes and absence of wrappers on local health/write routes so contract drift fails closed early.
duration: ""
verification_result: passed
completed_at: 2026-04-02T01:56:48.849Z
blocker_discovered: false
---

# T01: Rebased the Todo scaffold onto explicit HTTP.clustered(1, ...) read routes with typed handlers and updated the fast contract rails.

**Rebased the Todo scaffold onto explicit HTTP.clustered(1, ...) read routes with typed handlers and updated the fast contract rails.**

## What Happened

Updated compiler/mesh-pkg/src/scaffold.rs so the generated todo starter now wraps GET /todos and GET /todos/:id with HTTP.clustered(1, ...) while keeping GET /health and all mutating routes local. Added explicit Request -> Response signatures to the wrapped handle_list_todos and handle_get_todo handlers, reworded the generated README to describe the real explicit-count-1 starter contract, and kept work.mpl on the canonical route-free @cluster pub fn sync_todos() surface. Rebaselined the scaffold unit assertions, the fast CLI scaffold expectations, and the S05 generated-source contract checks so they fail closed on stale non-goal wording, missing explicit wrapper counts, missing typed wrapped handlers, or accidental wrapper use on health/write routes.

## Verification

Task-level verification passed with cargo test -p mesh-pkg m047_s05 -- --nocapture and cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture. Broader slice verification also passed with cargo test -p meshc --test e2e_m047_s05 -- --nocapture, cargo test -p meshc --test e2e_m047_s07 -- --nocapture, cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture, bash scripts/verify-m047-s05.sh, bash scripts/verify-m047-s06.sh, and npm --prefix website run build. The longer S05/S07 rails exercised expected fail-closed negative-path panics inside caught test assertions and still passed overall.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-pkg m047_s05 -- --nocapture` | 0 | ✅ pass | 39477ms |
| 2 | `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture` | 0 | ✅ pass | 26716ms |
| 3 | `cargo test -p meshc --test e2e_m047_s05 -- --nocapture` | 0 | ✅ pass | 508817ms |
| 4 | `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` | 0 | ✅ pass | 20786ms |
| 5 | `cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture` | 0 | ✅ pass | 7105ms |
| 6 | `bash scripts/verify-m047-s05.sh` | 0 | ✅ pass | 237946ms |
| 7 | `bash scripts/verify-m047-s06.sh` | 0 | ✅ pass | 319488ms |
| 8 | `npm --prefix website run build` | 0 | ✅ pass | 55215ms |


## Deviations

None.

## Known Issues

npm --prefix website run build still emits the pre-existing VitePress chunk-size warning about bundles larger than 500 kB after minification; the build passes and this task did not change that behavior.

## Files Created/Modified

- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`


## Deviations
None.

## Known Issues
npm --prefix website run build still emits the pre-existing VitePress chunk-size warning about bundles larger than 500 kB after minification; the build passes and this task did not change that behavior.
