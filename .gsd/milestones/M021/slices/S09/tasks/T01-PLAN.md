# T01: 113-mesher-rewrite-retention-and-final-cleanup 01

**Slice:** S09 — **Milestone:** M021

## Description

Rewrite 4 retention/storage query functions in `mesher/storage/queries.mpl` from raw SQL to ORM Query/Repo pipe chains, add ORM boundary documentation to 2 queries that must retain raw SQL, and verify zero Repo.query_raw/execute_raw calls remain for data queries across all Mesher .mpl files.

Purpose: Complete the Mesher ORM rewrite by eliminating all remaining raw SQL from retention/storage data queries (REWR-06) and verify the zero-raw-SQL data query invariant (REWR-08). DDL/partition operations (get_expired_partitions, drop_partition) and schema.mpl are excluded per success criteria.

Output: 4 rewritten functions + 2 boundary-documented functions + compilation verification + raw SQL audit.

## Must-Haves

- [ ] "delete_expired_events uses Repo.delete_where + Query.where_raw instead of Repo.execute_raw"
- [ ] "get_all_project_retention uses Query.from + Query.select_raw + Repo.all instead of Repo.query_raw"
- [ ] "get_project_storage uses Query.from + Query.where_raw + Query.select_raw + Repo.all instead of Repo.query_raw"
- [ ] "get_project_settings uses Query.from + Query.where_raw + Query.select_raw + Repo.all instead of Repo.query_raw"
- [ ] "update_project_settings has ORM boundary comment documenting JSONB extraction + COALESCE reason"
- [ ] "check_sample_rate has ORM boundary comment documenting random() + scalar subquery reason"
- [ ] "get_expired_partitions and drop_partition are excluded as DDL/catalog operations"
- [ ] "Zero Repo.query_raw/execute_raw calls remain for data queries (DDL/partition and documented ORM boundaries excluded)"
- [ ] "meshc build mesher compiles with zero errors"

## Files

- `mesher/storage/queries.mpl`
