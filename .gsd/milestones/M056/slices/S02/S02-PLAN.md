# S02: Narrative polish, slide visuals, and launch-ready closeout

**Goal:** Turn `/pitch` from a structurally correct evaluator route into a launch-ready hyperpush deck: product-forward, Mesh-visible, visually polished, responsive, and still truthful to the S01 navigation/export contract.
**Demo:** After this: The finished `/pitch` deck feels native to hyperpush: polished product, Mesh, token-flywheel, traction/team, and CTA slides animate cleanly, stay readable responsively, and keep the landing build green.

## Tasks
- [x] **T01: Rebuilt the `/pitch` deck as a validated six-beat evaluator narrative with explicit slide variants and synced route-shell proof.** — Lock the final evaluator-facing story first. Expand `pitchDeck` into a launch-ready arc with explicit product, Mesh/runtime moat, token-flywheel, traction/team, and CTA beats, then introduce a pitch-local slide-variant rendering seam so each frame can render richer interiors without breaking stable slide ids, ordered DOM sections, or the S01 state markers. Update the existing route-shell assertions in the shared Playwright file so the narrative source and the public route stay in sync.

Steps:
1. Extend `mesher/landing/lib/pitch/slides.ts` with the slide metadata/visual payload needed for launch-ready product, Mesh, economics, traction/team, and CTA frames.
2. Keep `mesher/landing/components/pitch/pitch-slide.tsx` as the shared shell, but move slide-specific interiors into a pitch-local variant component/module instead of keeping one generic article body.
3. Preserve `slide.id`, ordered section rendering, heading hooks, and existing `data-testid` / `data-*` state markers so navigation, hash sync, and export remain truthful.
4. Refresh the existing route-shell assertions in `mesher/landing/tests/pitch-route.spec.ts` for the new slide titles/count and evaluator-facing story markers.

Must-haves:
- The deck story explicitly covers product, Mesh, token-flywheel, traction/team, and CTA beats.
- Slide-specific rendering stays route-local and does not import whole homepage sections as-is.
- The route-shell proof follows the updated deck source instead of stale six-slide text-only copy.

Done when:
- `/pitch` renders the new ordered narrative from one deck source, each frame has the right variant hook, and the route-shell spec expects the new story rather than S01 placeholder copy.
  - Estimate: 2.5h
  - Files: mesher/landing/lib/pitch/slides.ts, mesher/landing/components/pitch/pitch-slide.tsx, mesher/landing/components/pitch/pitch-deck.tsx, mesher/landing/tests/pitch-route.spec.ts, mesher/landing/components/landing/flywheel.tsx, mesher/landing/components/landing/infrastructure.tsx, mesher/landing/components/landing/mesh-dataflow.tsx
  - Verify: npm --prefix mesher/landing run build
./mesher/landing/node_modules/.bin/playwright test --config ./mesher/landing/playwright.config.ts ./mesher/landing/tests/pitch-route.spec.ts --project=chromium --grep "route shell"
- [x] **T02: Turned `/pitch` into an asset-backed hyperpush deck with reusable CTA surfaces, denser mobile controls, and richer print-safe slide framing.** — Once the narrative seam exists, make the route feel launch-ready instead of text-first. Reuse existing landing assets and CTA surfaces to render product imagery, Mesh/failover framing, flywheel treatment, traction/team credibility, and the final evaluator CTA. Tighten the deck shell and control rail for smaller viewports, replace broad motion with specific transitions, and keep richer media blocks readable in browser print.

Failure modes:
- If asset composition or CTA reuse breaks, fail closed in the build rather than shipping missing-image or undefined-link states.
- If richer visuals overflow or collapse on small screens, keep the agenda and current-frame markers readable instead of letting chrome bury the slide body.
- If print styles leak interactive chrome or hide media/text, treat that as a deck regression, not a cosmetic follow-up.

Load profile:
- Shared resources: browser layout, image/media cards, sticky controls, and client-side transitions.
- Per-operation cost: one route render with a handful of local media assets and animated emphasis states.
- 10x breakpoint: mobile viewport density and print layout degrade before raw runtime cost, so polish work must compact and linearize intentionally.

Negative tests:
- Malformed inputs: long slide titles/summaries, narrow viewports, and print media emulation.
- Error paths: missing asset/link wiring, CTA controls not rendering on the final frame, or agenda chrome overwhelming the slide body on mobile.
- Boundary conditions: first and last slides remain readable, the sticky control rail does not cover slide content, and the print view hides chrome while keeping the richer cards visible.

