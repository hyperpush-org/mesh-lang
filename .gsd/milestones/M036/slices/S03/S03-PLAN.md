# S03: Explicit support tiers and real editor proof in public docs

**Goal:** Publish a truthful editor support contract for Mesh by making VS Code and Neovim the only explicitly first-class editors in public docs, adding a repo-owned VS Code editor-host smoke, and assembling one repo-root verifier that keeps those claims mechanically honest.
**Demo:** After this: A developer can read the tooling docs, see exactly which editors are first-class versus best-effort, and follow the published VS Code and Neovim workflows with smoke proof backing the claims.

## Tasks
- [x] **T01: Published explicit first-class vs best-effort editor support tiers across tooling docs and editor READMEs, backed by a fail-closed contract test.** — Make the public truth surface explicit before adding new proof so the repo stops overclaiming editor support.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `website/docs/docs/tooling/index.md` plus existing M034 public-surface markers | Keep the S03 wording change bounded and fail the new contract test if the tooling page drops any required installer/runbook markers. | Not applicable — local file assertions only. | Treat ambiguous or missing support-tier wording as failure instead of accepting broad phrases like “other editors” or “most editors”. |
| `tools/editors/vscode-mesh/README.md` and `tools/editors/neovim-mesh/README.md` | Stop on wording drift and point at the exact file that still overclaims or omits the public tier. | Not applicable — local file assertions only. | Treat mismatched tier names or stale Neovim caveats as failure, not best-effort pass. |
| `website/docs/.vitepress/config.mts` and the shared grammar/proof surfaces from S01/S02 | Keep docs copy anchored to existing proof surfaces instead of inventing new syntax/editor promises. | Not applicable — local file review only. | Treat references that widen past shared TextMate parity or Neovim’s classic-syntax boundary as contract drift. |

## Load Profile

- **Shared resources**: Static markdown/README truth surfaces only.
- **Per-operation cost**: A few local file reads and one Node contract test; trivial.
- **10x breakpoint**: Copy drift across multiple truth surfaces, not compute. The contract test should localize failures by file and promise instead of by a generic snapshot diff.

## Negative Tests

- **Malformed inputs**: Missing support-tier headings/table rows, stale “Other Editors” wording, or README language that still withholds Neovim’s public tier must fail.
- **Error paths**: Removing required M034 tooling markers while editing the page must fail the same task instead of waiting for a later docs build.
- **Boundary conditions**: VS Code and Neovim remain first-class; Emacs/Helix/Zed/Sublime/TextMate reuse remain best-effort and must not be promoted by implication.

## Steps

1. Update `website/docs/docs/tooling/index.md` to define first-class vs best-effort explicitly, split editor guidance into VS Code / Neovim / best-effort sections, and bound the format-on-save plus LSP configuration copy to those tiers.
2. Update `tools/editors/vscode-mesh/README.md` so it calls VS Code first-class, points back to the tooling page for the tier contract, and stays scoped to the VS Code install/run path instead of speaking for all editors.
3. Update `tools/editors/neovim-mesh/README.md` so it graduates from the S02-local caveat to the exact public first-class promise while keeping claims limited to the classic syntax + native `meshc lsp` path already proven in S02.
4. Add `scripts/tests/verify-m036-s03-contract.test.mjs` to fail closed on support-tier wording across the tooling page plus both READMEs while preserving the existing M034 tooling-page contract.

## Must-Haves

- [ ] Public docs define first-class vs best-effort in one explicit place.
- [ ] Neovim is documented as first-class, not as a generic “other editor” setup.
- [ ] Best-effort editors stay clearly bounded to generic LSP/TextMate reuse without repo-owned smoke.
- [ ] A repo-owned contract test catches wording drift before the full slice wrapper runs.
  - Estimate: 1.5h
  - Files: website/docs/docs/tooling/index.md, tools/editors/vscode-mesh/README.md, tools/editors/neovim-mesh/README.md, scripts/tests/verify-m036-s03-contract.test.mjs
  - Verify: node --test scripts/tests/verify-m036-s03-contract.test.mjs && python3 scripts/lib/m034_public_surface_contract.py local-docs --root "$PWD"
