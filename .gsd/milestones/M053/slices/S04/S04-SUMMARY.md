---
id: S04
parent: M053
milestone: M053
provides:
  - A truthful evaluator-facing docs contract that keeps SQLite explicitly local-only, presents the generated Postgres starter as the serious deployable path, and demotes Fly/`cluster-proof` to retained read-only/reference proof.
  - One slice-owned acceptance surface (`scripts/verify-m053-s04.sh` plus `scripts/tests/verify-m053-s04-contract.test.mjs`) that fails closed on public-doc drift across first-contact pages, distributed-proof pages, and retained Fly assets.
  - A retained docs/reference evidence bundle under `.tmp/m053-s04/verify/` with rendered HTML summaries, phase markers, log-path pointers, and a copied upstream `.tmp/m050-s02/verify/` bundle for downstream inspection.
requires:
  - slice: S02
    provides: The serious generated Postgres starter staged deploy + failover proof chain and retained `.tmp/m053-s02/**` evidence that the public docs now point at.
  - slice: S03
    provides: The hosted starter/packages/public-surface contract, reusable workflow graph, and `.tmp/m053-s03/verify/` evidence that S04 now describes as part of the same shipped story.
affects:
  []
key_files:
  - README.md
  - website/docs/docs/getting-started/index.md
  - website/docs/docs/getting-started/clustered-example/index.md
  - website/docs/docs/tooling/index.md
  - website/docs/docs/distributed/index.md
  - website/docs/docs/distributed-proof/index.md
  - scripts/fixtures/clustered/cluster-proof/README.md
  - scripts/verify-m043-s04-fly.sh
  - scripts/fixtures/clustered/cluster-proof/tests/work.test.mpl
  - compiler/meshc/tests/e2e_m047_s05.rs
  - scripts/tests/verify-m053-s04-contract.test.mjs
  - scripts/verify-m053-s04.sh
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - D410: Anchor Distributed Proof on the generated Postgres starter's M053 verifier chain and keep Fly/`cluster-proof` on a retained read-only/reference rail.
  - D411: Keep the scaffold/examples-first order explicit across first-contact docs and treat the retained M047 docs contract as part of the first-contact verification surface.
  - D412: Detect S04 docs/reference drift with exact source-line and rendered-text markers while keeping the wrapped retained docs rails aligned instead of weakening them.
patterns_established:
  - Keep first-contact docs scaffold/examples-first, then explicitly split the local SQLite starter from the serious shared/deployable Postgres starter instead of smuggling proof commands into README-grade pages.
  - For public-doc contract verification, pair exact source-line checks with rendered HTML text markers; substring-only greps are too weak and can false-positive on valid prose.
  - Treat retained Fly/reference fixtures as literal contract surfaces: their README/help wording is pinned by package tests, so meaning-preserving rewrites can still be regressions if exact markers drift.
  - When one docs verifier wraps older rails, retain the upstream `.tmp/.../verify/` bundle instead of summarizing it away; downstream slices need the delegated evidence, not just the wrapper's exit code.
observability_surfaces:
  - `.tmp/m053-s04/verify/status.txt`, `current-phase.txt`, and `phase-report.txt` provide the assembled verifier verdict plus the exact phase that failed or completed.
  - `.tmp/m053-s04/verify/log-paths.txt` and `full-contract.log` point to the wrapped docs-build, first-contact, proof-surface, fixture-test, and contract-test logs without forcing downstream slices to rediscover log locations.
  - `.tmp/m053-s04/verify/built-html/summary.json` plus the copied built HTML files retain the rendered-text evidence used to prove the public docs contract, including first-contact and distributed-proof markers.
  - `.tmp/m053-s04/verify/retained-m050-s02-verify/` preserves the upstream assembled first-contact bundle so later slices can inspect the exact delegated evidence instead of rerunning it blindly.
