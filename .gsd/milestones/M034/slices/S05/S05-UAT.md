# S05: Full public release assembly proof — UAT

**Milestone:** M034
**Written:** 2026-03-27T03:27:40.326Z

# S05 UAT — Full public release assembly proof

**Milestone:** M034  
**Written:** 2026-03-27

## Preconditions
- Worktree contains the landed S05 changes.
- `.env` contains the real registry publish credentials required by `scripts/verify-m034-s01.sh` (`MESH_PUBLISH_OWNER`, `MESH_PUBLISH_TOKEN`, and any other S01-required env vars).
- `gh` is authenticated for `snowdamiz/mesh-lang` so the remote-evidence phase can query workflow runs.
- Local tooling is installed: `node`, `npm`, `cargo`, `ruby`, `python3`, `curl`, `diff`, and `rg`.
- No manual cleanup is required under `.tmp/m034-s05/`; the verifier recreates its own artifact directories.

## Test Case 1 — Local deploy workflow and public contract stay exact
1. Run `bash scripts/verify-m034-s05-workflows.sh`.
   - **Expected:** exits 0 and prints `verify-m034-s05-workflows: ok (all)`.
2. Run `ruby -e 'require "yaml"; %w[.github/workflows/deploy.yml .github/workflows/deploy-services.yml].each { |f| YAML.load_file(f) }'`.
   - **Expected:** both workflow files parse as YAML.
3. Run `rg -n 'install\.sh|install\.ps1|packages/snowdamiz/mesh-registry-proof|api/v1/packages\?search=snowdamiz%2Fmesh-registry-proof' .github/workflows/deploy.yml .github/workflows/deploy-services.yml`.
   - **Expected:** the exact installer and scoped package proof URLs are present in the deploy workflows.
4. Inspect `.tmp/m034-s05/workflows/phase-report.txt`.
   - **Expected:** `docs`, `services`, and `full-contract` are marked `passed`.

## Test Case 2 — README/docs/extension metadata publish one runbook story
1. Run `rg -n 'meshlang.dev/install.sh|meshlang.dev/install.ps1|meshpkg --version' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md tools/editors/vscode-mesh/README.md`.
   - **Expected:** every required file references the public installer pair and `meshpkg --version`.
2. Run `! rg -n 'Today the verified install path is building \`meshc\` from source|mesh-lang/mesh' README.md website/docs/docs/getting-started/index.md website/docs/docs/tooling/index.md website/docs/public/install.ps1 tools/editors/vscode-mesh/package.json tools/editors/vscode-mesh/README.md`.
   - **Expected:** the stale source-build-only wording and old repo slug are absent.
3. Run `node -p "require('./tools/editors/vscode-mesh/package.json').repository.url"` and `node -p "require('./tools/editors/vscode-mesh/package.json').bugs.url"`.
   - **Expected:** outputs are `https://github.com/snowdamiz/mesh-lang.git` and `https://github.com/snowdamiz/mesh-lang/issues`.

## Test Case 3 — Candidate-tag policy and runbook contract are mechanically pinned
1. Run `bash -n scripts/verify-m034-s05.sh`.
   - **Expected:** shell syntax check passes.
2. Run `node --test scripts/tests/verify-m034-s05-contract.test.mjs`.
   - **Expected:** all three tests pass.
3. Run `rg -n 'verify-m034-s05|v<Cargo version>|ext-v<extension version>|deploy\.yml|deploy-services\.yml|authoritative-verification\.yml|extension-release-proof\.yml|publish-extension\.yml' README.md website/docs/docs/tooling/index.md`.
   - **Expected:** README and tooling docs both contain the canonical command, split candidate tags, and required workflow names.

## Test Case 4 — Standalone live registry proof still holds
1. Run `set -a && source .env && set +a && bash scripts/verify-m034-s01.sh`.
   - **Expected:** exits 0 and prints `verify-m034-s01: ok`.
2. Inspect the latest `.tmp/m034-s01/verify/` run directory.
   - **Expected:** it contains the normal S01 publish/install/visibility artifacts, proving the registry/package-manager path is still green independently of S05 rollout gaps.

