# M056/S02: Narrative polish, slide visuals, and launch-ready closeout — Research

**Date:** 2026-04-04

## Summary

This slice directly advances **R120**. S01 already shipped the risky platform seam: `/pitch` exists, navigation is hash-addressable and bounded, export is browser-native, print CSS is scoped, and the route has a repo-owned Playwright rail. What is still missing is the actual launch-ready pitch experience. The current deck is a clean six-slide, text-first evaluator route, but it does **not** yet satisfy the roadmap language for polished product, Mesh, token-flywheel, traction/team, and CTA slides.

The strongest finding is that S02 is **not** a navigation or export task. Those seams are already in the right place. The real work is:

- expanding the narrative from the current six generic text cards into the actual evaluator-facing arc
- introducing slide-specific visual rendering instead of one generic `PitchSlide` body
- reusing existing landing visuals/assets without pasting whole homepage sections into the deck
- extending the existing route-local Playwright rail to cover the new narrative and responsive polish

The route is also visually flatter than the acceptance bar. Live inspection of `/pitch` confirms what the code suggests: the shell feels native, but the slide bodies still read like well-styled notes, not a finished pitch deck.

## Recommendation

Keep S02 **pitch-local** and preserve S01’s architecture:

- server route shell stays in `app/pitch/page.tsx`
- client navigation stays in `components/pitch/use-pitch-navigation.ts`
- export remains `window.print()` plus the existing `[data-pitch-page]` print CSS

Do **not** rework navigation or convert the route into a client-only wrapper. That would violate the S01 split and ignore the `react-best-practices` guidance to avoid unnecessary client ownership and extra bundle cost when the server shell is already working.

For the actual polish seam, introduce **slide-specific rendering** while keeping one ordered `pitchDeck` source. The cleanest options are:

1. extend `PitchSlide` to a discriminated union with per-slide `variant` / `visual` payloads, or
2. keep the current base type and add a pitch-local id → renderer map under `components/pitch/`

Either is fine, but the important part is to preserve:

- `slides[].id`
- ordered deck structure
- current `data-*` observability markers
- stable print DOM

That keeps navigation, hash sync, and export truthful while giving S02 space to add richer interiors.

Design-wise, follow the loaded skills explicitly:

- **`frontend-design`**: commit to one evaluator-facing aesthetic direction — dark, editorial, boardroom-demo polish — instead of adding disconnected “cool” widgets.
- **`make-interfaces-feel-better`**: use tabular numbers, `text-balance`/`text-pretty`, interruptible animations, and explicit transition properties instead of broad `transition-all` drift.
- **`react-best-practices`**: keep heavy visuals route-local and deliberate; do not sprawl state or push more than necessary into the client shell.

## Implementation Landscape

### Key files

- `mesher/landing/lib/pitch/slides.ts`
  - Single ordered deck source.
  - Right now it only supports `eyebrow`, `title`, `summary`, `bullets`, and optional `metrics`.
  - This is the primary narrative seam and will likely need new slide variants or visual payload fields.

- `mesher/landing/components/pitch/pitch-slide.tsx`
  - Current generic slide renderer.
  - This is the main S02 rendering seam: it should likely become a shared shell plus variant-specific interior content.

- `mesher/landing/components/pitch/pitch-deck.tsx`
  - Owns the route hero, current-frame panel, and the slide loop.
  - Good place for top-level ambient polish and route hero refinement.
  - Should not take over navigation logic.

- `mesher/landing/components/pitch/pitch-controls.tsx`
  - Already a solid sticky agenda/control rail.
  - S02 likely needs styling and responsive compaction here, not a logic rewrite.

- `mesher/landing/components/pitch/use-pitch-navigation.ts`
  - Already the correct bounded-input seam for keyboard/wheel/hash/scroll sync.
  - Avoid touching unless slide-count changes expose a real bug.

- `mesher/landing/components/pitch/pitch-export-button.tsx`
  - Export contract is already correct.
  - Only minor copy/styling tweaks should happen here.

- `mesher/landing/app/globals.css`
  - Route-scoped print rules live here.
  - Any new rich media or accent panels added in S02 may need additional scoped print overrides.

