---
depends_on: [M023]
draft: true
---

# M024: Creator Product & Trust Controls — Context Draft

**Gathered:** 2026-03-22
**Status:** Draft — needs dedicated discussion before planning

## Seed Material from Prior Discussion

This milestone should make the product feel trustworthy for creators. The product is creator-first: creators are the ones trusting the fund with a share of their royalties, even though the app should still give investors enough visibility to understand the fund.

Key creator-trust themes surfaced already:
- the product must not feel too custodial / sketchy
- the product must not be opaque about where royalties went
- the product must not feel too manual to count as a real fund product
- the product must not feel like a generic DeFi dashboard with weak creator clarity
- the product must not feel fragile for something touching real money

Likely milestone shape:
- creator-facing receipts and trust surfaces become much stronger than in M023
- visibility around exact split, destination wallet, fund behavior, and participation state becomes a first-class UX concern
- creator controls likely expand around status, reversibility/update semantics, and clear explanation of what is and is not authorized
- failure visibility should be understandable in product language, not only operator language

## What This Milestone Likely Unlocks

- creator confidence strong enough that the product feels safe to join, not merely technically functional
- a more complete creator dashboard showing participation status, claims, treasury actions, and failure state in creator-readable form
- stronger investor/public visibility, but still in service of creator trust first

## Current Working Assumptions

- depends on M023 proving the live creator→fund→claim→treasury loop
- creators remain the trust anchor
- browser wallet signing continues to own creator-controlled setup actions
- backend-managed fund wallet remains acceptable for automated fund actions in the hackathon MVP

## Open Questions for the Dedicated Discussion

- what exact controls creators need after joining: pause, reduce share, leave, update token config, or only visibility
- what “clear enough to trust” means in concrete UI terms beyond receipts/history
- how much investor/public surface belongs here versus in later growth/discovery milestones
- whether this milestone includes creator-specific educational surfaces explaining the trust model and custody split
- what failure visibility creators should see directly versus what remains operator-only

## Existing Codebase / Prior Art To Revisit

- `mesher/frontend/` — strongest UI stack prior art already in the repo
- `mesher/api/*.mpl` + `mesher/frontend/src/pages` — examples of backend/frontend split patterns
- `mesher/tests/*.test.mpl` — examples of product-oriented Mesh tests

## Draft Carry-Forward Notes

Preserve the user’s exact framing in the dedicated discussion:
- creators come first because they are the ones trusting the fund
- the app should still support both creators and investors
- the product should not feel too custodial / sketchy, too opaque about where royalties went, too manual to feel like a real fund product, too DeFi-dashboard-like, or too fragile

When auto-mode reaches this milestone, it should pause for discussion rather than pretending this draft is final context.
