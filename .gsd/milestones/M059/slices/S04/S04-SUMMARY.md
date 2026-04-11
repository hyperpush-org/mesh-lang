---
id: S04
parent: M059
milestone: M059
provides:
  - Final assembly-level equivalence proof for the TanStack dashboard migration.
  - Canonical maintainer guidance that points dashboard work at `mesher/client` instead of `frontend-exp`.
  - A clean runtime parity rail that now covers the last high-signal behaviors needed for milestone validation.
requires:
  - slice: S02
    provides: The pathless `_dashboard` layout, route-key/testid seams, and Issues state observability that the final parity rail relies on.
  - slice: S03
    provides: The canonical `mesher/client` package move, package-local build/start contract, and root Playwright harness targeting the moved package.
affects:
  []
key_files:
  - ../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts
  - ../hyperpush-mono/mesher/client/components/dashboard/events-chart.tsx
  - ../hyperpush-mono/AGENTS.md
  - ../hyperpush-mono/CONTRIBUTING.md
  - ../hyperpush-mono/SUPPORT.md
  - ../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml
  - ../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml
  - ../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml
  - ./AGENTS.md
  - ./playwright.config.ts
key_decisions:
  - Keep the final proof inside the canonical `dashboard-route-parity.spec.ts` rail and fix surfaced runtime regressions in app code instead of weakening runtime-signal assertions.
  - Limit direct-operational cleanup to the selected maintainer-facing files and prove closure with paired stale-path/positive-path greps plus the existing `mesher/client` build/dev/prod/root-harness rails.
patterns_established:
  - Use one canonical browser parity rail with explicit `data-testid` state assertions and shared console/failed-request tracking rather than inventing a second migration smoke harness.
  - When the parity rail surfaces clean-boot console noise, fix the runtime mount path (here, Recharts animation) instead of downgrading the assertion contract.
  - For path migrations, close maintainer drift with a paired negative/positive grep over the exact operational surfaces rather than broad repo-wide string replacement.
  - Keep cross-repo Playwright verification rooted in `mesh-lang` but use the moved package's Playwright binary and project-selection contract for truthful `--list` checks.
observability_surfaces:
  - `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts` runtime-signal tracker for console errors and failed requests.
  - Dashboard shell `data-testid` / route-key / Issues-state attributes exercised by the parity suite.
  - Scoped `rg` checks over the selected maintainer docs/templates to catch stale `frontend-exp` references and confirm `mesher/client` replacements.
  - `./playwright.config.ts` plus `PLAYWRIGHT_PROJECT=dev ... --list` for root-harness resolution against the moved package.
drill_down_paths:
  - .gsd/milestones/M059/slices/S04/tasks/T01-SUMMARY.md
  - .gsd/milestones/M059/slices/S04/tasks/T02-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-11T19:12:49.413Z
blocker_discovered: false
---

# S04: Equivalence proof and direct operational cleanup

**Closed the TanStack dashboard migration by extending the canonical `mesher/client` parity rail for Solana AI/sidebar and Issues browser-history behavior, removing the last direct `frontend-exp` maintainer references, and re-verifying the build/dev/prod/root-harness contract from `mesh-lang`.**

## What Happened

## Delivered

S04 closed the remaining migration risk without widening the architecture. The canonical browser proof stayed inside `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`: it now proves that the Solana Programs route auto-collapses the sidebar when AI opens and restores it when AI closes, and that Issues search/filter/detail state survives real browser back/forward traversal after leaving the page. The parity rail continued to assert route key, pathname, active sidebar state, direct-entry behavior, and clean runtime signals in both dev and built production.

While strengthening that rail, the clean-runtime tracker surfaced a real regression on initial Issues boot: Recharts emitted a console warning from the events chart path. The slice fixed the runtime code instead of weakening the assertions by disabling Area animations in `../hyperpush-mono/mesher/client/components/dashboard/events-chart.tsx`. After that repair, all targeted and full dev/prod parity runs passed cleanly again with no console errors or failed requests.

The second half of the slice closed operational drift. The remaining direct maintainer-facing guidance in `../hyperpush-mono/AGENTS.md`, `../hyperpush-mono/CONTRIBUTING.md`, `../hyperpush-mono/SUPPORT.md`, the three product issue templates, and `./AGENTS.md` now point dashboard work at `mesher/client` while still preserving `mesher/landing` as the intentional Next.js app. Scoped negative and positive greps proved those selected operational surfaces no longer carry stale `frontend-exp` guidance, and the root Playwright harness still resolves the moved package path through `./playwright.config.ts`.

This slice stayed on the existing mock-data/client-state contract. It did not introduce TanStack loaders, server functions, backend calls, auth, or widened URL/search-param semantics. The result is final assembly-level proof that the migration changed the framework and package path, not the user-facing dashboard behavior.

## Operational Readiness (Q8)

