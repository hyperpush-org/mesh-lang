---
estimated_steps: 4
estimated_files: 5
skills_used:
  - frontend-design
  - react-best-practices
  - agent-browser
---

# T02: Wire slide navigation state for arrow keys, wheel/scroll, and indicators

**Slice:** S01 — Pitch route foundation, navigation, and export
**Milestone:** M056

## Description

Once the route exists, make the deck actually navigable from real browser input. Keep the state machine explicit and inspectable so arrow keys, wheel/scroll, and indicators all drive the same active-slide source of truth instead of layering independent handlers.

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

## Verification

- `npm --prefix mesher/landing exec playwright test tests/pitch-route.spec.ts --project=chromium --grep "navigation"`

## Observability Impact

- Signals added/changed: active slide hash plus visible/accessible current-slide markers on deck controls.
- How a future agent inspects this: open `/pitch`, inspect the hash and current indicator state, or rerun the focused navigation Playwright assertions.
- Failure state exposed: whether drift came from input handling, index bounds, or hash/DOM synchronization.

## Inputs

- `mesher/landing/app/pitch/page.tsx` — route entrypoint from T01.
- `mesher/landing/components/pitch/pitch-deck.tsx` — first-pass deck shell to extend with real state.
- `mesher/landing/lib/pitch/slides.ts` — ordered slide model that navigation must stay aligned with.
- `mesher/landing/tests/pitch-route.spec.ts` — baseline browser spec to extend with navigation assertions.
- `mesher/landing/components/landing/header.tsx` — shared header surface if controls or route actions need to live there.

## Expected Output

- `mesher/landing/components/pitch/pitch-deck.tsx` — deck container wired to explicit active-slide state.
- `mesher/landing/components/pitch/pitch-slide.tsx` — per-slide rendering primitive for visible/active state.
- `mesher/landing/components/pitch/pitch-controls.tsx` — inspectable indicators and previous/next controls.
- `mesher/landing/components/pitch/use-pitch-navigation.ts` — normalized navigation state/controller logic.
- `mesher/landing/tests/pitch-route.spec.ts` — keyboard, wheel, deep-link, and indicator navigation assertions.
