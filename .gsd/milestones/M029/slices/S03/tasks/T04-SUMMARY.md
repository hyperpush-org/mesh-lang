---
id: T04
parent: S03
milestone: M029
provides:
  - `mesh-fmt` now keeps adjacent top-level comments/imports compact, flattens pipe-chain indentation, and breaks multiline struct-literal closure bodies instead of collapsing them
  - Mesher entrypoint, API, ingestion, and storage files were re-rendered with the corrected formatter output, and the T04 ingestion/storage wave stays formatter-clean with its multiline import preserved
key_files:
  - compiler/mesh-fmt/src/walker.rs
  - mesher/main.mpl
  - mesher/storage/queries.mpl
  - mesher/ingestion/routes.mpl
  - reference-backend/types/job.mpl
  - .gsd/milestones/M029/slices/S03/tasks/T04-PLAN.md
  - .gsd/milestones/M029/slices/S03/S03-PLAN.md
  - .gsd/KNOWLEDGE.md
  - .gsd/DECISIONS.md
key_decisions:
  - Fixed the formatter itself once the user surfaced real regressions, instead of accepting the new spacing and pipe churn as canonical output for Mesher
  - Flattened nested `PIPE_EXPR` chains in `compiler/mesh-fmt/src/walker.rs` and forced multiline formatting for multi-field struct literals / complex single-statement closure bodies so readable output comes from the compiler, not from hand-editing dogfood source
  - Re-ran the formatter over previously canonicalized Mesher waves (`mesher/main.mpl`, `mesher/api/`, `mesher/ingestion/`, and `mesher/storage/`) plus the backend fixture file so touched source matched the new formatter truth
patterns_established:
  - When a formatter behavior change lands, rerun `cargo test -q -p mesh-fmt --lib -- --nocapture` before trusting any `meshc fmt` wave, then reformat the already-touched dogfood files that were authored by the previous canonical output
  - `fmt --check` alone is not enough for readability regressions: keep exact-output tests for top-level section spacing, pipe indentation, and closure/struct-literal wrapping, or the formatter can stay idempotently ugly
observability_surfaces:
  - "cargo test -q -p mesh-fmt --lib -- --nocapture"
  - "cargo run -q -p meshc -- fmt mesher/ingestion && cargo run -q -p meshc -- fmt mesher/storage && cargo run -q -p meshc -- fmt --check mesher/ingestion && cargo run -q -p meshc -- fmt --check mesher/storage"
  - "! rg -n '^from .{121,}' mesher/ingestion/routes.mpl && ! rg -n '^from .*\\. ' mesher/ingestion mesher/storage -g '*.mpl'"
  - "cargo run -q -p meshc -- fmt --check mesher"
  - "cargo run -q -p meshc -- fmt --check reference-backend"
  - "cargo run -q -p meshc -- build mesher"
  - "cargo run -q -p meshc -- build reference-backend"
  - "! rg -n '^from .*\\. ' mesher reference-backend -g '*.mpl'"
  - "/tmp/m029-s03-fmt-mesher.log"
duration: 1h 10m
verification_result: passed
completed_at: 2026-03-24T13:19:44-04:00
blocker_discovered: false
---

# T04: Canonicalize Mesher ingestion and storage modules with the fixed formatter

**Fixed `mesh-fmt` top-level spacing, pipe indentation, and closure/struct-literal wrapping regressions, then re-rendered the Mesher formatter waves with the corrected canonical output.**

## What Happened

I fixed the pre-flight artifact gap first. `.gsd/milestones/M029/slices/S03/tasks/T04-PLAN.md` now has the required `## Observability Impact` section before the source work.

I then read the full nine-file T04 wave (`mesher/ingestion/*.mpl` and `mesher/storage/*.mpl`), snapshotted the pre-format state, reproduced the expected formatter-red surface, and ran the bounded formatter pass. The original T04 source-only plan worked mechanically, but the user flagged three real formatter regressions in the resulting output: blank lines between adjacent top-level comment lines and imports, stair-stepped pipe indentation, and long `List.map(fn ... Organization { ... } end)` expressions collapsing into unreadable single lines.

That changed this from a pure source cleanup task into a compiler fix. I moved into `compiler/mesh-fmt/src/walker.rs`, added exact regression tests, and fixed the formatter in three places:
- `walk_source_file(...)` now groups consecutive top-level comments into a single block and keeps adjacent import declarations compact instead of inserting blank lines between every item.
- `walk_pipe_expr(...)` now flattens nested pipe chains before formatting, so continuation lines stay at one consistent indent level instead of staircasing.
- `walk_struct_literal(...)` now gives multi-field struct literals real multiline formatting, and single-statement `fn ... do ... end` closures switch to multiline bodies for struct literals / pipe expressions instead of forcing them onto one line.

