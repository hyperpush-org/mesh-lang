---
id: T01
parent: S02
milestone: M059
key_files:
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-route-map.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx
  - ../hyperpush-mono/mesher/frontend-exp/app/page.tsx
  - ../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx
  - ../hyperpush-mono/mesher/frontend-exp/playwright.config.ts
  - ../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts
  - ../hyperpush-mono/mesher/frontend-exp/package.json
  - ../hyperpush-mono/mesher/frontend-exp/package-lock.json
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D498 — Move the active dashboard runtime into a shared client shell plus route-map helper and have TanStack render that shell directly instead of importing the legacy monolithic page.
duration: 
verification_result: passed
completed_at: 2026-04-11T16:05:29.729Z
blocker_discovered: false
---

# T01: Extracted the active dashboard shell into shared TanStack runtime modules and added repo-owned Playwright parity tests for the `/` Issues view.

**Extracted the active dashboard shell into shared TanStack runtime modules and added repo-owned Playwright parity tests for the `/` Issues view.**

## What Happened

I moved the active dashboard runtime off the temporary `app/page.tsx` monolith by extracting its client-state shell into `components/dashboard/dashboard-shell.tsx` and introducing `components/dashboard/dashboard-route-map.ts` for canonical top-level route keys, pathnames, and titles with an explicit unknown-key fallback to Issues. `src/routes/index.tsx` now renders that shared shell directly, while `app/page.tsx` was reduced to a thin wrapper so the TanStack runtime no longer depends on legacy page implementation details. I added stable shell/sidebar/panel test hooks on the active runtime path, typed the sidebar against shared route keys, and then added package-local Playwright support with a managed dev `webServer` plus a real parity suite for the current `/` Issues experience. The suite now proves Issues-root landmarks, active-nav state, sidebar collapse persistence through AI-panel open/close, repeated AI toggles, small-screen auto-collapse, and zero console/request noise, all while keeping the slice entirely client-only and mock-data-only.

## Verification

Ran `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` successfully after the shell extraction and Playwright harness landed. Ran `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec -- playwright test ../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts --config ../hyperpush-mono/mesher/frontend-exp/playwright.config.ts --project=dev --grep "issues shell"` successfully; both `issues shell` tests passed and explicitly verified the root Issues shell, active-nav state, sidebar collapse behavior, AI-panel open/close behavior, repeated AI toggles, small-screen auto-collapse, and zero console/request noise.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` | 0 | ✅ pass | 4496ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp exec -- playwright test ../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts --config ../hyperpush-mono/mesher/frontend-exp/playwright.config.ts --project=dev --grep "issues shell"` | 0 | ✅ pass | 41800ms |

## Deviations

The task plan’s npm-style Playwright invocation needed one local adaptation because auto-mode runs from the `mesh-lang` repo root, not from `../hyperpush-mono/mesher/frontend-exp/`. I ran Playwright with `npm exec --` plus an explicit `--config ../hyperpush-mono/mesher/frontend-exp/playwright.config.ts` path so npm did not swallow Playwright flags and Playwright loaded the package-local config/projects truthfully from the sibling app.

## Known Issues

The new Playwright harness is intentionally dev-only in this task. Slice-level production parity (`--project=prod` / built-server deep-link proof) is still pending later S02 work.

## Files Created/Modified

- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-shell.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/dashboard-route-map.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.tsx`
- `../hyperpush-mono/mesher/frontend-exp/app/page.tsx`
- `../hyperpush-mono/mesher/frontend-exp/components/dashboard/sidebar.tsx`
- `../hyperpush-mono/mesher/frontend-exp/playwright.config.ts`
- `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts`
- `../hyperpush-mono/mesher/frontend-exp/package.json`
- `../hyperpush-mono/mesher/frontend-exp/package-lock.json`
- `.gsd/KNOWLEDGE.md`
