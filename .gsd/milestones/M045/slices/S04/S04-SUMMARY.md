---
id: S04
parent: M045
milestone: M045
provides:
  - A cleaned `cluster-proof` clustered boundary with no dead deterministic placement engine and no wrapper-owned manual continuity completion path.
  - The current public clustered closeout rail (`cargo test -p meshc --test e2e_m045_s04 ...` plus `bash scripts/verify-m045-s04.sh`) with docs/readmes pointed at M045 instead of the older M044 story.
  - Package and e2e coverage that fail closed when legacy placement fields, wrapper-owned declared work, or stale M044 closeout wording drift back in.
requires:
  - slice: S01
    provides: `Node.start_from_env()` plus typed `BootstrapStatus`, which the cleaned clustered example and `cluster-proof` both consume as the runtime-owned bootstrap seam.
  - slice: S02
    provides: The tiny clustered happy-path contract and the runtime-owned declared-work completion rail that S04 replays through `scripts/verify-m045-s02.sh`.
  - slice: S03
    provides: The scaffold-first failover verifier and retained runtime-owned artifact bundle that S04 reuses as the deeper proof subrail.
affects:
  - S05
key_files:
  - cluster-proof/cluster.mpl
  - cluster-proof/work.mpl
  - cluster-proof/work_continuity.mpl
  - cluster-proof/tests/work.test.mpl
  - cluster-proof/tests/config.test.mpl
  - cluster-proof/mesh.toml
  - compiler/meshc/tests/e2e_m045_s04.rs
  - scripts/verify-m045-s02.sh
  - scripts/verify-m045-s04.sh
  - README.md
  - cluster-proof/README.md
  - website/docs/docs/distributed/index.md
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/tooling/index.md
  - .gsd/PROJECT.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Keep only deterministic membership sorting in `cluster-proof/cluster.mpl`; delete the dead owner/replica placement chain and prove its absence through package tests instead of preserving helper-shaped behavior.
  - `cluster-proof/work.mpl` owns `declared_work_target()` and `execute_declared_work(...)`, while `work_continuity.mpl` stays a thin keyed submit/status translator over runtime `Continuity` and does not manually close records.
  - Use M045 S04 (`compiler/meshc/tests/e2e_m045_s04.rs` plus `scripts/verify-m045-s04.sh`) as the current public clustered closeout contract, with M045 S03 as the failover-specific subrail and M044 S05 as historical transition coverage only.
patterns_established:
  - Retire example-owned distributed logic by deleting dead helper chains and then proving the new absence directly in package/source-contract tests, not just by leaving newer rails green.
  - Keep the proof app split at stable seams: declared work in `Work`, runtime completion/failover in `Continuity`, and HTTP glue in `work_continuity` only.
  - Assembled closeout verifiers should replay prerequisite rails, retain upstream artifact bundles, and explicitly prebuild runtime dependencies when nested package-test paths still link against workspace runtime archives.
observability_surfaces:
  - `.tmp/m045-s04/verify/phase-report.txt`, `status.txt`, `current-phase.txt`, and `latest-failover-bundle.txt` from `bash scripts/verify-m045-s04.sh`.
  - `.tmp/m045-s02/verify/` from `bash scripts/verify-m045-s02.sh` for nested remote-owner/runtime-completion regressions and missing-runtime-archive failures.
  - The runtime-owned `meshc cluster status|continuity|diagnostics --json` surfaces exercised by the retained S03 failover bundle that `scripts/verify-m045-s04.sh` validates and points to.
drill_down_paths:
  - .gsd/milestones/M045/slices/S04/tasks/T01-SUMMARY.md
  - .gsd/milestones/M045/slices/S04/tasks/T02-SUMMARY.md
  - .gsd/milestones/M045/slices/S04/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-31T01:14:31.342Z
blocker_discovered: false
---

# S04: Remove Legacy Example-Side Cluster Logic

**Removed the last cluster-proof-era placement and wrapper completion glue, and made M045 S04 the live clustered closeout rail for the repo’s public distributed story.**

## What Happened

S04 finished the cleanup pass on the remaining legacy `cluster-proof`-shaped clustered story. T01 deleted the dead deterministic owner/replica placement engine from `cluster-proof/cluster.mpl` and rewrote the package tests so they now prove only the live membership/config seams that still matter: topology and durability validation, runtime authority/discovery payload truth, and the explicit absence of legacy placement fields. T02 moved the manifest-declared handler to `Work.execute_declared_work`, made `cluster-proof/work.mpl` own both `declared_work_target()` and `execute_declared_work(...)`, and stripped `cluster-proof/work_continuity.mpl` down to keyed submit/status HTTP translation over runtime `Continuity` with no manual `Continuity.mark_completed(...)` fallback. T03 added `compiler/meshc/tests/e2e_m045_s04.rs` and `scripts/verify-m045-s04.sh` as the current clustered closeout contract, rewired README/docs pages to point at M045 rails instead of the older M044 story, and narrowed the old M044 closeout coverage to historical transition checks. During slice closeout, the first assembled verifier replay exposed one remaining verifier-side legacy seam: `scripts/verify-m045-s02.sh` still assumed `target/debug/libmesh_rt.a` already existed. S04 fixed that by adding an explicit `cargo build -q -p mesh-rt` preflight so the assembled M045 closeout rail now fails on real contract drift instead of a missing runtime archive.

