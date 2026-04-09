# M055/S03 — Research

**Date:** 2026-04-06

## Summary

S03 is not an extraction slice. It is a **public-surface contract split** inside the current repo.

The language-owned side is already structurally present:
- generated evaluator-facing examples live under `examples/`
- starter generation lives in `compiler/mesh-pkg/src/scaffold.rs`
- public docs/install live under `website/docs/`
- packages/public-site lives under `packages-website/` and `registry/`

The remaining blocker is contract drift: public docs, scaffolded README text, skills, historical docs verifiers, and the hosted packages/public-surface proof still assume the deeper product handoff goes through local `mesher/...` paths and a monorepo-shaped `deploy-services.yml` that also owns `mesher/landing`.

That is exactly what S03 needs to remove.

## Requirements Focus

### R116 — evaluator-facing generated examples remain the public starting surface
This requirement is already strongly encoded and should be preserved, not redesigned.

Authoritative seams:
- `scripts/tests/verify-m049-s03-materialize-examples.mjs` — materializes `todo-sqlite` / `todo-postgres` from public CLI output and diffs them against `examples/`
- `compiler/meshc/tests/e2e_m049_s03.rs` — proves committed examples match scaffold output and stay runnable
- `examples/todo-sqlite/README.md`
- `examples/todo-postgres/README.md`

Planner implication: keep `examples/` language-owned in `mesh-lang`; do not invent a cross-repo example mirror for S03.

### R117 / R118 — docs must stay evaluator-facing and keep one clear clustered-app path
The first-contact ladder is already in decent shape, but the **deeper handoff** still leaks product-local source paths.

