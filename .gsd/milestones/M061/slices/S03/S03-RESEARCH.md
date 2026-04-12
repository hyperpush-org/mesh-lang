# S03 — Research

**Date:** 2026-04-12

## Summary

S03 primarily owns **R169** and directly supports **R170** and **R171**. The missing work is not another route audit; S01/S02 already established the canonical route and mixed-surface truth in `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`. What is still missing is the second half of that maintainer artifact: a **backend gap map** that traces each current client promise to either an existing backend seam, a partial seam with fallback payload holes, or a documented missing seam. `ROUTE-INVENTORY.md` currently has no backend-gap section at all (`rg "Backend gap|gap map" ../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` returns nothing).

The important code fact is that the currently shipped backend surface is much narrower than the dashboard shell. `../hyperpush-mono/mesher/main.mpl` only registers issue/event/dashboard routes, alert/alert-rule routes, settings/storage routes, team-member routes, and API-key routes. There are **no** registered `/api/v1` families for performance, releases, bounties, treasury, or solana-program pages. Inside the mixed routes, the adapters already expose the finer boundary: `issues-live-adapter.ts` still marks `affectedUsers`, `mttr`, `crashFreeSessions`, and `uptime` as fallback-only; `admin-ops-live-adapter.ts` still marks alert MTTA / duration metrics as fallback-only; `settings-page.tsx` keeps project identity, channel config, billing, security, notifications, profile, token, and bounty policy visibly shell-only.

The cleanest implementation path is to keep the backend gap map inside the existing canonical doc and extend the **existing** parser/test rail instead of inventing a second registry or new verifier. That follows the same pattern S02 used for mixed-surface tables. It also lines up with the installed `playwright-best-practices` skill guidance: reuse deterministic existing suites and explicit assertion-based proof rails, rather than introducing new broad UI flows or screenshot-only checks for a documentation slice.

## Recommendation

Add a new **stable table-based backend gap map** to `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`, keyed to the already-canonical route/surface vocabulary wherever possible:

- top-level route keys for wholly mock-only routes (`performance`, `solana-programs`, `releases`, `bounties`, `treasury`)
- existing S02 surface keys for mixed routes (`issues/overview`, `issues/live-actions`, `alerts/overview`, `settings/general`, `settings/alert-channels`, etc.)

Then extend `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` and `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` so the new gap rows are parsed and fail closed on drift. Keep the existing wrappers (`readRouteInventory`, `parseRouteInventoryMarkdown`) stable; add document-level gap-map parsing the same way S02 layered mixed-surface parsing on top of the S01 API.

The gap rows should distinguish at least these realities in prose, even if the parser keeps the cells free-form:

- **existing seam already covers the promise**
- **existing route family but missing payload/aggregate fields**
- **existing route family but missing mutation/control support**
- **no backend route family exists yet**

Do **not** make the retained `verify:route-inventory` wrapper the primary closeout check for S03. S01 already recorded a prod alert flake in that wrapper. For this slice, the primary verification should stay the structural `node:test` contract, with the existing dev Playwright subset reused only when a doc claim about mixed live behavior materially changes.

## Implementation Landscape

### Key Files

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` — canonical maintainer document; currently has top-level inventory + mixed-route breakdown + invariants, but no backend-gap section yet. This is the artifact S03 should extend.
- `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` — document parser. `parseRouteInventoryDocument()` already returns `topLevelRows`, `mixedSurfaceSections`, and `mixedSurfaceRows`; this is the natural place to add parsed backend-gap sections while keeping `readRouteInventory()` stable for top-level callers.
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` — fail-closed contract test. It already locks exact top-level row order and mixed-surface row order via `expectedMixedSurfaceRowsBySection` + `assertMixedSurfaceContract()`. S03 should add the backend-gap expectations here rather than creating a second doc checker.
- `../hyperpush-mono/mesher/main.mpl` — authoritative registry of what backend HTTP seams actually exist today. It proves that the live surface is limited to issues/events/dashboard aggregates, alerts/alert-rules, settings/storage, team membership, and API keys.
- `../hyperpush-mono/mesher/client/lib/mesher-api.ts` — client-side same-origin contract. This is the quickest inventory of what the frontend can currently call without re-reading every component.
- `../hyperpush-mono/mesher/client/lib/issues-live-adapter.ts` — exact issue-side payload boundary. The stats builder still marks `affectedUsers`, `mttr`, `crashFreeSessions`, and `uptime` as fallback, even though counts/open issues are live or derived-live.
- `../hyperpush-mono/mesher/client/lib/admin-ops-live-adapter.ts` — exact alert/settings payload boundary. Alert stats still fallback MTTA/duration fields; `adaptLiveAlert()` overlays live rows onto fallback templates, meaning some visible metadata/history/channel fields are still shell-backed even when status/actions are live.
- `../hyperpush-mono/mesher/client/components/dashboard/issue-detail.tsx` — documents the current issue promise boundary: live `resolve`/`unresolve`/`archive`; shell-only `AI Analysis`, `Link Issue`, and bounty chrome.
- `../hyperpush-mono/mesher/client/components/dashboard/alert-detail.tsx` — documents the alert promise boundary: live `acknowledge`/`resolve`; shell-only `Silence`/`Unsnooze`, `Copy Link`, notification-channel display, linked issue, assignee, and silence metadata.
- `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx` — most important S03 source for partial seams and missing seams. General mixes live retention/storage with mock-only project identity controls; team/API keys/alert rules are live; alert channels and the remaining tabs are explicitly non-live.
- `../hyperpush-mono/mesher/client/components/dashboard/performance-page.tsx` — pure mock route over `MOCK_TRANSACTIONS` / `MOCK_PERF_STATS`; useful for naming the missing backend route family for perf summaries, charts, and transaction detail.
- `../hyperpush-mono/mesher/client/components/dashboard/solana-programs-page.tsx` — pure mock route over `MOCK_PROGRAM*`, `MOCK_INSTRUCTIONS`, and `MOCK_LOG_EVENTS`; useful for naming missing Solana log/program seams.
- `../hyperpush-mono/mesher/client/components/dashboard/releases-page.tsx` + `release-detail.tsx` — pure mock release shell that visibly promises release list/detail plus actions like `Rollback`, `View Diff`, `Verify Contract`, and AI analysis.
- `../hyperpush-mono/mesher/client/components/dashboard/bounties-page.tsx` + `bounty-detail.tsx` — pure mock bounty claims shell with visible review / approve / payout / dispute actions.
- `../hyperpush-mono/mesher/client/components/dashboard/treasury-page.tsx` + `transaction-detail.tsx` — pure mock treasury shell over treasury stats, allocations, payout list, and transaction drill-down.
- `../hyperpush-mono/mesher/api/alerts.mpl`, `settings.mpl`, `team.mpl`, `search.mpl`, `detail.mpl`, `../hyperpush-mono/mesher/ingestion/routes.mpl` — concrete backend handlers the gap map should cite for existing coverage and for “route family exists but client promise outruns payload” cases.

