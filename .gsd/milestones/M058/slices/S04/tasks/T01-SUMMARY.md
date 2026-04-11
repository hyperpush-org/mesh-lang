---
id: T01
parent: S04
milestone: M058
key_files:
  - ../hyperpush-mono/mesher/api/dashboard.mpl
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/normalize.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/normalize.test.ts
  - ../hyperpush-mono/mesher/frontend-exp/src/routes/index.test.tsx
  - ../hyperpush-mono/mesher/frontend-exp/scripts/verify-s02-live-detail-alerts.mjs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Normalize issue-timeline rows through the typed `Event` JSON rail instead of direct `Map.get(...)` interpolation.
  - Keep backend threshold semantics unchanged and make the verifier seed two events for a temporary `threshold: 1` rule so the proof rail matches runtime truth.
duration:
verification_result: passed
completed_at: 2026-04-11T04:41:06.078Z
blocker_discovered: false
---

# T01: Fixed Mesher issue-timeline serialization, hardened the live replay verifier, and added regression coverage for endpoint-scoped detail failures.

**Fixed Mesher issue-timeline serialization, hardened the live replay verifier, and added regression coverage for endpoint-scoped detail failures.**

## What Happened

Reproduced the live `/api/v1/issues/:issue_id/timeline` seam against a real Mesher runtime and confirmed the route was returning pointer-like placeholder strings instead of truthful ids/levels/messages/timestamps. Fixed the runtime inside `../hyperpush-mono/mesher/api/dashboard.mpl` by normalizing timeline rows through the typed `Event` JSON rail before serializing response fields. Tightened the frontend timeline contract in `normalize.ts`, added focused regression coverage in `normalize.test.ts`, added a route-level `endpoint=issue-timeline` failure test in `src/routes/index.test.tsx`, and hardened `verify-s02-live-detail-alerts.mjs` so dishonest timeline rows fail closed. While replaying the live verifier, found the threshold-alert seed path was flaky because the backend fires on `event_count > threshold`; kept backend behavior unchanged and updated the verifier to seed two events for its temporary `threshold: 1` rule.

## Verification

Ran the focused Vitest suite for Mesher normalization and route behavior, reran Mesher migrations, replayed the Mesher smoke rail, and then executed the live S02 verifier against a freshly rebuilt Mesher binary on `http://127.0.0.1:18080`. The live timeline endpoint returned truthful UUID-backed rows after the fix and the replay verifier passed cleanly.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/lib/mesher/normalize.test.ts src/routes/index.test.tsx` | 0 | ✅ pass | 10299ms |
| 2 | `DATABASE_URL=${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5432/mesher} bash ../hyperpush-mono/mesher/scripts/migrate.sh up` | 0 | ✅ pass | 880ms |
| 3 | `DATABASE_URL=${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5432/mesher} bash ../hyperpush-mono/mesher/scripts/smoke.sh` | 0 | ✅ pass | 5501ms |
| 4 | `MESHER_BASE_URL=http://127.0.0.1:18080 node ../hyperpush-mono/mesher/frontend-exp/scripts/verify-s02-live-detail-alerts.mjs` | 0 | ✅ pass | 671ms |

## Deviations

None.

## Known Issues

`../hyperpush-mono/mesher/frontend-exp/src/routes/index.test.tsx` still emits the pre-existing jsdom warning `In HTML, <html> cannot be a child of <div>.` during test runs. It did not affect this task’s pass/fail status.

## Files Created/Modified

- `../hyperpush-mono/mesher/api/dashboard.mpl`
- `../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/normalize.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/lib/mesher/normalize.test.ts`
- `../hyperpush-mono/mesher/frontend-exp/src/routes/index.test.tsx`
- `../hyperpush-mono/mesher/frontend-exp/scripts/verify-s02-live-detail-alerts.mjs`
- `.gsd/KNOWLEDGE.md`
