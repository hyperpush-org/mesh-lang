# M053 / S04 Research — Public docs and Fly reference assets match the shipped contract

**Date:** 2026-04-05  
**Status:** Ready for planning

## Summary

S04 is mostly a docs-contract slice, not a new runtime or workflow slice.

The generated starter surfaces already hold the important M053 boundary correctly:

- SQLite stays explicitly local-only.
- The generated Postgres starter owns a portable staged deploy bundle contract.
- The generated Postgres starter README intentionally does **not** make Fly part of the starter contract.

The real drift is in the public proof-map docs:

- first-contact docs still describe the starter split correctly, but they do **not** surface the new M053 staged-deploy + failover + hosted-chain truth
- `website/docs/docs/distributed-proof/index.md` still centers older M047/M043 clustered rails and the old read-only Fly verifier instead of the new generated-starter-first M053 proof chain
- public docs do not currently explain that packages-site public-surface health and hosted starter failover proof now live in the same hosted release/deploy contract

If S04 is expected to satisfy **R120 literally across landing/docs/packages**, there is also a larger scope risk: `mesher/landing/` is currently a Hyperpush marketing site, not a Mesh landing surface, and existing deploy verification only health-checks that Hyperpush page. That is a separate seam from the docs proof-map cleanup.

## Requirements Focus

Primary requirements this slice supports or closes:

- **R115** — keep the dual-db starter split honest
- **R116** — generated examples stay primary; do not re-promote retained proof apps
- **R117** — public docs stay evaluator-facing, not proof-maze-first
- **R120** — landing/docs/packages need one coherent public story
- **R121** — packages site is part of the normal public CI/deploy contract
- **R122** — Postgres clustered deploy proof is real, starter-owned, and SQLite remains explicitly local

What S04 needs to make visible:

- SQLite is still local/single-node only
- Postgres starter is the serious deployable path via the staged bundle contract
- Fly is a **reference/proof environment**, not the public starter contract
- packages/public-surface checks are part of the same hosted contract as starter proof, not a side workflow

## Skills Discovered

Relevant installed skills already present:

- **flyio-cli-public**
  - Relevant rule: prefer **read-only** Fly actions first.
  - Relevant rule: do **not** use state-changing Fly actions without explicit approval.
  - Planning implication: S04 should keep Fly described as a bounded reference/read-only proof surface unless the slice explicitly adds a new approved operational contract.
- **vitepress**
  - Relevant rule: check `.vitepress/config.*` before changing docs structure.
  - Relevant rule: treat `public/` as static assets served as-is.
  - Planning implication: docs changes should stay grounded in `website/docs/.vitepress/config.mts` and verify with a real VitePress build.

No additional skill installs are needed for the likely docs-first implementation path.

## Implementation Landscape

### 1. Generator-owned starter surfaces already encode the right contract

**Files:**

- `compiler/mesh-pkg/src/scaffold.rs`
- `examples/todo-postgres/README.md`
- `examples/todo-sqlite/README.md`
- `scripts/tests/verify-m049-s03-materialize-examples.mjs`

What matters:

- The source of truth for generated starter README content is `compiler/mesh-pkg/src/scaffold.rs`.
- `examples/todo-postgres/README.md` and `examples/todo-sqlite/README.md` are generator-owned materializations, not independent docs.
- `scripts/tests/verify-m049-s03-materialize-examples.mjs` exists specifically to keep `/examples` identical to fresh scaffold output.

Current Postgres starter contract is already correct for M053:

- staged deploy bundle is named as the public deploy contract
- `DATABASE_URL` is required
- reads use bounded `HTTP.clustered(1, ...)`
- writes and `/health` stay local
- runtime inspection stays on `meshc cluster status|continuity|diagnostics`
- hosted providers are allowed behind the staged bundle, but are not the required starter surface

Current SQLite starter contract is already correct for M053:

- single-node SQLite only
- no `work.mpl`
- no `HTTP.clustered(...)`
- no `meshc cluster` story

Critical constraint for planners:

- `compiler/mesh-pkg/src/scaffold.rs` tests explicitly reject Fly wording in the generated Postgres starter README.
- That means S04 should **not** solve Fly reference wording by inserting Fly into generator-owned starter surfaces unless the starter contract is intentionally widened and all corresponding assertions are updated.

### 2. First-contact docs already protect the starter ladder, but they are stale on M053 proof ownership

**Files:**

