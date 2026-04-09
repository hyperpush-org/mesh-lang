---
id: M036
title: "Editor Parity & Multi-Editor Support — Context Draft"
status: complete
completed_at: 2026-03-28T07:19:08.501Z
key_decisions:
  - D115: Deliver first-class non-VS Code support as a repo-owned Neovim runtime pack under tools/editors/ with install docs and smoke proof.
  - D116/D117: Prove the shared VS Code/docs grammar through one corpus-backed TextMate/Shiki harness and one shared interpolation rule reused across string kinds.
  - D118/D119/D120: Keep the Neovim path honest with classic Vim syntax, native Neovim 0.11+ LSP bootstrap, explicit meshc discovery, honest root selection, and materialized corpus fixtures.
  - D121/D122/D123: Define first-class editor support operationally, make explicit mesh.lsp.path overrides authoritative in VS Code smoke, and require downstream proof markers in the milestone wrapper.
key_files:
  - scripts/fixtures/m036-s01-syntax-corpus.json
  - scripts/fixtures/m036-s01/interpolation_edge_cases.mpl
  - website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs
  - scripts/verify-m036-s01.sh
  - tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json
  - tools/editors/neovim-mesh/ftdetect/mesh.vim
  - tools/editors/neovim-mesh/syntax/mesh.vim
  - tools/editors/neovim-mesh/lua/mesh.lua
  - tools/editors/neovim-mesh/lsp/mesh.lua
  - tools/editors/neovim-mesh/plugin/mesh.lua
  - tools/editors/neovim-mesh/tests/smoke.lua
  - scripts/tests/verify-m036-s02-materialize-corpus.mjs
  - scripts/verify-m036-s02.sh
  - tools/editors/vscode-mesh/src/extension.ts
  - tools/editors/vscode-mesh/src/test/runTest.ts
  - tools/editors/vscode-mesh/src/test/suite/index.ts
  - tools/editors/vscode-mesh/src/test/suite/extension.test.ts
  - scripts/tests/verify-m036-s03-contract.test.mjs
  - scripts/tests/verify-m036-s03-wrapper.test.mjs
  - scripts/verify-m036-s03.sh
  - tools/editors/vscode-mesh/README.md
  - tools/editors/neovim-mesh/README.md
  - website/docs/docs/tooling/index.md
lessons_learned:
  - For editor support, 'first-class' should be an operational contract: repo-owned docs plus a repo-owned verifier for that exact editor host, not generic LSP/TextMate compatibility claims.
  - A single audited syntax corpus can safely drive multiple editor surfaces if docs-backed snippets are materialized into editor-native temporary files before host-specific smoke runs.
  - Real editor-host smoke should pin the compiler/LSP binary explicitly and surface the resolved path/source in logs; otherwise PATH or workspace fallback can hide wrong-binary drift.
  - Assembled milestone wrappers should require downstream artifact markers in addition to exit codes so partial success cannot look green when an editor host or replay phase never truly started.
---

# M036: Editor Parity & Multi-Editor Support — Context Draft

**Made Mesh’s editor story truthful and daily-driver credible by proving the shared VS Code/docs grammar against a real Mesh corpus, shipping a repo-owned first-class Neovim path, and publishing proof-scoped editor support backed by real VS Code and Neovim smoke.**

## What Happened

M036 closed the editor-truth gap by turning Mesh’s editor story into a repo-owned, verifier-backed contract instead of a mix of shipped behavior and aspirational docs.

**S01** established the shared syntax truth surface. The milestone added an audited corpus manifest spanning real Mesh sources, docs snippets, and a minimal interpolation edge-case fixture, then proved the single shared TextMate grammar through both standalone TextMate and docs-side Shiki. The grammar was repaired so `#{...}` and `${...}` share one interpolation rule across double- and triple-quoted strings, with recursive nested-brace handling inside interpolation bodies. That retired the already-shipping VS Code/docs highlighting drift and made future regressions fail closed through `scripts/verify-m036-s01.sh` and `website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs`.

**S02** turned non-VS Code support into a real first-class path by shipping a repo-owned Neovim runtime pack under `tools/editors/neovim-mesh/`. The pack forces `*.mpl` to `filetype=mesh`, provides a bounded classic Vim syntax surface aligned to the audited interpolation corpus, and bootstraps native Neovim 0.11+ `meshc lsp` without depending on external plugin-manager or `nvim-lspconfig` conventions. The acceptance path stayed honest by materializing every shared corpus case into temporary `.mpl` files, replaying the shared S01 grammar proof, replaying upstream `meshc` LSP transport truth, and then exercising the real package-runtime install path through headless Neovim smoke.

