# T02: 107-joins 02

**Slice:** S02 — **Milestone:** M021

## Description

Close verification gaps 1-3 from 107-VERIFICATION.md by adding runtime database tests for JOIN queries and updating requirement tracking.

Purpose: Phase 107 implemented JOIN support and verified it compiles, but ROADMAP success criteria SC2 and SC4 require runtime verification -- executing joins against a real database and inspecting returned rows. The existing `e2e_sqlite` test pattern (Sqlite.open(":memory:"), execute DDL/DML, query, inspect Map results) is the proven approach. Gap 3 is bookkeeping.

Output: One new Mesh fixture + one new Rust E2E test proving JOINs work at runtime, plus updated requirement tracking files.

## Must-Haves

- [ ] "A Mesh program executes a LEFT JOIN query against a real SQLite database and unmatched rows return empty strings for joined columns (NULL equivalent)"
- [ ] "A Mesh program executes an INNER JOIN query against a real SQLite database and returned rows contain fields from both joined tables"
- [ ] "JOIN-01 through JOIN-04 are marked complete in REQUIREMENTS.md and 107-01-SUMMARY.md"

## Files

- `tests/e2e/sqlite_join_runtime.mpl`
- `crates/meshc/tests/e2e_stdlib.rs`
- `.planning/REQUIREMENTS.md`
- `.planning/phases/107-joins/107-01-SUMMARY.md`
