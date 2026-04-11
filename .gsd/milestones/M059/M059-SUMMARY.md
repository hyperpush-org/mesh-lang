---
id: M059
title: "Frontend Framework Migration to TanStack Start"
status: complete
completed_at: 2026-04-11T19:23:26.082Z
key_decisions:
  - D494/D495 — Map the old Next document/page split onto TanStack root/index routes and keep the visible shell client-owned during groundwork.
  - D496 — Preserve the external `npm run start` contract with a package-local Node bridge over TanStack Start’s built `dist/` output.
  - D497/D498/D499/D500/D501 — Use a pathless `_dashboard` layout, shared shell/state ownership, pathname-derived navigation, and a matched root fallback for real route parity.
  - D502/D504 — Use `PLAYWRIGHT_PROJECT`-backed package scripts and mirror that project-selection logic in the root Playwright harness instead of trusting npm CLI forwarding.
  - D503 — Move the already-proven TanStack package wholesale to `../hyperpush-mono/mesher/client/` and rewire machine-checked callers around that canonical path.
key_files:
  - ../hyperpush-mono/mesher/client/package.json
  - ../hyperpush-mono/mesher/client/server.mjs
  - ../hyperpush-mono/mesher/client/src/routes/__root.tsx
  - ../hyperpush-mono/mesher/client/src/routes/_dashboard.tsx
  - ../hyperpush-mono/mesher/client/src/routes/$.tsx
  - ../hyperpush-mono/mesher/client/tests/e2e/dashboard-route-parity.spec.ts
  - ../hyperpush-mono/mesher/client/playwright.config.ts
  - ../hyperpush-mono/.github/workflows/ci.yml
  - ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh
  - ../hyperpush-mono/README.md
  - ../hyperpush-mono/AGENTS.md
  - ../hyperpush-mono/CONTRIBUTING.md
  - ../hyperpush-mono/SUPPORT.md
  - playwright.config.ts
  - AGENTS.md
lessons_learned:
  - On local `main` auto-mode closeout in this split workspace, use `origin/main` as the non-`.gsd` diff baseline and pair it with the owning repo’s diff/status checks; the literal local-`main` diff can go empty even when real product delivery exists.
  - Preserving the external `dev` / `build` / `start` contract during a framework migration matters more than preserving an internal runtime implementation; the package-local Node bridge kept the migration honest while TanStack Start emitted `dist/` output rather than a Next/Nitro layout.
  - Route/UI parity work is much easier to close confidently when shell-owned transient state and direct-entry behavior are asserted through one canonical dev/prod Playwright suite instead of ad hoc browser smoke.
  - In this workspace shape, package-local `PLAYWRIGHT_PROJECT` scripts plus a mirrored root harness are a more truthful verification seam than `npm exec playwright ... --project=...` from repo root.
---

# M059: Frontend Framework Migration to TanStack Start

**Replaced the product dashboard’s active Next.js runtime with TanStack Start, moved the canonical app to `../hyperpush-mono/mesher/client/`, and closed the migration with passing dev/prod parity proof plus updated operational guidance.**

## What Happened

M059 completed the frontend framework migration as a behavior-preserving runtime and package-path cutover instead of a redesign. S01 replaced the old Next.js-rooted plumbing in `../hyperpush-mono/mesher/frontend-exp/` with TanStack Start/Vite groundwork, preserved the visible dashboard shell, restored a truthful `dev` / `build` / `start` contract, and fixed the production runner with a package-local Node bridge over the built `dist/` output. S02 then decomposed the temporary single-route shell into real TanStack file routes under a pathless `_dashboard` layout, kept Issues search/filter/detail state shell-owned so leave-and-return behavior stayed stable, and established the route-parity Playwright rail for both dev and built production. S03 moved that already-proven package wholesale to canonical `../hyperpush-mono/mesher/client/`, rewired CI, Dependabot, maintainer verification, README guidance, and the root Playwright harness to the new path, and re-proved the build plus parity suite without restoring Next.js to the runtime path. S04 closed the remaining operational and equivalence risk by strengthening the canonical parity suite, fixing the Recharts runtime warning instead of weakening assertions, and updating the remaining direct maintainer-facing references (`AGENTS.md`, `CONTRIBUTING.md`, `SUPPORT.md`, issue templates, and root guidance) from `frontend-exp` to `mesher/client`.

