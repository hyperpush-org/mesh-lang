---
id: M051
title: "Mesher as the Living Reference App"
status: complete
completed_at: 2026-04-05T03:46:12.504Z
key_decisions:
  - Adopt the scaffold-style deeper-app bootstrap for Mesher: validate app config, open Postgres locally, then call `Node.start_from_env()` as the only clustered bootstrap path.
  - Move backend-only migration/deploy/health/recovery proof into `scripts/fixtures/backend/reference-backend/` with dedicated maintainer-owned verifier surfaces instead of keeping a public top-level app.
  - Retarget tooling/editor/LSP/formatter rails to the bounded retained backend fixture while keeping public editor/docs wording generic and examples-first.
  - Keep the public deeper-backend handoff behind `/docs/production-backend-proof/`, with Mesher and the retained backend replay treated as maintainer-facing follow-ons.
  - Use `scripts/verify-production-proof-surface.sh` plus the self-contained `scripts/verify-m051-s05.sh` retained bundle as the canonical post-deletion proof surfaces.
key_files:
  - mesher/main.mpl
  - mesher/config.mpl
  - mesher/README.md
  - mesher/.env.example
  - compiler/meshc/tests/support/m051_mesher.rs
  - compiler/meshc/tests/support/m051_reference_backend.rs
  - compiler/meshc/tests/e2e_m051_s01.rs
  - compiler/meshc/tests/e2e_m051_s02.rs
  - compiler/meshc/tests/e2e_m051_s03.rs
  - compiler/meshc/tests/e2e_m051_s04.rs
  - compiler/meshc/tests/e2e_m051_s05.rs
  - scripts/fixtures/backend/reference-backend/README.md
  - scripts/verify-m051-s01.sh
  - scripts/verify-m051-s02.sh
  - scripts/verify-m051-s03.sh
  - scripts/verify-m051-s04.sh
  - scripts/verify-m051-s05.sh
  - scripts/verify-production-proof-surface.sh
  - compiler/mesh-pkg/src/scaffold.rs
  - tools/skill/mesh/skills/clustering/SKILL.md
  - website/docs/docs/production-backend-proof/index.md
  - README.md
lessons_learned:
  - When milestone closeout runs on local `main`, `git diff HEAD $(git merge-base HEAD main)` can be empty even after real delivery; use an equivalent milestone boundary instead of treating that as automatic failure.
  - Retained backend smoke must wait for the full healthy `/health` contract (`status=ok`, `liveness=healthy`, `recovery_active=false`) before creating work, or later DB-backed proof rails can go red for startup-window reasons instead of real regressions.
  - Final closeout bundles must copy delegated verify trees and rewrite copied `latest-proof-bundle.txt` pointers to the copied child bundles, or downstream validation depends on live child `.tmp` trees instead of one self-contained retained artifact root.
  - Exact-string docs and verifier guards are worth keeping for public-surface honesty, but they only stay useful if README/docs changes and the paired Rust/Node/shell contract rails are updated in the same change.
---

# M051: Mesher as the Living Reference App

**M051 retired the repo-root `reference-backend/` path, made Mesher the maintained deeper reference app on the current runtime/bootstrap contract, preserved backend-only proof as an internal retained fixture, and closed on one post-deletion acceptance rail.**

## What Happened

M051 replaced the old split reference-app story with one honest chain. S01 modernized `mesher/` onto the current scaffold-style runtime contract (`validate config -> open Postgres pool -> Node.start_from_env() -> listener/service startup`), added a package-local maintainer runbook, and introduced a dedicated Postgres-backed proof harness plus `bash scripts/verify-m051-s01.sh`. S02 extracted the backend-only migration/deploy/health/recovery proof into `scripts/fixtures/backend/reference-backend/`, rebound the retained backend rails through shared Rust support, and made `bash scripts/verify-m051-s02.sh` the maintainer-owned replay instead of leaving that proof in a public top-level app. S03 moved bounded tooling/editor/LSP/formatter rails onto the retained backend fixture, kept the public editor docs generic, and published one assembled retained tooling bundle. S04 retargeted README, VitePress docs, scaffold output, and the clustering skill so public readers stay on scaffold/examples-first surfaces and only reach deeper backend work through `/docs/production-backend-proof/`, with Mesher and the retained backend verifier explicitly maintainer-facing. S05 then deleted the repo-root `reference-backend/` tree, installed the canonical top-level proof-page verifier at `scripts/verify-production-proof-surface.sh`, kept the retained backend rails truthful post-deletion, and closed the milestone with `cargo test -p meshc --test e2e_m051_s05 -- --nocapture` plus `DATABASE_URL=postgresql://postgres:postgres@127.0.0.1:51798/mesh_m051_complete bash scripts/verify-m051-s05.sh`.

