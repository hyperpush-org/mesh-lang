# S05 Research — Full public release assembly proof

## Summary

S05 is **deep/targeted integration research**. The core technologies are already in-tree and most subsystem proof surfaces already exist, but there is **no canonical assembly verifier** yet. The slice is mainly about composing existing truths, covering the remaining deploy/docs gaps, and making the final public-ready bar explicit instead of inferred.

The strongest architecture match is **M033/S05’s serial-wrapper pattern**: one repo-local verifier that runs existing slice-owned verifiers in order, adds only the missing S05-owned checks, and leaves first-failing-phase logs under a deterministic `.tmp/` root.

Biggest findings:

1. **No S05 verifier exists yet.** The repo has good component verifiers (`m034-s01`, `s02-workflows`, `m034-s03`, `m034-s04-extension`, `m034-s04-workflows`), but nothing assembles them.
2. **`deploy.yml` and `deploy-services.yml` are not locally contract-verified today.** S02/S04 hardened release and extension workflow truth, but the docs/Fly deploy workflows still only have basic YAML/runtime coverage.
3. **The current live public docs surface is stale in important places.** During research, `https://meshlang.dev/docs/getting-started/` still advertised source-build as the verified install path, and `https://meshlang.dev/install.ps1` still served `$Repo = "mesh-lang/mesh"` even though local files are already fixed. A root `200 OK` is not enough for S05.
4. **Remote hosted-workflow evidence for S02/S04 is still unavailable because the workflows are not on the remote default branch yet.** `gh run list --workflow authoritative-verification.yml --limit 1` and `gh run list --workflow extension-release-proof.yml --limit 1` both return GitHub 404 today.
5. **Release-candidate identity needs an explicit policy.** Local `release.yml` now requires `v< Cargo version >`, but the latest public binary release is still `v14.3` while `meshc`/`meshpkg` are `0.1.0`. The extension remains independently versioned at `0.3.0` with `ext-v*` tags.

## Requirements Targeting

The active requirement file in the working tree does not currently contain R045–R047 entries, but the milestone context explicitly names them as the M034 requirement pressure. Plan S05 against those context-owned requirements.

### Direct / primary

- **R045** — CI/CD and release flows must prove shipped Mesh surfaces instead of only building artifacts.
  - S05 is the milestone-level assembly slice for this requirement.
  - The remaining uncovered surfaces are: deploy workflow truth, exact deployed docs truth, and one canonical assembled verifier/runbook.
- **R046** — the package manager must be tested end to end on the real path.
  - S01/S02 already proved the real registry flow.
  - S05 should **reuse** that live proof and compose it into the release-candidate story instead of inventing a second registry verifier.

### Supporting

- **R047** — editor trust advances via hardened extension release truth, without claiming full editor parity.
  - S04 already built the extension proof lane.
  - S05 should consume that exact proof surface and include it in the assembled public-ready flow.

### Not primary

- **R021** is still deferred in `REQUIREMENTS.md`; it is not the active planning anchor for this slice.

## Skills Discovered

- **Existing installed skill:** `github-workflows`
  - Most relevant rule: **“No errors is not validation. Prove observable change.”**
  - For S05 this means: do not treat parsed YAML, green builds, or root-level `200` responses as sufficient. The assembled verifier needs exact public URLs, exact workflow graphs, and exact proof outputs.
- **Existing installed skill:** `vitepress`
  - Most relevant rule: `website/docs/public/` is the real docs-served static surface and `.vitepress/config.mts` is the deployment/build boundary.
  - For S05 this matters because the public install scripts and docs pages must be validated as deployed content, not only as local source files.
- **Installed during research for downstream units:** `thinkfleetai/thinkfleet-engine@flyio-cli-public` (`flyio-cli-public`)
  - Installed with: `npx skills add thinkfleetai/thinkfleet-engine@flyio-cli-public -g -y`
  - This was the only directly relevant missing skill for the Fly deploy half of the slice.

Planner note: the `github-workflows` skill assumes a repo-local `scripts/ci_monitor.cjs` helper, but that file does **not** exist in this repo. If S05 needs remote GitHub Actions evidence, it will need either direct `gh run ...` commands or a new repo-local helper.

## Implementation Landscape

### Existing proof surfaces to compose

