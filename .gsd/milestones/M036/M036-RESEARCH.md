# M036 Research — Editor Parity & Multi-Editor Support

**Researched:** 2026-03-27  
**Status:** Ready for roadmap planning

## Summary

M036 is not starting from zero editor support. Mesh already ships three real surfaces:

- an official VS Code extension in `tools/editors/vscode-mesh/`
- a shared TextMate grammar consumed by both VS Code and the docs site
- a real stdio JSON-RPC language server proven by `compiler/meshc/tests/e2e_lsp.rs`

The main finding is that the current editor trust gap is broader than one bad regex, but still naturally sliceable:

1. **Compiler truth and editor truth have diverged on interpolation.** The compiler currently accepts both `${...}` and `#{...}`; dogfood code and newer docs prefer `#{...}`; the shipped VS Code grammar still only highlights `${...}`.
2. **The shared grammar is a bigger blast radius than it looks.** `website/docs/.vitepress/config.mts` and `website/docs/.vitepress/theme/composables/useShiki.ts` import `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json` directly, so VS Code grammar drift is also website syntax-highlighting drift.
3. **The current proof lane is honest about packaging/LSP transport, but it does not prove editor parity.** M034’s extension verifier packages the VSIX, audits its contents, and reuses `e2e_lsp`; it does not prove real syntax highlighting parity or any non-VSCode path.
4. **There is currently no repo-owned non-VSCode support surface.** `tools/editors/` only contains the VS Code extension. Docs mention other editors, but there is no Neovim pack, no filetype/runtime assets, no Tree-sitter grammar, no Vim syntax, and no smoke checks.

That makes the natural milestone order clear:

- prove syntax truth against a real corpus first
- repair the already-shipping VS Code/shared-grammar surface second
- add the smallest honest repo-owned Neovim support pack third
- finish by making docs and support-tier claims explicit and mechanically checked

## Skills Discovered

Installed during research because they are directly relevant to the new editor-support surface:

- `neovim` — installed from `julianobarbosa/claude-code-skills@neovim`
- `tree-sitter` — installed from `plurigrid/asi@tree-sitter`

Also evaluated:

- VS Code extension skills were searched, but the top candidate repo did not expose the requested skill name cleanly. Existing repo knowledge plus the already-installed `vscode-extension-publisher` skill are enough for planning; M036 is about parity/support truth, not publish-lane work.

## What Exists Today

### 1. Official VS Code surface

Relevant files:

- `tools/editors/vscode-mesh/package.json`
- `tools/editors/vscode-mesh/src/extension.ts`
- `tools/editors/vscode-mesh/language-configuration.json`
- `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json`
- `tools/editors/vscode-mesh/README.md`

Key facts:

- The extension registers `.mpl` as language `mesh`.
- It starts `meshc lsp` through `vscode-languageclient`.
- It exposes only one user setting: `mesh.lsp.path`.
- Local extension tests only cover VSIX path packaging helper behavior (`scripts/vsix-path.test.mjs`). There are **no grammar regression tests** and **no extension-host behavior tests**.

### 2. Shared grammar already feeds the docs site

Relevant files:

- `website/docs/.vitepress/config.mts`
- `website/docs/.vitepress/theme/composables/useShiki.ts`

Both import `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json` directly. That is important for planning:

- **good:** one VS Code grammar fix also improves docs-site highlighting
- **bad:** one grammar mistake silently affects both the shipped editor surface and public docs rendering

### 3. Real LSP proof already exists

Relevant files:

- `compiler/meshc/tests/e2e_lsp.rs`
- `compiler/mesh-lsp/src/server.rs`
- `compiler/mesh-lsp/src/analysis.rs`

Current proof is stronger than docs-only claims:

- real `meshc lsp` subprocess
- stdio JSON-RPC initialize/open/change/hover/definition/signature-help/formatting/shutdown
- backend-shaped files from `reference-backend/`

I re-ran the current LSP transport proof successfully:

- `cargo test -q -p meshc --test e2e_lsp -- --nocapture`

This is worth reusing, not replacing.

### 4. M034 already closed packaging/release proof for VS Code

Relevant files:

