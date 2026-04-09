---
id: S02
parent: M055
milestone: M055
provides:
  - A movable Mesher maintainer contract that no longer depends on repo-root `cargo run -q -p meshc -- ... mesher` folklore.
  - An authoritative deeper-app verifier and retained proof bundle under `.tmp/m051-s01/verify/` that future extraction work can preserve.
  - A public-secondary docs handoff that keeps product-owned Mesher operations separate from language-owned first-contact surfaces.
requires:
  - slice: S01
    provides: The blessed sibling-workspace contract, repo-identity source, split language-vs-product ownership rules, and the authoritative `bash scripts/verify-m055-s01.sh` replay used to keep the new Mesher handoff aligned with repo-local `.gsd` authority.
affects:
  - S03
  - S04
key_files:
  - mesher/scripts/lib/mesh-toolchain.sh
  - mesher/scripts/test.sh
  - mesher/scripts/migrate.sh
  - mesher/scripts/build.sh
  - mesher/scripts/smoke.sh
  - mesher/scripts/verify-maintainer-surface.sh
  - compiler/meshc/tests/support/m051_mesher.rs
  - compiler/meshc/tests/e2e_m051_s01.rs
  - mesher/README.md
  - website/docs/docs/production-backend-proof/index.md
  - scripts/tests/verify-m055-s02-contract.test.mjs
  - scripts/verify-m051-s01.sh
  - scripts/verify-production-proof-surface.sh
  - .gsd/PROJECT.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Mesher’s primary deeper-app maintainer contract now lives under `mesher/scripts/` and `mesher/README.md`; `bash scripts/verify-m051-s01.sh` remains only as a compatibility wrapper.
  - The Mesher toolchain resolver is strict and source-first: enclosing `mesh-lang` checkout, then blessed sibling `../mesh-lang`, then installed `PATH`, with fail-closed behavior if a detected higher-priority source tier is broken.
  - `production-backend-proof` stays language-owned and public-secondary, but it now hands deeper verification to the product-owned Mesher runbook/verifier instead of teaching repo-root Mesher commands.
patterns_established:
  - Package-owned maintainer surfaces own deep-app build/test/migrate/smoke/verifier flows; the language repo keeps only compatibility wrappers and public-secondary handoffs.
  - Toolchain resolution must be explicit and fail closed when the expected source checkout is present-but-broken; silent fallback hides split-contract drift.
  - Retained verifier bundles should publish `status.txt`, `current-phase.txt`, `phase-report.txt`, and `latest-proof-bundle.txt` as the first diagnostic surface.
  - Docs/runbook contract drift is best pinned with exact-marker Node/shell rails rather than prose review.
observability_surfaces:
  - `.tmp/m051-s01/verify/status.txt` + `current-phase.txt`
  - `.tmp/m051-s01/verify/phase-report.txt`
  - `.tmp/m051-s01/verify/full-contract.log`
  - `.tmp/m051-s01/verify/runtime-smoke/`
  - `.tmp/m051-s01/verify/package-root.meta.json`
  - `.tmp/m051-s01/verify/latest-proof-bundle.txt`
drill_down_paths:
  - .gsd/milestones/M055/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M055/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M055/slices/S02/tasks/T03-SUMMARY.md
  - .gsd/milestones/M055/slices/S02/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-06T20:16:58.304Z
blocker_discovered: false
---

# S02: Hyperpush Toolchain Contract Outside `mesh-lang`

**Mesher now owns its deeper-app toolchain, runbook, and verifier contract through package-local scripts that work from the blessed sibling workspace, while `mesh-lang` keeps only compatibility and public-secondary handoff surfaces.**

## What Happened

S02 made Mesher operationally believable outside the current repo-root special case without extracting it yet. T01 introduced a package-owned toolchain layer under `mesher/scripts/` with one shared resolver (`mesher/scripts/lib/mesh-toolchain.sh`) that chooses `meshc` in explicit source-first order: enclosing `mesh-lang` checkout, blessed sibling `../mesh-lang`, then installed `PATH`. The wrappers now run from the package root and stage build output into explicit bundle directories outside `mesher/` instead of writing `mesher/mesher` or `mesher/output` back into tracked source.

T02 moved the authoritative deeper-app proof rail into `bash mesher/scripts/verify-maintainer-surface.sh`. That verifier now owns the retained `.tmp/m051-s01/verify/` surface, including phase markers, package-root metadata, Postgres bootstrap, migrate/build/test/smoke phases, and a retained proof bundle pointer. The Rust helper and `e2e_m051_s01` rail were refit to drive the package-owned scripts rather than repo-root `meshc build mesher` / `meshc migrate mesher` assumptions. `bash scripts/verify-m051-s01.sh` now survives only as a compatibility wrapper that delegates and validates the delegated markers.

T03 rewrote `mesher/README.md` and `.env.example` around the package-local maintainer loop. The runbook now starts from the package root, teaches the explicit toolchain resolution order, and names `bash mesher/scripts/verify-maintainer-surface.sh` as the primary deeper-app proof command while documenting `bash scripts/verify-m051-s01.sh` as compatibility-only. The slice-owned Node contract test was extended so this wording and command shape fail closed if repo-root Mesher commands creep back in.

T04 kept `website/docs/docs/production-backend-proof/index.md` language-owned and public-secondary, but changed its deeper Mesher handoff to the product-owned runbook/verifier contract. `scripts/verify-production-proof-surface.sh` now pins that handoff and compatibility wording, and the split-boundary replay still passes so the S01 ownership model and the new Mesher contract stay aligned.

