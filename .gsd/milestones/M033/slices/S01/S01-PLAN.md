# S01: Neutral expression core on real write paths

**Goal:** Ship a neutral expression builder and expression-aware Query/Repo write surface that can drive real Mesher mutations — upserts, computed updates, `NULL`, and `now()`-driven writes — without hiding PostgreSQL-only behavior inside the baseline API.
**Demo:** After this: After this: live Postgres-backed Mesher write paths for issue upserts, alert state transitions, settings updates, and `NULL`/`now()`-driven mutations run through structured Mesh expressions instead of recurring raw SQL.

## Tasks
- [x] **T01: Added Query.select_exprs with compiler/runtime wiring and expr select e2e coverage** — Land the neutral expression core end-to-end before touching Mesher storage code. This task adds the dedicated expression builder, the Query/Repo entrypoints needed for expression-valued `SELECT` / `SET` / `ON CONFLICT` work, the compiler/runtime wiring that makes those calls legal from Mesh code, and the first permanent `meshc` e2e proofs in `compiler/meshc/tests/e2e_m033_s01.rs`. The contract must stay portable: no JSONB, pgcrypto, search, or catalog-specific helpers belong in this layer.

Steps
1. Add a dedicated expression-builder surface under the runtime DB layer and expose only the portable nodes S01 needs: column refs, literal/parameter values, `NULL`, function calls, arithmetic/comparison, `CASE`, and `COALESCE`, plus the neutral conflict-update reference the upsert path will need later.
2. Extend `Query` / `Repo` so Mesh code can use those expression nodes for expression-valued `SELECT`, `SET`, and `ON CONFLICT` work without routing through `RAW:` strings.
3. Wire the new surface through `compiler/mesh-typeck/src/infer.rs`, `compiler/mesh-codegen/src/mir/lower.rs`, `compiler/mesh-codegen/src/codegen/intrinsics.rs`, and the runtime exports so the Mesh-side API is fully callable.
4. Add named `e2e_m033_expr_*` coverage in `compiler/meshc/tests/e2e_m033_s01.rs` that proves the contract compiles, executes, and keeps placeholder ordering / serializer output stable enough for later Mesher rewrites.

Must-Haves
- [ ] Mesh code can build neutral expression trees and pass them through Query/Repo without `RAW:` or `Repo.query_raw`
- [ ] `compiler/meshc/tests/e2e_m033_s01.rs` contains passing `e2e_m033_expr_*` proofs for expression-valued `SELECT`, `SET`, and conflict-update work
- [ ] The new core excludes PG-only JSONB/search/crypto helpers so the later vendor-specific slices still have an explicit seam
  - Estimate: 2.5h
  - Files: compiler/mesh-rt/src/db/expr.rs, compiler/mesh-rt/src/db/query.rs, compiler/mesh-rt/src/db/repo.rs, compiler/mesh-rt/src/db/mod.rs, compiler/mesh-rt/src/lib.rs, compiler/mesh-typeck/src/infer.rs, compiler/mesh-codegen/src/mir/lower.rs, compiler/mesh-codegen/src/codegen/intrinsics.rs, compiler/meshc/tests/e2e_m033_s01.rs
  - Verify: `cargo test -p meshc --test e2e_m033_s01 expr_ -- --nocapture`
`cargo run -q -p meshc -- build mesher`
- [x] **T02: Documented the live Mesher ingest blocker after verifying the S01 mutation rewrites already exist and normalizing queries.mpl formatting.** — Prove the new core on the simplest live Mesher write families before tackling conflict upserts. This task rewrites the direct mutation paths that currently use raw SQL for `NULL`, `now()`, or PG-only body parsing: API-key revoke, issue assign/unassign, alert acknowledge/resolve, and project settings updates. The route/service layer should stay behavior-compatible while the storage layer stops depending on raw SQL for these portable cases.

Steps
1. Rewrite `revoke_api_key`, `assign_issue`, `acknowledge_alert`, `resolve_fired_alert`, and `update_project_settings` in `mesher/storage/queries.mpl` to use the new expression-aware Query/Repo surface.
2. Parse project settings JSON in Mesh-side helpers so partial updates use the neutral core instead of PG-side `jsonb` extraction, and keep the HTTP/service signatures in `mesher/api/settings.mpl`, `mesher/api/team.mpl`, `mesher/api/alerts.mpl`, `mesher/services/project.mpl`, and `mesher/ingestion/routes.mpl` stable.
3. Extend `compiler/meshc/tests/e2e_m033_s01.rs` with live Mesher route proofs that hit the real endpoints, then inspect database rows for `NULL`, timestamp, and field-value changes instead of trusting HTTP status alone.
4. Re-run the targeted live test filter plus Mesher build/fmt to catch any behavioral or formatting drift before handing off T03.