Public ladder files:
- `README.md`
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/production-backend-proof/index.md`
- `tools/skill/mesh/skills/clustering/SKILL.md`

Current problem: these surfaces keep the public starter ladder intact, but the deeper backend handoff still names:
- `mesher/README.md`
- `bash mesher/scripts/verify-maintainer-surface.sh`
- `bash scripts/verify-m051-s01.sh`
- `bash scripts/verify-m051-s02.sh`

That is a product-repo source-path assumption living in the language repo’s public contract.

### R119 — Mesher remains the maintained deeper reference app
S02 already created the right product-owned seam:
- `mesher/README.md`
- `mesher/scripts/test.sh`
- `mesher/scripts/migrate.sh`
- `mesher/scripts/build.sh`
- `mesher/scripts/smoke.sh`
- `mesher/scripts/verify-maintainer-surface.sh`
- `scripts/verify-m051-s01.sh` as compatibility wrapper only

Planner implication: S03 should **consume** that boundary, not redesign it. The language repo should stop teaching local `mesher/...` paths as if they were language-owned public surfaces.

### R120 / R121 — language/docs/packages/public-site story stays coherent and deployable
Current blocker: the hosted/public contract is still monorepo-coupled.

Coupled surfaces:
- `.github/workflows/deploy-services.yml`
- `scripts/verify-m034-s05.sh`
- `scripts/verify-m034-s05-workflows.sh`
- `scripts/tests/verify-m034-s05-contract.test.mjs`
- `scripts/verify-m053-s03.sh`
- `scripts/tests/verify-m053-s03-contract.test.mjs`
- `scripts/verify-m053-s04.sh`
- `scripts/tests/verify-m053-s04-contract.test.mjs`

Today `deploy-services.yml` still deploys and verifies:
- `registry/`
- `packages-website/`
- `mesher/landing`

and the workflow verifier explicitly requires the landing job plus `Verify hyperpush landing` health checks.

That blocks any truthful claim that `mesh-lang` stands on its own for the packages/public-site surface.

### R122 — SQLite-local vs Postgres-deployable starter boundary must stay honest
This boundary is already well encoded and should survive untouched except for handoff wording.

Authoritative files:
- `examples/todo-sqlite/README.md`
- `examples/todo-postgres/README.md`
- `compiler/mesh-pkg/src/scaffold.rs`
- `scripts/tests/verify-m054-s01-contract.test.mjs`
- `scripts/tests/verify-m053-s04-contract.test.mjs`

Planner implication: any docs/handoff rewrite must preserve the existing SQLite-local vs Postgres-deployable language exactly enough that these rails can be updated without weakening the contract.

## Recommendation

1. **Do not move `examples/`, `website/`, `packages-website/`, or `registry/` in S03.**
   `WORKSPACE.md` explicitly says M055 is a two-repo split only and that those surfaces remain language-owned in `mesh-lang` for this milestone.

2. **Replace product-local public handoff markers before touching hosted workflow assembly.**
   The highest-leverage source seam is the clustered scaffold README template in `compiler/mesh-pkg/src/scaffold.rs`, then the public-secondary docs/skill surfaces.

3. **Make the mesh-lang public contract stop at a product handoff boundary, not a local `mesher/` path.**
   S03 should make it possible for `mesh-lang` to be truthful without requiring product-repo source paths to exist in the same checkout. That likely means the public docs stop naming `mesher/README.md` directly and instead point to a product-owned handoff identity derived from `scripts/lib/repo-identity.json` or an equivalent centralized product-handoff marker.

4. **Split the hosted/public proof so language-owned packages/public-site checks no longer require landing deployment.**
   `deploy-services.yml` plus the `m034`/`m053` verifier stack are the main seam here.

5. **Close with a slice-owned assembled verifier.**
   The repo already uses narrow, phase-marked assembled wrappers. S03 should follow that pattern with a mesh-lang-only public-surface/starter wrapper rather than widening S01/S02 or reusing the monorepo-wide `m034` wrapper as-is.

## Implementation Landscape

### 1. Generated starter + example contract

- `compiler/mesh-pkg/src/scaffold.rs`
  - clustered README template still hardcodes the deeper handoff to `mesher/README.md`, `bash scripts/verify-m051-s01.sh`, and `bash scripts/verify-m051-s02.sh`
  - has in-file assertions around those strings near the clustered README tests
- `examples/todo-sqlite/README.md`
  - authoritative honest local-only starter contract
- `examples/todo-postgres/README.md`
  - authoritative serious shared/deployable starter contract
- `scripts/tests/verify-m049-s03-materialize-examples.mjs`
  - existing mechanical parity seam; reuse it
- `compiler/meshc/tests/e2e_m049_s03.rs`
  - existing runnable example proof; reuse it

Natural task seam: generated/public starter text first, parity proof second.

### 2. Public docs and skill surfaces

Public docs that still carry the product-local handoff:
- `README.md`
- `website/docs/docs/production-backend-proof/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/databases/index.md`
- `website/docs/docs/web/index.md`
- `website/docs/docs/concurrency/index.md`
- `website/docs/docs/testing/index.md`
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/tooling/index.md`
- `tools/skill/mesh/skills/clustering/SKILL.md`

Important contract facts already present and worth preserving:
- starter/examples-first ladder
- SQLite local-only boundary
- Postgres starter as serious shared/deployable path
- Distributed Proof as the named clustered verifier map
- Production Backend Proof as public-secondary only

Natural task seam: rewrite the handoff text without disturbing the starter ladder.

### 3. Mutation tests and proof-surface verifiers that pin the old handoff

These will fail if docs/generator/skill text changes without updating them in the same task:
- `scripts/tests/verify-m049-s04-onboarding-contract.test.mjs`
- `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs`
- `scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs`
- `scripts/tests/verify-m053-s04-contract.test.mjs`
- `scripts/tests/verify-m048-s04-skill-contract.test.mjs`
- `scripts/verify-production-proof-surface.sh`
- `scripts/verify-m050-s03.sh`
- `scripts/verify-m051-s04.sh`
- `scripts/verify-m047-s06.sh`

Important wrinkle: `scripts/verify-m047-s06.sh` still expects old public docs markers including the old backend runbook URL shape. If S03 changes Distributed Proof / Production Backend Proof wording, this historical wrapper needs updating too or it becomes false-red noise.

Natural task seam: source changes and exact-marker rails must ship together.

### 4. Repo identity and hardcoded public URLs

