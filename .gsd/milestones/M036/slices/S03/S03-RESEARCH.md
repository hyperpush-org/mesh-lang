# S03 Research — Explicit support tiers and real editor proof in public docs

**Researched:** 2026-03-27  
**Status:** Ready for planning

## Summary

S03 directly supports **R006** (daily-driver tooling credibility), **R008** (docs/examples must tell the truth), and **R010** (concrete DX advantages instead of vague rhetoric).

The implementation gap is now mostly a **truth-surface gap**, not a new editor-capability gap:

- **S01 already owns shared VS Code/docs syntax proof** through `scripts/verify-m036-s01.sh` and the shared TextMate/Shiki corpus harness.
- **S02 already owns the repo-owned Neovim path** through `tools/editors/neovim-mesh/` and `scripts/verify-m036-s02.sh`.
- **What is still missing** is the public contract that explains support tiers honestly, plus one real **VS Code editor-host smoke** so the first-class VS Code path is backed by more than packaging + transport proof.

The slice should be planned as **docs/readme truth sync + one new VS Code proof surface + one slice wrapper that composes existing proof**. Do not reopen release/publish hardening from M034 and do not widen support claims beyond VS Code + Neovim.

## Skills Discovered

Relevant installed skills:

- **`vitepress`** — already installed and directly relevant to the docs site. Its core guidance says to inspect `.vitepress/config.*` and theme structure before changing docs behavior; I confirmed `website/docs/.vitepress/config.mts` is the right truth surface and that the docs site still imports the shared Mesh grammar there.
- **`agent-browser`** — already installed and relevant for optional final docs-page verification. Its core rule is to prefer explicit assertions over screenshot-only browsing; if the executor verifies the built `/docs/tooling/` page in-browser, use structured assertions rather than visual inspection alone.
- **`neovim`** — installed during research from `julianobarbosa/claude-code-skills@neovim` (top search result, 103 installs) so downstream units have a dedicated Neovim reference available if needed.

No extra VS Code skill install was necessary because the repo already has an installed `vscode-extension-publisher` skill and the remaining S03 gap is extension-host smoke, not publishing.

## Implementation Landscape

### 1. Public docs truth surface

Primary file:

- `website/docs/docs/tooling/index.md`

This is the main public truth surface S03 needs to repair. Current state:

- It has a **VS Code section** with honest syntax/LSP wording inherited from S01.
- It has **no support-tier vocabulary** such as first-class vs best-effort.
- It still treats Neovim as part of generic “other editors” guidance instead of a repo-owned first-class path.
- It has multiple broad statements outside the `Editor Support` section that will recreate drift if only one section is updated.

Specific over-broad spots:

- `### Format on Save` currently says: “Most editors can be configured…” and then distinguishes only VS Code vs everything else.
- LSP `### Configuration` currently says VS Code handles startup automatically, then lumps **Neovim, Emacs, Helix, and Zed** into one generic “other editors that support LSP” bucket.
- `## Editor Support` currently has `### VS Code` and then `### Other Editors`; there is **no Neovim subsection** and no explicit support-tier definition.
- The page does **not** mention `tools/editors/neovim-mesh/README.md` or `mesh.nvim` at all.

This is the highest-leverage docs file for candidate requirements A–C without needing new requirement IDs.

### 2. Docs build surface

Relevant file:

- `website/package.json`

Facts:

- The docs site scripts live in `website/package.json`.
- There is **no** `website/docs/package.json`; executor tasks should not waste time looking for one.
- The build command is `npm --prefix website run build`.

This makes VitePress render verification straightforward and repo-local.

### 3. VitePress configuration still binds docs to the shared grammar

Relevant file:

- `website/docs/.vitepress/config.mts`

Per the `vitepress` skill rule, this is the first config surface to inspect before changing docs behavior. It still imports:

- `../../../tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json`

That means S03 should keep public docs copy tied to the **existing shared proof surfaces** rather than inventing fresh syntax claims. Syntax parity is already owned by S01; S03 should point at that verified contract, not restate it more broadly.

### 4. VS Code README is honest about features, but not about support tiers

Relevant file:

