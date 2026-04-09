# S04 Research — Extension release path hardening

## Summary

S04 is **targeted research**. The technology is familiar, but the current codebase has a few subtle release-path failures that would let the extension lane go green while shipping a bad or partial public result.

The biggest finding is that the current VSIX packaging contract is wrong **before** the workflow even publishes anything: the extension runtime imports `vscode-languageclient/node`, but the packaged VSIX omits the runtime JS files needed for that dependency. That makes the current lane artifact-green but truth-red.

The safest sequence is:

1. fix the packaged extension contract first,
2. codify it in one repo-local verifier,
3. make the publish workflow a thin caller around that verifier,
4. only then make publication fail-closed.

## Requirements Targeting

### Direct / primary

- **R047** — harden the extension release lane without pretending full editor parity already exists.
  - The slice should prove packaging, version/tag prerequisites, and release-lane truth.
  - It should **not** claim S04 proves full VS Code/editor correctness beyond what existing compiler/LSP proof surfaces already cover.

### Supporting

- **R045** — CI/CD and release flows should prove shipped surfaces rather than only build artifacts.
  - For S04, that means the publish lane must prove the actual packaged VSIX and release prerequisites, not just that TypeScript compiles and a publish action ran.

## Skills Discovered

- **Existing installed skill:** `github-workflows`
  - Most relevant rule: its validation constraint says **“No errors is not validation.”** S04 should therefore produce an observable proof surface for the extension lane, not just YAML that parses.
  - The skill also emphasizes checking `on`, `permissions`, `concurrency`, and reusable-workflow syntax from current docs before changing workflow structure.
- **Newly installed for downstream units:** `luongnv89/skills@vscode-extension-publisher`
  - Installed with: `npx skills add luongnv89/skills@vscode-extension-publisher -g -y`
  - It is not available in this current prompt yet, but later planner/executor units should receive it automatically.

No other direct technology skill looked necessary.

## Implementation Landscape

### Current publish lane

- **`.github/workflows/publish-extension.yml`**
  - current trigger: `push.tags: ext-v*`
  - installs deps, compiles TS, packages a VSIX, then publishes to Open VSX and VS Marketplace
  - important current lines from `rg -n`:
    - line 36: `npx vsce package --no-dependencies`
    - line 37: `echo "vsix_path=$(ls *.vsix)" >> "$GITHUB_OUTPUT"`
    - line 42: `continue-on-error: true` on the Open VSX publish step
  - this workflow currently owns public publication directly; there is no prepublish verifier script and no reusable proof surface like S02 created for the package-manager lane.

### Extension packaging inputs

- **`tools/editors/vscode-mesh/package.json`**
  - extension version: `0.3.0`
  - publisher: `OpenWorthTechnologies`
  - runtime dependency: `vscode-languageclient`
  - current scripts:
    - line 81: `"package": "vsce package --no-dependencies"`
    - line 82: `"install-local": "vsce package --no-dependencies && code --install-extension mesh-lang-0.3.0.vsix"`
- **`tools/editors/vscode-mesh/.vscodeignore`**
  - line 13 excludes `node_modules/**`
  - line 4 re-includes `!**/*.d.ts`
  - effect: runtime JS files from dependencies are excluded, but type declaration files are re-included.
- **`tools/editors/vscode-mesh/src/extension.ts`**
  - line 9 imports `vscode-languageclient/node`
  - this is a real runtime dependency, not a type-only import.

### Existing shipped artifacts and tracked inputs

- The repo currently tracks:
  - `tools/editors/vscode-mesh/mesh-lang-0.2.0.vsix`
  - `tools/editors/vscode-mesh/mesh-lang-0.3.0.vsix`
  - `tools/editors/vscode-mesh/node_modules/**`
  - `tools/editors/vscode-mesh/out/**`
- **`.gitignore`** currently ignores `editors/vscode-mesh/node_modules/` (line 7), which does **not** match the real path `tools/editors/vscode-mesh/node_modules/`.
  - So the extension workspace has tracked vendor/build artifacts that the workflow currently sees.