- `scripts/lib/repo-identity.json`
  - canonical language vs product repo identity contract from S01
- `scripts/lib/m034_public_surface_contract.py`
  - already consumes repo identity for language-owned installer/docs URLs
- most Node mutation tests still hardcode:
  - `https://github.com/snowdamiz/mesh-lang/blob/main/...`
  - `mesher/README.md`
  - `bash scripts/verify-m051-s01.sh`

Natural seam: if S03 introduces a new product handoff identity, centralize it once instead of repeating hardcoded product markers across every Node test.

### 5. Hosted/public contract and workflow assembly

Current coupling:
- `.github/workflows/deploy-services.yml`
  - deploys registry, packages website, and hyperpush landing together
  - health-check job explicitly verifies both `m034_public_surface_contract.py public-http` and the landing/blog routes
- `scripts/verify-m034-s05-workflows.sh`
  - requires `deploy-hyperpush-landing` and `Verify hyperpush landing`
- `scripts/tests/verify-m034-s05-contract.test.mjs`
  - also hard-requires landing deployment inside the language repo’s public contract
- `scripts/verify-m034-s05.sh`
  - assembled public release proof still assumes the monorepo workflow graph
- `scripts/verify-m053-s03.sh`
  - hosted starter/packages/public-surface proof still depends on that same workflow graph

Natural task seam: workflow and hosted-proof split is its own unit, separate from doc wording.

## Build Order

1. **Decide the new product handoff marker shape.**
   This is the most important planning decision. S03 should not keep shipping local `mesher/...` source paths in the language repo’s public contract, but it also should not invent a fake product URL. Decide the stable handoff marker first.

2. **Update generated and public language-owned text surfaces.**
   Start with:
   - `compiler/mesh-pkg/src/scaffold.rs`
   - `README.md`
   - `website/docs/docs/production-backend-proof/index.md`
   - `website/docs/docs/distributed-proof/index.md`
   - `website/docs/docs/distributed/index.md`
   - `tools/skill/mesh/skills/clustering/SKILL.md`

3. **Update exact-marker tests and proof-surface verifiers in the same change wave.**
   Especially:
   - `scripts/tests/verify-m049-s04-onboarding-contract.test.mjs`
   - `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs`
   - `scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs`
   - `scripts/tests/verify-m053-s04-contract.test.mjs`
   - `scripts/tests/verify-m048-s04-skill-contract.test.mjs`
   - `scripts/verify-production-proof-surface.sh`
   - `scripts/verify-m047-s06.sh`

4. **Then split the language-owned workflow/public-surface proof from landing.**
   Touch together:
   - `.github/workflows/deploy-services.yml`
   - `scripts/verify-m034-s05-workflows.sh`
   - `scripts/tests/verify-m034-s05-contract.test.mjs`
   - `scripts/verify-m034-s05.sh`
   - `scripts/verify-m053-s03.sh`
   - `scripts/tests/verify-m053-s03-contract.test.mjs`

5. **Finish with a slice-owned assembled verifier** (likely a new `scripts/verify-m055-s03.sh`) that proves the language-owned public/starter contract without requiring `mesher/landing` or product-repo source paths.

## Verification Approach

### Existing direct rails to reuse

Examples/starter parity:
- `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`
- `cargo test -p meshc --test e2e_m049_s03 -- --nocapture`

Public source-contract rails likely to need updates but still remain authoritative:
- `bash scripts/verify-production-proof-surface.sh`
- `node --test scripts/tests/verify-m049-s04-onboarding-contract.test.mjs`
- `node --test scripts/tests/verify-m050-s02-first-contact-contract.test.mjs`
- `node --test scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs`
- `node --test scripts/tests/verify-m053-s04-contract.test.mjs`
- `node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs`

Build surfaces:
- `npm --prefix website run build`
- `npm --prefix packages-website run build`
- `python3 scripts/lib/m034_public_surface_contract.py local-docs --root .`
- if docs-build markers change materially: `python3 scripts/lib/m034_public_surface_contract.py built-docs --root . --dist-root website/docs/.vitepress/dist`

