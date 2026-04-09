---
estimated_steps: 4
estimated_files: 5
skills_used:
  - frontend-design
  - make-interfaces-feel-better
  - react-best-practices
---

# T02: Polish the deck visuals, CTA surfaces, and responsive/print layout

**Slice:** S02 — Narrative polish, slide visuals, and launch-ready closeout
**Milestone:** M056

## Description

Once the narrative seam exists, make the route feel launch-ready instead of text-first. Reuse existing landing assets and CTA surfaces to render product imagery, Mesh/failover framing, flywheel treatment, traction/team credibility, and the final evaluator CTA. Tighten the deck shell and control rail for smaller viewports, replace broad motion with specific transitions, and keep richer media blocks readable in browser print.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Existing landing assets and visuals | Fail the build if asset imports or image framing break instead of shipping missing-image states. | N/A | Treat mismatched aspect ratio/layout use as a route-regression that must be fixed locally. |
| `WaitlistButton` and `lib/external-links.ts` reuse | Reuse the existing CTA surfaces exactly; do not fork a pitch-only flow when integration is awkward. | N/A | Treat undefined/stale destinations as closeout blockers, not follow-up cleanup. |
| Route-scoped print CSS | Keep the richer cards readable and chrome-hidden in print; fail the later spec if chrome leaks back in. | N/A | Treat hidden text/media in print mode as a real export regression. |

## Load Profile

- **Shared resources**: browser layout, image/media cards, sticky controls, and client-side emphasis transitions.
- **Per-operation cost**: one route render with a handful of local media assets and a small amount of pitch-local motion.
- **10x breakpoint**: mobile viewport density and print layout degrade before raw runtime cost, so polish work must compact and linearize intentionally.

## Negative Tests

- **Malformed inputs**: long slide titles/summaries, narrow viewports, and print media emulation.
- **Error paths**: missing asset/link wiring, CTA controls not rendering on the final frame, or agenda chrome overwhelming the slide body on mobile.
- **Boundary conditions**: first and last slides remain readable, the sticky control rail does not cover slide content, and the print view hides chrome while keeping the richer cards visible.

## Steps

1. Reuse existing landing imagery/assets and visual language to give each slide a deliberate product/Mesh/economics/traction/CTA treatment without copy-pasting full homepage sections.
2. Wire the final CTA slide to the existing `WaitlistButton` and canonical GitHub/X/Discord links instead of inventing pitch-only lead capture or stale destinations.
3. Compact `PitchControls` / shell layout for mobile, swap `transition-all` style drift for property-specific transitions, and apply typographic polish such as balanced headings and tabular numbers where needed.
4. Extend the route-scoped print CSS so the richer media/CTA blocks still export as a readable browser-native PDF.

## Must-Haves

- [ ] `/pitch` looks like a native hyperpush deck, not a stack of generic note cards.
- [ ] The CTA slide reuses the existing waitlist/link surfaces and keeps Mesh visible in the closing story.
- [ ] Mobile and print remain readable after richer visuals and motion land.

## Verification

- `npm --prefix mesher/landing run build`
- Manual review — compare desktop, narrow viewport, and print-media layout in the route-local components before handing off to the final Playwright closeout task.

## Observability Impact

- Signals added/changed: richer frame content must still preserve existing current-frame, agenda, and export-state markers.
- How a future agent inspects this: open `/pitch`, compare the active slide shell/controls against the variant interior, and emulate print media to confirm chrome is removed.
- Failure state exposed: whether regressions come from asset/CTA wiring, responsive layout pressure, or print-specific CSS leakage.

## Inputs

- `mesher/landing/components/pitch/pitch-slide-variants.tsx` — slide-specific rendering seam from T01.
- `mesher/landing/components/pitch/pitch-slide.tsx` — shared slide shell that must keep stable markers.
- `mesher/landing/components/pitch/pitch-deck.tsx` — deck shell and hero/current-frame composition.
- `mesher/landing/components/pitch/pitch-controls.tsx` — sticky agenda/control rail that needs responsive compaction.
- `mesher/landing/app/globals.css` — route-scoped print styles and shared landing tokens.
- `mesher/landing/components/landing/waitlist-dialog.tsx` — canonical waitlist CTA surface to reuse.
- `mesher/landing/lib/external-links.ts` — canonical public GitHub/X/Discord links.
- `mesher/landing/public/promo-performance.png` — product visual candidate.
- `mesher/landing/public/promo-flywheel.png` — token-flywheel visual candidate.
- `mesher/landing/public/roadmap-banner.png` — traction/roadmap visual candidate.
- `mesher/landing/public/vs-sentry-pricing.png` — product positioning visual candidate.

## Expected Output

- `mesher/landing/components/pitch/pitch-slide-variants.tsx` — finished visual/CTA slide interiors.
- `mesher/landing/components/pitch/pitch-slide.tsx` — polished shared shell and slide framing.
- `mesher/landing/components/pitch/pitch-deck.tsx` — refined deck shell and hero/current-frame layout.
- `mesher/landing/components/pitch/pitch-controls.tsx` — compact, launch-ready control rail.
- `mesher/landing/app/globals.css` — responsive and print-safe polish for the richer deck.