#### `scripts/verify-m034-s01.sh`
- Owner of the **real live registry proof**.
- Covers: publish, metadata, versions list, search, download, checksum, install, `mesh.lock`, named install behavior, consumer build/run, duplicate publish rejection, and public packages-site detail/search visibility.
- Important for S05:
  - already hits the real registry and real packages site
  - already uses the correct full scoped search query
  - already produces per-phase artifacts under `.tmp/m034-s01/verify/<version>/`

#### `scripts/verify-m034-s02-workflows.sh`
- Local contract verifier for:
  - `.github/workflows/authoritative-live-proof.yml`
  - `.github/workflows/authoritative-verification.yml`
  - `.github/workflows/release.yml`
- Important for S05:
  - already proves the release graph, authoritative live proof reuse, and installer smoke wiring
  - **does not** cover `deploy.yml` or `deploy-services.yml`

#### `scripts/verify-m034-s03.sh` / `scripts/verify-m034-s03.ps1`
- Owner of the **staged installer/release-asset proof**.
- Covers canonical `website/docs/public/install.{sh,ps1}`, staged `SHA256SUMS`, both binaries, and negative failure cases.
- Important for S05:
  - already proves the local/staged installer contract
  - is not enough by itself to prove the **deployed** docs-served installer content

#### `scripts/verify-m034-s04-extension.sh`
- Owner of the **extension artifact/prepublish proof**.
- Covers: deterministic VSIX path, prereq drift, docs/package-script drift, archive audit, and shared `e2e_lsp` prerequisite.
- Important for S05:
  - already gives a reusable extension proof surface
  - does not verify remote marketplace run evidence on its own

#### `scripts/verify-m034-s04-workflows.sh`
- Local contract verifier for:
  - `.github/workflows/extension-release-proof.yml`
  - `.github/workflows/publish-extension.yml`
- Important for S05:
  - already gives the extension workflow half of the assembled proof

### Workflow surfaces that matter

#### `.github/workflows/release.yml`
- Current local graph includes:
  - `build`
  - `build-meshpkg`
  - `authoritative-live-proof`
  - `verify-release-assets`
  - `release`
- Tag-only live proof and tag-only release job are already wired.
- Important S05 constraint:
  - the workflow now fails any `v*` tag that does not match Cargo version `0.1.0`

#### `.github/workflows/deploy-services.yml`
- Deploys `registry/` and `packages-website/` to Fly on `push.tags: v*` and `workflow_dispatch`.
- Current post-deploy checks only curl:
  - `https://api.packages.meshlang.dev/api/v1/packages`
  - `https://packages.meshlang.dev`
  - `https://meshlang.dev`
- S05 implication:
  - these checks are too weak for the slice’s bar
  - they do not confirm installer files, docs pages, proof-package detail/search pages, or registry search semantics

#### `.github/workflows/deploy.yml`
- Builds and deploys the docs site on `main`, `v*`, and `workflow_dispatch`.
- Current local coverage is only build/deploy shape; there is no S05-style docs-truth verifier for the deployed public site.

#### `.github/workflows/publish-extension.yml`
- Thin `ext-v*` publisher that already depends on `.github/workflows/extension-release-proof.yml`.
- S05 should reuse this as-is rather than re-implement its publication logic.

### Live public surface observed during research

#### Docs / installers
- `curl -I https://meshlang.dev` returned `200`.
- `curl -I https://meshlang.dev/install.sh` returned `200` and the served file still contains the correct Unix installer contract (`REPO="snowdamiz/mesh-lang"`, installs `meshpkg`).
- `curl -L https://meshlang.dev/install.ps1` still serves the **old** Windows script with:
  - `$Repo = "mesh-lang/mesh"`
- `fetch_page https://meshlang.dev/docs/getting-started/` still shows the stale public text:
  - `Today the verified install path is building meshc from source`
- Local files are already newer:
  - `website/docs/public/install.ps1` is fixed locally
  - `website/docs/docs/getting-started/index.md` is fixed locally
- S05 implication:
  - final proof must check **deployed content**, not only local docs builds or docs-site root availability

