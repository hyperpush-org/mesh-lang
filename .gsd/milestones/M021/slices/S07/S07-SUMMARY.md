---
id: S07
parent: M021
milestone: M021
provides:
  - 10 issue management query functions rewritten to ORM APIs
  - Zero Repo.execute_raw for issue status transitions (except assign_issue NULL branch)
  - Issue listing and count queries use Query.where + Query.select_raw + Repo.all
  - 4 complex queries documented with ORM boundary rationale
  - All 14 issue + event queries addressed: 10 ORM + 4 documented raw SQL
requires: []
affects: []
key_files: []
key_decisions:
  - assign_issue retains Repo.execute_raw for NULL unassign branch since ORM Map<String,String> cannot represent NULL values
  - count_unresolved_issues combines WHERE conditions in single where_raw call for readability
  - upsert_issue retains raw SQL: ORM upsert cannot express event_count + 1 arithmetic or CASE status conditionals
  - check_volume_spikes retains raw SQL: ORM cannot express nested subquery with JOIN + HAVING + GREATEST + interval arithmetic
  - insert_event retains raw SQL: Repo.insert cannot express server-side JSONB extraction (j->>'field') in INSERT...SELECT
  - extract_event_fields retains raw SQL: ORM fragments cannot express CASE/jsonb_array_elements/string_agg fingerprint computation chain
patterns_established:
  - Repo.update_where with map literal for simple SET column = value updates
  - Two-step Repo.delete_where for FK-constrained cascading deletes
  - Query.select_raw with aggregate expressions (count(*)) for count queries
  - ORM boundary comment format: multi-line comment explaining specific ORM limitation, ending with 'Intentional raw SQL'
observability_surfaces: []
drill_down_paths: []
duration: 1min
verification_result: passed
completed_at: 2026-02-18
blocker_discovered: false
---
# S07: Mesher Rewrite Issues And Events

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

# Phase 111 Plan 02: Document ORM Boundaries for Complex Issue and Event Queries Summary

**4 complex queries (upsert_issue, check_volume_spikes, insert_event, extract_event_fields) documented with ORM boundary rationale explaining why each exceeds ORM expressiveness**

## Performance

- **Duration:** 1 min
- **Started:** 2026-02-18T02:07:23Z
- **Completed:** 2026-02-18T02:08:55Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Documented upsert_issue: ORM upsert cannot express arithmetic SET (event_count + 1) or CASE status conditionals
- Documented check_volume_spikes: ORM cannot express nested subquery with JOIN + HAVING + GREATEST + interval arithmetic
- Documented insert_event: Repo.insert cannot express server-side JSONB extraction in INSERT...SELECT pattern
- Documented extract_event_fields: ORM fragments cannot express CASE/jsonb_array_elements/string_agg fingerprint chain
- All 14 issue + event queries now addressed: 10 rewritten to ORM (Plan 01) + 4 documented raw SQL (Plan 02)

## Task Commits

Each task was committed atomically:

1. **Task 1: Document upsert_issue and check_volume_spikes** - `b0e04b1c` (docs)
2. **Task 2: Document insert_event and extract_event_fields** - `fecb6406` (docs)

## Files Created/Modified
- `mesher/storage/queries.mpl` - ORM boundary documentation added to upsert_issue, check_volume_spikes, and extract_event_fields
- `mesher/storage/writer.mpl` - ORM boundary documentation added to insert_event

## Decisions Made
- All 4 queries retain raw SQL as the correct approach -- the ORM does not have expressiveness for these patterns (arithmetic updates, nested subqueries, server-side JSONB extraction, complex computation chains)
- Documentation follows consistent format: multi-line comment explaining the specific ORM limitation, ending with "Intentional raw SQL"

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 111 complete: all 14 issue + event queries addressed
- REWR-02 (issue management) and REWR-07 (event writer/extraction) fulfilled
- Ready for Phase 112 (next ORM rewrite phase)

## Self-Check: PASSED

All files and commits verified.

---
*Phase: 111-mesher-rewrite-issues-and-events*
*Completed: 2026-02-18*
