# T01: 110-mesher-rewrite-auth-and-users 01

**Slice:** S06 — **Milestone:** M021

## Description

Rewrite the 5 user and session query functions in `mesher/storage/queries.mpl` to use ORM Query/Repo APIs instead of raw SQL strings.

Purpose: Eliminate raw SQL from the user authentication and session management domain, replacing `Repo.query_raw` and `Repo.execute_raw` with ORM constructs (`Query.where`, `Query.where_raw`, `Repo.one`, `Repo.delete_where`). For INSERT operations that require PostgreSQL expressions (crypt/gen_random_bytes), use a minimal two-step pattern: a tiny `Repo.query_raw` SELECT to compute the expression, then `Repo.insert` for the actual insert.

Output: 5 rewritten functions in queries.mpl with maximum ORM usage.

## Must-Haves

- [ ] "authenticate_user uses Query.where + Query.where_raw with crypt() instead of Repo.query_raw"
- [ ] "validate_session uses Query.where + Query.where_raw('expires_at > now()') instead of Repo.query_raw"
- [ ] "delete_session uses Repo.delete_where instead of Repo.execute_raw"
- [ ] "create_user uses two-step pattern: Repo.query_raw for crypt expression + Repo.insert for the insert"
- [ ] "create_session uses two-step pattern: Repo.query_raw for gen_random_bytes + Repo.insert for the insert"
- [ ] "All 5 functions compile without errors via meshc build mesher"

## Files

- `mesher/storage/queries.mpl`
