---
id: S01
parent: M056
milestone: M056
provides:
  - A real `/pitch` landing route with route-local metadata and existing landing chrome.
  - One validated pitch deck model plus deck/control/slide primitives ready for visual and narrative polish.
  - A repo-owned browser verification seam for route shell, hash-synced navigation, and browser-native print export.
requires:
  []
affects:
  - S02
key_files:
  - mesher/landing/app/pitch/layout.tsx
  - mesher/landing/app/pitch/page.tsx
  - mesher/landing/lib/pitch/slides.ts
  - mesher/landing/components/pitch/pitch-deck.tsx
  - mesher/landing/components/pitch/pitch-controls.tsx
  - mesher/landing/components/pitch/pitch-slide.tsx
  - mesher/landing/components/pitch/use-pitch-navigation.ts
  - mesher/landing/components/pitch/pitch-export-button.tsx
  - mesher/landing/app/globals.css
  - mesher/landing/playwright.config.ts
  - mesher/landing/tests/pitch-route.spec.ts
  - .gsd/PROJECT.md
key_decisions:
  - Keep `/pitch` landing-native and evaluator-facing by rendering it as an App Router route with route-local metadata instead of a detached microsite.
  - Keep the pitch content in one validated `pitchDeck` model so route shell, navigation, print export, and browser tests all read the same ordered slide source.
  - Keep the route shell server-owned and the deck controller client-owned so the landing footer/header remain native while navigation and export state stay isolated to the pitch deck.
  - Preserve export truth by rendering the deck as ordered DOM sections with explicit `[data-pitch-page]` / `data-pitch-print` markers and browser-native `window.print()` instead of adding a second PDF generator path.
patterns_established:
  - Server-rendered landing chrome plus a pitch-local client controller is the right split for `/pitch`; downstream polish should extend that seam instead of moving the whole route into a client wrapper.
  - Pitch state must stay inspectable from the DOM: URL hash, route-shell `data-active-slide-*`, current-frame marker, indicator `aria-current`, boundary button disabled state, and export `data-export-state` are now the canonical debugging hooks.
  - The landing-local Playwright harness is part of the feature contract, not an afterthought. Route shell, navigation, and export all share one `/pitch` spec file and explicit landing-local config.
observability_surfaces:
  - `[data-testid="pitch-route-shell"]` exposes `data-active-slide-id` and `data-active-slide-index` for the active deck position.
  - `[data-testid="pitch-current-marker"]`, indicator `aria-current="step"`, and disabled previous/next controls expose bounded navigation state without opening React internals.
  - `[data-testid="pitch-export-button"]` exposes `data-export-state`, and `mesher/landing/tests/pitch-route.spec.ts` is the authoritative route-shell/navigation/export regression surface.
drill_down_paths:
  - .gsd/milestones/M056/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M056/slices/S01/tasks/T02-SUMMARY.md
  - .gsd/milestones/M056/slices/S01/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-05T05:01:47.526Z
blocker_discovered: false
---

# S01: Pitch route foundation, navigation, and export

**Shipped a real landing-native `/pitch` route with one structured deck model, hash-addressable browser navigation, and browser-native print / Save as PDF export.**

## What Happened

S01 established `/pitch` as a first-class route inside `mesher/landing/` instead of a detached pitch microsite. The route now owns its own metadata in `app/pitch/layout.tsx`, renders through the existing landing header/footer in `app/pitch/page.tsx`, and reads all deck content from one validated `pitchDeck` source in `lib/pitch/slides.ts`. That same model feeds the hero copy, ordered slide sections, and the Playwright expectations, so later polish work has one narrative source instead of duplicated content drifting across components and tests.

The slice then turned the static route into a navigable browser deck without hiding state inside React internals. `use-pitch-navigation.ts` is the single controller for hash parsing/repair, bounded keyboard input, throttled wheel input, indicator jumps, and scroll reconciliation. `PitchDeck`, `PitchControls`, and `PitchSlide` expose the current state through `data-active-slide-id`, `data-active-slide-index`, the current-frame marker, `aria-current="step"` on the active indicator, and disabled previous/next buttons at the boundaries. That keeps drift debuggable from the DOM and gives downstream work stable hooks instead of implicit client-only state.

Finally, S01 closed the route on a truthful export path. `PitchExportButton` wraps browser-native `window.print()` with explicit `data-export-state`, and `app/globals.css` scopes print behavior to `[data-pitch-page]` and `data-pitch-print` markers so header/footer, controls, and decorative chrome disappear in print while the same ordered slide DOM stacks into a readable document. The landing-local Playwright harness now covers route shell, navigation, export, and print-media behavior together, so S02 can focus on polish without re-deriving the risky route/navigation/export seam.

