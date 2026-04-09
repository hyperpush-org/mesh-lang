# M054 / S03 Research — Public contract and guarded claims

## Requirement focus

Primary slice requirement:

- **R123** — public docs and proof surfaces need to describe the actual shipped one-public-URL story truthfully, with guarded claims instead of generic load-balancer language.

Supported boundaries from milestone context:

- **R060** — keep Fly as evidence, not the architecture story.
- **R124** — keep frontend-aware adapters deferred unless the current server-side/runtime model proved insufficient.

S01 and S02 already advanced the runtime proof. S03 is now a docs/contract slice, not a runtime-design slice.

## Skills Discovered

Relevant installed skills already present:

- `vitepress` — loaded for docs-site configuration and theme/sidebar conventions.
- `react-best-practices` and `frontend-design` — available if the slice expands into the Next.js landing surface.

Additional skill search run:

- `npx skills find "Next.js"`
  - strongest result: `wshobson/agents@nextjs-app-router-patterns`
  - **not installed**: this slice is currently copy/guardrail work, not new App Router implementation.

## Summary

This slice is **targeted**, not architectural. The real work is already done in S01/S02; the remaining gap is that the public docs/marketing surfaces still do not all say the same bounded thing.

The current public-state drift is concentrated in four places:

1. **Docs homepage claim overstates the product surface.**
   - `website/docs/index.md` still says: **"Built-in failover, load balancing, and exactly-once semantics — no orchestration layer required."**
   - `website/docs/.vitepress/config.mts` duplicates the same line as `DEFAULT_DESCRIPTION`, so the stale claim also leaks into site metadata/structured data defaults.

2. **The docs OG image generator still bakes in the same old claim.**
   - `website/scripts/generate-og-image.py` still renders: **"One annotation. Native speed. Auto-failover, load balancing, and exactly-once semantics."**
   - `npm --prefix website run build` does **not** regenerate that asset; changing the source without rerunning `generate:og` leaves the shipped social preview stale.

3. **`Distributed Proof` has not absorbed the S02 request-correlation model.**
   - `website/docs/docs/distributed-proof/index.md` still teaches the generic operator flow as:
     - status
     - continuity list
     - continuity record
     - diagnostics
   - and still says: **"Use the list form first to discover startup or request keys. Only then drill into a single continuity record."**
   - That is now stale for the serious starter’s clustered HTTP responses, because S02 proved the direct runtime-owned `X-Mesh-Continuity-Request-Key` response header seam.

4. **There is an optional extra public overclaim outside the docs site.**
   - `mesher/landing/app/mesh/page.tsx` still says: **"Failover, load balancing, clustering: all first-class."**
   - There is currently **no** `/mesh` route test or verifier. Including this surface makes S03 a two-toolchain slice (VitePress + Next.js) instead of a docs-only slice.

The good news: the serious starter guidance itself is already in good shape. The canonical bounded wording already exists in:

- `compiler/mesh-pkg/src/scaffold.rs`
- `examples/todo-postgres/README.md`
- `scripts/tests/verify-m054-s01-contract.test.mjs`
- `scripts/tests/verify-m054-s02-contract.test.mjs`

That means S03 should reuse those phrases, not invent a second vocabulary.

## Relevant files and what they do

### Canonical truth sources already landed in S01/S02

- `compiler/mesh-pkg/src/scaffold.rs`
  - generator-owned serious starter README template.
  - already contains the right bounded phrases:
    - one public app URL may front multiple nodes
    - proxy/platform ingress wording
    - `X-Mesh-Continuity-Request-Key`
    - operator/debug seam wording
    - explicit non-promises around frontend-aware routing, sticky sessions, and Fly-specific product contract.

- `examples/todo-postgres/README.md`
  - materialized committed starter README.
  - already matches the scaffold template and is guarded by S01/S02 contract tests.

- `scripts/tests/verify-m054-s01-contract.test.mjs`
- `scripts/tests/verify-m054-s02-contract.test.mjs`
  - best current source of exact required/stale marker strings.
  - planner should treat these as the wording oracle for bounded claims.

### Public docs surfaces S03 likely owns

- `website/docs/index.md`
  - docs-site homepage frontmatter only.
  - currently just a `description:` field; that field is the visible/SEO headline surface to tighten.

- `website/docs/.vitepress/config.mts`
  - site-level VitePress config.
  - `DEFAULT_DESCRIPTION` duplicates the stale homepage claim.
  - also feeds structured data and default site metadata, so it must stay in sync with `website/docs/index.md`.

- `website/scripts/generate-og-image.py`
  - generates `website/docs/public/og-image-v2.png`.
  - still uses the old load-balancing subtitle.
  - changing it requires an explicit regeneration step.