- [x] **T02: Added a repo-owned VS Code Extension Development Host smoke that pins meshc to target/debug and proves real hover/definition behavior on reference-backend Mesh files.** — Close the only net-new proof gap in this milestone by exercising the VS Code extension inside an Extension Development Host instead of relying only on packaging and transport-level tests.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `@vscode/test-electron` / Extension Development Host startup | Surface the launcher stderr/stdout and fail the smoke with the named setup phase instead of silently skipping editor proof. | Abort with a named startup/attach timeout and preserve the temp workspace/log path under `.tmp/m036-s03/`. | Treat partial startup or missing test entrypoints as failure, not as a passing no-op. |
| repo-local `target/debug/meshc` plus `mesh.lsp.path` override | Fail fast with an actionable “build meshc or set override” message; do not fall back to ambient PATH during proof. | Stop the smoke before any green claim if the binary never becomes usable. | Treat missing, non-executable, or wrong-path overrides as failure instead of letting the extension discover a different binary. |
| `reference-backend/` LSP probes and async editor state | Wait for clean diagnostics/activation before probing hover or definition, then fail on the exact probe that came back empty. | Fail with the named request (`hover`, `definition`, diagnostics wait, activation) and the opened source file. | Treat blank hover text, wrong definition target, or wrong `languageId` as proof drift. |

## Load Profile

- **Shared resources**: One Extension Development Host, one `meshc lsp` subprocess, and a temporary workspace/settings file.
- **Per-operation cost**: Startup of the VS Code test host plus a handful of editor/LSP requests on one real Mesh file; moderate but bounded.
- **10x breakpoint**: repeated host downloads/startups and duplicated LSP attachs. The smoke should stay scoped to one deterministic workspace/file so failures remain attributable.

## Negative Tests

- **Malformed inputs**: Missing repo-local `meshc`, wrong `mesh.lsp.path`, or malformed test workspace settings must fail loudly.
- **Error paths**: A `.mpl` document that never activates the extension, never reaches `languageId = mesh`, or never returns a real LSP result must fail the smoke.
- **Boundary conditions**: Position probes must be derived from current file contents at runtime rather than hardcoded line numbers, so doc drift in `reference-backend/` becomes a readable test failure instead of a silent false green.

## Steps

1. Add an `@vscode/test-electron` runner under `tools/editors/vscode-mesh/src/test/` that launches the extension in an Extension Development Host without requiring a user-installed `code` binary.
2. Implement a smoke suite that opens a real Mesh file from `reference-backend/`, sets an explicit `mesh.lsp.path` to the repo-local compiler, asserts the document opens as `languageId = mesh`, waits for clean attach/diagnostics, and exercises at least one real LSP behavior (`hover` or `definition`, with signature help optional).
3. Derive probe locations from source text at runtime using the same “find marker text first” pattern already proven in `compiler/meshc/tests/e2e_lsp.rs` so the editor smoke stays resilient to line drift.
4. Wire `package.json` / `package-lock.json` / `tsconfig.json` so the smoke compiles and runs through a repo-owned `npm` script, and emit attributable temp logs/artifacts under `.tmp/m036-s03/vscode-smoke/` or equivalent.

## Must-Haves

- [ ] The smoke runs through an Extension Development Host, not a user’s globally installed VS Code binary.
- [ ] `mesh.lsp.path` is pinned explicitly so proof cannot pass through PATH or workspace-root luck.
- [ ] The smoke opens a real `reference-backend/` Mesh file and proves at least one editor-facing LSP behavior after attach.
- [ ] Failures name the broken phase or probe instead of only surfacing a generic non-zero exit code.
  - Estimate: 2h
  - Files: tools/editors/vscode-mesh/package.json, tools/editors/vscode-mesh/package-lock.json, tools/editors/vscode-mesh/tsconfig.json, tools/editors/vscode-mesh/src/test/runTest.ts, tools/editors/vscode-mesh/src/test/suite/index.ts, tools/editors/vscode-mesh/src/test/suite/extension.test.ts
  - Verify: npm --prefix tools/editors/vscode-mesh run test:smoke