- `tools/editors/vscode-mesh/README.md`

Current role:

- Describes the extension’s syntax + LSP feature set.
- Points at the verified public installer pair for `meshc` / `meshpkg`.
- Documents local source build + `npm run package` + `npm run install-local`.

What is missing:

- No explicit “first-class supported editor” positioning.
- No link back to a canonical support-tier explanation.
- No reference to the new Neovim first-class story or the boundary that other editors are still best-effort.

This README should be aligned to S03, but it should stay scoped to the VS Code path rather than duplicating the full support-tier policy.

### 5. Neovim README deliberately withholds the public support promise

Relevant file:

- `tools/editors/neovim-mesh/README.md`

This README is still intentionally S02-local. It currently says:

- support is only for the audited classic syntax + native `meshc lsp` path
- **“No public support-tier promise beyond the repo-local proof in `scripts/verify-m036-s02.sh`.”**
- **“No broader editor/tooling contract that belongs in later S03-facing docs.”**

That makes the file the clearest S03 copy target after the tooling page. The implementation work here is not speculative: S02 already delivered the proof surface. S03’s job is to convert that repo-local truth into a public first-class support statement without broadening the technical claim.

### 6. Existing proof surfaces to reuse, not replace

Relevant files/scripts:

- `scripts/verify-m036-s01.sh`
- `scripts/verify-m036-s02.sh`
- `scripts/verify-m034-s04-extension.sh`
- `scripts/lib/m034_public_surface_contract.py`
- `scripts/tests/verify-m034-s05-contract.test.mjs`

What each already proves:

- `scripts/verify-m036-s01.sh` — shared VS Code/docs syntax parity via corpus-backed TextMate + Shiki proof.
- `scripts/verify-m036-s02.sh` — repo-owned Neovim install/runtime proof, including syntax + LSP.
- `scripts/verify-m034-s04-extension.sh` — VS Code packaging/docs handoff proof plus reused upstream `e2e_lsp` transport proof.
- `scripts/lib/m034_public_surface_contract.py` and `scripts/tests/verify-m034-s05-contract.test.mjs` — enforce existing required markers on `website/docs/docs/tooling/index.md` and other public surfaces.

Planning implication:

- **Do not re-implement these proofs inside S03.**
- Build a slice-owned wrapper that **replays** them where appropriate and adds only the missing S03-specific contract + VS Code smoke.

### 7. The only net-new proof surface is VS Code real-editor smoke

Relevant files today:

- `tools/editors/vscode-mesh/package.json`
- `tools/editors/vscode-mesh/tsconfig.json`
- `tools/editors/vscode-mesh/src/extension.ts`
- `tools/editors/vscode-mesh/scripts/vsix-path.mjs`

Current state:

- `package.json` has scripts for `compile`, `package`, `install-local`, and `test:vsix-path`.
- There is **no** extension-host smoke harness.
- There is **no** `@vscode/test-electron` dependency.
- There are **no** `extensionDevelopmentPath` / `extensionTestsPath` runners.
- There are **no** tests that exercise a real `.mpl` buffer inside an Extension Development Host.

`tsconfig.json` currently includes `"src"`, so adding `src/test/**` is cheap and should compile into `out/test/**` without a separate TypeScript config.

Official VS Code testing docs recommend `@vscode/test-electron` with:

- `extensionDevelopmentPath`
- `extensionTestsPath`
- `runTests(...)`

That is the cleanest way to add a repo-owned VS Code smoke path without relying on a user-installed `code` binary.

### 8. `install-local` is a human workflow, not a reliable proof harness

Relevant file:

- `tools/editors/vscode-mesh/scripts/vsix-path.mjs`

Important behavior:

- `install-local` packages the VSIX, then shells out to `code --install-extension ...`.

Research-time environment facts:

- `command -v code` returned **missing**.
- `command -v nvim` returned **missing**.
- The vendored Neovim from S02 is present at `.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim`.

Planning implication:

- The public docs can still keep `npm run install-local` as the human VS Code runbook.
- The repo-owned S03 proof **must not** depend on `code` being installed locally.
- Use `@vscode/test-electron` for VS Code proof and `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim` for Neovim proof on this machine.

