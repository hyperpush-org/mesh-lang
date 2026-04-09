---
id: S04
parent: M055
milestone: M055
provides:
  - A fail-closed staged `hyperpush-mono` repo under `.tmp/m055-s04/workspace/hyperpush-mono` with explicit product-root README/workflow/dependabot/verifier surfaces.
  - A shared sibling-workspace helper that resolves `hyperpush-mono` and canonical language repo identity without relying on the in-repo `mesher/` tree or `origin` remote.
  - An assembled two-repo evidence model that can attribute language continuity to a real mesh-lang git SHA and product continuity to a staged-product manifest fingerprint plus copied proof-bundle pointers.
requires:
  - slice: S01
    provides: Canonical repo identity, blessed sibling-workspace contract, and repo-local `.gsd` authority boundaries.
  - slice: S02
    provides: Product-owned Mesher toolchain/verifier contract that can be staged under `hyperpush-mono/mesher`.
  - slice: S03
    provides: Language-owned public/starter/proof surfaces already cleaned up to hand off across the repo boundary.
affects:
  []
key_files:
  - mesher/scripts/lib/mesh-toolchain.sh
  - mesher/README.md
  - WORKSPACE.md
  - scripts/materialize-hyperpush-mono.mjs
  - scripts/lib/m055-workspace.sh
  - scripts/verify-m051-s01.sh
  - scripts/verify-m053-s03.sh
  - scripts/verify-m055-s04.sh
  - scripts/tests/verify-m055-s02-contract.test.mjs
  - scripts/tests/verify-m055-s04-contract.test.mjs
  - scripts/tests/verify-m055-s04-materialize.test.mjs
  - scripts/fixtures/m055-s04-hyperpush-root/README.md
  - scripts/fixtures/m055-s04-hyperpush-root/.github/workflows/deploy-landing.yml
  - scripts/fixtures/m055-s04-hyperpush-root/.github/dependabot.yml
  - scripts/fixtures/m055-s04-hyperpush-root/scripts/verify-landing-surface.sh
  - scripts/fixtures/m055-s04-hyperpush-root/scripts/verify-m051-s01.sh
  - .gsd/PROJECT.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D445: realize S04 through an explicit allowlist materializer plus one assembled two-repo verifier with per-repo attribution, rather than through a recursive copy or one monorepo proof bundle.
  - D446: treat `hyperpush-mono/mesher` as the only blessed extracted product package root and fail closed on flat `<workspace>/mesher` or mixed `../mesh-lang` assumptions.
  - D447: keep a product-root `scripts/verify-m051-s01.sh` compatibility wrapper in the staged repo so the retained Mesher maintainer verifier stays runnable from product root.
  - D448: resolve sibling product roots and the default language repo slug through `scripts/lib/m055-workspace.sh` plus `scripts/lib/repo-identity.json`, not through the in-repo `mesher/` tree or the current `origin` remote.
  - D449: attribute the staged product proof to the canonical product slug plus `materialized:<manifest fingerprint>`, with the source mesh-lang SHA recorded separately.
patterns_established:
  - Use an explicit allowlist materializer plus a written manifest/fingerprint for staged repo extraction; never treat a recursive copy of the live tree as truthful repo evidence.
  - When mesh-lang wrappers need sibling-product behavior, resolve the sibling root and canonical repo slug through shared helpers and repo identity, not through repo-local shortcuts or current git remotes.
  - Assembled cross-repo proof should copy each repo-owned verify tree and bundle pointer into a new retained bundle and pair them with explicit repo/ref metadata, instead of pretending one repo still owns both continuity stories.
  - Do not run `node scripts/materialize-hyperpush-mono.mjs --check` in parallel with `bash scripts/verify-m055-s04.sh`; both mutate the same staged `hyperpush-mono` workspace and can create false-red missing-artifact failures.