- `scripts/verify-m034-s04-extension.sh`
- `tools/editors/vscode-mesh/scripts/vsix-path.mjs`
- `.github/workflows/extension-release-proof.yml`
- `.github/workflows/publish-extension.yml`

Important planning takeaway:

- packaging, artifact naming, and release handoff are already the proven lane
- that verifier explicitly reuses `e2e_lsp` and does **not** pretend to prove syntax parity
- M036 should build on that boundary, not reopen release-lane work

This aligns with existing decisions D085, D086, and D089.

## Compiler Truth vs Editor Truth

### Compiler truth right now: both interpolation forms work

I verified current compiler behavior directly:

- `cargo test -q -p meshc --test e2e e2e_string_interp -- --nocapture`
- `cargo test -q -p meshc --test e2e e2e_string_interp_hash -- --nocapture`
- `cargo test -q -p meshc --test e2e e2e_heredoc_interp -- --nocapture`
- `cargo test -q -p mesh-fmt idempotent_string_interpolation -- --nocapture`

Observed result:

- `${...}` works
- `#{...}` works
- `#{...}` works inside triple-quoted strings
- formatter idempotency already covers `#{...}`

### The implementation already accepts both forms, but internal comments/tests drift old

Relevant files:

- `compiler/mesh-lexer/src/lib.rs`
- `compiler/mesh-common/src/token.rs`
- `compiler/mesh-common/src/error.rs`
- `compiler/mesh-parser/src/syntax_kind.rs`
- `compiler/mesh-parser/tests/snapshots/parser_tests__string_interpolation.snap`
- `compiler/mesh-lexer/tests/lexer_tests.rs`

Important detail:

- the lexer implementation has explicit branches for both `${` and `#{`
- but comments, token docs, parser docs, lexer tests, and parser snapshots still mostly describe only `${...}`

That means the gap is not only user-facing grammar drift; it also exists in lower-level “truth surfaces” that future editor work might trust incorrectly.

## Corpus Drift Findings

I sampled the real corpus by counting interpolation forms across the main proof surfaces.

### Mesh source corpus counts

- `reference-backend/`: `#{}` = 36, `${}` = 0
- `mesher/`: `#{}` = 184, `${}` = 0
- `tests/e2e/`: `#{}` = 7, `${}` = 295

### Documentation corpus

Raw markdown counts in `website/docs/docs/` are mixed, but the more important finding is qualitative:

- `getting-started` and `language-basics` explicitly say `#{}` is preferred and `${}` is still valid
- many other docs pages still show Mesh code examples using `${}` (`concurrency`, `distributed`, `web`, `type-system`, `iterators`, `tooling` REPL examples, and portions of `language-basics` itself)
- `cheatsheet` currently documents both forms side-by-side

### Why this matters

The repo currently has **three different centers of gravity**:

- real dogfood code prefers `#{}`
- compiler tests still heavily exercise `${}`
- docs say `#{}` is preferred but many examples still teach `${}`

So M036 should not frame the task as “flip everything to one syntax immediately.” The honest first contract is:

- **support both while the compiler supports both**
- **treat `#{}` as the preferred documented/editor-first form**
- **use corpus-based checks so drift is detected instead of reintroduced**

## The Concrete VS Code Gap

The shipped grammar file still only matches `${` interpolation in both normal and triple-quoted strings:

- `tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json`

That is the clearest already-shipping bug, because the actual dogfood corpus is now strongly `#{}`-first.

This is not isolated to VS Code:

- the website docs import the same grammar, so Mesh code blocks there inherit the same interpolation-highlighting blind spot

## Non-VSCode Support Reality

Current repo state:

- `tools/editors/` contains only `vscode-mesh/`
- no Neovim runtime pack
- no `ftdetect/`, `syntax/`, `plugin/`, `after/`, `queries/`, or `parser/` assets for Mesh
- no Tree-sitter grammar in-repo
- no headless Neovim smoke scripts

Current docs claim:

- “other editors” can reuse the TextMate grammar or wire up `meshc lsp`

That claim is now too soft for the user’s stated goal and for D072’s direction.

## Technology Findings Relevant to Neovim

I checked current Neovim and nvim-treesitter docs.

### Neovim runtime layout makes a repo-owned support pack feasible

Neovim runtimepath supports these directories directly:

