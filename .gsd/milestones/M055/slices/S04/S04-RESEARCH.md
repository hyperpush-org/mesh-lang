# M055/S04 — Research

**Date:** 2026-04-06

## Summary

S04 is not blocked on “how to split repos” in the abstract. The hard part is narrower: the current tree already publishes a future `hyperpush-mono` handoff, but the product-side local toolchain contract, compatibility wrappers, hosted-evidence scripts, and extraction shape do **not** yet agree on what that repo actually looks like after extraction.

This slice directly supports the active visible contract around:

- **R116 / R117 / R118** — evaluator-facing examples and clustered docs stay language-owned and starter-first.
- **R119** — Mesher/Hyperpush remains the maintained deeper app with a truthful maintainer runbook and verifier.
- **R120** — language docs/packages and the product landing must still read as one coherent Mesh story after the split.
- **R121** — packages + registry remain part of the normal language deploy/release proof.
- **R122** — SQLite-local vs PostgreSQL-deployable starter boundaries stay honest and must not be blurred by the product extraction.

The highest-risk discovery is a **shape mismatch**:

- Public/docs identity currently says the product handoff lives at **`https://github.com/hyperpush-org/hyperpush-mono/blob/main/mesher/README.md`** and the product-owned verifier is **`bash mesher/scripts/verify-maintainer-surface.sh`**.
- But the Mesher toolchain resolver in `mesher/scripts/lib/mesh-toolchain.sh` and the slice-owned contract tests in `scripts/tests/verify-m055-s02-contract.test.mjs` only work if the extracted product package root is a direct sibling of `mesh-lang` and resolves Mesh tooling from **`../mesh-lang`**.

Those two assumptions cannot both be true if the blessed workspace is:

```text
<workspace>/
  mesh-lang/
  hyperpush-mono/
```

and the Hyperpush repo keeps `mesher/` as a subdirectory. In that layout, `mesher/scripts/lib/mesh-toolchain.sh` would need to find Mesh tooling at `../../mesh-lang`, not `../mesh-lang`.

So the first planning decision for S04 is not cosmetic. It is:

1. **Keep the public/product contract as-is** (`hyperpush-mono` repo contains `mesher/`), then update the Mesher toolchain resolver/tests/wrappers to that nested layout.
2. Or flatten the product repo so its root is the Mesher package root, then change the already-published handoff docs/URLs/commands away from `mesher/README.md` and `bash mesher/scripts/...`.

Given S01–S03 already published the nested `hyperpush-mono/blob/main/mesher/...` handoff widely, the lowest-risk path is **option 1**: keep `mesher/` nested inside `hyperpush-mono`, and fix the local toolchain/workspace contract to match.

A second concrete risk: this repo is **not extraction-clean today**. `mesher/` contains generated and local-state content (`mesher/mesher`, `mesher/mesher.ll`, `.next`, `node_modules`, test-results, `.env.local`, nested `mesher/frontend-exp/.git`), and the current working tree also has unrelated landing WIP. S04 should not do a raw recursive copy. It needs an explicit extraction/materialization allowlist.

## Recommendation

Plan S04 as four execution seams, in this order:

1. **Decide and harden the extracted Hyperpush repo shape.**
   - Preserve the already-published public handoff: `hyperpush-mono` repo with `mesher/README.md` and `mesher/scripts/verify-maintainer-surface.sh`.
   - Update Mesher’s sibling-toolchain resolver, README, and contract tests to look for the blessed sibling `mesh-lang/` from that nested location.
   - Retarget the mesh-lang compatibility wrapper (`scripts/verify-m051-s01.sh`) so it no longer depends on an in-repo `mesher/` copy.

2. **Build a fail-closed extraction/materialization path.**
   - Do not “move `mesher/` by hand.” Use a generated-tree pattern with an allowlist and manifest, similar to the existing example materializer and retained-bundle scripts.
   - The extraction path should exclude nested `.git`, build outputs, `.next`, `node_modules`, transient logs, and other workspace-local debris.

3. **Split proof ownership cleanly.**
   - `mesh-lang` keeps language-owned proofs: live proof, starter failover proof, deploy-services (registry/packages), release/install/docs/public-surface rails.
   - `hyperpush-mono` must gain its own product-owned proof entrypoints for at least:
     - Mesher maintainer surface
     - landing build/deploy surface
   - The mesh-lang compatibility wrappers should become sibling-repo coordinators, not hidden local delegates.

