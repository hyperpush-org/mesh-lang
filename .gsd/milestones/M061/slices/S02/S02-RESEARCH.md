# M061/S02 — Research

**Date:** 2026-04-12

## Summary

S02 primarily advances **R168** and supports **R170/R171** by taking the existing canonical maintainer doc at `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` from top-level route labels to a fail-closed mixed-surface map for `issues`, `alerts`, and `settings`. The product code already exposes enough stable seams to do this without a redesign: `settings-page.tsx` has explicit per-tab support labels plus `MockOnlyWrap` banners and granular `data-testid`s; `issues-page.tsx` / `issue-detail.tsx` expose list/detail/action source markers and a clearly separated proof harness; `alerts-page.tsx` / `alert-detail.tsx` expose overview/detail/action source markers and explicit shell-only buttons.

The main gap is not missing UI evidence — it is missing **machine-checked structure**. S01’s verifier only parses the `## Top-level inventory` table. The current `## mixed-route breakdown` section is prose-only bullets, so S02’s highest-leverage work is to convert that section into structured per-route tables and extend `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` plus `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` to fail closed on drift. Existing Playwright proof already covers most live boundaries; only a few shell-only controls may need extra assertions if the doc wants proof anchors at control granularity instead of code-only anchors.

## Recommendation

Keep `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` as the single canonical doc and expand its existing `## mixed-route breakdown` section into **three structured tables**: `Issues`, `Alerts`, and `Settings`. Do **not** introduce a second runtime registry. Instead, extend the existing markdown parser/test path so the verifier checks both the top-level route table and the new mixed-surface rows.

This matches the current codebase seams and follows two already-loaded skill rules:
- From **react-best-practices**: keep changes local to the stable seam instead of inventing a new abstraction/registry. Here, the stable seam is the existing maintainer doc + parser/test.
- From **playwright-best-practices**: reuse resilient selectors and explicit assertions rather than adding sleep-based or screenshot-first proof. The repo already has durable `data-testid` anchors and source/state attributes for most mixed surfaces.

The lightest path is:
1. Reshape the mixed-route doc into tables with explicit classifications (`live`, `mixed`, `mock-only`, or `shell-only` where the row is a control rather than a route).
2. Extend `client-route-inventory.mjs` with a parser for those tables.
3. Extend `verify-client-route-inventory.test.mjs` to assert expected rows and evidence cells.
4. Add only the minimum Playwright assertions needed to cover any new rows that currently have code evidence but no runtime proof evidence.

## Implementation Landscape

### Key Files

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` — Canonical maintainer doc. Today it has a strict top-level table plus prose-only mixed-route bullets. S02 should replace the prose bullets with structured mixed-surface tables for `issues`, `alerts`, and `settings`.
- `../hyperpush-mono/mesher/scripts/lib/client-route-inventory.mjs` — Existing parser for the top-level inventory only. Natural place to add `EXPECTED_MIXED_ROUTE_KEYS`, allowed mixed-surface classifications, and table parsers for route/panel/subsection/control rows.
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` — Existing fail-closed contract. Extend it so mixed-route rows are required, duplicates fail, evidence cells cannot be blank, and every mixed top-level route (`issues`, `alerts`, `settings`) has the expected fine-grained coverage.
- `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` — Existing retained wrapper. Likely no logic change required if the node:test contract is extended in place; the wrapper already runs that contract first.
- `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx` — Best existing evidence surface for fine-grained breakdown. `tabSupportLabel()` already encodes `general => mixed live`, `team/api-keys/alerts => live`, everything else `mock-only`. `MockOnlyWrap` emits stable `*-banner` markers. Live controls and error states already have explicit `data-testid`s.
- `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-live-state.tsx` — Truth source for Settings live sections and same-origin endpoints. `general.source` is intentionally `mixed`; `team`, `apiKeys`, and `alertRules` become `live`; failures fall back explicitly. Good place to anchor backend seam notes.
- `../hyperpush-mono/mesher/client/components/dashboard/issues-page.tsx` — Issues shell state and the retained `issue-action-proof-harness`. Important distinction: the proof harness is diagnostic chrome, not a supported maintainer action.
- `../hyperpush-mono/mesher/client/components/dashboard/issue-detail.tsx` — Exact live vs shell-only control split for Issues. `Resolve/Reopen/Ignore` buttons are `source="same-origin-live"`; `AI Analysis`, `Link Issue`, and bounty chrome are `source="shell-only"`; the banner and action note already explain the mixed boundary.
- `../hyperpush-mono/mesher/client/components/dashboard/issue-list.tsx` — Row-level mixed markers. Live-backed rows render `data-source="mixed"` plus `live status + fallback shell`; fallback rows stay `fallback`.
- `../hyperpush-mono/mesher/client/components/dashboard/stats-bar.tsx` — Issues summary-card truth surface. Some stat cards are `live`/`derived live`; others intentionally remain `fallback`, which makes this a good subsection-level mixed row in the doc.
- `../hyperpush-mono/mesher/client/components/dashboard/alerts-live-state.tsx` — Alerts bootstrap and action gatekeeping. Only `acknowledge` and `resolve` are supported; unsupported or non-live actions fail closed.
- `../hyperpush-mono/mesher/client/components/dashboard/alert-detail.tsx` — Exact alert control split. `alert-detail-action-acknowledge` / `resolve` are live-backed, `alert-detail-action-silence` is disabled shell-only chrome, and `alert-detail-copy-link` is shell-only.
- `../hyperpush-mono/mesher/client/components/dashboard/alert-list.tsx` — Row-level mixed markers for alerts (`mixed live` badge vs fallback).
- `../hyperpush-mono/mesher/client/components/dashboard/alert-stats.tsx` — Alerts summary-card mixed truth surface; route banner already says unsupported fields stay shell-backed.
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts` — Best proof for Issues live read + mixed detail hydration + explicit fallback behavior.
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts` — Best proof for Issues supported live actions, disabled/pending states, and refresh-failure fallback handling.
- `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts` — Best proof for Alerts live actions and Settings General/Team/API Keys/Alert Rules plus explicit Settings mock-only banners.
- `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts` — Cross-route walkthrough that already asserts many mixed route invariants and every Settings mock-only tab banner.

