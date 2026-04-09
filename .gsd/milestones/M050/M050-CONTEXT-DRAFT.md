# M050: Public Docs Truth Reset — Context Draft

**Gathered:** 2026-04-02
**Status:** Draft — needs dedicated discussion before planning

## Seed From Current Discussion

- Remove non-user-facing docs such as proof pages, verifier maps, and milestone-rail material from the main public docs experience.
- Go through the docs site command-by-command and code-sample-by-code-sample, testing each one and fixing any drift.
- Keep low-level distributed primitives public, but make clustered apps and the source-first `@cluster` story the primary evaluator path.
- Explain clearly why `Distributed Actors` exists, what it teaches, and how it differs from the higher-level clustered-app/runtime-owned path.
- Stop treating `reference-backend/`, `tiny-cluster/`, and `cluster-proof/` as coequal public onboarding surfaces.

## Technical Findings From Investigation

- `website/docs/docs/distributed-proof/index.md` is currently a public verifier map full of repo-owned rails and retained compatibility aliases.
- `website/docs/docs/production-backend-proof/index.md` still presents `reference-backend/` as the public proof surface.
- `website/docs/docs/getting-started/index.md`, `website/docs/docs/tooling/index.md`, `website/docs/docs/databases/index.md`, `website/docs/docs/testing/index.md`, `website/docs/docs/concurrency/index.md`, and `website/docs/docs/web/index.md` still point readers into proof-oriented repo paths.
- `website/docs/docs/distributed/index.md` currently blends low-level `Node.*` / `Global.*` primitives with the scaffold-first clustered story and then immediately points users at verifier rails.
- `website/docs/docs/getting-started/clustered-example/index.md` still treats `tiny-cluster/` and `cluster-proof/` as equal public surfaces.

## Likely Dependencies

- Depends on M049 producing the new dual-database scaffold and generated `/examples` so docs can point at the right first-contact surfaces.
- Likely shapes M052 because the landing/site/packages story should match the docs story.

## Scope Seed

### Likely In Scope
- restructure docs information architecture around evaluator-facing learning paths
- move proof-heavy material out of the main public docs experience
- separate low-level distribution primitives from clustered-app guidance
- verify commands and code samples one by one with repo-owned checks
- rewrite getting-started/tooling/distributed pages around the new example surfaces

### Likely Out of Scope
- landing-page marketing rewrite
- Mesher modernization itself
- the deeper deploy/load-balancing implementation work

## Questions For Dedicated Discussion

- Which proof material, if any, should remain public but clearly secondary?
- Should internal proof maps move to repo READMEs, contributor docs, or a non-public docs section?
- What is the right sample-verification mechanism: one assembled docs verifier, page-local checks, or both?
- How aggressively should docs rename or retire older terminology that contributors still know?
