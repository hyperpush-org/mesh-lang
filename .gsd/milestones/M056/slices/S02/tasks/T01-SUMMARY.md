---
id: T01
parent: S02
milestone: M056
provides: []
requires: []
affects: []
key_files: ["mesher/landing/lib/pitch/slides.ts", "mesher/landing/components/pitch/pitch-slide-variants.tsx", "mesher/landing/components/pitch/pitch-slide.tsx", "mesher/landing/components/pitch/pitch-deck.tsx", "mesher/landing/tests/pitch-route.spec.ts", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M056/slices/S02/tasks/T01-SUMMARY.md"]
key_decisions: ["D397: model /pitch as validated typed slide variants under a preserved shared shell.", "Keep print-media summary assertions scoped to each slide container so richer variant copy can reuse the same sentence without false route regressions."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the task plan rails and the full slice rail: `npm --prefix mesher/landing run build` passed, the focused `route shell` Playwright replay passed, and the full `pitch-route.spec.ts` replay passed after scoping the print summary assertion per slide. I also exercised the route in a live browser session against a local dev server and confirmed `data-active-slide-id`, `data-active-slide-index`, and the current-frame marker updated truthfully when loading `/pitch#product` and jumping to `#traction-team`."
completed_at: 2026-04-05T05:29:42.139Z
blocker_discovered: false
---

# T01: Rebuilt the `/pitch` deck as a validated six-beat evaluator narrative with explicit slide variants and synced route-shell proof.

> Rebuilt the `/pitch` deck as a validated six-beat evaluator narrative with explicit slide variants and synced route-shell proof.

## What Happened
---
id: T01
parent: S02
milestone: M056
key_files:
  - mesher/landing/lib/pitch/slides.ts
  - mesher/landing/components/pitch/pitch-slide-variants.tsx
  - mesher/landing/components/pitch/pitch-slide.tsx
  - mesher/landing/components/pitch/pitch-deck.tsx
  - mesher/landing/tests/pitch-route.spec.ts
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M056/slices/S02/tasks/T01-SUMMARY.md
key_decisions:
  - D397: model /pitch as validated typed slide variants under a preserved shared shell.
  - Keep print-media summary assertions scoped to each slide container so richer variant copy can reuse the same sentence without false route regressions.
duration: ""
verification_result: passed
completed_at: 2026-04-05T05:29:42.163Z
blocker_discovered: false
---

# T01: Rebuilt the `/pitch` deck as a validated six-beat evaluator narrative with explicit slide variants and synced route-shell proof.

**Rebuilt the `/pitch` deck as a validated six-beat evaluator narrative with explicit slide variants and synced route-shell proof.**

## What Happened

Replaced the old text-only `/pitch` deck with a validated typed slide-variant model that explicitly covers product, workload, Mesh moat, token flywheel, traction/team, and CTA beats. Added `pitch-slide-variants.tsx` for route-local slide-specific interiors, preserved the shared slide shell and existing state markers, refreshed the hero composition in `pitch-deck.tsx`, and updated the Playwright route-shell proof to match the new narrative plus malformed-model negative cases. During verification, the full print-media replay initially failed because a richer variant repeated the same summary sentence as the slide intro; I fixed that by scoping the print assertion to each `[data-slide-id]` container instead of weakening the deck copy.

## Verification

Verified the task plan rails and the full slice rail: `npm --prefix mesher/landing run build` passed, the focused `route shell` Playwright replay passed, and the full `pitch-route.spec.ts` replay passed after scoping the print summary assertion per slide. I also exercised the route in a live browser session against a local dev server and confirmed `data-active-slide-id`, `data-active-slide-index`, and the current-frame marker updated truthfully when loading `/pitch#product` and jumping to `#traction-team`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix mesher/landing run build` | 0 | ✅ pass | 25500ms |
| 2 | `./mesher/landing/node_modules/.bin/playwright test --config ./mesher/landing/playwright.config.ts ./mesher/landing/tests/pitch-route.spec.ts --project=chromium --grep "route shell"` | 0 | ✅ pass | 11600ms |
| 3 | `./mesher/landing/node_modules/.bin/playwright test --config ./mesher/landing/playwright.config.ts ./mesher/landing/tests/pitch-route.spec.ts --project=chromium` | 0 | ✅ pass | 27100ms |


## Deviations

None.

## Known Issues

Package-wide `./mesher/landing/node_modules/.bin/tsc --noEmit -p mesher/landing/tsconfig.json` still reports pre-existing landing errors outside this task (blog editor command typing and nullability issues in existing canvas-heavy landing components). The touched pitch files were filtered clean, but the package-wide typecheck is not fully green yet.

## Files Created/Modified

- `mesher/landing/lib/pitch/slides.ts`
- `mesher/landing/components/pitch/pitch-slide-variants.tsx`
- `mesher/landing/components/pitch/pitch-slide.tsx`
- `mesher/landing/components/pitch/pitch-deck.tsx`
- `mesher/landing/tests/pitch-route.spec.ts`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M056/slices/S02/tasks/T01-SUMMARY.md`


## Deviations
None.

## Known Issues
Package-wide `./mesher/landing/node_modules/.bin/tsc --noEmit -p mesher/landing/tsconfig.json` still reports pre-existing landing errors outside this task (blog editor command typing and nullability issues in existing canvas-heavy landing components). The touched pitch files were filtered clean, but the package-wide typecheck is not fully green yet.