## Test Case 5 — The assembled verifier fails closed on current hosted rollout gaps and leaves audit artifacts
1. Run `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh`.
   - **Current expected outcome:** exits non-zero at `remote-evidence` until the remote default branch and candidate-tag workflow history catch up.
2. Inspect `.tmp/m034-s05/verify/current-phase.txt`, `.tmp/m034-s05/verify/status.txt`, and `.tmp/m034-s05/verify/phase-report.txt`.
   - **Expected:** `current-phase.txt` is `remote-evidence`, `status.txt` is `failed`, and earlier phases through `s04-workflows` are marked `passed`.
3. Inspect `.tmp/m034-s05/verify/candidate-tags.json`.
   - **Expected:** `binaryTag` is `v0.1.0` and `extensionTag` is `ext-v0.3.0`.
4. Inspect `.tmp/m034-s05/verify/remote-runs.json`.
   - **Current expected outcome:** it names the hosted rollout gaps instead of synthesizing success — the latest remote `deploy.yml` run is missing `Verify public docs contract`, there are no `v0.1.0` runs for `deploy-services.yml` or `release.yml`, `authoritative-verification.yml` and `extension-release-proof.yml` are missing on the remote default branch, and there is no `ext-v0.3.0` run for `publish-extension.yml`.

## Test Case 6 — Current public-surface freshness split is explicit
1. Compare the deployed installers with the repo copies:
   ```bash
   tmpdir=$(mktemp -d)
   curl -fsSL https://meshlang.dev/install.sh -o "$tmpdir/install.sh"
   curl -fsSL https://meshlang.dev/install.ps1 -o "$tmpdir/install.ps1"
   diff -u website/docs/public/install.sh "$tmpdir/install.sh"
   diff -u website/docs/public/install.ps1 "$tmpdir/install.ps1"
   ```
   - **Current expected outcome:** both diffs are non-empty, proving the deployed installers are stale relative to the repo contract.
2. Fetch `https://meshlang.dev/docs/getting-started/` and `https://meshlang.dev/docs/tooling/`, normalize the HTML text, and check for the installer pair, `meshpkg --version`, the `verify-m034-s05.sh` command, the workflow list, candidate-tag markers, and proof artifact paths.
   - **Current expected outcome:** the deployed pages are still missing those new markers.
3. Fetch `https://packages.meshlang.dev/packages/snowdamiz/mesh-registry-proof`, `https://packages.meshlang.dev/search?q=snowdamiz%2Fmesh-registry-proof`, and `https://api.packages.meshlang.dev/api/v1/packages?search=snowdamiz%2Fmesh-registry-proof`.
   - **Expected:** all three surfaces still include the exact `snowdamiz/mesh-registry-proof` name and `Real registry publish/install proof fixture for M034 S01` description.

## Test Case 7 — Future all-green replay after rollout and redeploy
1. After the local workflow/docs changes are on the remote default branch, the `v0.1.0` and `ext-v0.3.0` workflows have run, and `meshlang.dev` has been redeployed, rerun `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh`.
   - **Expected final outcome:** exits 0, `.tmp/m034-s05/verify/status.txt` contains `ok`, `.tmp/m034-s05/verify/current-phase.txt` contains `complete`, and `phase-report.txt` shows `remote-evidence`, `public-http`, and `s01-live-proof` as `passed`.

## Edge Cases / Failure Triage
- If the assembled verifier fails at `remote-evidence`, inspect `.tmp/m034-s05/verify/remote-runs.json` and the `remote-*.log` files before touching local scripts or docs; that phase is intentionally earlier so hosted rollout gaps stay visible.
- If it reaches `public-http` and then fails, inspect the generated `public-*.body`, `public-*.headers`, `public-*.diff`, and `public-*-check.log` artifacts to see which exact deployed surface drifted.
- If standalone S01 fails, inspect the latest `.tmp/m034-s01/verify/` run directory and confirm `.env` was sourced before rerunning; do not treat an env-loading failure as package-manager drift.
