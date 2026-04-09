# S01: Pitch route foundation, navigation, and export

**Goal:** Ship the real `/pitch` route through the risky DOM-first architecture: route-local metadata, pitch-local slide model, server shell plus client deck controller, navigation state, and print-safe export behavior.
**Demo:** After this: Open `/pitch` in the real landing app and step through the deck with arrow keys, wheel/scroll, and slide indicators, then trigger browser print / Save as PDF from the live route.

## Tasks
- [x] **T01: Added the real `/pitch` landing route with route-local metadata, a single structured deck model, and a passing browser shell proof.** — Close the risky route foundation first: establish one structured slide source and a real `/pitch` page in the landing app, but do it alongside a browser test harness so later navigation/export work has a durable proof rail instead of ad hoc manual clicks.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Next.js App Router route + metadata load | Fail the baseline spec on `/pitch` render/title mismatch instead of silently falling back to `/` or a shared layout. | Treat boot timeout as route-start failure and keep the server log. | Treat missing slide content or metadata drift as contract failure, not a partial render. |
| Playwright dev server orchestration | Stop the test run with the startup error and keep the harness local to `mesher/landing`. | Fail fast if the local landing server never becomes ready. | Treat selector/title mismatches as route-shell drift rather than retry noise. |

## Load Profile

- **Shared resources**: local Next dev server, browser context, and route hydration.
- **Per-operation cost**: one page load plus baseline DOM assertions.
- **10x breakpoint**: repeated full reloads stress local startup first, so keep the baseline spec narrow and deterministic.

## Negative Tests

- **Malformed inputs**: direct navigation to `/pitch` with no prior app state, missing slide list entries, and missing route metadata.
- **Error paths**: server fails to boot, route throws during hydration, or the first slide content falls out of sync with the slide model.
- **Boundary conditions**: first-slide render, last slide existing in the source model, and route-only access with no homepage entry click.

## Steps

1. Add a landing-local Playwright harness in `mesher/landing/` with a Chromium project and a single spec file dedicated to `/pitch`.
2. Create `mesher/landing/app/pitch/layout.tsx` and `mesher/landing/app/pitch/page.tsx` plus a first-pass `PitchDeck` shell that uses the existing landing chrome instead of a standalone microsite.
3. Move the deck narrative into one structured source file so later interaction and print logic read the same ordered slide definitions.
4. Add a baseline browser spec named for the route-shell contract that asserts `/pitch` renders, carries route-local metadata, and shows the first slide/story markers expected by `R120` and `D392`.

## Must-Haves

- [ ] `/pitch` exists as a first-class landing route with route-local metadata and the same overall shell quality as existing landing routes.
- [ ] Slide content lives in one pitch-local data model rather than being duplicated across components.
- [ ] The landing app has a repo-owned browser test harness before navigation/export work starts.
  - Estimate: 2h
  - Files: mesher/landing/package.json, mesher/landing/package-lock.json, mesher/landing/playwright.config.ts, mesher/landing/app/pitch/layout.tsx, mesher/landing/app/pitch/page.tsx, mesher/landing/components/pitch/pitch-deck.tsx, mesher/landing/lib/pitch/slides.ts, mesher/landing/tests/pitch-route.spec.ts
  - Verify: `npm --prefix mesher/landing run build`
`npm --prefix mesher/landing exec playwright test tests/pitch-route.spec.ts --project=chromium --grep "route shell"`
- [x] **T02: Added a unified `/pitch` navigation controller with hash-synced controls, bounded keyboard/wheel handling, and verified browser navigation flows.** — Once the route exists, make the deck actually navigable from real browser input. Keep the state machine explicit and inspectable so arrow keys, wheel/scroll, and indicators all drive the same active-slide source of truth instead of layering independent handlers.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Browser `keydown` / wheel / scroll events | Keep navigation bounded to the current slide list and ignore unsupported inputs instead of throwing or skipping into invalid indices. | Debounce or throttle repeated wheel bursts so the deck settles predictably rather than jittering between slides. | Treat unexpected event payloads as no-ops and preserve the last known good active slide. |
| URL hash + DOM state sync | Prefer the in-memory active index and repair the hash/DOM markers on the next render instead of drifting into split state. | Do not block navigation on history updates; the deck should still move even if hash sync lags. | Treat unknown hashes as a fallback-to-first-valid-slide case, not as a blank deck. |