Verification for closeout followed the repo-local split-workspace rules from `.gsd/KNOWLEDGE.md`. The literal `git diff HEAD $(git merge-base HEAD main) -- ':!.gsd/'` check is empty on local `main`, so the truthful baseline used `origin/main`. `git diff --stat origin/main -- ':!.gsd/'` in `mesh-lang` showed non-`.gsd` changes in `AGENTS.md` and `playwright.config.ts`, and the owning-repo equivalent in `../hyperpush-mono` showed the machine-checked docs/workflow/script rewires plus the retirement of the old `frontend-exp` tree. Because the canonical `mesher/client` package is still untracked in the sibling repo worktree, closeout also verified `git -C ../hyperpush-mono status --short mesher/client` together with targeted `find` output for the new package’s runtime, route, and parity-test files. Fresh closeout replay then passed `npm --prefix ../hyperpush-mono/mesher/client run build`, `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:dev`, `npm --prefix ../hyperpush-mono/mesher/client run test:e2e:prod`, `bash -n ../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`, the root-harness load check `PLAYWRIGHT_PROJECT=dev npx --prefix ../hyperpush-mono/mesher/client playwright test --config ./playwright.config.ts --project=dev --list`, and the scoped stale-path / positive-path grep checks across the direct operational surfaces.

### Decision re-evaluation

| Decision | Re-evaluation | Result |
|---|---|---|
| D494/D495 — map the old Next document/page split onto TanStack root/index routes and keep the visible shell client-owned during groundwork | This let the migration land in place without a redesign, and the later route decomposition plus final parity suite proved that the preserved shell was the correct bridge rather than dead-end scaffolding. | Keep |
| D496 — preserve `npm run start` with a package-local Node bridge over TanStack Start’s built `dist/` output | Still valid at closeout: the current build emits `dist/client/` plus `dist/server/server.js`, so the bridge remains the truthful way to preserve the external `start` contract. Revisit only if a future deployment/runtime target replaces this packaging shape. | Keep; revisit only if the runtime contract changes |
| D497/D498/D499/D500/D501 — use a pathless `_dashboard` layout, shared shell/state, pathname-derived active nav, and matched root fallback instead of local nav-only state | These decisions are exactly what made URL parity, Issues leave-and-return behavior, AI panel state, settings chrome, and unknown-path fallback verifiable in both dev and production without widening into backend work. | Keep |
| D502/D504 — use `PLAYWRIGHT_PROJECT`-backed wrappers and mirror that selection logic in the root Playwright harness instead of trusting npm CLI forwarding | Still required. Fresh closeout proof from `mesh-lang` depends on the mirrored root harness plus package-local scripts because current npm forwarding still makes direct `exec playwright ... --project=...` unreliable in this workspace shape. | Keep |
| D503 — treat the path move as a wholesale package-and-contract migration once route/state parity is proven | Worked as intended. S03 moved the app to `mesher/client` and rewired machine-checked callers without reopening the already-proven dashboard architecture. | Keep |

No M059 decision needs immediate reversal; the only likely future revisit point is whether D496’s bridge server can be retired if TanStack Start’s production runner contract changes.

## Success Criteria Results

- **S01 must leave the dashboard bootable under TanStack Start migration plumbing without changing the visible shell or the `dev` / `build` / `start` command contract — PASS.** S01 converted `frontend-exp` to TanStack Start/Vite groundwork, preserved the visible dashboard shell, restored a truthful `dev` / `build` / `start` contract, and recorded zero-console-error / zero-failed-request browser proof in both dev and production. The final closeout replay reconfirmed the build and package-local runtime contract from the canonical moved package.
- **S02 must replace the old internal nav-only structure with proper TanStack routes while preserving the same visible UI, URLs, panels, filters, and interactions — PASS.** S02 replaced the temporary adapter with real TanStack file routes for `/`, `/performance`, `/solana-programs`, `/releases`, `/alerts`, `/bounties`, `/treasury`, and `/settings`, while preserving shell-owned Issues state, AI panel behavior, settings chrome, and the unknown-path Issues fallback. Fresh closeout replay passed the full 9-test `dashboard-route-parity.spec.ts` suite in both dev and production.
- **S03 must finalize the move to `../hyperpush-mono/mesher/client/`, remove Next.js from the critical runtime path, and keep the app buildable/startable under the normal commands — PASS.** S03 moved the package wholesale to `mesher/client`, rewired CI, README, Dependabot, the maintainer verifier, and the root Playwright harness, and the closeout replay passed `npm --prefix ../hyperpush-mono/mesher/client run build`, `... run test:e2e:dev`, `... run test:e2e:prod`, and the root-harness `--list` load check without restoring Next.js to the runtime path.
- **S04 must prove the migrated app is visually and behaviorally equivalent in the important screens/flows and update only the direct operational references that would otherwise become stale or broken — PASS.** S04 strengthened the canonical parity suite to cover Solana Programs AI/sidebar behavior, Issues browser-history restoration, direct-entry routes, and clean runtime-signal checks, and it repointed the remaining direct maintainer-facing references (`../hyperpush-mono/AGENTS.md`, `../hyperpush-mono/CONTRIBUTING.md`, `../hyperpush-mono/SUPPORT.md`, issue templates, and `./AGENTS.md`) to `mesher/client`. Fresh closeout grep checks found no stale `frontend-exp` references in those direct surfaces and confirmed positive `mesher/client` references.
- **Horizontal checklist — none rendered in the checked-in milestone roadmap or validation artifact.** No separate horizontal-checklist block remained to audit at closeout.

