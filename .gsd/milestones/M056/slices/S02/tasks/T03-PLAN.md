---
estimated_steps: 4
estimated_files: 5
skills_used:
  - agent-browser
  - react-best-practices
---

# T03: Extend the route-local browser proof and close the launch-ready contract

**Slice:** S02 — Narrative polish, slide visuals, and launch-ready closeout
**Milestone:** M056

## Description

Close the slice on one authoritative proof rail. Expand the existing `/pitch` Playwright file to cover the updated slide count/titles, CTA visibility, responsive readability, and print-media behavior for the richer deck, then replay the full landing build plus the shared Chromium spec with the package-local Playwright binary and explicit config path. Fix any drift exposed by the replay in the route-local deck code instead of adding a second harness.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Playwright + landing dev/build orchestration | Stop the replay immediately and keep the failure local to the landing route harness. | Fail the command rather than silently skipping assertions when the server or browser never becomes ready. | Treat missing selectors/title/story markers as route drift, not retry noise. |
| Responsive viewport assertions | Point failures back to the route-local deck/components instead of broad browser flake. | Fail fast rather than stretching timeouts around layout bugs. | Treat clipped/overlapping CTA or control states as proof failures. |
| Print-media assertions | Fail the shared spec if chrome stays visible or richer cards disappear in print mode. | N/A | Treat missing text/media under print emulation as export regression. |

## Load Profile

- **Shared resources**: local Next.js build, Playwright-managed landing server, Chromium viewport changes, and print-media emulation.
- **Per-operation cost**: one production build plus a single focused browser file with desktop/mobile/print assertions.
- **10x breakpoint**: dev-server startup and large visual snapshots fail before route logic does, so keep assertions semantic and route-local.

## Negative Tests

- **Malformed inputs**: stale/unknown hashes, deep-link entry on a mid-deck slide, and narrow viewport rendering with long slide titles.
- **Error paths**: missing CTA markers, hidden final-slide actions, broken `window.print()` wiring, or print mode leaving controls/header/footer visible.
- **Boundary conditions**: first slide, final CTA slide, mobile viewport, and print media all continue to expose the ordered narrative and active/export markers.

## Steps

1. Extend `mesher/landing/tests/pitch-route.spec.ts` so the same file proves the richer titles/count, CTA surfaces, mobile readability, and print-media behavior.
2. Keep using the package-local Playwright binary with explicit config; do not switch to `npm exec` or add a second browser harness.
3. Run the full landing build and the full `/pitch` Chromium spec, then fix any route-local regressions the replay exposes.
4. Leave the route in a state where the shared `/pitch` proof file is the single browser-first acceptance gate for the deck.

## Must-Haves

- [ ] The Playwright rail proves the launch-ready narrative and CTA surfaces, not just the S01 shell skeleton.
- [ ] Mobile and print behavior are asserted explicitly in the same shared browser file.
- [ ] Final acceptance is the real landing build plus the full `/pitch` Chromium replay.

## Verification

- `npm --prefix mesher/landing run build`
- `./mesher/landing/node_modules/.bin/playwright test --config ./mesher/landing/playwright.config.ts ./mesher/landing/tests/pitch-route.spec.ts --project=chromium`

## Observability Impact

- Signals added/changed: the shared route spec should assert `data-active-slide-id`, current-frame text, CTA markers, and print/export state for the richer deck.
- How a future agent inspects this: rerun the explicit Chromium command and inspect the failing assertion in `mesher/landing/tests/pitch-route.spec.ts` before broad browser spelunking.
- Failure state exposed: whether the breakage is narrative drift, responsive overflow, CTA wiring, or print/export regression.

## Inputs

- `mesher/landing/tests/pitch-route.spec.ts` — existing route-local browser proof file to extend.
- `mesher/landing/playwright.config.ts` — explicit local Playwright config that must stay the proof entrypoint.
- `mesher/landing/lib/pitch/slides.ts` — source of truth for slide ids/titles/count.
- `mesher/landing/components/pitch/pitch-deck.tsx` — deck shell whose DOM markers and responsive layout are asserted.
- `mesher/landing/components/pitch/pitch-controls.tsx` — control rail whose compact/mobile states are asserted.
- `mesher/landing/components/pitch/pitch-slide.tsx` — shared slide shell whose print/readability states are asserted.
- `mesher/landing/app/globals.css` — route-scoped print styles that must keep the richer deck readable.

## Expected Output

- `mesher/landing/tests/pitch-route.spec.ts` — single authoritative browser-first acceptance rail for the finished deck.
- `mesher/landing/components/pitch/pitch-deck.tsx` — any route-local fixes required by the full replay.
- `mesher/landing/components/pitch/pitch-controls.tsx` — any responsive control-rail fixes required by the full replay.
- `mesher/landing/components/pitch/pitch-slide.tsx` — any print/readability fixes required by the full replay.
- `mesher/landing/app/globals.css` — any print/layout fixes required by the full replay.
