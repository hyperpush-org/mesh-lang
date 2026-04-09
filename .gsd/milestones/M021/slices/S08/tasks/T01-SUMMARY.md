---
id: T01
parent: S08
milestone: M021
provides:
  - 12 search/dashboard/detail/team query functions rewritten to ORM APIs
  - 7 ORM boundary comments documenting why specific queries retain raw SQL
  - parse_limit helper for String-to-Int limit parameter conversion
requires: []
affects: []
key_files: []
key_decisions: []
patterns_established: []
observability_surfaces: []
drill_down_paths: []
duration: 5min
verification_result: passed
completed_at: 2026-02-18
blocker_discovered: false
---
# T01: 112-mesher-rewrite-search-dashboard-and-alerts 01

**# Phase 112 Plan 01: Rewrite Search/Dashboard/Detail/Team Queries Summary**

## What Happened

# Phase 112 Plan 01: Rewrite Search/Dashboard/Detail/Team Queries Summary

**12 query functions rewritten from raw SQL to ORM Query/Repo pipe chains with 7 ORM boundary rationale comments for complex queries**

## Performance

- **Duration:** 5 min
- **Started:** 2026-02-18T03:06:55Z
- **Completed:** 2026-02-18T03:12:05Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- Rewrote 12 query functions from Repo.query_raw/execute_raw to ORM APIs (Query.from pipe chains + Repo.all/Repo.update_where)
- Added 7 ORM boundary comments documenting why specific queries retain raw SQL (parameter binding in SELECT, scalar subqueries, server-side now())
- Created parse_limit helper function for safe String-to-Int limit conversion with default 25
- Mesher compiles successfully with all rewrites

## Task Commits

Each task was committed atomically:

1. **Task 1: Rewrite search, dashboard, detail, and team queries to ORM** - `73ce2abc` (feat)
2. **Task 2: Compile and verify query rewrites** - `cf84cf75` (fix)

## Files Created/Modified
- `mesher/storage/queries.mpl` - 12 query functions rewritten to ORM, 7 boundary comments added, parse_limit helper

## Decisions Made
- acknowledge_alert and resolve_fired_alert retain Repo.execute_raw because SET acknowledged_at/resolved_at = now() requires a PG server-side function call that Map<String,String> cannot express
- event_volume_hourly string-interpolates the bucket parameter into date_trunc expression rather than using a bound parameter, since the caller validates it to only "hour" or "day"
- Inline `let x = case ... end` assignment is not supported by the Mesh parser; extracted parse_limit as a helper function following the parse_event_count pattern

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Mesh parser does not support inline let = case ... end**
- **Found during:** Task 2 (Compile and verify)
- **Issue:** `let lim = case String.to_int(limit_str) do ... end` caused parse errors at line 474
- **Fix:** Created `parse_limit` helper function following the existing `parse_event_count` pattern; replaced all 5 inline case blocks with `parse_limit(limit_str)` calls
- **Files modified:** mesher/storage/queries.mpl
- **Verification:** `meshc build mesher` compiles successfully
- **Committed in:** cf84cf75 (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Auto-fix necessary for compilation. Cleaner code than inline case blocks. No scope creep.

## Issues Encountered
None beyond the parse error fixed above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All search, dashboard, detail, and team queries now use ORM APIs or have documented boundaries
- 33 remaining Repo.query_raw/execute_raw calls in queries.mpl (alert system, retention, auth crypto, and boundary-documented queries)
- Ready for Plan 02 (remaining alert and retention query rewrites)

## Self-Check: PASSED

All files and commits verified.

---
*Phase: 112-mesher-rewrite-search-dashboard-and-alerts*
*Completed: 2026-02-18*
