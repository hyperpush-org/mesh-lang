---
id: S03
parent: M059
milestone: M059
provides:
  - A canonical `../hyperpush-mono/mesher/client/` dashboard package that preserves the existing package-local `dev` / `build` / `start` / `test:e2e:dev` / `test:e2e:prod` contract.
  - A rewired machine-checked external path contract across product CI, maintainer verification, product README, Dependabot, and the `mesh-lang` root Playwright harness.
  - Fresh dev and built-production direct-entry route parity proof from the final package path, including the unknown-path Issues fallback and clean console/request signals.
  - Recorded root-harness and live-runtime gotchas in `.gsd/KNOWLEDGE.md` and the slice decisions ledger so downstream cleanup slices can reuse the same truthful verification seam.
requires:
  - slice: M059/S02
    provides: The proven TanStack route tree, shared dashboard shell/state model, package-local parity suite, and preserved mock-data client-state contract that S03 moved without reopening.
affects:
  - M059/S04
key_files:
  - ../hyperpush-mono/mesher/client/package.json
  - ../hyperpush-mono/mesher/client/server.mjs
  - ../hyperpush-mono/mesher/client/playwright.config.ts
  - ../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts
  - ../hyperpush-mono/mesher/client/README.md
  - ../hyperpush-mono/.github/workflows/ci.yml
  - ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh
  - ../hyperpush-mono/README.md
  - ../hyperpush-mono/.github/dependabot.yml
  - playwright.config.ts
  - .gsd/PROJECT.md
key_decisions:
  - D496 — Keep the truthful package-local Node bridge server for `npm run start` over the built TanStack `dist/` output instead of pretending the current build emits Nitro `.output/`.
  - D503 — Treat S03 as a path-and-contract migration: move the already-proven TanStack package wholesale to `../hyperpush-mono/mesher/client/` and update only the machine-checked external caller surfaces.
  - D504 — Mirror the package-local `PLAYWRIGHT_PROJECT` / `npm_config_project` selection logic in `mesh-lang/playwright.config.ts` so the root harness targets `../hyperpush-mono/mesher/client` without booting both environments.
patterns_established:
  - When a frontend migration’s route/UI parity is already proven, move the package wholesale to its canonical path, prune generated artifacts, and keep the runtime/test tree structurally intact unless the rename exposes a real break.
  - Mirror package-local Playwright project-selection logic in any cross-repo/root harnesses instead of trusting npm CLI forwarding to isolate the requested project.
  - Use zero-match stale-path greps plus explicit root-harness load checks as part of the closeout proof whenever a path migration spans CI, docs, verifiers, and sibling-repo tooling.
observability_surfaces:
  - ../hyperpush-mono/mesher/client/playwright.config.ts
  - ../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts
  - ../hyperpush-mono/mesher/client/server.mjs
  - ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh
  - playwright.config.ts
  - Package-local dev runtime on http://127.0.0.1:3000/ and built production on http://127.0.0.1:3001/
  - Playwright console-error and failed-request tracking embedded in the shared parity suite
drill_down_paths:
  - .gsd/milestones/M059/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M059/slices/S03/tasks/T02-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-11T18:21:53.931Z
blocker_discovered: false
---

# S03: Finalize move to `mesher/client` and remove Next.js runtime path

**Moved the TanStack dashboard to canonical `../hyperpush-mono/mesher/client/`, rewired the machine-checked cross-repo contract to that path, and re-proved dev and built-production route parity without putting Next.js back on the runtime path.**

## What Happened

# S03: Canonical `mesher/client` cutover and runtime contract closure

**Moved the TanStack dashboard from `../hyperpush-mono/mesher/frontend-exp/` to canonical `../hyperpush-mono/mesher/client/`, rewired the machine-checked cross-repo contract, and re-proved dev and built-production route parity without restoring Next.js to the runtime path.**

## What Happened

