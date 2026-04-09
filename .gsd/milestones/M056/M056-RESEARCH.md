# M056: Interactive Pitch Deck Page — Research

**Date:** 2026-04-04

## Summary

`mesher/landing/` is already capable of carrying this milestone without new framework work. The strongest existing reference is not the homepage but `mesher/landing/app/mesh/page.tsx`: it is already a standalone, long-form, motion-heavy narrative route that uses the same dark theme, green accent, mono labels, bordered sections, and animated systems storytelling the pitch page needs. The homepage components (`hero`, `features`, `flywheel`, `infrastructure`, `cta`) already contain most of the product language and visual motifs the deck should reuse.

The main architectural decision is not about animation libraries or layout primitives; those are already present. The real decision is export shape. There is no existing PDF or print surface in the landing app, and no `/pitch` route yet. That means the safest approach is a **print-first DOM enhanced into an interactive deck**, not a fullscreen slideshow bolted onto a later PDF path. If the page starts as absolute-positioned viewport scenes, print/export will become rework. If it starts as ordered slide sections, browser-native print-to-PDF is cheap, honest, and good enough for the stated “reasonable multi-page document” acceptance bar.

This milestone also intersects the broader public-story requirement already in the repo: `R120` says public surfaces should present one coherent evaluator-facing Mesh story. The existing landing copy still leans heavily into hyperpush tokenomics. The pitch page can safely sell hyperpush, but it should not drift back into a token-only product story that undersells Mesh’s distributed-systems differentiator.

## Recommendation

Build `/pitch` as a **route-local pitch system** with a server shell and a client interaction layer:

- `app/pitch/layout.tsx` for route metadata
- `app/pitch/page.tsx` as the route shell
- pitch-specific components under a local seam such as `components/landing/pitch/`
- a single slide-content source (static TS data/config) shared by screen mode and print mode
- a client deck controller for keyboard arrows, wheel/scroll navigation, clickable indicators, and export

Reuse existing landing patterns aggressively:

- visual language from `hero.tsx`, `cta.tsx`, and `app/mesh/page.tsx`
- systems animation ideas from `infrastructure.tsx` / `mesh-dataflow.tsx`
- CTA and social destinations from `waitlist-dialog.tsx` and `lib/external-links.ts`
- header/footer chrome from `header.tsx` and `footer.tsx`

For export, prefer **browser-native print / Save as PDF** over adding `jspdf`, `react-to-print`, or canvas snapshot tooling. That keeps the milestone additive, matches the “reasonable” export bar, and avoids a second rendering system.

## Implementation Landscape

### Key Files

- `mesher/landing/app/page.tsx` — homepage composition; shows the current public landing story and which sections already exist to mine for pitch content.
- `mesher/landing/app/mesh/page.tsx` — closest structural reference for a standalone narrative page inside the landing app; already uses route-local arrays, Framer Motion, section-based storytelling, and landing-consistent visuals.
- `mesher/landing/app/mesh/layout.tsx` — existing pattern for route-specific metadata; `/pitch` should follow this rather than relying only on root metadata.
- `mesher/landing/app/globals.css` — theme tokens and typography live here; there are currently no print rules, so export styles will need a new seam.
- `mesher/landing/components/landing/header.tsx` — sticky blurred header with optional `section`, `maxWidth`, and `extraActions`; likely reusable on `/pitch` with minimal or no shared changes.
- `mesher/landing/components/landing/footer.tsx` — existing footer/social contract; likely reused unchanged unless the pitch wants a lighter close.
- `mesher/landing/components/landing/waitlist-dialog.tsx` — existing waitlist CTA/modal; reuse instead of inventing a new lead-capture surface.
- `mesher/landing/components/landing/hero.tsx` — strongest product-positioning source for title/problem framing and the dashboard-style visual treatment.
- `mesher/landing/components/landing/features.tsx` — concise feature inventory for product/solution slides.
- `mesher/landing/components/landing/flywheel.tsx` — existing token-economics narrative and step structure; useful for the economics slide.
- `mesher/landing/components/landing/infrastructure.tsx` — current “Mesh-powered systems” section with canvas background pattern and evaluator-facing Mesh claims.
- `mesher/landing/components/landing/mesh-dataflow.tsx` — deeper animated cluster visual if the pitch needs a richer infrastructure slide.
- `mesher/landing/components/ui/carousel.tsx` — available Embla wrapper if a snap-based carousel is chosen, but it is optional and not obviously print-friendly by itself.
- `mesher/landing/lib/external-links.ts` — canonical GitHub / X / Discord links for CTA slides.
- `mesher/landing/public/{sentry-swap.png,vs-sentry-pricing.png,promo-*.png,roadmap-banner.png,x-banner.png}` — existing marketing assets that can seed product/traction slides without new illustration work.
- `.gsd/REQUIREMENTS.md` (`R120`) — current public-story contract; the pitch page should advance this, not fight it.

