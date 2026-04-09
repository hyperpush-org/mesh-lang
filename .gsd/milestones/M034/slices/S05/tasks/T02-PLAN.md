---
estimated_steps: 4
estimated_files: 7
skills_used:
  - vitepress
  - test
---

# T02: Reconcile local public docs, installers, and extension metadata with the release claim

**Slice:** S05 — Full public release assembly proof
**Milestone:** M034

## Description

Make every public-facing local source file say exactly what S05 will verify live. The deployed site is stale today, and the extension manifest still points at the old repository. This task updates docs, installer sources, and extension metadata so the local public contract matches the release-candidate story and gives the S05 verifier stable exact-string truth surfaces.

## Steps

1. Update `README.md`, `website/docs/docs/getting-started/index.md`, and `website/docs/docs/tooling/index.md` so the verified installer path, package-manager story, and assembled release-proof references all agree.
2. Reconfirm that `website/docs/public/install.sh` and `website/docs/public/install.ps1` carry the exact repo/install contract S05 expects to verify live: `snowdamiz/mesh-lang`, both binaries, and the public install commands.
3. Fix `tools/editors/vscode-mesh/package.json` repository/bugs URLs and `tools/editors/vscode-mesh/README.md` so the extension’s public metadata matches the current repo and verified install story.
4. Add or preserve grep-friendly strings so S05 can prove this public contract mechanically instead of relying on prose interpretation.

## Must-Haves

- [ ] Public docs no longer present source builds as the only verified install path.
- [ ] Both installer sources name `snowdamiz/mesh-lang` and `meshpkg` consistently.
- [ ] Extension repository and bugs URLs point at the current `snowdamiz/mesh-lang` repo.
- [ ] README, docs, and extension README describe the same release-proof/install contract S05 will verify live.

## Verification

- `rg -n 'meshlang.dev/install.sh|meshlang.dev/install.ps1|meshpkg --version' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md tools/editors/vscode-mesh/README.md`
- `! rg -n 'Today the verified install path is building \`meshc\` from source|mesh-lang/mesh' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md website/docs/public/install.ps1 tools/editors/vscode-mesh/package.json tools/editors/vscode-mesh/README.md`
- `node -p "require('./tools/editors/vscode-mesh/package.json').repository.url"`
- `node -p "require('./tools/editors/vscode-mesh/package.json').bugs.url"`

## Inputs

- `README.md` — top-level public install/release narrative that must match the assembled proof story.
- `website/docs/docs/getting-started/index.md` — deployed getting-started page currently known to drift on install guidance.
- `website/docs/docs/tooling/index.md` — deployed tooling page that must reflect the package-manager and extension story S05 verifies.
- `website/docs/public/install.sh` — canonical Unix installer source the public site serves.
- `website/docs/public/install.ps1` — canonical Windows installer source the public site serves.
- `tools/editors/vscode-mesh/package.json` — extension public metadata surface with the stale repo/bugs URLs.
- `tools/editors/vscode-mesh/README.md` — extension-facing install/setup doc that must match the verified public path.

## Expected Output

- `README.md` — release-candidate/install guidance aligned with the S05 proof story.
- `website/docs/docs/getting-started/index.md` — deployed install guidance aligned with the verified installer path.
- `website/docs/docs/tooling/index.md` — tooling/package-manager/extension guidance aligned with the assembled release claim.
- `website/docs/public/install.sh` — Unix installer source confirmed or updated to the exact live-proof contract.
- `website/docs/public/install.ps1` — Windows installer source confirmed or updated to the exact live-proof contract.
- `tools/editors/vscode-mesh/package.json` — extension repository and bugs URLs corrected to the current repo.
- `tools/editors/vscode-mesh/README.md` — extension setup doc aligned with the same public install contract.