Milestone verification was rerun during closeout instead of relying only on historical slice summaries. The literal `git diff --stat HEAD $(git merge-base HEAD main) -- ':!.gsd/'` came back empty because local `HEAD`, `main`, and `origin/main` are already the same integration tip, so code-change verification used the allowed equivalent milestone boundary: the parent of the earliest M051-tagged commit (`34739a28^`). That diff shows extensive non-`.gsd` changes across Mesher bootstrap/runtime files, retained backend fixtures, compiler/tooling tests, docs, scaffold output, and the slice-owned verifier stack. On the same current tree, the final S05 replay finished green with `.tmp/m051-s05/verify/status.txt = ok`, `.tmp/m051-s05/verify/current-phase.txt = complete`, `.tmp/m051-s05/verify/latest-proof-bundle.txt -> .tmp/m051-s05/verify/retained-proof-bundle`, and a phase report showing the S01-S04 wrapper replays plus retained-bundle checks all passed.

## Success Criteria Results

## Success Criteria Results

The roadmap does not render a separate success-criteria bullet list, so closeout verified the milestone vision and each slice's delivered "After this" outcome as the success contract.

- **Mesher is the maintained deeper reference app on the current runtime/bootstrap contract — MET.**
  - Evidence: S01 delivered `mesher/config.mpl`, `mesher/main.mpl`, `mesher/.env.example`, `mesher/README.md`, `compiler/meshc/tests/support/m051_mesher.rs`, `compiler/meshc/tests/e2e_m051_s01.rs`, and `scripts/verify-m051-s01.sh`.
  - Proof: S01 recorded green package/build/e2e/verifier rails, and the milestone closeout replay re-ran `bash scripts/verify-m051-s01.sh` successfully as part of `bash scripts/verify-m051-s05.sh`.

- **Backend-only proof survives as retained maintainer infrastructure instead of a public top-level app — MET.**
  - Evidence: S02 delivered `scripts/fixtures/backend/reference-backend/`, `compiler/meshc/tests/support/m051_reference_backend.rs`, `compiler/meshc/tests/e2e_reference_backend.rs`, `compiler/meshc/tests/e2e_m051_s02.rs`, and `scripts/verify-m051-s02.sh`.
  - Proof: S02’s retained backend replay stayed green and was re-run successfully during the final S05 wrapper.

- **Tooling/editor/LSP/formatter rails no longer depend on repo-root `reference-backend/` — MET.**
  - Evidence: S03 retargeted `e2e_lsp`, `tooling_e2e`, `e2e_fmt`, `mesh-lsp`, `mesh-fmt`, VS Code smoke, Neovim smoke, and the shared syntax corpus to the retained backend fixture.
  - Proof: S03’s assembled rail was re-run by `bash scripts/verify-m051-s05.sh`; the current S05 phase report records `m051-s03-wrapper passed`.

- **Public docs, scaffold output, and bundled skills are examples-first, with Mesher as maintainer-facing deeper reference material — MET.**
  - Evidence: S04 updated `README.md`, VitePress pages, scaffold README generation in `compiler/mesh-pkg/src/scaffold.rs`, and `tools/skill/mesh/skills/clustering/SKILL.md`.
  - Proof: Historical contract rails (`verify-m050-s02`, `verify-m050-s03`, M047 docs rails, proof-page verifier) were reconciled in S04, and the S05 replay records `m051-s04-wrapper passed`.

- **The repo ships without repo-root `reference-backend/`, and one post-deletion acceptance rail composes Mesher runtime, retained backend proof, tooling/editor cutover, and docs-story proof — MET.**
  - Evidence: `reference-backend/` is absent from the current tree, `scripts/verify-production-proof-surface.sh` exists at the top level, `compiler/meshc/tests/e2e_m051_s05.rs` exists, and `scripts/verify-m051-s05.sh` now publishes a self-contained retained bundle.
  - Proof: direct closeout replay passed on a fresh disposable Postgres database: `cargo test -p meshc --test e2e_m051_s05 -- --nocapture` and `DATABASE_URL=postgresql://postgres:postgres@127.0.0.1:51798/mesh_m051_complete bash scripts/verify-m051-s05.sh`.

## Decision Re-evaluation