### Build Order

1. **Prove the route shell and slide data model first.**
   - Create `/pitch` with route metadata and a single source of truth for slide content.
   - Render slides as ordinary ordered sections/cards in DOM order.
   - This is the key risk retire: it keeps print/export viable and gives the planner a stable content boundary.

2. **Layer the interaction model on top of that DOM.**
   - Active-slide state
   - keyboard arrow navigation
   - wheel/scroll progression with transition locking
   - clickable slide indicators
   - optional hash/deep-link support if it falls out cheaply

3. **Add pitch-specific visuals and motion.**
   - Reuse the landing’s grid/orb/mono-label/card vocabulary.
   - Pull in richer visuals only where they carry the story: e.g. product showcase, Mesh infrastructure, token flywheel.
   - Keep the first slice content-correct before spending time on the most elaborate animation.

4. **Add export/print behavior after the DOM and navigation settle.**
   - Export button calling `window.print()` from the client shell
   - print-specific hiding of sticky/nav chrome
   - page breaks per slide
   - ensure each slide becomes a reasonable printed page, not a cropped viewport screenshot

5. **Finish with responsive polish and regression verification.**
   - desktop-first, but tablet/mobile must remain readable
   - confirm no shared landing regressions

### Verification Approach

- **Authoritative existing gate:** `npm --prefix mesher/landing run build`
  - Baseline is currently green.
  - Current route list does **not** include `/pitch`, so the new route should appear after implementation.
- **Important caveat:** Next build currently reports `Skipping validation of types` in this app.
  - A direct TypeScript pass (`npx tsc -p mesher/landing/tsconfig.json --noEmit`) is already **red at baseline** because of unrelated existing issues in `components/blog/editor.tsx`, `components/landing/infrastructure.tsx`, and `components/landing/mesh-dataflow.tsx`.
  - That means whole-app `tsc` is **not** a usable completion gate for this milestone unless the slice explicitly takes ownership of that debt.
- **Practical verification for the milestone:**
  - `npm --prefix mesher/landing run build`
  - browser exercise of `/pitch` for:
    - left/right arrow navigation
    - wheel/scroll navigation without multi-slide skipping
    - clickable indicators
    - export button opening print flow
    - printed/PDF output showing one sensible page per slide
  - changed-file diagnostics only (LSP or equivalent), not whole-app `tsc`

## Don't Hand-Roll

| Problem | Existing Solution | Why Use It |
|---------|------------------|------------|
| Lead capture / CTA | `components/landing/waitlist-dialog.tsx` + `WaitlistButton` | Keeps the pitch page on the same waitlist surface as the rest of the landing app. |
| Social / outbound links | `lib/external-links.ts` | Prevents stale or duplicated CTA destinations. |
| Motion system | `framer-motion` already used across landing + `/mesh` | No new animation dependency or learning curve. |
| Systems visual language | `components/landing/infrastructure.tsx` and `mesh-dataflow.tsx` | The repo already has a “Mesh cluster as animated visual” pattern. |
| PDF export | Browser print / Save as PDF | Matches the acceptance bar without introducing a second rendering stack. |
| Route metadata | `app/mesh/layout.tsx` pattern | Keeps `/pitch` title/description/OG copy route-local and easy to reason about. |

