---
id: M046
title: "Language-Owned Tiny Cluster Proofs"
status: complete
completed_at: 2026-04-01T04:02:25.445Z
key_decisions:
  - D229/D230/D231: keep source-level clustered declaration intentionally narrow (`clustered(work)` on `fn|def`) and merge it with manifest declarations inside `mesh-pkg` instead of inventing a generic annotation system or a second planner.
  - D234/D235/D236/D237/D239: make startup work runtime-owned by keying registration and discovery on declared handler runtime names, using deterministic startup request identity, primary-only startup submission, route-free keepalive/diagnostics, and deterministic duplicate-session convergence for simultaneous two-node boot.
  - D244/D228: keep proof apps visibly tiny by moving failover observability behind a Mesh-owned `startup_dispatch_window` seam and keeping the canonical workload at trivial arithmetic (`1 + 1`).
  - D247/D248/D249/D250: rebuild `cluster-proof/` as a route-free direct-binary package and share local/package runtime proof harness logic rather than preserving legacy HTTP/Fly proof seams.
  - D252/D253/D254/D255/D259/D263: standardize the scaffold, `tiny-cluster/`, `cluster-proof/`, docs, and closeout rails on one runtime-owned operator flow, with S05 as the equal-surface subrail and S06 as the final authoritative milestone closeout surface.
key_files:
  - compiler/mesh-parser/src/parser/items.rs
  - compiler/mesh-parser/src/ast/item.rs
  - compiler/mesh-pkg/src/manifest.rs
  - compiler/mesh-codegen/src/codegen/mod.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/meshc/src/cluster.rs
  - compiler/meshc/src/main.rs
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/tests/support/m046_route_free.rs
  - compiler/meshc/tests/e2e_m046_s01.rs
  - compiler/meshc/tests/e2e_m046_s02.rs
  - compiler/meshc/tests/e2e_m046_s03.rs
  - compiler/meshc/tests/e2e_m046_s04.rs
  - compiler/meshc/tests/e2e_m046_s05.rs
  - compiler/meshc/tests/e2e_m046_s06.rs
  - scripts/verify-m046-s03.sh
  - scripts/verify-m046-s04.sh
  - scripts/verify-m046-s05.sh
  - scripts/verify-m046-s06.sh
  - tiny-cluster/main.mpl
  - tiny-cluster/work.mpl
  - cluster-proof/main.mpl
  - cluster-proof/work.mpl
  - README.md
  - website/docs/docs/getting-started/clustered-example/index.md
lessons_learned:
  - On local integration-branch closeout, the non-`.gsd` code-change proof may need an integration-range diff instead of the literal `merge-base main` diff, because `HEAD` can already equal local `main` even when the milestone landed substantial code.
  - The stable join point for language-owned clustered work is the declared handler runtime name: it lets parser/compiler planning, runtime startup identity, CLI continuity discovery, and proof rails all speak the same identifier.
  - Route-free proof apps stay honest only when proof-only timing and pending-window control live behind Mesh-owned runtime seams rather than package code or user-facing env knobs.
  - Historical wrapper rails are safest as thin delegates over one current verifier plus retained bundle checks; trying to keep old wrappers semantically alive reintroduces deleted product seams and splits present-tense proof authority.
  - For assembled clustered closeout, one top-level retained proof pointer plus a separately copied delegated verifier tree gives faster diagnosis than re-running or re-explaining the whole proof chain every time.
---

# M046: Language-Owned Tiny Cluster Proofs

**M046 made clustered proof apps language-owned end to end: clustered work can be declared in source or manifest, startup is runtime-triggered and route-free, `tiny-cluster/`, rebuilt `cluster-proof/`, and `meshc init --clustered` now share one tiny `1 + 1` clustered contract, and the milestone closes on the authoritative S06 assembled verifier.**

## What Happened

