# S01: Two-Repo Boundary & GSD Authority Contract — UAT

**Milestone:** M055
**Written:** 2026-04-06T18:38:43.152Z

# S01: Two-Repo Boundary & GSD Authority Contract — UAT

**Milestone:** M055
**Written:** 2026-04-05

## UAT Type

- UAT mode: artifact-driven
- Why this mode is sufficient: S01 ships workspace/docs/metadata/verifier surfaces, not a new runtime feature. The truthful acceptance seam is the repo-owned contract rails, consumer builds, and the retained verifier bundle under `.tmp/m055-s01/verify/`.

## Preconditions

- Start from the repo root.
- Node.js and npm are available so the packages site and landing app can build.
- Rust/Cargo are available so the named M046 repo-local `.gsd` regression can run.
- Existing repo dependencies for `packages-website/` and `mesher/landing/` are already installed.

## Smoke Test

Run:

```bash
node --test scripts/tests/verify-m055-s01-contract.test.mjs
```

**Expected:** 15 tests pass, 0 fail. The repo confirms the two-repo workspace contract, the assembled verifier contract, and the canonical repo-identity/public-surface contract.

## Test Cases

### 1. Maintainer docs publish the blessed two-repo workspace and repo-local `.gsd` authority

1. Run `node --test scripts/tests/verify-m055-s01-contract.test.mjs`.
2. Confirm the output includes passing checks for:
   - the M055 S01 two-repo workspace and repo-local `.gsd` contract
   - the assembled verifier and debug entrypoint contract
   - four-repo / umbrella-`.gsd` mutation failures
3. Open `WORKSPACE.md` and confirm it names only these siblings:

   ```text
   <workspace>/
     mesh-lang/
     hyperpush-mono/
   ```
4. Confirm `WORKSPACE.md` says `website/`, `packages-website/`, `registry/`, installers, and evaluator-facing examples remain language-owned in `mesh-lang`.
5. Confirm `WORKSPACE.md` says repo-local `.gsd` stays authoritative and explicitly rejects one umbrella milestone tree for both repos.
6. **Expected:** Maintainer-facing root docs describe one two-repo split only, and the node:test rail fails closed if that wording drifts.

### 2. Canonical repo identity and installer parity stay aligned

1. Run:

   ```bash
   diff -u tools/install/install.sh website/docs/public/install.sh
   diff -u tools/install/install.ps1 website/docs/public/install.ps1
   python3 scripts/lib/m034_public_surface_contract.py local-docs --root .
   ```
2. Open `scripts/lib/repo-identity.json` and confirm it contains both `languageRepo` and `productRepo` sections, with workspace dirs `mesh-lang` and `hyperpush-mono` respectively.
3. Confirm `languageRepo.slug` is `snowdamiz/mesh-lang` and `productRepo.slug` is `hyperpush-org/hyperpush-mono`.
4. **Expected:** The installer source/copy pairs are byte-identical, the local-docs helper passes, and the repo-identity JSON is the canonical slug/URL/root contract.

### 3. Packages, landing, and VS Code surfaces preserve the split-aware repo identity

1. Run:

   ```bash
   npm --prefix packages-website run build
   npm --prefix mesher/landing run build
   node --test scripts/tests/verify-m055-s01-contract.test.mjs
   ```
2. Open `packages-website/src/routes/+layout.svelte` and confirm the footer links include:
   - `mesh-lang repo`
   - `Workspace`
   - `https://github.com/snowdamiz/mesh-lang`
3. Open `mesher/landing/lib/external-links.ts` and confirm the GitHub markers point at `hyperpush-org/hyperpush-mono`.
4. Open `tools/editors/vscode-mesh/package.json` and confirm:
   - `repository.url` is `https://github.com/snowdamiz/mesh-lang.git`
   - `repository.directory` is `tools/editors/vscode-mesh`
   - `bugs.url` is `https://github.com/snowdamiz/mesh-lang/issues`
5. **Expected:** Language-owned public/editor surfaces stay on the language repo, product landing surfaces stay on the product repo, and the contract rail fails closed if those identities get mixed.

### 4. The assembled verifier publishes one auditable split-boundary bundle and replays the repo-local `.gsd` seam

