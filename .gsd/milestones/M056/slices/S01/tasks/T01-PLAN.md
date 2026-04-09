---
estimated_steps: 4
estimated_files: 8
skills_used:
  - frontend-design
  - react-best-practices
  - agent-browser
---

# T01: Create the `/pitch` route shell, slide source, and browser proof harness

**Slice:** S01 — Pitch route foundation, navigation, and export
**Milestone:** M056

## Description

Close the risky route foundation first: establish one structured slide source and a real `/pitch` page in the landing app, but do it alongside a browser test harness so later navigation/export work has a durable proof rail instead of ad hoc manual clicks.

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

## Verification

- `npm --prefix mesher/landing run build`
- `npm --prefix mesher/landing exec playwright test tests/pitch-route.spec.ts --project=chromium --grep "route shell"`

## Inputs

- `mesher/landing/package.json` — existing landing scripts and dependency surface.
- `mesher/landing/app/layout.tsx` — shared metadata and root shell behavior.
- `mesher/landing/app/page.tsx` — current landing shell composition to stay visually native.
- `mesher/landing/app/mesh/page.tsx` — closest existing long-form evaluator-facing route.
- `mesher/landing/components/landing/header.tsx` — shared sticky header contract for route-local reuse.
- `mesher/landing/app/globals.css` — landing design tokens and global styles.

## Expected Output

- `mesher/landing/package.json` — landing-local scripts and direct browser-test dependency wiring.
- `mesher/landing/package-lock.json` — locked dependency graph including the browser test harness.
- `mesher/landing/playwright.config.ts` — Chromium-first landing route test configuration.
- `mesher/landing/app/pitch/layout.tsx` — route-local metadata for `/pitch`.
- `mesher/landing/app/pitch/page.tsx` — App Router entrypoint for the pitch deck.
- `mesher/landing/components/pitch/pitch-deck.tsx` — first-pass deck shell in the shared landing chrome.
- `mesher/landing/lib/pitch/slides.ts` — structured slide narrative source of truth.
- `mesher/landing/tests/pitch-route.spec.ts` — baseline route-shell browser assertions.