Steps:
1. Reuse existing landing imagery/assets and visual language to give each slide a deliberate product/Mesh/economics/traction/CTA treatment without copy-pasting full homepage sections.
2. Wire the final CTA slide to the existing `WaitlistButton` and canonical GitHub/X/Discord links instead of inventing pitch-only lead capture or stale destinations.
3. Compact `PitchControls` / shell layout for mobile, swap `transition-all` style drift for property-specific transitions, and apply typographic polish such as balanced headings and tabular numbers where needed.
4. Extend the route-scoped print CSS so the richer media/CTA blocks still export as a readable browser-native PDF.

Must-haves:
- `/pitch` looks like a native hyperpush deck, not a stack of generic note cards.
- The CTA slide reuses the existing waitlist/link surfaces and keeps Mesh visible in the closing story.
- Mobile and print remain readable after richer visuals and motion land.

Done when:
- The deck reads cleanly on desktop and mobile, the final CTA is wired to existing public surfaces, and richer cards still degrade into a readable print/export document.
  - Estimate: 3h
  - Files: mesher/landing/components/pitch/pitch-slide.tsx, mesher/landing/components/pitch/pitch-deck.tsx, mesher/landing/components/pitch/pitch-controls.tsx, mesher/landing/app/globals.css, mesher/landing/components/landing/waitlist-dialog.tsx, mesher/landing/lib/external-links.ts, mesher/landing/public/promo-performance.png, mesher/landing/public/promo-flywheel.png, mesher/landing/public/roadmap-banner.png, mesher/landing/public/vs-sentry-pricing.png
  - Verify: npm --prefix mesher/landing run build
- [x] **T03: Hardened the /pitch browser rail for launch-ready CTA, mobile, and print behavior and fixed the mobile control-rail width bug.** — Close the slice on one authoritative proof rail. Expand the existing `/pitch` Playwright file to cover the updated slide count/titles, CTA visibility, responsive readability, and print-media behavior for the richer deck, then replay the full landing build plus the shared Chromium spec with the package-local Playwright binary and explicit config path. Fix any drift exposed by the replay in the route-local deck code instead of adding a second harness.

Failure modes:
- Playwright/dev-server boot failures should stop the replay immediately and keep the landing-local logs; do not mask them with partial green assertions.
- Responsive assertions that fail because the control rail or media blocks overflow must point back to the route-local deck/components, not to generic browser flake.
- Print assertions that keep chrome visible or lose slide content must fail the shared spec rather than leaving export correctness to manual review.

Load profile:
- Shared resources: local Next.js build, Playwright-managed landing server, Chromium viewport changes, and print-media emulation.
- Per-operation cost: one production build plus a single focused browser file with desktop/mobile/print assertions.
- 10x breakpoint: dev-server startup and large visual snapshots fail before route logic does, so keep assertions semantic and route-local.

Negative tests:
- Malformed inputs: stale/unknown hashes, deep-link entry on a mid-deck slide, and narrow viewport rendering with long slide titles.
- Error paths: missing CTA markers, hidden final-slide actions, broken `window.print()` wiring, or print mode leaving controls/header/footer visible.
- Boundary conditions: first slide, final CTA slide, mobile viewport, and print media all continue to expose the ordered narrative and active/export markers.

Steps:
1. Extend `mesher/landing/tests/pitch-route.spec.ts` so the same file proves the richer titles/count, CTA surfaces, mobile readability, and print-media behavior.
2. Keep using the package-local Playwright binary with explicit config; do not switch to `npm exec` or add a second browser harness.
3. Run the full landing build and the full `/pitch` Chromium spec, then fix any route-local regressions the replay exposes.
4. Leave the route in a state where the shared `/pitch` proof file is the single browser-first acceptance gate for the deck.

Must-haves:
- The Playwright rail proves the launch-ready narrative and CTA surfaces, not just the S01 shell skeleton.
- Mobile and print behavior are asserted explicitly in the same shared browser file.
- Final acceptance is the real landing build plus the full `/pitch` Chromium replay.

Done when:
- `mesher/landing/tests/pitch-route.spec.ts` is the single authoritative proof file for the finished deck and both acceptance commands pass cleanly.
  - Estimate: 2h
  - Files: mesher/landing/tests/pitch-route.spec.ts, mesher/landing/playwright.config.ts, mesher/landing/lib/pitch/slides.ts, mesher/landing/components/pitch/pitch-deck.tsx, mesher/landing/components/pitch/pitch-controls.tsx, mesher/landing/components/pitch/pitch-slide.tsx, mesher/landing/app/globals.css
  - Verify: npm --prefix mesher/landing run build
./mesher/landing/node_modules/.bin/playwright test --config ./mesher/landing/playwright.config.ts ./mesher/landing/tests/pitch-route.spec.ts --project=chromium
