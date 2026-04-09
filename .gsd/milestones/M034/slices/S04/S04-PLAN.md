# S04: Extension release path hardening

**Goal:** Turn the VS Code extension release lane into a truthful prepublish gate: the repo must produce a deterministic VSIX that contains its real runtime dependencies, mechanically reject tag/version/docs drift, reuse the existing `meshc lsp` proof as a prerequisite, and only let publication proceed through a thin fail-closed workflow that publishes the exact verified VSIX.
**Demo:** After this: The VS Code extension publish lane validates the packaged extension and release prerequisites before public publication.

## Tasks
- [x] **T01: Made VSIX packaging deterministic under dist/, restored runtime dependencies to the shipped extension, and removed hardcoded local-install VSIX drift.** — Fix the broken packaging contract before touching CI. The extension currently packages from a stale root, excludes the runtime JS tree behind `vscode-languageclient/node`, and teaches humans plus scripts to install a hardcoded `mesh-lang-0.3.0.vsix`. This task makes the VSIX path deterministic, keeps runtime dependencies in the package, and removes historical build-artifact/documentation drift that would otherwise let later verification pass against the wrong file.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `vsce` packaging / npm scripts | Stop immediately and leave the failing package command plus output path visible; do not fall back to a previous checked-in VSIX. | Treat packaging stalls as failure and keep the last attempted VSIX path in logs. | Treat a VSIX missing runtime dependency entries or missing `out/extension.js` as contract failure. |
| `.vscodeignore` filter rules | Refuse the task if runtime `node_modules` JS stays excluded, even if `vsce package` exits 0. | N/A | Treat over-broad excludes or declaration-only re-includes as a broken release contract. |
| Local install docs/scripts | Fail if `package.json`, `README.md`, or tooling docs still hardcode `mesh-lang-0.3.0.vsix` or another version-pinned filename. | N/A | Treat commands that can resolve multiple VSIXs or drift from the deterministic dist path as release-prereq failure. |

## Load Profile

- **Shared resources**: `tools/editors/vscode-mesh/node_modules/`, the new deterministic `tools/editors/vscode-mesh/dist/` packaging root, and checked-in extension artifacts that must stop shadowing fresh outputs.
- **Per-operation cost**: one extension compile plus one VSIX package build and one archive listing check.
- **10x breakpoint**: repeated packaging would first waste time on extension dependency install/build; the deterministic output path should let later tasks reuse one freshly produced VSIX instead of repackaging via globs.

## Negative Tests

- **Malformed inputs**: multiple historical `*.vsix` files in the extension directory, hardcoded stale version strings, or a `.vscodeignore` rule that still strips runtime JS.
- **Error paths**: packaging succeeds but the archive omits `vscode-languageclient` / `vscode-jsonrpc`, or local install still resolves the wrong file.
- **Boundary conditions**: the deterministic path must work when the extension version changes, not only for `0.3.0`, and package scripts/docs must no longer depend on repo-root globbing.

## Steps

1. Add a small helper under `tools/editors/vscode-mesh/scripts/` that computes the current versioned VSIX path under `tools/editors/vscode-mesh/dist/` so `package` and `install-local` stop hardcoding `0.3.0` or relying on `ls *.vsix`.
2. Update `tools/editors/vscode-mesh/package.json` and `.vscodeignore` so packaging emits the deterministic dist artifact, includes the runtime dependency tree required by `vscode-languageclient/node`, and still excludes source/tests/maps/dev-only clutter.
3. Fix extension artifact hygiene in `.gitignore` and remove the checked-in historical VSIXs if they are still present so future work cannot accidentally publish a stale file.
4. Rewrite `tools/editors/vscode-mesh/README.md` and `website/docs/docs/tooling/index.md` to use the version-agnostic local install path (`npm run package` / `npm run install-local` or the deterministic dist file) instead of `mesh-lang-0.3.0.vsix`.

## Must-Haves

- [ ] Packaging writes one deterministic VSIX under `tools/editors/vscode-mesh/dist/`
- [ ] The packaged VSIX contains runtime `vscode-languageclient` / `vscode-jsonrpc` files needed for activation
- [ ] Historical VSIX outputs and the real extension `node_modules/` path are ignored so they cannot shadow fresh packaging results
- [ ] Extension scripts and docs no longer hardcode a specific `mesh-lang-<version>.vsix` filename
  - Estimate: 2h
  - Files: tools/editors/vscode-mesh/package.json, tools/editors/vscode-mesh/.vscodeignore, tools/editors/vscode-mesh/scripts/vsix-path.mjs, .gitignore, tools/editors/vscode-mesh/README.md, website/docs/docs/tooling/index.md, tools/editors/vscode-mesh/mesh-lang-0.2.0.vsix, tools/editors/vscode-mesh/mesh-lang-0.3.0.vsix
  - Verify: - `npm --prefix tools/editors/vscode-mesh run compile`