S03 closed the path-and-contract migration promised by the milestone without reopening the already-proven route/state architecture from S02. The real TanStack dashboard package was moved wholesale into `../hyperpush-mono/mesher/client/`, disposable generated output was pruned from the canonical tree, and the package README was rewritten to document the truthful TanStack/Vite runtime plus package-local `dev` / `build` / `start` / `test:e2e:*` contract. The package-local runtime files stayed structurally intact: `server.mjs` still bridges the built `dist/client` and `dist/server/server.js` output, the shared dashboard route-parity suite still owns direct-entry and interaction proof, and the app remained on the existing mock-data/client-state contract.

The slice then rewired the surrounding machine-checked caller surfaces that still hardcoded `frontend-exp`. Product CI now installs and builds `mesher/client`, the product maintainer verifier now requires `mesher/client` markers, the product README documents `mesher/client` as the canonical dashboard package while leaving `mesher/landing` as the intentional Next.js app, Dependabot scopes npm updates to `/mesher/client`, and `mesh-lang/playwright.config.ts` now points at `../hyperpush-mono/mesher/client` while mirroring the package-local `PLAYWRIGHT_PROJECT` / `npm_config_project` selection logic so root-level callers only boot the requested environment.

The runtime and parity contract were then re-proved from the new path. Build passed from `mesher/client`. The package-local dev and built-production Playwright suites each passed all 7 route-parity tests, including `/`, `/performance`, `/solana-programs`, `/releases`, `/alerts`, `/bounties`, `/treasury`, `/settings`, and the unknown-path Issues fallback. The shared parity spec continued to assert zero console errors and zero failed requests, so the slice not only renamed the package but also proved that the moved package still behaves like the already-migrated S02 dashboard. The root cross-repo Playwright harness also loaded successfully from `mesh-lang` against the moved package path.

## Verification

Passed the full slice-plan rail:

- `npm --prefix ../hyperpush-mono/mesher/client run build`
- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev`
- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod`
- `! rg -n "mesher/frontend-exp|frontend-exp" ../hyperpush-mono/.github/workflows/ci.yml ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh ../hyperpush-mono/README.md ../hyperpush-mono/.github/dependabot.yml ./playwright.config.ts`
- `rg -n "mesher/client|client" ../hyperpush-mono/.github/workflows/ci.yml ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh ../hyperpush-mono/README.md ../hyperpush-mono/.github/dependabot.yml ./playwright.config.ts`

Additional observability and contract checks also passed:

- `bash -n ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`
- `PLAYWRIGHT_PROJECT=dev npx --prefix ../hyperpush-mono/mesher/client playwright test --config ./playwright.config.ts --project=dev --list`

The dev and prod parity suites both passed all 7 tests and the shared spec reported zero console errors and zero failed requests.

## Requirements Advanced

- **R143** — Re-proved the migrated dashboard from its final canonical package path while keeping the same visible shell, headings, direct-entry routes, fallback behavior, and mock-data interactions that S02 established.
- **R145** — Re-proved URL, navigation, Issues leave-and-return behavior, AI panel behavior, settings chrome behavior, and unknown-path fallback from `mesher/client` in both dev and built production.
- **R146** — Kept the move entirely on the existing mock-data/client-state contract: no TanStack loaders, no server functions, no Mesher backend calls, and no widened URL/search-param semantics were introduced.
- **R148** — Advanced the direct operational cleanup by rewiring the highest-signal machine-checked docs/workflow/config surfaces to `mesher/client`; S04 can now focus on broader equivalence proof and any residual non-critical stale references.

## Requirements Validated

- **R144** — `mesher/client` is now the canonical dashboard package path and still exposes the same external `dev` / `build` / `start` contract.
- **R147** — The migrated app now builds and starts from `mesher/client` under TanStack Start’s current Vite build plus the package-local Node bridge server, with Next.js removed from the critical runtime path.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Operational Readiness (Q8)

