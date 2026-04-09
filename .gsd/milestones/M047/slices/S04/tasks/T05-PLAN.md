---
estimated_steps: 4
estimated_files: 5
skills_used:
  - vitepress
---

# T05: Rewrite public docs and migration guidance to one source-first clustered model

Update the public docs surface so new and existing users see one clustered model: route-free ordinary clustered functions are declared with `@cluster`, the generated scaffold and proof packages all share that contract, migration off `clustered(work)` / `[cluster]` is explicit, and the docs do not over-claim the unshipped `HTTP.clustered(...)` wrapper.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| VitePress content build | fail the docs build rather than shipping broken markdown/config references | bounded by the docs build; do not add extra long-running site tasks here | malformed code fences or links should fail build/link checks instead of silently rendering bad guidance |
| README / docs parity | treat contradictory wording as a public-contract regression | N/A | do not leave one page teaching legacy syntax while another page teaches the new model |
| migration guidance wording | keep the old surface clearly marked as legacy migration context, not as a coequal supported syntax | N/A | ambiguous wording should be treated as drift because this slice exists to cut ambiguity down |

## Load Profile

- **Shared resources**: markdown pages and the VitePress site build output.
- **Per-operation cost**: one site build plus markdown/code-fence validation.
- **10x breakpoint**: build failures from broken links/code fences appear before any meaningful performance issue.

## Negative Tests

- **Malformed inputs**: stale code blocks still showing `clustered(work)` or manifest clustering as current practice.
- **Error paths**: docs must explicitly say the HTTP route wrapper is not shipped yet instead of implying it exists.
- **Boundary conditions**: generated scaffold, `tiny-cluster`, and `cluster-proof` are all described as the same route-free source-first contract, with explicit migration language for existing users.

## Steps

1. Rewrite the clustered example, tooling, distributed proof, distributed overview, and top-level README text/code samples to the `@cluster` route-free contract.
2. Add explicit migration guidance off `clustered(work)` / `[cluster]` for existing users instead of simply deleting the old words with no explanation.
3. Make the docs truthful about current scope: ordinary clustered functions and route-free startup work are shipped; `HTTP.clustered(...)` is still not.
4. Rebuild the docs site so broken code fences, links, or page references fail before the verifier layer tries to reuse the pages.

## Must-Haves

- [ ] Public docs and README teach `@cluster` as the clustered source surface and explain migration off the old model.
- [ ] No public page claims `HTTP.clustered(...)` already shipped.
- [ ] Scaffold/example/proof-package wording is consistent across README and VitePress pages.

## Inputs

- ``README.md``
- ``website/docs/docs/getting-started/clustered-example/index.md``
- ``website/docs/docs/tooling/index.md``
- ``website/docs/docs/distributed-proof/index.md``
- ``website/docs/docs/distributed/index.md``

## Expected Output

- ``README.md``
- ``website/docs/docs/getting-started/clustered-example/index.md``
- ``website/docs/docs/tooling/index.md``
- ``website/docs/docs/distributed-proof/index.md``
- ``website/docs/docs/distributed/index.md``

## Verification

npm --prefix website run build