- `mesher/landing/tests/pitch-route.spec.ts`
  - Current tests prove route shell, deep links, bounded navigation, export, and print-media behavior.
  - They do **not** prove the final narrative, responsive readability, or launch CTA content.

- `mesher/landing/components/landing/hero.tsx`
  - Best source for product-demo card vocabulary: live-feed panel, sparkline treatment, alert badges, CTA rhythm.

- `mesher/landing/components/landing/flywheel.tsx`
  - Existing economics-step card pattern.
  - Useful for the token flywheel slide.

- `mesher/landing/components/landing/infrastructure.tsx`
  - Best source for two-column product/Mesh storytelling structure.
  - Useful as a layout reference and copy/visual vocabulary source.

- `mesher/landing/components/landing/mesh-dataflow.tsx`
  - Strong Mesh/failover visual motif.
  - Important caveat: the exported component is a full section with its own heading, copy, status bar, and height contract.
  - Better to extract or borrow its visual wrapper/status treatment than to paste the full section inside a pitch slide.

- `mesher/landing/components/landing/waitlist-dialog.tsx`
  - `WaitlistButton` is the existing launch CTA surface.
  - Reuse it instead of inventing a pitch-only lead capture flow.

- `mesher/landing/lib/external-links.ts`
  - Canonical GitHub / X / Discord links for the final CTA slide.

- `mesher/landing/public/promo-flywheel.png`
- `mesher/landing/public/promo-oss.png`
- `mesher/landing/public/promo-performance.png`
- `mesher/landing/public/promo-token-pricing.png`
- `mesher/landing/public/sentry-swap.png`
- `mesher/landing/public/vs-sentry-pricing.png`
- `mesher/landing/public/roadmap-banner.png`
  - Ready-to-use marketing assets.
  - The promo/screenshot images are all **2400×1350**; `roadmap-banner.png` is **2400×3520**.
  - These are good enough to seed product, traction, and positioning slides without a new illustration pipeline.

### Natural seams

1. **Narrative/data seam**
   - `pitchDeck` currently describes six text slides:
     - wedge
     - burst-load
     - mesh-moat
     - economics
     - distribution
     - platform
   - S02 acceptance still needs clearer product, traction/team, and CTA beats.

2. **Slide renderer seam**
   - Every slide currently renders through one generic article template.
   - This is the main reason the deck feels flat.
   - Keep one shell, but let interior content vary by slide.

3. **Visual reuse seam**
   - Existing assets and landing components are enough.
   - The right move is pitch-local composition, not importing whole homepage sections unchanged.

4. **Verification seam**
   - The current Playwright file already owns the route contract.
   - Extend that file instead of adding a second browser harness.

### What is missing right now

- No `framer-motion` usage anywhere under `components/pitch/` or `app/pitch/`
- No image/media rendering in the pitch route
- No explicit product showcase slide
- No traction / roadmap / team credibility slide
- No final CTA slide inside the ordered deck flow
- No responsive test coverage in `pitch-route.spec.ts`
- No slide-specific rendering API

## Build Order

1. **Lock the final narrative and slide count first.**
   - Update `pitchDeck` so the evaluator-facing story actually matches the roadmap/acceptance bar.
   - This retires the biggest product risk: the current route exists, but the deck story is still incomplete.

2. **Introduce slide-specific rendering without disturbing navigation.**
   - Refactor `PitchSlide` into a shared shell plus variant bodies.
   - Keep existing `data-testid`, `data-slide-id`, `data-active`, and heading hooks stable.

3. **Layer visuals and motion onto the new slide variants.**
   - Product slide: screenshot/promo asset + dashboard-style framing
   - Mesh slide: pitch-local Mesh/failover visual using `Infrastructure` / `MeshDataflow` motifs
   - Economics slide: step/flywheel treatment
   - Traction/team slide: roadmap or OSS credibility surface
   - CTA slide: `WaitlistButton` + canonical GitHub/X/Discord links

4. **Close responsive and print behavior.**
   - Compact the mobile agenda/control block if needed.
   - Make sure image/media blocks still print cleanly under the existing route-scoped print rules.

5. **Extend route-local verification and replay the build.**
   - Add narrative and responsive assertions to the existing Playwright file.
   - Re-run the landing build and the full `/pitch` spec.

## Verification Approach

- **Authoritative build gate:**
  - `npm --prefix mesher/landing run build`

