# S04: Equivalence proof and direct operational cleanup

**Goal:** Re-prove the canonical `../hyperpush-mono/mesher/client/` dashboard package against the remaining meaningful route-behavior gaps and remove the last direct maintainer-facing `frontend-exp` guidance so the migrated app can be exercised and operated from truthful package-path references only.
**Demo:** After this: maintainers can run the migrated app, exercise the key dashboard flows, and rely on updated direct references without stale `frontend-exp` / Next.js operational guidance.

## Must-Haves

- The canonical parity suite in `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts` proves the remaining high-signal behaviors from `mesher/client`: Solana Programs AI auto-collapse/restore and browser back/forward navigation after Issues-state changes.
- The strengthened proof stays on the existing mock-data/client-state contract: no TanStack loaders, no server functions, no Mesher backend calls, and no widened URL/search-param semantics are introduced.
- The remaining direct operational references in `../hyperpush-mono/AGENTS.md`, `../hyperpush-mono/CONTRIBUTING.md`, `../hyperpush-mono/SUPPORT.md`, `../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml`, `../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml`, `../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml`, and `./AGENTS.md` all point at `mesher/client` instead of `frontend-exp`.
- The canonical package still passes `build`, dev parity, and prod parity from `../hyperpush-mono/mesher/client/`, and the mesh-lang root Playwright harness still lists against the moved package path.

## Threat Surface

- **Abuse**: malformed deep links, rapid route/history traversal, and repeated AI-panel toggles must not widen routing semantics, leave the shell in an inconsistent state, or reintroduce stale entrypoints in maintainer guidance.
- **Data exposure**: none beyond existing mock dashboard fixtures; this slice must not add auth, secrets, server-backed data, or new runtime diagnostics that expose private state.
- **Input trust**: browser pathname/history plus Issues search/filter inputs remain untrusted client input and must stay client-only with no new loaders, server functions, or backend calls.

## Requirement Impact

- **Requirements touched**: R143, R145, R146, R148, with supporting re-verification for R144 and R147.
- **Re-verify**: direct-entry and navigation parity from `mesher/client` in dev and built production, Issues leave-and-return behavior under browser history, Solana AI sidebar behavior, package `build` / `start` truth, and the selected maintainer-facing docs/templates/instructions.
- **Decisions revisited**: D496, D499, D501, D503, D504.

## Proof Level

- This slice proves: final-assembly equivalence and maintainer-operational closure from the real `mesher/client` runtime/test entrypoints.
- Real runtime required: yes.
- Human/UAT required: no.

## Verification

- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev -- tests/e2e/dashboard-route-parity.spec.ts`
- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod -- tests/e2e/dashboard-route-parity.spec.ts`
- `npm --prefix ../hyperpush-mono/mesher/client run build`
- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev`
- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod`
- `! rg -n "mesher/frontend-exp|frontend-exp" ../hyperpush-mono/AGENTS.md ../hyperpush-mono/CONTRIBUTING.md ../hyperpush-mono/SUPPORT.md ../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml ./AGENTS.md`
- `rg -n "mesher/client" ../hyperpush-mono/AGENTS.md ../hyperpush-mono/CONTRIBUTING.md ../hyperpush-mono/SUPPORT.md ../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml ./AGENTS.md`
- `PLAYWRIGHT_PROJECT=dev npx --prefix ../hyperpush-mono/mesher/client playwright test --config ./playwright.config.ts --project=dev --list`

## Observability / Diagnostics

- Runtime signals: the shared Playwright console-error and failed-request tracker in `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`, route-key/sidebar/AI-panel state assertions, scoped stale-path greps over the touched guidance files, and the root-harness `--list` command remain the primary failure-localization surfaces.
- Inspection surfaces: `../hyperpush-mono/mesher/client/playwright.config.ts`, `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`, `../hyperpush-mono/mesher/client/components/dashboard/dashboard-shell.tsx`, `playwright.config.ts`, and the selected docs/template files named in the scoped grep.
- Failure visibility: parity regressions surface as explicit heading/pathname/route-key/sidebar/AI assertions or console/request failures; stale operational drift surfaces as exact grep hits in the touched files.
- Redaction constraints: stay on mock data only and do not add env dumps, backend diagnostics, or secret-bearing logs.