### Existing proof surfaces worth reusing

- **`compiler/meshc/tests/e2e_lsp.rs`**
  - already proves `meshc lsp` over real stdio JSON-RPC against `reference-backend/`
  - covers diagnostics, hover, go-to-definition, formatting, and signature help
- **`tools/editors/vscode-mesh/README.md`** and **`website/docs/docs/tooling/index.md`**
  - both already describe the extension as depending on the proven `meshc lsp` path
  - they also hardcode the manual install filename `mesh-lang-0.3.0.vsix`
- **S02 reusable-workflow pattern**
  - `.github/workflows/authoritative-live-proof.yml`
  - `.github/workflows/authoritative-verification.yml`
  - `scripts/verify-m034-s02-workflows.sh`
  - this is the right architectural pattern for S04 if the extension proof needs reuse in S05.

## Key Findings

### 1) The current packaged VSIX omits runtime dependencies

This is the main release blocker.

Evidence chain:

- `src/extension.ts` imports `vscode-languageclient/node` at runtime.
- `.vscodeignore` excludes `node_modules/**` but re-includes only `*.d.ts`.
- both the workflow and `package.json` scripts package with `--no-dependencies`.
- `(cd tools/editors/vscode-mesh && npx vsce ls --no-dependencies)` returns only **8 files**:
  - `package.json`
  - `language-configuration.json`
  - `README.md`
  - `CHANGELOG.md`
  - `syntaxes/mesh.tmLanguage.json`
  - `out/extension.js`
  - `images/icon.png`
  - `images/icon-128.png`
- The tracked `mesh-lang-0.3.0.vsix` does **not** contain:
  - `extension/node_modules/vscode-languageclient/package.json`
  - `extension/node_modules/vscode-languageclient/node.js`
  - `extension/node_modules/vscode-jsonrpc/node.js`

This means the release lane can compile and publish a VSIX that is missing the JS runtime files needed for activation.

Important nuance: simply removing `--no-dependencies` is **not enough**. With the current `.vscodeignore`, `vsce` still sees the runtime dependency tree through a broken filter. `npx vsce ls` currently reports **121 files**, but because of the ignore pattern the packaged VSIXs still omit the required runtime JS modules. The root cause is the combination of:

- `--no-dependencies`, and
- `.vscodeignore` excluding `node_modules/**` while re-including only declarations.

### 2) The workflow chooses the VSIX path non-deterministically

Workflow line 37 uses:

```sh
echo "vsix_path=$(ls *.vsix)" >> "$GITHUB_OUTPUT"
```

But the repo already tracks both:

- `mesh-lang-0.2.0.vsix`
- `mesh-lang-0.3.0.vsix`

So `ls *.vsix` is not a single-file truth surface. In a shell command substitution here, the two filenames remain newline-separated, which is unsafe for `$GITHUB_OUTPUT` and also makes the selected artifact ambiguous.

Even if the workflow does not error, it is coupling publication to whatever historical VSIX files happen to be sitting in the repo tree.

### 3) The workflow allows partial public publication to look successful

The Open VSX publish step is marked `continue-on-error: true`.

That means the workflow can still pass after only one marketplace is updated. For the roadmap’s public-ready claim, that is a false green unless the product explicitly accepts one-market publication as success.

My recommendation: **treat dual-market publication as fail-closed** unless the user has explicitly decided otherwise.

### 4) Release prerequisites are not checked mechanically

The current lane does not verify several obvious prepublish contracts:

- `ext-vX.Y.Z` tag version vs `tools/editors/vscode-mesh/package.json` version
- manual install docs vs current package version
- `install-local` script vs current package version
- workflow header example comment vs current version
- exact packaged VSIX content before publish

Current hardcoded drift points:

- `tools/editors/vscode-mesh/package.json` line 82 hardcodes `mesh-lang-0.3.0.vsix`
- `tools/editors/vscode-mesh/README.md` line 49 hardcodes `mesh-lang-0.3.0.vsix`
- `website/docs/docs/tooling/index.md` line 349 hardcodes `mesh-lang-0.3.0.vsix`
- the workflow header comment still shows `ext-v0.2.0`

