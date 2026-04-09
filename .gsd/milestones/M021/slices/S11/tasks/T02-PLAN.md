# T02: 115-tracking-corrections-and-api-acceptance 02

**Slice:** S11 — **Milestone:** M021

## Description

Accept the Phase 109 positional API style as canonical by updating ROADMAP.md success criteria, and remove two dead-code query functions from mesher/storage/queries.mpl.

Purpose: Phase 109 implemented positional APIs (Repo.insert_or_update, Repo.delete_where_returning, Query.where_sub) that differ in style from the keyword-option API specified in the original ROADMAP. The functionality is correct and verified. ROADMAP needs to reflect reality. Dead code functions were never imported anywhere.
Output: ROADMAP with corrected Phase 109 SC descriptions; queries.mpl with dead code removed.

## Must-Haves

- [ ] "ROADMAP Phase 109 success criteria reflect the positional API style actually implemented"
- [ ] "get_project_id_by_key is absent from mesher/storage/queries.mpl"
- [ ] "get_user_orgs is absent from mesher/storage/queries.mpl"
- [ ] "No other .mpl file imports get_project_id_by_key or get_user_orgs"

## Files

- `.planning/ROADMAP.md`
- `mesher/storage/queries.mpl`
