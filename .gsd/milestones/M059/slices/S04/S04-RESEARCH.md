# S04 Research — Equivalence proof and direct operational cleanup

## Summary

**Primary requirements:** R143, R145, R146, R148.  
**Supporting requirements that must not regress while closing the slice:** R144, R147.

This is **targeted research**. S03 already made `../hyperpush-mono/mesher/client/` canonical and rewired the machine-checked CI/verifier/docs surfaces. The remaining S04 work splits cleanly into two low-coupling tracks:

1. **strengthen/close the final equivalence proof from the canonical package path**
2. **clean the remaining human-facing operational references that still say `frontend-exp`**

The active runtime route tree and package-local command contract already exist and should stay stable. The highest-value remaining proof work is inside the **existing Playwright parity spec**, not a new harness. The highest-value remaining cleanup is in **docs/templates/instructions across both repos**, not in app runtime code.

Current evidence:

- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts` already covers:
  - direct entry for `/`, `/performance`, `/solana-programs`, `/releases`, `/alerts`, `/bounties`, `/treasury`, and `/settings`
  - click-based Issues leave-and-return behavior
  - settings chrome behavior
  - AI panel open/close and close-on-nav behavior
  - unknown-path fallback rendering the Issues shell
  - zero console errors and zero failed requests
- The route/state model still intentionally keeps Issues filters/search/detail in **shell-owned client state** (`dashboard-issues-state.tsx`) and route identity derived from pathname (`dashboard-route-map.ts`, `dashboard-shell.tsx`). So S04 should **not** add URL/search-param persistence or reload persistence just to make proofs stronger.
- Active stale operational references remain in:
  - `../hyperpush-mono/AGENTS.md`
  - `../hyperpush-mono/CONTRIBUTING.md`
  - `../hyperpush-mono/SUPPORT.md`
  - `../hyperpush-mono/.github/ISSUE_TEMPLATE/{bug_report.yml,feature_request.yml,documentation.yml}`
  - `./AGENTS.md`
- Important exclusions:
  - repo-wide `Next.js` / `nextjs` strings still legitimately exist in `mesher/landing/**`
  - `../hyperpush-mono/mesher/client/lib/mock-data.ts` intentionally contains visible mock release text mentioning Next.js
  - those are **not** operational drift

## Skills Discovered

### Existing loaded skills used
- `react-best-practices`
  - relevant rule: **`rerender-derived-state-no-effect`** — keep active section derived from router pathname, not mirrored into new local state
  - relevant rule: **`rerender-use-ref-transient-values`** — current AI/sidebar timeout bookkeeping is already ref/local-state based; proof work should not refactor it
- `vite`
  - relevant guidance: stay on the existing **ESM `vite.config.ts` + package-local command contract**; do not widen S04 into new wrappers or build plumbing
- `test`
  - relevant critical rules: **MATCH EXISTING PATTERNS**, **READ BEFORE WRITING**, **VERIFY GENERATED TESTS**
  - practical implication: if S04 adds proof, extend the existing Playwright spec/harness rather than inventing a second one

### Newly discovered and installed for downstream units
- `deckardger/tanstack-agent-skills@tanstack-start-best-practices`
- `currents-dev/playwright-best-practices-skill@playwright-best-practices`

## Implementation Landscape

### 1) Existing equivalence proof is concentrated and already strong

**Files inspected**
- `../hyperpush-mono/mesher/client/package.json`
- `../hyperpush-mono/mesher/client/playwright.config.ts`
- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`
- `./playwright.config.ts`

**What matters**
- `package.json` exposes the canonical package-local rails:
  - `dev`: `vite dev`
  - `build`: `vite build`
  - `start`: `node server.mjs`
  - `test:e2e:dev` / `test:e2e:prod`
- Package-local `playwright.config.ts` already isolates dev vs prod with `PLAYWRIGHT_PROJECT` / `npm_config_project` and boots only the requested server.
- The shared parity spec already proves:
  - direct-entry shell state for every top-level route
  - click-based navigation parity and active-nav correctness
  - Issues search/filter/detail persistence when leaving and returning through sidebar nav
  - settings special chrome (`no shared header`, no AI button)
  - unknown-path fallback rendering Issues while preserving the unmatched pathname
  - zero console errors and zero failed requests in both environments
- `./playwright.config.ts` at the mesh-lang root still points at the moved package and remains a lightweight root harness, but S04 itself does not need to touch it unless the cleanup scope explicitly includes mesh-lang instructions only.

**Planner implication**
- There is no reason to create a second spec or a screenshot-only verifier first. The natural place for any extra proof is the existing `dashboard-route-parity.spec.ts`.

### 2) The remaining proof gaps are narrow and mostly test-only

**Files inspected**
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-shell.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-issues-state.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts`
- `../hyperpush-mono/mesher/client/components/dashboard/sidebar.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/issues-page.tsx`
- `../hyperpush-mono/mesher/client/src/routes/_dashboard.tsx`
- `../hyperpush-mono/mesher/client/src/routes/$.tsx`

**What matters**
- `DashboardShell` derives `activeNav` from `useLocation(...pathname)` via `getDashboardRouteKeyByPathname`; this is already the right route-truth seam.
- `DashboardIssuesStateProvider` keeps Issues search/status/severity/selected detail entirely in-memory and shell-scoped. This is why click-based leave-and-return persists today.
- The catch-all route `src/routes/$.tsx` intentionally renders the Issues shell without changing the unmatched pathname. Existing spec already knows this.
- The spec does **not** currently cover two behaviors that exist in code:
  1. **Solana Programs AI auto-collapse branch** — in `dashboard-shell.tsx`, opening AI on `solana-programs` with `window.innerWidth < 1920` auto-collapses the sidebar and closing AI restores it.
  2. **Browser history semantics** — current suite uses sidebar clicks and direct loads, but does not explicitly exercise `page.goBack()` / `page.goForward()` after route changes and Issues-state mutations.

**Manual dev observation**
- Running `npm --prefix ../hyperpush-mono/mesher/client run dev -- --host 127.0.0.1 --port 3000` and inspecting `http://127.0.0.1:3000/solana-programs` at the default desktop viewport showed:
  - opening AI sets `[data-testid="dashboard-sidebar"][data-collapsed="true"]`
  - closing AI restores `[data-collapsed="false"]`
- That confirms the Solana-only branch is real and testable with existing `data-testid` seams. No app code change appears necessary just to prove it.

**Planner implication**
- If S04 wants stronger equivalence proof, the cheapest truthful additions are:
  - one Playwright test for the Solana AI auto-collapse/restore behavior
  - one Playwright test using back/forward navigation with Issues state already mutated
- Existing test ids are sufficient; avoid app changes unless a proof attempt exposes a real bug.

### 3) Remaining direct operational drift is documentation/template/instruction text

**Files inspected**
- `../hyperpush-mono/AGENTS.md`
- `../hyperpush-mono/CONTRIBUTING.md`
- `../hyperpush-mono/SUPPORT.md`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml`
- `./AGENTS.md`

**Exact stale references found**
- `../hyperpush-mono/AGENTS.md`
  - repo ownership still lists `mesher/frontend-exp/`
- `../hyperpush-mono/CONTRIBUTING.md`
  - repo surface list still names `mesher/frontend-exp/`
  - development setup still says Node/npm are for `mesher/landing/` and `mesher/frontend-exp/`
  - common commands still run `npm --prefix mesher/frontend-exp ci` / `run build`
  - verification expectations still say “`frontend-exp` UI changes”
- `../hyperpush-mono/SUPPORT.md`
  - issue guidance still says `landing/frontend-exp bugs`
  - failure location prompt still says `mesher/frontend-exp/`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml`
  - description, dropdown options, and path placeholder still say `frontend-exp`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml`
  - description and area dropdown still say `frontend-exp`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml`
  - affected-page placeholder still says `mesher/frontend-exp/README.md`
- `./AGENTS.md` (mesh-lang)
  - workspace layout and ownership notes still say the product repo owns `mesher/frontend-exp/`

**Important exclusions**
- `../hyperpush-mono/README.md` is already canonical and should be treated as the baseline wording, not reopened casually.
- `REPO-SPLIT-SUMMARY.md` and `CI-SIMPLIFICATION-PLAN.md` in `mesh-lang` still contain `frontend-exp`, but they are historical/planning documents, not direct maintainer-operational guidance. I would not touch them unless the slice is explicitly widened to historical cleanup.
- `../hyperpush-mono/mesher/client/lib/mock-data.ts` contains visible mock content referencing Next.js. Do **not** “cleanup” those strings under R148; that would mutate product content rather than operational guidance.
- Repo-wide `Next.js` / `nextjs` greps are not a valid pass/fail signal because `mesher/landing/` is intentionally still a Next.js app.

### 4) Runtime code should stay stable unless proof exposes a bug

**Files inspected**
- `../hyperpush-mono/mesher/client/server.mjs`
- `../hyperpush-mono/mesher/client/vite.config.ts`
- `../hyperpush-mono/mesher/client/src/routes/__root.tsx`
- `../hyperpush-mono/mesher/client/README.md`

**What matters**
- `server.mjs` is still the truthful production bridge over `dist/client` and `dist/server/server.js`.
- `vite.config.ts` is minimal and path-agnostic.
- `src/routes/__root.tsx` is the established document/font/CSS seam and already correct.
- `README.md` inside `mesher/client` already documents the canonical runbook and should probably remain unchanged unless the added proof rails need one more command note.

**Planner implication**
- S04 is not the place to refactor build/start plumbing, route modules, or shared shell code for cleanliness alone.

## Recommendation

Treat S04 as a **proof + guidance cleanup** slice, not another migration slice.

1. **Keep the app runtime stable.**
   - Default plan: no route/runtime code changes.
2. **If proof needs strengthening, extend the existing parity spec only.**
   - Stay inside `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`.
3. **Update the remaining direct human-facing operational references from `frontend-exp` to `client`.**
   - Focus on docs/instruction/template files, not historical notes.
4. **Keep grep-based cleanup scoped.**
   - Negative greps should target the exact files S04 changed, not the whole repo, because landing and mock data legitimately still contain Next/nextjs/frontend-exp-adjacent history.

## Natural Task Seams

### Task seam 1 — Strengthen final equivalence proof
**Goal:** close any remaining meaningful parity gaps without reopening app architecture.

**Likely files**
- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`

**Likely additions**
- Solana Programs AI auto-collapse/restore proof
- Browser back/forward navigation proof after Issues state changes

**What should not change unless the new test fails**
- `server.mjs`
- `vite.config.ts`
- route files under `src/routes/**`
- `dashboard-route-map.ts`
- `dashboard-issues-state.tsx`

### Task seam 2 — Remaining direct operational cleanup in the product repo
**Goal:** make human-facing product maintainer guidance consistent with `mesher/client`.

**Likely files**
- `../hyperpush-mono/AGENTS.md`
- `../hyperpush-mono/CONTRIBUTING.md`
- `../hyperpush-mono/SUPPORT.md`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml`

### Task seam 3 — Remaining direct operational cleanup in mesh-lang
**Goal:** keep future agent/workspace instructions truthful about the product surface location.

**Likely files**
- `./AGENTS.md`

**Important scoping note**
- `REPO-SPLIT-SUMMARY.md` and `CI-SIMPLIFICATION-PLAN.md` are not necessary for the direct-operational minimum and can stay untouched.

### Task seam 4 — Closeout verification
**Goal:** prove the canonical package still builds/passes and that the selected docs/templates no longer contain stale path guidance.

**Likely commands**
- package build + dev/prod parity suite
- targeted stale-path greps against exactly the files changed in seams 2 and 3

## Risks / Fragile Seams

- **Do not accidentally mutate product mock content.** `lib/mock-data.ts` contains user-visible release text mentioning Next.js; changing it would be a product-content change, not operational cleanup.
- **Do not widen Issues state into URL params or persistence storage.** R146 is still the contract. Fresh reloads should boot the default state; in-session leave-and-return behavior is the honest proof target.
- **Do not use repo-wide “no Next.js/frontend-exp” greps as acceptance.** `mesher/landing/` is intentionally Next.js and historical planning docs legitimately mention the old path.
- **If touching AGENTS files, preserve the split-workspace safety rules verbatim where possible.** Only rename the product surface and workspace tree; do not reword the guardrails unless the text is already wrong.
- **If extending Playwright, keep the current style.** The existing spec centralizes runtime signal tracking and direct-entry helpers; follow that pattern instead of adding a parallel helper stack.

## Verification

### Package/runtime truth
From `mesh-lang/`:

```bash
npm --prefix ../hyperpush-mono/mesher/client run build
npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev
npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod
```

### Direct operational cleanup checks
After editing the selected docs/templates/instructions, run a scoped stale-path grep only on those files:

```bash
rg -n "mesher/frontend-exp|frontend-exp" \
  ../hyperpush-mono/AGENTS.md \
  ../hyperpush-mono/CONTRIBUTING.md \
  ../hyperpush-mono/SUPPORT.md \
  ../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml \
  ../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml \
  ../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml \
  ./AGENTS.md
```

And positively confirm the canonical path appears:

```bash
rg -n "mesher/client" \
  ../hyperpush-mono/AGENTS.md \
  ../hyperpush-mono/CONTRIBUTING.md \
  ../hyperpush-mono/SUPPORT.md \
  ../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml \
  ../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml \
  ../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml \
  ./AGENTS.md
```

### Optional root-harness confidence check
Only if the executor changes proof rails rather than docs-only text:

```bash
PLAYWRIGHT_PROJECT=dev npx --prefix ../hyperpush-mono/mesher/client playwright test --config ./playwright.config.ts --project=dev --list
```

## Planner Notes

- S04 is low coupling: proof work and docs/template cleanup can be separate tasks.
- The most likely “no app code required” path is:
  1. extend `dashboard-route-parity.spec.ts` only if you want stronger proof
  2. update stale docs/templates/instructions
  3. rerun build + package-local e2e + scoped greps
- If time/scope must be cut, the best value order is:
  1. direct operational cleanup across AGENTS/CONTRIBUTING/SUPPORT/issue templates
  2. one targeted Playwright addition for the Solana AI branch
  3. optional back/forward proof
- The riskiest false move is treating historical docs or mock-data strings as operational drift and “cleaning” them. Keep the slice surgical.
