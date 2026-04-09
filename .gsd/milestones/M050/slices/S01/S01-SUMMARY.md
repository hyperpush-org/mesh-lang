---
id: S01
parent: M050
milestone: M050
provides:
  - A fail-closed source + built-site docs-graph contract that keeps proof pages public-secondary and out of the default footer chain.
  - Retargeted retained M047 and production-proof docs rails that now guard structural markers and public routing instead of proof-first paragraph text.
  - One env-free `bash scripts/verify-m050-s01.sh` preflight that heavier wrappers can call before scaffold/example/runtime replays.
requires:
  []
affects:
  - S02
  - S03
key_files:
  - website/docs/.vitepress/config.mts
  - website/docs/.vitepress/theme/composables/usePrevNext.ts
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/production-backend-proof/index.md
  - scripts/tests/verify-m050-s01-onboarding-graph.test.mjs
  - scripts/verify-m050-s01.sh
  - compiler/meshc/tests/e2e_m050_s01.rs
  - compiler/meshc/tests/e2e_m047_s04.rs
  - compiler/meshc/tests/e2e_m047_s06.rs
  - scripts/verify-m047-s04.sh
  - scripts/verify-m047-s06.sh
  - reference-backend/scripts/verify-production-proof-surface.sh
  - scripts/verify-m049-s05.sh
  - scripts/tests/verify-m049-s05-contract.test.mjs
  - compiler/meshc/tests/e2e_m049_s05.rs
  - .gsd/PROJECT.md
key_decisions:
  - Keep proof pages public but demoted under a dedicated `Proof Surfaces` sidebar group instead of leaving them in primary onboarding groups.
  - Use exact normalized page matching in the VitePress footer resolver and a sidebar-level `includeInFooter: false` opt-out to keep proof pages out of neighboring prev/next links.
  - Retarget historical M047 and production-proof docs rails to structural/path markers instead of exact proof-first paragraphs so later copy slices can rewrite prose without resurrecting stale wording.
  - Make `bash scripts/verify-m050-s01.sh` the fast env-free docs-graph preflight and run it first inside `bash scripts/verify-m049-s05.sh` rather than nesting a heavier wrapper.
patterns_established:
  - For VitePress proof pages, frontmatter `prev: false` / `next: false` is not enough by itself; the sidebar items also need `includeInFooter: false`, and the footer resolver must filter those items before exact page matching.
  - Retained docs contracts should pin navigation shape and public routing markers, not exact intro paragraphs, or later copy work will be forced to preserve stale proof-maze wording.
  - A fast docs-only verifier should own one `.tmp/<slice>/verify` tree with source checks, built-site assertions, copied HTML evidence, and phase markers before heavier scaffold/example wrappers consume it as a preflight.
observability_surfaces:
  - .tmp/m050-s01/verify/status.txt
  - .tmp/m050-s01/verify/current-phase.txt
  - .tmp/m050-s01/verify/phase-report.txt
  - .tmp/m050-s01/verify/full-contract.log
  - .tmp/m050-s01/verify/latest-proof-bundle.txt
  - .tmp/m050-s01/verify/built-html/summary.json
  - .tmp/m050-s01/verify/built-html/getting-started.index.html
  - .tmp/m050-s01/verify/built-html/clustered-example.index.html
  - .tmp/m050-s01/verify/built-html/distributed-proof.index.html
  - .tmp/m050-s01/verify/built-html/production-backend-proof.index.html
drill_down_paths:
  - .gsd/milestones/M050/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M050/slices/S01/tasks/T02-SUMMARY.md
  - .gsd/milestones/M050/slices/S01/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-04T01:18:15.252Z
blocker_discovered: false
---

# S01: Onboarding Graph & Retained Rail Reset

**Reset the public docs graph so Getting Started and Clustered Example are the primary onboarding path, proof pages stay public but secondary, and one fast env-free verifier now fail-closes navigation drift.**

## What Happened