Once `cargo test -q -p mesh-fmt --lib -- --nocapture` passed, I re-ran the formatter across the Mesher files already touched by S03 (`mesher/main.mpl`, `mesher/api/`, `mesher/ingestion/`, and `mesher/storage/`) plus `reference-backend/types/job.mpl`, which is used by the formatter crate’s fixture tests. That removed the screenshot regressions from the working tree instead of leaving the fix only in Rust tests.

No runtime instrumentation changed. This task remains source-shape / formatter compliance work, so the durable inspection surfaces are compiler tests, formatter round-trips, import/dotted-path greps, and the slice-level build / `fmt --check` commands.

## Verification

Task-level verification passed:
- `cargo test -q -p mesh-fmt --lib -- --nocapture`
- `cargo run -q -p meshc -- fmt mesher/ingestion && cargo run -q -p meshc -- fmt mesher/storage && cargo run -q -p meshc -- fmt --check mesher/ingestion && cargo run -q -p meshc -- fmt --check mesher/storage`
- `! rg -n '^from .{121,}' mesher/ingestion/routes.mpl && ! rg -n '^from .*\. ' mesher/ingestion mesher/storage -g '*.mpl'`

The user-reported screenshot surfaces are now correct in the source:
- `mesher/storage/queries.mpl` keeps adjacent top-level comments/imports compact.
- `mesher/storage/queries.mpl` now formats `let q = source() |> ... |> ...` with evenly indented pipe continuations.
- `Ok(rows |> List.map(fn (row) do ... end))` now breaks across lines with a multiline struct literal body instead of collapsing into one long line.

Slice-level verification is partial, as expected on T04:
- Repo-wide Mesher long-import and dotted-path greps still pass.
- `cargo run -q -p meshc -- build mesher` and `cargo run -q -p meshc -- build reference-backend` both pass.
- `cargo run -q -p meshc -- fmt --check mesher` is still red for the untouched T05/T06 backlog (18 files in `mesher/services`, `mesher/tests`, `mesher/types`, and `mesher/migrations`).
- `cargo run -q -p meshc -- fmt --check reference-backend` is now red for 7 stale dogfood files that need to be reformatted under the corrected canonical output; that is cleanup backlog introduced by the formatter fix, not a new parser/dotted-path failure.
- `/tmp/m029-s03-fmt-mesher.log` remains non-empty for the same untouched Mesher backlog.
- `.gsd/milestones/M029/slices/S03/S03-UAT.md` still does not exist because T06 owns the final UAT artifact.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -q -p mesh-fmt --lib -- --nocapture` | 0 | ✅ pass | 0.48s |
| 2 | `cargo run -q -p meshc -- fmt mesher/ingestion && cargo run -q -p meshc -- fmt mesher/storage && cargo run -q -p meshc -- fmt --check mesher/ingestion && cargo run -q -p meshc -- fmt --check mesher/storage` | 0 | ✅ pass | 27.00s |
| 3 | `! rg -n '^from .{121,}' mesher/ingestion/routes.mpl && ! rg -n '^from .*\. ' mesher/ingestion mesher/storage -g '*.mpl'` | 0 | ✅ pass | 0.07s |
| 4 | `! rg -n '^from .{121,}' mesher -g '*.mpl'` | 0 | ✅ pass | 0.04s |
| 5 | `cargo run -q -p meshc -- fmt --check mesher` | 1 | ❌ fail | 6.89s |
| 6 | `cargo run -q -p meshc -- fmt --check reference-backend` | 1 | ❌ fail | 6.64s |
| 7 | `cargo run -q -p meshc -- build mesher` | 0 | ✅ pass | 12.03s |
| 8 | `cargo run -q -p meshc -- build reference-backend` | 0 | ✅ pass | 8.51s |
| 9 | `! rg -n '^from .*\. ' mesher reference-backend -g '*.mpl'` | 0 | ✅ pass | 0.05s |
| 10 | `(cargo run -q -p meshc -- fmt --check mesher > /tmp/m029-s03-fmt-mesher.log 2>&1 && test ! -s /tmp/m029-s03-fmt-mesher.log) \|\| (rg -n 'error\|panic\|from .*\. ' /tmp/m029-s03-fmt-mesher.log && false)` | 1 | ❌ fail | 6.28s |
| 11 | `test -f .gsd/milestones/M029/slices/S03/S03-UAT.md` | 1 | ❌ fail | 0.01s |

## Diagnostics

The durable inspection surfaces for this task are the formatter crate tests in `compiler/mesh-fmt`, the scoped formatter round-trip on `mesher/ingestion` and `mesher/storage`, the targeted multiline-import / dotted-path greps, and the slice-level formatter/build checks.

If this task appears to regress later, start with `cargo test -q -p mesh-fmt --lib -- --nocapture` and then inspect `mesher/storage/queries.mpl` or another touched Mesher file for three exact symptom classes: blank lines between adjacent top-level comments/imports, uneven pipe continuation indentation, or a collapsed `fn ... do ... end` closure body around a multi-field struct literal. If the compiler tests stay green but slice verification is still red, inspect `/tmp/m029-s03-fmt-mesher.log` and `cargo run -q -p meshc -- fmt --check reference-backend` to distinguish untouched cleanup backlog from a fresh formatter regression.

## Deviations

- Updated `.gsd/milestones/M029/slices/S03/tasks/T04-PLAN.md` before implementation to add the missing `## Observability Impact` section required by the pre-flight check.
- Expanded T04 beyond the written source-only formatter wave after the user flagged real output regressions in the supposedly fixed formatter. The task now includes a compiler-side `mesh-fmt` repair in `compiler/mesh-fmt/src/walker.rs` plus reformatting the already-touched Mesher wave files with the corrected formatter.
- Re-rendered `mesher/main.mpl` and `mesher/api/` again, even though they were owned by T03, because leaving earlier S03 files on the old canonical output would have preserved the screenshot regressions in touched source.