### Health signal
- `npm --prefix ../hyperpush-mono/mesher/client run build` completes successfully and emits `dist/client` plus `dist/server/server.js`.
- `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev` and `... run test:e2e:prod` each pass all 7 route-parity tests.
- The shared Playwright parity suite reports zero console errors and zero failed requests while exercising direct-entry routes and navigation.
- `PLAYWRIGHT_PROJECT=dev npx --prefix ../hyperpush-mono/mesher/client playwright test --config ./playwright.config.ts --project=dev --list` loads the root cross-repo harness against `mesher/client` and lists only the requested project.

### Failure signal
- Any `frontend-exp` hit in the stale-path grep means the external caller contract drifted back to the old package path.
- Playwright console/request failures indicate route parity or production-bridge regressions even if the page still renders.
- `node server.mjs` / `npm run start` failures will surface as the prod parity suite failing to boot the built app.
- Maintainer-verifier marker mismatches or syntax errors in `verify-maintainer-surface.sh` indicate CI/docs/verification surfaces have drifted away from the canonical path contract.

### Recovery procedure
1. Re-run `npm --prefix ../hyperpush-mono/mesher/client run build` to confirm the package still emits the expected `dist/` layout.
2. Re-run `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev` and `... run test:e2e:prod` to localize whether the break is dev-only, prod-only, or shared.
3. If the failure is root-harness-specific, validate from `mesh-lang` with `PLAYWRIGHT_PROJECT=dev npx --prefix ../hyperpush-mono/mesher/client playwright test --config ./playwright.config.ts --project=dev --list` so the moved package’s own Playwright binary resolves the sibling suite.
4. If startup fails only in production, inspect `../hyperpush-mono/mesher/client/server.mjs` plus the emitted `dist/client` and `dist/server/server.js` output rather than assuming a TanStack route regression.
5. If path-contract checks fail, inspect `../hyperpush-mono/.github/workflows/ci.yml`, `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`, `../hyperpush-mono/README.md`, `../hyperpush-mono/.github/dependabot.yml`, and `./playwright.config.ts` for stale `frontend-exp` references.

### Monitoring gaps
- There is still no deployed runtime telemetry or hosted health endpoint for the dashboard package; the authoritative readiness surface is local build/start plus Playwright parity.
- Production-start proof still depends on the package-local Node bridge server over built `dist/` output rather than a broader deployment-runtime integration story.
- S04 should finish the final equivalence pass and clean up any remaining non-critical stale guidance outside the task-owned machine-checked surfaces.

## Deviations

Added two lightweight checks beyond the written slice rail: `bash -n ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh` and the root-harness `--list` check from `mesh-lang`. Both were kept because they directly validate the observability/diagnostic surfaces named in the slice plan.

## Known Limitations

The dashboard still intentionally runs on mock data and client state only; S03 did not widen into backend integration, TanStack loaders, or server functions. The current production runtime remains the truthful package-local Node bridge over TanStack Start’s built `dist/` output rather than a different deployment target. Broader non-critical stale-reference cleanup remains S04 work.

## Follow-ups

- S04 should reuse the `mesher/client` parity rails for final equivalence proof and broader direct operational cleanup.
- If the product later wants a different deployment target than the current `server.mjs` bridge, treat that as a separate deployment/runtime decision rather than part of this path-migration slice.

## Files Created/Modified

- `../hyperpush-mono/mesher/client/README.md` — rewrote the package runbook around the truthful `mesher/client` TanStack/Vite runtime contract.
- `../hyperpush-mono/mesher/client/server.mjs` — preserved the package-local production bridge that serves built assets and forwards app requests to the TanStack server handler.
- `../hyperpush-mono/mesher/client/playwright.config.ts` — preserved isolated dev/prod parity project selection from the moved canonical package root.
- `../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts` — remained the authoritative direct-entry and interaction parity suite proving the moved package path.
- `../hyperpush-mono/.github/workflows/ci.yml` — rewired install/build surfaces from `frontend-exp` to `mesher/client`.
- `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh` — rewired maintainer-surface verification markers to `mesher/client`.
- `../hyperpush-mono/README.md` — updated the product-repo maintainer contract to name `mesher/client` as the canonical dashboard path.
- `../hyperpush-mono/.github/dependabot.yml` — repointed npm update scope to `/mesher/client`.
- `playwright.config.ts` — mirrored package-local project selection and repointed the root harness to `../hyperpush-mono/mesher/client`.
- `.gsd/PROJECT.md` — refreshed project state to reflect S03 completion and the canonical `mesher/client` runtime path.