## Verification

Verified with `cargo run -q -p meshc -- build cluster-proof`, `cargo run -q -p meshc -- test cluster-proof/tests`, `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture`, `cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture`, `bash scripts/verify-m045-s02.sh`, and `bash scripts/verify-m045-s04.sh` (all green). The package tests prove live topology/durability validation, Work-owned declared work, and membership payloads without legacy placement fields; the M044 failover rail still proves runtime-owned automatic recovery after the target move; and the M045 assembled rail replays S02/S03, validates the retained failover bundle shape, rebuilds/tests `cluster-proof`, and rebuilds the docs. One post-fix replay of `bash scripts/verify-m045-s04.sh` hit a transient nested S02 remote-owner `write_error` failure, but the direct `e2e_m045_s02` rail, `bash scripts/verify-m045-s02.sh`, and the final unchanged `bash scripts/verify-m045-s04.sh` replay all passed cleanly.

## Requirements Advanced

- R077 — Removed the remaining legacy `cluster-proof` placement/completion glue and tightened the current closeout rail so the clustered story stays visibly smaller and runtime-owned.
- R079 — Deleted dead placement logic, moved declared work into `Work`, and removed manual completion glue from `work_continuity`, shrinking the last example-owned distributed seams.
- R081 — Moved the current public readme/docs and clustered closeout verifier contract onto M045 rails instead of the older M044 story.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The written plan did not call out verifier plumbing, but the first closeout replay exposed that `scripts/verify-m045-s02.sh` was still relying on an ambient `mesh-rt` archive. S04 added the explicit runtime prebuild instead of weakening the nested package-test check or leaving the linker failure for a later slice to rediscover.

## Known Limitations

`scripts/verify-m045-s04.sh` still depends on replaying the older S02/S03 rails, so an upstream remote-owner/runtime flake can make the closeout rail red before the S04-specific docs and source-contract assertions run. S05 still needs to finish the docs-first teaching surface so the tiny scaffold example is unequivocally the primary clustered story everywhere, not just in the current readmes and closeout verifier.

## Follow-ups

1. In S05, finish the docs-first clustered story so `meshc init --clustered` is the first path readers see and `cluster-proof` stays explicitly secondary proof material.
2. If the assembled closeout rail flakes again, start with `.tmp/m045-s02/verify/` and `.tmp/m045-s04/verify/` before changing scaffold or runtime code; the remaining instability shows up there before the S04-specific contract checks fail.

## Files Created/Modified

- `cluster-proof/cluster.mpl` — Deleted the dead deterministic placement helpers while preserving canonical membership sorting and membership payload truth.
- `cluster-proof/work.mpl` — Made `Work` own `declared_work_target()` and `execute_declared_work(...)` for the manifest-declared clustered path.
- `cluster-proof/work_continuity.mpl` — Removed wrapper-era target/manual-completion glue so the file only translates keyed submit/status HTTP behavior over runtime `Continuity`.
- `cluster-proof/tests/work.test.mpl` — Added assertions for Work-owned declared work, runtime-backed authority/status payload truth, and the absence of legacy placement fields.
- `cluster-proof/tests/config.test.mpl` — Focused config tests on live durability/topology validation instead of dead helper-shaped placement behavior.
- `cluster-proof/mesh.toml` — Retargeted the declared handler to `Work.execute_declared_work`.
- `compiler/meshc/tests/e2e_m045_s04.rs` — Added the S04 source/docs/verifier contract target that proves the cleaned target shape and current public closeout wording.
- `scripts/verify-m045-s02.sh` — Added an explicit `mesh-rt` prebuild so nested package tests no longer fail on a missing `target/debug/libmesh_rt.a` archive.
- `scripts/verify-m045-s04.sh` — Added the assembled M045 closeout verifier that replays S02/S03, validates the retained failover bundle, rebuilds/tests `cluster-proof`, and rebuilds the docs.
- `README.md` — Updated the primary clustered story to point at the M045 closeout rail instead of the older M044 story.
- `cluster-proof/README.md` — Reframed `cluster-proof` as the deeper proof consumer behind the current M045 clustered story.
- `website/docs/docs/distributed/index.md` — Updated distributed docs to point at M045 rails as the current clustered closeout path.
- `website/docs/docs/distributed-proof/index.md` — Updated deeper proof docs to reference the new M045 closeout relationship and keep proof rails secondary.
- `website/docs/docs/tooling/index.md` — Updated tooling docs so the current clustered verification story points at `scripts/verify-m045-s04.sh`.
- `.gsd/PROJECT.md` — Refreshed the living project state to mark S04 complete and leave S05 as the remaining M045 gap.
- `.gsd/KNOWLEDGE.md` — Recorded the verifier-side `mesh-rt` prebuild requirement for nested `cluster-proof` package-test rails.