4. **Assemble one cross-repo evidence chain without pretending one repo owns all artifacts.**
   - The assembled S04 verifier should point at repo-local outputs from both repos and record which repo/ref produced which proof.
   - Follow the `github-workflows` skill rule here: workflow changes need observable proof, not just YAML edits. Reuse repo-owned verifiers and hosted evidence artifacts rather than inventing a new umbrella CI story.

From the `using-git-worktrees` skill, this slice should run in an isolated worktree/clean baseline, not in a dirty main checkout. That matters here because the current tree has unrelated `mesher/landing` changes and generated product artifacts; extraction work will be impossible to reason about otherwise.

## Implementation Landscape

### Key files and what they mean

- `WORKSPACE.md`
  - Blessed workspace contract: only `mesh-lang/` + `hyperpush-mono/` siblings.
  - Says product work belongs in `hyperpush-mono`, but does **not** yet describe the concrete extracted repo shape beyond that.

- `scripts/lib/repo-identity.json`
  - Canonical public identity contract.
  - Today it publishes:
    - `languageRepo.workspaceDir = mesh-lang`
    - `productRepo.workspaceDir = hyperpush-mono`
    - `productHandoff.relativeRunbookPath = mesher/README.md`
  - This is the strongest visible signal that the future product repo keeps `mesher/` as a nested path.

- `mesher/scripts/lib/mesh-toolchain.sh`
  - Current product-side toolchain resolver.
  - Good news: it is already package-local and fail-closed.
  - Bad news: its sibling fallback is `MESHER_PACKAGE_DIR/../mesh-lang`, which only works if the extracted package root is itself a direct sibling of `mesh-lang`.

- `mesher/README.md`
  - Current deeper-app runbook.
  - Same mismatch as above: it tells maintainers to work from `cd mesher` and says sibling toolchain resolution is `../mesh-lang`.
  - This must be reconciled with the published `hyperpush-mono/blob/main/mesher/README.md` handoff.

- `scripts/tests/verify-m055-s02-contract.test.mjs`
  - The slice-owned Mesher contract test.
  - Its extraction simulation installs the product package at `tmpRoot/mesher` and the language repo at `tmpRoot/mesh-lang`.
  - That test shape confirms the current package-local resolver was never updated to the nested `hyperpush-mono/mesher` interpretation.

- `scripts/verify-m051-s01.sh`
  - Mesh-lang compatibility wrapper for the Mesher maintainer contract.
  - Still hardcodes local delegation to `mesher/scripts/verify-maintainer-surface.sh` under the current repo root.
  - After extraction, this is no longer truthful unless it explicitly reaches into the sibling Hyperpush repo.

