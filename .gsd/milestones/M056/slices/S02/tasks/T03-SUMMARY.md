---
id: T03
parent: S02
milestone: M056
provides: []
requires: []
affects: []
key_files: ["mesher/landing/tests/pitch-route.spec.ts", "mesher/landing/components/pitch/pitch-controls.tsx", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M056/slices/S02/tasks/T03-SUMMARY.md"]
key_decisions: ["Keep the richer /pitch closeout on the existing route-local Playwright file instead of introducing a second browser harness.", "Fix the mobile control-rail width bug in pitch-controls.tsx with min-w-0 layout constraints rather than weakening the responsive proof."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the slice acceptance commands from the task plan: npm --prefix mesher/landing run build passed, and ./mesher/landing/node_modules/.bin/playwright test --config ./mesher/landing/playwright.config.ts ./mesher/landing/tests/pitch-route.spec.ts --project=chromium passed with 12 tests. I also exercised the real /pitch#cta route in a live mobile browser session on a local landing dev server and used explicit browser assertions to confirm the control rail, CTA slide, waitlist button, and canonical public CTA cards were visible."
completed_at: 2026-04-05T05:54:11.596Z
blocker_discovered: false
---

# T03: Hardened the /pitch browser rail for launch-ready CTA, mobile, and print behavior and fixed the mobile control-rail width bug.

> Hardened the /pitch browser rail for launch-ready CTA, mobile, and print behavior and fixed the mobile control-rail width bug.

## What Happened
---
id: T03
parent: S02
milestone: M056
key_files:
  - mesher/landing/tests/pitch-route.spec.ts
  - mesher/landing/components/pitch/pitch-controls.tsx
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M056/slices/S02/tasks/T03-SUMMARY.md
key_decisions:
  - Keep the richer /pitch closeout on the existing route-local Playwright file instead of introducing a second browser harness.
  - Fix the mobile control-rail width bug in pitch-controls.tsx with min-w-0 layout constraints rather than weakening the responsive proof.
duration: ""
verification_result: passed
completed_at: 2026-04-05T05:54:11.597Z
blocker_discovered: false
---

# T03: Hardened the /pitch browser rail for launch-ready CTA, mobile, and print behavior and fixed the mobile control-rail width bug.

**Hardened the /pitch browser rail for launch-ready CTA, mobile, and print behavior and fixed the mobile control-rail width bug.**

## What Happened

Expanded mesher/landing/tests/pitch-route.spec.ts into the single closeout rail for the finished deck. The shared spec now proves ordered slide titles and agenda labels from pitchDeck, the final CTA slide’s real waitlist/public-link surfaces, an unsupported-print failure path, mobile-width behavior for the control rail and CTA actions, and the richer print/export contract. During replay, the new mobile assertion exposed a real route-local issue: the scrollable outline in PitchControls was forcing the whole control rail to max-content width on narrow viewports. I fixed that by constraining the outer aside and first card wrapper with min-w-0 in mesher/landing/components/pitch/pitch-controls.tsx, then reran the authoritative acceptance commands until both the build and Chromium rail passed cleanly. I also recorded the layout trap in .gsd/KNOWLEDGE.md for future /pitch work.

## Verification

Verified the slice acceptance commands from the task plan: npm --prefix mesher/landing run build passed, and ./mesher/landing/node_modules/.bin/playwright test --config ./mesher/landing/playwright.config.ts ./mesher/landing/tests/pitch-route.spec.ts --project=chromium passed with 12 tests. I also exercised the real /pitch#cta route in a live mobile browser session on a local landing dev server and used explicit browser assertions to confirm the control rail, CTA slide, waitlist button, and canonical public CTA cards were visible.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix mesher/landing run build` | 0 | ✅ pass | 28200ms |
| 2 | `./mesher/landing/node_modules/.bin/playwright test --config ./mesher/landing/playwright.config.ts ./mesher/landing/tests/pitch-route.spec.ts --project=chromium` | 0 | ✅ pass | 39300ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `mesher/landing/tests/pitch-route.spec.ts`
- `mesher/landing/components/pitch/pitch-controls.tsx`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M056/slices/S02/tasks/T03-SUMMARY.md`


## Deviations
None.

## Known Issues
None.
