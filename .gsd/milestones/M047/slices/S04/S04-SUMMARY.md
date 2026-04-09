---
id: S04
parent: M047
milestone: M047
provides:
  - A single supported public clustered declaration model: `@cluster` / `@cluster(N)` on ordinary route-free functions, with legacy source/manifest inputs cut off.
  - One aligned dogfood surface across `meshc init --clustered`, `tiny-cluster/`, and `cluster-proof/` built around `Work.execute_declared_work` and runtime-owned CLI inspection.
  - One authoritative local cutover rail (`bash scripts/verify-m047-s04.sh`) with retained bundle markers that downstream slices can reuse instead of reconstructing the migration proof.
requires:
  - slice: S01
    provides: source-first `@cluster` parsing, provenance, and source-ranged compiler/LSP diagnostics that S04 could hard-cut without inventing a second validation path
  - slice: S02
    provides: runtime registration and continuity truth for ordinary clustered functions, including generic runtime names and replication-count semantics that the migrated scaffold/examples could keep using unchanged
  - slice: S03
    provides: the explicit blocker evidence that `HTTP.clustered(...)` remains unshipped, which let S04 keep the public contract honest and route-free instead of over-claiming clustered HTTP support
affects:
  - S05
  - S06
key_files:
  - compiler/mesh-parser/src/parser/items.rs
  - compiler/mesh-pkg/src/manifest.rs
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/tests/tooling_e2e.rs
  - tiny-cluster/work.mpl
  - tiny-cluster/tests/work.test.mpl
  - cluster-proof/work.mpl
  - cluster-proof/tests/work.test.mpl
  - compiler/meshc/tests/support/m046_route_free.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/meshc/tests/e2e_m047_s04.rs
  - scripts/verify-m047-s04.sh
  - README.md
  - website/docs/docs/getting-started/clustered-example/index.md
  - website/docs/docs/distributed-proof/index.md
key_decisions:
  - Only `@cluster` / `@cluster(N)` remain supported clustered declaration inputs; legacy `clustered(work)` and `[cluster]` are now migration-only errors.
  - Runtime-owned handler identity stays on the function name `execute_declared_work`; the scaffold/examples did not keep `declared_work_runtime_name()` as a second compatibility seam.
  - `meshc init --clustered`, `tiny-cluster/`, and `cluster-proof/` were intentionally kept byte-level aligned on `@cluster pub fn execute_declared_work(...) -> Int do 1 + 1 end` so later parity rails keep one clustered work contract.
  - Historical M045/M046 route-free rails now share centralized `@cluster` contract strings from `compiler/meshc/tests/support/m046_route_free.rs` instead of pinning stale wording independently.
  - For `startup::...` request keys, automatic recovery now reuses the startup single-node replica relaxation before re-submitting declared work after promotion.
  - `scripts/verify-m047-s04.sh` is the single authoritative clustered cutover rail; M045/M046 verifier names remain replayable compatibility aliases that delegate to it.
patterns_established:
  - Source-first clustered parity works best when scaffold output, repo examples, and exact-string historical rails all share one canonical contract string source instead of hand-copied wording.
  - Hard cutovers should fail closed with one migration-oriented diagnostic and an authoritative retained verifier bundle, not with long-lived compatibility aliases hidden across multiple subsystems.
  - Legacy verifier names can survive as compatibility aliases, but only if they delegate to one authoritative rail and preserve the delegated bundle markers instead of re-running their own divergent logic.
  - Repo-owned examples stay trustworthy when their smoke tests assert both the positive source-first markers and the absence of the retired public tokens/helper seams.
observability_surfaces:
  - `scripts/verify-m047-s04.sh` retains `.tmp/m047-s04/verify/{status.txt,current-phase.txt,phase-report.txt,full-contract.log,latest-proof-bundle.txt}` as the authoritative cutover status surface.
  - Legacy clustered-source and manifest failures now surface one explicit migration diagnostic anchored on the real source or manifest location instead of compatibility-path cascades.
  - Route-free clustered examples and historical rails still use `meshc cluster status|continuity|diagnostics` as the runtime-owned operator surface.
  - `cargo test -p mesh-rt startup_automatic_recovery_relaxes_single_node_required_replica_count -- --nocapture` now guards the recovery seam found during slice closeout.
drill_down_paths:
  - .gsd/milestones/M047/slices/S04/tasks/T01-SUMMARY.md
  - .gsd/milestones/M047/slices/S04/tasks/T02-SUMMARY.md
  - .gsd/milestones/M047/slices/S04/tasks/T03-SUMMARY.md
  - .gsd/milestones/M047/slices/S04/tasks/T04-SUMMARY.md
  - .gsd/milestones/M047/slices/S04/tasks/T05-SUMMARY.md
  - .gsd/milestones/M047/slices/S04/tasks/T06-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-01T10:47:51.820Z
