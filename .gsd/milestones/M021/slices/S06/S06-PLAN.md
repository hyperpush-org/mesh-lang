# S06: Mesher Rewrite Auth And Users

**Goal:** Rewrite the 5 user and session query functions in `mesher/storage/queries.
**Demo:** Rewrite the 5 user and session query functions in `mesher/storage/queries.

## Must-Haves


## Tasks

- [x] **T01: 110-mesher-rewrite-auth-and-users 01** `est:6min`
  - Rewrite the 5 user and session query functions in `mesher/storage/queries.mpl` to use ORM Query/Repo APIs instead of raw SQL strings.

Purpose: Eliminate raw SQL from the user authentication and session management domain, replacing `Repo.query_raw` and `Repo.execute_raw` with ORM constructs (`Query.where`, `Query.where_raw`, `Repo.one`, `Repo.delete_where`). For INSERT operations that require PostgreSQL expressions (crypt/gen_random_bytes), use a minimal two-step pattern: a tiny `Repo.query_raw` SELECT to compute the expression, then `Repo.insert` for the actual insert.

Output: 5 rewritten functions in queries.mpl with maximum ORM usage.
- [x] **T02: 110-mesher-rewrite-auth-and-users 02** `est:4min`
  - Rewrite the 4 API key query functions in `mesher/storage/queries.mpl` to use ORM Query/Repo APIs instead of raw SQL strings.

Purpose: Eliminate raw SQL from the API key domain. `get_project_by_api_key` and `get_project_id_by_key` are converted from raw JOIN queries to `Query.join` + `Query.where` + `Query.where_null`. `revoke_api_key` uses `Repo.update_where`. `create_api_key` uses the two-step pattern (gen_random_bytes via PG call + Repo.insert).

Output: 4 rewritten functions with maximum ORM usage, completing the auth/user/session/API-key domain rewrite.

## Files Likely Touched

- `mesher/storage/queries.mpl`
- `mesher/storage/queries.mpl`
