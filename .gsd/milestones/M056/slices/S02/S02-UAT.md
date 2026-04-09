# S02: Narrative polish, slide visuals, and launch-ready closeout — UAT

**Milestone:** M056
**Written:** 2026-04-05T06:01:08.648Z

# S02: Narrative polish, slide visuals, and launch-ready closeout — UAT

**Milestone:** M056
**Written:** 2026-04-04

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: this slice is a browser-facing landing route, so the right acceptance bar is a mix of real route interaction, responsive/manual readability checks, and browser-native print/PDF confirmation against the shipped `/pitch` page.

## Preconditions

- `mesher/landing` dependencies are installed.
- Start the landing app locally (for example `npm --prefix mesher/landing run dev -- --hostname 127.0.0.1 --port 3100`).
- Use a desktop browser plus one narrow mobile viewport/emulator around 390x844.
- Allow the browser's native print dialog / Save as PDF flow.

## Smoke Test

1. Open `http://127.0.0.1:3100/pitch`.
2. Confirm the hero heading reads `hyperpush is the incident workflow teams can trust when event traffic gets ugly.`
3. Confirm the current-frame marker reads `Current frame · 01 / 06` and the first agenda item is the product slide.
4. **Expected:** the route loads as a finished deck, not a blank route or placeholder shell.

## Test Cases

### 1. Ordered narrative and agenda stay in sync

1. Open `/pitch` on desktop.
2. Verify the six slides appear in this order through the agenda or by scrolling the page: `product`, `workload`, `mesh-moat`, `flywheel`, `traction-team`, `cta`.
3. Confirm the first slide headline is `Start with a familiar wedge: open-source error tracking that feels ready for real incident ownership.`
4. Confirm the third slide contains the Mesh moat framing (`Actor isolation`, `Cluster continuity`, `Operator-visible recovery`).
5. Confirm the final slide headline is `Back the wedge now, then let Mesh turn one incident product into a platform story with real leverage.`
6. **Expected:** agenda labels, visible slide titles, and the hero/current-frame markers all reflect the same six-beat evaluator story.

### 2. Deep links and navigation controls stay truthful

1. Load `/pitch#mesh-moat` directly.
2. Confirm the current-frame marker reads `Current frame · 03 / 06` and the Mesh moat agenda item is marked current.
3. Press `ArrowRight` once and confirm the URL advances to `#flywheel`.
4. Click the agenda button for slide 05 and confirm the route jumps to `#traction-team`.
5. Navigate to `/pitch#not-a-real-slide`.
6. **Expected:** known hashes land on the requested slide, keyboard and agenda navigation move exactly one frame at a time, and an unknown hash repairs back to `#product` instead of leaving the deck in a broken state.

### 3. Final CTA slide uses the real landing conversion surfaces

1. Open `/pitch#cta`.
2. Confirm the final slide is active and the current-frame marker reads `Current frame · 06 / 06`.
3. Verify the `Join Waitlist` button is visible on the CTA slide.
4. Verify the three public CTA cards are visible and labeled `GitHub`, `X`, and `Discord`.
5. Open each card and confirm it resolves to the canonical public surface rather than a pitch-only destination.
6. **Expected:** the close of the deck uses the same waitlist and public social/community surfaces the landing site already owns.

### 4. Mobile viewport keeps controls and CTA actions readable

1. Switch to a narrow mobile viewport/emulator around `390x844`.
2. Open `/pitch#cta`.
3. Confirm the `Previous`, `Next`, and export controls are all visible and fit inside the viewport.
4. Confirm the CTA slide still shows the `Join Waitlist` button plus the three public CTA cards without clipping.
5. Swipe/scroll horizontally at the page level and confirm the document does not overflow sideways.
6. **Expected:** the control rail stays compact, the CTA actions remain usable, and the route does not develop page-level horizontal overflow on mobile.

### 5. Browser-native print / Save as PDF keeps the deck readable

1. Open `/pitch#traction-team` on desktop.
2. Trigger the route export button or open the browser print dialog for the page.
3. In print preview, confirm the landing header, footer, current-frame marker, and control rail are hidden.
4. Confirm all six slides remain visible in order, including the CTA slide.
5. Confirm the CTA slide still shows the `Join Waitlist` button and the GitHub/X/Discord cards in print preview.
6. Save the page as PDF.
7. **Expected:** the exported document reads like a print-safe deck, not an app shell screenshot, and the richer media/CTA framing remains legible.

## Edge Cases

### First and last frame bounds

1. Open `/pitch` and confirm the `Previous` button is disabled on the first slide.
2. Navigate to `/pitch#cta` and confirm the `Next` button is disabled on the final slide.
3. **Expected:** frame bounds stay explicit and the deck never wraps or skips beyond the first/last slide.

### Unsupported hash recovery

1. Open `/pitch#not-a-real-slide` in a fresh tab.
2. **Expected:** the route repairs to `#product`, the product slide becomes active, and the current-frame marker resets to `01 / 06`.

## Failure Signals

- `/pitch` loads without the six ordered slides or shows stale/placeholder copy from the earlier shell-only deck.
- The CTA slide is missing `Join Waitlist` or any of the canonical `GitHub` / `X` / `Discord` cards.
- Mobile view shows page-level horizontal overflow, clipped controls, or CTA cards extending beyond the viewport.
- Print preview keeps the landing header/footer/control rail visible or drops slide/CTA content from the exported order.
- Deep-link hashes stop matching the visible active slide or unknown hashes no longer repair back to `#product`.

## Requirements Proved By This UAT

- R120 — proves the landing app now has a coherent evaluator-facing `/pitch` route that frames hyperpush as the product while making Mesh a visible runtime moat.

## Not Proven By This UAT

- Full public-surface coherence across the broader homepage, docs, and packages site; this UAT only proves the shipped `/pitch` route.
- Package-wide TypeScript cleanliness for `mesher/landing`; the Next build still skips type validation and separate `tsc` evidence remains a targeted engineering check rather than a human-experience UAT step.

## Notes for Tester

- If the route looks broken only in Playwright CLI replay, prefer the package-local binary plus explicit config path (`./mesher/landing/node_modules/.bin/playwright test --config ./mesher/landing/playwright.config.ts ...`); `npm --prefix ... exec playwright` is not trustworthy on this host.
- If a future mobile regression shows oversized control buttons without obvious page overflow, inspect the `min-w-0` constraints in `pitch-controls.tsx` before weakening the responsive acceptance bar.
- If print assertions start failing after slide-copy edits, verify the assertion is scoped to the specific `[data-slide-id]` container before assuming the print/export contract regressed.