## Verification

Verified `npm --prefix mesher/landing run build` and confirmed `/pitch` is emitted in the landing route table. Replayed the focused route-shell rail with `./mesher/landing/node_modules/.bin/playwright test --config ./mesher/landing/playwright.config.ts ./mesher/landing/tests/pitch-route.spec.ts --project=chromium --grep "route shell"`, the focused navigation rail with the same explicit-config invocation plus `--grep navigation`, and the full `/pitch` spec with the same package-local Playwright binary/config path. The first all-tests replay hit a transient `/pitch` 404/home-shell dev-server mismatch, but the immediate rerun passed all eight route-shell, navigation, export, and print-media tests; the focused rails and final full replay all proved the shipped route behavior. In a live browser against a local landing dev server, opened `/pitch`, confirmed initial hash/state markers (`#wedge`, current-frame marker, disabled previous button, export button `data-export-state=ready`), advanced with ArrowRight, jumped to slide 05 through the indicator controls, and verified the route continued exposing active-slide state through the DOM with clean console/network logs. A live export stub also confirmed the browser-native print handler increments without requiring a separate PDF service or backend path.

## Requirements Advanced

- R120 — The landing site now has a dedicated evaluator-facing `/pitch` route that keeps hyperpush’s product story visibly tied to Mesh-backed distributed-systems behavior instead of leaving landing positioning entirely stale or disconnected.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

- The plan’s raw `npm --prefix mesher/landing exec playwright ...` verification shape is not truthful on this host because npm 11 strips `--project` / `--grep` instead of passing them to Playwright. The authoritative replay seam is the package-local Playwright binary plus explicit `mesher/landing/playwright.config.ts`.
- The route chrome could not stay entirely inside the client deck component. `app/pitch/page.tsx` now owns the server-rendered landing header/footer while the interactive deck remains client-side so print/export stays route-local without turning the whole route wrapper into a client component.

## Known Limitations

- The deck foundation is shipped, but the visuals are still a first-pass structural version; S02 still needs narrative polish, responsive typography/layout tuning, motion, and CTA finish work.
- Export is intentionally browser-native print / Save as PDF. There is no separate branded PDF renderer or downloadable artifact beyond the browser print path.
- Operational visibility is repo-owned and route-local (`data-*` markers plus Playwright rails); there is no production monitoring or analytics surface for `/pitch` yet.

## Follow-ups

- Build S02 polish on the existing `pitchDeck` model, DOM-visible active/export state, and landing-local Playwright file rather than introducing a second content/render path.
- If the transient first-run full-spec `/pitch` 404 resurfaces, inspect the landing dev-server / Playwright web-server seam before rewriting route logic; the focused rails plus immediate rerun showed the route itself was healthy.

## Files Created/Modified

- `mesher/landing/app/pitch/layout.tsx` — Added route-local metadata for the evaluator-facing `/pitch` route, including title, description, and canonical/Open Graph surfaces.
- `mesher/landing/app/pitch/page.tsx` — Added the landing-native server shell for `/pitch`, keeping the existing header/footer while scoping the route with print hooks.
- `mesher/landing/lib/pitch/slides.ts` — Created the single validated deck model that now owns the route title/description, hero content, and ordered slide narrative.
- `mesher/landing/components/pitch/pitch-deck.tsx` — Implemented the main deck shell, current-frame state markers, and the composition boundary between hero, controls, and slide sections.
- `mesher/landing/components/pitch/pitch-controls.tsx` — Added bounded previous/next controls and agenda indicators with `aria-current` and stable test hooks.
- `mesher/landing/components/pitch/pitch-slide.tsx` — Added slide rendering primitives that preserve ordered DOM sections for both interactive browsing and print layout.
- `mesher/landing/components/pitch/use-pitch-navigation.ts` — Centralized hash parsing/repair, keyboard input, wheel throttling, scroll reconciliation, and bounded active-slide transitions.
- `mesher/landing/components/pitch/pitch-export-button.tsx` — Implemented the browser-native print button with explicit export-state observability and duplicate-request protection.
- `mesher/landing/app/globals.css` — Added route-scoped print media rules that hide chrome and linearize the same slide DOM into a readable exported document.
- `mesher/landing/playwright.config.ts` — Added the landing-local Playwright harness with an owned dev-server/baseURL seam for `/pitch` browser verification.
- `mesher/landing/tests/pitch-route.spec.ts` — Added and extended the `/pitch` route-shell, navigation, export, and print-media browser regression suite.
- `.gsd/PROJECT.md` — Refreshed project state to record that M056/S01 shipped the `/pitch` route foundation and that S02 now builds on it.
