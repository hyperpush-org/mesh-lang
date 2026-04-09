# S08: Mesher Rewrite Search Dashboard And Alerts

**Goal:** Rewrite search, dashboard, detail, and team queries from Repo.
**Demo:** Rewrite search, dashboard, detail, and team queries from Repo.

## Must-Haves


## Tasks

- [x] **T01: 112-mesher-rewrite-search-dashboard-and-alerts 01** `est:5min`
  - Rewrite search, dashboard, detail, and team queries from Repo.query_raw to ORM APIs. Document ORM boundaries for queries with parameterized SELECT expressions.

Purpose: Eliminate Repo.query_raw from search, dashboard, detail, and team domains.
Output: ~13 queries rewritten to ORM; ~5 queries documented with ORM boundary rationale.
- [x] **T02: 112-mesher-rewrite-search-dashboard-and-alerts 02** `est:3min`
  - Rewrite alert system queries from Repo.query_raw/execute_raw to ORM APIs where expressible, and document ORM boundaries for complex alert queries.

Purpose: Eliminate Repo.query_raw from the alert system domain, completing REWR-05.
Output: ~7 alert queries rewritten to ORM; ~3 queries documented with ORM boundary rationale.

## Files Likely Touched

- `mesher/storage/queries.mpl`
- `mesher/storage/queries.mpl`