M046 completed the remaining move from proof-app-owned clustered behavior to Mesh-owned language/runtime/tooling behavior. S01 added the narrow source-level `clustered(work)` declaration and merged it with manifest declarations through the shared clustered planner so both surfaces converge on the same declared-handler runtime boundary. S02 moved startup triggering and startup-state inspection onto runtime-owned surfaces by threading clustered startup registrations through codegen, having `mesh-rt` auto-submit deterministic startup work on boot, and surfacing `declared_handler_runtime_name` plus startup lifecycle truth through `meshc cluster status|continuity|diagnostics`. S03 then shipped a real repo-owned `tiny-cluster/` package that stays route-free, source-first, and visibly trivial (`1 + 1`) while proving startup dedupe, failover promotion/recovery/completion, and fenced rejoin entirely through runtime/tooling surfaces. S04 deleted the old routeful `cluster-proof/` shape and rebuilt it as the packaged sibling of `tiny-cluster/`, with direct-binary Docker/Fly packaging, shared route-free harness support, and historical M044/M045 wrappers demoted to delegates over the new verifier. S05 aligned `meshc init --clustered`, `tiny-cluster/`, `cluster-proof/`, docs, and verifier rails onto one equal-surface route-free clustered-work story. S06 closed the milestone on one authoritative assembled verifier, `scripts/verify-m046-s06.sh`, that wraps the S05 equal-surface rail, republishes fresh S03/S04 runtime truth under one retained bundle pointer, and anchors milestone validation/docs on that evidence chain. Because auto-mode was already running on the local integration branch (`main`), the literal `git diff --stat HEAD $(git merge-base HEAD main) -- ':!.gsd/'` check was empty; using the integration-branch equivalent milestone range `ba2ed4b1^..HEAD` showed 75 non-`.gsd/` files changed across parser, package tooling, runtime, CLI, proofs, and docs, confirming this milestone produced real code and product-surface changes rather than planning-only artifacts.

## Success Criteria Results

- [x] **Dual clustered-work declaration converges on one runtime boundary.** S01 delivered source-level `clustered(work)` plus manifest support through the shared planner, and validated convergence with `cargo test -p mesh-parser --test parser_tests m046_s01_parser_ -- --nocapture`, `cargo test -p mesh-pkg m046_s01_ -- --nocapture`, `cargo test -p meshc --test e2e_m046_s01 m046_s01_ -- --nocapture`, `cargo test -p mesh-lsp m046_s01_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture`, and `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture`.
- [x] **Route-free clustered apps auto-run startup work and are inspected entirely through built-in `meshc cluster ...` surfaces.** S02 moved startup registration/triggering into runtime/codegen and proved the route-free startup contract with `cargo test -p mesh-rt startup_work_ -- --nocapture`, `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_ -- --nocapture`, and `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture`.
- [x] **`tiny-cluster/` is a local route-free proof with trivial `1 + 1` work and runtime-owned failover/status truth.** S03 shipped the repo package and verified it with `cargo run -q -p meshc -- build tiny-cluster`, `cargo run -q -p meshc -- test tiny-cluster/tests`, `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_ -- --nocapture`, `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_startup_ -- --nocapture`, `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_failover_ -- --nocapture`, and `bash scripts/verify-m046-s03.sh`.
- [x] **`cluster-proof/` is rebuilt as a tiny packaged route-free proof on the same trivial clustered contract.** S04 validated the reset package with `cargo run -q -p meshc -- build cluster-proof && test ! -e cluster-proof/cluster.mpl && test ! -e cluster-proof/config.mpl && test ! -e cluster-proof/work_continuity.mpl`, `cargo run -q -p meshc -- test cluster-proof/tests && docker build -f cluster-proof/Dockerfile -t mesh-cluster-proof:m046-s04-local .`, `cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture`, `bash scripts/verify-m046-s04.sh`, and the delegated M044/M045 wrapper rails.
- [x] **The scaffold, `tiny-cluster/`, and `cluster-proof/` now express the same clustered-work story and fail closed if they drift.** S05 validated equal-surface alignment with `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`, `cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture`, `npm --prefix website run build`, `bash scripts/verify-m046-s05.sh`, and `bash scripts/verify-m045-s05.sh`.
- [x] **One assembled verifier replays the local and packaged route-free proofs and re-checks startup-triggered work, failover, and status truth end to end.** S06 validated the final closeout seam with `cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture`, `cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture`, `npm --prefix website run build`, `bash scripts/verify-m046-s06.sh`, `bash scripts/verify-m045-s05.sh`, and the rendered `.gsd/milestones/M046/M046-VALIDATION.md` pass verdict plus live `.tmp/m046-s06/verify/status.txt=ok`, `.tmp/m046-s06/verify/current-phase.txt=complete`, and `.tmp/m046-s06/verify/latest-proof-bundle.txt`.

