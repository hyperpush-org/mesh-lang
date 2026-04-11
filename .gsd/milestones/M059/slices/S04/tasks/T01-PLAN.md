---
estimated_steps: 4
estimated_files: 4
skills_used:
  - test
  - react-best-practices
---

# T01: Extend the canonical parity suite for Solana AI collapse and browser-history equivalence

**Slice:** S04 — Equivalence proof and direct operational cleanup
**Milestone:** M059

## Description

Close the remaining meaningful route-behavior proof gaps from the canonical `../hyperpush-mono/mesher/client/` package path inside the existing Playwright parity harness instead of inventing a second verifier. Start with the current `dashboard-route-parity.spec.ts` seams, add one assertion path for the Solana Programs AI auto-collapse/restore branch and one assertion path for browser back/forward semantics after Issues search/filter/detail state changes, and keep the runtime contract stable unless the new proof exposes a real bug.

The default execution path is test-first and spec-only. Only touch `dashboard-shell.tsx`, `dashboard-issues-state.tsx`, or related route-map/runtime files if the added proof exposes an actual parity defect, and if that happens keep the fix on the existing mock-data/client-state boundary with no loaders, server functions, backend calls, or URL/search-param widening.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `../hyperpush-mono/mesher/client/playwright.config.ts` web-server orchestration | Treat boot failures as parity-contract regressions and inspect the requested project selection plus server command before changing app code. | Fail the task and inspect which environment did not become ready instead of weakening the test. | Reject misrouted base URLs or the wrong project selection as harness bugs, not app parity failures. |
| `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts` canonical proof surface | Keep new coverage inside the existing suite so future failures stay localized to one authoritative browser rail. | Treat hanging navigation or assertions as real regressions in route/history or AI-panel behavior. | Fail if assertions depend on unstable selectors or infer state without using the existing test ids and route-key markers. |
| `../hyperpush-mono/mesher/client/components/dashboard/dashboard-shell.tsx` client-state contract | Only patch runtime code if the new proof exposes a real defect. | Abort speculative refactors; keep the task centered on the observed failing seam. | Reject any fix that adds loaders, server functions, backend calls, or URL/search-param persistence. |

## Load Profile

- **Shared resources**: one dev server, one built-production server, the shared Playwright parity suite, and the shell-owned Issues/AI/sidebar client state.
- **Per-operation cost**: two browser parity runs against one spec file plus any minimal runtime fix required to restore truthful behavior.
- **10x breakpoint**: repeated route/history transitions and AI toggles would show state-restoration bugs first, not infrastructure saturation.

## Negative Tests

- **Malformed inputs**: deep-link entry into `/solana-programs`, repeated AI open/close actions, and browser history traversal after Issues filters/detail have changed.
- **Error paths**: parity suite reports a console error, failed request, wrong route key, lost Issues state, or incorrect sidebar restoration after AI closes.
- **Boundary conditions**: the proof must pass in both dev and built production without widening the pathname/search-param contract.

## Steps

1. Extend `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts` with a Solana Programs AI auto-collapse/restore assertion path that uses existing `data-testid` seams and the shared runtime-signal tracker.
2. Extend the same spec with browser back/forward assertions that prove Issues search/filter/detail state remains truthful when leaving and returning through real navigation history.
3. Run the updated spec in both dev and prod; only if a test exposes a real bug, make the minimum runtime change in the existing dashboard shell/state files.
4. Re-run the same dev/prod spec commands after any fix so the canonical parity suite becomes the lasting proof surface.

## Must-Haves

- [ ] The Solana Programs AI branch is proved through the existing parity suite, including sidebar auto-collapse on open and restoration on close when the branch conditions are met.
- [ ] Browser back/forward navigation is proved against the existing Issues client-state contract after search/filter/detail mutations.
- [ ] Any runtime fix stays within the existing mock-data/client-state model and does not add loaders, server functions, backend calls, or new URL/search-param semantics.

## Verification

- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev -- tests/e2e/dashboard-route-parity.spec.ts`
- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod -- tests/e2e/dashboard-route-parity.spec.ts`

## Observability Impact

- Signals added/changed: explicit route-key, pathname, sidebar-collapse, AI-panel, and console/request assertions for the remaining high-signal equivalence gaps.
- How a future agent inspects this: run the single-spec dev/prod commands and inspect `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts` plus the touched shell/state file if a failure appears.
- Failure state exposed: wrong active nav, missing AI auto-collapse/restore, lost Issues state after browser history traversal, or any console/request failure during parity proof.

## Inputs

- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts` — existing direct-entry and interaction parity suite that should remain the one browser proof rail.
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-shell.tsx` — current AI/sidebar behavior seam and the most likely runtime file if proof exposes a real defect.
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-issues-state.tsx` — current shell-owned Issues state seam that browser history proof must preserve.
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts` — pathname-to-route-key contract used by the dashboard shell and parity assertions.
- `../hyperpush-mono/mesher/client/playwright.config.ts` — package-local project and web-server contract for dev/prod parity.

## Expected Output

- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts` — strengthened canonical parity suite covering Solana AI collapse/restore and browser-history equivalence.
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-shell.tsx` — only if required, the minimum runtime fix that restores truthful AI/sidebar behavior without widening architecture.
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-issues-state.tsx` — only if required, the minimum client-state fix to keep Issues history behavior honest.
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts` — only if required, the minimum route-key contract fix surfaced by the new proof.
