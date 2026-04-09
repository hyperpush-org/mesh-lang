---
id: M048
title: "Entrypoint Flexibility & Tooling Truth Reset"
status: complete
completed_at: 2026-04-02T19:31:33.941Z
key_decisions:
  - Centralize executable entry selection in `mesh_pkg::manifest::resolve_entrypoint(...)` and reuse that seam across compiler, test, LSP, editor, and publish surfaces.
  - Keep executable-entry truth separate from module identity: only root `main.mpl` becomes `Main`, while non-root executable entries stay path-derived modules.
  - Ship `meshc update` and `meshpkg update` on one shared installer-backed updater seam, and keep `--json update` fail-closed until there is a truthful machine-readable updater protocol.
  - Keep editor syntax and init-time skill parity grounded in one shared syntax corpus and the current clustered-runtime story instead of duplicating stale rules per surface.
  - Close the milestone with one assembled verifier and retained proof bundle, and keep public docs bounded to the actually shipped same-file-definition/editor truth.
key_files:
  - compiler/mesh-pkg/src/manifest.rs
  - compiler/meshc/src/main.rs
  - compiler/meshc/src/discovery.rs
  - compiler/meshc/src/test_runner.rs
  - compiler/mesh-codegen/src/lib.rs
  - compiler/mesh-lsp/src/analysis.rs
  - compiler/mesh-pkg/src/toolchain_update.rs
  - compiler/mesh-pkg/tests/toolchain_update.rs
  - compiler/meshc/tests/e2e_m048_s01.rs
  - compiler/meshc/tests/e2e_lsp.rs
  - compiler/meshc/tests/e2e_m048_s03.rs
  - compiler/meshc/tests/tooling_e2e.rs
  - compiler/meshpkg/src/main.rs
  - compiler/meshpkg/src/publish.rs
  - compiler/meshpkg/tests/update_cli.rs
  - tools/editors/neovim-mesh/lua/mesh.lua
  - tools/editors/neovim-mesh/syntax/mesh.vim
  - tools/editors/vscode-mesh/src/test/suite/extension.test.ts
  - tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json
  - tools/skill/mesh/SKILL.md
  - tools/skill/mesh/skills/clustering/SKILL.md
  - scripts/verify-m048-s05.sh
  - scripts/tests/verify-m048-s04-skill-contract.test.mjs
  - scripts/tests/verify-m048-s05-contract.test.mjs
  - README.md
  - website/docs/docs/tooling/index.md
lessons_learned:
  - When auto-mode is already running on merged `main`, the truthful non-`.gsd` diff baseline is `origin/main` (or the pre-milestone commit if even that is merged), not `git merge-base HEAD main`.
  - Default-plus-override entrypoint behavior stays stable only when every project-aware surface reuses the same manifest resolver seam; re-encoding path rules per tool invites drift.
  - Installer-backed CLI features need explicit proof rails for help text, `--json` guards, staged release downloads, and installed-binary repair; unit coverage alone is not enough.
  - Public docs should contract-test bounded truth, especially when an editor capability is intentionally limited (same-file definition vs. cross-file definition).
  - The current GSD requirements DB still does not know about the M048 requirement family (`R112`-`R114`), so closeout had to keep the checked-in `.gsd/REQUIREMENTS.md` truthful manually after `gsd_requirement_update` rejected those IDs as not found.
---

# M048: Entrypoint Flexibility & Tooling Truth Reset

**M048 made Mesh entrypoints overrideable across compiler/editor/package surfaces, added installer-backed self-update commands, reset editor and init-skill truth to the current clustered contract, and closed the loop with one retained verifier plus truthful public docs.**

## What Happened

M048 reset Mesh's first-contact tooling truth around one shared executable-entry contract. S01 introduced optional `[package].entrypoint` support and `mesh_pkg::manifest::resolve_entrypoint(...)`, made `meshc build` and `meshc test` consume that same validated project-root-relative seam, preserved path-derived names for non-root entries, and fixed MIR merge ordering so the designated entry module actually wins when both root `main.mpl` and an override entry define `fn main()`.