**S03** finished the public contract. The tooling docs and editor READMEs now publish explicit first-class vs best-effort support tiers, with first-class meaning Mesh owns both the installation path/docs and a verifier for that exact editor host. The repo now also contains a real VS Code Extension Development Host smoke path pinned to `target/debug/meshc`; the smoke opened `reference-backend` Mesh files, waited for clean diagnostics, and proved hover plus definition behavior inside the actual editor host. `scripts/verify-m036-s03.sh` assembled the public editor story end to end by replaying docs contract checks, a VitePress build, the existing VSIX proof, the new VS Code smoke, and the Neovim replay.

The assembled verification passed in this closeout run. `bash scripts/verify-m036-s03.sh` completed successfully and emitted a clean milestone artifact root at `.tmp/m036-s03/`. Its VS Code smoke log shows the extension resolved `meshc` from configuration at `/Users/sn0w/Documents/dev/mesh-lang/target/debug/meshc`, opened `reference-backend/api/health.mpl` and `reference-backend/api/jobs.mpl` as `languageId=mesh`, waited for clean diagnostics on both files, returned hover type `Result<Job, String>` at `jobs.mpl:62:16`, and resolved definition back to `reference-backend/api/jobs.mpl:33`. The same assembled run also replayed the Neovim verifier, which materialized all 15 shared syntax corpus cases, proved `filetype=mesh` / `syntax=mesh`, and attached the Mesh LSP both in a rooted `reference-backend` workspace (`marker=main.mpl`) and a standalone single-file buffer (`root=<none>`).

## Decision Re-evaluation

| Decision | Re-evaluation |
|---|---|
| D115 — deliver Neovim as a repo-owned runtime pack under `tools/editors/` with install docs and smoke checks | **Still valid.** The shipped `tools/editors/neovim-mesh/` pack, README, and `scripts/verify-m036-s02.sh` proved this was the smallest honest first-class non-VS Code path. |
| D116 / D117 — prove the shared VS Code/docs surface with one corpus-backed TextMate/Shiki harness and one shared interpolation rule | **Still valid.** `scripts/verify-m036-s01.sh` and `website/scripts/tests/verify-m036-s01-syntax-parity.test.mjs` passed against the audited corpus, and the repaired shared interpolation rule now anchors both editor/docs wording and regression detection. |
| D118 / D119 / D120 — implement Neovim as a classic-syntax + native `meshc lsp` pack, reusing the shared corpus through materialized `.mpl` fixtures and honest root/meshc discovery | **Still valid.** The final Neovim smoke passed through the real package-runtime install path, correctly handled the built-in Maple `*.mpl` collision, and proved both rooted and standalone LSP attachment without overstating Tree-sitter or plugin-manager support. |
| D121 / D122 / D123 — define first-class support operationally, treat explicit `mesh.lsp.path` overrides as authoritative, and require replayed artifact markers in the repo-root wrapper | **Still valid.** The docs contract tests passed, the VS Code smoke surfaced the resolved configured `meshc` path, and `scripts/verify-m036-s03.sh` stayed fail-closed on real downstream proof markers rather than exit code alone. |

No M036 decisions currently need immediate reversal. The next milestone should revisit them only if Mesh adds another editor with its own repo-owned proof surface or chooses to invest in a broader Neovim Tree-sitter path.

## Success Criteria Results

The roadmap does not define a separate `Success Criteria` section; it expresses milestone success through the vision plus the three slice `After this` outcomes. Those outcomes were verified directly.

### SC1: The shared VS Code/docs surface truthfully handles real Mesh interpolation syntax, including `#{...}` and `${...}` in double- and triple-quoted strings, with corpus-backed parity checks that localize regressions.
**MET.** `bash scripts/verify-m036-s01.sh` passed in the assembled S02/S03 replay, proving compiler lexer truth plus shared TextMate/Shiki parity. The S02 Neovim replay materialized **15** audited corpus cases, including docs-backed markdown snippets and nested-brace interpolation fixtures, and the shared-surface parity test reported all current corpus cases green. This confirms the repaired shared grammar contract now matches real Mesh sources and docs snippets instead of a one-off example.

