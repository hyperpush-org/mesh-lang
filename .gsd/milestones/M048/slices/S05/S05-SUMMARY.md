---
id: S05
parent: M048
milestone: M048
provides:
  - A fail-fast public docs contract test for the three minimal first-contact surfaces (`README.md`, tooling docs, and the VS Code README).
  - An authoritative assembled closeout entrypoint at `bash scripts/verify-m048-s05.sh` with named phases and retained `.tmp/m048-s05/verify` bookkeeping.
  - A stable retained proof bundle that combines the fixed M036 editor artifacts with fresh M048 entrypoint/update artifacts behind one pointer for milestone validation.
  - Minimal public touchpoints that now truthfully teach installer-backed updates, default-plus-override entrypoints, bounded editor proof, grammar parity, and the assembled verifier.
requires:
  - slice: S02
    provides: Manifest-first override-entry editor/package proof, truthful Neovim and VS Code host coverage, and recursive publish archive behavior for non-root executable entries.
  - slice: S03
    provides: Installer-backed `meshc update` / `meshpkg update` commands plus the retained staged-release acceptance rail and updater artifacts.
  - slice: S04
    provides: Shared `@cluster` / interpolation syntax parity rails and the clustering-aware init-time Mesh skill contract.
affects:
  []
key_files:
  - README.md
  - website/docs/docs/tooling/index.md
  - tools/editors/vscode-mesh/README.md
  - scripts/tests/verify-m048-s05-contract.test.mjs
  - scripts/verify-m048-s05.sh
  - .gsd/PROJECT.md
  - .gsd/KNOWLEDGE.md
  - .gsd/DECISIONS.md
key_decisions:
  - D328: Treat R112 as validated by the assembled S05 replay and retained proof bundle.
  - D329: Keep the public VS Code README bounded to same-file definition plus manifest-first hover/diagnostics proof; omit cross-file definition claims.
  - D330: Replay the truthful S02 editor rails directly with `NEOVIM_BIN="${NEOVIM_BIN:-nvim}"` handling and cargo-before-VS-Code ordering instead of delegating to `scripts/verify-m036-s03.sh`.
  - D331: Copy fixed `.tmp/m036-s02` / `.tmp/m036-s03` artifacts directly, snapshot-copy fresh timestamped `.tmp/m048-s01/*` and `.tmp/m048-s03/*` buckets, and expose a stable `latest-proof-bundle.txt` pointer.
patterns_established:
  - Use a small exact-string docs contract test as phase 1 of assembly slices so public-touchpoint drift fails before expensive retained replays.
  - Keep evaluator-facing editor documentation bounded to the surface the repo actually proves; do not claim cross-file definition support when the live transport proof is still same-file-local.
  - For assembled closeout wrappers, replay retained subrails directly when older historical wrappers assume stale tool paths or hide ordering dependencies.
  - Retain fixed artifact directories via direct copy and fresh timestamped evidence via snapshot-copy behind one stable `latest-proof-bundle.txt` pointer.
observability_surfaces:
  - `bash scripts/verify-m048-s05.sh` as the authoritative assembled closeout entrypoint.
  - `.tmp/m048-s05/verify/status.txt` and `.tmp/m048-s05/verify/current-phase.txt` as the top-level green/red state markers.
  - `.tmp/m048-s05/verify/phase-report.txt` as the per-phase replay ledger for docs, entrypoint, editor, publish, update, grammar, skill, docs-build, and artifact-retention phases.
  - `.tmp/m048-s05/verify/latest-proof-bundle.txt` as the stable pointer to the retained assembled artifact bundle.
  - `.tmp/m048-s05/verify/retained-proof-bundle/` containing `retained-m036-s02-lsp`, `retained-m036-s02-syntax`, `retained-m036-s03-vscode-smoke`, `retained-m048-s01-artifacts`, and `retained-m048-s03-artifacts` for downstream diagnosis.
drill_down_paths:
  - .gsd/milestones/M048/slices/S05/tasks/T01-SUMMARY.md
  - .gsd/milestones/M048/slices/S05/tasks/T02-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-02T19:09:12.162Z
blocker_discovered: false
---