### Build Order

1. **Define the mixed-surface schema in the doc/parser first.**
   Start by deciding the row shape for the new tables in `ROUTE-INVENTORY.md` (for example: `Surface | Classification | Code evidence | Proof evidence | Live seam | Boundary note`). This is the main contract S02 is delivering.

2. **Fill Settings first.**
   `settings-page.tsx` is the cleanest evidence surface because it already encodes tab support in `tabSupportLabel()`, exposes per-section `data-*` attributes, and uses `MockOnlyWrap` banners with stable test ids. This table shape will likely generalize to Issues and Alerts.

3. **Add Issues and Alerts rows second.**
   For Issues, natural rows are: overview stats/cards, issue list rows, detail hydration/timeline, supported maintainer actions, shell-only detail controls, and the provider proof harness. For Alerts, natural rows are: overview stats/cards, alert rows, detail status/history, supported lifecycle actions, and shell-only controls (`Silence`, `Copy Link`).

4. **Lock the schema with node:test before touching runtime proof.**
   Extend `client-route-inventory.mjs` and `verify-client-route-inventory.test.mjs` so the new rows are required and drift fails closed. This keeps S02 aligned with R170 without changing the wrapper surface.

5. **Only then patch proof gaps.**
   If a documented control has only code evidence today, add the smallest explicit assertion to an existing Playwright suite instead of creating new suites. The likely candidates are shell-only controls like `alert-detail-copy-link` or any Issues shell-only control you choose to claim at control granularity.

### Verification Approach

- Structural contract:
  - `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
- Targeted browser proof from `mesh-lang` using the split-workspace rule established by S01 (explicit client config path, no cwd inference):
  - `env PLAYWRIGHT_PROJECT=dev npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts`
- Optional retained wrapper smoke:
  - `bash ../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`
  - Useful for the full named proof rail and retained logs, but do not confuse the known prod alert seed-read flake from S01 with a new S02 regression unless the touched assertions actually intersect that path.

## Constraints

- The canonical artifact must stay at `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`; do not move this truth surface back into `.gsd/`.
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts` remains the only structural source of truth for top-level routes. S02 should not create a second top-level registry.
- Fine-grained breakdown only needs to cover the mixed routes: `issues`, `alerts`, and `settings`.
- `fallback` and `mock-only` are different truths: fallback means a live seam failed and the shell stayed mounted; mock-only means no live seam is claimed.
- From `mesh-lang`, Playwright must be run with the explicit sibling config path: `--config ../hyperpush-mono/mesher/client/playwright.config.ts`.

## Common Pitfalls

- **Treating fallback as mock-only** — Issues and Alerts often degrade to `fallback` at runtime while still being canonically `mixed`. The doc must describe the permanent support boundary, not transient bootstrap failure.
- **Documenting the Issues proof harness as a supported control** — `issue-action-proof-harness` exists for provider validation/error-path diagnostics and should be called out as proof/diagnostic chrome, not maintainer product functionality.
- **Overstating Settings Team support** — Team is live for list/add/role/remove, but the UI is intentionally honest that adding a member still requires a raw `user_id`; no email invite flow exists.
- **Using the in-app `mixed live` label as the canonical doc vocabulary** — S01 already normalized route-level language to `mixed`. Keep that normalization in the maintainer doc while still referencing the in-app badge text where useful.

## Open Risks

- Control-level rows for shell-only Issues/Alerts chrome may need a small number of new Playwright assertions if S02 wants every row to have proof evidence instead of code evidence alone.
- The retained wrapper’s known prod alert seed-read flake can create noise during verification; prefer the node:test contract plus targeted suites when isolating S02 changes.

## Skills Discovered

| Technology | Skill | Status |
|------------|-------|--------|
| React UI surfaces | `react-best-practices` | available |
| Playwright proof rails | `playwright-best-practices` | available |
