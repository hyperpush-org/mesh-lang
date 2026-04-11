# S03 Research — Finalize move to `mesher/client` and remove Next.js runtime path

## Summary

**Primary requirements:** R144, R147.
**Supporting requirements that must not regress during the move:** R143, R145, R146.
**R148 note:** most of the broad stale-reference cleanup still looks like S04 work, but a small set of path references are now build-/verifier-critical and should move with S03.

This is **targeted research**, not a new architecture slice. S02 already finished the TanStack route migration inside `../hyperpush-mono/mesher/frontend-exp/`. The remaining risk is mostly **filesystem move + hardcoded path contract fallout**, not route/layout logic.

The good news: the package-local app is already path-agnostic. I found **no `frontend-exp` or `mesher/client` strings inside the app package itself**, and the current built-production bridge (`server.mjs`) resolves from `__dirname`, so the app should survive a directory rename with little or no runtime code change.

The real work is in the **external surfaces that still hardcode `mesher/frontend-exp`**:
- product CI
- product maintainer verifier
- product root README/docs
- Dependabot npm directory
- mesh-lang root Playwright config used for cross-repo verification

## Skills Discovered

### Installed / referenced skills
- `vite` (already installed, used for guidance)
- `react-best-practices` (already installed, used for guidance)

### Newly discovered and installed for downstream units
- `deckardger/tanstack-agent-skills@tanstack-start-best-practices`
- `currents-dev/playwright-best-practices-skill@playwright-best-practices`

### Relevant rules from loaded skills
- **`vite` skill:** keep the move on the existing **package-local ESM + `vite.config.ts`** contract. Do **not** widen this into a new root workspace wrapper, CommonJS bridge, or config reshuffle just to rename the directory.
- **`react-best-practices` skill:** preserve the existing pathname-derived shell behavior; this slice should **not** re-open route/state ownership. The move should avoid new effect-driven nav/state logic (`rerender-derived-state-no-effect`, `rerender-dependencies` are the relevant guardrails).

## Implementation Landscape

### 1) The app package is already TanStack/Vite and mostly move-safe