#### Packages site / registry
- `curl -I https://packages.meshlang.dev` returned `200`.
- `curl -I https://api.packages.meshlang.dev/api/v1/packages` returned `200`.
- `https://packages.meshlang.dev/packages/snowdamiz/mesh-registry-proof` renders the S01 proof package live.
- `https://packages.meshlang.dev/search?q=snowdamiz%2Fmesh-registry-proof` renders **1 result** and the proof package card.
- `https://api.packages.meshlang.dev/api/v1/packages?search=snowdamiz%2Fmesh-registry-proof` returns the proof package.
- `https://api.packages.meshlang.dev/api/v1/packages?search=mesh-registry-proof` returns `[]`.
- S05 implication:
  - reuse the **full scoped query** from S01; do not invent a slug-only packages search probe

### Remote GitHub evidence state

Observed with `gh` during research:

- `gh run list --workflow authoritative-verification.yml --limit 1` → **404 workflow not found on default branch**
- `gh run list --workflow extension-release-proof.yml --limit 1` → **404 workflow not found on default branch**
- `gh run list --workflow deploy.yml --limit 1` → latest remote run exists and succeeded
- `gh run list --workflow deploy-services.yml --limit 1` → latest remote run exists and succeeded
- `gh run list --workflow release.yml --limit 1` → latest remote run exists, but its job graph is the old one without the new S02/S03 proof jobs
- `gh run list --workflow publish-extension.yml --limit 1` → latest remote run exists, but it is the older single-job publish graph

S05 implication:
- The local code is ahead of the remote default branch.
- Any final assembled proof that depends on hosted-run evidence must either:
  - push the updated workflows first, or
  - honestly fail/mark pending until rollout happens.

### Version / candidate constraints

Current versions in the working tree:
- `compiler/meshc/Cargo.toml` → `0.1.0`
- `compiler/meshpkg/Cargo.toml` → `0.1.0`
- `tools/editors/vscode-mesh/package.json` → `0.3.0`

Current public binary release observed during research:
- `gh release view` → latest public release is still `v14.3`

Local workflow rule now enforced by `release.yml`:
- `v*` tag must exactly match the Cargo version

S05 implication:
- The slice cannot assume the historical `v14.x` release naming still works.
- The assembled proof needs an explicit candidate identity policy:
  - likely `v0.1.0` (or another Cargo-aligned binary tag) for the binary release flow
  - `ext-v0.3.0` for the extension publish flow
- Do **not** try to invent a fake single-version story if the release and extension are still intentionally independent.

## Key Findings

### 1) M033/S05 is the right structural precedent

Decision D068 from M033 used a **serial wrapper over existing slice verifiers plus a docs-truth sweep** instead of inventing a new runtime harness. S05 has the same shape:
- S01 already owns the live registry proof
- S02 already owns release/workflow truth for binaries
- S03 already owns installer/release-asset truth
- S04 already owns extension proof/workflow truth

The missing work is **assembly**, not a fifth independent subsystem harness.

### 2) Deploy workflows are the uncovered local seam

There is strong local contract coverage for:
- release/workflow proof (`verify-m034-s02-workflows.sh`)
- extension workflow proof (`verify-m034-s04-workflows.sh`)

There is **no equivalent local verifier** for:
- `.github/workflows/deploy.yml`
- `.github/workflows/deploy-services.yml`

S05 should either:
- add a small dedicated `scripts/verify-m034-s05-workflows.sh`, or
- fold deploy-workflow contract checks into the assembled S05 wrapper

but it should not leave the deploy half of the public story as unchecked YAML plus root curls.

### 3) Root-level health checks are too weak for the public-ready bar

The current live site proved this directly during research:
- `https://meshlang.dev` is healthy
- but `https://meshlang.dev/install.ps1` is stale
- and `https://meshlang.dev/docs/getting-started/` is stale

So S05 should check **exact deployed public content**, not just host reachability.

Minimum public docs/install checks worth enforcing:
- installer shell script content
- installer PowerShell script content
- getting-started install guidance
- tooling page package-manager / installer guidance

### 4) Hosted evidence is still blocked by rollout, not by local code understanding

The new authoritative verification and reusable extension proof workflows are valid locally, but GitHub does not know about them on the default branch yet.

This is not a local research gap. It is a **rollout dependency** that S05 must either:
- explicitly plan for, or
- treat as a final closeout gate after the local assembled verifier lands.

### 5) Release candidate naming is a real planning decision, not a cosmetic detail

The repo currently has:
- historical public release tags like `v14.3`
- Cargo versions at `0.1.0`
- local release workflow enforcement that tag == Cargo version
- extension version at `0.3.0`