S01 changed the docs structure first so later copy work does not fight a proof-first graph. In `website/docs/.vitepress/config.mts`, the `/docs/` sidebar now keeps `Getting Started` limited to `Introduction` and `Clustered Example`, moves `Distributed Proof` and `Production Backend Proof` into a final `Proof Surfaces` group, and marks those proof-page links with `includeInFooter: false` so they stay public without re-entering the default footer chain. In `website/docs/.vitepress/theme/composables/usePrevNext.ts`, the footer resolver now finds the current page by exact normalized path equality instead of prefix matching, which fixes the old `Clustered Example -> Clustered Example` self-link caused by resolving `/docs/getting-started/clustered-example/` through `/docs/getting-started/`. Both proof pages now also set `prev: false` and `next: false` in frontmatter so they opt out locally as well as structurally.

The slice then retargeted the active retained docs rails away from proof-maze intro paragraphs and onto the new M050 structural contract. `compiler/meshc/tests/e2e_m047_s04.rs`, `compiler/meshc/tests/e2e_m047_s06.rs`, `scripts/verify-m047-s04.sh`, `scripts/verify-m047-s06.sh`, and `reference-backend/scripts/verify-production-proof-surface.sh` now assert public-secondary proof discoverability, starter/readme routing markers, footer opt-out markers, and the shared onboarding-graph contract instead of pinning exact proof-first wording. That kept the root `README.md` bounded to public routing and proof-page discoverability, while explicit retained verifier-command maps stayed on `Distributed Proof`, `Production Backend Proof`, and the deeper docs surfaces that intentionally own those rails.

Finally, S01 added one slice-local fast verifier, `scripts/verify-m050-s01.sh`, plus the matching Rust contract in `compiler/meshc/tests/e2e_m050_s01.rs`. The shell rail is deliberately env-free: it runs the new Node docs-graph contract, the retargeted M047 docs rails, the production proof-surface verifier, one VitePress build, and a built-HTML assertion pass, then records `status.txt`, `current-phase.txt`, `phase-report.txt`, `full-contract.log`, `latest-proof-bundle.txt`, copied HTML snapshots, and `built-html/summary.json` under `.tmp/m050-s01/verify/`. The active M049 wrapper now calls this rail first: `scripts/verify-m049-s05.sh`, `scripts/tests/verify-m049-s05-contract.test.mjs`, and `compiler/meshc/tests/e2e_m049_s05.rs` were updated so the heavier scaffold/example replay always starts with the docs-graph preflight instead of rediscovering docs drift late.

## Verification

All slice-plan rails passed from the current tree:
- `node --test scripts/tests/verify-m050-s01-onboarding-graph.test.mjs`
- `cargo test -p meshc --test e2e_m047_s04 -- --nocapture`
- `cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture`
- `bash reference-backend/scripts/verify-production-proof-surface.sh`
- `cargo test -p meshc --test e2e_m050_s01 -- --nocapture`
- `node --test scripts/tests/verify-m049-s05-contract.test.mjs`
- `cargo test -p meshc --test e2e_m049_s05 -- --nocapture`
- `bash scripts/verify-m050-s01.sh`

The assembled S01 verifier produced `.tmp/m050-s01/verify/status.txt = ok`, `.tmp/m050-s01/verify/current-phase.txt = complete`, and a passed `phase-report.txt` covering `m050-s01-onboarding-graph`, the retained M047 rails, `production-proof-surface`, `docs-build`, `retain-built-html`, `built-html`, and `m050-s01-bundle-shape`. The retained built-site evidence in `.tmp/m050-s01/verify/built-html/summary.json` shows the intended footer flow exactly: `Getting Started -> Clustered Example`, `Clustered Example -> Getting Started + Language Basics`, and no footer links on either proof page.

## Requirements Advanced