### Build Order

1. **Define the gap-map row model from real code, not prose.**
   - Start from `main.mpl` + `mesher-api.ts` to mark which route families exist.
   - Use `issues-live-adapter.ts`, `admin-ops-live-adapter.ts`, `issue-detail.tsx`, `alert-detail.tsx`, and `settings-page.tsx` to mark which visible fields/actions still fall back.
   - This prevents the planner from confusing “visible in UI” with “backed by route”.

2. **Extend the parser/test contract before filling the doc.**
   - Add backend-gap parsing to `parseRouteInventoryDocument()`.
   - Add explicit expected row order/keys to `verify-client-route-inventory.test.mjs`.
   - This makes the doc fail closed while it is being authored.

3. **Populate `ROUTE-INVENTORY.md` with stable backend-gap tables.**
   - Reuse existing surface keys where S02 already created them.
   - For wholly mock-only routes, keep the rows at route or major-subsection granularity; do not explode every control into its own gap row.

4. **Only after the doc/test contract is in place, tighten proof references if needed.**
   - Existing proofs already cover Issues / Alerts / Settings.
   - For mock-only routes, structural code anchors are probably enough; no new Playwright coverage is obviously required for S03.

### Verification Approach

Primary verification:

- `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`

Use targeted browser proof only if the gap-map rows add or change claims about existing mixed live behavior:

- `env PLAYWRIGHT_PROJECT=dev npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts`

I reran the structural contract during research and it is currently green.

## Constraints

- S03 must satisfy **R169** without implementing backend routes; the slice is documentation + mapping only.
- The canonical artifact must remain `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`, not a new `.gsd`-only doc.
- `readRouteInventory()` / `parseRouteInventoryMarkdown()` already serve top-level callers; extending the parser must not break those wrappers.
- The split workspace still matters: commands run from `mesh-lang`, but all owning client/backend files live under `../hyperpush-mono/mesher/`.
- The full retained wrapper (`npm --prefix ../hyperpush-mono/mesher/client run verify:route-inventory`) is not the right closeout gate for S03 because S01 recorded a remaining prod alert-seed flake there.

## Common Pitfalls

- **Confusing fallback-populated fields with live backend coverage** — `adaptLiveIssue()` and `adaptLiveAlert()` both overlay live rows onto fallback templates. If a detail panel shows data, that does not automatically mean the backend returns that field today.
- **Over-granular gap rows for wholly mock routes** — the fine-grained S02 pattern is necessary for mixed pages, but mock-only routes like Performance or Treasury should stay route/subsection scoped or the document will become unmaintainable.
- **Breaking stable parser callers while adding doc parsing** — follow the S02 layering pattern in `client-route-inventory.mjs`; add document-level backend-gap helpers, but keep the older top-level reader behavior intact.
- **Treating the wrapper as the required verifier** — for this slice, prefer the deterministic structural `node:test` contract and only rerun the dev Playwright subset if a live-boundary claim changes.

## Open Risks

- The biggest ambiguity is not route existence; it is **partial payload truth** inside already-live families. Issues and alerts both keep fallback-derived summary/detail fields, so the gap map needs to distinguish “missing backend route” from “missing aggregate/payload enrichment”.
- Mock-only pages surface many plausible future actions (`Rollback`, `Process Payout`, `Connect`, `Configure`, etc.). The gap map should document those promises, but only at the level backend maintainers can realistically sequence later.

## Skills Discovered

| Technology | Skill | Status |
|------------|-------|--------|
| Playwright verification | `playwright-best-practices` | available |
| Mesh / MPL | none found via `npx skills find "Mesh language"` | none found |
