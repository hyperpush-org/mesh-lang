---
id: T02
parent: S01
milestone: M056
provides: []
requires: []
affects: []
key_files: ["mesher/landing/app/pitch/page.tsx", "mesher/landing/components/pitch/pitch-deck.tsx", "mesher/landing/components/pitch/pitch-slide.tsx", "mesher/landing/components/pitch/pitch-controls.tsx", "mesher/landing/components/pitch/use-pitch-navigation.ts", "mesher/landing/tests/pitch-route.spec.ts", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Keep `/pitch` as a server-rendered route shell and nest the interactive deck controller inside it so the landing footer stays server-owned while navigation state remains client-side and print-safe.", "Expose the active slide through the URL hash, route-shell data attributes, `aria-current`, and bounded previous/next buttons so navigation drift is inspectable from the DOM instead of React internals."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the landing app still builds with the new server-shell/client-controller split. Replayed the slice’s focused navigation rail through the package-local Playwright binary and explicit landing config after confirming the plan’s raw `npm exec` command still drops `--project` / `--grep` on this host. Replayed the full `/pitch` Playwright spec to keep the route-shell baseline green. In the live browser, exercised `/pitch` directly and confirmed hash repair on first load, bounded keyboard navigation, indicator-driven jumps, scroll-driven progression to the last slide, and clean console/network state."
completed_at: 2026-04-05T04:34:12.729Z
blocker_discovered: false
---

# T02: Added a unified `/pitch` navigation controller with hash-synced controls, bounded keyboard/wheel handling, and verified browser navigation flows.

> Added a unified `/pitch` navigation controller with hash-synced controls, bounded keyboard/wheel handling, and verified browser navigation flows.

## What Happened
---
id: T02
parent: S01
milestone: M056
key_files:
  - mesher/landing/app/pitch/page.tsx
  - mesher/landing/components/pitch/pitch-deck.tsx
  - mesher/landing/components/pitch/pitch-slide.tsx
  - mesher/landing/components/pitch/pitch-controls.tsx
  - mesher/landing/components/pitch/use-pitch-navigation.ts
  - mesher/landing/tests/pitch-route.spec.ts
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Keep `/pitch` as a server-rendered route shell and nest the interactive deck controller inside it so the landing footer stays server-owned while navigation state remains client-side and print-safe.
  - Expose the active slide through the URL hash, route-shell data attributes, `aria-current`, and bounded previous/next buttons so navigation drift is inspectable from the DOM instead of React internals.
duration: ""
verification_result: mixed
completed_at: 2026-04-05T04:34:12.748Z
blocker_discovered: false
---

# T02: Added a unified `/pitch` navigation controller with hash-synced controls, bounded keyboard/wheel handling, and verified browser navigation flows.

**Added a unified `/pitch` navigation controller with hash-synced controls, bounded keyboard/wheel handling, and verified browser navigation flows.**

## What Happened

I moved the landing chrome back to `app/pitch/page.tsx` so the route can keep the existing server footer while the interactive deck stays in a client-owned controller. Inside the deck, I added `use-pitch-navigation.ts` as the single state machine for active-slide index, hash parsing/repair, hash updates, bounded keyboard input, throttled wheel input, and scroll-driven active-slide reconciliation.

I split the old static shell into `PitchDeck`, `PitchControls`, and `PitchSlide` primitives. The deck now renders active state through the current-frame marker, `data-active-slide-id` / `data-active-slide-index` on the route shell, `data-active` on each slide, `aria-current="step"` on the active indicator button, and disabled previous/next buttons at the first and last slides. Indicator clicks, keyboard movement, wheel bursts, deep links, and scroll progression all converge on the same bounded index instead of owning separate ad hoc handlers.

I also expanded `mesher/landing/tests/pitch-route.spec.ts` with named navigation coverage for keyboard, wheel-burst throttling, deep-link repair, and indicator jumps. Finally, I recorded the landing-local Playwright npm-argument quirk in `.gsd/KNOWLEDGE.md` so later `/pitch` tasks rerun the truthful harness instead of the host-broken `npm exec` form.

## Verification

Verified the landing app still builds with the new server-shell/client-controller split. Replayed the slice’s focused navigation rail through the package-local Playwright binary and explicit landing config after confirming the plan’s raw `npm exec` command still drops `--project` / `--grep` on this host. Replayed the full `/pitch` Playwright spec to keep the route-shell baseline green. In the live browser, exercised `/pitch` directly and confirmed hash repair on first load, bounded keyboard navigation, indicator-driven jumps, scroll-driven progression to the last slide, and clean console/network state.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix mesher/landing run build` | 0 | ✅ pass | 32300ms |
| 2 | `npm --prefix mesher/landing exec playwright test tests/pitch-route.spec.ts --project=chromium --grep "navigation"` | 1 | ❌ fail | 22200ms |
| 3 | `./mesher/landing/node_modules/.bin/playwright test --config ./mesher/landing/playwright.config.ts ./mesher/landing/tests/pitch-route.spec.ts --project=chromium --grep "navigation"` | 0 | ✅ pass | 22500ms |
| 4 | `./mesher/landing/node_modules/.bin/playwright test --config ./mesher/landing/playwright.config.ts ./mesher/landing/tests/pitch-route.spec.ts --project=chromium` | 0 | ✅ pass | 23300ms |


## Deviations

The task-plan shape assumed the route shell could stay inside `components/pitch/pitch-deck.tsx`, but the local Next composition boundary made that dishonest because the deck now needs both the client header and the server footer. I moved route chrome into `app/pitch/page.tsx` and kept `pitch-deck.tsx` as the client-owned deck content. I also had to verify the named navigation rail through the package-local Playwright binary because npm 11 still strips `--project` and `--grep` from `npm --prefix mesher/landing exec playwright ...` on this host.

## Known Issues

None.

## Files Created/Modified

- `mesher/landing/app/pitch/page.tsx`
- `mesher/landing/components/pitch/pitch-deck.tsx`
- `mesher/landing/components/pitch/pitch-slide.tsx`
- `mesher/landing/components/pitch/pitch-controls.tsx`
- `mesher/landing/components/pitch/use-pitch-navigation.ts`
- `mesher/landing/tests/pitch-route.spec.ts`
- `.gsd/KNOWLEDGE.md`


## Deviations
The task-plan shape assumed the route shell could stay inside `components/pitch/pitch-deck.tsx`, but the local Next composition boundary made that dishonest because the deck now needs both the client header and the server footer. I moved route chrome into `app/pitch/page.tsx` and kept `pitch-deck.tsx` as the client-owned deck content. I also had to verify the named navigation rail through the package-local Playwright binary because npm 11 still strips `--project` and `--grep` from `npm --prefix mesher/landing exec playwright ...` on this host.

## Known Issues
None.