S02 carried that same contract through `mesh-lsp`, `meshc lsp`, the repo-owned Neovim and VS Code hosts, and `meshpkg publish`, so manifest-first override-entry projects now behave like first-class Mesh workspaces instead of falling back to stale root-`main.mpl` assumptions. S03 added installer-backed `meshc update` / `meshpkg update` commands via a shared updater seam and proved both staged-install and installed-repair flows. S04 reset syntax and init-time teaching surfaces so `@cluster`, `@cluster(N)`, `#{...}`, `${...}`, and current clustered-runtime guidance are the truth everywhere users first touch the project.

S05 then assembled the milestone into one closeout rail instead of leaving proof fragmented across slice-local commands. Fresh closeout verification passed via `bash scripts/verify-m048-s05.sh`, and `.tmp/m048-s05/verify/phase-report.txt` shows every named phase passed: docs contract, S01 entrypoint replay, S02 LSP/editor/package rails, S03 toolchain-update rails, S04 grammar/skill rails, docs build, retained-artifact capture, and final bundle-shape validation. Because auto-mode is closing out on local `main`, the truthful code-diff baseline was `origin/main` merge base `df209ffac23a6e3b9785532e452f21c41849a2d9`; `git diff --stat "$BASE" HEAD -- ':!.gsd/'` showed 41 non-`.gsd/` files changed with 6,937 insertions and 299 deletions across compiler, package, editor, test, and docs surfaces.

### Decision re-evaluation

| Decision | Status | Notes |
| --- | --- | --- |
| Reuse one shared manifest entrypoint resolver across compiler/test/LSP/editor/package surfaces | Keep | Fresh assembled verification passed across all consumer surfaces, so the seam prevented contract drift instead of creating it. |
| Keep non-root executable entries path-derived instead of inventing a second `Main` naming rule | Keep | Build, LSP, and publish rails all stayed green without any downstream need for a second naming exception. |
| Ship updater behavior through one installer-backed seam for both CLIs | Keep | Core/help/CLI/e2e updater phases all passed, and both `meshc update` and `meshpkg update` repaired the toolchain through the canonical path. |
| Make `--json update` fail closed until a real machine-readable installer protocol exists | Keep | The help/guard rails stayed truthful; no fake JSON success surface was needed to pass milestone goals. |
| Keep editor/public docs bounded to current same-file-definition truth and contract-test that wording | Keep, revisit only if capability expands | The docs contract passed and the same-file limitation remained honestly documented; revisit next milestone only if cross-file definition becomes a shipped guarantee. |

## Success Criteria Results

- [x] **Default-plus-override executable contract delivered for compiler build and `meshc test`.**
  - Evidence: S01 shipped the shared `[package].entrypoint` seam plus `compiler/meshc/tests/e2e_m048_s01.rs`, and fresh closeout verification replayed `cargo test -p meshc --test e2e_m048_s01 m048_s01 -- --nocapture` successfully. `.tmp/m048-s05/verify/phase-report.txt` records `m048-s01-entrypoint\tpassed`.
- [x] **Non-root non-`main.mpl` projects behave like first-class projects across LSP, editor hosts, and package publish/discovery surfaces.**
  - Evidence: fresh closeout verification passed `m048-s02-lsp-neovim`, `m048-s02-vscode`, and `m048-s02-publish`, including `NEOVIM_BIN=nvim bash scripts/verify-m036-s02.sh lsp`, `npm --prefix tools/editors/vscode-mesh run test:smoke`, and `cargo test -p meshpkg publish_archive_members_ -- --nocapture`.
- [x] **Installer-backed self-update commands exist and refresh the toolchain through the canonical installer path.**
  - Evidence: fresh closeout verification passed `m048-s03-toolchain-update-core`, `m048-s03-toolchain-update-help`, `m048-s03-toolchain-update-cli`, and `m048-s03-toolchain-update-e2e`, replaying `cargo test -p mesh-pkg --test toolchain_update -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_update -- --nocapture`, `cargo test -p meshpkg --test update_cli -- --nocapture`, and `cargo test -p meshc --test e2e_m048_s03 m048_s03 -- --nocapture`.
- [x] **Syntax and init-skill parity were reset to the current clustered/runtime contract.**
  - Evidence: fresh closeout verification passed `m048-s04-shared-grammar`, `m048-s04-neovim-syntax`, `m048-s04-neovim-contract`, and `m048-s04-skill-contract`, replaying `bash scripts/verify-m036-s01.sh`, `NEOVIM_BIN=nvim bash scripts/verify-m036-s02.sh syntax`, `node --test scripts/tests/verify-m036-s02-contract.test.mjs`, and `node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs`.
