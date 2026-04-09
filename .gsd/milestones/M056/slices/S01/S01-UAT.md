# S01: Pitch route foundation, navigation, and export — UAT

**Milestone:** M056
**Written:** 2026-04-05T05:01:47.528Z

# S01: Pitch route foundation, navigation, and export — UAT

**Milestone:** M056
**Written:** 2026-04-05

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: This slice is a browser-first route. The meaningful proof is a live `/pitch` session plus browser print-preview behavior, backed by the repo-owned Playwright rails that already exercise route shell, navigation, and export state.

## Preconditions

- Install landing dependencies (`npm --prefix mesher/landing install` if needed).
- Start the landing app locally: `npm --prefix mesher/landing run dev -- --hostname 127.0.0.1 --port 3100`.
- Open a desktop browser at `http://127.0.0.1:3100/pitch`.
- Use a browser that supports the normal print dialog / Save as PDF flow.

## Smoke Test

Open `/pitch` directly in a fresh tab. Confirm the page title includes `Pitch Deck — hyperpush on Mesh`, the hero heading reads `hyperpush sells the workflow. Mesh makes the story durable.`, and the route shows `Current frame · 01 / 06` with the previous button disabled.

## Test Cases

### 1. Direct route load shows the pitch shell instead of falling back to generic landing content

1. Navigate directly to `http://127.0.0.1:3100/pitch` with no prior app state.
2. Confirm the URL normalizes to `/pitch#wedge`.
3. Confirm the hero heading, current-frame marker, and six agenda items are visible.
4. Confirm the first agenda item is marked current and the first slide summary references the open-source error-tracking wedge.
5. **Expected:** `/pitch` renders as a dedicated landing route with route-local title/description, the first slide active, and no missing-shell fallback to `/`.

### 2. Keyboard navigation stays bounded and inspectable

1. With `/pitch#wedge` open, press `ArrowLeft`.
2. Confirm the URL stays `#wedge` and the previous button remains disabled.
3. Press `ArrowRight` once.
4. Confirm the URL changes to `#burst-load`, the current-frame marker changes to `02 / 06`, and the second agenda item becomes current.
5. Keep pressing `ArrowRight` until the last slide is reached.
6. Confirm the URL ends at `#platform`, the current-frame marker reads `06 / 06`, and the next button is disabled.
7. **Expected:** Keyboard navigation never moves before the first slide or past the last slide, and the URL/hash plus visible controls always match the active slide.

### 3. Deep links, indicator jumps, and wheel/scroll all converge on the same slide state

1. Open `http://127.0.0.1:3100/pitch#mesh-moat` in a fresh tab.
2. Confirm slide 03 is active, the current-frame marker reads `03 / 06`, and the third agenda item is current.
3. Replace the hash with `#not-a-real-slide` and reload.
4. Confirm the route repairs back to `#wedge` instead of showing a blank or broken deck.
5. Return to `/pitch`, click the agenda item for slide 05 (`Open source turns the tool into a distribution channel instead of a gated funnel.`).
6. Confirm the URL becomes `#distribution` and the current-frame marker reads `05 / 06`.
7. From the main deck area, use a mouse wheel / trackpad scroll gesture to advance one slide, then wait briefly and scroll again.
8. **Expected:** Deep links, stale hashes, agenda clicks, and wheel/scroll all drive the same active-slide state, and the route keeps that state visible through the hash and current agenda marker.

### 4. Export uses the browser print path and keeps the deck readable in print preview

1. Navigate to slide 05 (`#distribution`) using the agenda controls.
2. Confirm the export button reads `Print / Save as PDF` once hydration completes.
3. Click the export button.
4. Confirm the browser’s native print dialog / print preview opens instead of a custom download, modal, or backend-generated PDF flow.
5. In print preview, confirm the landing header/footer and deck controls are absent.
6. Confirm each slide title still appears in order and the slide content is stacked vertically as a readable document.
7. Close print preview and confirm the route remains usable on `/pitch`.
8. **Expected:** Export is browser-native, print preview hides interactive chrome, and the same slide content remains readable as a stacked printable deck.

### 5. Export control is not armed before hydration

1. Hard-refresh `/pitch` and watch the export control as the page hydrates.
2. Confirm the button initially shows `Preparing export` and is disabled.
3. Wait for hydration to complete.
4. Confirm the button changes to `Print / Save as PDF` and becomes enabled.
5. **Expected:** Server render stays inert; the export path only arms after hydration and never throws on first paint.

## Edge Cases

### Unknown hash fallback

1. Open `/pitch#definitely-not-real` directly.
2. **Expected:** The route repairs to the first valid slide (`#wedge`) and still shows a usable deck.

### Boundary controls

1. On slide 01, confirm `Previous` is disabled.
2. On slide 06, confirm `Next` is disabled.
3. **Expected:** Boundary controls stay deterministic; the deck never moves to an invalid frame.

### Middle-slide export

1. Jump to slide 05 with the agenda controls.
2. Trigger print export from that state.
3. **Expected:** Print preview still shows the full deck content in order, not a clipped single-slide viewport or a broken mid-deck partial export.

## Failure Signals

- The page title stays on the generic landing title instead of the pitch-specific title.
- `/pitch` loads without a hash/current-frame marker or without six agenda items.
- Indicator clicks, wheel/scroll, and keyboard navigation move different slides or leave the hash/agenda state out of sync.
- The export button never arms after hydration, throws an error, or opens anything other than the native browser print path.
- Print preview still shows the landing header/footer or deck controls, or printed content is clipped/missing.

## Requirements Proved By This UAT

- R120 — The landing surface now has a dedicated evaluator-facing pitch route that keeps the hyperpush story visibly tied to Mesh-backed systems behavior.

## Not Proven By This UAT

- Final visual polish, animation quality, and launch-ready presentation finish work planned for S02.
- Cross-browser PDF parity across every browser/OS combination; this UAT proves the browser-native print path, not every downstream PDF renderer.
- Production monitoring/analytics for `/pitch`; this slice only proves route-local DOM state and repo-owned browser regressions.

## Notes for Tester

- If you replay the repo-owned browser suite from the shell, use the package-local Playwright binary with explicit config: `./mesher/landing/node_modules/.bin/playwright test --config ./mesher/landing/playwright.config.ts ./mesher/landing/tests/pitch-route.spec.ts --project=chromium`. On this host, the raw `npm --prefix mesher/landing exec playwright ...` form drops `--project` / `--grep` and is not a truthful replay surface.
- The deck foundation is intentionally structural in S01. If the route behavior is correct but the visuals still feel rough, that is expected closeout state for this slice rather than a regression by itself.
