---
id: S01
parent: M021
milestone: M021
provides:
  - Query.where_not_in(q, field, values) -- NOT IN clause with parameterized values
  - Query.where_between(q, field, low, high) -- BETWEEN clause with two params
  - Query.where_or(q, fields, values) -- grouped OR conditions
  - ILIKE operator via Query.where_op(q, field, :ilike, pattern)
  - NOT_IN, BETWEEN, OR encoding formats in WHERE clause system
  - renumber_placeholders helper for $N and ? placeholder renumbering in SQL fragments
  - Query.order_by_raw(q, expression) -- raw ORDER BY expressions (random(), count DESC)
  - Query.group_by_raw(q, expression) -- raw GROUP BY expressions (date_trunc, level)
  - Fixed $N renumbering in where_raw and fragment SQL builders across all 3 code paths
  - RAW: prefix support in ORDER BY and GROUP BY SQL generation
requires: []
affects: []
key_files: []
key_decisions:
  - OR clause encoding uses OR:field1,field2,...:N format with field names embedded in clause string
  - ILIKE added as atom_to_sql_op mapping -- no new function needed, works via existing where_op
  - Unified renumber_placeholders helper handles both ? and $N styles in a single pass
  - RAW: prefix reused for ORDER BY and GROUP BY raw expressions, consistent with existing select_raw/where_raw pattern
patterns_established:
  - Multi-param WHERE clauses use count-encoded format (NOT_IN:N, BETWEEN) for SQL builder parsing
  - All raw SQL clause positions (SELECT, WHERE, ORDER BY, GROUP BY) use RAW: prefix encoding
observability_surfaces: []
drill_down_paths: []
duration: 8min
verification_result: passed
completed_at: 2026-02-17
blocker_discovered: false
---
# S01: Advanced Where Operators And Raw Sql Fragments

**# Phase 106 Plan 01: Advanced WHERE Operators Summary**

## What Happened

# Phase 106 Plan 01: Advanced WHERE Operators Summary

**NOT IN, BETWEEN, ILIKE, and OR operators added to Query builder with full pipeline (runtime, SQL gen, MIR, LLVM codegen, JIT, typechecker) and 10 new tests**

## Performance

- **Duration:** 8 min
- **Started:** 2026-02-17T20:22:45Z
- **Completed:** 2026-02-17T20:30:50Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments
- Four new WHERE clause types working end-to-end: NOT IN, BETWEEN, ILIKE (via where_op), and OR
- Correct parameter index sequencing across all clause types when combined in a single query
- Full pipeline registration across all 7 touch points (runtime, SQL gen x4, MIR, LLVM codegen, JIT, typechecker, lib exports)
- 5 new unit tests and 5 new E2E tests all passing

## Task Commits

Each task was committed atomically:

1. **Task 1: Add NOT IN, BETWEEN, ILIKE, and OR runtime + SQL generation** - `305f11ce` (feat)
2. **Task 2: Register new WHERE functions in MIR/codegen/JIT and add E2E tests** - `bd00f8ac` (feat)

## Files Created/Modified
- `crates/mesh-rt/src/db/query.rs` - Added ILIKE to atom_to_sql_op, plus mesh_query_where_not_in, mesh_query_where_between, mesh_query_where_or extern C functions
- `crates/mesh-rt/src/db/repo.rs` - Added NOT_IN, BETWEEN, OR clause handling in all 4 SQL builder WHERE parsers, plus 5 unit tests
- `crates/mesh-rt/src/lib.rs` - Re-exported three new functions
- `crates/mesh-codegen/src/mir/lower.rs` - Registered known_functions and map_builtin_name for 3 new functions
- `crates/mesh-codegen/src/codegen/intrinsics.rs` - Declared 3 LLVM intrinsics
- `crates/mesh-repl/src/jit.rs` - Registered 3 JIT symbols
- `crates/mesh-typeck/src/infer.rs` - Added type signatures for where_not_in, where_between, where_or in Query module
- `crates/meshc/tests/e2e.rs` - Added 5 E2E tests (where_not_in, where_between, where_ilike, where_or, advanced_where_combined)

