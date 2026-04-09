# S07: Mesher Rewrite Issues And Events

**Goal:** Rewrite 10 issue management queries from raw SQL (Repo.
**Demo:** Rewrite 10 issue management queries from raw SQL (Repo.

## Must-Haves


## Tasks

- [x] **T01: 111-mesher-rewrite-issues-and-events 01** `est:7min`
  - Rewrite 10 issue management queries from raw SQL (Repo.execute_raw/Repo.query_raw) to ORM Query/Repo APIs.

Purpose: Eliminate raw SQL for all simple issue CRUD operations, establishing the ORM pattern for issue domain queries. These are straightforward rewrites following the patterns proven in Phase 110 (auth/user queries).

Output: 10 issue query functions using ORM APIs instead of raw SQL in queries.mpl.
- [x] **T02: 111-mesher-rewrite-issues-and-events 02** `est:1min`
  - Document ORM boundaries for the 4 complex queries that cannot be expressed with ORM APIs: upsert_issue, check_volume_spikes (issue domain), insert_event, and extract_event_fields (event domain). Each retains its existing raw SQL with added documentation explaining why the ORM cannot express the pattern.

Purpose: Complete REWR-02 (issue management) and REWR-07 (event writer/extraction) by documenting the 4 queries that legitimately exceed ORM expressiveness. Combined with Plan 01's 10 ORM rewrites, this covers all 14 issue + event queries: 10 rewritten to ORM, 4 documented with ORM boundary rationale.

Output: 4 raw SQL queries with documentation comments explaining the specific ORM limitation for each (arithmetic SET expressions, nested subqueries, server-side JSONB extraction, complex JSONB computation chains).

## Files Likely Touched

- `mesher/storage/queries.mpl`
- `mesher/storage/queries.mpl`
- `mesher/storage/writer.mpl`
