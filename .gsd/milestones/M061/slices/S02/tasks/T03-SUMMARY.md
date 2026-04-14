---
id: T03
parent: S02
milestone: M061
key_files:
  - ../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts
  - ../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts
  - ../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts
  - ../hyperpush-mono/mesher/client/components/ui/use-toast.ts
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Reused existing stable selectors and `data-source`/`data-state` markers for mixed-surface proof instead of introducing new component-specific test ids.
  - Preserved the strict console-error verification bar and fixed the shared toast listener leak in `use-toast.ts` after the Playwright gate exposed it, rather than loosening the selected-issue failure-path assertion.
duration: 
verification_result: passed
completed_at: 2026-04-12T07:09:24.824Z
blocker_discovered: false
---

# T03: Expanded mixed-surface Playwright proof assertions and fixed the shared toast listener leak they exposed.

**Expanded mixed-surface Playwright proof assertions and fixed the shared toast listener leak they exposed.**

## What Happened

I mapped the T01 mixed-surface rows against the existing Playwright suites, then tightened only the missing proof edges instead of adding new flows. In `../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts` I added explicit shell-only/runtime checks for the Issues detail helper cluster and asserted the retained proof-harness diagnostics directly (`issue-action-proof-last-action`, `issue-action-proof-last-issue`, `issue-action-proof-error`, and `issue-action-proof-stage`) so the shell-only proof rail is honest at control granularity. In `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts` and `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts` I added explicit assertions for alert shell-only controls (`alert-detail-action-silence`, `alert-detail-copy-link`), alert mixed-state badges (`alert-detail-source-badge`), and Settings mixed/mock markers (`settings-shell-support-badge`, `settings-general-source-badge`, `settings-general-mock-only-banner`, `settings-alert-rules-status-banner`, `settings-alert-channels-source-badge`).

The first full Playwright gate then surfaced a real runtime bug outside the planned assertion changes: the selected-issue failure-path test emitted a React warning about updating state before mount. I traced that to `../hyperpush-mono/mesher/client/components/ui/use-toast.ts`, where `useToast()` re-subscribed listeners on every state change because its effect depended on `state`. I fixed the root cause by making the subscription effect stable (`[]` deps), reran the exact failing issue-read repro, and then reran the full slice browser suite. After that fix, the targeted issue failure-path repro and the full slice browser contract both passed cleanly. I also recorded the toast-listener gotcha in `.gsd/KNOWLEDGE.md` because it is subtle and likely to recur in future error-path verification work.

## Verification

Ran the slice structural contract and browser proof rails from `mesh-lang`. `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` passed, confirming the inventory still matches the canonical route map and mixed-surface contract after the proof updates. The first full Playwright pass exposed an existing React warning in the selected-issue failure path, so I reproduced that exact case with a focused `issues-live-read.spec.ts -g "shows a visible toast when selected-issue reads fail"` run, fixed the shared toast listener leak in `use-toast.ts`, reran that exact repro successfully, and then reran the full targeted Playwright command for `issues-live-read`, `issues-live-actions`, `admin-ops-live`, and `seeded-walkthrough`. The final full Playwright rerun passed 21/21 tests.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` | 0 | ✅ pass | 643ms |
| 2 | `env PLAYWRIGHT_PROJECT=dev npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts -g "shows a visible toast when selected-issue reads fail"` | 0 | ✅ pass | 43200ms |
| 3 | `env PLAYWRIGHT_PROJECT=dev npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts ../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts` | 0 | ✅ pass | 169300ms |

## Deviations

The plan only called for minimal proof-assertion updates, but the first full verification run surfaced a pre-existing runtime bug in the shared toast store. I fixed that root cause in `../hyperpush-mono/mesher/client/components/ui/use-toast.ts` rather than weakening the console-error assertion in the failing issue-read test.

## Known Issues

None.

## Files Created/Modified

- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts`
- `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts`
- `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts`
- `../hyperpush-mono/mesher/client/components/ui/use-toast.ts`
- `.gsd/KNOWLEDGE.md`
