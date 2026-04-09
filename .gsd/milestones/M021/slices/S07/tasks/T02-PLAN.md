# T02: 111-mesher-rewrite-issues-and-events 02

**Slice:** S07 — **Milestone:** M021

## Description

Document ORM boundaries for the 4 complex queries that cannot be expressed with ORM APIs: upsert_issue, check_volume_spikes (issue domain), insert_event, and extract_event_fields (event domain). Each retains its existing raw SQL with added documentation explaining why the ORM cannot express the pattern.

Purpose: Complete REWR-02 (issue management) and REWR-07 (event writer/extraction) by documenting the 4 queries that legitimately exceed ORM expressiveness. Combined with Plan 01's 10 ORM rewrites, this covers all 14 issue + event queries: 10 rewritten to ORM, 4 documented with ORM boundary rationale.

Output: 4 raw SQL queries with documentation comments explaining the specific ORM limitation for each (arithmetic SET expressions, nested subqueries, server-side JSONB extraction, complex JSONB computation chains).

## Must-Haves

- [ ] "upsert_issue retains Repo.query_raw with a documentation comment explaining ORM upsert cannot express event_count + 1 arithmetic or CASE status conditionals"
- [ ] "check_volume_spikes retains Repo.execute_raw with a documentation comment explaining ORM cannot express nested subquery with JOIN + HAVING + GREATEST + interval arithmetic"
- [ ] "insert_event retains Repo.execute_raw with a documentation comment explaining Repo.insert cannot express server-side JSONB extraction (j->>'field') in INSERT...SELECT pattern"
- [ ] "extract_event_fields retains Repo.query_raw with a documentation comment explaining ORM fragments cannot express CASE/jsonb_array_elements/string_agg fingerprint computation chain"

## Files

- `mesher/storage/queries.mpl`
- `mesher/storage/writer.mpl`
