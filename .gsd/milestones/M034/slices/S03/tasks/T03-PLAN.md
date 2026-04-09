---
estimated_steps: 3
estimated_files: 4
skills_used:
  - best-practices
---

# T03: Align public install docs with the verified installer path

**Slice:** S03 — Release assets and installer truth
**Milestone:** M034

## Description

Only after the staged verifiers and release workflow smoke are in place should the public story stop telling users that building from source is the only verified path. Rewrite the top-level quick start, getting-started guide, tooling docs, and editor README around the documented installer path, and phrase platform/binary coverage precisely so the docs claim exactly what S03 now proves.

## Steps

1. Update `README.md` so the quick start uses `https://meshlang.dev/install.sh` and `https://meshlang.dev/install.ps1` as the verified install path, while keeping source-build instructions only as an explicit alternative or contributor path.
2. Update `website/docs/docs/getting-started/index.md` to replace the old source-build truth claim with installer-based instructions and verification commands for both `meshc` and `meshpkg`, using wording that matches the actual platform coverage proven by T01 and T02.
3. Update `website/docs/docs/tooling/index.md` and `tools/editors/vscode-mesh/README.md` so tooling/editor docs reference the same installer truth and do not over- or under-claim what the installer provides.

## Must-Haves

- [ ] No top-level or getting-started docs still claim that building `meshc` from source is the only verified install path.
- [ ] Public docs say plainly that the installer path installs `meshc` and `meshpkg` on the platforms S03 proves.
- [ ] Any retained source-build instructions are clearly labeled as an alternative or contributor workflow, not the authoritative public install proof.

## Verification

- `rg -n 'meshlang.dev/install.sh|meshlang.dev/install.ps1' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md tools/editors/vscode-mesh/README.md`
- `! rg -n 'Today the verified install path is building `meshc` from source' website/docs/docs/getting-started/index.md`
- `rg -n 'meshc --version|meshpkg --version' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md`

## Inputs

- `README.md` — repo quick-start still leading with source build.
- `website/docs/docs/getting-started/index.md` — current getting-started install claim that still points at source build.
- `website/docs/docs/tooling/index.md` — public tooling contract that must stay aligned with installer truth.
- `tools/editors/vscode-mesh/README.md` — editor-facing install wording that should stay consistent with the verified path.

## Expected Output

- `README.md` — quick-start install instructions aligned to the verified installer path.
- `website/docs/docs/getting-started/index.md` — getting-started guide rewritten around installer truth.
- `website/docs/docs/tooling/index.md` — tooling docs aligned to the installed `meshc`/`meshpkg` contract.
- `tools/editors/vscode-mesh/README.md` — editor README wording kept consistent with the verified installer path.