- `website/package.json`
  - contains `generate:og`.
  - useful for slice verification because docs build does not cover OG generation.

- `website/docs/docs/distributed-proof/index.md`
  - public-secondary proof map.
  - already names the correct M053 starter-owned verifier chain.
  - missing the ingress-vs-runtime boundary in plain language.
  - still teaches pre-S02 continuity-list-first request discovery.

### Existing docs/verifier patterns worth reusing

- `scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs`
  - node:test mutation-style contract guard for docs sources.
  - good pattern for a new M054 S03 source-level contract.

- `scripts/verify-m050-s03.sh`
  - built-HTML verifier pattern for VitePress pages.
  - copies built HTML snapshots and asserts visible text/links/order via Python HTML parsing.

- `compiler/meshc/tests/e2e_m050_s03.rs`
- `compiler/meshc/tests/e2e_m051_s04.rs`
  - Rust-side wrapper-contract tests for shell verifiers.
  - good pattern if S03 needs a repo-owned e2e contract for its assembled wrapper.

- `website/docs/.vitepress/theme/composables/usePrevNext.ts`
- `scripts/tests/verify-m050-s01-onboarding-graph.test.mjs`
  - footer/sidebar graph guard.
  - probably unchanged unless S03 moves pages around. Current research found no need to change the graph.

### Optional extra-scope public surface

- `mesher/landing/app/mesh/page.tsx`
- `mesher/landing/app/mesh/layout.tsx`
- `mesher/landing/package.json`
  - separate Next.js marketing surface.
  - current overclaim is in the page body, not layout metadata.
  - no current `/mesh` Playwright or contract test exists.

## What is already truthful and should probably stay untouched

These surfaces already match the bounded starter-first story and do **not** need broad rewrites unless the final copy wants a small handoff tweak:

- `examples/todo-postgres/README.md`
- `compiler/mesh-pkg/src/scaffold.rs`
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `README.md`

Notable nuance:

- `Clustered Example` still says to use the continuity list first — but that page is the **route-free scaffold** story, so that remains truthful there.
- `Distributed Proof` is different: it explicitly covers the serious starter and its public proof rails, so it is the page that must absorb the S02 response-header correlation seam.

## The exact public gaps

### 1. Homepage wording is stale in multiple sources

Current stale strings:

- `website/docs/index.md`
  - `Built-in failover, load balancing, and exactly-once semantics — no orchestration layer required.`
- `website/docs/.vitepress/config.mts`
  - same string in `DEFAULT_DESCRIPTION`
- `website/scripts/generate-og-image.py`
  - `One annotation. Native speed. Auto-failover, load balancing, and exactly-once semantics.`

This is not one file drift. It is a **homepage/metadata/social-preview cluster**.

### 2. `Distributed Proof` is missing the public boundary sentence

The page already names the correct verifier chain, but it does **not** currently say the S01/S02 story in plain language:

- one public app URL can front multiple nodes
- proxy/platform ingress chooses the ingress node
- Mesh runtime chooses owner/replica/execution after ingress
- clustered HTTP gives operators a direct `X-Mesh-Continuity-Request-Key`
- that header is an operator/debug seam, not client routing

That sentence cluster is what aligns the docs with the serious starter README.

### 3. The current operator flow text is stale for clustered HTTP

Current `Distributed Proof` wording says:

- use the list form first to discover startup or request keys
- only then drill into a single continuity record

That should stay true for:

- startup work
- general manual inspection
- route-free scaffold flows

But it is now incomplete for:

- serious starter clustered HTTP request correlation

The page needs a bifurcated rule, not a blanket list-first rule.

### 4. The landing `/mesh` page is a real drift seam if included

`mesher/landing/app/mesh/page.tsx` currently says:

- `Failover, load balancing, clustering: all first-class.`

That is stronger than the proven S01/S02 contract. But because there is no existing `/mesh` test, pulling it into S03 is a separate verification decision.

## Natural task seams

### Seam 1 — VitePress copy + metadata alignment

Smallest coherent unit:

- `website/docs/index.md`
- `website/docs/.vitepress/config.mts`
- `website/docs/docs/distributed-proof/index.md`
- `website/scripts/generate-og-image.py`
- regenerated `website/docs/public/og-image-v2.png` if the tagline changes

This is the core S03 work.

Notes for planner:

- Reuse the serious-starter vocabulary from S01/S02 rather than inventing new product language.
- Keep Fly explicitly secondary/evidence-only.
- Do not promise sticky sessions, frontend-aware routing, load-aware balancing, or client topology awareness.
- The proof page should explain the boundary, not become a new starter walkthrough.

### Seam 2 — Source-level contract guard for the new public wording

Add a dedicated M054 S03 source contract, probably as:

