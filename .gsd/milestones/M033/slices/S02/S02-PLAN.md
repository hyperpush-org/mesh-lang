# S02: Explicit PG extras for JSONB, search, and crypto

**Goal:** Ship explicit PostgreSQL helper surfaces for JSONB, full-text search, and pgcrypto on top of the S01 expression core, then move the real Mesher auth/search/JSONB runtime paths onto those helpers without pretending they are portable.
**Demo:** After this: After this: Mesher event ingest, JSONB extraction, full-text search, and pgcrypto-backed auth flows work through explicit PostgreSQL helpers on the real runtime path.

## Tasks
- [x] **T01: Add explicit Pg auth helpers and move Mesher auth off raw pgcrypto SQL** — Extend the existing S01 expression runtime instead of replacing it. Add cast-capable internals and explicit `Pg.*` constructors for JSONB/search/pgcrypto, teach `Query` to bind expression-valued `SELECT` / `WHERE` clauses through the reserved `select_params` slot, add expression-valued insert support in `Repo`, wire the new calls through type inference/codegen/runtime exports, and migrate `create_user` / `authenticate_user` onto the new helper surface as the first vertical proof. Keep PG-only names under `Pg` so the neutral `Expr` surface remains portable.
  - Estimate: 3h
  - Files: compiler/mesh-rt/src/db/expr.rs, compiler/mesh-rt/src/db/query.rs, compiler/mesh-rt/src/db/repo.rs, compiler/mesh-rt/src/lib.rs, compiler/mesh-typeck/src/infer.rs, compiler/mesh-codegen/src/mir/lower.rs, compiler/mesh-codegen/src/codegen/intrinsics.rs, mesher/storage/queries.mpl
  - Verify: cargo run -q -p meshc -- build mesher
- [x] **T02: Rewrite Mesher JSONB and search helpers onto explicit Pg expression surfaces** — Move the remaining S02-owned Mesher PG query families onto the explicit helper surface: full-text search/ranking, JSONB containment/extraction/defaulting, alert-rule JSON inserts, alert snapshot construction, and the simple JSONB predicate helpers. Rewrite `search_events_fulltext`, `filter_events_by_tag`, `event_breakdown_by_tag`, `create_alert_rule`, `fire_alert`, `insert_event`, `get_event_alert_rules`, and `get_threshold_rules` to use `Pg.*`, `Query.select_expr`, `Query.where_expr`, and `Repo.insert_expr`. Re-evaluate `extract_event_fields`; if it still needs ordinality/scalar-subquery work, keep it raw with an explicit S03 boundary comment instead of faking portability.
  - Estimate: 2.5h
  - Files: mesher/storage/queries.mpl, mesher/storage/writer.mpl
  - Verify: cargo run -q -p meshc -- build mesher
- [x] **T03: Draft the S02 Postgres proof bundle and verifier script scaffolding** — Add the slice proof bundle without depending on the blocked S01 HTTP-readiness path. Reuse the Postgres harness pattern from `compiler/meshc/tests/e2e_m033_s01.rs` to execute the rewritten Mesher storage paths directly against live Postgres, proving pgcrypto hash/verify, full-text search ranking and parameter ordering, JSONB insert/defaulting, tag filtering/breakdown, alert-rule create/fire helpers, and the owned raw keep-list boundary. Close with `scripts/verify-m033-s02.sh` to run the new test target, `meshc` build/fmt checks, and a keep-list sweep that allows only the named leftovers.
  - Estimate: 2h
  - Files: compiler/meshc/tests/e2e_m033_s02.rs, scripts/verify-m033-s02.sh
  - Verify: cargo test -p meshc --test e2e_m033_s02 -- --nocapture && bash scripts/verify-m033-s02.sh
