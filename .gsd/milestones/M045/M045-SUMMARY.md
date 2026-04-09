---
id: M045
title: "Clean Language-Owned Clustered Example"
status: complete
completed_at: 2026-03-31T03:08:28.514Z
key_decisions:
  - Use `meshc init --clustered` plus one local scaffold-first example as the primary clustered teaching surface, with `cluster-proof` demoted to a deeper secondary rail.
  - Keep clustered behavior ownership in the runtime/language: bootstrap, routing choice, authority/failover, and status truth must come from runtime/codegen plus public CLI surfaces, not example-side helpers.
  - Expose clustered bootstrap as typed public Mesh API (`Node.start_from_env()` returning `BootstrapStatus`) instead of hand-rolled env parsing and direct `Node.start(...)` orchestration in examples.
  - Keep declared-work completion runtime/codegen-owned and register only manifest-approved declared wrapper symbols for remote spawn.
  - Use runtime CLI truth (`meshc cluster status`, `meshc cluster continuity`, `meshc cluster diagnostics`) plus retained artifact bundles as the clustered acceptance surface.
  - Keep the tiny scaffold valid for both single-node and two-node failover rails by deriving replica requirements from live peers (`Node.list()`) and using only a small deterministic pending window.
  - Promote M045’s S04/S05 verifier stack as the live closeout contract and leave M044’s older wrappers as historical transition checks rather than the present-tense story.
key_files:
  - compiler/mesh-rt/src/dist/bootstrap.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-codegen/src/codegen/mod.rs
  - compiler/mesh-codegen/src/codegen/expr.rs
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/tests/e2e_m045_s01.rs
  - compiler/meshc/tests/e2e_m045_s02.rs
  - compiler/meshc/tests/e2e_m045_s03.rs
  - compiler/meshc/tests/e2e_m045_s04.rs
  - compiler/meshc/tests/e2e_m045_s05.rs
  - cluster-proof/main.mpl
  - cluster-proof/work.mpl
  - cluster-proof/work_continuity.mpl
  - scripts/verify-m045-s04.sh
  - scripts/verify-m045-s05.sh
  - website/docs/docs/getting-started/clustered-example/index.md
  - website/docs/docs/getting-started/index.md
lessons_learned:
  - For clustered examples, the honest product boundary is runtime/codegen plus public `meshc cluster` surfaces; if the example needs custom placement, completion, or failover helpers, the ownership boundary has regressed.
  - One scaffold can cover both the simple clustered path and destructive failover proof if replica requirements follow live peer presence and the pending window is deterministic but minimal.
  - Top-level verifier wrappers are more stable when they replay direct prerequisite commands and retain leaf bundles explicitly instead of nesting older wrappers that can hang after already-passed phases.
  - Public docs and closeout rails should point at the same scaffold-first surface; older proof rails can remain as historical migration checks, but not as the present-tense teaching story.
---

# M045: Clean Language-Owned Clustered Example

**M045 turned the clustered Mesh story into a small docs-first scaffold example whose bootstrap, remote execution, failover, and status truth are owned by the runtime and public CLI surfaces instead of example-side glue.**

## What Happened

M045 removed the remaining proof-app-shaped seams from Mesh’s primary clustered example and replaced them with runtime-owned surfaces end to end. S01 moved clustered startup behind the public typed `Node.start_from_env()` / `BootstrapStatus` boundary, so scaffolded clustered apps and `cluster-proof` stopped hand-rolling cluster-mode detection, identity parsing, and direct `Node.start(...)` orchestration. S02 finished the tiny happy path by making declared-work completion runtime/codegen-owned, keeping the scaffold small while proving runtime-chosen remote execution and completed continuity truth on a two-node local cluster. S03 kept that same tiny scaffold surface for destructive failover, adding only the minimum runtime-honest timing seam needed to observe mirrored pending work before primary loss and then proving automatic promotion, recovery, and stale-primary rejoin through `meshc cluster status|continuity|diagnostics`. S04 removed the last obvious legacy example-side cluster glue from `cluster-proof`, moving the manifest-declared handler into `Work.execute_declared_work`, leaving `work_continuity.mpl` as a submit/status translator only, and promoting the M045 verifier/docs surfaces over the older M044 closeout story. S05 rewrote the public teaching surface so `/docs/getting-started/clustered-example/` is now the first clustered path readers see, while `scripts/verify-m045-s05.sh` replays the S04 contract, rebuilds the docs, and retains the fresh S04/S03 evidence chain.

## Verification Notes

