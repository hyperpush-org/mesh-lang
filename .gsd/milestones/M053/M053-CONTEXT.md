# M053: Deploy Truth for Scaffolds & Packages — Context Draft

**Gathered:** 2026-04-04
**Status:** Ready for planning

## Project Description

This milestone turns the current starter split into a truthful deploy story. The serious path is the generated Postgres starter, not the SQLite starter. SQLite stays the honest local/single-node starter and must never imply shared clustered durability. The generated Postgres starter becomes the serious shared/deployable path, with real deployment proof, runtime/operator truth, reusable app deploy assets, and public wording that keeps Fly as the current proving ground without turning Fly into the product contract.

In parallel, the packages website stays a separately deployed app, but its deployment and verification must become part of the normal main release evidence chain instead of feeling bolted on beside the docs/site surface.

## Why This Milestone

Right now the repo has the starter split and some truthful wording, but the serious starter still does not have the deploy-grade proof bar the public story implies. That leaves a gap between “this is the serious shared/deployable starter” and “this has actually been proven under deployment and failure conditions.”

This milestone exists to close that gap without lying in either direction. SQLite should stay easy and honest. The Postgres starter should carry the serious deploy claim. Packages should not look like a side surface outside the real release contract. And Fly should remain the current proving environment, not the architecture story Mesh claims as its public contract.

## User-Visible Outcome

### When this milestone is complete, the user can:

- generate the Postgres Todo starter, deploy it in a production-like clustered setup, exercise real endpoints, inspect runtime truth, and see a node-loss/failover path survive
- rely on the public docs and release evidence to show that SQLite is local-first, Postgres is the serious deployable path, and the packages site is part of the normal public release surface

### Entry point / environment

- Entry point: `meshc init --template todo-api --db postgres`, `meshc cluster ...`, GitHub Actions release/deploy workflows, packages website deploy surface
- Environment: local dev, CI, production-like deployment, Fly as current proof environment
- Live dependencies involved: PostgreSQL, Fly.io, GitHub Actions, packages website

## Completion Class

- Contract complete means: the generated Postgres starter has named verifier/test/artifact proof for deploy + CRUD + runtime inspection + failover, the SQLite starter remains explicitly local-only in generated output and public docs, and packages-website evidence is part of the main release/deploy contract
- Integration complete means: generated starter, runtime cluster inspection, deployment assets, workflows, packages website deploy checks, and public docs all tell the same story without proof-maze drift
- Operational complete means: the serious Postgres starter can be deployed in a production-like clustered environment, survive a node-loss/failover scenario honestly, and the packages website can fail the same release/deploy chain if its public surface is broken

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- a generated Postgres Todo starter can be migrated, deployed in a clustered production-like environment, serve real CRUD traffic, expose truthful `meshc cluster` inspection, and survive a visible node-loss/failover path
- the packages website deploy and public-surface health checks run inside the normal main release/deploy evidence chain and fail that chain when broken
- the public starter/docs story cannot be satisfied by a local-only run, a Fly-only claim, or a lower-level maintainer verifier standing in for the serious starter’s own deploy proof

## Risks and Unknowns

- Deploy proof goes green while still being effectively local-only or Fly-only — that would recreate the exact fake-green problem this milestone exists to remove
- The Postgres starter’s shipped contract may still be narrower than a naive “full clustered app” expectation — the failover proof has to match the starter’s real semantics instead of inventing stronger claims
- Packages-site gating may widen the release blast radius — that is desirable for honesty, but it raises operational cost and will need careful failure surfaces
- Reusable deploy assets could drift into maintainer-only repo machinery — that would satisfy internal proof but fail the external-adopter goal
- Load-balancing expectations may surface during deploy proof — if current server-side routing/platform behavior is not strong enough, this milestone may need to stay honest and hand follow-through to the next milestone instead of silently overclaiming

## Existing Codebase / Prior Art

