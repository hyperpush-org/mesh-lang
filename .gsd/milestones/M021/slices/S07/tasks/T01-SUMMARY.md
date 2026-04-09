---
id: T01
parent: S07
milestone: M021
provides:
  - 10 issue management query functions rewritten to ORM APIs
  - Zero Repo.execute_raw for issue status transitions (except assign_issue NULL branch)
  - Issue listing and count queries use Query.where + Query.select_raw + Repo.all
requires: []
affects: []
key_files: []
key_decisions: []
patterns_established: []
observability_surfaces: []
drill_down_paths: []
duration: 7min
verification_result: passed
completed_at: 2026-02-18
blocker_discovered: false
---
# T01: 111-mesher-rewrite-issues-and-events 01

**# Phase 111 Plan 01: Rewrite Issue Management Queries Summary**

## What Happened

# Phase 111 Plan 01: Rewrite Issue Management Queries Summary

**10 issue management queries rewritten from raw SQL to ORM Repo.update_where, Repo.delete_where, and Query.where + Repo.all APIs**

## Performance

- **Duration:** 7 min
- **Started:** 2026-02-18T01:56:59Z
- **Completed:** 2026-02-18T02:04:26Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- Rewrote 4 status transition functions (resolve, archive, unresolve, discard) to use Repo.update_where
- Rewrote assign_issue to use Repo.update_where for assign branch (NULL branch retains execute_raw)
- Rewrote delete_issue to use two-step Repo.delete_where (events then issues)
- Rewrote is_issue_discarded to use Query.where + Repo.all pattern
- Rewrote 3 listing/count functions (list_issues_by_status, count_unresolved_issues, get_issue_project_id) to use Query.from + Query.where + Repo.all

## Task Commits

Each task was committed atomically:

1. **Task 1: Rewrite issue status transition and CRUD queries** - `f5d2b804` (feat)
2. **Task 2: Rewrite issue listing and count queries** - `ee2390cb` (feat)

## Files Created/Modified
- `mesher/storage/queries.mpl` - 10 issue management query functions rewritten to use ORM APIs

## Decisions Made
- assign_issue retains one Repo.execute_raw for the NULL unassign branch, since ORM Map<String,String> values cannot represent SQL NULL. This is acceptable and documented in the plan.
- count_unresolved_issues combines project_id and status conditions in a single where_raw call for readability.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All 10 issue management functions now use ORM APIs
- Only upsert_issue and check_volume_spikes remain as raw SQL in the issue domain (Plan 02 scope)
- Ready for Plan 02 (event queries and remaining complex issue queries)

## Self-Check: PASSED

All files and commits verified.

---
*Phase: 111-mesher-rewrite-issues-and-events*
*Completed: 2026-02-18*
