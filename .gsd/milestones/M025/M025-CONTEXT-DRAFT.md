---
depends_on: [M024]
draft: true
---

# M025: Treasury Modes & Fund Market Surface — Context Draft

**Gathered:** 2026-03-22
**Status:** Draft — needs dedicated discussion before planning

## Seed Material from Prior Discussion

This milestone should turn the fund from a proven loop into a real treasury product with a stronger public surface.

The user explicitly wants support for both:
- buyback-and-burn
- dividend-style routing / holder distribution

Important nuance from discussion:
- the architecture should support both from the start
- live proof does not need to perfect both in the same first milestone
- the product should not get lost in abstract financial modeling before the operational loop is real

The likely role of this milestone is to take the fund token from “real and touched by the first proof loop” to “operationally legible and financially understandable.”

## What This Milestone Likely Unlocks

- treasury mode abstraction that can execute both buyback/burn and dividend-style routing honestly
- stronger public dashboard for fund health, participating creator tokens, treasury actions, and value-accrual story
- a clearer investor-facing surface without flipping the product into investor-first mode

## Current Working Assumptions

- depends on M024, which should sharpen creator trust and visibility first
- Mesh backend continues to own automation and operational orchestration
- a real fund token already exists from M023
- public credibility matters because the user wants strong growth/virality potential, but creators remain the trust anchor

## Open Questions for the Dedicated Discussion

- what exactly counts as “support both” in milestone terms: architecture only, one live path + one contract path, or both live
- which treasury mode gets the first polished product surface if they cannot mature equally at the same time
- what fund-token metrics matter most on the public surface: AUM, claim totals, treasury action history, estimated APY, volume, market cap
- how to communicate treasury behavior in a way that is legible to creators and investors without making the app feel like a generic DeFi terminal
- whether dividend-style routing is owned here directly or split with a later growth/discovery milestone

## Existing Codebase / Prior Art To Revisit

- Bags trade and claim docs for transaction generation, swaps, and claim history
- `mesher/frontend/` charting/state patterns for public-facing dashboards
- `mesher/api/*.mpl` patterns for API surfaces returning product metrics

## Draft Carry-Forward Notes

Preserve the user’s exact direction:
- support both treasury modes
- keep the architecture open to both now
- sequence proof instead of pretending both must be equally mature in the same first pass
- do not let the product drift into a generic DeFi dashboard

When auto-mode reaches this milestone, it should pause for discussion rather than treat this draft as final context.
