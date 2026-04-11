# S01: Seeded real context and issues/events live read seam — UAT

**Milestone:** M060
**Written:** 2026-04-11T21:12:30.188Z

# UAT — S01 Seeded real context and issues/events live read seam

## Preconditions

1. A local Mesher backend is available on `http://127.0.0.1:18080` with the seeded `default` project.
2. Run the slice verification prep once before manual walkthroughs:
   - `bash mesher/scripts/seed-live-issue.sh`
3. Start either dashboard runtime:
   - Dev: `npm --prefix mesher/client run dev`
   - Built production: `npm --prefix mesher/client run build && npm --prefix mesher/client run start`
4. Open the dashboard root URL for the chosen runtime.

## Test Case 1 — Seeded default context boots through same-origin live reads

1. Navigate to `/`.
   - Expected: the page loads the existing Issues shell without any login/session prompt.
2. Confirm the page shows the `Issues` heading, stats bar, and event chart.
   - Expected: overview surfaces are visible and the shell copy indicates live or mixed data (for example `Live overview active`).
3. Inspect the issue list for the seeded issue titled `M060 seeded live issue read seam`.
   - Expected: the seeded issue row is present in the list and remains styled like the existing shell, with fallback-only shell extras still visible.
4. Verify network behavior.
   - Expected: browser requests stay on same-origin `/api/v1/...` paths only; no direct browser call goes to `:8080` or `:18080`.

## Test Case 2 — Selecting the seeded issue hydrates live detail and timeline

1. Click the `M060 seeded live issue read seam` issue row.
   - Expected: the existing right-side issue detail panel opens.
2. Observe the live-status banner in the detail panel.
   - Expected: it reports `Live event detail + timeline active` and includes the latest event id.
3. Review the detail content.
   - Expected: recent-event timeline content includes the seeded issue title, the stack/file surface shows `seed/live-issue-read.ts`, and unsupported shell fields remain visibly populated by fallback content.
4. Click `Breadcrumbs`.
   - Expected: the seeded breadcrumb message `Seeded live issue read breadcrumb` is visible.
5. Click `Context`.
   - Expected: the seeded context tag `seed_case:m060-live-read-seam` is visible.

## Test Case 3 — Sparse live detail still preserves fallback shell sections

_Precondition_: use the existing Playwright route-override seam or another controlled proxy to force the selected issue's latest-event detail to return a sparse payload (empty `stacktrace`/`breadcrumbs`) while keeping timeline and tags readable.

1. Load `/` and select the seeded issue under the sparse-detail override.
   - Expected: the detail panel still reaches a ready mixed state rather than crashing.
2. Inspect the panel after selection.
   - Expected: the live banner remains active, the recent-event timeline shows the sparse live timeline entry, and a fallback shell stack/file surface such as `src/solana/tx.ts:88` remains visible.
3. Click `Breadcrumbs` and then `Context`.
   - Expected: fallback breadcrumb content still renders, and the sparse live tag/environment values (for example `environment:sparse-env`) are visible.

## Test Case 4 — Selected-issue read failures surface through the existing toast pattern

_Precondition_: use the existing Playwright route-override seam or another controlled proxy to force `/api/v1/issues/<seeded-id>/timeline` to return HTTP 500 while leaving the rest of the page boot path intact._

1. Load `/` and click the seeded issue row.
   - Expected: the detail panel stays open and preserves fallback shell content instead of disappearing.
2. Inspect the detail panel banner.
   - Expected: it reports `Live issue detail unavailable` and indicates the read failed while fallback shell content stayed mounted.
3. Inspect the notification region.
   - Expected: a visible destructive toast appears with the title `Live issue timeline failed`.
4. Confirm the shell remains usable.
   - Expected: `AI Analysis` and the detail close button remain visible and interactive.

## Test Case 5 — Built production runtime matches the dev seam

1. Repeat Test Cases 1, 2, and 4 against the built production runtime (`npm --prefix mesher/client run start`).
   - Expected: seeded live boot, selected-issue hydration, and selected-issue failure toast behavior match the dev runtime with the same same-origin `/api/v1` transport contract.

## Edge Cases to Recheck Before Signoff

1. Unknown live severity/status values on the overview payload.
   - Expected: the shell normalizes them without crashing, keeping stats/chart/list surfaces visible.
2. Bootstrap failure on `/api/v1/projects/default/dashboard/health`.
   - Expected: the shell stays mounted, the overview drops to explicit fallback state, and failure is observable through runtime/request assertions rather than silent empty UI.
