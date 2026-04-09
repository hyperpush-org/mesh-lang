---
estimated_steps: 4
estimated_files: 5
skills_used:
  - vitepress
---

# T04: Publish the distributed proof page and guard docs truth

**Slice:** S04 — One-Image Operator Path, Local/Fly Verifiers, and Docs Truth
**Milestone:** M039

## Description

Reconcile the public docs surface so distributed/operator claims point at concrete commands and proof artifacts instead of leaving readers in the generic `Node.start` / `Node.connect` tutorial path.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| VitePress docs build | fail the task with the build log and stop before claiming docs truth | fail with the bounded build log rather than skipping the site build | reject broken links/frontmatter/sidebar config rather than relying on unchecked markdown |
| docs-truth verifier script | fail closed on missing commands, links, or stale phrases across the runbook/README/proof page/sidebar | N/A — static content | reject partial matches that do not prove the same canonical commands are shared |

## Load Profile

- **Shared resources**: one VitePress build and one repo-local grep/contract verifier.
- **Per-operation cost**: static markdown/config updates plus one site build.
- **10x breakpoint**: docs drift and stale-command sprawl before compute cost; the verifier should centralize the canonical command list.

## Negative Tests

- **Malformed inputs**: missing proof-page frontmatter, missing sidebar link, or missing runbook/proof links must fail the verifier.
- **Error paths**: stale generic distributed claims, stale README links, or docs that mention commands not present in the real runbook/scripts must fail closed.
- **Boundary conditions**: the generic distributed guide still teaches primitives, but all operator-proof claims route to the new distributed-proof page and `cluster-proof/README.md`.

## Steps

1. Add `website/docs/docs/distributed-proof/index.md` mirroring the production-backend-proof pattern, with the exact local verifier, Fly verifier, runbook, and contract summary for S04.
2. Update `website/docs/docs/distributed/index.md`, `website/docs/.vitepress/config.mts`, and `README.md` so generic distributed guidance routes operator claims to the proof page and runbook instead of implying the tutorial path is the proof surface.
3. Add `scripts/verify-m039-s04-proof-surface.sh` to assert the README, proof page, sidebar, generic distributed page, and `cluster-proof/README.md` share the same canonical commands and links while rejecting stale operator wording.
4. Finish only when the docs build passes and the docs-truth verifier passes from repo root.

## Must-Haves

- [ ] The public distributed proof page points at the exact S04 local/Fly verifier commands and the `cluster-proof/README.md` runbook.
- [ ] `README.md`, the sidebar, and the generic distributed guide route readers to the proof surface instead of leaving operator claims in the tutorial page.
- [ ] `scripts/verify-m039-s04-proof-surface.sh` fail-closes on docs drift and becomes the named verifier for R053.

## Verification

- `npm --prefix website run build`
- `bash scripts/verify-m039-s04-proof-surface.sh`

## Inputs

- `cluster-proof/README.md` — deepest operator runbook produced in T03.
- `website/docs/docs/distributed/index.md` — current generic distributed guide that needs rerouting.
- `website/docs/docs/production-backend-proof/index.md` — existing proof-page pattern to mirror.
- `website/docs/.vitepress/config.mts` — docs sidebar configuration.
- `README.md` — repo landing-page proof links.
- `reference-backend/scripts/verify-production-proof-surface.sh` — existing docs-truth verifier pattern.

## Expected Output

- `website/docs/docs/distributed-proof/index.md` — dedicated public distributed proof page.
- `website/docs/docs/distributed/index.md` — generic distributed guide with proof-surface routing.
- `website/docs/.vitepress/config.mts` — sidebar link to the new proof page.
- `README.md` — canonical distributed proof link on the repo landing page.
- `scripts/verify-m039-s04-proof-surface.sh` — docs-truth verifier for distributed claims.
