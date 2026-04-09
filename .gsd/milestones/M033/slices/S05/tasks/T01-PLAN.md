---
estimated_steps: 4
estimated_files: 8
skills_used:
  - antfu/skills@vitepress
  - postgresql-database-engineering
---

# T01: Rewrite the public database guide around the honest Mesh/Mesher boundary

Why: The current database guide is still a generic `Sqlite` / `Pg` / `Pool` brochure, so S05 needs a public Mesher-backed rewrite before automation can lock the milestone’s truth surface.

Steps:
1. Reframe `website/docs/docs/databases/index.md` around the shipped M033 contract instead of the generic database tour: neutral `Expr` / `Query` / `Repo` / honest `Migration.create_index(...)`, explicit `Pg.*` extras, remaining raw escape hatches, and the proof/failure map.
2. Build the neutral section from the real S01/S02/S04 sources and examples: use `Expr.label`, `Expr.value`, `Expr.column`, `Expr.null`, `Expr.case_when`, `Expr.coalesce`, `Query.where_expr`, `Query.select_exprs`, `Repo.insert_expr`, `Repo.update_where_expr`, and `Repo.insert_or_update_expr` as they appear in `compiler/meshc/tests/e2e_m033_s01.rs`, `mesher/storage/writer.mpl`, and `mesher/storage/queries.mpl`.
3. Build the PostgreSQL-only section from the real Mesher-backed helpers in `mesher/storage/queries.mpl`, `mesher/migrations/20260216120000_create_initial_schema.mpl`, `mesher/storage/schema.mpl`, and `compiler/meshc/src/migrate.rs`; keep JSONB/search/crypto/partition/schema helpers explicitly under `Pg.*`, and avoid inventing brittle pseudo-examples (for example, do not introduce a `jsonb_build_object(...)` example unless the required `Pg.text(...)` casts are shown).
4. Add the honest boundary/proof sections: name `Repo.query_raw`, `Repo.execute_raw`, and `Migration.execute` as escape hatches; explain that M033 leaves a short named raw leftover list instead of promising zero raw SQL/DDL; say SQLite extras are later work and not runtime-proven here; keep the page aligned with the repo’s proof-surface language/style.

Must-Haves:
- [ ] The page teaches the shipped neutral surface with real API names, including `Expr.label` rather than `Expr.alias`.
- [ ] The page marks JSONB/search/crypto/partition/schema helpers as PostgreSQL-only `Pg.*` behavior, not as portable APIs.
- [ ] The page tells an honest raw-leftover / SQLite-later story and anchors it in the real Mesher files that ship today.

## Inputs

- `website/docs/docs/databases/index.md`
- `website/docs/docs/production-backend-proof/index.md`
- `compiler/meshc/tests/e2e_m033_s01.rs`
- `mesher/storage/queries.mpl`
- `mesher/storage/writer.mpl`
- `mesher/migrations/20260216120000_create_initial_schema.mpl`
- `mesher/storage/schema.mpl`
- `compiler/meshc/src/migrate.rs`
- `website/package.json`

## Expected Output

- `website/docs/docs/databases/index.md`

## Verification

npm --prefix website run build