## Integration Closure

- Upstream surfaces consumed: `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`, `../hyperpush-mono/mesher/client/playwright.config.ts`, `../hyperpush-mono/mesher/client/components/dashboard/dashboard-shell.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/dashboard-issues-state.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts`, product maintainer docs/templates under `../hyperpush-mono/`, and workspace guidance in `./AGENTS.md`.
- New wiring introduced in this slice: stronger assertions inside the existing parity suite and the last direct operational reference rewrites from `frontend-exp` to `mesher/client`; no new runtime boundaries should be added.
- What remains before the milestone is truly usable end-to-end: nothing beyond executing the planned proof and cleanup successfully.

## Tasks

- [x] **T01: Extend the canonical parity suite for Solana AI collapse and browser-history equivalence** `est:95m`
  - Why: retire the remaining runtime-risk proof gaps first using the existing `mesher/client` test seam instead of inventing a second verifier.
  - Files: `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`, `../hyperpush-mono/mesher/client/components/dashboard/dashboard-shell.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/dashboard-issues-state.tsx`, `../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts`, `../hyperpush-mono/mesher/client/playwright.config.ts`
  - Do: add one parity test path for the Solana Programs AI auto-collapse/restore branch and one parity test path for browser back/forward semantics after Issues-state mutations; keep the default path test-first and spec-only; only touch shell/state/runtime code if the new proof exposes a real defect; and if a runtime fix is needed, keep it on the existing mock-data/client-state boundary with no loaders, server functions, backend calls, or URL/search-param widening.
  - Verify: `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev -- tests/e2e/dashboard-route-parity.spec.ts && npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod -- tests/e2e/dashboard-route-parity.spec.ts`
  - Done when: the canonical parity suite proves the new Solana AI and browser-history behaviors in both environments and any necessary runtime fix stays within the existing client-state contract.
- [x] **T02: Rewrite the remaining direct operational guidance to `mesher/client` and close the slice** `est:85m`
  - Why: the slice is not complete until human-facing operational guidance and the canonical verification rails all agree on `mesher/client` as the dashboard package path.
  - Files: `../hyperpush-mono/AGENTS.md`, `../hyperpush-mono/CONTRIBUTING.md`, `../hyperpush-mono/SUPPORT.md`, `../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml`, `../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml`, `../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml`, `./AGENTS.md`, `./playwright.config.ts`, `../hyperpush-mono/mesher/client/package.json`
  - Do: update the selected AGENTS/CONTRIBUTING/SUPPORT/issue-template surfaces plus mesh-lang workspace guidance to name `mesher/client` as the dashboard package; preserve `mesher/landing` as the intentional Next.js app; leave historical planning documents and mock release text untouched; then run the full closeout proof from `mesh-lang`, including canonical build, full dev/prod parity, scoped stale-path and positive-path greps, and the root-harness `--list` check.
  - Verify: `npm --prefix ../hyperpush-mono/mesher/client run build && npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev && npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod && ! rg -n "mesher/frontend-exp|frontend-exp" ../hyperpush-mono/AGENTS.md ../hyperpush-mono/CONTRIBUTING.md ../hyperpush-mono/SUPPORT.md ../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml ./AGENTS.md && rg -n "mesher/client" ../hyperpush-mono/AGENTS.md ../hyperpush-mono/CONTRIBUTING.md ../hyperpush-mono/SUPPORT.md ../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml ../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml ./AGENTS.md && PLAYWRIGHT_PROJECT=dev npx --prefix ../hyperpush-mono/mesher/client playwright test --config ./playwright.config.ts --project=dev --list`
  - Done when: the selected docs/templates/instructions no longer mention `frontend-exp`, the canonical runtime rails stay green from `mesher/client`, and the root harness still resolves the moved package path.

## Files Likely Touched

- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-shell.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-issues-state.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts`
- `../hyperpush-mono/mesher/client/playwright.config.ts`
- `../hyperpush-mono/AGENTS.md`
- `../hyperpush-mono/CONTRIBUTING.md`
- `../hyperpush-mono/SUPPORT.md`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml`
- `./AGENTS.md`
- `./playwright.config.ts`
- `../hyperpush-mono/mesher/client/package.json`