- Code changes exist beyond `.gsd/`: the literal `git diff --stat HEAD $(git merge-base HEAD main) -- ':!.gsd/'` check returned empty because local `main` already contains merged milestone work in this repo. Per repo knowledge, the equivalent integration-baseline check against `origin/main` was used and showed non-`.gsd/` changes. A targeted `git diff --name-only HEAD $(git merge-base HEAD origin/main) -- ...` confirmed M045-owned surfaces changed, including `compiler/mesh-pkg/src/scaffold.rs`, `cluster-proof/main.mpl`, `compiler/meshc/tests/e2e_m045_s03.rs`, `compiler/meshc/tests/e2e_m045_s04.rs`, `compiler/meshc/tests/e2e_m045_s05.rs`, `scripts/verify-m045-s05.sh`, and `website/docs/docs/getting-started/clustered-example/index.md`.
- Terminal assembled verification passed: `bash scripts/verify-m045-s05.sh` completed green with `.tmp/m045-s05/verify/status.txt = ok`, `.tmp/m045-s05/verify/current-phase.txt = complete`, and phase report entries for the retained S04 replay, S05 contract, and docs build.
- The retained S04 replay inside S05 also passed (`.tmp/m045-s05/verify/retained-m045-s04-verify/status.txt = ok`), proving the closeout chain still replays S02, S03, cluster-proof build/tests, and the current distributed closeout contract before the docs-first wrapper claims success.

## Decision Re-evaluation

| Decisions | Re-evaluation | Verdict | Evidence |
|---|---|---|---|
| D208–D210 | Keep `meshc init --clustered` as the primary clustered teaching surface, keep runtime ownership of clustered behavior, and require the same tiny example to carry failover as well as happy-path clustering. | Still valid | Delivered by the scaffold-first S02/S03 proof rails and the S05 docs-first rewrite. |
| D211–D214 | Put clustered bootstrap behind typed `Node.start_from_env()` and keep `cluster-proof` on the same runtime-owned startup boundary. | Still valid | S01 shipped the public bootstrap seam and both scaffold/`cluster-proof` now consume it. |
| D215–D217 | Keep completion/status truth runtime-owned, register only manifest-approved declared wrappers for remote spawn, and use runtime CLI truth as the tiny-example acceptance surface. | Still valid | S02 rails and retained bundles prove remote-owner execution/completion through `meshc cluster status` and `meshc cluster continuity` without app-owned helpers. |
| D218 & D220 | Prove failover on the scaffold-first example with runtime CLI truth and a small peer-aware timing seam instead of new app-owned failover logic. | Still valid | S03’s retained failover bundle records mirrored pending, automatic recovery (`attempt-1` -> `attempt-2`), and stale-primary rejoin on the tiny scaffold surface. |
| D221–D224 | Promote M045’s S04 rail as the current public clustered closeout contract and collapse the last `cluster-proof` declared-work glue into `Work.execute_declared_work`. | Still valid | The retained S04 replay passed inside S05 and remains the live distributed closeout contract; M044 is now only a transition checker. |
| D225–D227 | Make the docs lead with the small clustered example and keep the M045 S05 wrapper as the present-tense closeout rail. | Still valid | `scripts/verify-m045-s05.sh` is green, the Getting Started clustered example exists, and the docs build passed. |

No M045 decision currently needs immediate rework next milestone.

## Success Criteria Results

The roadmap does not contain a separate `Success Criteria` section, so milestone closeout verified the vision plus each slice overview row’s `After this` contract.

- **S01 after-this met:** `meshc init --clustered` now produces a visibly smaller clustered app whose startup and inspection path are runtime/public-surface owned. Evidence: S01 summary, shipped `Node.start_from_env()` / `BootstrapStatus`, scaffold rewrite, and the retained M045 bootstrap rails.
- **S02 after-this met:** one small local clustered example runs on two nodes and proves runtime-chosen remote execution without app-owned routing or placement logic. Evidence: S02 summary; `meshc cluster status --json` + dual-node `meshc cluster continuity --json` acceptance surface; S04 replay inside S05 shows `m045-s02-replay passed`.
- **S03 after-this met:** the same tiny example survives primary loss and reports failover/status truth from the runtime without app-owned authority or failover choreography. Evidence: S03 summary; `.tmp/m045-s03/verify/status.txt = ok`; `.tmp/m045-s03/verify/03-m045-s03-e2e.test-count.log` reports `running-counts=[3]`; retained scenario metadata shows `attempt-1` rolling to `attempt-2` on standby after primary loss.
- **S04 after-this met:** old `cluster-proof`-style placement/config/status glue is gone or deeply collapsed, and the repo no longer teaches example-owned distributed mechanics as the primary story. Evidence: D221–D224, changed `cluster-proof/main.mpl` / `cluster-proof/work.mpl` / `cluster-proof/work_continuity.mpl`, and retained S04 replay with `03-e2e-m045-s04: running-counts=[3]`, `cluster-proof-build passed`, and `cluster-proof-tests passed`.
- **S05 after-this met:** the docs teach the tiny clustered example first, deeper proof rails are secondary, and the verifier stack proves the same simple language-owned story end to end. Evidence: `bash scripts/verify-m045-s05.sh` passed; `.tmp/m045-s05/verify/03-e2e-m045-s05.test-count.log` reports `running-counts=[2]`; docs build passed; Getting Started clustered docs page is present.
- **Vision met:** the primary clustered example is now small, docs-grade, and language/runtime-owned while still showing clustering, runtime-chosen remote execution, and failover on one local example. Evidence is the assembled S05 verifier plus the requirement outcomes below.