S05 should not ignore this. Any real final proof that uses hosted release/deploy workflows needs a chosen tag policy first.

### 6) Potential remaining public-metadata gap outside S04’s current verifier

`tools/editors/vscode-mesh/package.json` still points:
- `repository.url` → `https://github.com/mesh-lang/mesh.git`
- `bugs.url` → `https://github.com/mesh-lang/mesh/issues`

That does not match the current repo (`snowdamiz/mesh-lang`). S04 did not verify these fields. Decide explicitly whether S05 fixes them or scopes them out of the public-ready claim.

## Recommendation

### Recommended architecture

Create **`scripts/verify-m034-s05.sh`** as the one canonical S05 acceptance command.

Use the M033/S05 pattern:
- strictly serial execution
- deterministic artifact root like `.tmp/m034-s05/verify/`
- named phase logs
- first-failing-phase reporting
- reuse existing slice-owned verifiers unchanged where possible

Add only the missing S05-owned checks:
- docs build + exact docs/public truth sweep
- deploy workflow contract coverage
- exact deployed-content HTTP checks
- remote hosted-run evidence checks after rollout

### Recommended phase order for `scripts/verify-m034-s05.sh`

1. **`docs-build`**
   - `npm --prefix website run build`
   - keep serial; do not overlap with another VitePress build (`KNOWLEDGE.md` already records the `.vitepress/.temp` race)
2. **`docs-truth`**
   - exact-string sweep over the local public contract
   - likely files:
     - `README.md`
     - `website/docs/docs/getting-started/index.md`
     - `website/docs/docs/tooling/index.md`
     - `website/docs/public/install.sh`
     - `website/docs/public/install.ps1`
3. **`s02`**
   - `bash scripts/verify-m034-s02-workflows.sh`
4. **`s03`**
   - `bash scripts/verify-m034-s03.sh`
5. **`s04-proof`**
   - `EXPECTED_TAG="ext-v$(node -p \"require('./tools/editors/vscode-mesh/package.json').version\")" bash scripts/verify-m034-s04-extension.sh`
6. **`s04-workflows`**
   - `bash scripts/verify-m034-s04-workflows.sh`
7. **`public-http`**
   - exact live URL/content checks against:
     - `https://meshlang.dev/install.sh`
     - `https://meshlang.dev/install.ps1`
     - `https://meshlang.dev/docs/getting-started/`
     - `https://meshlang.dev/docs/tooling/`
     - `https://packages.meshlang.dev/packages/snowdamiz/mesh-registry-proof`
     - `https://packages.meshlang.dev/search?q=snowdamiz%2Fmesh-registry-proof`
     - `https://api.packages.meshlang.dev/api/v1/packages?search=snowdamiz%2Fmesh-registry-proof`
8. **`live-s01`**
   - `set -a && source .env && set +a && bash scripts/verify-m034-s01.sh`
   - this should remain the only owner of the live publish/install proof
9. **`remote-runs`**
   - use `gh run list/view` to verify hosted workflow evidence once the branch and candidate tags are actually pushed

### Natural seams for planning

#### Seam A — canonical S05 wrapper
**Files likely touched**
- new `scripts/verify-m034-s05.sh`
- maybe `.tmp/m034-s05/verify/*`

**Goal**
- one assembled proof entrypoint with named phases and localized failure logs

**Risk**
- medium; mostly composition work

