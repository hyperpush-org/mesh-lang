---
id: T02
parent: S03
milestone: M036
provides: []
requires: []
affects: []
key_files: ["tools/editors/vscode-mesh/package.json", "tools/editors/vscode-mesh/package-lock.json", "tools/editors/vscode-mesh/src/extension.ts", "tools/editors/vscode-mesh/src/test/runTest.ts", "tools/editors/vscode-mesh/src/test/suite/index.ts", "tools/editors/vscode-mesh/src/test/suite/extension.test.ts", ".gsd/milestones/M036/slices/S03/tasks/T02-SUMMARY.md"]
key_decisions: ["Treat non-default mesh.lsp.path as authoritative so the smoke cannot pass through workspace or PATH fallback.", "Expose the resolved meshc path/source through extension activation exports so the Extension Development Host smoke can assert it used the configured compiler binary."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran npm --prefix tools/editors/vscode-mesh run compile and npm --prefix tools/editors/vscode-mesh run test:smoke. The smoke launched a real Extension Development Host via @vscode/test-electron, confirmed mesh.lsp.path was pinned to /Users/sn0w/Documents/dev/mesh-lang/target/debug/meshc, confirmed the extension resolved that path from configuration, observed clean diagnostics on reference-backend/api/health.mpl and reference-backend/api/jobs.mpl, returned a non-empty hover (Result<Job, String>) for create_job_response(job, body), and resolved definition to reference-backend/api/jobs.mpl:33."
completed_at: 2026-03-28T06:50:09.878Z
blocker_discovered: false
---

# T02: Added a repo-owned VS Code Extension Development Host smoke that pins meshc to target/debug and proves real hover/definition behavior on reference-backend Mesh files.

> Added a repo-owned VS Code Extension Development Host smoke that pins meshc to target/debug and proves real hover/definition behavior on reference-backend Mesh files.

## What Happened
---
id: T02
parent: S03
milestone: M036
key_files:
  - tools/editors/vscode-mesh/package.json
  - tools/editors/vscode-mesh/package-lock.json
  - tools/editors/vscode-mesh/src/extension.ts
  - tools/editors/vscode-mesh/src/test/runTest.ts
  - tools/editors/vscode-mesh/src/test/suite/index.ts
  - tools/editors/vscode-mesh/src/test/suite/extension.test.ts
  - .gsd/milestones/M036/slices/S03/tasks/T02-SUMMARY.md
key_decisions:
  - Treat non-default mesh.lsp.path as authoritative so the smoke cannot pass through workspace or PATH fallback.
  - Expose the resolved meshc path/source through extension activation exports so the Extension Development Host smoke can assert it used the configured compiler binary.
duration: ""
verification_result: passed
completed_at: 2026-03-28T06:50:09.879Z
blocker_discovered: false
---

# T02: Added a repo-owned VS Code Extension Development Host smoke that pins meshc to target/debug and proves real hover/definition behavior on reference-backend Mesh files.

**Added a repo-owned VS Code Extension Development Host smoke that pins meshc to target/debug and proves real hover/definition behavior on reference-backend Mesh files.**

## What Happened

Added @vscode/test-electron plus a repo-owned test:smoke entrypoint for tools/editors/vscode-mesh. Implemented a runTest launcher that builds an isolated workspace under .tmp/m036-s03/vscode-smoke, pins mesh.lsp.path to the repo-local target/debug/meshc binary, and preserves log/context artifacts for future debugging. Updated the VS Code extension so an explicit non-default mesh.lsp.path is treated as authoritative, logs the resolved path/source, throws a named startup failure when that path cannot start meshc lsp, and returns the resolved path/source from activation. Added an Extension Development Host smoke suite that opens real reference-backend/api/health.mpl and reference-backend/api/jobs.mpl files, asserts languageId=mesh, waits for clean diagnostics, derives hover/definition probe positions from current source text, and proves real editor-facing hover and definition results against backend-shaped code.

## Verification

Ran npm --prefix tools/editors/vscode-mesh run compile and npm --prefix tools/editors/vscode-mesh run test:smoke. The smoke launched a real Extension Development Host via @vscode/test-electron, confirmed mesh.lsp.path was pinned to /Users/sn0w/Documents/dev/mesh-lang/target/debug/meshc, confirmed the extension resolved that path from configuration, observed clean diagnostics on reference-backend/api/health.mpl and reference-backend/api/jobs.mpl, returned a non-empty hover (Result<Job, String>) for create_job_response(job, body), and resolved definition to reference-backend/api/jobs.mpl:33.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix tools/editors/vscode-mesh run compile` | 0 | ✅ pass | 2862ms |
| 2 | `npm --prefix tools/editors/vscode-mesh run test:smoke` | 0 | ✅ pass | 78400ms |


## Deviations

No tsconfig.json edit was needed because the existing rootDir/include settings already compiled src/test/** into out/test/**.

## Known Issues

None.

## Files Created/Modified

- `tools/editors/vscode-mesh/package.json`
- `tools/editors/vscode-mesh/package-lock.json`
- `tools/editors/vscode-mesh/src/extension.ts`
- `tools/editors/vscode-mesh/src/test/runTest.ts`
- `tools/editors/vscode-mesh/src/test/suite/index.ts`
- `tools/editors/vscode-mesh/src/test/suite/extension.test.ts`
- `.gsd/milestones/M036/slices/S03/tasks/T02-SUMMARY.md`


## Deviations
No tsconfig.json edit was needed because the existing rootDir/include settings already compiled src/test/** into out/test/**.

## Known Issues
None.
