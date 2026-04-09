# S11: Tracking Corrections And Api Acceptance

**Goal:** Close the 13 requirement tracking gaps identified in the v11.
**Demo:** Close the 13 requirement tracking gaps identified in the v11.

## Must-Haves


## Tasks

- [x] **T01: 115-tracking-corrections-and-api-acceptance 01** `est:3min`
  - Close the 13 requirement tracking gaps identified in the v11.0 milestone audit: mark WHERE-01..06, FRAG-01..04, and UPS-01..03 as complete in REQUIREMENTS.md and add requirements-completed fields to Phase 106 SUMMARY files.

Purpose: Phase 106 implementation was verified correct (all E2E tests pass, VERIFICATION.md status=passed) but documentation was never updated. UPS-01..03 were implemented in Phase 109 but their checkboxes were never flipped. This plan closes the tracking gaps — no code changes needed.
Output: REQUIREMENTS.md with 13 checkboxes updated; 106-01-SUMMARY.md and 106-02-SUMMARY.md with requirements-completed frontmatter.
- [x] **T02: 115-tracking-corrections-and-api-acceptance 02** `est:3min`
  - Accept the Phase 109 positional API style as canonical by updating ROADMAP.md success criteria, and remove two dead-code query functions from mesher/storage/queries.mpl.

Purpose: Phase 109 implemented positional APIs (Repo.insert_or_update, Repo.delete_where_returning, Query.where_sub) that differ in style from the keyword-option API specified in the original ROADMAP. The functionality is correct and verified. ROADMAP needs to reflect reality. Dead code functions were never imported anywhere.
Output: ROADMAP with corrected Phase 109 SC descriptions; queries.mpl with dead code removed.

## Files Likely Touched

- `.planning/REQUIREMENTS.md`
- `.planning/phases/106-advanced-where-operators-and-raw-sql-fragments/106-01-SUMMARY.md`
- `.planning/phases/106-advanced-where-operators-and-raw-sql-fragments/106-02-SUMMARY.md`
- `.planning/ROADMAP.md`
- `mesher/storage/queries.mpl`