- `ftdetect/`
- `syntax/`
- `lsp/`
- `parser/`
- `queries/`

That means M036 does **not** need a plugin-manager ecosystem push to become first-class. A repo-owned runtime pack under something like `tools/editors/neovim-mesh/` is a viable delivery shape.

### Neovim LSP setup can stay very small

From current `nvim-lspconfig` guidance, the minimum honest server config is:

- `cmd`
- `filetypes`
- `root_markers`

For Mesh that strongly suggests:

- `cmd = { "meshc", "lsp" }`
- `filetypes = { "mesh" }`
- root markers aligned with existing Mesh behavior, especially `main.mpl` and likely `mesh.toml` / `.git`

This should follow the existing Mesh LSP project-root contract rather than inventing a different root story.

### Tree-sitter is feasible, but it is still a scope multiplier

Current `nvim-treesitter` docs support custom parsers from a local path or Git URL plus in-repo queries.

That means a repo-owned Mesh Tree-sitter grammar is technically straightforward to wire into Neovim **once it exists**.

But the key planning point is that Tree-sitter is still a net-new grammar surface, query set, and maintenance burden. It should be chosen because the corpus proves it is the smallest honest answer for Neovim, not because it sounds more modern.

## Recommended Planning Boundary: Start With a Corpus Contract, Not With Tree-sitter

The highest-leverage first proof is **not** “build a Neovim plugin” and **not** “rewrite everything to `#{}`.”

It is:

1. define the authoritative syntax corpus
2. encode the interpolation truth contract explicitly
3. make the VS Code/shared-grammar surface match that contract
4. choose the smallest honest Neovim syntax surface after the corpus is visible

That ordering retires the biggest risk first: hidden parity drift beyond the already-known interpolation bug.

## Existing Patterns to Reuse

### Reuse 1: Evidence-first milestone pattern

Relevant decisions:

- D004 — evidence-first hardening over surface-area expansion
- D015 — daily-driver tooling work should sequence proof before broader claims
- D072 — non-VSCode support must be first-class, not best-effort

This milestone fits that pattern exactly.

### Reuse 2: Keep one LSP truth surface

Do **not** fork editor behavior by editor. Reuse:

- `meshc lsp`
- the stdio JSON-RPC contract
- `compiler/meshc/tests/e2e_lsp.rs`

Editor-specific work should stay about:

- filetype detection
- syntax/highlighting artifact(s)
- install/runtime packaging
- docs and smoke proof

### Reuse 3: Keep M034’s release-lane boundary intact

Do not spend M036 on:

- VSIX publishing workflow reshaping
- artifact naming work
- marketplace release plumbing

Those are already proven.

### Reuse 4: Treat shared grammar as shared infrastructure

Any TextMate parity fix should be planned knowing it affects:

- VS Code extension behavior
- docs-site Shiki highlighting

That makes a corpus-driven regression check more valuable than a narrow extension-only test.

## Boundary Contracts That Matter

### 1. Interpolation truth contract

Until the compiler removes `${...}`, editor support must handle both forms.

But because docs and dogfood already prefer `#{...}`, editor/docs surfaces should treat `#{...}` as preferred.

### 2. Shared grammar contract

`tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json` is not just a VS Code asset. It is also website highlighting infrastructure.

### 3. LSP backend contract

Multi-editor support must continue to use `meshc lsp` over stdin/stdout JSON-RPC. No separate Neovim-specific server path.

### 4. Support-tier contract

After M036, docs should clearly distinguish:

- first-class: VS Code, Neovim
- best-effort or not-yet-first-class: other editors

### 5. Smallest-honest Neovim contract

“First-class” for M036 should mean:

- repo-owned install path
- repo-owned runtime assets
- syntax support
- `meshc lsp` setup
- regression/smoke checks

It should **not** automatically mean marketplace/plugin-manager distribution to every ecosystem.

## Recommended Slice Boundaries

### Slice 1 — Syntax truth audit + corpus harness

**Goal:** define what editor parity actually means before changing artifacts.

Include:

- corpus manifest spanning `reference-backend/`, `mesher/`, docs examples, and representative tests
- explicit interpolation contract: both supported, `#{}` preferred
- lower-level lexer/parser coverage for `#{}` so internal truth surfaces stop lying by omission
- a machine-checkable grammar test harness for representative scope expectations

