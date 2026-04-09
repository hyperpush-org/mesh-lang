---
estimated_steps: 4
estimated_files: 7
skills_used:
  - vitepress
---

# T03: Repoint clustered docs and runbooks to the S06 closeout rail

Promote the new S06 closeout rail across the public clustered story without changing the underlying operator flow or reopening any routeful example seams.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `README.md` and clustered website docs | Fail `npm --prefix website run build` if the new references break markdown, links, or VitePress rendering. | Treat a slow docs build as task failure; do not leave the public story half-rewritten. | Treat contradictory S05/S06 authority wording or revived routeful instructions as docs drift. |
| `tiny-cluster/README.md` and `cluster-proof/README.md` runbooks | Fail the doc/content guards if the package READMEs stop sharing the same status → continuity list → continuity record → diagnostics operator sequence. | N/A | Treat stale wrapper names or request-key-only continuity guidance as incomplete operator documentation. |

## Negative Tests

- **Malformed inputs**: stale references to `scripts/verify-m046-s05.sh` as current truth, `[cluster]`, `Continuity.submit_declared_work(...)`, `/health`, `/work/:request_key`, or proof-only timing edits.
- **Error paths**: docs must clearly demote S05 to the equal-surface subrail and keep `scripts/verify-m045-s05.sh` historical instead of presenting multiple present-tense closeout rails.
- **Boundary conditions**: the three canonical clustered surfaces may keep scope-specific notes, but they must share the same runtime-owned operator flow and final closeout pointer.

## Steps

1. Update `README.md`, `website/docs/docs/distributed-proof/index.md`, `website/docs/docs/distributed/index.md`, `website/docs/docs/tooling/index.md`, and `website/docs/docs/getting-started/clustered-example/index.md` so they name `scripts/verify-m046-s06.sh` as the authoritative route-free closeout rail and demote `scripts/verify-m046-s05.sh` to the lower-level equal-surface subrail.
2. Update `tiny-cluster/README.md` and `cluster-proof/README.md` so their package runbooks still teach the canonical `meshc cluster status`, continuity list, continuity record, diagnostics sequence while pointing repo-wide closeout readers at S06.
3. Keep the three clustered surfaces (`meshc init --clustered`, `tiny-cluster/`, and `cluster-proof/`) explicitly equal and canonical, preserving R090 while preventing stale S05-only wording from becoming the public truth.
4. Remove any remaining routeful or app-owned operator language from the slice-owned docs surfaces instead of trying to explain both stories side by side.

## Must-Haves

- [ ] Public docs and repo/package READMEs name `scripts/verify-m046-s06.sh` as the final closeout rail.
- [ ] `scripts/verify-m046-s05.sh` is described only as the equal-surface subrail and `scripts/verify-m045-s05.sh` remains clearly historical.
- [ ] The operator flow stays `meshc cluster status`, continuity list, continuity record, diagnostics across all clustered surfaces.
- [ ] Routeful/app-owned submit/status/timing instructions do not reappear in the repointed docs surfaces.

## Done When

- [ ] `npm --prefix website run build` passes against the repointed docs.
- [ ] The S06 Rust doc/content guards pass against the updated authoritative-versus-historical wording.

## Inputs

- `README.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `tiny-cluster/README.md`
- `cluster-proof/README.md`
- `compiler/meshc/tests/e2e_m046_s06.rs`

## Expected Output

- `README.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `tiny-cluster/README.md`
- `cluster-proof/README.md`

## Verification

npm --prefix website run build && cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture && ! rg -n "\[cluster\]|Continuity\.submit_declared_work|/health|/work/:request_key|Timer\.sleep\(5000\)" README.md website/docs/docs/distributed-proof/index.md website/docs/docs/distributed/index.md website/docs/docs/tooling/index.md website/docs/docs/getting-started/clustered-example/index.md tiny-cluster/README.md cluster-proof/README.md
