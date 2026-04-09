# M053: Deploy Truth for Scaffolds & Packages — Context Draft

**Gathered:** 2026-04-02
**Status:** Draft — needs dedicated discussion before planning

## Seed From Current Discussion

- The serious starter proof should be the Postgres variant, not the SQLite variant.
- SQLite stays the easy local/single-node starter and must not imply shared clustered durability.
- The packages site should stay a separate deployed app, but its verification and deployment need to become part of the normal main CI/release evidence chain.
- Fly is the current proving ground, not the full product contract; public claims should stay platform-agnostic.

## Technical Findings From Investigation

- `.github/workflows/deploy-services.yml` already deploys `registry/` and `packages-website/` as separate Fly apps.
- Fly volumes are one-to-one with Machines, exist on one server, and do not replicate automatically.
- Fly Proxy can distribute traffic across Machines, and `Fly-Replay` exists for machine/region targeting, but that does not create shared SQLite durability.
- The current scaffold is still SQLite-only, so the serious Postgres starter path does not exist yet.

## Likely Dependencies

- Depends on M049 creating the dual-database scaffold and generated examples.
- Likely depends on M052 so the public-surface claim and deploy evidence line up.

## Scope Seed

### Likely In Scope
- real deployment proof for the Postgres starter
- packages-site verification/deploy evidence folded into the main CI/release contract
- truthful public explanation of what the serious starter proves versus what the local starter proves
- platform-agnostic contract wording even if Fly is the proving environment

### Likely Out of Scope
- pretending SQLite has shared clustered durability
- frontend-aware balancing adapters unless the later deep dive proves they are needed

## Questions For Dedicated Discussion

- What exact production-like proof bar should the Postgres starter meet: two-node cluster, endpoint exercise, failover, operator inspection, or all of them?
- How should the packages-site evidence chain plug into the main CI/release flow without collapsing it into the main docs deploy?
- Which parts of the deploy story must stay explicitly Fly-specific and which should be described generically?
- Should this milestone also produce reusable deployment assets/templates, or just proof and verification?