### SC2: Mesh ships a repo-owned first-class Neovim path that a developer can install from repo docs, open a `.mpl` file in, and get `filetype=mesh`, syntax support, and `meshc lsp` through the documented path.
**MET.** `bash scripts/verify-m036-s02.sh` was replayed successfully inside `bash scripts/verify-m036-s03.sh`. The closeout run proved `filetype=mesh` / `syntax=mesh` on all materialized corpus cases, verified the runtime pack install path under `pack/*/start/mesh-nvim`, and showed the Mesh LSP attaching both to rooted `reference-backend` files (`marker=main.mpl`, `meshc_class=workspace-target-debug`) and to a standalone temporary `.mpl` file (`root=<none>`, `marker=single-file`). The repo-owned install and runbook live in `tools/editors/neovim-mesh/README.md`.

### SC3: Public docs tell the truth about editor support tiers, and the published VS Code and Neovim workflows are backed by real smoke proof.
**MET.** `bash scripts/verify-m036-s03.sh` passed the docs contract tests, VitePress build, VSIX proof, real VS Code Extension Development Host smoke, and Neovim replay. The docs contract suite (`scripts/tests/verify-m036-s03-contract.test.mjs`) passed all 6 checks, including fail-closed tests for missing support-tier headings and stale best-effort wording. The VS Code smoke log under `.tmp/m036-s03/vscode-smoke/smoke.log` shows the extension resolving the configured `target/debug/meshc`, waiting for clean diagnostics in `reference-backend/api/health.mpl` and `reference-backend/api/jobs.mpl`, returning hover type `Result<Job, String>`, and resolving definition to `jobs.mpl:33`.

## Definition of Done Results

### DoD1: All planned slices for M036 are complete.
**MET.** The roadmap shows S01, S02, and S03 marked `✅`, and `gsd_complete_milestone` validated slice-complete state before rendering the milestone summary.

### DoD2: All slice summaries exist for milestone drill-down.
**MET.** The closeout check `find .gsd/milestones/M036/slices -maxdepth 2 -name '*-SUMMARY.md' | sort` returned:
- `.gsd/milestones/M036/slices/S01/S01-SUMMARY.md`
- `.gsd/milestones/M036/slices/S02/S02-SUMMARY.md`
- `.gsd/milestones/M036/slices/S03/S03-SUMMARY.md`

### DoD3: Cross-slice integration points work correctly in the assembled editor story.
**MET.** `bash scripts/verify-m036-s03.sh` served as the milestone-level integration check and passed end to end. It replayed the docs contract, VitePress build, VSIX proof, real VS Code smoke, and the full Neovim verifier; the Neovim verifier in turn replayed the shared S01 grammar proof and upstream `cargo test -q -p meshc --test e2e_lsp -- --nocapture`. This confirms S01’s shared grammar truth feeds S02’s Neovim surface, and S01+S02 both feed S03’s public support-tier contract.

### DoD4: The milestone includes real non-`.gsd` code/documentation changes, not planning-only artifacts.
**MET.** Because auto-mode is running on local `main`, the practical closeout diff check used the working-tree equivalent `git diff --stat -- ':!.gsd/'`, which shows non-`.gsd` changes in the VS Code smoke harness (`tools/editors/vscode-mesh/src/test/runTest.ts` plus compiled outputs). The slice summaries also record the broader non-`.gsd` editor/docs/tooling changes delivered across the milestone, including the shared grammar, corpus/verifier scripts, Neovim runtime pack, tooling docs, and VS Code smoke/test files.

### Horizontal Checklist
No `Horizontal Checklist` section is present in `.gsd/milestones/M036/M036-ROADMAP.md`, so there were no additional horizontal items to audit for this closeout.

## Requirement Outcomes

No requirement status transitions occurred during M036.

M036 materially strengthens the evidence behind the already-validated tooling credibility requirement (**R006**) by adding corpus-backed shared grammar proof, a repo-owned first-class Neovim path, explicit editor support tiers, and real VS Code/Neovim smoke verification. However, because R006 was already in `validated` status before this milestone and no requirement was explicitly advanced or re-scoped during M036, `.gsd/REQUIREMENTS.md` does not require a status update for this closeout.

## Deviations

The roadmap stayed within scope, but the closeout proof used the repo-local portable Neovim 0.11.6 binary under `.tmp/m036-s02/vendor/` because this host does not provide a system `nvim`. The VS Code smoke path also retains the bounded launch-level retry introduced in S03: it retries only when the Extension Development Host exits before the suite-start marker is logged, and any failure after the suite starts remains a real regression.

## Follow-ups

If Mesh wants broader editor claims beyond VS Code and Neovim, add them only after the repo owns both the install path and an editor-host verifier for that editor. Future Neovim expansion should be a separate proof-backed slice (for example Tree-sitter) rather than being implied by the classic runtime pack. Keep growing the shared syntax corpus before widening interpolation or highlighting claims in any editor or docs surface.