## Decisions Made
- OR clause encoding uses `OR:field1,field2,...:N` format -- embeds field names directly in the clause string rather than requiring a separate encoding mechanism
- ILIKE implemented as an atom_to_sql_op mapping rather than a new function -- consistent with existing LIKE pattern and requires zero new infrastructure

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added typechecker registration for new Query functions**
- **Found during:** Task 2 (E2E tests)
- **Issue:** Plan did not mention `mesh-typeck/src/infer.rs` as a touch point, but the Query module functions must be registered in the typechecker for the compiler to recognize `Query.where_not_in`, `Query.where_between`, and `Query.where_or`
- **Fix:** Added type signatures for all three functions in the Query module type registration
- **Files modified:** `crates/mesh-typeck/src/infer.rs`
- **Verification:** All E2E tests pass after adding typechecker registrations
- **Committed in:** bd00f8ac (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Essential fix -- without typechecker registration, the compiler rejects all new function calls. No scope creep.

## Issues Encountered
- Stale build artifact caused linker errors ("Undefined symbols") even though symbols existed in libmesh_rt.a. Resolved by `cargo clean -p mesh-rt` followed by rebuild.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All four advanced WHERE operators working and tested
- Ready for Phase 106 Plan 02 (raw SQL fragments or remaining WHERE operator work)
- Parameter indexing verified correct across mixed clause types

---
*Phase: 106-advanced-where-operators-and-raw-sql-fragments*
*Completed: 2026-02-17*

# Phase 106 Plan 02: Fragment Renumbering and Raw ORDER BY/GROUP BY Summary

**Fixed $N placeholder renumbering in fragments/where_raw, added Query.order_by_raw and Query.group_by_raw with full pipeline registration and 14 new tests**

## Performance

- **Duration:** 8 min
- **Started:** 2026-02-17T20:33:32Z
- **Completed:** 2026-02-17T20:41:51Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments
- Fragment $N renumbering works correctly: `crypt($1, gen_salt('bf'))` after 2 WHERE params becomes `crypt($3, gen_salt('bf'))`
- ORDER BY raw and GROUP BY raw expressions emit verbatim SQL (random(), date_trunc, count DESC)
- All 4 SQL clause positions now support raw expressions: SELECT, WHERE, ORDER BY, GROUP BY
- JSONB operators (metadata @> $1::jsonb), PG functions (crypt, date_trunc) generate correct parameterized SQL
- 7 new unit tests and 7 new E2E tests, all 521 mesh-rt tests pass, all 20 query builder E2E tests pass

## Task Commits

Each task was committed atomically:

1. **Task 1: Fix $N renumbering and add ORDER BY/GROUP BY raw support** - `90d67808` (feat)
2. **Task 2: Register new functions in MIR/codegen/JIT and add E2E tests** - `c7986a56` (feat)

## Files Created/Modified
- `crates/mesh-rt/src/db/repo.rs` - Added renumber_placeholders helper, updated where_raw and fragment injection in all 3 SQL builders, updated ORDER BY and GROUP BY generation for RAW: prefix, added 7 unit tests
- `crates/mesh-rt/src/db/query.rs` - Added mesh_query_order_by_raw and mesh_query_group_by_raw extern C functions
- `crates/mesh-rt/src/lib.rs` - Re-exported two new functions
- `crates/mesh-codegen/src/mir/lower.rs` - Registered known_functions and map_builtin_name for 2 new functions
- `crates/mesh-codegen/src/codegen/intrinsics.rs` - Declared 2 LLVM intrinsics, added 2 assertions to intrinsics test
- `crates/mesh-repl/src/jit.rs` - Registered 2 JIT symbols
- `crates/mesh-typeck/src/infer.rs` - Added type signatures for order_by_raw and group_by_raw, fixed fragment params type from Ptr to List<String>
- `crates/meshc/tests/e2e.rs` - Added 7 E2E tests (order_by_raw, group_by_raw, select_raw+group_by_raw, where_raw $1, fragment crypt, JSONB, all_positions)

## Decisions Made
- Unified renumber_placeholders helper handles both `?` and `$N` styles in a single char-by-char pass, returning (renumbered_sql, params_consumed)
- RAW: prefix reused for ORDER BY and GROUP BY raw expressions, keeping the established pattern from select_raw/where_raw

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added typechecker registration for new Query functions**
- **Found during:** Task 2 (E2E tests)
- **Issue:** Plan did not mention `mesh-typeck/src/infer.rs` as a touch point, but Query module functions must be registered in the typechecker for the compiler to recognize `Query.order_by_raw` and `Query.group_by_raw`
- **Fix:** Added type signatures for both functions in the Query module type registration
- **Files modified:** `crates/mesh-typeck/src/infer.rs`
- **Verification:** All E2E tests pass after adding typechecker registrations
- **Committed in:** c7986a56 (Task 2 commit)

**2. [Rule 1 - Bug] Fixed Query.fragment typechecker entry for params parameter**
- **Found during:** Task 2 (E2E fragment_crypt test)
- **Issue:** The typechecker had `fragment` typed as `(Ptr, String, Ptr)` but the params argument is actually a `List<String>` in Mesh source code. The `Ptr` type caused a type mismatch when passing `["secret"]` as the params list.
- **Fix:** Changed third parameter type from `Ptr` to `List<String>` to match the actual calling convention (consistent with `where_raw`)
- **Files modified:** `crates/mesh-typeck/src/infer.rs`
- **Verification:** e2e_query_builder_fragment_crypt test passes
- **Committed in:** c7986a56 (Task 2 commit)

---

**Total deviations:** 2 auto-fixed (1 blocking, 1 bug)
**Impact on plan:** Both fixes essential for compiler acceptance of new function calls. No scope creep.

## Issues Encountered
- Stale build artifact caused linker errors ("Undefined symbols for _mesh_query_order_by_raw") even though symbols existed in source. Resolved by `cargo clean -p mesh-rt` followed by rebuild. Same issue as Plan 01.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 106 complete: all 5 success criteria verified
  - SC1: Comparison operators work (existing e2e_query_builder_where_op)
  - SC2: IN, IS NULL, IS NOT NULL, BETWEEN, LIKE, ILIKE all generate correct SQL (Plan 01)
  - SC3: OR conditions generate grouped parenthesized SQL (Plan 01)
  - SC4: Fragment with $1 placeholders renumbers correctly (this plan: unit + E2E tests)
  - SC5: Fragments work in WHERE, SELECT, ORDER BY, GROUP BY (this plan: e2e_query_builder_fragments_all_positions)
- Ready for Phase 107 (next phase in v11.0 roadmap)

## Self-Check: PASSED

- All 8 modified files verified present on disk
- Task 1 commit `90d67808` verified in git log
- Task 2 commit `c7986a56` verified in git log
- 521 mesh-rt unit tests pass (0 failures)
- 20 query builder E2E tests pass (0 failures)
- Intrinsics declaration test passes

---
*Phase: 106-advanced-where-operators-and-raw-sql-fragments*
*Completed: 2026-02-17*
