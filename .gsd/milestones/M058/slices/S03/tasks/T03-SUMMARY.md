---
id: T03
parent: S03
milestone: M058
key_files:
  - ../hyperpush-mono/mesher/frontend-exp/BACKEND-GAP-LEDGER.md
  - ../hyperpush-mono/mesher/frontend-exp/scripts/verify-s03-supported-admin.mjs
  - ../hyperpush-mono/mesher/frontend-exp/package.json
  - ../hyperpush-mono/mesher/api/team.mpl
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Serialize Mesher API-key list rows through a typed ApiKey record plus Json.encode(...) instead of manual string interpolation so live UUID/label/timestamp fields round-trip truthfully.
  - Use the dedicated S03 replay verifier as the authoritative live admin proof surface while logging only masked API-key previews, ids, labels, statuses, and endpoint names.
duration: 
verification_result: passed
completed_at: 2026-04-11T03:56:47.756Z
blocker_discovered: false
---

# T03: Published the checked-in backend gap ledger, added the redacted S03 admin replay verifier, and fixed the live API-key list serializer so the replay closes against a truthful contract.

**Published the checked-in backend gap ledger, added the redacted S03 admin replay verifier, and fixed the live API-key list serializer so the replay closes against a truthful contract.**

## What Happened

Added ../hyperpush-mono/mesher/frontend-exp/BACKEND-GAP-LEDGER.md with the four required classification headings and concrete entries for missing route families, deferred team membership discovery, API-key response-shape mismatch, and removed frontend-only shell affordances. Added ../hyperpush-mono/mesher/frontend-exp/scripts/verify-s03-supported-admin.mjs plus the verify:s03-supported-admin package script to check the ledger headings, guard the active path against fake/mock-era identifiers, and replay real Mesher API-key list/create/revoke behavior with secret-redacted output and cleanup-aware revoke logic. During live replay debugging, found that ../hyperpush-mono/mesher/api/team.mpl was serializing API-key list rows incorrectly via manual string interpolation; fixed it by normalizing rows through a typed ApiKey record and Json.encode(...), then rebuilt and reran the full slice verification chain to green. Also recorded the serializer choice in DECISIONS.md and the replay/smoke gotcha in .gsd/KNOWLEDGE.md.

## Verification

Passed the full slice verification chain after fixing the live API-key serializer: the focused Vitest suite passed, the frontend production build passed, mesher/scripts/migrate.sh up passed against a disposable local Docker Postgres 16 instance, mesher/scripts/smoke.sh passed against the rebuilt Mesher runtime, and MESHER_BASE_URL=http://127.0.0.1:18080 npm --prefix ../hyperpush-mono/mesher/frontend-exp run verify:s03-supported-admin passed with a redacted create/list/revoke replay plus ledger-heading and active-path honesty checks.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run test -- --run src/lib/mesher/normalize.test.ts src/lib/dashboard/search.test.ts src/routes/index.test.tsx src/routes/__root.test.tsx components/dashboard/live-shell.test.tsx components/dashboard/live-alerts-settings.test.tsx components/dashboard/live-admin-settings.test.tsx` | 0 | ✅ pass | 8348ms |
| 2 | `npm --prefix ../hyperpush-mono/mesher/frontend-exp run build` | 0 | ✅ pass | 4539ms |
| 3 | `DATABASE_URL=${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5432/mesher} bash ../hyperpush-mono/mesher/scripts/migrate.sh up` | 0 | ✅ pass | 767ms |
| 4 | `DATABASE_URL=${DATABASE_URL:-postgres://postgres:postgres@127.0.0.1:5432/mesher} bash ../hyperpush-mono/mesher/scripts/smoke.sh` | 0 | ✅ pass | 5938ms |
| 5 | `MESHER_BASE_URL=http://127.0.0.1:18080 npm --prefix ../hyperpush-mono/mesher/frontend-exp run verify:s03-supported-admin` | 0 | ✅ pass | 799ms |

## Deviations

None beyond local verification environment bootstrapping: used a disposable Docker Postgres 16 container because nothing native was listening on 127.0.0.1:5432.

## Known Issues

None.

## Files Created/Modified

- `../hyperpush-mono/mesher/frontend-exp/BACKEND-GAP-LEDGER.md`
- `../hyperpush-mono/mesher/frontend-exp/scripts/verify-s03-supported-admin.mjs`
- `../hyperpush-mono/mesher/frontend-exp/package.json`
- `../hyperpush-mono/mesher/api/team.mpl`
- `.gsd/KNOWLEDGE.md`