blocker_discovered: false
---

# S04: Hard cutover and dogfood migration

**Mesh now teaches a single public clustered function story: `@cluster` is the supported source surface, legacy `clustered(work)` / `[cluster]` fail closed with migration guidance, and the scaffold/example/verifier stack all dogfoods the same route-free contract.**

## What Happened

S04 finished the public clustered-authoring reset that M047/S01 deliberately left as a compatibility bridge. At the language/package layer, the live parser and manifest-loading paths no longer accept `clustered(work)` or `[cluster]` as supported clustered declarations: they now fail closed with explicit migration guidance while valid `@cluster` / `@cluster(N)` declarations keep their source-ranged compiler and LSP diagnostics. At the scaffold and dogfood layer, `meshc init --clustered`, `tiny-cluster/`, and `cluster-proof/` now all share the same route-free source-first contract: package-only `mesh.toml`, `main.mpl` booting through `Node.start_from_env()`, and `work.mpl` declaring `@cluster pub fn execute_declared_work(...) -> Int do 1 + 1 end` with runtime-owned inspection through `meshc cluster status|continuity|diagnostics`. The historical M045/M046 route-free rails were then rewired to those same shared contract strings so old proof surfaces still localize failures honestly without teaching the removed model. While closing the slice, the verification replay exposed one real runtime regression that was not visible in the earlier text-only migration work: after standby promotion, startup automatic recovery could re-submit with the raw declared-handler replica requirement and reject itself with `replica_required_unavailable`. Fixing that seam in `compiler/mesh-rt/src/dist/node.rs` kept the cutover rails honest and preserved the existing route-free startup failover story. Finally, S04 added a single authoritative proof surface for the new model: `scripts/verify-m047-s04.sh` now replays parser/pkg/compiler cutover checks, scaffold output checks, package smoke/build checks, docs build, and the new `e2e_m047_s04` contract test, then retains one coherent `.tmp/m047-s04/verify/` bundle. The older M045/M046 wrapper script names still work, but only as compatibility aliases into that M047 rail. The net result is that the repo now teaches one public clustered function model — source-first `@cluster` on ordinary route-free functions — while staying explicit that `HTTP.clustered(...)` is still missing and belongs to later slices, not to this cutover.

## Verification

All slice-plan verification rails passed in one serial replay recorded at `.tmp/m047-s04-closeout/verification.log`. That matrix covered: `cargo test -p mesh-parser m047_s04 -- --nocapture`; `cargo test -p mesh-pkg m047_s04 -- --nocapture`; `cargo test -p meshc --test e2e_m047_s01 -- --nocapture`; `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture`; `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`; `cargo run -q -p meshc -- test tiny-cluster/tests`; `cargo run -q -p meshc -- build tiny-cluster`; `cargo run -q -p meshc -- test cluster-proof/tests`; `cargo run -q -p meshc -- build cluster-proof`; `cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture`; `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture`; `cargo test -p meshc --test e2e_m045_s03 m045_s03_ -- --nocapture`; `cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture`; `cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture`; `cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture`; `cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture`; `npm --prefix website run build`; `cargo test -p meshc --test e2e_m047_s04 -- --nocapture`; `cargo test -p meshc --test e2e_m045_s04 -- --nocapture`; `cargo test -p meshc --test e2e_m045_s05 -- --nocapture`; and `bash scripts/verify-m047-s04.sh`. The authoritative verifier retained `.tmp/m047-s04/verify/status.txt=ok`, `.tmp/m047-s04/verify/current-phase.txt=complete`, a phase report showing every phase passed, and `.tmp/m047-s04/verify/latest-proof-bundle.txt` pointing at the retained artifact bundle.

## Requirements Advanced

- R099 — By migrating scaffold/package/proof examples to ordinary `@cluster` functions instead of route-only or manifest-only surfaces, S04 kept the new model grounded in general clustered function capability rather than an HTTP-only story.
- R106 — README, VitePress pages, package READMEs, and verifier references now teach one source-first route-free clustered model with explicit migration guidance, but the final docs/assembled closeout still belongs to S06.

## Requirements Validated