**Files inspected**
- `../hyperpush-mono/mesher/frontend-exp/package.json`
- `../hyperpush-mono/mesher/frontend-exp/server.mjs`
- `../hyperpush-mono/mesher/frontend-exp/vite.config.ts`
- `../hyperpush-mono/mesher/frontend-exp/tsconfig.json`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/__root.tsx`
- `../hyperpush-mono/mesher/frontend-exp/src/router.tsx`
- `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts`

**What matters**
- `package.json` already exposes the correct contract locally:
  - `dev`: `vite dev`
  - `build`: `vite build`
  - `start`: `node server.mjs`
  - `test:e2e:dev` / `test:e2e:prod`: package-local Playwright rails
- `server.mjs` uses `__dirname` + `dist/client` + `dist/server/server.js`. It is **directory-name agnostic**.
- `vite.config.ts` is simple and local: `tanstackStart()`, `react()`, `tailwindcss()`; no path assumptions to `frontend-exp`.
- `src/routes/__root.tsx` loads CSS and font assets by relative package-local paths, not repo-root paths.
- `src/router.tsx` / route tree are package-local only.
- `tests/e2e/dashboard-route-parity.spec.ts` asserts behavior, not filesystem path. After the move, it should keep working unchanged if run from the moved package.

**Important result**
- `rg -n "frontend-exp|mesher/client" ../hyperpush-mono/mesher/frontend-exp ...` returned **no in-package hardcoded path references**.
- The internal runtime is already decoupled from the old directory name.

### 2) Next.js is already off the client app runtime path; the remaining drift is docs/labels

**What I checked**
- `rg -n "from 'next|from \"next|next/" ../hyperpush-mono/mesher/frontend-exp ...`

**Result**
- No real Next runtime imports remain in the dashboard app.
- The only `next-*` runtime surface still in the client package is `next-themes`, which is fine and expected.
- `package-lock.json` still contains `next` only as a **peer metadata trace** from ecosystem packages, not as an app dependency.
- `README.md` inside the package is still stale **Next.js/v0 boilerplate** and should be rewritten when the app becomes canonical at `mesher/client`.

**Critical scoping note**
- `../hyperpush-mono/mesher/landing/**` is still a legitimate **Next.js 16** app.
- S03 must **not** turn into a repo-wide “remove Next” sweep. R147 here means: **the canonical dashboard app no longer needs Next on its runtime path**. Landing is out of scope.

### 3) Current verification baseline is already green from the old path

**Commands run**
- `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build`
- `PORT=3011 npm --prefix ../hyperpush-mono/mesher/frontend-exp run start`
- Browser assertions against `http://127.0.0.1:3011/`

**Observed**
- `vite build` passed and emitted:
  - `dist/client/**`
  - `dist/server/server.js`
- `npm run start` booted cleanly on an alternate port through `server.mjs`.
- Browser assertions passed for:
  - visible `Issues` shell
  - no console errors
  - no failed requests

**Implication**
- The package-local `dev` / `build` / `start` contract is already honest.
- S03 risk is **not** the runtime itself; it is the **path move and its external references**.

### 4) The machine-checked old-path references are the main blocker

**Critical path-sensitive files inspected**
- `../hyperpush-mono/.github/workflows/ci.yml`
- `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`
- `../hyperpush-mono/README.md`
- `../hyperpush-mono/.github/dependabot.yml`
- `playwright.config.ts` (mesh-lang root)

#### `../hyperpush-mono/.github/workflows/ci.yml`
Hardcodes `frontend-exp` in multiple places:
- job name: `Landing and frontend-exp build`
- npm cache dependency path: `mesher/frontend-exp/package-lock.json`
- install/build steps:
  - `npm --prefix mesher/frontend-exp ci`
  - `npm --prefix mesher/frontend-exp run build`

This must change with the rename or CI will immediately go stale.

#### `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`
This is the sneaky one. The product-contract checks currently **require** old strings, including:
- `mesher/frontend-exp` in root README markers
- `npm --prefix mesher/frontend-exp ci`
- `npm --prefix mesher/frontend-exp run build`
- CI wording that still says `frontend-exp`

So if S03 updates README/CI to `mesher/client` but does **not** update this verifier, the maintainer verifier becomes self-contradictory.

#### `../hyperpush-mono/README.md`
Currently describes:
- `mesher/frontend-exp/` as the dashboard surface
- `.github/workflows/ci.yml` as product CI for Mesher + landing + `frontend-exp`
- “The landing app and `frontend-exp` stay product-owned here.”

This is part of the machine-checked contract via `verify-maintainer-surface.sh`, so README + verifier + CI should move together.

#### `../hyperpush-mono/.github/dependabot.yml`
Still points npm updates at:
- `directory: "/mesher/frontend-exp"`

This is not the highest-risk runtime break, but it is a cheap operational fix that should likely move with S03 because the package path is changing anyway.

#### `playwright.config.ts` in `mesh-lang`
The root config added for cross-repo verification still hardcodes:
- `const frontendExpRoot = '../hyperpush-mono/mesher/frontend-exp'`
- `testDir` under that path
- both `webServer` commands under that path

If S03 moves the app but leaves this file stale, mesh-lang-side parity verification immediately points at the wrong directory.

### 5) Broader stale references exist, but not all of them are equally urgent

These files still reference `frontend-exp` and are likely **S04 follow-up** unless the executor wants to absorb low-risk text cleanup early:
- `../hyperpush-mono/CONTRIBUTING.md`
- `../hyperpush-mono/SUPPORT.md`
- `../hyperpush-mono/AGENTS.md`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/bug_report.yml`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/feature_request.yml`
- `../hyperpush-mono/.github/ISSUE_TEMPLATE/documentation.yml`
- `./AGENTS.md` (mesh-lang root)
- `./REPO-SPLIT-SUMMARY.md`
- `./CI-SIMPLIFICATION-PLAN.md`

My read: **minimum S03** should update the path references that are directly required to keep the canonical app runnable and the verification surfaces truthful. Broader documentation/template cleanup still fits S04.

### 6) Workspace-state constraint: the product repo is dirty and includes untracked S01/S02 files

`git -C ../hyperpush-mono status --short` currently shows the sibling repo is dirty **only under `mesher/frontend-exp`**, including:
- modified tracked files from S01/S02
- deleted old Next files (`next-env.d.ts`, `next.config.mjs`)
- untracked new TanStack/runtime files such as:
  - `src/`
  - `tests/`
  - `playwright.config.ts`
  - `server.mjs`
  - new dashboard shell/provider files

This matters because S03 is likely executing **on top of an uncommitted migrated tree**, not from a clean committed baseline.

**Planner implication**
- Do not assume a clean git rename only over tracked files.
- The move must preserve the untracked-but-real S01/S02 additions.
- Also note transient directories:
  - `dist/` is repo-ignored at the product root
  - `.next/` and `node_modules/` are package-ignored
  - `test-results/` is untracked noise
  - `.tanstack/` exists locally

It is worth deciding explicitly whether to clean transient artifacts before/after the move or widen `.gitignore` so the renamed package does not keep dragging Playwright/TanStack scratch state around.

## Recommendation

Treat S03 as a **path + contract migration**.

Do **not** reopen the route decomposition, dashboard shell logic, mock-data contract, or TanStack runtime shape unless the move exposes a real break.

### Recommended execution order

1. **Move the package tree** from `../hyperpush-mono/mesher/frontend-exp` to `../hyperpush-mono/mesher/client` while preserving the already-migrated TanStack files.
2. **Keep app internals unchanged** unless the move exposes an actual path/runtime break.
3. In the same slice, update the **machine-checked path contract surfaces**:
   - `../hyperpush-mono/.github/workflows/ci.yml`
   - `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`
   - `../hyperpush-mono/README.md`
   - `../hyperpush-mono/.github/dependabot.yml` (cheap, path-direct)
   - `./playwright.config.ts` in mesh-lang
4. Update the **package-local README** so the canonical app no longer claims it is a Next.js/v0 project.
5. Re-run package-local runtime/parity verification from the **new path**.
6. Leave the broader text/template sweep for S04 unless there is cheap adjacent leverage.

## Natural Task Seams

### Task seam 1 — Filesystem move + package-local truthfulness
**Goal:** make `../hyperpush-mono/mesher/client/` the canonical app path without changing behavior.

**Likely files / paths**
- whole directory move from `../hyperpush-mono/mesher/frontend-exp/` to `../hyperpush-mono/mesher/client/`
- `../hyperpush-mono/mesher/client/README.md`

**What not to touch unless broken**
- route files under `src/routes/**`
- dashboard shell/provider code
- mock-data modules
- `server.mjs`
- `vite.config.ts`

### Task seam 2 — Product-root automation/verifier updates
**Goal:** keep the repo’s machine-checked path contract aligned with the renamed package.

**Likely files**
- `../hyperpush-mono/.github/workflows/ci.yml`
- `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`
- `../hyperpush-mono/README.md`
- `../hyperpush-mono/.github/dependabot.yml`

### Task seam 3 — mesh-lang-side verification surface updates
**Goal:** keep cross-repo browser verification pointed at the new canonical path.

**Likely files**
- `./playwright.config.ts`

**Optional nearby cleanup if the executor wants it**
- `./AGENTS.md`

### Task seam 4 — Broader path/reference cleanup (probably S04)
**Goal:** replace remaining human-facing stale `frontend-exp` references that are not runtime-critical.

**Likely files**
- `../hyperpush-mono/CONTRIBUTING.md`
- `../hyperpush-mono/SUPPORT.md`
- `../hyperpush-mono/AGENTS.md`
- issue templates / docs in both repos

## Verification

### High-signal commands for S03
Run these from `mesh-lang/` after the move:

```bash
npm --prefix ../hyperpush-mono/mesher/client run build
PORT=3001 npm --prefix ../hyperpush-mono/mesher/client run start
```

Then verify browser/runtime behavior against the built app, or use the existing package-local parity rails:

```bash
npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev
npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod
```

### Targeted path-contract checks
After edits, run targeted greps to make sure the critical old path is gone from the surfaces S03 changed:

```bash
rg -n "mesher/frontend-exp|frontend-exp" \
  ../hyperpush-mono/.github/workflows/ci.yml \
  ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh \
  ../hyperpush-mono/README.md \
  ../hyperpush-mono/.github/dependabot.yml \
  ./playwright.config.ts
```

And verify the new path is present where expected:

```bash
rg -n "mesher/client|client" \
  ../hyperpush-mono/.github/workflows/ci.yml \
  ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh \
  ../hyperpush-mono/README.md \
  ../hyperpush-mono/.github/dependabot.yml \
  ./playwright.config.ts
```

### Scope guard
Use a scoped search before claiming “Next is gone”:

```bash
rg -n "from 'next|from \"next|next/" ../hyperpush-mono/mesher/client -g '!**/node_modules/**' -g '!**/dist/**'
```

This should stay clean except for legitimate `next-themes` usage. Do **not** use repo-wide `rg next` as a pass/fail check because `mesher/landing` is intentionally still a Next app.

## Planner Notes

- The runtime migration itself is basically done; S03 is mostly about making the **new path canonical and truthful**.
- The riskiest hidden edge is **not** TanStack — it is the **machine-checked root verifier** that still requires `frontend-exp` strings.
- The second hidden edge is that the sibling repo is **already dirty and partially untracked**, so the move must preserve the current migrated tree rather than reconstructing it from the old tracked baseline.
- If you need to cut scope aggressively, prioritize:
  1. move package to `mesher/client`
  2. keep `dev` / `build` / `start` green there
  3. update CI + verifier + root README + mesh-lang Playwright config
  4. leave broader documentation/template sweep for S04