- **Authoritative browser gate:**
  - `./mesher/landing/node_modules/.bin/playwright test --config ./mesher/landing/playwright.config.ts ./mesher/landing/tests/pitch-route.spec.ts --project=chromium`

- **Important existing constraint:**
  - Keep using the package-local Playwright binary with explicit config.
  - S01 already proved npm exec is not a truthful wrapper here.

- **What S02 should add to the spec:**
  - assertions for new slide titles/content markers
  - assertions for final CTA surfaces
  - at least one mobile-sized viewport assertion (for example via `page.setViewportSize(...)`)
  - print-media assertion that new visual blocks stay readable while chrome still hides

- **Useful live smoke after implementation:**
  - run `/pitch` locally on desktop and mobile
  - step through at least one newly visual slide
  - confirm CTA controls are present and usable
  - confirm console/network logs stay clean if browser verification is used

## Don't Hand-Roll

| Problem | Existing Solution | Why Use It |
|---------|------------------|------------|
| Waitlist CTA | `WaitlistButton` from `components/landing/waitlist-dialog.tsx` | Keeps the launch CTA aligned with the rest of the site and avoids a second lead-capture pattern. |
| External CTA links | `lib/external-links.ts` | Prevents stale GitHub/X/Discord destinations. |
| Product / marketing visuals | `mesher/landing/public/promo-*.png`, `sentry-swap.png`, `vs-sentry-pricing.png`, `roadmap-banner.png` | Existing assets are already landing-branded and high enough resolution for deck use. |
| Mesh visual vocabulary | `Infrastructure` / `MeshDataflow` | Gives a credible Mesh/failover visual without inventing a new visual system from zero. |
| Route/browser verification | existing `pitch-route.spec.ts` + `playwright.config.ts` | Keeps shell, navigation, print, and polish on one truthful route-local rail. |

## Constraints

- This slice supports **R120**. The polished deck still has to keep Mesh visible as the moat, not collapse back into a token-only hyperpush story.
- S01’s hash/nav/export contract should remain intact. New polish work must preserve the ordered DOM, current marker, agenda buttons, and print route markers.
- Print/export remains browser-native. New visuals must degrade into readable print instead of relying on fixed viewport choreography.
- The landing app already uses `antialiased`, `text-balance`, `text-pretty`, mono labels, and tabular numbers. S02 should stay inside that design language.
- `mesh-dataflow.tsx` and `infrastructure.tsx` are client-heavy homepage sections. Reuse carefully; do not paste whole section wrappers into the deck and call it done.
- `pitch-slide.tsx` currently uses `transition-all`; if S02 touches those animations, switch to specific transition properties per `make-interfaces-feel-better` instead of widening motion drift.

## Common Pitfalls

- **Expanding slide count without updating test assumptions.**
  - `pitch-route.spec.ts` currently hard-codes six agenda items and named slide targets.

- **Using full landing sections as slides.**
  - `MeshDataflow` and `Infrastructure` carry their own headings/copy/layout and will feel stitched-on if imported wholesale.

- **Adding rich visuals without print fallbacks.**
  - New image cards and accent panels will need scoped print resets under the existing `[data-pitch-page]` selectors.

- **Making the mobile agenda even taller.**
  - Current mobile layout already front-loads the agenda before slide content. S02 should compact that block, not bloat it.

- **Letting motion become the feature.**
  - The pitch route currently has zero pitch-local motion. Add staged emphasis, not ambient animation everywhere.

## Open Risks

- **Narrative drift risk** — product glam without explicit Mesh/failover framing would regress R120.
- **Over-reuse risk** — the easiest implementation is to paste homepage sections into the deck, but that will weaken the evaluator-deck tone.
- **Test drift risk** — changing slide titles/count without updating the existing Playwright expectations will make the route look broken even when navigation is still fine.

## Skills Discovered

| Technology | Skill | Status |
|---|---|---|
| Frontend UI / landing-page design | `frontend-design` | loaded |
| React / Next.js UI patterns | `react-best-practices` | loaded |
| Interaction polish / motion details | `make-interfaces-feel-better` | loaded |
| Framer Motion | `patricio0312rev/skills@framer-motion-animator` | installed |
| Tailwind CSS | `hairyf/skills@tailwindcss` | installed |