---
estimated_steps: 4
estimated_files: 7
skills_used:
  - vitepress
---

# T03: Rewrite public and package docs around three equal canonical clustered surfaces

Update the public clustered story so the scaffold, `tiny-cluster/`, and `cluster-proof/` all teach the same route-free runtime-owned contract instead of splitting into routeful scaffold docs and route-free proof docs.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| VitePress docs pages and repo `README.md` | Fail `npm --prefix website run build` if rewritten pages break navigation or markdown structure. | Treat a slow docs build as task failure; do not leave the public story half-rewritten. | Treat contradictory routeful and route-free instructions as documentation drift rather than keeping both stories alive. |
| `tiny-cluster/README.md` and `cluster-proof/README.md` runbooks | Fail docs/content guards if the proof-package READMEs keep diverging on continuity list vs single-record inspection. | N/A | Treat request-key-only continuity guidance as incomplete operator documentation. |

## Negative Tests

- **Malformed inputs**: stale references to `[cluster]`, `Continuity.submit_declared_work(...)`, `/health`, `/work`, `Timer.sleep(...)` failover edits, or old verifier names as current truth.
- **Error paths**: the docs must explicitly reject app-owned proof/status routes instead of silently omitting them.
- **Boundary conditions**: the three surfaces may keep scope-specific notes, but they must share the same canonical operator flow: status, continuity list, continuity record, diagnostics.

## Steps

1. Rewrite `website/docs/docs/getting-started/clustered-example/index.md` and `website/docs/docs/tooling/index.md` around the new scaffold output: package-only manifest, source-owned `clustered(work)`, automatic startup work, and CLI-only inspection.
2. Rewrite `website/docs/docs/distributed-proof/index.md`, `website/docs/docs/distributed/index.md`, and repo `README.md` so they present the scaffold, `tiny-cluster/`, and `cluster-proof/` as equal canonical surfaces and point at the authoritative S05 verifier rather than the old S04 wrapper story.
3. Align `tiny-cluster/README.md` and `cluster-proof/README.md` on the same operator sequence: `meshc cluster status`, continuity list, continuity record, then diagnostics.
4. Remove the last routeful tutorial language (`[cluster]`, HTTP submit/health routes, delay-edit failover steps) and keep all cross-links pointed at the route-free clustered story.

## Must-Haves

- [ ] The public docs no longer teach `[cluster]`, HTTP submit/status routes, or proof-only delay edits as part of the clustered-example story.
- [ ] The scaffold, `tiny-cluster/`, and `cluster-proof/` are all named as equally canonical clustered-example surfaces.
- [ ] Every owned/package README that talks about inspection uses the same continuity list-then-record workflow.
- [ ] The docs/reference surfaces point at `scripts/verify-m046-s05.sh` as the authoritative closeout rail and keep `scripts/verify-m045-s05.sh` clearly historical.

## Done When

- [ ] The docs build cleanly and no slice-owned docs page still teaches the deleted routeful clustered contract.
- [ ] A reader following any of the three surface runbooks would learn the same runtime-owned clustered story.

## Inputs

- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `README.md`
- `tiny-cluster/README.md`
- `cluster-proof/README.md`

## Expected Output

- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `README.md`
- `tiny-cluster/README.md`
- `cluster-proof/README.md`

## Verification

npm --prefix website run build && ! rg -n "\[cluster\]|Continuity\.submit_declared_work|/health|/work/:request_key|Timer\.sleep\(5000\)" website/docs/docs/getting-started/clustered-example/index.md website/docs/docs/tooling/index.md website/docs/docs/distributed-proof/index.md website/docs/docs/distributed/index.md README.md