These are exactly the kinds of release prerequisites S04 should make mechanical.

### 5) There is no extension-specific proof surface today

There are no extension tests under `tools/editors/vscode-mesh/`.

The meaningful existing editor-truth surface is upstream in `compiler/meshc/tests/e2e_lsp.rs`, which is good news: S04 does **not** need to invent a full VS Code integration suite. It can reuse the existing LSP semantic proof and focus on what is unique to the extension lane:

- packaged VSIX truth
- publish prereqs
- workflow fail-closed behavior
- dual-market publication contract

## Recommendation

### Recommended architecture

Use the same pattern S02 established:

1. **One repo-local extension verifier script** as the canonical truth surface.
2. **One thin workflow** that calls it before any public publish happens.
3. Prefer a **reusable workflow** if S05 will need to call the extension proof from a larger public-release assembly flow.

### What the verifier should own

A new script like **`scripts/verify-m034-s04-extension.sh`** should likely own all local proof.

It should:

- read the extension version from `tools/editors/vscode-mesh/package.json`
- run `npm ci` and `npm run compile` in `tools/editors/vscode-mesh/`
- package to an **explicit output path** (do not glob `*.vsix`)
- inspect the produced VSIX with Python `zipfile` and fail if required files are missing
- assert the packaged VSIX includes at least:
  - `extension/package.json`
  - `extension/out/extension.js`
  - `extension/syntaxes/mesh.tmLanguage.json`
  - `extension/readme.md`
  - `extension/changelog.md`
  - `extension/images/icon.png`
  - runtime JS for the LSP client stack (`vscode-languageclient`, `vscode-jsonrpc`, protocol/runtime files)
- assert docs/install instructions either:
  - derive the filename/version dynamically, or
  - exactly match the current package version
- optionally accept an expected tag/version argument and fail if `ext-v...` does not match `package.json`
- optionally rerun the existing LSP semantic proof (`cargo test -q -p meshc --test e2e_lsp -- --nocapture`) as a prepublish prerequisite

That last item is optional because it is already a compiler proof surface, but it is a strong prepublish gate if the slice wants extension publication to depend on currently proven LSP truth.

### What the workflow should own

Keep the workflow thin:

- checkout
- setup Node 20
- run the verifier
- publish the exact VSIX the verifier produced
- upload diagnostics on failure

If the team wants to stay on **`HaaLeo/publish-vscode-extension@v2`**, that is a good choice. Its README already supports the needed shape:

- `extensionFile` to publish a prebuilt VSIX
- `dryRun` for packaging-only checks
- `dependencies` (default `true`)
- one package / dual registry publish pattern

So S04 does **not** need to replace it with custom curl or ad hoc registry calls.

### What to fix first

1. **Packaged artifact truth** (`.vscodeignore`, `package.json`, deterministic output path)
2. **Repo-local verifier**
3. **Workflow fail-closed publish behavior**
4. **Doc/install version drift cleanup**

That order matters. Hardening workflow logic around a broken package just creates a more authoritative false green.

## Natural Seams for Planning

### Seam A — package contract cleanup

**Files likely touched**

- `tools/editors/vscode-mesh/.vscodeignore`
- `tools/editors/vscode-mesh/package.json`
- possibly `.gitignore`
- possibly remove reliance on tracked `.vsix` files entirely

**Goal**

- let the packaged extension include its actual runtime dependencies
- make package output deterministic
- stop release logic from depending on historical tracked `.vsix` artifacts

**Risk**

- high: this is the root-cause seam

### Seam B — canonical extension verifier

**Files likely touched**

- new `scripts/verify-m034-s04-extension.sh`
- maybe a companion diagnostics directory under `.tmp/m034-s04/verify/`

**Goal**

- one mechanical prepublish proof surface for the extension lane
- reusable locally, in CI, and later in S05

**Risk**

- medium

### Seam C — workflow hardening

