---
depends_on: [M034, M035, M036]
---

# M037: Package Experience & Ecosystem Polish — Context Draft

**Gathered:** 2026-03-26
**Status:** Draft — needs dedicated discussion before planning

## Seed From Current Discussion

The user wants the package manager improved because the UI is simple. When asked where that effort should center, they chose **website UX first** rather than CLI-first or equal split.

This came after choosing a harder trust-first path for M034. So the intended sequencing is: prove the package-manager and public release path honestly first, then improve the user-facing package experience on top of that trustworthy base.

## What This Milestone Likely Covers

- improve the packages website and publish/discovery flows first
- make the package manager feel more complete and less bare from a public-user perspective
- tighten the CLI experience where needed, but without displacing the website-first focus
- raise the overall ecosystem feel from “credible but thin” to “mature enough to be inviting”

## Why This Needs Its Own Discussion

The direction is clear, but the actual UX target is still under-specified. There is no concrete user decision yet on questions like:
- whether the main pain is discovery, package detail pages, publish guidance, account/token flow, version/history clarity, or overall visual polish
- how much CLI polish belongs in the same milestone once the website-first work is underway
- what package trust or metadata expectations are table stakes for a mature ecosystem experience

A dedicated discussion should turn “the UI is simple” into concrete user-visible capability requirements and a more precise polish bar.

## Existing Codebase / Prior Art To Revisit

- `packages-website/src/routes/+page.svelte`
- `packages-website/src/routes/publish/+page.svelte`
- `packages-website/src/routes/packages/[name]/+page.svelte`
- `compiler/meshpkg/src/search.rs`
- `compiler/meshpkg/src/install.rs`
- `website/docs/docs/tooling/index.md`

## Technical Findings Already Established

- the packages website is real and usable, but still visually and ergonomically simple
- the CLI package-manager surface is functional enough to support trust work, but not yet the main user-facing polish target
- the publish token and publish page flow already exist, which gives M037 a real base to improve instead of starting from scratch

## Likely Risks / Unknowns

- package UX can sprawl into a full product redesign if the milestone does not stay tied to concrete user-visible improvements
- some “simple UI” complaints may actually be metadata, trust-signaling, or package-discovery gaps rather than visual styling alone
- polishing the website before M034 trust work lands would risk making the package story look stronger than it really is

## Likely Outcome When Done

The package experience feels meaningfully more complete and polished, especially on the public website, while still staying grounded in the real proven package-manager path.

## Open Questions For The Dedicated Discussion

- What would make the packages website feel materially less simple and more complete?
- Which package details or trust signals are missing today?
- How much CLI polish belongs in the same milestone once website-first work is scoped?
