---
estimated_steps: 4
estimated_files: 4
skills_used:
  - vitepress
  - test
---

# T01: Publish the support-tier contract across tooling docs and editor READMEs

**Slice:** S03 — Explicit support tiers and real editor proof in public docs
**Milestone:** M036

## Description

Make the public truth surface explicit before adding new proof so the repo stops overclaiming editor support.

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

## Verification

- `node --test scripts/tests/verify-m036-s03-contract.test.mjs && python3 scripts/lib/m034_public_surface_contract.py local-docs --root "$PWD"`
- The check only passes when the tooling page and both editor READMEs agree on tier names, Neovim is no longer buried under generic editor guidance, and the existing M034 public markers still survive the copy edit.

## Inputs

- `website/docs/docs/tooling/index.md` — current public tooling truth surface that still over-broadly groups Neovim with other editors.
- `tools/editors/vscode-mesh/README.md` — current VS Code README that needs explicit first-class positioning.
- `tools/editors/neovim-mesh/README.md` — current Neovim README that still explicitly withholds a public support-tier promise.
- `website/docs/.vitepress/config.mts` — docs-side grammar/source-of-truth wiring that should anchor syntax claims to S01 instead of new prose invention.
- `scripts/lib/m034_public_surface_contract.py` — existing M034 tooling-page marker contract that must stay green while S03 edits the page.
- `scripts/verify-m036-s01.sh` — shared VS Code/docs syntax proof the public copy should reference honestly.
- `scripts/verify-m036-s02.sh` — existing Neovim proof surface the public support promise must point at.

## Expected Output

- `website/docs/docs/tooling/index.md` — tooling page with explicit first-class vs best-effort tiers and separate VS Code / Neovim / best-effort guidance.
- `tools/editors/vscode-mesh/README.md` — VS Code README aligned to the first-class contract without speaking for unsupported editors.
- `tools/editors/neovim-mesh/README.md` — Neovim README upgraded from S02-local caveat to the bounded public first-class promise.
- `scripts/tests/verify-m036-s03-contract.test.mjs` — fail-closed docs/README contract test for S03 tier wording and cross-surface drift.
