# M036: Editor Parity & Multi-Editor Support — Context Draft

**Gathered:** 2026-03-27
**Status:** Ready for planning

## Project Description

Mesh is a programming language and backend platform repo with a Rust compiler/tooling workspace under `compiler/`, a docs site under `website/`, and a shipped VS Code extension under `tools/editors/vscode-mesh/`. The repo already has a real editor/tooling surface: `meshc lsp`, a VS Code TextMate grammar, editor docs, and backend-shaped proof code. But the current editor story is ahead of reality: the shipped VS Code grammar still only matches `${...}` interpolation while Mesh docs and current language guidance prefer `#{...}`, and the broader syntax-parity claim has not been audited against real Mesh usage.

This milestone is about editor parity, not just extension packaging. VS Code must stop lagging the language, and Mesh must gain a real first-class non-VSCode path instead of a hand-wavy “other editors can wire it up” story.

## Why This Milestone

The concrete bug already noticed is string interpolation highlighting drift, but the real problem is bigger: editor support is currently being trusted more than it has been proven. That undercuts the daily-driver credibility established elsewhere in the repo.

This needs to happen now because the release/publish lane for the VS Code extension was hardened in M034 without pretending the actual editor syntax truth was done there, and because the user explicitly wants first-class multi-editor support rather than a VS Code-only or best-effort-secondary story. M036 is where the repo makes those editor claims honest.

## User-Visible Outcome

### When this milestone is complete, the user can:

- open real Mesh files in VS Code and get syntax highlighting that matches actual Mesh syntax across an audited corpus, including the preferred `#{...}` interpolation form if the compiler truth still says both forms are valid
- set up Neovim through a repo-owned support pack and documented install path, then use Mesh with syntax support plus `meshc lsp` under a first-class supported story backed by checks

### Entry point / environment

- Entry point: VS Code extension, repo-owned Neovim support pack, and `meshc lsp`
- Environment: local dev editors
- Live dependencies involved: `meshc lsp` JSON-RPC subprocess

## Completion Class

- Contract complete means: named regression checks prove the audited syntax corpus is covered honestly, the supported editor setup paths are documented, and the repo-owned support surfaces match what the compiler and docs actually support
- Integration complete means: the VS Code extension, shared syntax artifacts, Neovim support pack, `meshc lsp`, and public docs all tell the same truthful story about what is supported and how to install/use it
- Operational complete means: a normal local developer can install Mesh, follow the documented VS Code and Neovim setup paths, and get the claimed editor behavior without depending on repo-internal assumptions only

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- a real VS Code flow can open representative Mesh files from the docs, dogfood code, and test corpus and show syntax support that no longer drifts on known language forms such as preferred `#{...}` interpolation
- a real Neovim flow can use the repo-owned support pack plus `meshc lsp` under a documented install path that is backed by regression checks rather than best-effort instructions
- the support claim is exercised in real editors, not only simulated through isolated grammar files or LSP unit tests

## Risks and Unknowns

- full syntax-parity audit may expose more drift than the already-known interpolation gap — that matters because the milestone is about honest editor support, not a one-off bugfix
- a first-class Neovim path may require a parser-driven or Tree-sitter-style artifact alongside the existing TextMate grammar — that matters because the user chose a dual-track direction instead of pretending one artifact fits every editor equally well
- editor-support breadth can sprawl quickly if “first-class” expands beyond install path + docs + checks into “support every editor deeply at once” — that matters because the user wants a real proof target, not a mushy multi-editor aspiration
- interpolation truth must stay aligned with compiler reality — that matters because if both `${...}` and `#{...}` are still valid, the editor story should support both while treating `#{...}` as the preferred form instead of drifting again

## Existing Codebase / Prior Art