The net effect is a movable product-owned maintainer surface that can survive the future `hyperpush-mono` extraction: Mesher’s real build/test/migrate/smoke/proof path no longer depends on hidden repo-root `cargo run -q -p meshc -- ... mesher` folklore, and the remaining mesh-lang-side references have been narrowed to compatibility and public-secondary guidance.

## Verification

Replayed every slice-plan verification rail after the task work landed:

- `node --test scripts/tests/verify-m055-s02-contract.test.mjs`
- `bash mesher/scripts/test.sh`
- `bash mesher/scripts/build.sh .tmp/m055-s02/mesher-build`
- `cargo test -p meshc --test e2e_m051_s01 -- --nocapture`
- `bash mesher/scripts/verify-maintainer-surface.sh`
- `bash scripts/verify-m051-s01.sh`
- `bash scripts/verify-production-proof-surface.sh`
- `bash scripts/verify-m055-s01.sh`

All passed. The retained Mesher verifier surfaces are live and coherent: `.tmp/m051-s01/verify/status.txt` is `ok`, `current-phase.txt` is `complete`, `phase-report.txt` records the package-test/build/Postgres/migrate/runtime-smoke/bundle-shape phases as passed, and `latest-proof-bundle.txt` points at `.tmp/m051-s01/verify/retained-proof-bundle`. That gives future agents one authoritative first-stop debug bundle for this contract.

## Requirements Advanced

- R119 — Strengthened the maintained deeper-reference-app contract by moving Mesher’s real maintainer loop onto package-owned build/test/migrate/smoke/verifier surfaces that can survive extraction without hidden repo-root coupling.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The slice stayed inside plan scope, but two pragmatic adjustments were required while proving the contract: stale in-package Mesher build outputs were removed once staged builds became authoritative, and `mesher/scripts/smoke.sh` needed a real fix for piped JSON parsing plus unique verifier cluster ports so the new package-owned rail would fail only on Mesher drift instead of local process collisions.

## Known Limitations

Mesher still physically lives inside `mesh-lang`; this slice only made the contract movable and explicit ahead of extraction. The package-owned scripts rely on one of three toolchain tiers being present (enclosing source checkout, blessed sibling `../mesh-lang`, or installed `PATH` `meshc`) and intentionally fail closed instead of trying to manage that workspace for the user. Public docs still keep Mesher as the deeper maintainer path rather than a first-contact evaluator path; that broader repo split and final public-surface cleanup remains for later slices.

## Follow-ups

S03 should keep consolidating `mesh-lang` public surfaces so evaluator-facing docs, examples, and packages guidance do not depend on product-repo source paths. S04 should extract `mesher/` into `hyperpush-mono` while preserving the new package-owned script/verifier paths, the `.tmp/m051-s01/verify/` retained bundle shape, and the mesh-lang compatibility wrapper/public-secondary handoff story without reintroducing repo-root Mesher commands.

## Files Created/Modified

- `mesher/scripts/lib/mesh-toolchain.sh` — Added the explicit source-first `meshc` resolver, timeout helpers, and outside-package bundle-path guards for the Mesher maintainer contract.
- `mesher/scripts/test.sh` — Added the package-root `meshc test tests` wrapper for Mesher.
- `mesher/scripts/migrate.sh` — Added the package-root `meshc migrate . status|up` wrapper with fail-closed subcommand handling.
- `mesher/scripts/build.sh` — Added staged Mesher build output into explicit bundle directories outside `mesher/`.
- `mesher/scripts/smoke.sh` — Kept the package-local runtime smoke surface truthful and fixed piped JSON parsing plus verifier-specific cluster-port isolation.
- `mesher/scripts/verify-maintainer-surface.sh` — Added the authoritative product-owned Mesher replay with phase markers, Postgres bootstrap, retained proof bundle publication, and package-local command phases.
- `compiler/meshc/tests/support/m051_mesher.rs` — Refit the Rust support harness to drive the package-owned Mesher scripts and record package-root metadata instead of repo-root Mesher assumptions.
- `compiler/meshc/tests/e2e_m051_s01.rs` — Updated the Mesher e2e rail to assert the package-owned verifier/wrapper contract and runtime truth against the new maintainer surface.
- `mesher/README.md` — Rewrote the Mesher maintainer runbook around package-local commands, explicit toolchain resolution, and the primary/compatibility verifier split.
- `website/docs/docs/production-backend-proof/index.md` — Changed the public-secondary deeper-app handoff to Mesher-owned runbook/verifier surfaces while keeping mesh-lang ownership of the page itself.
- `scripts/tests/verify-m055-s02-contract.test.mjs` — Pinned the new Mesher script/runbook contract and forbade repo-root Mesher command drift.
- `scripts/verify-m051-s01.sh` — Reduced the old repo-root M051 rail to a compatibility wrapper that delegates and validates the product-owned verifier markers.
- `scripts/verify-production-proof-surface.sh` — Pinned the updated public-secondary Mesher handoff and tightened exact-marker checks with bullet-safe ripgrep usage.
- `.gsd/PROJECT.md` — Refreshed the project’s current-state description so M055/S02 is represented in the active split-contract story.
- `.gsd/KNOWLEDGE.md` — Captured the fail-closed Mesher toolchain resolver/staging contract and the exact-marker verifier `rg -Fq --` gotcha for future agents.