**Why first:** this is the risk-retirement slice. It reveals whether interpolation is the only parity gap.

### Slice 2 — VS Code/shared-grammar parity

**Goal:** fix the already-shipping parity bug and put the grammar under regression.

Include:

- TextMate grammar updates for `#{}` while preserving `${}`
- regression checks over the audited corpus
- README/tooling page truth sync for syntax claims
- website inheritance check because the docs site consumes the same grammar

**Why second:** it repairs the current production drift with the smallest blast radius and highest visible value.

### Slice 3 — Repo-owned Neovim support pack

**Goal:** make one non-VSCode editor first-class under a repo-owned path.

Minimum shape should be a runtime pack with:

- filetype detection for `*.mpl`
- syntax support via the chosen honest artifact
- LSP config for `meshc lsp`
- installation docs
- smoke checks

**Implementation choice to decide during planning:**

- start with a short spike comparing classic Vim syntax vs Tree-sitter against the corpus
- choose Tree-sitter only if regex/Vim syntax would be dishonest or too brittle for the required corpus

### Slice 4 — Real-editor proof + support-tier docs cleanup

**Goal:** stop relying on artifact-only confidence.

Include:

- a real VS Code smoke flow
- a real Neovim smoke flow
- public docs/README support-tier cleanup
- removal of any remaining blanket “other editors can just wire it up” wording that lacks repo-owned proof

## Known Failure Modes That Should Shape Ordering

1. **Fixing only the visible interpolation regex** and missing other syntax drift the corpus would have exposed.
2. **Reusing M034’s extension proof as if it were parity proof.** It is not.
3. **Starting with Tree-sitter before the corpus audit,** turning the milestone into a grammar-engine project instead of an editor-truth project.
4. **Trying to make every editor equally first-class now.** That will sprawl immediately.
5. **Migrating all docs/examples to `#{}` without preserving legacy truth,** which would misdocument the currently supported compiler behavior.
6. **Leaving support tiers vague,** which recreates the current “best-effort looks official” problem.

## Requirements Read-Through

Relevant existing requirements remain the right anchor:

- **R006** — daily-driver tooling credibility
- **R008** — docs/examples must tell the truth about real workflows
- **R010** — Mesh must point to concrete DX advantages, not vague rhetoric

### What is still missing or under-specified

These are good **candidate requirements**, not auto-binding scope expansions.

#### Candidate requirement A — Editor support must be tiered and explicit

The repo should name which editors are first-class, which are best-effort, and what that means operationally.

Why this may deserve requirement status: it is the cleanest way to prevent future overclaiming.

#### Candidate requirement B — Supported editor syntax must be proven against a non-toy corpus

Not just hand-picked snippets and not only grammar-library tests.

Why this may deserve requirement status: it directly closes the parity drift class that created M036.

#### Candidate requirement C — The first-class non-VSCode editor path must be repo-owned and installable without folklore

Neovim is the chosen proof target for M036.

Why this may deserve requirement status: it turns D072’s milestone-level intent into a concrete acceptance bar.

### Clearly out of scope

- equal first-class support for Emacs, Helix, Zed, Sublime, and others in this milestone
- redoing VS Code release/publish hardening from M034
- broad new LSP feature work unrelated to parity/support truth
- ecosystem/package-manager polish for editors beyond the repo-owned support pack

## Bottom Line for the Roadmap Planner

The milestone should be planned as **truth repair plus one new first-class editor path**, not as “build a bunch of editor integrations.”

The decisive facts are:

- the compiler already supports both interpolation syntaxes
- real dogfood code has moved to `#{}`
- the shipped grammar still only highlights `${}`
- the website consumes the same grammar
- there is no repo-owned Neovim support surface yet
- the existing proof lane covers packaging and LSP transport, not syntax parity or multi-editor installability

So the best roadmap shape is:

1. **audit and freeze syntax truth on a real corpus**
2. **repair VS Code/shared TextMate parity**
3. **add the smallest honest repo-owned Neovim pack**
4. **close with real-editor smoke proof and support-tier doc truth**

That sequence retires the hidden-risk part first, fixes the already-shipping drift second, and keeps the new multi-editor work bounded enough to finish honestly.
