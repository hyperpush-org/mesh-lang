---
id: T01
parent: S01
milestone: M056
provides: []
requires: []
affects: []
key_files: ["mesher/landing/package.json", "mesher/landing/package-lock.json", "mesher/landing/app/pitch/layout.tsx", "mesher/landing/app/pitch/page.tsx", "mesher/landing/components/pitch/pitch-deck.tsx", "mesher/landing/lib/pitch/slides.ts", "mesher/landing/playwright.config.ts", "mesher/landing/tests/pitch-route.spec.ts", ".gsd/milestones/M056/slices/S01/tasks/T01-SUMMARY.md"]
key_decisions: ["Keep `/pitch` print-first and data-driven by rendering ordered DOM sections from one `pitchDeck` model instead of starting with a viewport-locked slideshow.", "Use route-local metadata plus a route-local Playwright harness so later navigation/export work extends one owned seam instead of shared landing globals."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified `npm --prefix mesher/landing run build` successfully and confirmed `/pitch` appears in the generated app route table. Exercised `http://127.0.0.1:3100/pitch` in the real browser and passed explicit assertions for the route URL, route-shell heading, current-frame marker, first/last slide story markers, and clean console/network state. Ran the landing-local Chromium Playwright route-shell spec successfully with the package-local Playwright binary plus `mesher/landing/playwright.config.ts`."
completed_at: 2026-04-05T04:20:20.736Z
blocker_discovered: false
---

# T01: Added the real `/pitch` landing route with route-local metadata, a single structured deck model, and a passing browser shell proof.

> Added the real `/pitch` landing route with route-local metadata, a single structured deck model, and a passing browser shell proof.

## What Happened
---
id: T01
parent: S01
milestone: M056
key_files:
  - mesher/landing/package.json
  - mesher/landing/package-lock.json
  - mesher/landing/app/pitch/layout.tsx
  - mesher/landing/app/pitch/page.tsx
  - mesher/landing/components/pitch/pitch-deck.tsx
  - mesher/landing/lib/pitch/slides.ts
  - mesher/landing/playwright.config.ts
  - mesher/landing/tests/pitch-route.spec.ts
  - .gsd/milestones/M056/slices/S01/tasks/T01-SUMMARY.md
key_decisions:
  - Keep `/pitch` print-first and data-driven by rendering ordered DOM sections from one `pitchDeck` model instead of starting with a viewport-locked slideshow.
  - Use route-local metadata plus a route-local Playwright harness so later navigation/export work extends one owned seam instead of shared landing globals.
duration: ""
verification_result: passed
completed_at: 2026-04-05T04:20:20.737Z
blocker_discovered: false
---

# T01: Added the real `/pitch` landing route with route-local metadata, a single structured deck model, and a passing browser shell proof.

**Added the real `/pitch` landing route with route-local metadata, a single structured deck model, and a passing browser shell proof.**

## What Happened

Added `mesher/landing/app/pitch/layout.tsx` and `page.tsx`, a new `components/pitch/pitch-deck.tsx` shell, and a validated `lib/pitch/slides.ts` source-of-truth model so the route renders through the existing landing chrome instead of a separate microsite. The deck now carries six ordered slides that keep hyperpush product framing tied to the Mesh systems story required by R120/D392, with visible baseline state markers (`Current frame · 01 / 06`, outline items, slide data attributes) for later navigation/export work. I also added a landing-local Playwright harness and baseline `/pitch` route-shell spec.

## Verification

Verified `npm --prefix mesher/landing run build` successfully and confirmed `/pitch` appears in the generated app route table. Exercised `http://127.0.0.1:3100/pitch` in the real browser and passed explicit assertions for the route URL, route-shell heading, current-frame marker, first/last slide story markers, and clean console/network state. Ran the landing-local Chromium Playwright route-shell spec successfully with the package-local Playwright binary plus `mesher/landing/playwright.config.ts`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix mesher/landing run build` | 0 | ✅ pass | 27300ms |
| 2 | `./mesher/landing/node_modules/.bin/playwright test --config ./mesher/landing/playwright.config.ts ./mesher/landing/tests/pitch-route.spec.ts --project=chromium --grep "route shell"` | 0 | ✅ pass | 4900ms |


## Deviations

The task-plan `npm --prefix mesher/landing exec playwright ...` form was not truthful on this host because npm 11 parsed `--project` and `--grep` as npm flags and skipped the package-local Playwright config from repo root. I verified the same landing-local harness with the package-local Playwright binary and explicit config path instead.

## Known Issues

The product route is green. The only remaining issue is verifier ergonomics: replay this rail with `./mesher/landing/node_modules/.bin/playwright test --config ./mesher/landing/playwright.config.ts ...` instead of the raw `npm exec` form on this host.

## Files Created/Modified

- `mesher/landing/package.json`
- `mesher/landing/package-lock.json`
- `mesher/landing/app/pitch/layout.tsx`
- `mesher/landing/app/pitch/page.tsx`
- `mesher/landing/components/pitch/pitch-deck.tsx`
- `mesher/landing/lib/pitch/slides.ts`
- `mesher/landing/playwright.config.ts`
- `mesher/landing/tests/pitch-route.spec.ts`
- `.gsd/milestones/M056/slices/S01/tasks/T01-SUMMARY.md`


## Deviations
The task-plan `npm --prefix mesher/landing exec playwright ...` form was not truthful on this host because npm 11 parsed `--project` and `--grep` as npm flags and skipped the package-local Playwright config from repo root. I verified the same landing-local harness with the package-local Playwright binary and explicit config path instead.

## Known Issues
The product route is green. The only remaining issue is verifier ergonomics: replay this rail with `./mesher/landing/node_modules/.bin/playwright test --config ./mesher/landing/playwright.config.ts ...` instead of the raw `npm exec` form on this host.
