# S03: Public contract and guarded claims

**Goal:** Align the public VitePress contract with the serious starter’s proven one-public-URL/runtime-owned correlation story, and add guardrails that fail closed when homepage, proof docs, metadata, or assembled verifier copy drifts.
**Demo:** After this: After this: homepage, distributed-proof docs, and serious starter guidance all describe the same bounded load-balancing model, and contract tests fail if copy overclaims.

## Tasks
- [x] **T01: Aligned the public docs and OG copy to the one-public-URL runtime-owned placement story, and added S03 drift guards.** — Close the actual public wording gap on the VitePress side. The serious starter README/template already says the right bounded thing, but homepage metadata, Distributed Proof, and the OG generator still overstate generic load balancing or omit the direct request-correlation follow-through. Reuse the starter’s vocabulary instead of inventing a second public story.

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
  - Estimate: 45m
  - Files: website/docs/index.md, website/docs/.vitepress/config.mts, website/docs/docs/distributed-proof/index.md, website/scripts/generate-og-image.py, website/docs/public/og-image-v2.png
  - Verify: - `npm --prefix website run generate:og`
- `npm --prefix website run build`
- [x] **T02: Hardened the S03 docs verifier to replay the Cargo contract, retain built-HTML evidence, and republish its own proof bundle after S02.** — Turn the wording into an enforceable surface. The slice only closes if public docs drift is caught cheaply in source, then replayed through one assembled verifier that reuses S02’s proof bundle instead of re-implementing the runtime story.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| delegated `bash scripts/verify-m054-s02.sh` replay | Stop immediately and preserve the copied S02 verify tree; do not claim green docs truth when the underlying starter proof is red. | Treat a hung delegated replay as a real acceptance blocker and surface the failing S02 phase in the S03 bundle. | Fail closed if the delegated verify directory is missing status/pointer markers or if the copied S02 bundle shape drifts. |
| source-contract / built-HTML assertions in the new S03 rails | Reject the change instead of weakening older M050/M054 guards; a missing or moved marker is drift, not a warning. | Bound the VitePress build and HTML inspection phases and keep their logs in the retained bundle. | Fail closed if built HTML still contains stale copy, the OG asset is missing, or the retained bundle leaks unredacted `DATABASE_URL`. |

## Load Profile

- **Shared resources**: `.tmp/m054-s02/verify`, the new `.tmp/m054-s03/verify` bundle, VitePress build output, and copied HTML/OG artifacts.
- **Per-operation cost**: one source-contract test, one Rust verifier-contract test, one delegated S02 replay, one OG regeneration, and one docs build.
- **10x breakpoint**: repeated full wrapper replays and bundle copies dominate before the contract checks themselves become expensive.

## Negative Tests

- **Malformed inputs**: stale homepage tagline markers, stale distributed-proof operator-flow markers, missing built HTML snapshots, missing OG asset, and malformed delegated bundle pointers.
- **Error paths**: S02 delegation missing, built HTML assertion drift, redaction leak, or wrapper phases passing without retaining the proof bundle.
- **Boundary conditions**: homepage and distributed-proof built HTML match the edited source, the S03 wrapper repoints `latest-proof-bundle.txt` to its own retained bundle, and the copied S02 verifier state stays intact.

## Steps

1. Add `scripts/tests/verify-m054-s03-contract.test.mjs` to guard the homepage/config/distributed-proof/OG-generator source markers and stale-marker exclusions without widening older M050 or M054 tests.
2. Add `compiler/meshc/tests/e2e_m054_s03.rs` to archive the same source files, assert the S03 wrapper layering/bundle contract, and keep the verifier behavior under Cargo like the older docs-closeout rails.
3. Add `scripts/verify-m054-s03.sh` so the assembled rail delegates `bash scripts/verify-m054-s02.sh`, runs the new source/Rust contract tests, regenerates the OG asset, builds VitePress, copies homepage/distributed-proof built HTML and the OG asset into a retained bundle, and fail-closes on bundle-shape or redaction drift.
4. Reuse the S02/starter wording oracle and copied verify trees instead of mutating `.tmp/m054-s02/verify` or inventing a second proof story.

## Must-Haves

- [ ] `scripts/tests/verify-m054-s03-contract.test.mjs` catches stale public-copy markers and missing bounded markers in the homepage, VitePress config, distributed-proof page, and OG generator.
- [ ] `compiler/meshc/tests/e2e_m054_s03.rs` pins the S03 verifier layering and retained-bundle contract in a repo-owned Cargo rail.
- [ ] `scripts/verify-m054-s03.sh` delegates S02 unchanged, reruns OG generation + VitePress build, retains built HTML/OG evidence, and republishes its own `latest-proof-bundle.txt`.
- [ ] The retained S03 bundle includes the copied S02 verify tree, built HTML snapshots, OG asset evidence, phase logs, and no unredacted `DATABASE_URL`.
  - Estimate: 1h
  - Files: scripts/tests/verify-m054-s03-contract.test.mjs, compiler/meshc/tests/e2e_m054_s03.rs, scripts/verify-m054-s03.sh
  - Verify: - `node --test scripts/tests/verify-m054-s03-contract.test.mjs`
- `cargo test -p meshc --test e2e_m054_s03 -- --nocapture`
- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash scripts/verify-m054-s03.sh`