### 9. VS Code `meshc` discovery has one critical constraint

Relevant file:

- `tools/editors/vscode-mesh/src/extension.ts`

`findMeshc()` resolves the compiler in this order:

1. explicit `mesh.lsp.path`
2. workspace-local `target/debug/meshc`
3. workspace-local `target/release/meshc`
4. well-known install paths (`~/.mesh/bin/meshc`, `/usr/local/bin/meshc`, `/opt/homebrew/bin/meshc`)
5. `PATH` fallback (`meshc`)

This matters for planning because:

- if the VS Code smoke opens only `reference-backend/` as a workspace, the extension **will not** discover repo-root `target/debug/meshc`
- the smoke must either:
  - open the repo root as workspace, **or**
  - set `mesh.lsp.path` to an explicit absolute path

**Recommended:** use an explicit `mesh.lsp.path` override in the smoke workspace for determinism. That avoids coupling the proof to workspace-folder choice or shell PATH.

### 10. There is one adjacent public-claim wrinkle worth noting

Relevant file:

- `tools/editors/vscode-mesh/package.json`

The marketplace/package description still says the extension provides:

- syntax highlighting
- diagnostics
- hover
- go-to-definition
- **completions**
- signature help

The server code does implement completion, but the current upstream proof surface (`compiler/meshc/tests/e2e_lsp.rs`) does **not** prove completion today.

This does **not** need to become mandatory S03 scope, but it is worth knowing:

- if the new VS Code smoke can cheaply add a completion assertion, that would retire one more public-claim gap
- if not, keep S03 docs/readme language bounded to the features already proven elsewhere and do not expand feature claims casually

## Natural Seams

### Seam A — support-tier contract and copy sync

Files likely touched:

- `website/docs/docs/tooling/index.md`
- `tools/editors/vscode-mesh/README.md`
- `tools/editors/neovim-mesh/README.md`

What belongs here:

- define first-class vs best-effort explicitly
- add a first-class Neovim subsection to public tooling docs
- align both editor READMEs to the same public story
- keep all claims bounded to existing proof surfaces

### Seam B — VS Code extension-host smoke

Files likely touched:

- `tools/editors/vscode-mesh/package.json`
- `tools/editors/vscode-mesh/package-lock.json`
- likely new `tools/editors/vscode-mesh/src/test/runTest.ts`
- likely new `tools/editors/vscode-mesh/src/test/suite/index.ts`
- likely new `tools/editors/vscode-mesh/src/test/suite/extension.test.ts`

What belongs here:

- add `@vscode/test-electron`-style integration test scaffolding
- open a real `.mpl` file in an Extension Development Host
- assert small, real editor behaviors rather than re-proving all backend capabilities

Recommended assertion surface:

- extension activates on a `.mpl` document
- document language is `mesh`
- `meshc lsp` attaches successfully using explicit `mesh.lsp.path`
- one or two real LSP behaviors succeed on `reference-backend/` (hover or definition is sufficient; formatting is optional but good)

Use the D089 pattern from `compiler/meshc/tests/e2e_lsp.rs`: derive symbol positions from source text at runtime instead of hardcoding line numbers.

### Seam C — slice-owned verifier and contract tests

Files likely touched:

- likely new `scripts/tests/verify-m036-s03-contract.test.mjs`
- likely new `scripts/verify-m036-s03.sh`
- possibly generated temp workspace/artifacts under `.tmp/m036-s03/`

What belongs here:

- fail-closed wording contract for public docs + READMEs
- docs build check (`npm --prefix website run build`)
- VS Code smoke invocation
- replay of S02 Neovim proof
- optional replay of M034/S04 extension packaging proof if the slice wants to keep VS Code package/install docs tied to the same upstream contract

## Recommendation

### Recommended support-tier contract

Use one explicit public definition and keep it small:

- **First-class** = repo-owned install path/docs + repo-owned proof/verifier in this repository
- **Best-effort** = generic commands may work, but Mesh does not ship editor-specific runtime assets or smoke proof for that editor in M036

That keeps candidate requirements A–C satisfied operationally without introducing new requirement IDs during this scout pass.