- [x] **T03: Assembled a fail-closed repo-root S03 verifier that ties the public docs contract to real VS Code packaging/smoke proof and the Neovim replay.** — Close the slice by wiring the new docs contract and VS Code smoke into one repo-root verifier, while replaying the already-owned VSIX and Neovim proof surfaces that the public docs still depend on.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/tests/verify-m036-s03-contract.test.mjs` and `npm --prefix website run build` | Stop immediately on docs drift or build failure and preserve the failing phase log. | Fail with a named `docs-contract` or `docs-build` phase instead of hanging inside the wrapper. | Treat missing headings/markers or malformed rendered docs as failure. |
| `bash scripts/verify-m034-s04-extension.sh` and the new VS Code smoke | Stop the assembled verifier before claiming the published VS Code path is proven. | Abort on the first timed-out VS Code proof phase and preserve its upstream artifact path. | Treat partial packaging success or partial smoke startup as failure, not as enough proof for first-class support. |
| `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh` | Fail the named Neovim replay phase with the exact missing binary/log path. | Abort with the `neovim` phase name rather than hanging after docs and VS Code already passed. | Treat partial Neovim smoke or stale replay assumptions as failure, not as “good enough” because S02 previously passed. |

## Load Profile

- **Shared resources**: Sequential docs build, extension packaging/smoke, and Neovim replay; mainly wall-clock bound.
- **Per-operation cost**: Reuses existing verifiers plus one new Extension Development Host smoke; moderate but appropriate for a final slice acceptance command.
- **10x breakpoint**: Total runtime and debug clarity, not throughput. The wrapper must stay phase-oriented with preserved artifact paths so failures are localizable without rerunning everything blindly.

## Negative Tests

- **Malformed inputs**: Missing VS Code smoke script, missing Neovim vendor binary override, or missing contract test file must fail the wrapper before any green summary.
- **Error paths**: If docs build passes but VS Code or Neovim proof fails, the wrapper must stop on that named phase and report the retained artifact path.
- **Boundary conditions**: The assembled wrapper should exercise the same public story the docs tell: tier contract, VitePress render, packaged VS Code path, real VS Code smoke, and first-class Neovim replay.

## Steps

1. Add `scripts/verify-m036-s03.sh` with named phases for `docs-contract`, `docs-build`, `vsix-proof`, `vscode-smoke`, and `neovim`, preserving logs under `.tmp/m036-s03/` and stopping on the first failing phase.
2. Replay `bash scripts/verify-m034-s04-extension.sh` inside the wrapper so the documented VS Code package/install-local workflow remains tied to an existing repo-owned packaging proof instead of only to the new editor-host smoke.
3. Replay `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh` so the new public first-class Neovim promise stays chained to the already-owned S02 verifier.
4. Make a final pass over the tooling page and editor READMEs only if needed to point readers at the new repo-root S03 verification entrypoint without expanding the technical claims beyond what the wrapper actually runs.

## Must-Haves

- [ ] One repo-root command assembles the whole S03 proof chain and fails closed by named phase.
- [ ] The public VS Code story is backed by both existing packaging proof and the new real-editor smoke.
- [ ] The public Neovim story is backed by a replay of the S02 verifier on the documented repo-local binary path.
- [ ] The wrapper leaves enough artifact/log detail that a future agent can localize failures without interactive editors.
  - Estimate: 1.5h
  - Files: scripts/verify-m036-s03.sh, website/docs/docs/tooling/index.md, tools/editors/vscode-mesh/README.md, tools/editors/neovim-mesh/README.md
  - Verify: bash scripts/verify-m036-s03.sh