- R117 — The public docs graph now keeps proof pages out of the primary onboarding rail and enforces that demotion with source-level and built-site fail-closed verification.
- R118 — Cluster guidance now routes readers through Getting Started and Clustered Example before any proof surface, while proof pages stay public-secondary and out of footer navigation.
- R120 — README/docs/proof-page routing now share one coherent public graph instead of competing first-contact routes, which is an early step toward one consistent evaluator story across public surfaces.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None.

## Known Limitations

This slice reset structure and retained contracts, not prose density. `Clustered Example`, `Distributed Proof`, `Distributed Actors`, and `Tooling` still carry retained verifier maps and historical rail detail that later M050 slices need to rewrite or rebalance without moving proof pages back into the primary onboarding graph. `bash scripts/verify-m050-s01.sh` is also intentionally scoped to graph/footer/docs-contract truth; it does not replace the heavier scaffold/example/runtime sample replays.

## Follow-ups

S02 should rewrite the first-contact copy on Getting Started, Clustered Example, and Tooling around the install -> hello-world -> clustered/sqlite/postgres choice while keeping the S01 graph and footer contract green. S03 should keep proof pages public-secondary, cleanly separate low-level distributed primitives from clustered-app guidance, and extend sample-verified wording without reviving proof pages as primary navigation.

## Files Created/Modified

- `website/docs/.vitepress/config.mts` — Reordered the `/docs/` sidebar, introduced the secondary `Proof Surfaces` group, and added sidebar-level footer opt-outs for proof pages.
- `website/docs/.vitepress/theme/composables/usePrevNext.ts` — Changed footer resolution to exact normalized page matching and filtered out `includeInFooter: false` links so `Clustered Example` no longer self-links.
- `website/docs/docs/distributed-proof/index.md` — Marked the proof page as footer-opted-out and kept it explicitly public-secondary in the rewritten docs graph.
- `website/docs/docs/production-backend-proof/index.md` — Marked the proof page as footer-opted-out and kept the production backend proof public-secondary instead of first-contact.
- `scripts/tests/verify-m050-s01-onboarding-graph.test.mjs` — Added the fail-closed Node contract for sidebar group order, exact footer matching, proof-page opt-out markers, and typo regressions.
- `scripts/verify-m050-s01.sh` — Added the env-free slice verifier that runs source contracts, retained docs rails, one docs build, built-HTML assertions, and retains one `.tmp/m050-s01/verify` bundle.
- `compiler/meshc/tests/e2e_m050_s01.rs` — Pinned the S01 verifier phases, expected artifact/bundle markers, and built-HTML contract strings in a Rust contract test.
- `compiler/meshc/tests/e2e_m047_s04.rs` — Retargeted the retained M047 S04 docs rail to the M050 onboarding graph markers and proof-page demotion contract.
- `compiler/meshc/tests/e2e_m047_s06.rs` — Retargeted the retained M047 S06 docs rail to the same structural graph contract and bounded proof-page discoverability markers.
- `scripts/verify-m047-s04.sh` — Updated the retained shell verifier to assert public-secondary proof surfaces and to call the shared M050 onboarding-graph contract.
- `scripts/verify-m047-s06.sh` — Updated the retained closeout shell verifier to guard the new graph, proof-page discoverability, and bounded public routing markers.
- `reference-backend/scripts/verify-production-proof-surface.sh` — Changed the production proof-surface verifier to keep the proof page public but secondary, ahead of no longer-first-contact clustered entrypoints.
- `scripts/verify-m049-s05.sh` — Wired the active M049 assembled wrapper to run `bash scripts/verify-m050-s01.sh` first as the docs-graph preflight.
- `scripts/tests/verify-m049-s05-contract.test.mjs` — Pinned the M049 wrapper to the new M050 preflight ordering and fail-closed preflight discoverability markers.
- `compiler/meshc/tests/e2e_m049_s05.rs` — Added Rust-side contract coverage for the new M050 preflight requirement inside the M049 assembled wrapper.
- `.gsd/PROJECT.md` — Refreshed the living project state to record the completed M050/S01 docs-graph reset and new fast verifier surface.