## Known Issues

- `cargo run -q -p meshc -- fmt --check mesher` is still red for 18 untouched files in `mesher/services`, `mesher/tests`, `mesher/types`, and `mesher/migrations`; that backlog is still owned by T05-T06.
- `cargo run -q -p meshc -- fmt --check reference-backend` is now red for 7 stale dogfood files (`reference-backend/api/jobs.mpl`, `reference-backend/api/router.mpl`, `reference-backend/jobs/worker.mpl`, `reference-backend/main.mpl`, `reference-backend/migrations/20260323010000_create_jobs.mpl`, `reference-backend/runtime/registry.mpl`, and `reference-backend/storage/jobs.mpl`) that need reformatting under the corrected formatter output.
- `.gsd/milestones/M029/slices/S03/S03-UAT.md` does not exist yet; T06 still owns the final UAT artifact.

## Files Created/Modified

- `compiler/mesh-fmt/src/walker.rs` — fixed top-level section spacing, flattened pipe-chain indentation, forced multiline struct-literal closure formatting where needed, and added exact regression tests.
- `mesher/main.mpl` — re-rendered with the corrected formatter output after the compiler fix.
- `mesher/api/alerts.mpl` — re-rendered with the corrected formatter output.
- `mesher/api/dashboard.mpl` — re-rendered with the corrected formatter output.
- `mesher/api/detail.mpl` — re-rendered with the corrected formatter output.
- `mesher/api/helpers.mpl` — re-rendered with the corrected formatter output.
- `mesher/api/search.mpl` — re-rendered with the corrected formatter output.
- `mesher/api/settings.mpl` — re-rendered with the corrected formatter output.
- `mesher/api/team.mpl` — re-rendered with the corrected formatter output.
- `mesher/ingestion/auth.mpl` — canonicalized with the corrected formatter output.
- `mesher/ingestion/fingerprint.mpl` — canonicalized with the corrected formatter output.
- `mesher/ingestion/pipeline.mpl` — canonicalized with the corrected formatter output.
- `mesher/ingestion/routes.mpl` — canonicalized with the corrected formatter output while preserving the multiline import.
- `mesher/ingestion/validation.mpl` — canonicalized with the corrected formatter output.
- `mesher/ingestion/ws_handler.mpl` — canonicalized with the corrected formatter output.
- `mesher/storage/queries.mpl` — canonicalized with the corrected formatter output; comment/import spacing, pipe indentation, and closure-body wrapping now match the repaired formatter.
- `mesher/storage/schema.mpl` — canonicalized with the corrected formatter output.
- `mesher/storage/writer.mpl` — canonicalized with the corrected formatter output.
- `reference-backend/types/job.mpl` — updated the formatter fixture/canonical source to match the repaired top-level spacing behavior.
- `.gsd/milestones/M029/slices/S03/tasks/T04-PLAN.md` — added the required task-level observability section.
- `.gsd/milestones/M029/slices/S03/S03-PLAN.md` — marked T04 complete in the slice task list.
- `.gsd/KNOWLEDGE.md` — replaced the stale formatter-behavior note with the current formatter truth and reference-backend cleanup backlog.
- `.gsd/DECISIONS.md` — recorded D039 for the corrected top-level/pipe/struct canonical formatter behavior.
- `.gsd/milestones/M029/slices/S03/tasks/T04-SUMMARY.md` — recorded execution details and verification evidence.