**Files likely touched**

- `.github/workflows/publish-extension.yml`
- maybe new reusable workflow file if extracting a proof lane
- maybe new workflow contract verifier, e.g. `scripts/verify-m034-s04-workflows.sh`

**Goal**

- publish only after proof passes
- publish the exact verified VSIX
- fail closed on marketplace failures unless intentionally designed otherwise

**Risk**

- medium

### Seam D — docs/version prereq cleanup

**Files likely touched**

- `tools/editors/vscode-mesh/README.md`
- `website/docs/docs/tooling/index.md`
- `tools/editors/vscode-mesh/package.json` (`install-local`)

**Goal**

- remove or mechanize stale versioned install instructions
- align human docs with the actual release contract

**Risk**

- low once package contract is fixed

## What To Build or Prove First

1. **Prove the VSIX contents are correct.** This is the riskiest and currently failing contract.
2. **Make version/tag/doc prerequisites mechanical.** Otherwise the lane can still publish the wrong thing cleanly.
3. **Only then harden workflow publication semantics.**
4. **If reusable-workflow extraction is chosen, do it after the verifier exists.** The verifier should be the thing reused.

## Don’t Hand-Roll

- **Do not hand-maintain a whitelist of transitive runtime files** for `vscode-languageclient` and friends if `vsce` dependency detection can own that. A handwritten allowlist for `node_modules` contents will drift.
- **Do not replace `HaaLeo/publish-vscode-extension@v2` with custom HTTP publish logic.** The action already supports publishing an identical prebuilt VSIX to both registries.
- **Do not duplicate LSP semantic assertions inside YAML.** Reuse the existing compiler-side LSP proof surface if S04 wants that prerequisite.

## Verification Guidance

### Investigative commands that produced the findings

- `npm --prefix tools/editors/vscode-mesh exec vsce -- package --help`
- `(cd tools/editors/vscode-mesh && npx vsce ls --no-dependencies)`
- `(cd tools/editors/vscode-mesh && npx vsce ls | wc -l)` → `121`
- Python `zipfile` inspection of `tools/editors/vscode-mesh/mesh-lang-0.3.0.vsix`
- `git ls-files 'tools/editors/vscode-mesh/*.vsix' 'tools/editors/vscode-mesh/node_modules/*'`
- `rg -n` sweeps over the workflow, `.vscodeignore`, `package.json`, README, website docs, and extension source

### What should become the post-implementation proof bundle

At minimum:

- `bash scripts/verify-m034-s04-extension.sh`
- `ruby -e 'require "yaml"; YAML.load_file(".github/workflows/publish-extension.yml")'`

If S04 mirrors S02’s pattern, also add:

- `bash scripts/verify-m034-s04-workflows.sh`

Strong optional prerequisite gate:

- `cargo test -q -p meshc --test e2e_lsp -- --nocapture`

The verifier should leave diagnostics under a deterministic root like `.tmp/m034-s04/verify/` so CI failure artifacts and local debugging are identical.

## Open Questions / Planner Notes

- **Keep `ext-v*` tags or not?**
  - S04 can keep them, but the proof surface should be reusable so S05 can call it from a broader release-candidate flow without being trapped in a separate tag-only lane.
- **Should dual-market publication be fail-closed?**
  - I recommend yes. Otherwise the public-ready claim stays ambiguous.
- **Should the extension version remain independent from `meshc` / `meshpkg`?**
  - Current state is independent (`extension 0.3.0` vs `meshc/meshpkg 0.1.0`). S04 only needs to enforce extension-tag/package alignment, but S05 may want an explicit policy.
- **Should docs stay versioned or become version-agnostic?**
  - For local-from-source install instructions, version-agnostic commands or script-driven install are safer than hardcoded filenames.

## Sources

- Local `@vscode/vsce` help and README from `tools/editors/vscode-mesh/node_modules/@vscode/vsce/`
- `HaaLeo/publish-vscode-extension` README: https://raw.githubusercontent.com/HaaLeo/publish-vscode-extension/master/README.md
