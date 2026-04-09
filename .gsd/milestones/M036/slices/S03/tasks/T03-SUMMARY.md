---
id: T03
parent: S03
milestone: M036
provides: []
requires: []
affects: []
key_files: ["scripts/verify-m036-s03.sh", "scripts/tests/verify-m036-s03-contract.test.mjs", "scripts/tests/verify-m036-s03-wrapper.test.mjs", "website/docs/docs/tooling/index.md", "tools/editors/vscode-mesh/README.md", "tools/editors/neovim-mesh/README.md"]
key_decisions: ["Treat replayed upstream artifact markers as part of the acceptance contract instead of trusting command exit code alone."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the updated docs contract and wrapper semantics with node --test scripts/tests/verify-m036-s03-contract.test.mjs scripts/tests/verify-m036-s03-wrapper.test.mjs, including the required negative cases. Then ran bash scripts/verify-m036-s03.sh from the repo root and confirmed it completed successfully through docs-contract, docs-build, vsix-proof, vscode-smoke, and neovim. After the successful end-to-end run, inspected .tmp/m036-s03/status.txt, .tmp/m036-s03/current-phase.txt, .tmp/m036-s03/vscode-smoke.log, .tmp/m036-s03/neovim.log, and the .tmp/m036-s03 file inventory to confirm the new observability surface was written as documented."
completed_at: 2026-03-28T07:01:15.058Z
blocker_discovered: false
---

# T03: Assembled a fail-closed repo-root S03 verifier that ties the public docs contract to real VS Code packaging/smoke proof and the Neovim replay.

> Assembled a fail-closed repo-root S03 verifier that ties the public docs contract to real VS Code packaging/smoke proof and the Neovim replay.

## What Happened
---
id: T03
parent: S03
milestone: M036
key_files:
  - scripts/verify-m036-s03.sh
  - scripts/tests/verify-m036-s03-contract.test.mjs
  - scripts/tests/verify-m036-s03-wrapper.test.mjs
  - website/docs/docs/tooling/index.md
  - tools/editors/vscode-mesh/README.md
  - tools/editors/neovim-mesh/README.md
key_decisions:
  - Treat replayed upstream artifact markers as part of the acceptance contract instead of trusting command exit code alone.
duration: ""
verification_result: passed
completed_at: 2026-03-28T07:01:15.058Z
blocker_discovered: false
---

# T03: Assembled a fail-closed repo-root S03 verifier that ties the public docs contract to real VS Code packaging/smoke proof and the Neovim replay.

**Assembled a fail-closed repo-root S03 verifier that ties the public docs contract to real VS Code packaging/smoke proof and the Neovim replay.**

## What Happened

Added scripts/verify-m036-s03.sh as the repo-root S03 acceptance wrapper with named phases for docs-contract, docs-build, vsix-proof, vscode-smoke, and neovim. The wrapper now preserves per-phase logs under .tmp/m036-s03/, enforces explicit timeouts, surfaces upstream artifact roots on failure, and post-checks concrete downstream proof markers so partial success cannot pass silently. Added scripts/tests/verify-m036-s03-wrapper.test.mjs to cover the happy path plus missing contract input, missing VS Code smoke script, VS Code smoke failure, and missing Neovim vendor override. Updated the tooling page and both editor READMEs to reference bash scripts/verify-m036-s03.sh alongside the narrower editor-local commands, and tightened scripts/tests/verify-m036-s03-contract.test.mjs so those public references fail closed if they drift.

## Verification

Verified the updated docs contract and wrapper semantics with node --test scripts/tests/verify-m036-s03-contract.test.mjs scripts/tests/verify-m036-s03-wrapper.test.mjs, including the required negative cases. Then ran bash scripts/verify-m036-s03.sh from the repo root and confirmed it completed successfully through docs-contract, docs-build, vsix-proof, vscode-smoke, and neovim. After the successful end-to-end run, inspected .tmp/m036-s03/status.txt, .tmp/m036-s03/current-phase.txt, .tmp/m036-s03/vscode-smoke.log, .tmp/m036-s03/neovim.log, and the .tmp/m036-s03 file inventory to confirm the new observability surface was written as documented.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m036-s03-contract.test.mjs scripts/tests/verify-m036-s03-wrapper.test.mjs` | 0 | ✅ pass | 9387ms |
| 2 | `bash scripts/verify-m036-s03.sh` | 0 | ✅ pass | 76500ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `scripts/verify-m036-s03.sh`
- `scripts/tests/verify-m036-s03-contract.test.mjs`
- `scripts/tests/verify-m036-s03-wrapper.test.mjs`
- `website/docs/docs/tooling/index.md`
- `tools/editors/vscode-mesh/README.md`
- `tools/editors/neovim-mesh/README.md`


## Deviations
None.

## Known Issues
None.
