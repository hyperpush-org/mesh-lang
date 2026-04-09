---
id: S03
parent: M036
milestone: M036
provides:
  - An explicit public editor support contract that names VS Code and Neovim as first-class and keeps all other editors best-effort unless they gain repo-owned proof.
  - A repo-owned VS Code Extension Development Host smoke path pinned to `target/debug/meshc` that proves real diagnostics, hover, and definition behavior against `reference-backend/`.
  - One repo-root acceptance command, `bash scripts/verify-m036-s03.sh`, that ties docs truth, VSIX proof, real VS Code smoke, and the Neovim replay into one fail-closed public story.
requires:
  - slice: S01
    provides: The audited shared VS Code/docs grammar contract and corpus-backed interpolation proof reused by the public support story.
  - slice: S02
    provides: The repo-owned Neovim install/runtime contract and `scripts/verify-m036-s02.sh` replay that S03 promotes into the public first-class tier.
affects:
  []
key_files:
  - website/docs/docs/tooling/index.md
  - tools/editors/vscode-mesh/README.md
  - tools/editors/neovim-mesh/README.md
  - scripts/tests/verify-m036-s03-contract.test.mjs
  - tools/editors/vscode-mesh/package.json
  - tools/editors/vscode-mesh/package-lock.json
  - tools/editors/vscode-mesh/src/extension.ts
  - tools/editors/vscode-mesh/src/test/runTest.ts
  - tools/editors/vscode-mesh/src/test/suite/index.ts
  - tools/editors/vscode-mesh/src/test/suite/extension.test.ts
  - scripts/tests/verify-m036-s03-wrapper.test.mjs
  - scripts/verify-m036-s03.sh
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - Treat first-class editor support as an operational contract: Mesh only calls an editor first-class when the repo owns both the documentation path and a verifier for that specific editor host.
  - Treat any non-default `mesh.lsp.path` as authoritative in the VS Code extension and expose the resolved path/source through activation so the editor-host smoke cannot pass through PATH or workspace fallback.
  - Treat replayed upstream artifact markers as part of the repo-root acceptance contract instead of trusting command exit code alone.
  - Retry the Extension Development Host smoke only when the suite never logged its start marker, after resetting temporary VS Code state; once the suite starts, any failure remains a real regression.
patterns_established:
  - Public support-tier language should be proof-scoped: first-class means repo-owned docs plus repo-owned verification, while best-effort means generic LSP/TextMate reuse without editor-host proof.
  - For editor-host smoke, pin the compiler path explicitly and surface the resolved path/source so proof can localize wrong-binary drift instead of silently falling back.
  - Repo-root acceptance wrappers should replay narrower verifiers by named phase, retain one shared artifact root, and post-check downstream markers/logs so partial success cannot look green.
  - When the Extension Development Host occasionally dies before the suite entrypoint runs, treat that as a launch-level flake only if the smoke log never reaches the suite-start marker; otherwise fail immediately as a real smoke regression.
observability_surfaces:
  - `scripts/verify-m036-s03.sh` phase banners plus `.tmp/m036-s03/status.txt`, `.tmp/m036-s03/current-phase.txt`, and per-phase logs under `.tmp/m036-s03/`.
  - `.tmp/m036-s03/vscode-smoke/smoke.log` and `.tmp/m036-s03/vscode-smoke/context.json`, which record the pinned `meshc` path, opened files, diagnostics waits, hover/definition probes, and pass/fail outcome.
  - The inherited `.tmp/m034-s04/verify/` VSIX proof artifacts and `.tmp/m036-s02/all/neovim-smoke.log`, which remain the downstream evidence surfaces the S03 wrapper replays and checks.
drill_down_paths:
  - .gsd/milestones/M036/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M036/slices/S03/tasks/T02-SUMMARY.md
  - .gsd/milestones/M036/slices/S03/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-28T07:12:12.448Z
blocker_discovered: false
---

