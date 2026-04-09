# T01: 112-mesher-rewrite-search-dashboard-and-alerts 01

**Slice:** S08 — **Milestone:** M021

## Description

Rewrite search, dashboard, detail, and team queries from Repo.query_raw to ORM APIs. Document ORM boundaries for queries with parameterized SELECT expressions.

Purpose: Eliminate Repo.query_raw from search, dashboard, detail, and team domains.
Output: ~13 queries rewritten to ORM; ~5 queries documented with ORM boundary rationale.

## Must-Haves

- [ ] "Simple search/dashboard/detail/team queries use Query.from + Query.where_raw + Query.select_raw + Repo.all instead of Repo.query_raw"
- [ ] "Queries with parameterized SELECT expressions retain raw SQL with ORM boundary documentation"
- [ ] "All rewritten functions preserve identical signatures and behavior"

## Files

- `mesher/storage/queries.mpl`