drill_down_paths:
  - .gsd/milestones/M053/slices/S04/tasks/T01-SUMMARY.md
  - .gsd/milestones/M053/slices/S04/tasks/T02-SUMMARY.md
  - .gsd/milestones/M053/slices/S04/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-05T22:13:22.422Z
blocker_discovered: false
---

# S04: Public docs and Fly reference assets match the shipped contract

**S04 made the public docs and retained Fly reference assets tell one truthful M053 story and backed that story with a fail-closed docs/reference verifier bundle.**

## What Happened

S04 closed the public-story gap that remained after S01-S03. The repo now presents one evaluator-facing clustered contract instead of three competing stories.

First-contact docs are back on a strict starter-first ladder. README, Getting Started, Clustered Example, and Tooling still lead with the generated scaffold/examples flow, but they now say the quiet parts out loud: SQLite is the honest local-only/single-node starter, the generated Postgres Todo starter is the serious shared/deployable path, and the staged deploy + failover + hosted packages/public-surface proof chain belongs to that generated Postgres starter once the reader steps onto the proof pages.

Public-secondary proof docs and retained Fly assets now tell the same story. `Distributed Proof` no longer reads like Fly or `cluster-proof` is a coequal public starter surface; instead it names the M053 starter-owned verifier chain directly (`scripts/verify-m053-s01.sh`, `scripts/verify-m053-s02.sh`, `scripts/verify-m053-s03.sh`), keeps SQLite outside clustered proof, and treats Fly/`cluster-proof` as retained read-only/reference evidence for already-deployed environments.

S04 also turned that prose contract into a mechanical acceptance surface. `scripts/tests/verify-m053-s04-contract.test.mjs` now checks the public-doc and retained-reference copy directly, including negative cases for Fly-first wording, proof-maze-first regressions, and duplicated/corrupted tails. `scripts/verify-m053-s04.sh` assembles the docs build, first-contact rail, proof-surface rail, retained fixture test rail, and the new Node contract into one fail-closed bundle under `.tmp/m053-s04/verify/`, with copied built HTML, retained upstream M050 bundle evidence, phase markers, and per-phase log pointers.

The practical outcome for downstream readers is simple: if they need the shipped public contract, they can trust the docs to say SQLite-local, Postgres-serious/deployable, packages/public-surface proof in the same hosted chain, and Fly-reference-only — and they can trust `.tmp/m053-s04/verify/` to fail loudly the next time those surfaces drift apart.

## Verification

Ran all slice-level verification rails and confirmed the slice-owned observability bundle works.

- `bash scripts/verify-m050-s02.sh` ✅ passed and wrote a green `.tmp/m050-s02/verify/` bundle (`status.txt=ok`, `current-phase.txt=complete`).
- `bash scripts/verify-production-proof-surface.sh && cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests` ✅ passed, proving the public proof pages and retained Fly/reference package still agree.
- `node --test scripts/tests/verify-m053-s04-contract.test.mjs` ✅ passed (`5/5` green).
- `bash scripts/verify-m053-s04.sh` ✅ passed and wrote a green `.tmp/m053-s04/verify/` bundle with `status.txt=ok`, `current-phase.txt=complete`, fully passed `phase-report.txt`, `log-paths.txt`, `built-html/summary.json`, copied rendered HTML, and a retained copy of `.tmp/m050-s02/verify/` under `.tmp/m053-s04/verify/retained-m050-s02-verify/`.

That combination verifies both the prose contract and the assembled fail-closed evidence surface for this slice.

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

No slice-scope plan change was needed, but the assembled docs rails exposed two transitive surfaces that had to be kept truthful: `compiler/meshc/tests/e2e_m047_s05.rs` still pinned older Tooling/Distributed Proof wording, and `website/docs/docs/tooling/index.md` had a duplicated/corrupted tail that only became visible once the new exact-line/full-rendered-text verifier was in place. S04 also updated `.gsd/PROJECT.md` and recorded D411/D412 plus matching knowledge notes so downstream slices do not have to rediscover those verifier boundaries.

