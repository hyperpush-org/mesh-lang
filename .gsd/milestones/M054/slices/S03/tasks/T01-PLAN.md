---
estimated_steps: 4
estimated_files: 5
skills_used:
  - vitepress
---

# T01: Align homepage, Distributed Proof, and OG copy to the bounded one-public-URL contract

**Slice:** S03 — Public contract and guarded claims
**Milestone:** M054

## Description

Close the actual public wording gap on the VitePress side. The serious starter README/template already says the right bounded thing, but homepage metadata, Distributed Proof, and the OG generator still overstate generic load balancing or omit the direct request-correlation follow-through. Reuse the starter’s vocabulary instead of inventing a second public story.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `npm --prefix website run build` VitePress build | Stop on the first site-config or markdown failure and keep the edited source files local; do not ship wording that only exists in unbuilt markdown. | Treat a hung build as toolchain drift and stop before changing verifier expectations. | Fail closed if the built page omits the new boundary text or still renders stale homepage copy. |
| `npm --prefix website run generate:og` / `website/scripts/generate-og-image.py` | Keep the source generator truth and stop; do not claim the social-preview surface is updated until the asset regenerates. | Treat a hung image render as an asset-pipeline regression and stop before updating bundle-shape assertions. | Fail closed if the generated subtitle or output asset drifts from the bounded public wording. |

## Load Profile

- **Shared resources**: VitePress build output under `website/docs/.vitepress/dist` and the generated `website/docs/public/og-image-v2.png` asset.
- **Per-operation cost**: one static image render plus one docs-site build.
- **10x breakpoint**: build time and asset regeneration dominate long before copy size matters.

## Negative Tests

- **Malformed inputs**: stale homepage tagline text, stale OG subtitle text, and missing direct-header/request-key wording in the proof page.
- **Error paths**: docs build or OG generation fails after source edits, or the built proof page still teaches continuity-list diffing as the only clustered HTTP flow.
- **Boundary conditions**: homepage stays broad but bounded, Distributed Proof preserves startup/manual continuity-list discovery, and Fly remains secondary evidence rather than the architecture story.

## Steps

1. Rewrite `website/docs/index.md` and `website/docs/.vitepress/config.mts` so the homepage/frontmatter/default description adopt the serious starter’s bounded one-public-URL/server-side-first language instead of the generic “Built-in failover, load balancing, and exactly-once semantics” claim.
2. Update `website/docs/docs/distributed-proof/index.md` to explain where proxy/platform ingress ends, where Mesh runtime placement begins, and when operators should use `X-Mesh-Continuity-Request-Key` direct lookup versus continuity-list discovery.
3. Update `website/scripts/generate-og-image.py` to render the same bounded story and regenerate `website/docs/public/og-image-v2.png`.
4. Keep scope on the VitePress/OG surfaces only: do not broaden the slice into `mesher/landing/` or rewrite the already-truthful starter README/template copy.

## Must-Haves

- [ ] Homepage frontmatter and default site metadata share one bounded description string.
- [ ] Distributed Proof names the response-header -> direct continuity lookup flow for clustered HTTP and keeps list-first discovery for startup/manual inspection.
- [ ] OG generator source and rendered asset reflect the same bounded contract.
- [ ] Fly stays evidence-only and the copy does not promise sticky sessions, frontend-aware routing, or client-visible topology.

## Verification

- `npm --prefix website run generate:og`
- `npm --prefix website run build`

## Inputs

- `website/docs/index.md` — current homepage frontmatter still carries the stale generic load-balancing promise.
- `website/docs/.vitepress/config.mts` — default description and structured-data sink that must stay aligned with the homepage copy.
- `website/docs/docs/distributed-proof/index.md` — public clustered-proof page that still needs the S02 request-correlation follow-through and bounded operator wording.
- `website/scripts/generate-og-image.py` — OG subtitle generator that still bakes in the stale generic tagline.
- `compiler/mesh-pkg/src/scaffold.rs` — canonical bounded starter wording oracle to reuse.
- `examples/todo-postgres/README.md` — materialized serious-starter wording oracle to mirror.

## Expected Output

- `website/docs/index.md` — homepage frontmatter aligned with the bounded one-public-URL story.
- `website/docs/.vitepress/config.mts` — site-wide default description and metadata aligned with the same bounded copy.
- `website/docs/docs/distributed-proof/index.md` — public proof page updated with ingress-vs-runtime boundary and direct request-key lookup guidance.
- `website/scripts/generate-og-image.py` — OG subtitle generator updated to the same bounded contract.
- `website/docs/public/og-image-v2.png` — regenerated social-preview asset produced from the updated generator.
