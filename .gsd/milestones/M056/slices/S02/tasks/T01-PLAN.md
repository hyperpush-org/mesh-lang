---
estimated_steps: 4
estimated_files: 5
skills_used:
  - frontend-design
  - react-best-practices
---

# T01: Recast the pitch narrative into slide-specific evaluator variants

**Slice:** S02 — Narrative polish, slide visuals, and launch-ready closeout
**Milestone:** M056

## Description

Lock the final evaluator-facing story first. Expand `pitchDeck` into a launch-ready arc with explicit product, Mesh/runtime moat, token-flywheel, traction/team, and CTA beats, then introduce a pitch-local slide-variant rendering seam so each frame can render richer interiors without breaking stable slide ids, ordered DOM sections, or the S01 state markers. Update the existing route-shell assertions in the shared Playwright file so the narrative source and the public route stay in sync.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `mesher/landing/lib/pitch/slides.ts` slide model | Fail the route-shell proof on stale copy/count drift instead of leaving the route and source out of sync. | N/A | Treat duplicate ids, empty variant payloads, or missing narrative sections as contract failures. |
| Pitch-local slide renderer wiring | Fail the build if the shared shell loses a required variant branch or prop. | N/A | Treat unsupported variant/data combinations as explicit implementation bugs, not silent fallbacks to a generic card. |
| Existing `/pitch` route-shell assertions | Update the shared spec in the same task so stale story markers do not make the new deck look broken. | Stop on the first failing route-shell replay. | Treat mismatched titles/counts/markers as public-route drift, not test flake. |

## Load Profile

- **Shared resources**: ordered slide DOM, active-slide marker attributes, and route-shell browser assertions.
- **Per-operation cost**: one route render plus a focused route-shell browser replay.
- **10x breakpoint**: narrative drift and variant mismatch break before runtime cost does, so the task must keep the renderer/data contract explicit.

## Negative Tests

- **Malformed inputs**: duplicate slide ids, missing product/Mesh/CTA beats, or incomplete visual payloads.
- **Error paths**: a variant component not matching the declared slide type, or route-shell assertions still expecting the old six-card copy.
- **Boundary conditions**: the first slide, the final CTA slide, and every intermediate slide still preserve heading ids and stable DOM order.

## Steps

1. Extend `mesher/landing/lib/pitch/slides.ts` with the metadata and visual payload needed for launch-ready product, Mesh, economics, traction/team, and CTA frames.
2. Keep `mesher/landing/components/pitch/pitch-slide.tsx` as the shared shell, but move slide-specific interiors into `mesher/landing/components/pitch/pitch-slide-variants.tsx`.
3. Preserve `slide.id`, ordered section rendering, heading hooks, and the existing `data-testid` / `data-*` state markers so navigation, hash sync, and export remain truthful.
4. Refresh the existing route-shell assertions in `mesher/landing/tests/pitch-route.spec.ts` for the new slide titles/count and evaluator-facing story markers.

## Must-Haves

- [ ] The deck story explicitly covers product, Mesh, token-flywheel, traction/team, and CTA beats.
- [ ] Slide-specific rendering stays route-local and does not import whole homepage sections as-is.
- [ ] The route-shell proof follows the updated deck source instead of stale six-slide text-only copy.

## Verification

- `npm --prefix mesher/landing run build`
- `./mesher/landing/node_modules/.bin/playwright test --config ./mesher/landing/playwright.config.ts ./mesher/landing/tests/pitch-route.spec.ts --project=chromium --grep "route shell"`

## Inputs

- `mesher/landing/lib/pitch/slides.ts` — current six-slide source model and validation rules.
- `mesher/landing/components/pitch/pitch-slide.tsx` — existing shared slide shell that must keep stable markers.
- `mesher/landing/components/pitch/pitch-deck.tsx` — current deck composition boundary and hero/story framing.
- `mesher/landing/tests/pitch-route.spec.ts` — existing route-shell assertions that must be kept in sync with the deck.
- `mesher/landing/components/landing/flywheel.tsx` — existing token-flywheel visual vocabulary.
- `mesher/landing/components/landing/infrastructure.tsx` — existing product-vs-Mesh layout vocabulary.
- `mesher/landing/components/landing/mesh-dataflow.tsx` — existing Mesh/failover visual vocabulary to borrow from, not paste wholesale.

## Expected Output

- `mesher/landing/lib/pitch/slides.ts` — launch-ready slide narrative plus variant payloads.
- `mesher/landing/components/pitch/pitch-slide.tsx` — preserved shared shell with variant composition seam.
- `mesher/landing/components/pitch/pitch-slide-variants.tsx` — new pitch-local slide-specific interiors.
- `mesher/landing/components/pitch/pitch-deck.tsx` — updated deck composition for the richer narrative.
- `mesher/landing/tests/pitch-route.spec.ts` — route-shell expectations aligned with the new story.
