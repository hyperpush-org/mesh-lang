---
estimated_steps: 4
estimated_files: 4
skills_used:
  - test
  - vitepress
---

# T03: Assemble the S03 verifier and final public proof chain

**Slice:** S03 — Explicit support tiers and real editor proof in public docs
**Milestone:** M036

## Description

Close the slice by wiring the new docs contract and VS Code smoke into one repo-root verifier, while replaying the already-owned VSIX and Neovim proof surfaces that the public docs still depend on.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/tests/verify-m036-s03-contract.test.mjs` and `npm --prefix website run build` | Stop immediately on docs drift or build failure and preserve the failing phase log. | Fail with a named `docs-contract` or `docs-build` phase instead of hanging inside the wrapper. | Treat missing headings or malformed rendered docs as failure. |
| `bash scripts/verify-m034-s04-extension.sh` and the new VS Code smoke | Stop the assembled verifier before claiming the published VS Code path is proven. | Abort on the first timed-out VS Code proof phase and preserve its upstream artifact path. | Treat partial packaging success or partial smoke startup as failure, not as enough proof for first-class support. |
| `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh` | Fail the named Neovim replay phase with the exact missing binary or log path. | Abort with the `neovim` phase name rather than hanging after docs and VS Code already passed. | Treat partial Neovim smoke or stale replay assumptions as failure, not as “good enough” because S02 previously passed. |

## Load Profile

- **Shared resources**: Sequential docs build, extension packaging or smoke, and Neovim replay; mainly wall-clock bound.
- **Per-operation cost**: Reuses existing verifiers plus one new Extension Development Host smoke; moderate but appropriate for a final slice acceptance command.
- **10x breakpoint**: Total runtime and debug clarity, not throughput. The wrapper must stay phase-oriented with preserved artifact paths so failures are localizable without rerunning everything blindly.

## Negative Tests

- **Malformed inputs**: Missing VS Code smoke script, missing Neovim vendor binary override, or missing contract test file must fail the wrapper before any green summary.
- **Error paths**: If docs build passes but VS Code or Neovim proof fails, the wrapper must stop on that named phase and report the retained artifact path.
- **Boundary conditions**: The assembled wrapper should exercise the same public story the docs tell: tier contract, VitePress render, packaged VS Code path, real VS Code smoke, and first-class Neovim replay.

## Steps

1. Add `scripts/verify-m036-s03.sh` with named phases for `docs-contract`, `docs-build`, `vsix-proof`, `vscode-smoke`, and `neovim`, preserving logs under `.tmp/m036-s03/` and stopping on the first failing phase.
2. Replay `bash scripts/verify-m034-s04-extension.sh` inside the wrapper so the documented VS Code package and install-local workflow remains tied to an existing repo-owned packaging proof instead of only to the new editor-host smoke.
3. Replay `NEOVIM_BIN=.tmp/m036-s02/vendor/nvim-macos-arm64/bin/nvim bash scripts/verify-m036-s02.sh` so the new public first-class Neovim promise stays chained to the already-owned S02 verifier.
4. Update the tooling page and both editor READMEs to reference the final repo-root S03 verifier alongside the editor-specific instructions, without expanding the technical claims beyond what the wrapper actually runs.

## Must-Haves

- [ ] One repo-root command assembles the whole S03 proof chain and fails closed by named phase.
- [ ] The public VS Code story is backed by both existing packaging proof and the new real-editor smoke.
- [ ] The public Neovim story is backed by a replay of the S02 verifier on the documented repo-local binary path.
- [ ] The wrapper leaves enough artifact or log detail that a future agent can localize failures without interactive editors.

## Verification

- `bash scripts/verify-m036-s03.sh`
- The wrapper exits 0 only after the docs contract, VitePress build, VSIX/public-README proof, VS Code editor-host smoke, and Neovim replay all pass from the repo root.

## Observability Impact

- Signals added or changed: the wrapper should emit named phase banners and preserve the path to the first failing log or artifact set under `.tmp/m036-s03/`.
- How a future agent inspects this: run `bash scripts/verify-m036-s03.sh` for the full chain or the underlying component command for the named failing phase.
- Failure state exposed: docs or build drift, VSIX proof failures, VS Code smoke failures, and Neovim replay failures become attributable by phase instead of collapsing into a single opaque non-zero exit.

## Inputs

- `scripts/tests/verify-m036-s03-contract.test.mjs` — task-level docs and README contract test to make the first verifier phase fail closed.
- `website/package.json` — docs build entrypoint that the assembled wrapper must execute exactly.
- `scripts/verify-m034-s04-extension.sh` — existing VSIX packaging and install proof the public VS Code docs still rely on.
- `tools/editors/vscode-mesh/package.json` — new smoke script entrypoint to invoke from the wrapper.
- `scripts/verify-m036-s02.sh` — existing first-class Neovim verifier to replay without widening claims.
- `website/docs/docs/tooling/index.md` — final tooling page that must reference the repo-root S03 verification entrypoint.
- `tools/editors/vscode-mesh/README.md` — VS Code README that must reference the final S03 wrapper alongside its install workflow.
- `tools/editors/neovim-mesh/README.md` — Neovim README that must reference the final S03 wrapper alongside the pack-local verifier.

## Expected Output

- `scripts/verify-m036-s03.sh` — fail-closed repo-root acceptance wrapper for the full S03 support-tier and editor-proof contract.
- `website/docs/docs/tooling/index.md` — tooling docs updated to reference the final repo-root S03 proof entrypoint.
- `tools/editors/vscode-mesh/README.md` — VS Code README updated to reference the final S03 wrapper alongside its install workflow.
- `tools/editors/neovim-mesh/README.md` — Neovim README updated to reference the final S03 wrapper alongside the pack-local verifier.