- `README.md`
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/tooling/index.md`
- `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs`
- `scripts/verify-m050-s02.sh`

What exists already:

- first-contact docs strongly enforce the scaffold → SQLite → Postgres → deeper proof ordering
- first-contact docs already avoid re-promoting repo fixtures as first-contact surfaces
- tooling docs already mention packages.meshlang.dev and the public-surface verifier chain

What is missing:

- no first-contact doc currently explains that the serious Postgres starter now has an M053 staged deploy + clustered failover proof chain
- no first-contact doc currently explains that hosted verification now couples starter failover proof and packages public-surface proof
- no first-contact doc currently distinguishes “portable staged bundle contract” from “Fly reference proof environment” in M053 language

Planning implication:

- these files are the right place for small evaluator-facing wording updates
- they are **not** the right place to dump raw verifier command ledgers or deep proof bundles
- any edits here should preserve the existing examples-first ladder that `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` already guards

### 3. `Distributed Proof` is the natural seam for the M053 proof-map update

**Files:**

- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/production-backend-proof/index.md`
- `scripts/verify-production-proof-surface.sh`
- `scripts/verify-m043-s04-proof-surface.sh`

Current state:

- `Distributed Proof` is the only public-secondary page that carries named clustered proof rails
- it still points at older M047/M043 proof ownership:
  - `bash scripts/verify-m047-s04.sh`
  - `bash scripts/verify-m047-s05.sh`
  - `cargo test -p meshc --test e2e_m047_s07 -- --nocapture`
  - `bash scripts/verify-m043-s04-fly.sh`
- it still frames Fly through the older `cluster-proof` rail as a read-only proof path

What is stale relative to M053:

- no mention of `scripts/verify-m053-s01.sh`
- no mention of `scripts/verify-m053-s02.sh`
- no mention of `scripts/verify-m053-s03.sh`
- no mention of the reusable hosted starter failover proof workflow
- no mention that packages public-surface proof now fails the same hosted contract as starter proof
- no public-secondary explanation that the generated Postgres starter owns the portable contract while Fly stays secondary/reference

Planning implication:

- if S04 needs a single public proof-map handoff for M053, `website/docs/docs/distributed-proof/index.md` is the best place to carry it
- `website/docs/docs/distributed/index.md` likely only needs a lighter routing update
- `website/docs/docs/production-backend-proof/index.md` is already compact and maintainer-facing; it probably should stay narrow unless S04 truly needs a backend-proof cross-link tweak

### 4. Hosted packages/starter contract is already implemented in code, but not surfaced clearly in docs

**Files:**

- `.github/workflows/authoritative-starter-failover-proof.yml`
- `.github/workflows/authoritative-verification.yml`
- `.github/workflows/release.yml`
- `.github/workflows/deploy-services.yml`
- `scripts/verify-m053-s03.sh`
- `scripts/tests/verify-m053-s03-contract.test.mjs`
- `scripts/lib/m034_public_surface_contract.py`

What exists already:

- S03 created a reusable hosted starter failover proof lane
- `authoritative-verification.yml` and `release.yml` both require that lane
- `deploy-services.yml` still owns the packages website deploy + public surface contract step
- `scripts/verify-m053-s03.sh` already proves that hosted state must fail closed when either the starter proof lane or packages/public-surface evidence drifts

What public docs do not currently say:

- that this is now one hosted public contract
- that packages-site public-surface health is not a side check anymore
- that the serious starter proof is part of the same hosted release/deploy evidence chain

Planning implication:

- docs updates should reuse the exact workflow/verifier names above
- avoid inventing new operational narratives; S03 already defined the hosted contract shape

### 5. Fly reference assets already exist, but they belong to the old clustered proof rail, not the generated Postgres starter

**Files:**

- `scripts/verify-m043-s04-fly.sh`
- `scripts/fixtures/clustered/cluster-proof/fly.toml`
- `scripts/fixtures/clustered/cluster-proof/README.md`
- `packages-website/fly.toml`

Important boundaries:

- `scripts/verify-m043-s04-fly.sh` is explicitly read-only and explicitly not destructive failover proof
- `scripts/fixtures/clustered/cluster-proof/fly.toml` is a fixture/reference Fly asset for the older `cluster-proof` packaged rail
- `packages-website/fly.toml` is a separate app deploy asset; it is not a starter reference asset
- the generated Postgres starter itself has **no** Fly asset and no Fly README wording

Planning implication:

- the least risky S04 path is to keep Fly described in public docs as a bounded reference/proof environment, not to inject Fly into starter-owned contract files
- if S04 needs to expose Fly reference assets, the docs should point at secondary/reference surfaces, not rewrite the generated starter contract

### 6. Landing/docs/packages coherence is the largest unresolved scope question

**Files showing current landing identity:**

- `mesher/landing/app/layout.tsx`
- `mesher/landing/lib/external-links.ts`
- `mesher/landing/components/landing/hero.tsx`
- `mesher/landing/components/landing/cta.tsx`
- `mesher/landing/components/landing/header.tsx`
- `mesher/landing/components/landing/footer.tsx`
- `.github/workflows/deploy-services.yml`

What is true today:

- the deployed landing job is literally named `Deploy hyperpush landing`
- the workflow health-check greps for `hyperpush` and `Open Source Error Tracking`
- `mesher/landing/` metadata, branding, links, and copy all target Hyperpush / hyperpush.dev / hyperpush-org
- this is not just one stale sentence; it is the whole landing app identity