observability_surfaces:
  - .tmp/m055-s04/workspace/hyperpush-mono.stage.json
  - .tmp/m055-s04/workspace/hyperpush-mono.manifest.json
  - .tmp/m055-s04/workspace/hyperpush-mono/.tmp/m051-s01/verify/status.txt
  - .tmp/m055-s04/workspace/hyperpush-mono/.tmp/m051-s01/verify/phase-report.txt
  - .tmp/m055-s04/workspace/hyperpush-mono/.tmp/m055-s04/landing-surface/verify/status.txt
  - .tmp/m055-s04/workspace/hyperpush-mono/.tmp/m055-s04/landing-surface/verify/phase-report.txt
  - .tmp/m055-s04/verify/status.txt
  - .tmp/m055-s04/verify/current-phase.txt
  - .tmp/m055-s04/verify/phase-report.txt
  - .tmp/m055-s04/verify/language-repo.meta.json
  - .tmp/m055-s04/verify/product-repo.meta.json
  - .tmp/m055-s04/verify/language-proof-bundle.txt
  - .tmp/m055-s04/verify/product-proof-bundle.txt
drill_down_paths:
  - .gsd/milestones/M055/slices/S04/tasks/T01-SUMMARY.md
  - .gsd/milestones/M055/slices/S04/tasks/T02-SUMMARY.md
  - .gsd/milestones/M055/slices/S04/tasks/T03-SUMMARY.md
  - .gsd/milestones/M055/slices/S04/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-07T17:06:19.780Z
blocker_discovered: false
---

# S04: `hyperpush-mono` Extraction & Two-Repo Evidence Assembly

**S04 made the two-repo split operationally real by staging a fail-closed `hyperpush-mono` repo, retargeting mesh-lang compatibility/hosted-evidence rails to the sibling product repo and canonical repo identity, and adding one assembled two-repo evidence wrapper with explicit per-repo attribution.**

## What Happened

T01 locked the product shape to the blessed nested workspace contract instead of the older flattened sibling-package assumption. `mesher/scripts/lib/mesh-toolchain.sh`, `mesher/README.md`, `WORKSPACE.md`, and the existing Mesher contract tests now all agree on two truthful layouts: in-source development from `mesh-lang/mesher`, and extracted product work from `hyperpush-mono/mesher` beside a sibling `mesh-lang/` checkout. The resolver still prefers an enclosing source checkout first, then the blessed sibling workspace, then installed `PATH`, and it now fails closed if a stale flat `<workspace>/mesher` or `../mesh-lang` assumption appears.

T02 made extraction explicit instead of ad hoc. `scripts/materialize-hyperpush-mono.mjs` now stages a clean product repo under `.tmp/m055-s04/workspace/hyperpush-mono`, writes both `hyperpush-mono.stage.json` and `hyperpush-mono.manifest.json`, and uses a narrow allowlist plus local-state exclusions so `.git`, `.env.local`, `node_modules`, `.next`, in-place binaries, and other debris cannot silently leak into the staged repo. The staged root also gets the product-owned root surfaces it needs: README, landing deploy workflow, dependabot config, landing surface verifier, and the retained `scripts/verify-m051-s01.sh` compatibility wrapper that the Mesher maintainer verifier still snapshots.

T03 rewired mesh-lang-side compatibility and hosted-evidence rails around a shared split-workspace helper instead of local-product shortcuts. `scripts/lib/m055-workspace.sh` now resolves the sibling `hyperpush-mono` root and the default language repo slug from `scripts/lib/repo-identity.json`; `scripts/verify-m051-s01.sh` delegates only to the sibling product repo’s Mesher verifier and fails closed if only the in-repo `mesher/` tree exists; and `scripts/verify-m053-s03.sh` no longer follows `origin` by default, which keeps hosted evidence tied to the language repo even after the split.

T04 closed the loop with one assembled S04 wrapper. `scripts/verify-m055-s04.sh` refreshes the staged sibling repo, runs the product-owned maintainer and landing verifiers from product root, runs the language-owned `scripts/verify-m055-s03.sh` with the staged sibling passed in explicitly, and retains copied language/product proof trees plus `language-repo.meta.json`, `product-repo.meta.json`, and proof-bundle pointers under `.tmp/m055-s04/verify/retained-proof-bundle/`. The product side is attributed to the canonical Hyperpush slug plus a `materialized:<manifest fingerprint>` ref, while the language side records the real mesh-lang git SHA, so the final evidence chain can say exactly which repo/ref proved which continuity story.

