---
id: T04
parent: S01
milestone: M033
provides: []
requires: []
affects: []
key_files: ["mesher/storage/queries.mpl", "mesher/ingestion/routes.mpl", "mesher/api/team.mpl", "mesher/api/alerts.mpl", "mesher/api/settings.mpl", "compiler/meshc/tests/e2e_m033_s01.rs", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Avoid binding integer success payloads (`Ok(n)`) in the live Mesher mutation handlers when the HTTP response does not truly need the count, because the current lowering still treats those `Int ! String` payloads like boxed pointers and can drop the route connection even after the mutation succeeds.", "Add a focused UUID-column `update_where_expr` proof in `compiler/meshc/tests/e2e_m033_s01.rs` so future debugging can distinguish route-layer lowering bugs from neutral expression-runtime regressions before touching Mesher storage code again."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the boxed-int route fix with `cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture`, which now passes the live assign/unassign, revoke, acknowledge/resolve, and settings mutation proof against Postgres-backed Mesher. Verified the neutral UUID assignment path directly with `cargo test -p meshc --test e2e_m033_s01 expr_uuid_update -- --nocapture`, which passes and shows `Repo.update_where_expr(...)` can set and clear a nullable UUID column outside the route layer. Reran `cargo test -p meshc --test e2e_m033_s01 mesher_issue_upsert -- --nocapture`; it still fails, but only on the final persisted `events` row-count check after the issue row has already been upserted/reopened correctly. I did not rerun `bash scripts/verify-m033-s01.sh` after that because the named issue-upsert proof is still failing and would make the full serialized gate fail for the same remaining reason."
completed_at: 2026-03-25T15:42:17.439Z
blocker_discovered: false
---

# T04: Stabilized Mesher live mutation handlers and isolated the remaining StorageWriter gap in the issue-upsert proof

> Stabilized Mesher live mutation handlers and isolated the remaining StorageWriter gap in the issue-upsert proof

## What Happened
---
id: T04
parent: S01
milestone: M033
key_files:
  - mesher/storage/queries.mpl
  - mesher/ingestion/routes.mpl
  - mesher/api/team.mpl
  - mesher/api/alerts.mpl
  - mesher/api/settings.mpl
  - compiler/meshc/tests/e2e_m033_s01.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Avoid binding integer success payloads (`Ok(n)`) in the live Mesher mutation handlers when the HTTP response does not truly need the count, because the current lowering still treats those `Int ! String` payloads like boxed pointers and can drop the route connection even after the mutation succeeds.
  - Add a focused UUID-column `update_where_expr` proof in `compiler/meshc/tests/e2e_m033_s01.rs` so future debugging can distinguish route-layer lowering bugs from neutral expression-runtime regressions before touching Mesher storage code again.
duration: ""
verification_result: mixed
completed_at: 2026-03-25T15:42:17.441Z
blocker_discovered: false
---

# T04: Stabilized Mesher live mutation handlers and isolated the remaining StorageWriter gap in the issue-upsert proof

**Stabilized Mesher live mutation handlers and isolated the remaining StorageWriter gap in the issue-upsert proof**

## What Happened

I started by rereading the T04/S01 contract, the repaired-ingest summaries, the live acceptance harness, and the Mesher write surfaces before changing anything. The first serialized `mesher_mutations` rerun reproduced the same route-local failure from T03’s follow-up: `/api/v1/issues/:id/assign` dropped the connection instead of returning JSON. I traced that path through `mesher/storage/queries.mpl`, `mesher/ingestion/routes.mpl`, and fresh `--emit-llvm` output for Mesher. That showed the issue was no longer the neutral expression write itself; the direct UUID-expression proof passed, but the live handler was still destructuring `Ok(n)` integer payloads in a way the current lowering treats like boxed pointers. I fixed the live mutation handlers by avoiding bound integer success payloads on the tested HTTP routes (`issues`, `alerts`, `api-keys`, and settings), returning status-only success responses where the tests only care about `status`, and keeping the structured neutral writes in place underneath.

To avoid guessing about the write core, I added `e2e_m033_expr_uuid_update_executes` plus a small `wait_for_query_value(...)` helper in `compiler/meshc/tests/e2e_m033_s01.rs`. That direct proof confirmed `Repo.update_where_expr(...)` can assign and clear a nullable UUID column correctly outside the Mesher route layer, so the live handler crash was a route/lowering issue rather than a neutral expression-runtime bug.

After the handler fix, the serialized `mesher_mutations` acceptance proof passed end to end. I then reran `mesher_issue_upsert`. That proof now gets through the real slice behavior the plan cares about: repeated ingest creates/updates the same issue, increments `issues.event_count`, advances `last_seen`, resolves successfully, and reopens the same issue through the structured upsert path after a later event. The remaining failure moved to the final persistence check: even after adding an explicit wait, `SELECT count(*) FROM events WHERE issue_id = ...` stayed at `0` while the route returned `202` and the issue row itself advanced correctly. I stopped there because the context-budget warning required wrap-up. The durable state now separates two facts cleanly for the next unit: the S01 route-handler boxed-int crash is fixed, and the remaining open gap is downstream in async `StorageWriter` event persistence rather than in the expression upsert path itself.

## Verification

Verified the boxed-int route fix with `cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture`, which now passes the live assign/unassign, revoke, acknowledge/resolve, and settings mutation proof against Postgres-backed Mesher. Verified the neutral UUID assignment path directly with `cargo test -p meshc --test e2e_m033_s01 expr_uuid_update -- --nocapture`, which passes and shows `Repo.update_where_expr(...)` can set and clear a nullable UUID column outside the route layer. Reran `cargo test -p meshc --test e2e_m033_s01 mesher_issue_upsert -- --nocapture`; it still fails, but only on the final persisted `events` row-count check after the issue row has already been upserted/reopened correctly. I did not rerun `bash scripts/verify-m033-s01.sh` after that because the named issue-upsert proof is still failing and would make the full serialized gate fail for the same remaining reason.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m033_s01 expr_uuid_update -- --nocapture` | 0 | ✅ pass | 24100ms |
| 2 | `cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture` | 0 | ✅ pass | 37300ms |
| 3 | `cargo test -p meshc --test e2e_m033_s01 mesher_issue_upsert -- --nocapture` | 101 | ❌ fail | 66600ms |


## Deviations

Instead of only re-running the preexisting proofs, I added a focused `e2e_m033_expr_uuid_update_executes` isolation test and a `wait_for_query_value(...)` helper to distinguish route-lowering failures from neutral expression-runtime behavior. I also changed the affected live Mesher mutation handlers to avoid binding integer success payloads (`Ok(n)`) because the current lowering crashes on that pattern; the responses now return status-only success JSON where the T04 acceptance tests only assert `status`. I stopped before rerunning `bash scripts/verify-m033-s01.sh` because `mesher_issue_upsert` still fails on the remaining StorageWriter persistence gap and the context-budget warning required wrap-up.

## Known Issues

`cargo test -p meshc --test e2e_m033_s01 mesher_issue_upsert -- --nocapture` still fails on the final `events` table persistence check: the route returns `202`, `issues.event_count` increments to `3`, `last_seen` advances, and the issue reopens from `resolved`, but `SELECT count(*)::text AS count FROM events WHERE issue_id = $1::uuid` stays at `0` even after waiting through multiple writer/health/load-monitor intervals. That points to an unresolved async `StorageWriter`/`flush_batch` persistence gap in `mesher/services/writer.mpl` / `mesher/storage/writer.mpl`. Because of that remaining failing named proof, the full `bash scripts/verify-m033-s01.sh` gate was not rerun in this unit.

## Files Created/Modified

- `mesher/storage/queries.mpl`
- `mesher/ingestion/routes.mpl`
- `mesher/api/team.mpl`
- `mesher/api/alerts.mpl`
- `mesher/api/settings.mpl`
- `compiler/meshc/tests/e2e_m033_s01.rs`
- `.gsd/KNOWLEDGE.md`


## Deviations
Instead of only re-running the preexisting proofs, I added a focused `e2e_m033_expr_uuid_update_executes` isolation test and a `wait_for_query_value(...)` helper to distinguish route-lowering failures from neutral expression-runtime behavior. I also changed the affected live Mesher mutation handlers to avoid binding integer success payloads (`Ok(n)`) because the current lowering crashes on that pattern; the responses now return status-only success JSON where the T04 acceptance tests only assert `status`. I stopped before rerunning `bash scripts/verify-m033-s01.sh` because `mesher_issue_upsert` still fails on the remaining StorageWriter persistence gap and the context-budget warning required wrap-up.

## Known Issues
`cargo test -p meshc --test e2e_m033_s01 mesher_issue_upsert -- --nocapture` still fails on the final `events` table persistence check: the route returns `202`, `issues.event_count` increments to `3`, `last_seen` advances, and the issue reopens from `resolved`, but `SELECT count(*)::text AS count FROM events WHERE issue_id = $1::uuid` stays at `0` even after waiting through multiple writer/health/load-monitor intervals. That points to an unresolved async `StorageWriter`/`flush_batch` persistence gap in `mesher/services/writer.mpl` / `mesher/storage/writer.mpl`. Because of that remaining failing named proof, the full `bash scripts/verify-m033-s01.sh` gate was not rerun in this unit.