No separate horizontal checklist is present in `M046-ROADMAP.md`; the slice `After this` outcomes above are the milestone’s delivered success criteria.

## Definition of Done Results

- [x] **All planned slices are complete.** `M046-ROADMAP.md` marks S01–S06 done, and `gsd_complete_milestone` validated slice completion before rendering the milestone summary.
- [x] **All slice summaries exist.** `find .gsd/milestones/M046/slices -maxdepth 2 -name 'S*-SUMMARY.md' | sort` returned S01 through S06, and all 22 task summaries exist under the slice task directories.
- [x] **The milestone produced real code/product changes, not only planning artifacts.** On the local integration branch the literal merge-base diff against `main` was empty because `HEAD` is already on `main`; using the equivalent integration-branch milestone range `ba2ed4b1^..HEAD` showed 75 non-`.gsd/` files changed, including parser, package, runtime, CLI, verifier, package, and docs surfaces.
- [x] **Cross-slice integration points work correctly.**
  - S01’s shared clustered declaration planner is consumed by S02 startup registration/runtime-name discovery.
  - S02’s runtime-owned startup and CLI inspection contract is reused directly by S03 `tiny-cluster/`, S04 `cluster-proof/`, S05 scaffold parity, and S06 assembled closeout.
  - S05’s equal-surface verifier and retained bundle chain are wrapped by S06 rather than forked.
  - `.tmp/m046-s06/verify/status.txt`, `current-phase.txt`, `latest-proof-bundle.txt`, and `.gsd/milestones/M046/M046-VALIDATION.md` are present and point at the final retained S06 evidence chain.
- [x] **Public/docs/operator truth matches shipped behavior.** S05 and S06 repointed README/docs/package runbooks to the route-free operator sequence and passed `npm --prefix website run build` plus routeful-string/content-guard rails.
- [x] **Milestone validation exists and passed.** `.gsd/milestones/M046/M046-VALIDATION.md` is present with `verdict: pass` and cites the S06 closeout rail and requirement coverage.

No unmet definition-of-done items were found.

## Requirement Outcomes

| Requirement | From -> To | Evidence |
| --- | --- | --- |
| R085 | active -> validated | S01 added source-level `clustered(work)` and merged it with manifest declarations through the shared clustered planner; validated by `cargo test -p mesh-parser --test parser_tests m046_s01_parser_ -- --nocapture`, `cargo test -p mesh-pkg m046_s01_ -- --nocapture`, `cargo test -p meshc --test e2e_m046_s01 m046_s01_ -- --nocapture`, `cargo test -p mesh-lsp m046_s01_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s01 m044_s01_ -- --nocapture`, and `cargo test -p meshc --test e2e_m044_s02 m044_s02_ -- --nocapture`. |
| R086 | active -> validated | S02 moved startup triggering/status truth into runtime/tooling, S03/S04 kept both proof apps at `clustered(work)` + `Node.start_from_env()` only, and S06 validated the assembled route-free proof chain via `bash scripts/verify-m046-s06.sh` plus `.gsd/milestones/M046/M046-VALIDATION.md`. |
| R087 | active -> validated | S02 proved route-free startup submission and inspection without app-owned HTTP/status or explicit continuity submission calls using `cargo test -p mesh-rt startup_work_ -- --nocapture`, `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_ -- --nocapture`, and `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture`; S06 retained that evidence in the final closeout chain. |
| R088 | active -> validated | S03 shipped `tiny-cluster/` as the local-first route-free proof and validated it with `cargo run -q -p meshc -- build tiny-cluster`, `cargo run -q -p meshc -- test tiny-cluster/tests`, `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_ -- --nocapture`, `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_startup_ -- --nocapture`, `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_failover_ -- --nocapture`, and `bash scripts/verify-m046-s03.sh`; S06 republishes the retained bundles. |
| R089 | active -> validated | S04 rebuilt `cluster-proof/` as the tiny packaged route-free proof and validated it with `cargo run -q -p meshc -- build cluster-proof`, `cargo run -q -p meshc -- test cluster-proof/tests && docker build -f cluster-proof/Dockerfile -t mesh-cluster-proof:m046-s04-local .`, `cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture`, `bash scripts/verify-m046-s04.sh`, and delegated wrapper rails; S06 retains the packaged startup bundle. |
| R090 | active -> validated | S05 locked `meshc init --clustered`, `tiny-cluster/`, and `cluster-proof/` to one equal-surface contract and validated it with `cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`, the M044/M045 scaffold guards, `cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture`, and `bash scripts/verify-m046-s05.sh`; S06 nests that green S05 bundle inside the final proof pointer. |
| R091 | active -> validated | S02 established `meshc cluster status|continuity|diagnostics` as the runtime-owned inspection contract, S03/S04 proved failover/startup truth through those CLI surfaces, and S06 validated the retained runtime/tooling artifact chain with `bash scripts/verify-m046-s06.sh` and `.gsd/milestones/M046/M046-VALIDATION.md`. |
| R092 | active -> validated | S05 repointed public docs/runbooks/verifiers to the route-free operator story and validated it with `npm --prefix website run build`, routeful-string guards, `cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture`, and `bash scripts/verify-m046-s05.sh`; S06 preserved that authority hierarchy with additional content guards and the assembled closeout rail. |
| R093 | active -> validated | S03 kept `tiny-cluster/work.mpl` at literal `1 + 1` and runtime-owned failover timing, S04 kept `cluster-proof/work.mpl` at the same trivial workload, and S06 replayed both bundles under the final milestone pointer. |