## Definition of Done Results

- **All slices complete — PASS.** `gsd_milestone_status` reports S01, S02, S03, and S04 all `complete`, with task counts 3/3, 5/5, 2/2, and 2/2 respectively.
- **All slice summaries exist — PASS.** `find .gsd/milestones/M059/slices -maxdepth 2 -name 'S*-SUMMARY.md' | sort` returned `S01-SUMMARY.md`, `S02-SUMMARY.md`, `S03-SUMMARY.md`, and `S04-SUMMARY.md`.
- **Cross-slice integration works correctly — PASS.** `.gsd/milestones/M059/M059-VALIDATION.md` records PASS for the S01 → S02, S02 → S03, S02 → S04, and S03 → S04 producer/consumer boundaries, and the fresh closeout replay from canonical `mesher/client` passed build, dev parity, prod parity, maintainer-verifier shell syntax, and root-harness load checks.
- **Milestone contains real non-`.gsd` code changes — PASS.** Using the repo-local closeout baseline `origin/main`, `git diff --stat origin/main -- ':!.gsd/'` in `mesh-lang` showed non-`.gsd` changes in `AGENTS.md`, `playwright.config.ts`, and the tracked Playwright result marker; `git -C ../hyperpush-mono diff --stat origin/main` showed the workflow/docs/verifier rewires and removal of the old `frontend-exp` tree; and `git -C ../hyperpush-mono status --short mesher/client` plus targeted `find` output confirmed the new canonical `mesher/client` runtime, route, and parity-test files exist in the sibling product repo worktree.
- **Milestone validation passed before completion — PASS.** `.gsd/milestones/M059/M059-VALIDATION.md` records verdict `pass` with requirements coverage, cross-slice integration, and acceptance-criteria reviews all marked PASS.

## Requirement Outcomes

- **R143 — Active → Validated.** Supported by the final dev/prod `dashboard-route-parity.spec.ts` suite from canonical `mesher/client`, which re-proved that the migration changed the framework and package path without meaningfully changing the visible dashboard shell or key user-facing behavior.
- **R144 — Active → Validated.** Supported by the successful `mesher/client` build plus dev/prod parity runs, together with the rewired CI, README, Dependabot, maintainer verifier, and root Playwright harness references that now point at `mesher/client` instead of `frontend-exp`.
- **R145 — Active → Validated.** Supported by the 9-test parity suite in both dev and production covering URL/navigation parity, AI panel behavior, settings chrome, Issues search/filter/detail persistence, browser back/forward restoration, direct-entry routes, and unknown-path fallback.
- **R146 — Active → Validated.** Supported by the final build/dev/prod/root-harness verification passing while the app stayed on the existing mock-data/client-state contract with no TanStack loaders, server functions, Mesher backend calls, auth work, or widened URL/search-param semantics.
- **R147 — Active → Validated.** Supported by successful `npm --prefix ../hyperpush-mono/mesher/client run build`, `... run test:e2e:dev`, `... run test:e2e:prod`, and the root-harness `--list` load check, proving the migrated TanStack Start app builds, starts, and serves the final route tree without Next.js on the critical runtime path.
- **R148 — Active → Validated.** Supported by the direct-operational documentation and workflow cleanup across `../hyperpush-mono/AGENTS.md`, `../hyperpush-mono/CONTRIBUTING.md`, `../hyperpush-mono/SUPPORT.md`, the issue templates, CI, README, Dependabot, and `./AGENTS.md`, with zero stale `frontend-exp` matches in the direct maintainer surfaces and positive `mesher/client` references confirmed.
- **R149 — Deferred unchanged.** The milestone intentionally preserved mock-data/client-state behavior and left future real backend integration as later work rather than widening the migration scope.
- **R150–R152 — Guardrail requirements reconfirmed with no status change.** Closeout evidence shows no dashboard redesign, no new product features/pages/data domains, and no intentional URL or mock-data behavior changes during the migration.

## Deviations

No milestone-scope delivery deviation from the roadmap. The only closeout-process deviation was using the repo-local `origin/main` diff baseline plus owning-repo worktree checks because M059 delivery spans the split `mesh-lang` / `hyperpush-mono` workspace and the canonical `mesher/client` package currently exists as new sibling-repo worktree content rather than already-committed history.

## Follow-ups

Future product work can build real Mesher backend integration on top of the now-stable `mesher/client` shell, but it should preserve the M059 route-parity suite and root-harness checks as the regression envelope. If TanStack Start’s production packaging contract changes later, revisit D496 and retire the bridge server only when the replacement preserves an equally truthful `npm run start` contract. Any remaining indirect or historical `frontend-exp` references outside the direct maintainer surfaces can be cleaned opportunistically rather than treated as migration blockers.