- `.github/workflows/deploy-services.yml` — already deploys `registry/` and `packages-website/` as separate Fly apps and already contains post-deploy public-surface checks; this is the existing seam that needs to become part of the normal main release evidence chain rather than a side story
- `compiler/mesh-pkg/src/scaffold.rs` — already generates the clustered scaffold plus the SQLite and Postgres Todo starters; the current README text explicitly says Postgres is the serious shared/deployable starter and SQLite is the honest local starter
- `examples/todo-postgres/README.md` — current serious-starter contract: PostgreSQL-backed state, `work.mpl`, explicit clustered read routes, local health/mutating routes, migrations, Docker packaging, and runtime inspection
- `examples/todo-sqlite/README.md` — current honest-local contract: local-only routes, SQLite state, no `work.mpl`, no `HTTP.clustered(...)`, and no `meshc cluster` story
- `website/docs/docs/getting-started/clustered-example/index.md` — current public ladder: `meshc init --clustered` first, then honest local SQLite starter, then serious shared/deployable PostgreSQL starter, then deeper proof pages
- `packages-website/fly.toml` — current separate deployed app surface that should stay separate operationally while becoming part of the normal public release contract
- `scripts/lib/m034_public_surface_contract.py` and the existing deploy verifiers — prior art for parser-backed/public-surface checks that fail closed on public drift

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- `R115` — advances the dual-database starter contract by making the Postgres branch genuinely deployable instead of merely described as such
- `R116` — advances the examples-first public story by making generated starters, not proof apps, carry the serious deploy claim
- `R117` — advances the evaluator-facing docs contract by keeping maintainer proof rails secondary to verified starter surfaces
- `R120` — advances the coherent public Mesh story across landing/docs/packages by aligning deploy truth with the public positioning work
- `R121` — directly advances the requirement that the packages website be part of the normal CI/deploy contract for the public Mesh surface
- `R122` — directly advances the requirement that Postgres gets truthful clustered deploy proof while SQLite stays explicitly local

## Scope

### In Scope

- real deployment proof for the generated Postgres starter
- end-to-end starter acceptance that includes deploy, endpoint exercise, runtime/operator truth, and failover
- reusable app deploy kit assets for the serious starter
- hard release/deploy gating for packages-website public-surface health
- truthful public explanation of what the Postgres starter proves versus what the SQLite starter proves
- platform-agnostic public contract wording with Fly retained as the current proof environment

### Out of Scope / Non-Goals

- pretending SQLite has shared clustered durability
- treating Fly as the public product contract or the only meaningful target Mesh supports
- collapsing packages-website into the docs deploy or treating it as “just docs”
- shipping infra-provisioning/template sprawl as the main output of the milestone
- maintainer-only proof surfaces standing in for the serious starter’s own deploy truth

## Technical Constraints

- SQLite must remain explicitly local/single-node in generated output, docs, and proof surfaces
- The public contract must stay platform-agnostic even if Fly remains the proving environment
- Packages-website stays a separately deployed app even while becoming a hard gate in the main release/deploy evidence chain
- The serious starter proof must be generated-starter-first, not a hand-curated proof fixture that only resembles the starter
- Public wording must not imply stronger clustered durability or broader deploy semantics than the starter/runtime actually prove

## Integration Points

- `meshc init --template todo-api --db postgres` — generated serious starter surface that must become deploy-truthful
- `meshc cluster status|continuity|diagnostics` — runtime/operator truth surface that must stay part of the serious starter acceptance story
- PostgreSQL — shared-state dependency for the serious starter
- Fly.io — current proving environment and deploy target for the production-like proof path
- GitHub Actions release/deploy workflows — normal evidence chain that must absorb packages-site gating and starter deploy proof
- `packages-website/` — separate deployed app whose verification/deploy must become part of the same normal release story
- public docs/README/packages surfaces — evaluator-facing wording that must match the actual proof bar

## Open Questions

- Should the reusable app deploy kit include a Fly-specific reference asset alongside the portable-first deploy kit, or should Fly stay entirely in proof/verifier surfaces? — Current thinking: keep the public contract portable-first, but allow Fly-specific reference assets as clearly secondary examples rather than the primary story.
- How much of the starter’s failover bar should be visible through the starter’s own public docs versus deeper proof pages? — Current thinking: the starter’s public docs should claim only the failover behavior the starter itself proves end to end, then point deeper/runtime-shaped details to secondary proof pages.
- If deploy proof reveals that current load-balancing/runtime follow-through is not good enough for the public clustered-app story, should M053 expand to include that follow-through or stop at honest documentation of the current limit? — Current thinking: fail closed on overclaiming and hand broader runtime/platform follow-through to the next milestone rather than smuggling it into this one without a clean proof bar.