# S03: Explicit support tiers and real editor proof in public docs

**Published an explicit public editor support contract, added repo-owned VS Code editor-host proof, and assembled one fail-closed repo-root verifier that keeps the VS Code and Neovim claims honest.**

## What Happened

S03 closed the credibility gap between editor work already landed in M036 and the public story Mesh tells about that work. The tooling page now defines one explicit support-tier contract: VS Code and Neovim are first-class only because Mesh owns their install/run docs plus repo-owned verification, while Emacs, Helix, Zed, Sublime Text, TextMate reuse, and similar setups remain best-effort consumers of shared `meshc lsp` or the shared TextMate grammar. The VS Code and Neovim READMEs were tightened to point back to that public contract and to stay bounded to the proof each editor actually owns instead of speaking for “other editors.”

The slice also closed the missing VS Code proof surface. `tools/editors/vscode-mesh` now has a repo-owned Extension Development Host smoke that runs through `@vscode/test-electron`, pins `mesh.lsp.path` to the repo-local `target/debug/meshc`, opens real `reference-backend/api/health.mpl` and `reference-backend/api/jobs.mpl`, waits for clean diagnostics, and proves real hover plus go-to-definition behavior against backend-shaped Mesh code. The extension now treats an explicit non-default `mesh.lsp.path` as authoritative and exposes the resolved binary path/source through activation so the smoke can fail closed instead of accidentally passing through PATH or workspace fallback. During slice closeout, I also hardened the smoke launcher against a real pre-suite Extension Development Host launch flake by resetting the temporary VS Code state and retrying once only when the suite never logged its start marker; any failure after the suite begins still fails as a real smoke regression.

Finally, S03 assembled the public proof chain into one repo-root acceptance command. `scripts/verify-m036-s03.sh` now replays the docs contract test, VitePress build, existing VSIX/public README proof from M034, the real VS Code smoke, and the Neovim replay from S02 in named phases: `docs-contract`, `docs-build`, `vsix-proof`, `vscode-smoke`, and `neovim`. The wrapper preserves phase logs under `.tmp/m036-s03/`, writes `status.txt` and `current-phase.txt`, checks downstream artifact markers instead of trusting exit codes alone, and stops on the first failing phase with the retained artifact root. For downstream readers, the key established pattern is that editor support is now operationally defined: no editor should be described as first-class unless Mesh owns both the documentation path and a repo-owned verifier for that specific editor host.

## Verification

Re-ran all slice-level acceptance commands from the repo root and confirmed they passed after hardening the VS Code smoke launcher.

- `node --test scripts/tests/verify-m036-s03-contract.test.mjs && python3 scripts/lib/m034_public_surface_contract.py local-docs --root "$PWD"` — passed; the support-tier contract stayed fail-closed and the inherited M034 tooling-page markers still held.
- `npm --prefix tools/editors/vscode-mesh run test:smoke` — passed; the Extension Development Host smoke pinned `mesh.lsp.path` to `/Users/sn0w/Documents/dev/mesh-lang/target/debug/meshc`, opened `reference-backend/api/health.mpl` and `reference-backend/api/jobs.mpl` as `languageId=mesh`, observed clean diagnostics, returned hover content `Result<Job, String>`, and resolved definition to `reference-backend/api/jobs.mpl:33`.
- `bash scripts/verify-m036-s03.sh` — passed; the wrapper completed `docs-contract`, `docs-build`, `vsix-proof`, `vscode-smoke`, and `neovim`, then wrote `.tmp/m036-s03/status.txt` = `ok`, `.tmp/m036-s03/current-phase.txt` = `complete`, `.tmp/m036-s03/vscode-smoke/smoke.log` with the pass marker, and replayed the Neovim syntax/LSP pass markers through `.tmp/m036-s02/all/neovim-smoke.log`.
- `node --test scripts/tests/verify-m036-s03-wrapper.test.mjs` — passed; the wrapper’s happy path and negative fail-closed cases for missing contract input, missing VS Code smoke script, VS Code smoke failure, and missing Neovim vendor override all remained covered.

