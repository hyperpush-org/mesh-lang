---
depends_on: [M026]
draft: true
---

# M027: Launch Convenience + Hardening — Context Draft

**Gathered:** 2026-03-22
**Status:** Draft — needs dedicated discussion before planning

## Seed Material from Prior Discussion

This milestone combines two related late-stage concerns:
1. launch-through-app convenience if the Bags launch flow proves smooth and well supported in practice
2. hardening after real dogfooding, including any Mesh language/runtime/tooling fixes discovered during the build

Important user instruction carried forward verbatim in intent:
- if language limitations in Mesh are found while working on this project, stop what you are doing and fix the language, then dogfood those changes into this new project

This milestone should likely be where the product becomes easier to use and more resilient after the earlier milestones have already proven the live fund loop.

## What This Milestone Likely Unlocks

- optional Bags token launch-through-app flow for creators if it proves worth the complexity
- product and platform hardening after real usage and real failure discovery
- explicit closure on Mesh gaps exposed by the app

## Current Working Assumptions

- depends on the earlier milestones proving the core product loop first
- launch-through-app is desirable but intentionally secondary to the existing-token join flow
- security/signer hardening becomes more important after the hot-wallet hackathon MVP has proven the product loop
- Mesh fixes remain in scope and may land earlier if blocking, but this milestone is where broader consolidation/hardening likely belongs

## Open Questions for the Dedicated Discussion

- whether launch-through-app is truly worth the product and implementation complexity after observing the earlier Bags integration experience
- which hardening work matters most: signer architecture, operator controls, recovery tooling, deployment model, observability, reconciliation, or language/runtime cleanup
- how much of the Mesh dogfooding story should be visible in the product milestone versus treated as platform engineering underneath it
- whether this milestone should stay combined or split once real execution data exists

## Existing Codebase / Prior Art To Revisit

- Bags launch-token docs and token-launch transaction flow
- Mesh compiler/runtime tests and Mesher patterns for what the language already does well versus where it breaks under real product pressure
- any language or runtime issues encountered in M023-M026

## Draft Carry-Forward Notes

Preserve the user’s exact instruction and emphasis:
- if Mesh limitations block the app, fix Mesh first and then dogfood those changes here
- launch-through-app is wanted only if Bags support is strong enough and the complexity is justified

When auto-mode reaches this milestone, it should pause for discussion rather than treat this draft as final context.
