# S09: Mesher Rewrite Retention And Final Cleanup

**Goal:** Rewrite 4 retention/storage query functions in `mesher/storage/queries.
**Demo:** Rewrite 4 retention/storage query functions in `mesher/storage/queries.

## Must-Haves


## Tasks

- [x] **T01: 113-mesher-rewrite-retention-and-final-cleanup 01** `est:5min`
  - Rewrite 4 retention/storage query functions in `mesher/storage/queries.mpl` from raw SQL to ORM Query/Repo pipe chains, add ORM boundary documentation to 2 queries that must retain raw SQL, and verify zero Repo.query_raw/execute_raw calls remain for data queries across all Mesher .mpl files.

Purpose: Complete the Mesher ORM rewrite by eliminating all remaining raw SQL from retention/storage data queries (REWR-06) and verify the zero-raw-SQL data query invariant (REWR-08). DDL/partition operations (get_expired_partitions, drop_partition) and schema.mpl are excluded per success criteria.

Output: 4 rewritten functions + 2 boundary-documented functions + compilation verification + raw SQL audit.

## Files Likely Touched

- `mesher/storage/queries.mpl`
