---
estimated_steps: 3
estimated_files: 3
skills_used:
  - vitepress
---

# T01: Add a first-class clustered tutorial under Getting Started

Create the first-class clustered tutorial under Getting Started so the docs entrypoint matches the actual scaffold contract instead of the old inline aside. Keep the tutorial language-first: start with `meshc init --clustered`, show the generated files, run two local nodes, submit one keyed request, inspect cluster formation and continuity with the runtime CLI, and end with a concise failover walkthrough on the same tiny example plus a pointer to deeper proof docs.

## Steps

1. Add `website/docs/docs/getting-started/clustered-example/index.md` using the real scaffold contract from `compiler/mesh-pkg/src/scaffold.rs`: generated files, `Node.start_from_env()`, `Work.execute_declared_work`, `POST /work/:request_key`, and runtime `meshc cluster status|continuity|diagnostics`.
2. Update `website/docs/.vitepress/config.mts` and `website/docs/docs/getting-started/index.md` so clustered users see a dedicated Getting Started entry instead of the current inline digression inside hello-world.
3. Keep the page scoped to the scaffold-first story: include the same-example happy path and failover walkthrough, and point deeper operator/Fly details at `/docs/distributed-proof/` rather than teaching `cluster-proof`-only HTTP surfaces or `CLUSTER_PROOF_*` env as the primary contract.

## Must-Haves

- [ ] `website/docs/docs/getting-started/clustered-example/index.md` exists and teaches the actual scaffold contract from `compiler/mesh-pkg/src/scaffold.rs`.
- [ ] The Getting Started sidebar and introduction page route clustered readers to the new page as a first-class tutorial.
- [ ] The new page keeps proof rails secondary and does not teach `cluster-proof` HTTP/status surfaces as if they were part of the scaffold.

## Inputs

- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/meshc/tests/tooling_e2e.rs`
- `website/docs/.vitepress/config.mts`
- `website/docs/docs/getting-started/index.md`

## Expected Output

- `website/docs/.vitepress/config.mts`
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`

## Verification

npm --prefix website run build
rg -n '/docs/getting-started/clustered-example/|meshc init --clustered|meshc cluster status|meshc cluster continuity|meshc cluster diagnostics' website/docs/.vitepress/config.mts website/docs/docs/getting-started/index.md website/docs/docs/getting-started/clustered-example/index.md