## Verification

Slice-level verification passed end to end. `npm --prefix ../hyperpush-mono/mesher/client run build`, `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev`, and `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod` all succeeded. The stale-path grep returned zero matches across the task-owned machine-checked surfaces, the positive `mesher/client|client` grep returned the expected updated markers, `bash -n ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh` passed, and `PLAYWRIGHT_PROJECT=dev npx --prefix ../hyperpush-mono/mesher/client playwright test --config ./playwright.config.ts --project=dev --list` confirmed the root cross-repo harness loads the moved package path. Both parity suites passed all 7 tests with zero console errors and zero failed requests.

## Requirements Advanced

- R143 — Re-proved the migrated dashboard from its final canonical package path while preserving the same visible shell, route set, fallback behavior, and mock-data interactions established in S02.
- R145 — Re-proved direct-entry URLs, navigation, Issues leave-and-return behavior, AI-panel behavior, settings chrome behavior, and unknown-path fallback from `mesher/client` in both dev and built production.
- R146 — Kept the move entirely on the existing mock-data/client-state contract with no loaders, server functions, backend calls, or widened search-param semantics.
- R148 — Advanced the highest-signal direct operational cleanup by rewiring machine-checked docs/workflow/config surfaces from `frontend-exp` to `mesher/client`, leaving only broader non-critical cleanup for S04.

## Requirements Validated

- R144 — Validated by successful `mesher/client` build/dev/prod parity runs plus zero stale `frontend-exp` hits across the machine-checked path-contract surfaces.
- R147 — Validated by successful `mesher/client` build/start parity proof and root-harness load validation, showing Next.js is no longer on the critical runtime path.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Added `bash -n` on the maintainer verifier and a root-harness `--list` load check beyond the written slice rail. Both were intentional because the slice plan explicitly named those surfaces as observability/diagnostic boundaries.

## Known Limitations

The dashboard still intentionally runs on mock data and client state only. The current production runtime remains the truthful package-local Node bridge over TanStack Start’s built `dist/` output, and broader non-critical stale-reference cleanup remains S04 work.

## Follow-ups

S04 should reuse the `mesher/client` dev/prod parity rails and root harness for final equivalence proof and any broader stale-reference cleanup outside the task-owned machine-checked surfaces.

## Files Created/Modified

- `../hyperpush-mono/mesher/client/README.md` — Rewrote the package runbook to describe the canonical TanStack/Vite runtime and package-local command contract at `mesher/client`.
- `../hyperpush-mono/mesher/client/server.mjs` — Preserved the truthful production bridge that serves built static assets and forwards app requests to the TanStack server handler from the moved package root.
- `../hyperpush-mono/.github/workflows/ci.yml` — Repointed product CI dependency install/build steps and cache inputs from `mesher/frontend-exp` to `mesher/client`.
- `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh` — Updated maintainer-surface marker checks and delegated verification to the `mesher/client` package contract.
- `../hyperpush-mono/README.md` — Updated product-repo maintainer guidance so `mesher/client` is the canonical dashboard surface and `mesher/landing` remains the intentional Next.js app.
- `../hyperpush-mono/.github/dependabot.yml` — Moved the npm update scope from `/mesher/frontend-exp` to `/mesher/client`.
- `playwright.config.ts` — Mirrored package-local project selection and pointed the root cross-repo Playwright harness at `../hyperpush-mono/mesher/client`.
- `.gsd/PROJECT.md` — Refreshed project state to reflect S03 completion and the canonical `mesher/client` path.