Hosted/workflow contract rails if the packages/public-surface proof changes:
- `node --test scripts/tests/verify-m034-s05-contract.test.mjs`
- `bash scripts/verify-m034-s05-workflows.sh`
- `node --test scripts/tests/verify-m053-s03-contract.test.mjs`

### Likely final assembled S03 rail

One new narrow wrapper should probably replay:
- the slice-owned source contract tests
- example parity (`m049-s03` seams)
- website build
- packages build
- language-owned workflow/public-surface contract rails

and publish the usual:
- `status.txt`
- `current-phase.txt`
- `phase-report.txt`
- `full-contract.log`
- `latest-proof-bundle.txt`

under `.tmp/m055-s03/verify/`.

## Constraints

- `WORKSPACE.md` explicitly says M055 is a **two-repo split only**. `mesh-packages/` and `mesh-website/` are *not* sibling repos in this milestone. S03 should preserve future extractability, but it should not rewrite the workspace contract.
- Repo-local `.gsd` and `.tmp/` bundles remain authoritative. Do not solve S03 by inventing an umbrella workspace verifier.
- Per the VitePress skill, the relevant docs seams are `.vitepress/config` plus `public/`; here that means `website/docs/.vitepress/config.mts` and `website/docs/public/install.{sh,ps1}`. Those public installer files are served as-is and already checked byte-for-byte against `tools/install/install.{sh,ps1}` by `scripts/lib/m034_public_surface_contract.py`.
- Per the SvelteKit skill, the packages site is a normal file-routed app under `src/routes/`; here `packages-website/package.json` only exposes `vite build`, so S03 should treat `npm --prefix packages-website run build` as the direct proof seam rather than inventing a custom package-site verifier first.
- The current git remote is not the same thing as the public language identity contract. `scripts/lib/repo-identity.json` is the public source of truth for language/product URLs; do not derive public language repo URLs from `origin` just because some hosted-evidence scripts derive the actual workflow repo slug that way.

## Common Pitfalls

- **Changing docs without changing the generator.** `compiler/mesh-pkg/src/scaffold.rs` is a public-surface file, not an implementation detail.
- **Changing docs/generator without changing the exact-marker rails.** The repo has many mutation tests and historical wrappers that pin current wording.
- **Splitting the deploy workflow without updating both verifier layers.** `deploy-services.yml`, `scripts/verify-m034-s05-workflows.sh`, and `scripts/tests/verify-m034-s05-contract.test.mjs` move together.
- **Accidentally broadening the milestone back to four repos.** S01 explicitly froze M055 to two repos.
- **Keeping product-local `mesher/...` paths in the mesh-lang public contract.** That is the core drift S03 is supposed to remove.

## Don’t Hand-Roll

- Reuse `scripts/tests/verify-m049-s03-materialize-examples.mjs` and `compiler/meshc/tests/e2e_m049_s03.rs` for starter/example parity.
- Reuse `scripts/lib/m034_public_surface_contract.py` for installer/docs/public-surface checks instead of adding new ad hoc curl/grep scripts.
- Reuse the S02 Mesher package-root contract (`mesher/scripts/*` + `mesher/README.md`) as the product-owned boundary; do not invent a second deep-app verification shape inside `mesh-lang`.

## Open Risks

- The stable public/maintainer handoff shape for the product repo is not fully decided yet. `mesher/README.md` is the right contract **inside this monorepo**, but it is probably the wrong long-term public path once `hyperpush-mono` exists as its own repo root.
- Historical docs rails from M047/M050/M051 will create false-red noise if they are not updated in the same change wave.
- The hosted/public proof split may touch both workflow YAML and the remote-evidence assembly code; that is the highest-risk part of the slice after the wording handoff.

## Skills Discovered

| Technology | Skill | Status |
|---|---|---|
| VitePress docs site | `vitepress` | available and used for config/public-dir guidance |
| SvelteKit packages site | `sveltekit` | installed globally during research; read directly from `~/.agents/skills/sveltekit/SKILL.md` |
