---
id: T01
parent: S01
milestone: M059
key_files:
  - .gsd/milestones/M059/slices/S01/tasks/T01-PLAN.md
  - .gsd/KNOWLEDGE.md
  - .gsd/DECISIONS.md
key_decisions:
  - D494: map `app/layout.tsx` to `src/routes/__root.tsx` and the current dashboard shell from `app/page.tsx` to `src/routes/index.tsx` during the in-place TanStack Start migration.
duration: 
verification_result: passed
completed_at: 2026-04-11T07:03:40.645Z
blocker_discovered: false
---

# T01: Captured the real Next-specific seam inventory and mapped each seam to its in-place TanStack Start/Vite replacement so S01 can migrate `frontend-exp` without drifting into a parallel app rewrite.

**Captured the real Next-specific seam inventory and mapped each seam to its in-place TanStack Start/Vite replacement so S01 can migrate `frontend-exp` without drifting into a parallel app rewrite.**

## What Happened

Inspected the live `../hyperpush-mono/mesher/frontend-exp` root/framework files (`package.json`, `next.config.mjs`, `app/page.tsx`, `app/layout.tsx`, `app/globals.css`, `tsconfig.json`, `postcss.config.mjs`, `tailwind.config.ts`, and `components.json`) and updated `.gsd/milestones/M059/slices/S01/tasks/T01-PLAN.md` with a concrete seam inventory plus target file/module mapping. The audit confirmed the real authored Next lock-in is concentrated in the runtime scripts/dependency, the app-router root, Next-only config/types files, and a few tooling assumptions, while the dashboard component tree is mostly plain React. It also confirmed there are no active authored `next/link`, `next/image`, or `next/navigation` imports in the dashboard/component source tree. Recorded decision D494 to keep document plumbing in `src/routes/__root.tsx` and map the current dashboard shell to `src/routes/index.tsx` as the parity adapter for later route decomposition, and added the non-obvious migration note to `.gsd/KNOWLEDGE.md`.

## Verification

Validated the audit with explicit checks: (1) a plan-content assertion proved the updated task plan names the required current seams and TanStack targets, (2) a local-seam assertion proved the expected Next root/config dependencies still exist in the authored source tree, and (3) an authored-source scan proved there is no hidden `next/link` / `next/image` / `next/navigation` usage in the dashboard/component source tree once generated/vendor directories and `next-env.d.ts` are excluded.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `python3 - <<'PY' ... audit-plan-check ... PY` | 0 | ✅ pass | 0ms |
| 2 | `python3 - <<'PY' ... local-seam-check ... PY` | 0 | ✅ pass | 1ms |
| 3 | `python3 - <<'PY' ... authored-source-next-import-scan ... PY` | 0 | ✅ pass | 523ms |

## Deviations

Expanded the audit beyond the seed files to include `tsconfig.json`, `next-env.d.ts`, `postcss.config.mjs`, `tailwind.config.ts`, and `components.json` because they are real migration seams in the live tree. Also documented `src/routes/index.tsx` as a required target mapping for the current dashboard shell even though T02’s file list only explicitly named `src/routes/__root.tsx` and `src/router.tsx`.

## Known Issues

None.

## Files Created/Modified

- `.gsd/milestones/M059/slices/S01/tasks/T01-PLAN.md`
- `.gsd/KNOWLEDGE.md`
- `.gsd/DECISIONS.md`