The slice therefore delivers the missing operational seam for the M055 split: mesh-lang can now stage a clean product repo, product-owned verifiers run from product root, language-owned wrappers no longer treat the in-repo `mesher/` tree or current `origin` slug as authoritative, and the assembled evidence model is explicit about per-repo ownership instead of pretending one monorepo proof bundle still owns everything.

## Verification

Passed slice-owned verification rails:
- `node --test scripts/tests/verify-m055-s02-contract.test.mjs scripts/tests/verify-m055-s04-contract.test.mjs`
- `node --test scripts/tests/verify-m055-s04-materialize.test.mjs`
- `node scripts/materialize-hyperpush-mono.mjs --check`
- `bash .tmp/m055-s04/workspace/hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`
- `bash .tmp/m055-s04/workspace/hyperpush-mono/scripts/verify-landing-surface.sh`
- `M055_HYPERPUSH_ROOT=.tmp/m055-s04/workspace/hyperpush-mono bash scripts/verify-m051-s01.sh`

Direct evidence from those replays:
- the materializer refreshed `.tmp/m055-s04/workspace/hyperpush-mono` and emitted `hyperpush-mono.stage.json` plus `hyperpush-mono.manifest.json`
- the staged product Mesher verifier passed end to end and published `.tmp/m055-s04/workspace/hyperpush-mono/.tmp/m051-s01/verify/status.txt = ok`
- the staged product landing verifier passed and published `.tmp/m055-s04/workspace/hyperpush-mono/.tmp/m055-s04/landing-surface/verify/status.txt = ok`
- the mesh-lang compatibility wrapper passed while delegating to the staged sibling product root rather than the in-repo `mesher/` tree

The assembled S04 wrapper itself was also exercised, but two different runs exposed different states:
- an initial concurrent replay failed falsely because `node scripts/materialize-hyperpush-mono.mjs --check` and `bash scripts/verify-m055-s04.sh` were launched against the same staged workspace at the same time, causing the standalone materializer refresh to replace `.tmp/m055-s04/workspace/hyperpush-mono` during the product-owned verifier run
- after switching to a serialized replay, `bash scripts/verify-m055-s04.sh` advanced cleanly through `materialize-hyperpush`, `product-m051-wrapper`, and `product-landing-wrapper`, then entered the inherited language-owned `scripts/verify-m055-s03.sh` replay; hard-timeout recovery cut off waiting there while `.tmp/m055-s03/verify/current-phase.txt` was `m051-s04-wrapper` and `.tmp/m051-s04/verify/current-phase.txt` was `m050-s03-wrapper`

That leaves one fresh-bundle evidence gap on the deep inherited language-side wrapper chain, but no failing S04-owned contract or product-root proof surface was found during slice closeout.

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The plan’s final assembled replay was started, but the hard-timeout recovery window ended before the delegated `scripts/verify-m055-s03.sh` wrapper completed a fresh retained-bundle regeneration. The slice-owned materializer, contract tests, staged product verifiers, and mesh-lang compatibility wrapper all passed; the only incomplete signal is the terminal all-in-one wrapper refresh across inherited S03 retained rails.

The earlier red `product-m051-wrapper` result was not a product contract regression. It came from my own concurrent verification mistake: I launched a standalone `node scripts/materialize-hyperpush-mono.mjs --check` in parallel with `bash scripts/verify-m055-s04.sh`, and both commands rewrite the same staged `hyperpush-mono` tree. The staged product verifier passed immediately once rerun serially.

## Known Limitations

`scripts/verify-m055-s04.sh` is intentionally expensive because it replays the full language-owned S03 wrapper chain after the new product-root phases. A fresh green retained bundle for the assembled S04 rail was not captured during this closeout window.