1. Run:

   ```bash
   bash scripts/verify-m055-s01.sh
   ```
2. Confirm `.tmp/m055-s01/verify/status.txt` contains `ok`.
3. Confirm `.tmp/m055-s01/verify/current-phase.txt` contains `complete`.
4. Open `.tmp/m055-s01/verify/phase-report.txt` and confirm it includes `passed` markers for:
   - `init`
   - `m055-s01-contract`
   - `m055-s01-local-docs`
   - `m055-s01-packages-build`
   - `m055-s01-landing-build`
   - `m055-s01-gsd-regression`
5. Open `.tmp/m055-s01/verify/full-contract.log` and confirm it shows the wrapper replaying:
   - `node --test scripts/tests/verify-m055-s01-contract.test.mjs`
   - `python3 scripts/lib/m034_public_surface_contract.py local-docs --root .`
   - `npm --prefix packages-website run build`
   - `npm --prefix mesher/landing run build`
   - `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_contract_remains_source_first_and_route_free -- --nocapture`
6. **Expected:** The wrapper fails closed on the first broken boundary and leaves one obvious debug path in `.tmp/m055-s01/verify/`.

## Edge Cases

### Four-repo or umbrella-`.gsd` drift is rejected before extraction work starts

1. Temporarily change `WORKSPACE.md` so it mentions `mesh-packages/` or `mesh-website/` as sibling repos, or replace the repo-local `.gsd` wording with umbrella-workspace language.
2. Run `node --test scripts/tests/verify-m055-s01-contract.test.mjs`.
3. **Expected:** The contract test fails closed and names the missing/stale marker instead of letting the drift survive into later extraction slices.

### Whole-app landing `tsc --noEmit` is not the authoritative S01 gate yet

1. Run:

   ```bash
   ./mesher/landing/node_modules/.bin/tsc --noEmit -p mesher/landing/tsconfig.json
   ```
2. Confirm the failure still comes from the known baseline files:
   - `mesher/landing/components/blog/editor.tsx`
   - `mesher/landing/components/landing/infrastructure.tsx`
   - `mesher/landing/components/landing/mesh-dataflow.tsx`
3. Run `npm --prefix mesher/landing run build`.
4. **Expected:** The standalone build succeeds and remains the truthful S01 consumer-facing seam until the unrelated whole-app TypeScript baseline debt is fixed.

## Failure Signals

- `node --test scripts/tests/verify-m055-s01-contract.test.mjs` reports missing two-repo markers, missing verifier discoverability text, malformed repo-identity JSON, or mixed language/product repo URLs.
- Either installer `diff -u` command reports drift between the editable installer source and the docs-served public copy.
- `python3 scripts/lib/m034_public_surface_contract.py local-docs --root .` reports repo-identity drift in docs, installers, or VS Code metadata.
- `.tmp/m055-s01/verify/status.txt` is not `ok`, `current-phase.txt` is not `complete`, or `phase-report.txt` is missing a required passed phase.
- The wrapper’s `m055-s01-gsd-regression` phase fails, which means the repo-local `.gsd` seam no longer matches the retained M046 tiny-cluster contract.

## Requirements Advanced By This UAT

- R120 — advances the coherent public Mesh story by forcing packages/docs/installers/editor surfaces to stay language-owned while the landing/product surface stays product-owned.

## Not Proven By This UAT

- This UAT does not prove the deeper S02 toolchain contract for running the extracted product repo against a sibling `mesh-lang` checkout.
- This UAT does not extract `mesher/` into `hyperpush-mono`; it only establishes the ownership and verification boundary that later slices must follow.
- This UAT does not fix the whole-app landing TypeScript baseline; it only records the truthful build seam for S01.

## Notes for Tester

Start with `.tmp/m055-s01/verify/phase-report.txt` if the assembled rail fails. That file tells you which boundary drifted first. If the red phase is `m055-s01-gsd-regression`, rerun `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_contract_remains_source_first_and_route_free -- --nocapture` directly before touching `scripts/fixtures/clustered/tiny-cluster/`, `.gsd/milestones/M046/slices/S03/S03-PLAN.md`, or `scripts/verify-m046-s03.sh`.