- R097 — `cargo test -p mesh-parser m047_s04 -- --nocapture`, `cargo test -p mesh-pkg m047_s04 -- --nocapture`, `cargo test -p meshc --test e2e_m047_s01 -- --nocapture`, and `bash scripts/verify-m047-s04.sh` prove `@cluster` / `@cluster(N)` are now the supported public clustered syntax while `clustered(work)` is rejected with migration guidance.
- R102 — Parser/pkg/compiler flows reject `clustered(work)` and `[cluster]`, scaffold/examples/docs/readmes were migrated to `@cluster`, and the assembled cutover verifier `bash scripts/verify-m047-s04.sh` passed with retained artifacts under `.tmp/m047-s04/verify/`.
- R103 — `meshc init --clustered`, `tiny-cluster/`, `cluster-proof/`, the historical route-free rails, and the public docs/verifier story now all dogfood `@cluster pub fn execute_declared_work(...)` and reject the old helper/manifest markers; the full slice verification matrix passed.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

T05 was planned as a docs-only task, but the required historical verification replay exposed a deterministic runtime regression in startup automatic recovery after standby promotion. I fixed that narrow seam in `compiler/mesh-rt/src/dist/node.rs` in the same slice because the failure was local, reproducible, and directly blocked the slice acceptance rail. The slice also confirmed that the GSD requirements DB still does not know the M047 requirement family (`R097`–`R106`), so status changes were recorded through requirement decisions instead of `gsd_requirement_update`.

## Known Limitations

`HTTP.clustered(...)` still does not exist; S04 intentionally keeps docs, examples, and verifiers explicit that clustered HTTP route wrappers remain unshipped. The GSD requirements DB still rejects M047 IDs (`R097`–`R106`) as unknown even though the checked-in `.gsd/REQUIREMENTS.md` renders them, so automated requirement status projection is still incomplete for this milestone.

## Follow-ups

1. S05 should build the SQLite Todo scaffold directly on the now-single public clustered contract (`@cluster` plus route-free runtime-owned inspection) and must not reintroduce manifest clustering, helper names, or proof-app route seams.
2. S06 should treat `bash scripts/verify-m047-s04.sh` as the retained prerequisite rail, then extend the docs/assembled closeout around it rather than minting another coequal authority.
3. If milestone closeout needs rendered requirement status changes, someone will need to repair the GSD requirements DB projection for `R097`–`R106`; the checked-in `.gsd/REQUIREMENTS.md` remains the truthful visible contract in the meantime.

## Files Created/Modified

- `compiler/mesh-parser/src/parser/items.rs` — Removed live legacy `clustered(work)` acceptance, kept source-local cutover diagnostics, and preserved valid `@cluster` AST access.
- `compiler/mesh-parser/src/ast/item.rs` — Restricted clustered AST access to the supported decorator path instead of exposing the legacy marker as a live clustered declaration.
- `compiler/mesh-pkg/src/manifest.rs` — Rejected legacy `[cluster]` manifest sections with explicit migration guidance and kept clustered export-surface construction source-first.
- `compiler/mesh-pkg/src/scaffold.rs` — Switched generated clustered projects to `@cluster pub fn execute_declared_work(...)` with package-only manifests and route-free bootstrap/readme guidance.
- `compiler/meshc/tests/tooling_e2e.rs` — Pinned CLI scaffold output to the new source-first contract and legacy-marker absence.
- `tiny-cluster/work.mpl` — Migrated the canonical local proof package to the shared `@cluster` route-free contract.
- `tiny-cluster/tests/work.test.mpl` — Updated tiny-cluster smoke tests and README to require the new source-first contract and runtime-owned CLI inspection story.
- `cluster-proof/work.mpl` — Migrated the packaged proof app to the shared `@cluster` route-free contract.
- `cluster-proof/tests/work.test.mpl` — Updated cluster-proof smoke tests and README to reject legacy helper/manifest markers and point at the new authoritative verifier.
- `compiler/meshc/tests/support/m046_route_free.rs` — Centralized the shared route-free `@cluster` contract literals that historical M045/M046 rails now reuse.
- `compiler/meshc/tests/e2e_m045_s01.rs` — Repointed historical route-free e2e rails to the new source-first contract and compatibility-alias verifier story.
- `compiler/meshc/tests/e2e_m046_s03.rs` — Kept the two-node startup/failover route-free rails green after the cutover and the startup recovery fix.
- `README.md` — Updated public clustered docs and README surfaces to teach one route-free `@cluster` model and explicit migration guidance without over-claiming `HTTP.clustered(...)`.
- `scripts/verify-m047-s04.sh` — Added the authoritative S04 cutover verifier with retained phase/bundle markers and repointed older wrapper scripts to delegate to it.
- `compiler/mesh-rt/src/dist/node.rs` — Adjusted startup automatic recovery so promoted single-node standby recovery reuses the startup replica relaxation instead of self-rejecting with `replica_required_unavailable`.