- [x] **One retained verifier plus minimal public touchpoints tell the truth about the shipped contract.**
  - Evidence: `node --test scripts/tests/verify-m048-s05-contract.test.mjs` passed, `npm --prefix website run build` passed, and `bash scripts/verify-m048-s05.sh` finished with `verify-m048-s05: ok`. `.tmp/m048-s05/verify/status.txt` reads `ok`, `.tmp/m048-s05/verify/current-phase.txt` reads `complete`, and every named phase in the phase report is marked `passed`.

## Definition of Done Results

- [x] **All roadmap slices complete.** The inlined roadmap marks S01-S05 done, and milestone completion validated slice completion before writing this summary.
- [x] **All slice summaries exist.** `find .gsd/milestones/M048/slices -maxdepth 2 -type f -name 'S*-SUMMARY.md' | sort` returned all five slice summaries, and the corresponding task summaries are present under each `tasks/` directory.
- [x] **The milestone produced real code, not only planning artifacts.** Because closeout is running on merged local `main`, the truthful integration-branch equivalent was `BASE=$(git merge-base HEAD origin/main)`. `git diff --stat "$BASE" HEAD -- ':!.gsd/'` showed 41 non-`.gsd/` files changed, with 6,937 insertions and 299 deletions across compiler, package, editor, tests, scripts, and docs surfaces.
- [x] **Cross-slice integration works correctly.** Fresh `bash scripts/verify-m048-s05.sh` passed end to end; `.tmp/m048-s05/verify/phase-report.txt` shows the entrypoint, LSP/editor/package, updater, grammar/skill, docs-build, retained-artifact, and bundle-shape phases all passed in one assembled replay.
- [x] **Milestone validation passed.** `.gsd/milestones/M048/M048-VALIDATION.md` records verdict `pass` at remediation round `0`, and its success-criteria, slice-delivery, cross-slice integration, and requirement-coverage sections reconcile with the fresh closeout replay.
- [x] **Horizontal checklist reconciled.** The roadmap does not include a separate Horizontal Checklist section, so there were no additional horizontal items to audit or mark incomplete.

## Requirement Outcomes

- **R112** — `active -> validated`. Evidence: S01 delivered the shared `[package].entrypoint` resolver for compiler build and `meshc test`, S02 propagated the same override-entry truth into LSP/editor/package surfaces, and the fresh assembled verifier passed `m048-s01-entrypoint`, `m048-s02-lsp-neovim`, `m048-s02-vscode`, and `m048-s02-publish`.
- **R113** — `active -> validated`. Evidence: S03 shipped `meshc update` and `meshpkg update` on the shared installer-backed updater seam, and the fresh assembled verifier passed `m048-s03-toolchain-update-core`, `m048-s03-toolchain-update-help`, `m048-s03-toolchain-update-cli`, and `m048-s03-toolchain-update-e2e`.
- **R114** — `active -> validated`. Evidence: S02 made manifest-first root detection, diagnostics, and editor-host smoke truthful for override-entry projects; S04 reset syntax and init-time skill parity; and the fresh assembled verifier passed `m048-s02-lsp-neovim`, `m048-s02-vscode`, `m048-s04-shared-grammar`, `m048-s04-neovim-syntax`, `m048-s04-neovim-contract`, and `m048-s04-skill-contract`.
- **Requirement bookkeeping note.** `gsd_requirement_update` rejected `R112`, `R113`, and `R114` as not found even though `.gsd/REQUIREMENTS.md` renders them, so the checked-in requirements file was updated manually to keep the visible project state truthful.
- No M048 requirements were deferred, blocked, invalidated, or re-scoped during closeout.

## Deviations

Milestone closeout used the integration-branch equivalent of the code-diff check (`origin/main` merge base) because M048 landed directly on local `main`; otherwise no milestone-level closeout deviation was needed.

## Follow-ups

If cross-file imported-call go-to-definition becomes part of Mesh's public editor promise, extend `mesh-lsp` beyond its current open-document-local definition path and widen the docs contract accordingly. M049 and M050 should build on M048's truthful first-contact surfaces to reset scaffolds/examples and public docs without reintroducing stale `main.mpl`, syntax, or clustered-runtime guidance.