| Decision | Re-evaluation |
|---|---|
| D376 + D377 — Mesher should adopt the scaffold-style bootstrap and make `Node.start_from_env()` the only clustered bootstrap path | **Still valid.** S01 proved this is the right deeper-app contract and no later slice had to reintroduce app-owned cluster wiring. |
| D381 + D382 + D383 — Backend-only proof should move to an internal retained fixture with dedicated verifier ownership | **Still valid.** S02-S05 depended on this split; it is what allowed public docs retargeting, tooling cutover, and final deletion without losing backend-only proof. |
| D384 + D385 — Tooling/editor rails should use a bounded retained backend fixture while public editor docs stay generic | **Still valid.** S03 kept the rails deterministic and did not leak an internal fixture into first-contact docs. |
| D386 + D387 — Public deeper-backend handoff should stay behind `/docs/production-backend-proof/`, with Mesher and retained backend replay maintainer-facing | **Still valid.** S04 and S05 delivered this public/maintainer split cleanly. |
| D388 + D389 + D390 + D391 — Final post-deletion proof should use a top-level verifier path and a self-contained retained bundle, with retained backend recovery proved through crash/requeue correctness plus stable `/health` restart metadata | **Still valid.** The S05 replay passed on the post-deletion tree, and the copied retained bundle under `.tmp/m051-s05/verify/retained-proof-bundle/` is the correct downstream evidence surface. |

## Definition of Done Results

## Definition of Done Results

- **All roadmap slices complete — MET.** `M051-ROADMAP.md` shows S01-S05 checked complete.
- **All slice summaries exist — MET.** `find .gsd/milestones/M051/slices -maxdepth 2 -type f \( -name 'S*-SUMMARY.md' -o -name 'S*-UAT.md' \) | wc -l` returned `10`, covering summary + UAT for S01-S05.
- **Cross-slice integration works on the final tree — MET.** The final `bash scripts/verify-m051-s05.sh` replay passed and its phase report shows `m051-s01-wrapper passed`, `m051-s02-wrapper passed`, `m051-s03-wrapper passed`, `m051-s04-wrapper passed`, followed by all retained-bundle phases passing.
- **Milestone produced real non-`.gsd` code and docs changes — MET.** Because `HEAD == main == origin/main`, code-change verification used the equivalent boundary `34739a28^`; `git diff --stat 34739a28^ HEAD -- ':!.gsd/'` shows extensive non-`.gsd` changes across Mesher, retained backend fixtures, compiler/tooling tests, public docs, scaffold output, and verifier scripts.
- **Horizontal checklist — N/A.** The roadmap does not render a separate Horizontal Checklist section for M051.


## Requirement Outcomes

## Requirement Outcomes

- **R119 — `mesher` replaces `reference-backend` as the maintained deeper reference app and keeps working on current Mesh features**
  - **Status transition:** `active` -> `validated`
  - **Why the transition is supported:**
    - **S01** proved Mesher on the current bootstrap/runtime contract with a dedicated maintainer runbook and live Postgres-backed proof rail.
    - **S02** removed backend-only deploy/recovery/health proof from the public app path and preserved it as an internal retained fixture plus maintainer verifier.
    - **S03** moved tooling/editor/LSP/formatter rails off repo-root `reference-backend/` and onto the retained fixture, which was required before deletion could be honest.
    - **S04** rewrote public docs, scaffold copy, and bundled skills so public readers stay scaffold/examples-first while Mesher is named only as the deeper maintainer-facing app.
    - **S05** deleted the repo-root `reference-backend/` tree and passed the post-deletion assembled acceptance rail on the final tree.
  - **Validation evidence:** `cargo test -p meshc --test e2e_m051_s05 -- --nocapture`; `DATABASE_URL=postgresql://postgres:postgres@127.0.0.1:51798/mesh_m051_complete bash scripts/verify-m051-s05.sh`; current retained markers `.tmp/m051-s05/verify/status.txt = ok`, `.tmp/m051-s05/verify/current-phase.txt = complete`, and `.tmp/m051-s05/verify/latest-proof-bundle.txt -> .tmp/m051-s05/verify/retained-proof-bundle`.

- **R120-R123**
  - **Status transition:** none
  - **Reason:** They remain active and are owned by later milestones (`M052`/`M053`/`M054`), not by M051.


## Deviations

The literal step-3 diff against `main` was empty because local `HEAD`, `main`, and `origin/main` were already the same integration tip, so code-change verification used the parent of the earliest M051-tagged commit (`34739a28^`) as the equivalent baseline. The final DB-backed acceptance replay also used a disposable local Docker Postgres URL because this non-interactive shell had no inherited `DATABASE_URL`.

## Follow-ups

M052-M054 still own the remaining public-surface work: landing/packages coherence, deploy truth for scaffolds and packages, and an honest load-balancing story. None of those block M051 completion; they build on the post-deletion Mesher/retained-backend structure this milestone established.