## Load Profile

- **Shared resources**: browser event stream, React state updates, and the slide viewport container.
- **Per-operation cost**: one bounded index update plus a small DOM/animation refresh.
- **10x breakpoint**: rapid wheel bursts and repeated hash writes would cause skipped-slide or history churn first, so normalization and throttling must happen before rendering.

## Negative Tests

- **Malformed inputs**: unknown hash values, repeated wheel deltas, and non-navigation keyboard keys.
- **Error paths**: first/last slide boundary presses, navigation during hydration, and a stale hash when the page opens directly on `/pitch#...`.
- **Boundary conditions**: first slide cannot move backward, last slide cannot move forward, and indicator clicks jump exactly to the requested slide.

## Steps

1. Add a pitch-local navigation controller that normalizes arrow keys, wheel/scroll progression, and indicator clicks into one bounded active slide index.
2. Split rendering into deck, slide, and control primitives that use existing landing tokens while keeping the current slide readable and obviously active.
3. Reflect active state in the URL hash and accessible control state (`aria-current`, disabled previous/next, or equivalent data attributes) so failures are inspectable from the DOM.
4. Extend Playwright with named navigation tests for keyboard, wheel, deep-link, and indicator-driven slide changes.

## Must-Haves

- [ ] Arrow keys, wheel/scroll, and slide indicators all drive the same active-slide state.
- [ ] Active slide state is inspectable from the URL and rendered controls instead of living only inside React internals.
- [ ] Navigation stays bounded and deterministic at the first and last slides.

## Observability Impact

- Signals added/changed: active slide hash plus visible/accessible current-slide markers on deck controls.
- How a future agent inspects this: open `/pitch`, inspect the hash and current indicator state, or rerun the focused navigation Playwright assertions.
- Failure state exposed: whether drift came from input handling, index bounds, or hash/DOM synchronization.
  - Estimate: 2h
  - Files: mesher/landing/components/pitch/pitch-deck.tsx, mesher/landing/components/pitch/pitch-slide.tsx, mesher/landing/components/pitch/pitch-controls.tsx, mesher/landing/components/pitch/use-pitch-navigation.ts, mesher/landing/tests/pitch-route.spec.ts
  - Verify: `npm --prefix mesher/landing exec playwright test tests/pitch-route.spec.ts --project=chromium --grep "navigation"`
- [x] **T03: Added browser-native `/pitch` print export with print-safe layout and end-to-end export assertions.** — Finish the slice by making export real in the browser rather than inventing a separate PDF generator. The `/pitch` route should trigger `window.print()`, switch cleanly into a print-friendly document, and keep the same slide content usable in print order.

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

## Observability Impact

- Signals added/changed: export control state plus print-media assertions in the shared pitch spec.
- How a future agent inspects this: rerun the full `/pitch` Playwright file and emulate print media in the browser while checking the export button path.
- Failure state exposed: whether breakage came from hydration/print wiring, print CSS, or broader landing build regressions.
  - Estimate: 1.5h
  - Files: mesher/landing/app/globals.css, mesher/landing/app/pitch/page.tsx, mesher/landing/components/pitch/pitch-deck.tsx, mesher/landing/components/pitch/pitch-export-button.tsx, mesher/landing/tests/pitch-route.spec.ts
  - Verify: `npm --prefix mesher/landing run build`
`npm --prefix mesher/landing exec playwright test tests/pitch-route.spec.ts --project=chromium`