- **Health signal:** `dashboard-route-parity.spec.ts` now acts as the high-signal health surface in both dev and prod by checking pathname, route-key, sidebar state, AI visibility, Issues state restoration, and the shared runtime-signal tracker for console errors and failed requests. The root `PLAYWRIGHT_PROJECT=dev ... --list` check confirms the mesh-lang harness still resolves the sibling `mesher/client` suite.
- **Failure signal:** Any parity assertion drift (route key, pathname, AI/sidebar state, Issues history restoration), any console error or failed request captured by the runtime-signal tracker, or any scoped grep hit for `frontend-exp` in the selected maintainer files is a slice-level failure.
- **Recovery procedure:** Re-run the targeted dev/prod parity spec first to localize runtime regressions, inspect the affected dashboard shell/state component or mount path (especially `dashboard-route-parity.spec.ts`, `events-chart.tsx`, and the dashboard shell/state files), then rerun the scoped grep pair and root `--list` command after any docs/path correction.
- **Monitoring gaps:** This remains test-driven browser proof only. The slice intentionally added no long-lived production telemetry or backend diagnostics because the migration contract stayed client-only and mock-data-only.

## Assumptions

- The existing mock-data/client-state contract remained the authoritative behavior baseline for final equivalence proof.
- Only direct maintainer-facing operational references needed updating; historical planning artifacts and unrelated product copy were intentionally left untouched.

## Verification

All slice-plan verification checks passed from `/Users/sn0w/Documents/dev/mesh-lang`:

- `npm --prefix ../hyperpush-mono/mesher/client run build`
- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev -- tests/e2e/dashboard-route-parity.spec.ts`
- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod -- tests/e2e/dashboard-route-parity.spec.ts`
- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev`
- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod`
- `! rg -n "mesher/frontend-exp|frontend-exp" ../hyperpush-mono/AGENTS.md ../hyperpush-mono/CONTRIBUTING.md ../hyperpush-mono/SUPPORT.md ../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml ./AGENTS.md`
- `rg -n "mesher/client" ../hyperpush-mono/AGENTS.md ../hyperpush-mono/CONTRIBUTING.md ../hyperpush-mono/SUPPORT.md ../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml ./AGENTS.md`
- `PLAYWRIGHT_PROJECT=dev npx --prefix ../hyperpush-mono/mesher/client playwright test --config ./playwright.config.ts --project=dev --list`

The targeted and full parity runs each passed all 9 tests in both environments, and the runtime-signal assertions stayed clean.

## Requirements Advanced

- R144 — Re-verified the canonical `mesher/client` package-path and root-harness contract after final slice closeout.
- R147 — Re-verified the TanStack Start build/start path through targeted and full dev/prod parity plus the root `--list` harness check.

## Requirements Validated

- R143 — Final `mesher/client` dev/prod parity runs passed with no meaningful user-visible drift from the migrated dashboard shell.
- R145 — Expanded parity coverage proved Solana AI/sidebar behavior, Issues history restoration, direct-entry route behavior, and clean runtime signals in both environments.
- R146 — All closeout work stayed on the mock-data/client-state boundary with no loaders, server functions, backend calls, or widened URL/search-param semantics.
- R148 — Scoped maintainer-file greps confirmed the selected docs/templates now reference `mesher/client` and no longer reference `frontend-exp`.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None.

## Known Limitations

No new functional limitations were introduced. Monitoring remains test-driven; the slice intentionally did not add backend telemetry or production diagnostics because the migration contract stayed client-only and mock-data-only.

## Follow-ups

Implementation follow-up is complete for this slice. The remaining work is milestone-level validation and closeout.

## Files Created/Modified

- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts` — Expanded the canonical parity suite to cover Solana Programs AI/sidebar restoration and Issues browser-history state restoration.
- `../hyperpush-mono/mesher/client/components/dashboard/events-chart.tsx` — Disabled Recharts Area animations to remove a real clean-boot console warning caught by the parity rail.
- `../hyperpush-mono/AGENTS.md` — Repointed product-workspace guidance to the canonical `mesher/client` dashboard package.
- `../hyperpush-mono/CONTRIBUTING.md` — Updated contributor commands and verification guidance to use `mesher/client`.
- `../hyperpush-mono/SUPPORT.md` — Updated support triage guidance to name `mesher/client` as the dashboard package.
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml` — Updated the bug-report template to reference the `mesher/client` dashboard surface.
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml` — Updated the feature-request template to reference the `mesher/client` dashboard surface.
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml` — Updated the docs issue template examples to reference `mesher/client` paths.
- `./AGENTS.md` — Updated mesh-lang workspace guidance so the product-owned dashboard package is named `mesher/client`.
- `./playwright.config.ts` — Remains the root harness that resolves the moved `mesher/client` suite; re-verified by the slice closeout `--list` command.
