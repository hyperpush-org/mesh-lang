---
id: T01
parent: S03
milestone: M059
key_files:
  - ../hyperpush-mono/mesher/client/package.json
  - ../hyperpush-mono/mesher/client/server.mjs
  - ../hyperpush-mono/mesher/client/playwright.config.ts
  - ../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts
  - ../hyperpush-mono/mesher/client/README.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Move the package wholesale to `mesher/client` and prune generated output after the move instead of reconstructing files piecemeal.
duration: 
verification_result: mixed
completed_at: 2026-04-11T18:03:43.971Z
blocker_discovered: false
---

# T01: Moved the TanStack dashboard package to `mesher/client`, rewrote the package README, and proved dev/prod route parity from the new root.

**Moved the TanStack dashboard package to `mesher/client`, rewrote the package README, and proved dev/prod route parity from the new root.**

## What Happened

Moved the real dashboard package from `../hyperpush-mono/mesher/frontend-exp/` to `../hyperpush-mono/mesher/client/` as a filesystem move, then pruned generated output so the canonical tree no longer carries disposable `node_modules/`, `dist/`, `test-results/`, `.tanstack/`, or stale `.next/` artifacts. Verified the moved package had no internal `frontend-exp` self-references that needed repair, preserved the existing package-local `dev` / `build` / `start` / `test:e2e:*` contract, and replaced the stale package README with truthful TanStack/Vite maintainer guidance rooted at `mesher/client`. Reinstalled dependencies at the new root and reran the real package-local build plus dev/prod Playwright parity suites successfully. Recorded one non-obvious runtime gotcha in `.gsd/KNOWLEDGE.md`: `app/globals.css` remains live because `src/routes/__root.tsx` imports it directly.

## Verification

Task-local checks passed: the moved package root exists at `../hyperpush-mono/mesher/client/`, `../hyperpush-mono/mesher/frontend-exp/package.json` is gone, the new package README advertises `vite dev`, `vite build`, `node server.mjs`, and the package-local parity scripts, and the README no longer mentions `Next.js`, `v0`, or `frontend-exp`. Real runtime proof also passed from the new path via `npm --prefix ../hyperpush-mono/mesher/client run build`, `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev`, and `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod`, with both Playwright projects passing all 7 parity tests. Slice-level external path-contract checks are still red, as expected for T01, because T02 still needs to rewrite the cross-repo `frontend-exp` references to `mesher/client`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `test -f ../hyperpush-mono/mesher/client/package.json && test -f ../hyperpush-mono/mesher/client/server.mjs && test -f ../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts && test ! -e ../hyperpush-mono/mesher/frontend-exp/package.json` | 0 | ✅ pass | 13ms |
| 2 | `rg -n 'vite dev|vite build|node server\\.mjs|test:e2e:dev|test:e2e:prod' ../hyperpush-mono/mesher/client/README.md` | 0 | ✅ pass | 31ms |
| 3 | `! rg -n 'Next\\.js|v0|frontend-exp' ../hyperpush-mono/mesher/client/README.md` | 0 | ✅ pass | 22ms |
| 4 | `npm --prefix ../hyperpush-mono/mesher/client run build` | 0 | ✅ pass | 24700ms |
| 5 | `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev` | 0 | ✅ pass | 64600ms |
| 6 | `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod` | 0 | ✅ pass | 30700ms |
| 7 | `! rg -n 'mesher/frontend-exp|frontend-exp' ../hyperpush-mono/.github/workflows/ci.yml ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh ../hyperpush-mono/README.md ../hyperpush-mono/.github/dependabot.yml ./playwright.config.ts` | 1 | ❌ fail | 26ms |
| 8 | `rg -n 'mesher/client' ../hyperpush-mono/.github/workflows/ci.yml ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh ../hyperpush-mono/README.md ../hyperpush-mono/.github/dependabot.yml ./playwright.config.ts` | 1 | ❌ fail | 28ms |

## Deviations

Also removed stale `.next/` output during the move even though the task explicitly named only `dist/`, `node_modules/`, `test-results`, and `.tanstack/`. This kept generated Next-era artifacts from becoming part of the new canonical package root.

## Known Issues

Cross-repo machine-checked path-contract files still reference `frontend-exp` and do not yet reference `mesher/client`; that follow-on work belongs to T02. The broader slice-plan grep `rg -n "mesher/client|client" ...` is also noisy because unrelated strings like `PostgreSQL client` can satisfy the generic `client` branch even while the path contract remains stale.

## Files Created/Modified

- `../hyperpush-mono/mesher/client/package.json`
- `../hyperpush-mono/mesher/client/server.mjs`
- `../hyperpush-mono/mesher/client/playwright.config.ts`
- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts`
- `../hyperpush-mono/mesher/client/README.md`
- `.gsd/KNOWLEDGE.md`
