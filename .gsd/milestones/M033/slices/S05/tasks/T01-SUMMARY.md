---
id: T01
parent: S05
milestone: M033
provides: []
requires: []
affects: []
key_files: ["website/docs/docs/databases/index.md"]
key_decisions: ["Reframed the public database docs around the shipped Mesh/Mesher M033 contract instead of preserving the older generic SQLite/Pg/Pool brochure.", "Documented PostgreSQL-specific behavior explicitly under `Pg.*` and called out the remaining raw escape hatches by name instead of implying zero raw SQL/DDL."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the rewritten docs page with a full `npm --prefix website run build`, which passed after the frontmatter quoting fix. Ran a targeted `rg` truth sweep on `website/docs/docs/databases/index.md` to confirm the required public contract strings are present: `Expr.label`, `Expr.value`, `Expr.column`, `Expr.null`, `Expr.case_when`, `Expr.coalesce`, `Query.where_expr`, `Query.select_exprs`, `Repo.insert_expr`, `Repo.update_where_expr`, `Repo.insert_or_update_expr`, `Migration.create_index`, `Repo.query_raw`, `Repo.execute_raw`, `Migration.execute`, explicit `Pg.*` references, and the SQLite-later wording. Ran the slice-level acceptance command `bash scripts/verify-m033-s05.sh`; it failed with exit code 127 because the script does not exist yet, which is the expected partial state for T01 because T02 owns creation of the canonical S05 verifier."
completed_at: 2026-03-26T00:56:26.194Z
blocker_discovered: false
---

# T01: Rewrite the public database guide around the real Mesh/Mesher boundary and proof surface

> Rewrite the public database guide around the real Mesh/Mesher boundary and proof surface

## What Happened
---
id: T01
parent: S05
milestone: M033
key_files:
  - website/docs/docs/databases/index.md
key_decisions:
  - Reframed the public database docs around the shipped Mesh/Mesher M033 contract instead of preserving the older generic SQLite/Pg/Pool brochure.
  - Documented PostgreSQL-specific behavior explicitly under `Pg.*` and called out the remaining raw escape hatches by name instead of implying zero raw SQL/DDL.
duration: ""
verification_result: mixed
completed_at: 2026-03-26T00:56:26.211Z
blocker_discovered: false
---

# T01: Rewrite the public database guide around the real Mesh/Mesher boundary and proof surface

**Rewrite the public database guide around the real Mesh/Mesher boundary and proof surface**

## What Happened

Rewrote `website/docs/docs/databases/index.md` from the old generic SQLite/PostgreSQL brochure into a contract-first guide anchored in the shipped M033 surface. The new page now explains the neutral `Expr` / `Query` / `Repo` / `Migration` boundary with real source-backed examples from `compiler/meshc/tests/e2e_m033_s01.rs`, `mesher/storage/writer.mpl`, and `mesher/storage/queries.mpl`; explicitly separates PostgreSQL-only behavior under `Pg.*`; and documents the honest raw escape hatches (`Repo.query_raw`, `Repo.execute_raw`, `Migration.execute`) plus the current named raw keep-sites that still live in `mesher/storage/queries.mpl`. I also added a proof/failure map that points readers to the existing `verify-m033-s01.sh` through `verify-m033-s04.sh` commands and the docs build command, matching the real repo proof surface instead of aspirational examples. During verification, VitePress surfaced a YAML frontmatter parse error because the new description contained a colon; I fixed that by quoting the description string and reran the build successfully.

## Verification

Verified the rewritten docs page with a full `npm --prefix website run build`, which passed after the frontmatter quoting fix. Ran a targeted `rg` truth sweep on `website/docs/docs/databases/index.md` to confirm the required public contract strings are present: `Expr.label`, `Expr.value`, `Expr.column`, `Expr.null`, `Expr.case_when`, `Expr.coalesce`, `Query.where_expr`, `Query.select_exprs`, `Repo.insert_expr`, `Repo.update_where_expr`, `Repo.insert_or_update_expr`, `Migration.create_index`, `Repo.query_raw`, `Repo.execute_raw`, `Migration.execute`, explicit `Pg.*` references, and the SQLite-later wording. Ran the slice-level acceptance command `bash scripts/verify-m033-s05.sh`; it failed with exit code 127 because the script does not exist yet, which is the expected partial state for T01 because T02 owns creation of the canonical S05 verifier.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix website run build` | 0 | ✅ pass | 49462ms |
| 2 | `bash scripts/verify-m033-s05.sh` | 127 | ❌ fail | 192ms |
| 3 | `rg -n "Expr\\.label|Expr\\.value|Expr\\.column|Expr\\.null|Expr\\.case_when|Expr\\.coalesce|Query\\.where_expr|Query\\.select_exprs|Repo\\.insert_expr|Repo\\.update_where_expr|Repo\\.insert_or_update_expr|Migration\\.create_index|Repo\\.query_raw|Repo\\.execute_raw|Migration\\.execute|Pg\\.|SQLite-specific extras are later work" website/docs/docs/databases/index.md` | 0 | ✅ pass | 124ms |


## Deviations

None beyond quoting the new frontmatter description after the docs build exposed a YAML parsing issue; the planned content contract and output file stayed the same.

## Known Issues

`scripts/verify-m033-s05.sh` does not exist yet, so the slice-level acceptance command remains pending T02. Slice verification is therefore partial at the end of T01 even though the docs page itself now builds cleanly.

## Files Created/Modified

- `website/docs/docs/databases/index.md`


## Deviations
None beyond quoting the new frontmatter description after the docs build exposed a YAML parsing issue; the planned content contract and output file stayed the same.

## Known Issues
`scripts/verify-m033-s05.sh` does not exist yet, so the slice-level acceptance command remains pending T02. Slice verification is therefore partial at the end of T01 even though the docs page itself now builds cleanly.
