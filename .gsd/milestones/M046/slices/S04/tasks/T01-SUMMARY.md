---
id: T01
parent: S04
milestone: M046
provides: []
requires: []
affects: []
key_files: ["cluster-proof/mesh.toml", "cluster-proof/main.mpl", "cluster-proof/work.mpl", "cluster-proof/cluster.mpl", "cluster-proof/config.mpl", "cluster-proof/work_continuity.mpl", "cluster-proof/tests/work.test.mpl", "cluster-proof/tests/config.test.mpl", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M046/slices/S04/tasks/T01-SUMMARY.md"]
key_decisions: ["Removed the obsolete config smoke test during T01 because deleting `config.mpl` would otherwise leave `meshc test cluster-proof/tests` broken immediately.", "Used on-disk source-contract assertions in `cluster-proof/tests/work.test.mpl` instead of importing deleted helper modules so the package rail now fails closed on route/continuity drift."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the task contract by building `cluster-proof` and confirming the three deleted modules stayed absent after build. Passed the package smoke rail with the new route-free source assertions after fixing the boolean-helper codegen issue. Verified the observability change directly by launching the built `cluster-proof/cluster-proof` binary under an explicit cluster env and capturing the expected `[cluster-proof] runtime bootstrap ...` log. Ran the slice-level checks as an intermediate-task gate: `e2e_m046_s04` and `scripts/verify-m046-s04.sh` still fail because the T03 packaged proof rail is not present yet, and the retained `e2e_m046_s03` rail currently fails on a pre-existing S03 plan-text assertion unrelated to `cluster-proof/`."
completed_at: 2026-03-31T22:15:00.608Z
blocker_discovered: false
---

# T01: Reset `cluster-proof/` to a route-free source-owned clustered contract and aligned the package smoke rail with the deleted legacy modules.

> Reset `cluster-proof/` to a route-free source-owned clustered contract and aligned the package smoke rail with the deleted legacy modules.

## What Happened
---
id: T01
parent: S04
milestone: M046
key_files:
  - cluster-proof/mesh.toml
  - cluster-proof/main.mpl
  - cluster-proof/work.mpl
  - cluster-proof/cluster.mpl
  - cluster-proof/config.mpl
  - cluster-proof/work_continuity.mpl
  - cluster-proof/tests/work.test.mpl
  - cluster-proof/tests/config.test.mpl
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M046/slices/S04/tasks/T01-SUMMARY.md
key_decisions:
  - Removed the obsolete config smoke test during T01 because deleting `config.mpl` would otherwise leave `meshc test cluster-proof/tests` broken immediately.
  - Used on-disk source-contract assertions in `cluster-proof/tests/work.test.mpl` instead of importing deleted helper modules so the package rail now fails closed on route/continuity drift.
duration: ""
verification_result: mixed
completed_at: 2026-03-31T22:15:00.614Z
blocker_discovered: false
---

# T01: Reset `cluster-proof/` to a route-free source-owned clustered contract and aligned the package smoke rail with the deleted legacy modules.

**Reset `cluster-proof/` to a route-free source-owned clustered contract and aligned the package smoke rail with the deleted legacy modules.**

## What Happened

Rewrote `cluster-proof/mesh.toml` to a package-only manifest with no `[cluster]` section or manifest-owned declarations, replaced `cluster-proof/main.mpl` with a single `Node.start_from_env()` bootstrap path plus route-free success/failure logs, and replaced `cluster-proof/work.mpl` with a tiny source-owned `clustered(work)` handler that keeps the runtime name `Work.execute_declared_work` and returns `1 + 1`. Deleted `cluster-proof/cluster.mpl`, `cluster-proof/config.mpl`, and `cluster-proof/work_continuity.mpl` instead of leaving wrappers behind. Rewrote `cluster-proof/tests/work.test.mpl` into a route-free source-contract smoke test and removed `cluster-proof/tests/config.test.mpl` because it only exercised the deleted config module. While updating the smoke test, the first `meshc test` run exposed a compiler-side LLVM verifier failure caused by a chained `&&` helper; I rewrote that helper with nested `if` branches and recorded the gotcha in `.gsd/KNOWLEDGE.md`.

## Verification

Passed the task contract by building `cluster-proof` and confirming the three deleted modules stayed absent after build. Passed the package smoke rail with the new route-free source assertions after fixing the boolean-helper codegen issue. Verified the observability change directly by launching the built `cluster-proof/cluster-proof` binary under an explicit cluster env and capturing the expected `[cluster-proof] runtime bootstrap ...` log. Ran the slice-level checks as an intermediate-task gate: `e2e_m046_s04` and `scripts/verify-m046-s04.sh` still fail because the T03 packaged proof rail is not present yet, and the retained `e2e_m046_s03` rail currently fails on a pre-existing S03 plan-text assertion unrelated to `cluster-proof/`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- build cluster-proof && test ! -e cluster-proof/cluster.mpl && test ! -e cluster-proof/config.mpl && test ! -e cluster-proof/work_continuity.mpl` | 0 | ✅ pass | 8988ms |
| 2 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 8538ms |
| 3 | `cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture` | 101 | ❌ fail | 652ms |
| 4 | `bash scripts/verify-m046-s04.sh` | 127 | ❌ fail | 30ms |
| 5 | `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_ -- --nocapture` | 101 | ❌ fail | 2118ms |
| 6 | `./cluster-proof/cluster-proof with cluster env (terminated after log capture)` | -15 | ✅ pass | 1512ms |


## Deviations

Moved part of the planned T02 test cleanup into T01 by rewriting `cluster-proof/tests/work.test.mpl` and deleting `cluster-proof/tests/config.test.mpl`, because leaving tests pointed at deleted modules would have made `meshc test cluster-proof/tests` immediately false after the source reset.

## Known Issues

`cluster-proof/README.md`, `cluster-proof/Dockerfile`, `cluster-proof/docker-entrypoint.sh`, and `cluster-proof/fly.toml` still describe the legacy routeful package shape and remain for T02. `compiler/meshc/tests/e2e_m046_s04.rs` and `scripts/verify-m046-s04.sh` do not exist yet, so those slice-level checks remain red until T03. `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_ -- --nocapture` currently fails on a pre-existing `.gsd/milestones/M046/slices/S03/S03-PLAN.md` assertion mismatch rather than on `cluster-proof/` behavior.

## Files Created/Modified

- `cluster-proof/mesh.toml`
- `cluster-proof/main.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/cluster.mpl`
- `cluster-proof/config.mpl`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/tests/work.test.mpl`
- `cluster-proof/tests/config.test.mpl`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M046/slices/S04/tasks/T01-SUMMARY.md`


## Deviations
Moved part of the planned T02 test cleanup into T01 by rewriting `cluster-proof/tests/work.test.mpl` and deleting `cluster-proof/tests/config.test.mpl`, because leaving tests pointed at deleted modules would have made `meshc test cluster-proof/tests` immediately false after the source reset.

## Known Issues
`cluster-proof/README.md`, `cluster-proof/Dockerfile`, `cluster-proof/docker-entrypoint.sh`, and `cluster-proof/fly.toml` still describe the legacy routeful package shape and remain for T02. `compiler/meshc/tests/e2e_m046_s04.rs` and `scripts/verify-m046-s04.sh` do not exist yet, so those slice-level checks remain red until T03. `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_ -- --nocapture` currently fails on a pre-existing `.gsd/milestones/M046/slices/S03/S03-PLAN.md` assertion mismatch rather than on `cluster-proof/` behavior.