#### Seam B — docs/public truth sweep
**Files likely touched**
- `README.md`
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/tooling/index.md`
- maybe a new public proof page/runbook if the slice wants a docs-owned canonical explanation

**Goal**
- make the local public contract match exactly what S05 enforces

**Risk**
- medium because the deployed public docs are currently stale and the slice must decide how much user-facing proof/runbook text it wants to own

#### Seam C — deploy workflow contract coverage
**Files likely touched**
- maybe new `scripts/verify-m034-s05-workflows.sh`
- `.github/workflows/deploy.yml`
- `.github/workflows/deploy-services.yml`

**Goal**
- mechanically reject drift in the docs deploy and Fly deploy workflows

**Risk**
- medium; this is the main local coverage gap left after S02/S04

#### Seam D — remote rollout / candidate policy
**Files likely touched**
- possibly `scripts/verify-m034-s05.sh`
- maybe `.gsd/KNOWLEDGE.md` or docs if the tag/version policy needs to be made explicit

**Goal**
- define which hosted runs, tags, and public URLs count as the assembled release candidate evidence

**Risk**
- high; depends on remote rollout and on resolving `v14.x` vs `0.1.0` binary tag policy

### What to build or prove first

1. **Build the local assembled wrapper first.**
   - This gives S05 a stable acceptance surface before any rollout work.
2. **Add deploy/docs truth coverage next.**
   - That is the remaining uncovered local seam.
3. **Only after local assembly is green, attempt remote rollout evidence / real candidate tags.**
   - This avoids spending irreversible publication/deploy effort on problems that local composition could have found earlier.
4. **Decide the binary tag policy before any real candidate release run.**
   - The local workflow and the historical public tags currently disagree.

## Don’t Hand-Roll

- **Do not merge** `release.yml`, `deploy.yml`, `deploy-services.yml`, and `publish-extension.yml` into one mega-workflow. The milestone context and S02 research both say the responsibilities should stay split.
- **Do not rewrite** S01/S03/S04 proof logic inline in YAML. Reuse the repo-local verifier scripts unchanged.
- **Do not accept** root-level `200 OK` checks as sufficient public health. The live site already showed why that is a false green.
- **Do not probe package search by slug only.** Reuse the exact full scoped query pattern from S01.
- **Do not run VitePress builds concurrently** with the assembled verifier. `KNOWLEDGE.md` already records the `.vitepress/.temp` race.

## Verification Guidance

### Existing local checks that should stay in the assembled bundle

- `npm --prefix website run build`
- `bash scripts/verify-m034-s02-workflows.sh`
- `bash scripts/verify-m034-s03.sh`
- `EXPECTED_TAG="ext-v$(node -p "require('./tools/editors/vscode-mesh/package.json').version")" bash scripts/verify-m034-s04-extension.sh`
- `bash scripts/verify-m034-s04-workflows.sh`

### Existing live proof to reuse

- `set -a && source .env && set +a && bash scripts/verify-m034-s01.sh`

### Exact live URL/content checks worth making mechanical

- `curl -sSfL https://meshlang.dev/install.sh | rg 'snowdamiz/mesh-lang|meshpkg'`
- `curl -sSfL https://meshlang.dev/install.ps1 | rg 'snowdamiz/mesh-lang|meshpkg'`
  - **currently red on the live site** because the served PS1 still contains `mesh-lang/mesh`
- `fetch_page https://meshlang.dev/docs/getting-started/`
  - **currently red** because the deployed page still says source-build is the verified path
- `curl -sSf 'https://api.packages.meshlang.dev/api/v1/packages?search=snowdamiz%2Fmesh-registry-proof'`
- `curl -sSfL 'https://packages.meshlang.dev/packages/snowdamiz/mesh-registry-proof' | rg 'snowdamiz/mesh-registry-proof|v0.34.0'`
- `curl -sSfL 'https://packages.meshlang.dev/search?q=snowdamiz%2Fmesh-registry-proof' | rg '1 result|snowdamiz/mesh-registry-proof'`

### Remote workflow evidence after rollout

- `gh run list --workflow deploy.yml --limit 1`
- `gh run list --workflow deploy-services.yml --limit 1`
- `gh run list --workflow authoritative-verification.yml --limit 1`
  - currently 404 until the workflow is on the remote default branch
- `gh run list --workflow extension-release-proof.yml --limit 1`
  - currently 404 until the workflow is on the remote default branch
- `gh run view <run-id> --json jobs,workflowName,displayTitle,event,headBranch,conclusion,status,url`

## Open Questions / Planner Notes

- **Is S05 allowed to push/tag for real hosted evidence, or must it stop at local proof + live HTTP checks?**
  - Final hosted-run evidence is impossible without rollout.
- **What binary tag naming should count as the release candidate?**
  - Local workflow requires Cargo-aligned `v0.1.0`-style tags; historical public releases are still `v14.x`.
- **Should S05 fix the extension `repository` / `bugs` metadata URLs, or explicitly scope them out of the public-ready claim?**
- **Should S05 publish a public docs/runbook page for the assembled release proof, or keep the canonical entrypoint repo-local only?**
  - The milestone context only requires a canonical script/workflow entrypoint, not necessarily a public docs page, so this is a planning choice rather than a fixed requirement.