- `scripts/tests/verify-m054-s03-contract.test.mjs`

Recommended scope for that test:

- require bounded markers in:
  - `website/docs/index.md`
  - `website/docs/.vitepress/config.mts`
  - `website/docs/docs/distributed-proof/index.md`
  - `website/scripts/generate-og-image.py`
- fail closed on stale markers like:
  - `Built-in failover, load balancing, and exactly-once semantics`
  - `Auto-failover, load balancing, and exactly-once semantics`
- require the new `Distributed Proof` operator-flow caveat that distinguishes:
  - clustered HTTP response-header lookup
  - startup/manual continuity-list discovery

Reason to keep this separate from older tests:

- `m050_*` and `m053_*` tests guard earlier onboarding/proof-page concerns.
- S03 needs its own bounded contract instead of widening old tests into a second responsibility.

### Seam 3 — Assembled verifier + retained built artifacts

Recommended pattern:

- `scripts/verify-m054-s03.sh`
- probably paired with `compiler/meshc/tests/e2e_m054_s03.rs`

Recommended wrapper behavior:

1. delegate `bash scripts/verify-m054-s02.sh`
2. run `node --test scripts/tests/verify-m054-s03-contract.test.mjs`
3. run `npm --prefix website run generate:og` **if** the OG copy changed
4. run `npm --prefix website run build`
5. retain built HTML snapshots for:
   - `website/docs/.vitepress/dist/index.html` (**new; current wrappers do not copy homepage HTML**)
   - `website/docs/.vitepress/dist/docs/distributed-proof/index.html`
   - optionally `website/docs/.vitepress/dist/docs/getting-started/clustered-example/index.html` if changed
6. assert built HTML text/order plus bundle-shape markers
7. copy the delegated S02 verify tree instead of mutating `.tmp/m054-s02/verify`

Important detail:

- If the wrapper delegates S02, it will still need `DATABASE_URL` because S02 replays the starter runtime rail.

### Seam 4 — Optional landing-page cleanup (separate unless explicitly wanted)

If the slice must cover **all** public Mesh marketing copy, this becomes a second unit:

- `mesher/landing/app/mesh/page.tsx`
- maybe a new file-level contract or a small Playwright route shell for `/mesh`
- `npm --prefix mesher/landing run build`
- optionally `./mesher/landing/node_modules/.bin/tsc --noEmit -p mesher/landing/tsconfig.json`

I would keep this separate from the VitePress docs wrapper unless the user explicitly wants the landing page included in S03’s definition of “homepage.”

## Verification strategy

### Cheap source-level proof

- `node --test scripts/tests/verify-m054-s03-contract.test.mjs`

### VitePress/public-asset proof

- `npm --prefix website run build`
- `npm --prefix website run generate:og` if the OG subtitle changes

Because `generate:og` is separate from `build`, the wrapper should treat OG regeneration as its own phase or explicitly check asset sync after running it.

### Assembled slice rail (recommended)

If S03 follows the existing dependency-stack pattern:

- `DATABASE_URL=<local disposable postgres> bash scripts/verify-m054-s03.sh`

If S03 intentionally stays docs-only and does **not** replay S02, then the wrapper can be env-free — but that would break the usual later-slice replay pattern already used in this repo.

### Optional landing scope

Only if the landing page is included:

- `npm --prefix mesher/landing run build`
- `./mesher/landing/node_modules/.bin/tsc --noEmit -p mesher/landing/tsconfig.json`
- targeted `/mesh` contract or Playwright rail

## Risks and constraints

- **Do not rely on docs build to cover OG image drift.** The generator is separate.
- **Do not silently widen M050/M053 tests.** Their current job is onboarding/proof-page graph truth, not M054 load-balancing wording.
- **Do not re-open starter README/template copy unless needed.** That wording is already the S01/S02 oracle.
- **Do not let `Distributed Proof` regress into a Fly-first or fixture-first page.** It already has strong older guardrails around first-contact ordering.
- **If landing is included, treat it as explicit extra scope.** There is no existing `/mesh` verifier to piggyback on.

## Recommendation

Recommended core S03 scope:

1. **Keep the slice on VitePress docs + metadata + OG asset sync.**
2. **Use the serious starter README/template wording as the canonical phrase set.**
3. **Add one dedicated M054 S03 source contract and one assembled verifier.**
4. **Treat `mesher/landing/app/mesh/page.tsx` as optional follow-on scope unless the user explicitly wants all public Mesh marketing copy cleaned up now.**

That gives the planner a clean decomposition:

- bounded copy alignment on the docs site
- bounded new contract test
- bounded assembled wrapper reusing the existing M054 proof chain

without turning a low-risk docs slice into a mixed VitePress/Next.js marketing sweep.