No success-criterion failures were found.

## Definition of Done Results

- **All roadmap slices complete:** roadmap overview shows S01–S05 all marked `✅`.
- **All slice summaries exist:** `find .gsd/milestones/M045 -maxdepth 4 ...` confirmed `S01-SUMMARY.md` through `S05-SUMMARY.md`, along with matching plan and UAT artifacts.
- **Cross-slice integration works:** the terminal wrapper `bash scripts/verify-m045-s05.sh` passed; its retained S04 replay also passed, and that replay explicitly records `m045-s02-replay passed`, `m045-s03-replay passed`, `cluster-proof-build passed`, `cluster-proof-tests passed`, and `docs-build passed`.
- **Code-change verification passed:** the repository’s integration-baseline equivalent (`origin/main`) shows non-`.gsd/` code/doc/verifier changes, and targeted diff checks hit the exact runtime/scaffold/tests/docs surfaces M045 was supposed to change.
- **Horizontal checklist:** the roadmap has no `Horizontal Checklist` section, so there were no separate checklist items to audit.

No definition-of-done failures were found.

## Requirement Outcomes

| Requirement | Transition | Evidence |
|---|---|---|
| R077 | active -> validated | S01 made clustered bootstrap runtime-owned through `Node.start_from_env()` and typed `BootstrapStatus`; S02 kept the scaffold tiny while moving completion into runtime/codegen; S04 removed remaining example-side `cluster-proof` glue; S05 made the smaller scaffold-first example the docs surface. The final wrapper `bash scripts/verify-m045-s05.sh` passed. |
| R078 | active -> validated | S02 proved cluster formation plus runtime-chosen remote execution on the tiny scaffold surface; S03 proved the same tiny example survives primary loss and reports truthful failover status; the retained S03 bundle records `attempt-1` -> `attempt-2` recovery on the same request key, and S05 replays that evidence chain successfully. |
| R079 | active -> validated | Across S01/S02/S03/S04, app-owned bootstrap, placement, completion, failover, and status seams were removed or collapsed behind runtime/codegen and `meshc cluster` truth surfaces. The current proof rails depend on `meshc cluster status|continuity|diagnostics`, not app-owned status helpers. |
| R080 | active -> validated | S05 made `meshc init --clustered` the first-class public clustered entry surface, added the Getting Started clustered tutorial, passed `cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture`, and passed `npm --prefix website run build` inside `bash scripts/verify-m045-s05.sh`. |
| R081 | active -> validated | S05 reordered docs/readme guidance so readers land on `/docs/getting-started/clustered-example/` before deeper proof material; the docs build and `m045_s05_` contract rails passed, and the S05 wrapper keeps the deeper proof chain secondary but retained. |

## Deviations

The roadmap ships without standalone `Success Criteria` or `Horizontal Checklist` sections, so closeout verified the vision plus each slice overview row’s `After this` contract. The local integration baseline also required the repo’s documented `origin/main` equivalent for non-`.gsd` diff verification because `git diff ... $(git merge-base HEAD main)` can go empty on this host even when milestone code shipped. Verification passed with that repo-specific baseline and with the assembled S05 wrapper.

## Follow-ups

Future clustered milestones should preserve the M045 scaffold-first entrypoint and only grow runtime/bootstrap/operator surfaces when the runtime truly owns the new behavior. If new clustered regressions appear, start from the retained evidence chain (`.tmp/m045-s05/verify/retained-m045-s04-verify/` -> `.tmp/m045-s03/verify/retained-m045-s03-artifacts/`) before inventing new app-owned diagnostics.