This matters because R120 says landing/docs/packages should tell one coherent public story.

Planning implication:

- if S04 is expected to satisfy R120 across **all** public surfaces, landing is a separate implementation seam and the biggest scope/risk in the slice
- if S04 is intended as docs + Fly reference cleanup only, planners should explicitly keep landing out of scope and note that R120 will remain only partially advanced
- there is currently **no** landing-content contract verifier comparable to the first-contact or proof-surface docs verifiers

## Natural Seams for Task Planning

### Seam A — public docs proof-map refresh

Likely files:

- `README.md`
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- possibly `website/docs/docs/production-backend-proof/index.md`

Goal:

- make M053 visible without turning first-contact pages into a verifier maze
- keep SQLite local-only and Postgres serious/deployable
- explain that Fly is reference proof, not the starter contract
- explain that packages public-surface proof and starter proof now share the hosted contract

### Seam B — generator-owned starter copy, only if absolutely necessary

Likely files:

- `compiler/mesh-pkg/src/scaffold.rs`
- `examples/todo-postgres/README.md` via materialization
- `examples/todo-sqlite/README.md` via materialization if wording changes ripple

Goal:

- only update if the public starter README truly needs one more portability/contract sentence
- do **not** casually inject Fly into generated starter copy; current scaffold tests forbid it

### Seam C — S04-specific contract automation

There is no current M053/S04 verifier.

Likely new files:

- `scripts/verify-m053-s04.sh`
- `scripts/tests/verify-m053-s04-contract.test.mjs`

Good reuse patterns:

- `scripts/verify-m050-s02.sh` for built-docs + phase/bundle structure
- `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` for string/order contract tests
- `scripts/verify-production-proof-surface.sh` for proof-page routing assertions
- `scripts/tests/verify-m053-s03-contract.test.mjs` for hosted-contract vocabulary and fail-closed pattern

### Seam D — optional landing coherence work

Only needed if S04 truly owns R120 end-to-end.

Likely files:

- `mesher/landing/app/layout.tsx`
- `mesher/landing/lib/external-links.ts`
- `mesher/landing/components/landing/header.tsx`
- `mesher/landing/components/landing/hero.tsx`
- `mesher/landing/components/landing/cta.tsx`
- `mesher/landing/components/landing/footer.tsx`
- maybe `.github/workflows/deploy-services.yml` and its contract tests if health-check strings change

This is materially larger than the docs-only seam.

## What to Build or Prove First

1. **Decide scope on landing.**  
   If landing is in scope, it should be a dedicated task with separate verification. If not, keep it explicitly out of the docs-only task list.

2. **Update `Distributed Proof` first.**  
   That is the main stale proof-map surface and the cleanest place to thread M053 starter proof + hosted packages/starter contract + Fly reference boundary together.

3. **Then update first-contact docs lightly.**  
   Preserve the current scaffold/examples-first ordering while adding only the minimal M053-facing wording needed.

4. **Touch generator-owned starter copy last, only if needed.**  
   That path is higher-friction because it flows through `scaffold.rs` and example materialization.

5. **Add S04 verifier last.**  
   Existing wording and routing should stabilize first, then the verifier should pin it.

## Verification

Current baseline checks I ran:

- `npm --prefix website run build` ✅
- `npm --prefix packages-website run build` ✅
- `npm --prefix mesher/landing run build` ✅

Recommended final verification for S04:

- `npm --prefix website run build`
- `bash scripts/verify-m050-s02.sh`
- `bash scripts/verify-production-proof-surface.sh`
- `node --test scripts/tests/verify-m053-s04-contract.test.mjs` *(new)*
- `bash scripts/verify-m053-s04.sh` *(new)*

If generator-owned example copy changes:

- `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`
- or regenerate examples through the established materialization path used by the repo

If landing work is included:

- `npm --prefix mesher/landing run build`
- update/add a landing-content contract check; existing workflow grep checks are too weak for R120

## Risks and Watchouts

- **Biggest trap:** putting Fly wording into generated starter surfaces. Current scaffold tests intentionally reject that.
- **Second trap:** updating docs pages without adding/adjusting a contract verifier; drift will come back.
- **Third trap:** treating the old `cluster-proof` Fly rail as if it were the new generated Postgres starter proof. It is not.
- **Fourth trap:** ignoring the landing mismatch if planners claim full R120 closure.
- **Fifth trap:** hand-editing `examples/todo-postgres/README.md` directly without matching `compiler/mesh-pkg/src/scaffold.rs` and example materialization.

## Planner Recommendation

Plan this slice as **two clear tracks**:

1. **Required track:** public docs proof-map alignment for M053 (docs + verifier).  
   This is the smallest honest path to satisfy the slice title.

2. **Optional / scope-gated track:** landing coherence remediation if the milestone truly needs R120 closed across landing/docs/packages.  
   Treat this as a separate task because it is a product-surface rewrite, not a wording tweak.
