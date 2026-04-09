---
id: T02
parent: S07
milestone: M021
provides:
  - 4 complex queries documented with ORM boundary rationale
  - All 14 issue + event queries addressed: 10 ORM + 4 documented raw SQL
requires: []
affects: []
key_files: []
key_decisions: []
patterns_established: []
observability_surfaces: []
drill_down_paths: []
duration: 1min
verification_result: passed
completed_at: 2026-02-18
blocker_discovered: false
---
# T02: 111-mesher-rewrite-issues-and-events 02

**# Phase 111 Plan 02: Document ORM Boundaries for Complex Issue and Event Queries Summary**

## What Happened

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