- `npm --prefix tools/editors/vscode-mesh run package`
- `VERSION=$(node -p "require('./tools/editors/vscode-mesh/package.json').version"); VSIX="tools/editors/vscode-mesh/dist/mesh-lang-${VERSION}.vsix"; test -f "$VSIX" && unzip -l "$VSIX" | rg 'extension/node_modules/(vscode-languageclient|vscode-jsonrpc)'`
- `! rg -n 'mesh-lang-0\.[0-9]+\.vsix|--no-dependencies' tools/editors/vscode-mesh/package.json tools/editors/vscode-mesh/README.md website/docs/docs/tooling/index.md`
- [x] **T02: Added the canonical extension prepublish verifier with VSIX audit, prereq drift checks, and reused `e2e_lsp` proof.** — Once packaging is truthful, freeze it behind one repo-local proof surface that CI and S05 can call unchanged. This task creates the extension verifier that packages the deterministic VSIX, audits the archive contents, checks release prerequisites, and reuses the already-existing `meshc lsp` transport proof so the publish lane inherits real language-server truth without pretending S04 proves full editor parity.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `npm ci` / `vsce package` | Stop at the failing phase and keep compile/package output plus the intended VSIX path under `.tmp/m034-s04/verify/`; never continue with an old artifact. | Abort the verifier and record which phase stalled. | Treat a VSIX missing required files or runtime dependency entries as proof failure. |
| `cargo test -q -p meshc --test e2e_lsp -- --nocapture` | Fail the prepublish gate and keep the named `e2e_lsp` output visible; do not replace it with VS Code-specific guesses. | Treat the timeout as verifier failure and report the stalled prerequisite phase. | Treat missing JSON-RPC proof output or a failing test filter as a broken upstream contract. |
| Version/tag/docs prereq sweep | Reject drift immediately when the expected tag, package version, workflow comment, or local-install docs disagree. | N/A | Treat malformed tag strings, missing docs/install commands, or an unreadable `package.json` version as proof failure. |

## Load Profile

- **Shared resources**: the extension `node_modules/` tree, Cargo build/test caches, and `.tmp/m034-s04/verify/` diagnostics.
- **Per-operation cost**: one clean extension dependency install, one compile/package cycle, one VSIX archive scan, and one targeted `e2e_lsp` cargo test run.
- **10x breakpoint**: repeated verifier runs would spend most time in Node/Cargo setup, so the script should package once, audit that one VSIX, and reuse the same diagnostics root for all phases.

## Negative Tests

- **Malformed inputs**: mismatched `EXPECTED_TAG`, stale doc/install filename, missing icon/readme/changelog entries, or a VSIX with only `.d.ts` files from runtime dependencies.
- **Error paths**: `npm ci` failure, `vsce package` success with a broken archive, or `e2e_lsp` regressions must each stop the proof at a named phase.
- **Boundary conditions**: the verifier must reject both package-level drift (wrong version/file path) and content-level drift (missing runtime JS) even when `out/extension.js` exists.

## Steps

1. Create `scripts/verify-m034-s04-extension.sh` as the single extension release proof entrypoint; it should prepare `.tmp/m034-s04/verify/`, derive the extension version, and call the deterministic packaging path from T01.
2. Make the verifier run `npm ci`, `npm run compile`, and `npm run package` in `tools/editors/vscode-mesh/`, then inspect the generated VSIX with Python `zipfile` checks for required shipped files and runtime dependency entries.
3. Add prereq checks for `EXPECTED_TAG` / `ext-vX.Y.Z`, current package version, `install-local`, extension README, tooling docs, and any workflow-facing version examples so release-lane drift fails before publish.
4. Rerun `cargo test -q -p meshc --test e2e_lsp -- --nocapture` inside the verifier and write the exact verified VSIX path plus archive manifest into `.tmp/m034-s04/verify/` for downstream workflow reuse and postmortems.

## Must-Haves

- [ ] `scripts/verify-m034-s04-extension.sh` owns compile/package/audit/prereq checks for the extension lane
- [ ] The verifier leaves phase-scoped diagnostics plus the exact verified VSIX path under `.tmp/m034-s04/verify/`
- [ ] Tag/package/docs/install-local drift fails before publish
- [ ] The existing `e2e_lsp` proof is rerun as an upstream prerequisite without adding new editor-parity claims
  - Estimate: 2h
  - Files: scripts/verify-m034-s04-extension.sh, tools/editors/vscode-mesh/package.json, tools/editors/vscode-mesh/scripts/vsix-path.mjs, .tmp/m034-s04/verify/verified-vsix-path.txt, .tmp/m034-s04/verify/vsix-contents.txt, .tmp/m034-s04/verify/e2e-lsp.log
  - Verify: - `EXPECTED_TAG="ext-v$(node -p "require('./tools/editors/vscode-mesh/package.json').version")" bash scripts/verify-m034-s04-extension.sh`