# S05: Assembled contract proof and minimal public touchpoints

**S05 closed M048 by adding a fail-fast public docs contract, a retained assembled verifier, and minimal first-contact doc updates that now match the shipped entrypoint, self-update, grammar, and skill contract.**

## What Happened

S05 closed M048's remaining truth gap without adding new product behavior. T01 rewrote the three minimal public touchpoints so they finally match the shipped contract: the root README now teaches installer-backed `meshc update` / `meshpkg update`, keeps `main.mpl` as the default executable, documents optional `[package].entrypoint = "lib/start.mpl"`, and points readers at `bash scripts/verify-m048-s05.sh`; the tooling docs now include a canonical update section, an override-entry `mesh.toml` example, a truthful `meshpkg publish` note about preserving nested project-root-relative `.mpl` paths while excluding hidden/test-only files, a manifest-first editor/grammar note, and the assembled verifier entrypoint; and the VS Code README was narrowed to the surface the repo actually proves today by removing the stale cross-file definition claim and documenting same-file definition plus manifest-first override-entry hover/diagnostics proof alongside `@cluster` / `@cluster(N)` and both interpolation forms.

T01 also added `scripts/tests/verify-m048-s05-contract.test.mjs` as a fail-fast public-truth rail. Instead of letting wording drift silently, S05 now uses exact include/exclude markers so the README, tooling docs, and VS Code README must keep the update, entrypoint, publish, grammar, and verifier markers while continuing to omit the stale `jump to definitions across files` claim.

T02 added `scripts/verify-m048-s05.sh` as the retained closeout wrapper for the whole milestone. The script owns `.tmp/m048-s05/verify`, runs the docs-contract phase first, then replays the retained S01 entrypoint rail, the truthful S02 Neovim/LSP and VS Code smoke rails, the S02 publish rail, the S03 toolchain-update rails, the S04 shared grammar / Neovim contract / skill rails, and the website docs build in a named-phase sequence. It keeps the watchouts explicit by using `NEOVIM_BIN="${NEOVIM_BIN:-nvim}"` instead of the older vendor-path assumption, running a cargo phase before VS Code smoke so `target/debug/meshc` exists, and failing closed when expected scripts, package scripts, or retained artifacts are missing.

The wrapper also gives downstream closeout work one stable diagnosis seam. It copies fixed `.tmp/m036-s02` and `.tmp/m036-s03` directories directly, snapshot-copies fresh timestamped `.tmp/m048-s01/*` and `.tmp/m048-s03/*` buckets, validates the retained bundle shape, and writes `.tmp/m048-s05/verify/latest-proof-bundle.txt` pointing at `.tmp/m048-s05/verify/retained-proof-bundle`. After S05, M048 no longer requires readers to reconstruct the milestone from separate slice rails: one verifier and one retained bundle now prove the override-entry project, self-update commands, grammar parity, refreshed skill contract, and minimal public touchpoints together.

## Verification

I reran every slice-level verification command from the S05 plan and confirmed the new diagnostics surfaces as the closer.

- `node --test scripts/tests/verify-m048-s05-contract.test.mjs` ✅ passed (`4/4` assertions green).
- `npm --prefix website run build` ✅ passed.
- `bash scripts/verify-m048-s05.sh` ✅ passed and printed `verify-m048-s05: ok`.
- `test "$(cat .tmp/m048-s05/verify/status.txt)" = "ok" && test "$(cat .tmp/m048-s05/verify/current-phase.txt)" = "complete"` ✅ passed.
- `.tmp/m048-s05/verify/phase-report.txt` shows every named phase passed: `docs-contract`, `m048-s01-entrypoint`, `m048-s02-lsp-neovim`, `m048-s02-vscode`, `m048-s02-publish`, `m048-s03-toolchain-update-core`, `m048-s03-toolchain-update-help`, `m048-s03-toolchain-update-cli`, `m048-s03-toolchain-update-e2e`, `m048-s04-shared-grammar`, `m048-s04-neovim-syntax`, `m048-s04-neovim-contract`, `m048-s04-skill-contract`, `docs-build`, `retain-fixed-m036-artifacts`, `retain-m048-s01-artifacts`, `retain-m048-s03-artifacts`, and `m048-s05-bundle-shape`.
- `.tmp/m048-s05/verify/latest-proof-bundle.txt` resolves to `.tmp/m048-s05/verify/retained-proof-bundle`, and that bundle contains `retained-m036-s02-lsp`, `retained-m036-s02-syntax`, `retained-m036-s03-vscode-smoke`, `retained-m048-s01-artifacts`, and `retained-m048-s03-artifacts` as expected.

