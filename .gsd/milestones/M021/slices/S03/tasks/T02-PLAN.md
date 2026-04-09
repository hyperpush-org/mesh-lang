# T02: 108-aggregations 02

**Slice:** S03 — **Milestone:** M021

## Description

Verify aggregation functions execute correctly at runtime against real SQLite databases, proving count/sum/avg/min/max return correct values with GROUP BY grouping and HAVING filtering.

Purpose: Close the runtime verification gap -- Plan 01 proves the compiler pipeline, this plan proves the generated SQL executes against real data and returns correct results. This follows the same pattern as Phase 107's runtime JOIN verification.

Output: Mesh fixture file and Rust E2E test with assertions for all four aggregation requirements.

## Must-Haves

- [ ] "count(*) query against SQLite returns correct integer count"
- [ ] "sum()/avg()/min()/max() on numeric column return correct values from real SQLite data"
- [ ] "GROUP BY produces one aggregated row per group"
- [ ] "HAVING filters out groups that fail the condition"

## Files

- `tests/e2e/sqlite_aggregate_runtime.mpl`
- `crates/meshc/tests/e2e_stdlib.rs`