I also verified the observability surface directly by reading `.tmp/m036-s03/status.txt`, `.tmp/m036-s03/current-phase.txt`, `.tmp/m036-s03/vscode-smoke/smoke.log`, and `.tmp/m036-s02/all/neovim-smoke.log` after the successful wrapper run.

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None. Closeout only hardened the existing VS Code smoke path with a bounded pre-suite retry and did not widen the public contract or add new proof claims.

## Known Limitations

The first-class public contract intentionally remains limited to VS Code and Neovim. Best-effort editors can still reuse `meshc lsp` or the shared TextMate grammar, but Mesh does not ship editor-host packaging, smoke proof, or troubleshooting for them. The VS Code smoke proves one representative backend-shaped hover/definition path plus clean diagnostics, not exhaustive editor UX for every Mesh feature. The Neovim side remains bounded to the classic syntax plus native `meshc lsp` path already proven in S02; it does not imply Tree-sitter support or broader plugin-manager coverage.

## Follow-ups

If future work wants to promote another editor beyond best-effort, it should follow the S03 pattern exactly: add a repo-owned install/runtime path, keep claims bounded to what is actually proven, and wire that editor into a named-phase repo-root verifier before changing the public tooling copy. Future editor-feature expansions should also extend the shared corpus and editor-host smokes together so docs wording cannot outrun proof again.

## Files Created/Modified

- `website/docs/docs/tooling/index.md` — Published the explicit first-class vs best-effort editor support tiers, bounded editor guidance by tier, and pointed readers at the repo-root verifier.
- `tools/editors/vscode-mesh/README.md` — Scoped the VS Code README to the first-class VS Code contract, packaging/install path, and repo-root verification entrypoint.
- `tools/editors/neovim-mesh/README.md` — Promoted Neovim to the explicit first-class public contract while keeping claims limited to the verified classic syntax plus native `meshc lsp` path.
- `scripts/tests/verify-m036-s03-contract.test.mjs` — Added fail-closed support-tier contract assertions and tightened them to require the repo-root verifier references in the public docs surfaces.
- `tools/editors/vscode-mesh/package.json` — Added the repo-owned `test:smoke` entrypoint and the test-electron-driven VS Code smoke path.
- `tools/editors/vscode-mesh/package-lock.json` — Captured the VS Code smoke dependencies required for deterministic local replay.
- `tools/editors/vscode-mesh/src/extension.ts` — Made explicit `mesh.lsp.path` overrides authoritative and exposed the resolved binary path/source through activation for smoke verification.
- `tools/editors/vscode-mesh/src/test/runTest.ts` — Implemented the Extension Development Host launcher, repo-local `meshc` pinning, artifact logging, and bounded retry-on-pre-suite-launch-flake behavior.
- `tools/editors/vscode-mesh/src/test/suite/index.ts` — Added the dedicated VS Code smoke suite entrypoint used by `@vscode/test-electron`.
- `tools/editors/vscode-mesh/src/test/suite/extension.test.ts` — Proved language activation, clean diagnostics, hover, and definition against real `reference-backend/` Mesh files.
- `scripts/tests/verify-m036-s03-wrapper.test.mjs` — Covered the assembled wrapper happy path and fail-closed negative cases for missing inputs and broken downstream proof surfaces.
- `scripts/verify-m036-s03.sh` — Assembled the docs contract, VitePress build, VSIX proof, real VS Code smoke, and Neovim replay into one named-phase repo-root verifier.
- `.gsd/KNOWLEDGE.md` — Recorded the Extension Development Host pre-suite launch flake pattern and the bounded retry rule that keeps the VS Code smoke honest.
- `.gsd/PROJECT.md` — Updated project state to reflect that M036 S03 closed the public editor support contract and assembled the repo-root editor proof chain.
