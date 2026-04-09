# S03: Explicit support tiers and real editor proof in public docs — UAT

**Milestone:** M036
**Written:** 2026-03-28T07:12:12.448Z

# S03: Explicit support tiers and real editor proof in public docs — UAT

**Milestone:** M036
**Written:** 2026-03-28T23:49:09-04:00

## UAT: Explicit support tiers and real editor proof in public docs

### Preconditions
- Repo checkout contains the S03 changes.
- `target/debug/meshc` exists so the VS Code smoke can pin `mesh.lsp.path` to the repo-local compiler.
- Website dependencies and `tools/editors/vscode-mesh` dependencies are installed.
- The repo-local Neovim vendor binary exists at `.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim`.
- The repo can write verification artifacts under `.tmp/m036-s03/` and `.tmp/m036-s02/`.

### Test 1: Public tooling docs publish one explicit support-tier contract
1. Open `website/docs/docs/tooling/index.md`.
2. Open `tools/editors/vscode-mesh/README.md` and `tools/editors/neovim-mesh/README.md`.
3. Review the editor support sections and verification references.

**Expected:**
- The tooling page contains one explicit support-tier table naming **VS Code** and **Neovim** as **First-class** and editors like **Emacs, Helix, Zed, Sublime Text, and TextMate reuse** as **Best-effort**.
- The tooling page has separate sections for VS Code, Neovim, and best-effort editors instead of broad "other editors" wording.
- The VS Code README says VS Code is first-class and points back to `https://meshlang.dev/docs/tooling/` for the support contract.
- The Neovim README says Neovim is first-class but keeps claims bounded to the verified classic syntax plus native `meshc lsp` path.
- All three public surfaces reference `bash scripts/verify-m036-s03.sh` as the repo-root proof chain.

### Test 2: The docs/support-tier contract fails closed mechanically
1. Run:
   ```bash
   node --test scripts/tests/verify-m036-s03-contract.test.mjs && \
   python3 scripts/lib/m034_public_surface_contract.py local-docs --root "$PWD"
   ```
2. Observe the node:test output and the inherited M034 tooling-page helper output.

**Expected:**
- The node:test suite passes its current-repo contract checks.
- The node:test suite also includes negative cases for removing the support-tier heading, reintroducing stale "Other Editors" wording, and reverting the Neovim README to a non-first-class statement.
- The inherited M034 tooling-page contract still passes, proving S03 did not break the existing public tooling markers.

### Test 3: VS Code editor-host smoke proves a pinned repo-local compiler on real backend files
1. Run:
   ```bash
   npm --prefix tools/editors/vscode-mesh run test:smoke
   ```
2. Inspect the terminal output and `.tmp/m036-s03/vscode-smoke/smoke.log`.

**Expected:**
- The smoke launches an Extension Development Host through `@vscode/test-electron`, not a user-installed `code` binary.
- The smoke records `mesh.lsp.path=/Users/sn0w/Documents/dev/mesh-lang/target/debug/meshc` (or the repo-local equivalent for this checkout) and reports that the extension resolved the compiler from **configuration**.
- `reference-backend/api/health.mpl` and `reference-backend/api/jobs.mpl` open as `languageId=mesh` and reach clean diagnostics.
- The hover probe returns non-empty Mesh type content (`Result<Job, String>` on the current reference file), and the definition probe resolves to the `create_job_response` definition in `reference-backend/api/jobs.mpl`.
- `.tmp/m036-s03/vscode-smoke/context.json` and `.tmp/m036-s03/vscode-smoke/smoke.log` are written for postmortem debugging.

### Test 4: The repo-root S03 verifier assembles the full public proof chain
1. Run:
   ```bash
   bash scripts/verify-m036-s03.sh
   ```
2. Observe the phase banners and inspect the written artifact files.

**Expected:**
- The wrapper runs phases in this order: `docs-contract`, `docs-build`, `vsix-proof`, `vscode-smoke`, `neovim`.
- The wrapper stops on the first failing phase if any phase breaks.
- On success, `.tmp/m036-s03/status.txt` contains `ok` and `.tmp/m036-s03/current-phase.txt` contains `complete`.
- `.tmp/m036-s03/vscode-smoke/smoke.log` contains `Extension Development Host smoke passed`.
- `.tmp/m036-s02/all/neovim-smoke.log` contains both `[m036-s02] phase=syntax result=pass` and `[m036-s02] phase=lsp result=pass`.
- The wrapper reports the retained artifact roots for `.tmp/m036-s03`, `.tmp/m034-s04/verify`, `.tmp/m036-s03/vscode-smoke`, and `.tmp/m036-s02/all`.

### Test 5: Wrapper negative cases stay fail-closed and attributable
1. Run:
   ```bash
   node --test scripts/tests/verify-m036-s03-wrapper.test.mjs
   ```
2. Review the passing test list.

**Expected:**
- The happy-path wrapper test passes.
- Negative tests prove the wrapper fails closed when the docs-contract input is missing, when `tools/editors/vscode-mesh/package.json` lacks `test:smoke`, when the VS Code smoke fails, and when the documented Neovim vendor override is missing.
- The negative tests verify that downstream phases do not continue after an upstream failure and that artifact paths remain inspectable.

### Edge Cases
- Best-effort editors must remain best-effort unless they gain a repo-owned install/runtime path and verifier; reintroducing broad wording like "other editors" should fail the contract test.
- The VS Code smoke may retry once only if the Extension Development Host exits before `[suite] Starting VS Code smoke suite` appears in `smoke.log`; any failure after the suite-start marker is a real smoke failure and must stay red.
- If the repo-local Neovim binary at `.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim` is missing, `bash scripts/verify-m036-s03.sh` must fail in the `neovim` phase before claiming a green public contract.
- If the VSIX proof or VS Code smoke exits 0 without writing its expected downstream marker/artifact, the repo-root wrapper must still fail during post-checks instead of trusting the exit code alone.
