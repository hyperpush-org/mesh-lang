# T02: 110-mesher-rewrite-auth-and-users 02

**Slice:** S06 — **Milestone:** M021

## Description

Rewrite the 4 API key query functions in `mesher/storage/queries.mpl` to use ORM Query/Repo APIs instead of raw SQL strings.

Purpose: Eliminate raw SQL from the API key domain. `get_project_by_api_key` and `get_project_id_by_key` are converted from raw JOIN queries to `Query.join` + `Query.where` + `Query.where_null`. `revoke_api_key` uses `Repo.update_where`. `create_api_key` uses the two-step pattern (gen_random_bytes via PG call + Repo.insert).

Output: 4 rewritten functions with maximum ORM usage, completing the auth/user/session/API-key domain rewrite.

## Must-Haves

- [ ] "get_project_by_api_key uses Query.join + Query.where + Query.where_null instead of Repo.query_raw"
- [ ] "get_project_id_by_key uses Query.join + Query.where + Query.where_null instead of Repo.query_raw"
- [ ] "create_api_key uses two-step pattern: Repo.query_raw for gen_random_bytes + Repo.insert for the data insert"
- [ ] "revoke_api_key uses Repo.update_where with Query.where_raw instead of Repo.execute_raw"
- [ ] "All 4 functions compile without errors and maintain existing signatures"
- [ ] "Zero Repo.execute_raw calls remain in the auth/user/session/API-key sections of queries.mpl"

## Files

- `mesher/storage/queries.mpl`