Must-Haves
- [ ] The named direct-mutation functions stop calling `Repo.execute_raw` / `Repo.query_raw` for S01-owned portable cases
- [ ] Settings partial updates are driven by Mesh-side parsing plus neutral expressions, not PG-only body extraction
- [ ] Live Mesher tests assert DB-side field changes for `assigned_to`, `acknowledged_at`, `resolved_at`, `retention_days`, `sample_rate`, and `revoked_at`
  - Estimate: 2h
  - Files: mesher/storage/queries.mpl, mesher/api/alerts.mpl, mesher/api/settings.mpl, mesher/api/team.mpl, mesher/services/project.mpl, mesher/ingestion/routes.mpl, compiler/meshc/tests/e2e_m033_s01.rs
  - Verify: `cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture`
`cargo run -q -p meshc -- fmt --check mesher`
`cargo run -q -p meshc -- build mesher`
  - Blocker: Clean-start Mesher still returns HTTP 429 on the first `/api/v1/events` request in `cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture`, which blocks the live mutation and issue-upsert proofs before any `assigned_to`, `acknowledged_at`, `resolved_at`, `retention_days`, `sample_rate`, or `revoked_at` assertions can execute. Manual standalone reproduction against a fresh Postgres container confirmed `/api/v1/projects/default/settings` returns 200 while `/api/v1/events` with the seeded default API key returns 429, so the remaining blocker is upstream in Mesher’s ingest/rate-limit path rather than in the S01 neutral write rewrites.
- [x] **T03: Fixed clean-start Mesher ingest by stabilizing rate-limit and processor service returns and adding a live first-event rate-limit proof** — Why: Every remaining S01 proof depends on fresh-instance event ingest working with the seeded default API key. Right now the first /api/v1/events request is rejected with HTTP 429 before the neutral write paths are exercised, so the slice cannot be truthfully closed.

Do:
- Reproduce the clean-start 429 in the live Postgres-backed harness and trace the auth -> sampling -> rate-limit path for the seeded default project/API key.
- Fix the state or routing bug that causes the first event to be treated as already over limit, without weakening genuine rate limiting for later bursts.
- Add a focused e2e proof that a freshly started Mesher instance accepts the first seeded-key event and only returns 429 when the configured threshold is actually exceeded.

Done when: the first /api/v1/events call on a clean Mesher boot returns 202 instead of 429, and the focused proof shows the limiter still behaves honestly after the fix.
  - Estimate: 2h
  - Files: mesher/services/rate_limiter.mpl, mesher/ingestion/routes.mpl, mesher/ingestion/pipeline.mpl, compiler/meshc/tests/e2e_m033_s01.rs
  - Verify: cargo test -p meshc --test e2e_m033_s01 mesher_ingest_first_event -- --nocapture
- [x] **T04: Stabilized Mesher live mutation handlers and isolated the remaining StorageWriter gap in the issue-upsert proof** — Why: The neutral write rewrites and the expression-based upsert path are already present in local reality, but S01 is not done until the live Mesher routes are re-proven after the ingest blocker is fixed.

Do:
- Re-run and tighten the existing live mutation and issue-upsert acceptance proofs against the repaired ingest path.
- Keep meshc build mesher, the e2e harness, and the verify script serialized so the shared mesher/mesher(.o) outputs do not create false linker failures.
- Prove repeated event ingest still creates or updates the same issue, increments event_count, advances last_seen, and reopens resolved issues through the structured upsert path.
- Keep the raw-write keep-list check honest: only the S02-owned PG helpers (create_alert_rule, fire_alert, insert_event) may remain raw after this slice closes.

Done when: the live Postgres-backed slice demo passes end-to-end and the remaining raw write keep-sites are limited to the explicit PG helpers deferred to S02.
  - Estimate: 1.5h
  - Files: compiler/meshc/tests/e2e_m033_s01.rs, scripts/verify-m033-s01.sh, mesher/storage/queries.mpl, mesher/storage/writer.mpl
  - Verify: cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture
cargo test -p meshc --test e2e_m033_s01 mesher_issue_upsert -- --nocapture
bash scripts/verify-m033-s01.sh