## Requirements Advanced

- R113 — S05 promoted installer-backed `meshc update` / `meshpkg update` from slice-local proof into the README, tooling docs, and the milestone-level retained verifier, so the self-update contract is now part of the first-contact public surface.
- R114 — S05 promoted the S04 grammar/skill reset into minimal public touchpoints by updating the tooling docs and VS Code README to mention `@cluster` / `@cluster(N)`, both interpolation forms, and the bounded editor proof surface, while the assembled verifier replays the underlying syntax/skill rails.

## Requirements Validated

- R112 — `bash scripts/verify-m048-s05.sh`, `node --test scripts/tests/verify-m048-s05-contract.test.mjs`, `npm --prefix website run build`, and the retained `.tmp/m048-s05/verify/retained-proof-bundle` now prove that `main.mpl` remains the default executable while manifest override entrypoints such as `lib/start.mpl` behave truthfully across build, test, analyze, publish, and first-contact docs.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None.

## Known Limitations

- Override-entry editor documentation remains intentionally bounded to same-file definition plus hover/diagnostics proof; imported cross-module `goto_definition` is still not part of the public promise.
- Broader evaluator-facing docs cleanup still belongs to later milestones; S05 only corrected the minimal first-contact touchpoints rather than rewriting the wider docs surface.
- Requirement bookkeeping is still partially manual for M048: `gsd_requirement_update` could not find `R112`, so the decision register currently carries the authoritative validation record.

## Follow-ups

- Start milestone validation from `bash scripts/verify-m048-s05.sh`, and debug from `.tmp/m048-s05/verify/phase-report.txt` plus `latest-proof-bundle.txt` before reopening lower-level slice rails.
- Reconcile the GSD requirement DB entry for `R112` if the rendered requirement status needs to flip automatically; decision D328 is the authoritative validation record because `gsd_requirement_update` could not resolve the ID in this environment.
- Keep future README/tooling/VS Code wording changes aligned with `scripts/tests/verify-m048-s05-contract.test.mjs` so first-contact surfaces stay bounded to the proved contract.

## Files Created/Modified

- `README.md` — Updated first-contact README guidance to teach installer-backed updates, keep `main.mpl` as the default executable, document optional `[package].entrypoint = "lib/start.mpl"`, and point readers at the assembled S05 verifier.
- `website/docs/docs/tooling/index.md` — Rewrote the tooling docs touchpoint around the shipped contract: canonical update commands, override-entry manifest example, truthful `meshpkg publish` archive behavior, manifest-first editor/grammar note, and the assembled verifier entrypoint.
- `tools/editors/vscode-mesh/README.md` — Removed the stale cross-file definition overclaim and bounded the VS Code README to same-file definition plus manifest-first override-entry hover/diagnostics proof, while documenting `@cluster` / `@cluster(N)` and both interpolation forms.
- `scripts/tests/verify-m048-s05-contract.test.mjs` — Added a fail-fast docs contract test that asserts the required public markers and bans stale VS Code wording before the long replay runs.
- `scripts/verify-m048-s05.sh` — Added the retained closeout wrapper with named phases, `.tmp/m048-s05/verify` bookkeeping, direct S01-S04 replay, retained bundle copying, and bundle-shape validation.
- `.gsd/PROJECT.md` — Recorded the current M048 state after S05 closeout and noted that the milestone is ready for validation/closeout from the assembled verifier.
- `.gsd/KNOWLEDGE.md` — Captured reusable guidance for the S05 closeout wrapper, bounded VS Code contract, and retained-bundle debugging path.
- `.gsd/DECISIONS.md` — Recorded the S05 requirement, documentation, and verification decisions (D328-D331).