### Recommended docs shape

In `website/docs/docs/tooling/index.md`, add a short support-tier table near `## Editor Support`:

- **VS Code** — first-class — official extension, shared syntax parity proof, transport proof, and S03 real-editor smoke
- **Neovim** — first-class — repo-owned pack under `tools/editors/neovim-mesh/`, `scripts/verify-m036-s02.sh`, Neovim 0.11+
- **Other editors** — best-effort — generic `meshc lsp` or TextMate reuse only, no repo-owned smoke in M036

Then split the existing `Other Editors` section into:

- `### VS Code`
- `### Neovim`
- `### Best-effort editors`

Do **not** put Neovim into the generic “other editors” bucket anymore.

### Recommended VS Code smoke approach

Use `@vscode/test-electron` and a small Extension Development Host suite.

Why this is the best fit:

- official supported path from VS Code docs
- does not require `code` on PATH
- keeps proof repo-owned and deterministic
- avoids turning S03 into publish-lane work

Recommended workspace strategy:

- use an explicit `mesh.lsp.path` pointing to repo-local `target/debug/meshc`
- open a `reference-backend/` file for LSP assertions
- derive probe positions from file contents at runtime

### Recommended wrapper strategy

Keep `scripts/verify-m036-s03.sh` as an **assembly wrapper**, not a new monolith.

A good phase order is:

1. `docs-contract` — new `node --test scripts/tests/verify-m036-s03-contract.test.mjs`
2. `docs-build` — `npm --prefix website run build`
3. `vscode-smoke` — new Extension Development Host smoke
4. `neovim` — replay `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh`
5. optional upstream `vsix-proof` — replay `bash scripts/verify-m034-s04-extension.sh` if you want package/install docs to stay chained to M034’s verified VSIX contract

That preserves the M034 and S02 proof boundaries instead of cloning them.

## Forward Intelligence

- **Do not rely on `code --install-extension` for proof.** That is fine for user docs, but not for repo-owned smoke.
- **Do not hardcode line numbers in the VS Code smoke.** Follow D089’s runtime-position derivation pattern.
- **Do not widen Neovim claims.** S02 proved classic syntax + native `meshc lsp` + repo-owned install path. It did **not** prove Tree-sitter, plugin-manager distribution, or other editors.
- **Do not break M034 tooling-page markers.** `scripts/lib/m034_public_surface_contract.py` and `scripts/tests/verify-m034-s05-contract.test.mjs` already require specific tooling page strings; S03 edits must preserve them.
- **Do not duplicate the shared syntax proof inside VS Code smoke.** S01 already owns syntax parity better than an extension-host test can.
- **Tool Summary table is probably not the right place to represent Neovim.** Neovim is not a CLI tool; a support-tier table under `Editor Support` is the cleaner surface.
- **If the executor wants visible docs-page proof, use structured browser assertions.** That follows the `agent-browser` skill rule to assert outcomes explicitly instead of trusting screenshots.

## Verification

Minimum honest verification for S03:

1. `node --test scripts/tests/verify-m036-s03-contract.test.mjs`
   - docs + both editor READMEs agree on first-class vs best-effort wording
   - Neovim is documented as first-class, not generic “other editor” setup
   - best-effort wording stays bounded for Emacs/Helix/Zed/Sublime/etc.

2. `npm --prefix website run build`
   - VitePress render check for the updated tooling page

3. new VS Code smoke command (to be added in this slice)
   - runs via `@vscode/test-electron`
   - opens a real `.mpl` document
   - proves extension activation + LSP attachment + at least one real editor command result

4. `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh`
   - replays first-class Neovim proof on this host

5. `bash scripts/verify-m036-s03.sh`
   - final fail-closed slice entrypoint

Optional extra confidence:

- preview the built docs locally and assert the support-tier headings/table with browser tools
- replay `bash scripts/verify-m034-s04-extension.sh` if the slice wants to keep VSIX packaging/install doc drift caught in the same final run

## Sources

- VS Code Extension API — Testing Extensions: https://code.visualstudio.com/api/working-with-extensions/testing-extension
