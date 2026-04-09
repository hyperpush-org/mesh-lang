---
depends_on: [M034, M035]
---

# M036: Editor Parity & Multi-Editor Support — Context Draft

**Gathered:** 2026-03-26
**Status:** Draft — needs dedicated discussion before planning

## Seed From Current Discussion

The user wants the VS Code extension to support all syntax. The concrete issue they already noticed is that string interpolation is not supported correctly in syntax highlighting. They also want all syntax support checked more broadly, not just that one bug.

Beyond VS Code, they want support for other editors like Vim “in some way,” and when asked how serious that should be, they chose a **first-class multi-editor** direction instead of VS Code-only or best-effort secondary support.

## What This Milestone Likely Covers

- audit the shipped VS Code grammar and extension against real Mesh syntax coverage
- repair syntax-highlighting drift, starting with the known `#{}` string-interpolation gap
- decide what the canonical reusable editor-support artifact should be: improved TextMate grammar reuse, a parser-driven highlighting path, or another maintainable shared source
- give at least one non-VSCode editor a first-class supported path instead of vague “should work if you wire it yourself” guidance
- keep LSP integration honest across editor docs and supported setups

## Why This Needs Its Own Discussion

The direction is clear, but the proof target is not. There is still no explicit user choice for:
- which non-VSCode editor should become the first serious proof target
- whether the repo should stay TextMate-first, move toward a parser/Tree-sitter-style shared highlighting surface, or support both
- what “first-class” support means in practice for installability, maintenance, docs, and regression coverage

Those decisions materially affect milestone size and architecture, so this should not be improvised from the current conversation alone.

## Existing Codebase / Prior Art To Revisit

- `tools/editors/vscode-mesh/package.json`
- `tools/editors/vscode-mesh/src/extension.ts`
- `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json`
- `tools/editors/vscode-mesh/README.md`
- `website/docs/docs/tooling/index.md`
- `compiler/meshc/tests/e2e_lsp.rs`

## Technical Findings Already Established

- the current shipped grammar still matches `${...}` interpolation while Mesh documentation and current language guidance prefer `#{...}`
- the repo already documents a best-effort “other editors” story through TextMate grammar reuse and `meshc lsp`, but that is not yet a first-class support model
- the extension publish path and editor syntax truth are related but should not be collapsed into the same milestone goal

## Likely Risks / Unknowns

- full syntax parity may expose more drift than the already-known interpolation issue
- a truly first-class non-VSCode story may require a different shared grammar strategy than the current VS Code-only packaging assumptions
- editor-support breadth can sprawl quickly if the first supported secondary target is not chosen deliberately

## Likely Outcome When Done

Mesh editor support stops lagging behind the language, the VS Code surface is honest, and there is at least one real non-VSCode editor path the repo can support without hand-wavy caveats.

## Open Questions For The Dedicated Discussion

- Which editor should be the first serious non-VSCode proof target?
- Is the long-term source of truth still a TextMate grammar, or should a parser-driven/highlighting-query surface become canonical?
- What regression coverage is necessary before the repo claims syntax support is complete?