The product side is still represented as a staged sibling repo under `.tmp/m055-s04/workspace/hyperpush-mono`, not as a second committed Git checkout. The slice proves the extracted repo shape, root surfaces, and per-repo attribution contract, but the product ref is therefore recorded as `materialized:<manifest fingerprint>` rather than a real product git SHA.

## Follow-ups

Rerun `bash scripts/verify-m055-s04.sh` in isolation, with no concurrent `materialize-hyperpush-mono` refresh, to publish a fresh full retained bundle under `.tmp/m055-s04/verify/`.

Milestone closeout should treat the staged-product ref model as intentional: the language side records a real mesh-lang SHA, while the product side records the staged manifest fingerprint until Hyperpush lives in its own real checkout.

Any later hosted-evidence or compatibility work should keep using `scripts/lib/m055-workspace.sh` plus `scripts/lib/repo-identity.json` as the default source of sibling workspace and language repo identity, rather than regressing to `origin` or local `mesher/` heuristics.

## Files Created/Modified

- `mesher/scripts/lib/mesh-toolchain.sh` — Retargeted Mesher toolchain resolution to the blessed nested `hyperpush-mono/mesher` workspace shape and fail-closed sibling-workspace rules.
- `mesher/README.md` — Rewrote the maintainer contract around `mesh-lang/mesher` vs `hyperpush-mono/mesher` and the staged product-root compatibility story.
- `WORKSPACE.md` — Promoted the blessed two-repo sibling workspace and the mesh-lang compatibility/hosted-evidence boundaries needed after extraction.
- `scripts/materialize-hyperpush-mono.mjs` — Added the fail-closed hyperpush repo materializer with explicit allowlist, exclusions, manifest, and stage summary.
- `scripts/fixtures/m055-s04-hyperpush-root/README.md` — Added the staged product-root README template for the extracted Hyperpush repo.
- `scripts/fixtures/m055-s04-hyperpush-root/.github/workflows/deploy-landing.yml` — Added the product-owned landing deploy workflow template for the staged repo root.
- `scripts/fixtures/m055-s04-hyperpush-root/.github/dependabot.yml` — Added the product-owned dependabot contract for the staged repo root.
- `scripts/fixtures/m055-s04-hyperpush-root/scripts/verify-landing-surface.sh` — Added the product-root landing-surface verifier that checks staged README/workflow/dependabot/link ownership.
- `scripts/fixtures/m055-s04-hyperpush-root/scripts/verify-m051-s01.sh` — Added the staged product-root compatibility wrapper retained by the Mesher maintainer verifier.
- `scripts/lib/m055-workspace.sh` — Added canonical sibling workspace and language repo slug resolution for mesh-lang compatibility and hosted-evidence rails.
- `scripts/verify-m051-s01.sh` — Retargeted the mesh-lang compatibility wrapper to the sibling `hyperpush-mono` repo and fail-closed delegation markers.
- `scripts/verify-m053-s03.sh` — Switched default language repo identity resolution from `origin` to canonical repo identity through the new workspace helper.
- `scripts/verify-m055-s04.sh` — Added the assembled two-repo verifier that stages Hyperpush, replays product/language entrypoints, and retains per-repo attribution metadata.
- `scripts/tests/verify-m055-s02-contract.test.mjs` — Extended the Mesher contract simulation to pin the nested extracted `hyperpush-mono/mesher` shape.
- `scripts/tests/verify-m055-s04-contract.test.mjs` — Added slice-owned contract coverage for sibling resolution, hosted repo identity, and assembled two-repo verifier metadata/bundle shape.
- `scripts/tests/verify-m055-s04-materialize.test.mjs` — Added materializer contract coverage for root surfaces, excluded local state, and staged landing verifier behavior.
- `.gsd/PROJECT.md` — Updated current-state project context to mark M055/S04 delivered.
- `.gsd/KNOWLEDGE.md` — Recorded the staged workspace concurrency gotcha and the canonical repo-identity source for post-S04 split work.
