---
estimated_steps: 10
estimated_files: 7
skills_used: []
---

# T02: Documented the live Mesher ingest blocker after verifying the S01 mutation rewrites already exist and normalizing queries.mpl formatting.

Prove the new core on the simplest live Mesher write families before tackling conflict upserts. This task rewrites the direct mutation paths that currently use raw SQL for `NULL`, `now()`, or PG-only body parsing: API-key revoke, issue assign/unassign, alert acknowledge/resolve, and project settings updates. The route/service layer should stay behavior-compatible while the storage layer stops depending on raw SQL for these portable cases.

Steps
1. Rewrite `revoke_api_key`, `assign_issue`, `acknowledge_alert`, `resolve_fired_alert`, and `update_project_settings` in `mesher/storage/queries.mpl` to use the new expression-aware Query/Repo surface.
2. Parse project settings JSON in Mesh-side helpers so partial updates use the neutral core instead of PG-side `jsonb` extraction, and keep the HTTP/service signatures in `mesher/api/settings.mpl`, `mesher/api/team.mpl`, `mesher/api/alerts.mpl`, `mesher/services/project.mpl`, and `mesher/ingestion/routes.mpl` stable.
3. Extend `compiler/meshc/tests/e2e_m033_s01.rs` with live Mesher route proofs that hit the real endpoints, then inspect database rows for `NULL`, timestamp, and field-value changes instead of trusting HTTP status alone.
4. Re-run the targeted live test filter plus Mesher build/fmt to catch any behavioral or formatting drift before handing off T03.

Must-Haves
- [ ] The named direct-mutation functions stop calling `Repo.execute_raw` / `Repo.query_raw` for S01-owned portable cases
- [ ] Settings partial updates are driven by Mesh-side parsing plus neutral expressions, not PG-only body extraction
- [ ] Live Mesher tests assert DB-side field changes for `assigned_to`, `acknowledged_at`, `resolved_at`, `retention_days`, `sample_rate`, and `revoked_at`

## Inputs

- `compiler/meshc/tests/e2e_m033_s01.rs`
- `mesher/storage/queries.mpl`
- `mesher/api/alerts.mpl`
- `mesher/api/settings.mpl`
- `mesher/api/team.mpl`
- `mesher/services/project.mpl`
- `mesher/ingestion/routes.mpl`

## Expected Output

- `mesher/storage/queries.mpl`
- `mesher/api/alerts.mpl`
- `mesher/api/settings.mpl`
- `mesher/api/team.mpl`
- `mesher/services/project.mpl`
- `mesher/ingestion/routes.mpl`
- `compiler/meshc/tests/e2e_m033_s01.rs`

## Verification

`cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture`
`cargo run -q -p meshc -- fmt --check mesher`
`cargo run -q -p meshc -- build mesher`

## Observability Impact

- Signals added/changed: named `mesher_mutations` acceptance proofs for each direct write family plus direct row snapshots of `alerts`, `projects`, `api_keys`, and `issues`
- How a future agent inspects this: rerun the `mesher_mutations` filter in `compiler/meshc/tests/e2e_m033_s01.rs` and inspect the captured row assertions for the route that drifted
- Failure state exposed: when a route still uses raw SQL or mutates the wrong field, the harness reports the exact route/storage function and the mismatched DB columns
