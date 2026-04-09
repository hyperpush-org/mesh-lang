---
estimated_steps: 4
estimated_files: 5
skills_used:
  - frontend-design
  - agent-browser
---

# T03: Finish browser-native print export and close the slice with end-to-end assertions

**Slice:** S01 — Pitch route foundation, navigation, and export
**Milestone:** M056

## Description

Finish the slice by making export real in the browser rather than inventing a separate PDF generator. The `/pitch` route should trigger `window.print()`, switch cleanly into a print-friendly document, and keep the same slide content usable in print order.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Client-side `window.print()` wiring | Keep the export control inert until hydration and surface a visible button/path failure instead of throwing during server render. | Treat a missing print callback in the test harness as export failure and keep the page stable. | Ignore repeated clicks after the first invocation rather than opening broken or duplicate print flows. |
| Print media CSS/layout | Fall back to readable stacked content instead of clipped full-screen slides or hidden sections. | N/A | Treat missing slide content under print media as contract failure, not a cosmetic issue. |
| Full landing build + browser test replay | Stop on the first failing phase and keep the failure local to build, render, or print assertions. | Fail fast if the browser harness cannot boot after the print/layout changes. | Treat print button/state mismatches as route export drift rather than silently passing the build alone. |

## Load Profile

- **Shared resources**: browser print media emulation, the full slide list in one printable document, and landing build output.
- **Per-operation cost**: one print invocation plus one print-media layout pass across the deck.
- **10x breakpoint**: oversized slide sections or sticky chrome leaking into print would degrade readability first, so print CSS must linearize content explicitly.

## Negative Tests

- **Malformed inputs**: export triggered from the first slide, a middle slide, or after direct deep-link load.
- **Error paths**: print button clicked before hydration, hidden controls still appearing in print mode, or slide content overflowing/clipping in print media.
- **Boundary conditions**: every slide title/body remains present in print mode and the browser-native export path does not depend on a custom backend or PDF service.

## Steps

1. Add a pitch-local export control wired into the route shell or deck controls that triggers browser-native print only after hydration.
2. Add print media styles that linearize every slide, remove sticky/navigation chrome, and keep the exported document readable as a browser PDF.
3. Extend the Playwright spec with a named export test that stubs `window.print`, verifies the button path, and checks print-media layout behavior.
4. Re-run the full landing build plus the complete pitch spec as the slice acceptance gate.

## Must-Haves

- [ ] `/pitch` exposes a real browser-native print / Save as PDF path instead of a fake export CTA.
- [ ] Print media produces a readable stacked deck with interactive chrome removed.
- [ ] The final build plus full Playwright spec prove route render, navigation, and export together.

## Verification

- `npm --prefix mesher/landing run build`
- `npm --prefix mesher/landing exec playwright test tests/pitch-route.spec.ts --project=chromium`

## Observability Impact

- Signals added/changed: export control state plus print-media assertions in the shared pitch spec.
- How a future agent inspects this: rerun the full `/pitch` Playwright file and emulate print media in the browser while checking the export button path.
- Failure state exposed: whether breakage came from hydration/print wiring, print CSS, or broader landing build regressions.

## Inputs

- `mesher/landing/app/globals.css` — landing global styles to extend with print rules.
- `mesher/landing/app/pitch/page.tsx` — route shell that needs the export entrypoint.
- `mesher/landing/components/pitch/pitch-deck.tsx` — navigable deck container from T02.
- `mesher/landing/components/pitch/pitch-controls.tsx` — existing control surface that can host or coordinate export.
- `mesher/landing/tests/pitch-route.spec.ts` — shared browser spec to finish with export assertions.

## Expected Output

- `mesher/landing/app/globals.css` — print media styling for readable slide export.
- `mesher/landing/app/pitch/page.tsx` — route shell updated with the export path.
- `mesher/landing/components/pitch/pitch-deck.tsx` — deck container coordinated with print/export behavior.
- `mesher/landing/components/pitch/pitch-export-button.tsx` — explicit browser-native export control.
- `mesher/landing/tests/pitch-route.spec.ts` — full route render, navigation, and export assertions.