- `website/docs/docs/production-backend-proof/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `README.md`
- `compiler/mesh-pkg/src/scaffold.rs`
  - All of these already publish the product handoff as Hyperpush repo + `mesher/README.md` + `bash mesher/scripts/verify-maintainer-surface.sh`.
  - This is why flattening the product repo would be expensive and contract-breaking.

- `scripts/verify-production-proof-surface.sh`
- `scripts/verify-m051-s04.sh`
- `scripts/verify-m047-s06.sh`
- `compiler/meshc/tests/e2e_m051_s04.rs`
- `compiler/meshc/tests/e2e_m047_s06.rs`
  - These retained docs rails pin the public handoff literally.
  - If the product repo shape changes, these all need coordinated updates.

- `scripts/verify-m053-s03.sh`
  - Hosted evidence verifier for language-owned main/tag workflows.
  - Important nuance: it derives the repo slug from `git remote get-url origin` unless `M053_S03_GH_REPO` is set.
  - In this checkout, `origin` already points at `hyperpush-org/hyperpush-mono.git`, while the language/public contract still wants `snowdamiz/mesh-lang`.
  - So origin-derived hosted evidence is already unsafe for language-owned proof unless explicitly overridden.

- `.github/workflows/deploy-services.yml`
  - Now language-owned only: registry + packages website + public-surface health checks.
  - Landing is already removed, so S04 must create the product-side replacement instead of putting landing back here.

- `.github/workflows/authoritative-verification.yml`
- `.github/workflows/authoritative-live-proof.yml`
- `.github/workflows/authoritative-starter-failover-proof.yml`
- `.github/workflows/release.yml`
  - Language-owned proof graph.
  - These remain in mesh-lang; S04 should not blur them with product continuity proof.

- `.github/dependabot.yml`
  - Still includes `/mesher/landing` in the current repo.
  - Another signal that product dependency ownership has not yet been extracted.

- `mesher/landing/lib/external-links.ts`
  - Already points to `hyperpush-org/hyperpush-mono`.
  - Product public identity is already split-aware here.

- `packages-website/src/routes/+layout.svelte`
  - Still language-owned and intentionally points to `snowdamiz/mesh-lang` + `WORKSPACE.md`.
  - This is correct and should remain language-owned in S04.

### Extraction cleanliness / what cannot be copied blindly

`mesher/` currently contains non-source or nested-repo content that should not be treated as extractable source of truth:

- `mesher/frontend-exp/.git` — nested Git repo
- `mesher/mesher`
- `mesher/mesher.ll`
- `mesher/frontend-exp/node_modules`
- `mesher/frontend-exp/.next`
- `mesher/landing/node_modules`
- `mesher/landing/.next`
- `mesher/landing/test-results`
- `mesher/landing/.env.local`
- other local state / transient files (`.DS_Store`, `.bg-shell`, etc.)

This is why S04 needs a materializer/allowlist, not `cp -R mesher ../hyperpush-mono`.

### Existing patterns worth reusing

Do **not** invent new extraction mechanics from scratch. The repo already has good patterns:

- `scripts/tests/verify-m049-s03-materialize-examples.mjs`
  - Mature generated-tree / compare-tree / fail-closed materializer pattern.
  - Good model for “generate expected repo tree and compare or write” behavior.

- `scripts/verify-m055-s03.sh`
- `scripts/verify-m051-s02.sh`
- `mesher/scripts/verify-maintainer-surface.sh`
  - Established retained-bundle patterns:
    - `status.txt`
    - `current-phase.txt`
    - `phase-report.txt`
    - `full-contract.log`
    - `latest-proof-bundle.txt`
    - `copy_fixed_dir_or_fail`
    - `copy_new_prefixed_artifacts_or_fail`
  - S04 should follow this evidence style rather than inventing a new artifact grammar.

## Build Order

### 1. Resolve the repo-shape contradiction first

This is the planner’s first task. Until resolved, all later extraction/evidence work is unstable.

Minimum files in that decision seam:

- `scripts/lib/repo-identity.json`
- `mesher/scripts/lib/mesh-toolchain.sh`
- `mesher/README.md`
- `scripts/tests/verify-m055-s02-contract.test.mjs`
- `scripts/verify-m051-s01.sh`
- probably `WORKSPACE.md` if command examples need to clarify the nested product layout

Recommended direction:

- Keep public handoff shape as `hyperpush-mono/blob/main/mesher/...`
- Update product-side local toolchain resolution and compatibility wrapper logic to match that

### 2. Add a product extraction/materialization seam

Goal: generate a clean Hyperpush repo tree from the language repo’s current product-owned source set.

Likely new or changed surfaces:

- new extraction/materializer script under `scripts/` or `scripts/tests/`
- new contract test for allowed/excluded paths
- maybe a tracked manifest of extracted files / required root files

This task should also decide whether S04 writes a real sibling repo tree, a staged bundle under `.tmp/`, or both.

### 3. Split proof entrypoints by repo ownership

Mesh-lang side:

- keep:
  - `scripts/verify-m034-s05.sh`
  - `scripts/verify-m053-s03.sh`
  - language-owned workflows
  - public docs/starter proof pages
- change:
  - `scripts/verify-m051-s01.sh` to become a sibling-repo compatibility coordinator

Product side (future extracted repo):

- own:
  - Mesher maintainer verifier
  - landing build/deploy verifier/workflow
  - any product repo metadata/workspace docs

### 4. Assemble a two-repo evidence bundle

This should be the final slice wrapper.

It should prove:

- which **mesh-lang repo/ref** proved language continuity
- which **hyperpush-mono repo/ref** proved product continuity
- where the retained artifacts live for both sides

The current repo already has a good model for phase-marked assembled wrappers; reuse it.

## Verification Approach

### Mesh-lang local verification that must remain green

These are the language-owned rails S04 must preserve:

- `bash scripts/verify-m055-s01.sh`
- `bash scripts/verify-m055-s03.sh`
- `bash scripts/verify-production-proof-surface.sh`
- `bash scripts/verify-m053-s03.sh` (with explicit repo override if needed)
- relevant retained wrappers if public handoff text changes:
  - `bash scripts/verify-m051-s04.sh`
  - `bash scripts/verify-m047-s06.sh`

If the product handoff command/path changes, the paired Rust/node contract tests must move with the shell wrappers and docs verifiers.

### Product-side local verification that must survive extraction

Current source of truth to preserve when moved:

- `bash mesher/scripts/test.sh`
- `bash mesher/scripts/migrate.sh status`
- `bash mesher/scripts/migrate.sh up`
- `bash mesher/scripts/build.sh <bundle-dir>`
- `bash mesher/scripts/smoke.sh`
- `bash mesher/scripts/verify-maintainer-surface.sh`

After extraction, these should run from the actual Hyperpush repo layout, not from a mesh-lang-local copy.

### Two-repo evidence verification

The slice-done wrapper should fail closed if any of these drift:

- mesh-lang repo/ref for language-owned hosted evidence
- hyperpush repo/ref for product-owned hosted evidence
- missing sibling repo
- missing product-owned verifier path
- stale public handoff docs/README/scaffold paths
- retained bundle pointers/artifact manifests

Given the `github-workflows` skill rule, workflow ownership changes are not done when YAML parses; they are done when repo-owned verifiers show the expected jobs/steps and hosted evidence is attributable to the correct repo/ref.

## Constraints

- **Current origin remote is misleading for language proof.** `git remote get-url origin` returns `https://github.com/hyperpush-org/hyperpush-mono.git` in this checkout, but language-owned public identity still intentionally points to `snowdamiz/mesh-lang`. Any language hosted-evidence script that trusts `origin` is unsafe unless explicitly overridden.