## Constraints

- The milestone context explicitly says the page must live at `mesher/landing/app/pitch/page.tsx`.
- Existing landing components are out of scope for broad changes; the safest plan is additive pitch-local work, not a shared landing refactor.
- The landing site is already committed to Next.js 16 App Router, Tailwind v4, Geist, Framer Motion, Radix/shadcn primitives, dark theme, and green accent tokens from `app/globals.css`.
- There is currently **no** `/pitch` route, **no** print CSS, and **no** existing PDF/export helper in the landing app.
- The app favors page-local composition over broad abstraction. `app/mesh/page.tsx` is a large route-local implementation; planners should not assume shared deck primitives already exist.
- Whole-app strict TypeScript is already red at baseline, so the milestone cannot realistically use “project-wide tsc clean” as its definition of done unless scope expands.

## Common Pitfalls

- **Fullscreen slideshow first, export later** — This is the likeliest bad plan. If slides only exist as viewport-locked scenes, print/PDF becomes screenshot theater. Keep slides as ordered DOM sections first, then enhance screen mode.
- **Raw wheel events causing skipped slides** — Trackpads can fire enough events to jump multiple slides. Use a lock/debounce or an observer-driven active-slide model instead of naive wheel increments.
- **Client-only route shell** — A fully client-only `app/pitch/page.tsx` works, but it makes metadata/export seams worse. Prefer a server route shell plus client deck component.
- **Refactoring shared landing components too early** — The milestone does not need a new global design system. Keep pitch-specific visuals local until a reusable seam is obvious.
- **Tokenomics dominating the deck** — The homepage copy already leans hard into tokens. The pitch page should still carry the evaluator-facing Mesh infrastructure story, or it will drift from `R120`.
- **Assuming build catches type issues** — In this app it does not. Treat build, browser behavior, and changed-file diagnostics as the real signals.

## Open Risks

- **Narrative balance risk** — The deck needs to sell hyperpush while still sounding like the same repo that just reset its public Mesh story. Too much token-economics language will make the public surfaces feel split-brain again.
- **Export-contract ambiguity** — If stakeholders expect a downloadable, custom-generated PDF file rather than browser print-to-PDF, that should be clarified early. The current acceptance text only supports the lighter contract.
- **Animation scope creep** — The landing app supports rich motion, but a 10-slide pitch deck can sprawl fast. The planner should keep “strong motion on a few key slides” separate from “elaborate animation on every slide.”

## Candidate Requirements

These are advisory findings from research, not auto-binding scope changes.

- **Candidate requirement:** Treat browser-native print-to-PDF as the milestone’s export contract unless the user explicitly wants a generated file download.
- **Candidate requirement:** The deck should remain readable as stacked content in print mode and under reduced-motion conditions, not only as an animated slideshow.
- **Candidate requirement:** The pitch narrative should explicitly include the Mesh infrastructure story strongly enough to advance `R120`, rather than presenting hyperpush as only a tokenized Sentry alternative.
- **Advisory only:** A visible current-slide URL/hash is nice to have but not table stakes unless sharing deep links between slides matters.
- **Advisory only:** Adding a homepage/nav entrypoint to `/pitch` is probably out of scope for this milestone, since existing landing component changes were explicitly excluded.

## Skills Discovered

| Technology | Skill | Status |
|------------|-------|--------|
| Frontend UI / landing-page design | `frontend-design` | available |
| React / Next.js performance patterns | `react-best-practices` | available |
| Framer Motion | `patricio0312rev/skills@framer-motion-animator` | installed |
| Tailwind CSS | `hairyf/skills@tailwindcss` | installed |