All active M046 requirement-family items R085–R093 are now validated in the checked-in requirements contract.

## Decision Re-evaluation

| Decision | Outcome after delivery | Revisit next milestone? |
| --- | --- | --- |
| D229/D230/D231 — narrow source `clustered(work)` surface merged through the shared `mesh-pkg` planner | Still valid. S01 proved the narrow source surface was sufficient and shared planning kept compiler/LSP/runtime registration behavior aligned. | No |
| D234/D235/D236/D237/D239 — runtime-owned startup keyed by declared handler runtime name, primary-only submission, deterministic transport convergence | Still valid. S02 and the final S06 replay both depend on this runtime identity/ownership model for truthful route-free startup dedupe and two-node convergence. | No |
| D244/D228 — keep proof apps tiny by moving failover observability into Mesh-owned seams and keeping work at `1 + 1` | Still valid. S03/S04/S06 prove the tiny proof shape keeps orchestration complexity visibly Mesh-owned. | No |
| D247/D248/D249/D250 — rebuild `cluster-proof/` as a route-free direct-binary package with shared proof harness and thin historical delegates | Still valid. The rebuilt packaged proof and delegated wrappers stayed green and avoided reviving deleted HTTP/Fly seams. | No |
| D252/D253/D254/D255/D259/D263 — one canonical runtime-owned operator story, one equal-surface verifier, one final authoritative S06 closeout rail | Still valid. S05 and S06 delivered the intended hierarchy and the final retained bundle chain makes diagnosis faster, not slower. | No |
| D233 — keep the S01 happy-path compiler proof on the broader M044-shaped fixture until the older single-function LLVM verifier bug is fixed | Valid as a temporary truth-preserving workaround, but it remains technical debt because the minimal source-only fixture is still red for an unrelated backend reason. | Yes |

## Deviations

No milestone-goal deviation. The main closeout wrinkle was procedural: because auto-mode is operating on local `main`, the literal merge-base diff against `main` is empty, so milestone code-change verification had to use the equivalent integration-branch milestone range (`ba2ed4b1^..HEAD`). The slice-level work itself remained aligned with the roadmap and the S06 closeout rail is green.

## Follow-ups

- Future clustered regressions should start from `.tmp/m046-s06/verify/latest-proof-bundle.txt`, inspect `retained-m046-s05-verify/` first, and only then drill into the targeted S03/S04 retained bundles.
- Historical wrapper rails (`scripts/verify-m045-s05.sh` and older delegates) should remain compatibility surfaces only; do not re-promote them above the S06 closeout seam.
- The older single-function LLVM verifier issue that forced S01’s broader happy-path compiler fixture is still worth retiring in a future compiler/codegen milestone.