## Known Limitations

The S04 docs/reference contract is green locally, but the upstream hosted-state limitation from S03 still exists: the latest green `authoritative-verification.yml` run on `main` predates the new starter-proof lane, and the current release tag still lacks the peeled ref required by the hosted freshness check. S04 documents that hosted chain truthfully, but it does not by itself make the live remote evidence green. Also, broader landing-site copy alignment remains intentionally out of scope for this slice.

## Follow-ups

1. Re-run the S03 hosted verifier after remote `main` and release-tag state catch up so milestone closeout can point at fresh green hosted evidence instead of the current fail-closed red state.
2. Keep any later landing-site or product-marketing copy alignment separate from S04 unless it also wires into the same public docs/reference contract and verifier surface.
3. If future wording changes touch first-contact or retained Fly/reference surfaces, update `compiler/meshc/tests/e2e_m047_s05.rs`, `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs`, and `scripts/tests/verify-m053-s04-contract.test.mjs` together rather than treating them as independent rails.

## Files Created/Modified

- `README.md` — Updated the repo-first starter chooser so README now keeps scaffold/examples-first order, names SQLite local-only, and hands deploy/failover/packages proof to the generated Postgres starter.
- `website/docs/docs/getting-started/index.md` — Reframed Getting Started around the explicit clustered scaffold -> SQLite local-only -> Postgres shared/deployable ladder and the proof-page handoff.
- `website/docs/docs/getting-started/clustered-example/index.md` — Updated the clustered example page so the public follow-on ladder stays starter-first and points deeper proof work to Distributed Proof instead of turning the page into a verifier maze.
- `website/docs/docs/tooling/index.md` — Aligned Tooling with the M053 starter split, fixed the corrupted tail, and kept first-contact CLI guidance explicit about SQLite-local vs Postgres-shared/deployable truth.
- `website/docs/docs/distributed/index.md` — Adjusted the public distributed guide so it now hands readers to the M053 starter-owned proof map and retained Fly reference lane.
- `website/docs/docs/distributed-proof/index.md` — Rewrote Distributed Proof around the `verify-m053-s01.sh` -> `verify-m053-s02.sh` -> `verify-m053-s03.sh` chain, the SQLite local-only boundary, and the retained Fly reference lane.
- `scripts/fixtures/clustered/cluster-proof/README.md` — Reframed the retained `cluster-proof` README as a bounded read-only/reference fixture rather than a coequal public starter surface.
- `scripts/verify-m043-s04-fly.sh` — Updated Fly verifier help text so it stays explicitly read-only/reference-only and does not widen Fly into the public starter contract.
- `scripts/fixtures/clustered/cluster-proof/tests/work.test.mpl` — Pinned the retained-reference wording in the `cluster-proof` fixture tests.
- `compiler/meshc/tests/e2e_m047_s05.rs` — Extended the retained M047 docs contract so the assembled first-contact rails stay aligned with the new M053 wording instead of silently drifting.
- `scripts/tests/verify-m053-s04-contract.test.mjs` — Added the slice-owned contract suite that fails closed on public-doc drift, Fly-first wording, corrupted tails, and retained-reference regressions.
- `scripts/verify-m053-s04.sh` — Added the assembled S04 verifier that builds docs, checks rendered HTML, replays upstream docs/proof rails, runs retained fixture tests, and retains one `.tmp/m053-s04/verify/` bundle.
- `.gsd/DECISIONS.md` — Recorded the S04 public-contract and verifier design decisions.
- `.gsd/KNOWLEDGE.md` — Recorded the exact-phrase and exact-line verifier gotchas that future docs work must preserve.
- `.gsd/PROJECT.md` — Updated current project state to note that M053 S04 is complete and that `.tmp/m053-s04/verify/` is now the public-docs/reference acceptance surface.