- `test -f .tmp/m034-s04/verify/verified-vsix-path.txt`
- `test -f .tmp/m034-s04/verify/vsix-contents.txt`
- `rg -n 'vscode-languageclient|vscode-jsonrpc' .tmp/m034-s04/verify/vsix-contents.txt`
- [x] **T03: Added a reusable extension proof workflow and fail-closed publish handoff for the exact verified VSIX.** — Finish the slice by making GitHub Actions a thin caller around the repo-local verifier instead of another place where release truth can drift. Following the S02 pattern, add a reusable extension-proof workflow, make the tag-triggered publish lane consume the exact verified VSIX, and back the workflow contract with a repo-local verifier so future edits cannot quietly restore globbing or partial-success publication.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Reusable workflow ↔ publish workflow handoff | Block publication and preserve the missing output/artifact name in logs; never repackage ad hoc in the publish job. | Fail the caller job and keep the proof job/artifact status visible. | Treat missing workflow outputs, wrong artifact names, or a publish job that repackages independently as contract drift. |
| Marketplace publish actions | Fail closed if either Open VSX or VS Marketplace publish fails; do not mark the lane green after partial success. | Let the publish step timeout fail the job and keep the already-verified VSIX path visible for reruns. | Treat missing tokens, wrong `extensionFile`, or registry-specific config drift as publish-lane failure. |
| Local workflow contract verifier | Reject trigger, permission, reusable-call, diagnostics-upload, or `extensionFile` drift before CI. | N/A | Treat `continue-on-error`, `ls *.vsix`, or direct verifier execution from the publish workflow as broken contract. |

## Load Profile

- **Shared resources**: GitHub runner minutes, workflow artifacts, and marketplace publication rate limits/tokens.
- **Per-operation cost**: one reusable proof job, one artifact/output handoff, and two publish actions against different registries.
- **10x breakpoint**: runner queueing and artifact handoff would dominate first, so the publish job must reuse the exact verified VSIX instead of recompiling or repackaging.

## Negative Tests

- **Malformed inputs**: stale trigger comment/tag example, missing reusable workflow output, `continue-on-error`, or a caller workflow that still resolves `*.vsix` via globbing.
- **Error paths**: reusable proof failure, missing diagnostics artifact, or one-registry publish failure must each keep the overall lane red.
- **Boundary conditions**: both registries must publish the same verified VSIX file, and the publish workflow must not call `bash scripts/verify-m034-s04-extension.sh` directly if the reusable workflow owns that proof.

## Steps

1. Add `.github/workflows/extension-release-proof.yml` as a reusable workflow that checks out the repo, sets up the toolchain needed by `scripts/verify-m034-s04-extension.sh`, runs that verifier exactly once, and uploads `.tmp/m034-s04/verify/**` on failure.
2. Rewrite `.github/workflows/publish-extension.yml` so the tag lane calls the reusable proof workflow, consumes the exact verified VSIX path or artifact it emits, updates the trigger comment/example, and removes `continue-on-error` plus `ls *.vsix` selection.
3. Add `scripts/verify-m034-s04-workflows.sh` to parse both workflow files and mechanically enforce the reusable-owner pattern, exact verifier invocation, diagnostics retention, deterministic `extensionFile`, fail-closed dual-market publication, and the absence of globbing / inline proof logic.
4. Rerun the local workflow verifier plus YAML parse sweep until workflow drift is rejected mechanically before CI.

## Must-Haves

- [ ] The reusable proof workflow is the only workflow file that directly runs `bash scripts/verify-m034-s04-extension.sh`
- [ ] The tag-triggered publish workflow depends on reusable proof and publishes the exact verified VSIX to both registries
- [ ] Partial publication cannot pass green; `continue-on-error` and `ls *.vsix` are gone
- [ ] `scripts/verify-m034-s04-workflows.sh` mechanically rejects contract drift before CI
  - Estimate: 2h
  - Files: .github/workflows/extension-release-proof.yml, .github/workflows/publish-extension.yml, scripts/verify-m034-s04-workflows.sh
  - Verify: - `bash scripts/verify-m034-s04-workflows.sh`
- `ruby -e 'require "yaml"; %w[.github/workflows/extension-release-proof.yml .github/workflows/publish-extension.yml].each { |f| YAML.load_file(f) }'`
- `! rg -n 'continue-on-error|ls \*\.vsix|bash scripts/verify-m034-s04-extension.sh' .github/workflows/publish-extension.yml`
- `rg -n './.github/workflows/extension-release-proof.yml|extensionFile:' .github/workflows/publish-extension.yml`
