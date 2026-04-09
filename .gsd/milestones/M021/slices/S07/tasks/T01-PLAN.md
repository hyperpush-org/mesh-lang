# T01: 111-mesher-rewrite-issues-and-events 01

**Slice:** S07 — **Milestone:** M021

## Description

Rewrite 10 issue management queries from raw SQL (Repo.execute_raw/Repo.query_raw) to ORM Query/Repo APIs.

Purpose: Eliminate raw SQL for all simple issue CRUD operations, establishing the ORM pattern for issue domain queries. These are straightforward rewrites following the patterns proven in Phase 110 (auth/user queries).

Output: 10 issue query functions using ORM APIs instead of raw SQL in queries.mpl.

## Must-Haves

- [ ] "Issue status transitions (resolve, archive, unresolve, discard) use Repo.update_where instead of Repo.execute_raw"
- [ ] "Issue assignment uses Repo.update_where with conditional NULL handling instead of Repo.execute_raw"
- [ ] "Issue deletion uses Repo.delete_where for both events and issues instead of Repo.execute_raw"
- [ ] "is_issue_discarded uses Query.where + Repo.all instead of Repo.query_raw"
- [ ] "list_issues_by_status uses Query.where + Query.order_by + Query.select_raw + Repo.all instead of Repo.query_raw"
- [ ] "count_unresolved_issues uses Query.where + Query.select_raw with count(*) instead of Repo.query_raw"
- [ ] "get_issue_project_id uses Query.where + Query.select_raw instead of Repo.query_raw"

## Files

- `mesher/storage/queries.mpl`
