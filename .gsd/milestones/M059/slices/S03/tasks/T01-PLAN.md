---
estimated_steps: 3
estimated_files: 6
skills_used:
  - vite
  - react-best-practices
  - test
---

# T01: Move the TanStack dashboard package to `mesher/client` and make the package-local contract truthful

**Slice:** S03 — Finalize move to `mesher/client` and remove Next.js runtime path
**Milestone:** M059

## Description

Close the highest-risk gap first by moving the already-migrated TanStack/Vite package wholesale instead of reconstructing it file-by-file. Preserve the route tree, bridge server, Playwright harness, and mock-data/client-state behavior that S02 already proved, and treat transient artifacts (`dist/`, `node_modules/`, `test-results/`, `.tanstack/`) as disposable rather than canonical content.

Keep the package-local runtime shape stable from the new path: only touch internal files if the directory move exposes a real path break. Rewrite the package README so maintainers see `vite dev`, `vite build`, `node server.mjs`, and the package-local parity rails instead of the stale Next.js/v0 boilerplate.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `../hyperpush-mono/mesher/frontend-exp/` working tree | Stop and preserve the old tree rather than partially moving tracked/untracked app files. | Abort the move and inspect the filesystem state before retrying so the canonical package does not split across two directories. | Reject partial copies or missing route/test/runtime files as move failures, not cleanup chores. |
| `../hyperpush-mono/mesher/client/package.json` runtime contract | Keep the existing `dev` / `build` / `start` / `test:e2e:*` script contract intact; only patch relative paths if the move breaks it. | Treat hanging dev/build/start commands as package-level regressions surfaced by the move. | Fail if the moved package points at stale roots or missing entrypoints. |
| `../hyperpush-mono/mesher/client/README.md` maintainer guidance | Replace stale Next.js/v0 instructions in the same task so the new path is truthful immediately. | N/A — static doc update. | Fail if README still describes `app/page.tsx`, Next.js, or v0 as the active runtime path. |

## Load Profile

- **Shared resources**: one package tree, package-local build output, and the shared Playwright/runtime scripts that must survive the move unchanged.
- **Per-operation cost**: filesystem rename plus targeted doc/runtime truthfulness checks; no new runtime surfaces should be added.
- **10x breakpoint**: path drift shows up first as missing files or broken script entrypoints, not as CPU load.

## Negative Tests

- **Malformed inputs**: missing old root, stale transient directories, or a partial move that leaves route/test files behind.
- **Error paths**: moved package still references `frontend-exp`, README still claims Next.js/v0, or script entrypoints no longer resolve from the new directory.
- **Boundary conditions**: tracked and currently-untracked TanStack files survive the move together, while ignored scratch directories can be regenerated.

## Steps

1. Move the current dashboard package from `../hyperpush-mono/mesher/frontend-exp/` to `../hyperpush-mono/mesher/client/`, preserving the real TanStack runtime/test files and excluding disposable build/test scratch directories from the canonical tree.
2. Verify the moved package keeps the same package-local command contract (`dev`, `build`, `start`, `test:e2e:dev`, `test:e2e:prod`) and only repair internal relative paths if the directory rename exposes a real break.
3. Replace the stale package README with TanStack/Vite-specific setup, run, and parity-test guidance rooted at `../hyperpush-mono/mesher/client/`.

## Must-Haves

- [ ] `../hyperpush-mono/mesher/client/` becomes the canonical package root for the migrated dashboard app.
- [ ] The moved package keeps the existing package-local `dev` / `build` / `start` / `test:e2e:*` contract.
- [ ] The package README no longer claims the app is a Next.js/v0 project.
- [ ] No loaders, server functions, backend calls, or new URL/search-param contracts are introduced during the move.

## Verification

- `test -f ../hyperpush-mono/mesher/client/package.json && test -f ../hyperpush-mono/mesher/client/server.mjs && test -f ../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts && test ! -e ../hyperpush-mono/mesher/frontend-exp/package.json`
- `rg -n "vite dev|vite build|node server\.mjs|test:e2e:dev|test:e2e:prod" ../hyperpush-mono/mesher/client/README.md`
- `! rg -n "Next\.js|v0|frontend-exp" ../hyperpush-mono/mesher/client/README.md`

## Observability Impact

- Signals added/changed: the first move failure should surface as a missing file, broken script, or stale README claim under `../hyperpush-mono/mesher/client/`.
- How a future agent inspects this: inspect `../hyperpush-mono/mesher/client/package.json`, `../hyperpush-mono/mesher/client/server.mjs`, and `../hyperpush-mono/mesher/client/README.md` before touching route logic.
- Failure state exposed: stale `frontend-exp` references, missing moved route/test files, or broken package-local entrypoints.

## Inputs

- `../hyperpush-mono/mesher/frontend-exp/package.json` — current package-local command contract to preserve at the new path.
- `../hyperpush-mono/mesher/frontend-exp/server.mjs` — production bridge that must stay directory-name agnostic after the move.
- `../hyperpush-mono/mesher/frontend-exp/vite.config.ts` — Vite/TanStack runtime config that should keep working without a root reshuffle.
- `../hyperpush-mono/mesher/frontend-exp/playwright.config.ts` — package-local parity harness to preserve.
- `../hyperpush-mono/mesher/frontend-exp/tests/e2e/dashboard-route-parity.spec.ts` — route parity proof surface that must move intact.
- `../hyperpush-mono/mesher/frontend-exp/README.md` — stale package-local doc surface that must be rewritten.

## Expected Output

- `../hyperpush-mono/mesher/client/package.json` — canonical package contract at the new path.
- `../hyperpush-mono/mesher/client/server.mjs` — moved production bridge entrypoint.
- `../hyperpush-mono/mesher/client/playwright.config.ts` — moved package-local parity harness.
- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts` — moved browser parity suite.
- `../hyperpush-mono/mesher/client/README.md` — truthful TanStack/Vite maintainer guidance at the new path.
- `../hyperpush-mono/mesher/client/src/routes/_dashboard.tsx` — moved route tree anchor proving the real app content survived the rename.
