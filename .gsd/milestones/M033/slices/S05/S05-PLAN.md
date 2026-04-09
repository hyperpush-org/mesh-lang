# S05: Public docs and integrated Mesher acceptance

**Goal:** Publish the real M033 database boundary in the public Mesh docs and close the milestone with one canonical acceptance command that replays the assembled live-Postgres proof stack plus a docs-truth sweep.
**Demo:** After this: After this: the public Mesh database docs explain the shipped neutral DSL and PG extras through a Mesher-backed path, and the assembled Mesher data-layer behavior is re-proven end-to-end on live Postgres.

## Tasks
- [x] **T01: Rewrite the public database guide around the real Mesh/Mesher boundary and proof surface** — Why: The current database guide is still a generic `Sqlite` / `Pg` / `Pool` brochure, so S05 needs a public Mesher-backed rewrite before automation can lock the milestone’s truth surface.

Steps:
1. Reframe `website/docs/docs/databases/index.md` around the shipped M033 contract instead of the generic database tour: neutral `Expr` / `Query` / `Repo` / honest `Migration.create_index(...)`, explicit `Pg.*` extras, remaining raw escape hatches, and the proof/failure map.
2. Build the neutral section from the real S01/S02/S04 sources and examples: use `Expr.label`, `Expr.value`, `Expr.column`, `Expr.null`, `Expr.case_when`, `Expr.coalesce`, `Query.where_expr`, `Query.select_exprs`, `Repo.insert_expr`, `Repo.update_where_expr`, and `Repo.insert_or_update_expr` as they appear in `compiler/meshc/tests/e2e_m033_s01.rs`, `mesher/storage/writer.mpl`, and `mesher/storage/queries.mpl`.
3. Build the PostgreSQL-only section from the real Mesher-backed helpers in `mesher/storage/queries.mpl`, `mesher/migrations/20260216120000_create_initial_schema.mpl`, `mesher/storage/schema.mpl`, and `compiler/meshc/src/migrate.rs`; keep JSONB/search/crypto/partition/schema helpers explicitly under `Pg.*`, and avoid inventing brittle pseudo-examples (for example, do not introduce a `jsonb_build_object(...)` example unless the required `Pg.text(...)` casts are shown).
4. Add the honest boundary/proof sections: name `Repo.query_raw`, `Repo.execute_raw`, and `Migration.execute` as escape hatches; explain that M033 leaves a short named raw leftover list instead of promising zero raw SQL/DDL; say SQLite extras are later work and not runtime-proven here; keep the page aligned with the repo’s proof-surface language/style.

Must-Haves:
- [ ] The page teaches the shipped neutral surface with real API names, including `Expr.label` rather than `Expr.alias`.
- [ ] The page marks JSONB/search/crypto/partition/schema helpers as PostgreSQL-only `Pg.*` behavior, not as portable APIs.
- [ ] The page tells an honest raw-leftover / SQLite-later story and anchors it in the real Mesher files that ship today.
  - Estimate: 2h
  - Files: website/docs/docs/databases/index.md, website/docs/docs/production-backend-proof/index.md, compiler/meshc/tests/e2e_m033_s01.rs, mesher/storage/queries.mpl, mesher/storage/writer.mpl, mesher/migrations/20260216120000_create_initial_schema.mpl, mesher/storage/schema.mpl, compiler/meshc/src/migrate.rs
  - Verify: npm --prefix website run build
- [x] **T02: Added the canonical S05 acceptance wrapper, locked the docs truth surface, and repaired stale verifier ownership so the full replay passes.** — Why: R038 only closes once one public command replays the assembled proof stack and mechanically fails when docs drift away from the real boundary.

Steps:
1. Add `scripts/verify-m033-s05.sh` using the same failure-reporting pattern as the existing slice verifiers plus the docs-truth style from `reference-backend/scripts/verify-production-proof-surface.sh`, with a dedicated `.tmp/m033-s05/verify` artifact directory and named phase logs.
2. Make the wrapper run the cheap docs gate first (`npm --prefix website run build`), then an exact-string Python sweep over `website/docs/docs/databases/index.md`, then `bash scripts/verify-m033-s02.sh`, `bash scripts/verify-m033-s03.sh`, and `bash scripts/verify-m033-s04.sh` serially. Do not parallelize: the Postgres-backed proof surfaces share host port `5432`.
3. In the docs-truth sweep, require the exact neutral API names, PG-only API names, honest boundary wording, SQLite-later wording, Mesher-backed file references, and canonical proof commands that the public docs are supposed to stand behind.
4. Tighten `website/docs/docs/databases/index.md` as needed so the final public page includes the new canonical `bash scripts/verify-m033-s05.sh` command and the exact phrases the verifier enforces, without turning the page into zero-raw marketing.

Must-Haves:
- [ ] `scripts/verify-m033-s05.sh` becomes the canonical S05 acceptance command and preserves serial execution across the existing live-Postgres verifiers.
- [ ] The Python docs-truth sweep fails on missing API names, boundary wording, Mesher file anchors, or proof commands instead of silently allowing docs drift.
- [ ] The final docs page and the new script agree on the exact public contract, including the honest leftover / escape-hatch story and the SQLite-later seam.
  - Estimate: 2h
  - Files: scripts/verify-m033-s05.sh, website/docs/docs/databases/index.md, scripts/verify-m033-s02.sh, scripts/verify-m033-s03.sh, scripts/verify-m033-s04.sh, reference-backend/scripts/verify-production-proof-surface.sh
  - Verify: bash scripts/verify-m033-s05.sh