- `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json` — current shipped VS Code TextMate grammar; today it still matches `${...}` interpolation and is the clearest known parity gap
- `tools/editors/vscode-mesh/src/extension.ts` — VS Code client bootstrap that starts `meshc lsp` and defines the real editor entrypoint for the existing official extension
- `tools/editors/vscode-mesh/package.json` — official extension packaging, language registration, and grammar wiring surface
- `tools/editors/vscode-mesh/README.md` — current extension truth surface; currently claims comprehensive TextMate coverage and verified LSP integration
- `website/docs/docs/tooling/index.md` — public tooling/editor story, including the current best-effort “other editors” guidance through TextMate reuse and `meshc lsp`
- `compiler/meshc/tests/e2e_lsp.rs` — current repo-level real JSON-RPC LSP proof against backend-shaped files; relevant to keeping multi-editor LSP claims honest even when syntax/highlighting artifacts differ by editor
- `.gsd/milestones/M036/M036-CONTEXT-DRAFT.md` — draft seed that established the known interpolation gap, the need for a first serious non-VSCode target, and the architectural gray areas this context now resolves at a higher level

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R006 — advances the daily-driver tooling credibility bar by making editor syntax/highlighting/LSP truth match real Mesh usage instead of docs-only claims
- R008 — advances the requirement that documentation and examples stay honest about production-style developer workflows, including editor setup and what support is actually first-class
- R010 — supports the claim that Mesh is nicer for backend development in concrete, user-visible ways instead of vague DX rhetoric

## Scope

### In Scope

- audit the VS Code grammar and extension support against an authoritative proof corpus made of docs/examples, dogfood code, and compiler/tests
- fix the known interpolation-highlighting drift and any other syntax-parity gaps uncovered by that audit
- keep interpolation/editor truth aligned with compiler reality; if both `${...}` and `#{...}` are supported, support both while treating `#{...}` as the preferred documented/editor-truth form
- define editor-support ownership as a dual-track model: keep TextMate where it is the honest fit, and allow a parser-driven or Tree-sitter-style path where the target editor needs it
- make Neovim the first serious non-VSCode proof target
- ship a repo-owned Neovim support pack with install path, docs, and regression checks
- keep `meshc lsp` integration honest across supported editors and their documentation

### Out of Scope / Non-Goals

- making every editor named in the docs equally first-class in M036
- redoing the VS Code release/publish hardening work already owned by M034
- broad test-framework work already split into M035
- a giant Neovim plugin-ecosystem push if a repo-owned support pack is the smallest honest first-class delivery shape
- package-manager UX or packages website polish owned by later work

## Technical Constraints

- editor claims must follow compiler truth; the repo should not document syntax/editor support that the shipped surfaces do not actually handle
- “first-class” means install path + docs + checks, not only a README snippet and not necessarily a maximal plugin ecosystem commitment
- Neovim is the first serious secondary target; other editors may still be mentioned, but they are not the proof owner for this milestone
- the proof corpus for syntax parity is not toy-only; it must cover public docs/examples, real dogfood code, and compiler/test material
- the milestone should not force one highlighting artifact to pretend it is equally suitable for every editor; the chosen direction is explicitly dual-track
- multi-editor LSP support continues to rely on `meshc lsp` over stdin/stdout JSON-RPC, so editor-specific work should not invent a separate language-server truth surface

## Integration Points

- `tools/editors/vscode-mesh/` — official extension packaging, grammar wiring, and existing editor runtime surface
- `meshc lsp` — shared LSP backend used by VS Code and the supported multi-editor path
- `website/docs/docs/tooling/index.md` and editor READMEs — public support claims that must match the real supported setup paths
- docs/examples, dogfood Mesh code, and compiler/test fixtures — authoritative syntax audit corpus for parity work
- Neovim runtime/config support surface — new repo-owned support-pack integration point for first-class secondary-editor proof

## Open Questions

- Which exact parser-driven artifact should own the Neovim-side syntax story if TextMate reuse is not the honest fit there — current thinking: keep the milestone-level decision at dual-track, then choose the smallest honest Neovim implementation during planning
- What exact regression harness shape should prove syntax scopes across the audited corpus and both editor paths — current thinking: corpus-based checks plus real editor smoke proof, with final harness design chosen during planning