- **Current public contract already hardcodes nested Hyperpush paths.** `mesher/README.md` and `bash mesher/scripts/verify-maintainer-surface.sh` are already published across docs, scaffold output, and contract tests.

- **Current Mesher sibling-toolchain contract does not match that nested shape.** The resolver and tests still assume extracted package root sibling layout.

- **Raw extraction is unsafe.** `mesher/` contains generated binaries, nested Git state, local env files, and dependency caches.

- **There are many older retained tests/scripts that still reference `meshc build mesher` / `meshc migrate mesher`.** These are outside the narrow M055 surface but will become immediate breakage if `mesher/` simply disappears from this repo. Representative examples include `compiler/meshc/tests/e2e_m033_s0{1,2,3,4}.rs` and older `scripts/verify-m033-*` / `verify-m032-*` rails.

- **Current working tree is dirty with unrelated changes.** This slice should not be executed in-place on the current checkout.

## Common Pitfalls

- **Changing public handoff docs before fixing the repo shape.** That would churn README/docs/scaffold/contract tests while still leaving the local Mesher toolchain contract wrong.

- **Deleting `mesher/` from mesh-lang before retargeting `scripts/verify-m051-s01.sh`.** The current compatibility wrapper would just break immediately.

- **Trusting `origin` to mean “language repo.”** In this checkout it already means the product repo.

- **Copying `mesher/` recursively.** That will pull nested `.git`, build outputs, `.next`, `node_modules`, and local state into the extracted repo.

- **Reintroducing landing into mesh-lang workflows.** S03 explicitly removed it; S04 should create a product-owned replacement, not unwind that decision.

- **Letting first-contact docs absorb product-repo details.** R116/R117/R118/R122 are already in a good place. S04 should preserve the starter/examples-first boundary.

## Open Risks

- The extracted Hyperpush repo root shape is still the main decision/risk. The public contract currently points one way; the local toolchain contract points another.

- Broad historical Mesher references outside active M055 rails may create more cleanup than the slice title suggests if the planner tries to make the whole repo Mesher-free in one pass.

- Product hosted workflows do not yet exist in this repo as a replacement for the removed landing job. S04 needs to define that ownership explicitly.

- If the slice tries to solve cross-repo coordination with a new umbrella orchestration layer, it will overshoot. The established repo-local verifier/bundle pattern is already enough.

## Skills Discovered

| Technology | Skill | Status | Relevance |
|---|---|---|---|
| GitHub Actions / hosted workflow evidence | `github-workflows` | available and consulted | Relevant for splitting repo-owned workflow proof and keeping hosted evidence observable. |
| Repo boundary / extraction planning | `wshobson/agents@monorepo-management` | installed during research | Relevant for deciding explicit ownership boundaries and avoiding ad hoc extraction. |
| Isolated multi-checkout execution | `obra/superpowers@using-git-worktrees` | installed during research | Relevant because S04 should run in an isolated worktree / clean baseline, not this dirty checkout. |
