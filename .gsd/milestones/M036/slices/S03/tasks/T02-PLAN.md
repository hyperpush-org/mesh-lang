---
estimated_steps: 4
estimated_files: 6
skills_used:
  - test
  - debug-like-expert
---

# T02: Add a real VS Code extension-host smoke with explicit meshc path and backend probes

**Slice:** S03 — Explicit support tiers and real editor proof in public docs
**Milestone:** M036

## Description

Close the only net-new proof gap in this milestone by exercising the VS Code extension inside an Extension Development Host instead of relying only on packaging and transport-level tests.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `@vscode/test-electron` / Extension Development Host startup | Surface the launcher stderr/stdout and fail the smoke with the named setup phase instead of silently skipping editor proof. | Abort with a named startup/attach timeout and preserve the temp workspace/log path under `.tmp/m036-s03/`. | Treat partial startup or missing test entrypoints as failure, not as a passing no-op. |
| repo-local `target/debug/meshc` plus `mesh.lsp.path` override | Fail fast with an actionable “build meshc or set override” message; do not fall back to ambient PATH during proof. | Stop the smoke before any green claim if the binary never becomes usable. | Treat missing, non-executable, or wrong-path overrides as failure instead of letting the extension discover a different binary. |
| `reference-backend/` LSP probes and async editor state | Wait for clean diagnostics/activation before probing hover or definition, then fail on the exact probe that came back empty. | Fail with the named request (`hover`, `definition`, diagnostics wait, activation) and the opened source file. | Treat blank hover text, wrong definition target, or wrong `languageId` as proof drift. |

## Load Profile

- **Shared resources**: One Extension Development Host, one `meshc lsp` subprocess, and a temporary workspace/settings file.
- **Per-operation cost**: Startup of the VS Code test host plus a handful of editor/LSP requests on one real Mesh file; moderate but bounded.
- **10x breakpoint**: repeated host downloads/startups and duplicated LSP attaches. The smoke should stay scoped to one deterministic workspace/file so failures remain attributable.

## Negative Tests

- **Malformed inputs**: Missing repo-local `meshc`, wrong `mesh.lsp.path`, or malformed test workspace settings must fail loudly.
- **Error paths**: A `.mpl` document that never activates the extension, never reaches `languageId = mesh`, or never returns a real LSP result must fail the smoke.
- **Boundary conditions**: Position probes must be derived from current file contents at runtime rather than hardcoded line numbers, so source drift in `reference-backend/` becomes a readable test failure instead of a silent false green.

## Steps

1. Add an `@vscode/test-electron` runner under `tools/editors/vscode-mesh/src/test/` that launches the extension in an Extension Development Host without requiring a user-installed `code` binary.
2. Implement a smoke suite that opens a real Mesh file from `reference-backend/`, sets an explicit `mesh.lsp.path` to the repo-local compiler, asserts the document opens as `languageId = mesh`, waits for clean attach/diagnostics, and exercises at least one real LSP behavior (`hover` or `definition`, with signature help optional).
3. Derive probe locations from source text at runtime using the same “find marker text first” pattern already proven in `compiler/meshc/tests/e2e_lsp.rs` so the editor smoke stays resilient to line drift.
4. Wire `package.json`, `package-lock.json`, and `tsconfig.json` so the smoke compiles and runs through a repo-owned `npm` script, and emit attributable temp logs/artifacts under `.tmp/m036-s03/vscode-smoke/` or equivalent.

## Must-Haves

- [ ] The smoke runs through an Extension Development Host, not a user’s globally installed VS Code binary.
- [ ] `mesh.lsp.path` is pinned explicitly so proof cannot pass through PATH or workspace-root luck.
- [ ] The smoke opens a real `reference-backend/` Mesh file and proves at least one editor-facing LSP behavior after attach.
- [ ] Failures name the broken phase or probe instead of only surfacing a generic non-zero exit code.

## Verification

- `npm --prefix tools/editors/vscode-mesh run test:smoke`
- The smoke only passes after an Extension Development Host opens a real `.mpl` document, the Mesh extension attaches with the intended `meshc` path, and at least one real backend-shaped LSP probe returns the expected result.

## Observability Impact

- Signals added/changed: the smoke should log the opened file, resolved `meshc` path, attach or diagnostic wait, and the exact LSP probe being asserted.
- How a future agent inspects this: run `npm --prefix tools/editors/vscode-mesh run test:smoke` and inspect the emitted temp workspace or log path under `.tmp/m036-s03/`.
- Failure state exposed: missing compiler binary, failed extension activation, failed attach, or wrong hover or definition target becomes visible without interactive VS Code debugging.

## Inputs

- `tools/editors/vscode-mesh/package.json` — current extension scripts and dependencies surface that lacks an editor-host smoke entrypoint.
- `tools/editors/vscode-mesh/package-lock.json` — lockfile that must capture any new test dependency deterministically.
- `tools/editors/vscode-mesh/tsconfig.json` — compile surface that should include new test sources without a second TypeScript config unless needed.
- `tools/editors/vscode-mesh/src/extension.ts` — activation and `meshc` discovery logic the smoke should exercise rather than bypass.
- `compiler/meshc/tests/e2e_lsp.rs` — existing runtime-derived probe pattern and backend-shaped LSP truth to mirror.
- `reference-backend/api/jobs.mpl` — stable backend-shaped file for hover, definition, or signature probes.
- `reference-backend/api/health.mpl` — clean-open file that can also prove language activation and diagnostics behavior.

## Expected Output

- `tools/editors/vscode-mesh/package.json` — extension scripts and dependencies updated with a repo-owned smoke command.
- `tools/editors/vscode-mesh/package-lock.json` — lockfile updated for the editor-host smoke dependency set.
- `tools/editors/vscode-mesh/tsconfig.json` — TypeScript config adjusted so the smoke sources compile into `out/test/**`.
- `tools/editors/vscode-mesh/src/test/runTest.ts` — Extension Development Host launcher for the Mesh smoke suite.
- `tools/editors/vscode-mesh/src/test/suite/index.ts` — suite bootstrap that discovers and runs the VS Code smoke tests.
- `tools/editors/vscode-mesh/src/test/suite/extension.test.ts` — real editor-host smoke covering activation, deterministic `meshc` resolution, and at least one backend-shaped LSP probe.
