---
id: T02
parent: S02
milestone: M029
provides:
  - Mesher storage helpers now use pipe/interpolation style, and the slice closes with only the five designated `<>` keep sites remaining
key_files:
  - mesher/storage/queries.mpl
  - .gsd/milestones/M029/slices/S02/S02-PLAN.md
  - .gsd/milestones/M029/slices/S02/tasks/T02-PLAN.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Kept the storage-query edits line-neutral so the hardcoded `file:line` `<>` keep-site diff stayed authoritative at slice closeout
patterns_established:
  - Prefer `Ok(rows |> List.map(...))` over `Ok(List.map(rows, ...))` in Mesher query helpers when the result shape is unchanged and the goal is pipe-style consistency
observability_surfaces:
  - "cargo run -q -p meshc -- build mesher"
  - "(cargo run -q -p meshc -- build mesher > /tmp/m029-s02-build.log 2>&1 && rg -n 'Compiled: mesher/mesher' /tmp/m029-s02-build.log) || (rg -n 'error(\\[|:)|panic' /tmp/m029-s02-build.log && false)"
  - "! rg -n '<>' mesher/api/alerts.mpl mesher/api/detail.mpl mesher/api/search.mpl"
  - "diff -u <(rg -n '<>' mesher -g '*.mpl' | cut -d: -f1-2 | sort) <(printf '%s\\n' mesher/storage/queries.mpl:486 mesher/storage/queries.mpl:787 mesher/storage/schema.mpl:11 mesher/storage/schema.mpl:12 mesher/storage/schema.mpl:13)"
  - "! rg -n 'List\\.map\\(rows,|Ok\\(List\\.map\\(' mesher -g '*.mpl'"
duration: 18m
verification_result: passed
completed_at: 2026-03-24T12:17:39-04:00
blocker_discovered: false
---

# T02: Convert storage helpers to pipe style and close the slice proofs

**Converted Mesher storage helpers to pipe/interpolation style and closed the S02 grep/build gate.**

## What Happened

I fixed the pre-flight artifact gaps first: `S02-PLAN.md` now includes an explicit captured-build verification step and `T02-PLAN.md` now documents the task's observability surface. After that I read the remaining storage survivors in `mesher/storage/queries.mpl` and the designated DDL keep sites in `mesher/storage/schema.mpl`.

The code change stayed narrow. `list_orgs`, `list_projects_by_org`, `get_members`, and `list_issues_by_status` now return mapped rows as `Ok(rows |> List.map(...))`, which matches Mesher's existing pipe style without changing any struct construction or `Result` shapes. `create_api_key` now builds its token as `"mshr_#{Crypto.uuid4()}"`, and `create_session` now joins the two UUID fragments with interpolation as `"#{uuid1}#{uuid2}"`.

The hardcoded `<>` keep-site diff in this slice depends on exact `file:line` locations, so I kept the storage edits line-neutral rather than rewriting those blocks more aggressively. I also appended that rule to `.gsd/KNOWLEDGE.md` so a future cleanup task does not accidentally fail the gate by shifting valid keep sites.

No dedicated test file was added. This task is source-only cleanup, and the slice contract here is the compiler plus the exact-location grep/diff proofs.

## Verification

All task-level and slice-level checks passed:
- `cargo run -q -p meshc -- build mesher` succeeded.
- The captured-build status check wrote `/tmp/m029-s02-build.log` and confirmed `Compiled: mesher/mesher`.
- The targeted API negative grep still confirms T01 scope has no `<>` survivors.
- The repo-wide `<>` diff matches only the five designated keep sites: `mesher/storage/queries.mpl:486`, `mesher/storage/queries.mpl:787`, and `mesher/storage/schema.mpl:11-13`.
- The authoritative wrapping-map grep returned zero matches.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- build mesher` | 0 | ✅ pass | 12.76s |
| 2 | `(cargo run -q -p meshc -- build mesher > /tmp/m029-s02-build.log 2>&1 && rg -n 'Compiled: mesher/mesher' /tmp/m029-s02-build.log) \|\| (rg -n 'error(\[\|:)|panic' /tmp/m029-s02-build.log && false)` | 0 | ✅ pass | 12.67s |
| 3 | `! rg -n '<>' mesher/api/alerts.mpl mesher/api/detail.mpl mesher/api/search.mpl` | 0 | ✅ pass | 0.08s |
| 4 | `diff -u <(rg -n '<>' mesher -g '*.mpl' \| cut -d: -f1-2 \| sort) <(printf '%s\n' mesher/storage/queries.mpl:486 mesher/storage/queries.mpl:787 mesher/storage/schema.mpl:11 mesher/storage/schema.mpl:12 mesher/storage/schema.mpl:13)` | 0 | ✅ pass | 0.05s |
| 5 | `! rg -n 'List\.map\(rows,\|Ok\(List\.map\(' mesher -g '*.mpl'` | 0 | ✅ pass | 0.04s |

## Diagnostics

No new runtime signals were added. The durable inspection surfaces for this task are:
- `cargo run -q -p meshc -- build mesher`
- `/tmp/m029-s02-build.log` for captured compiler success/error output
- `! rg -n '<>' mesher/api/alerts.mpl mesher/api/detail.mpl mesher/api/search.mpl`
- `diff -u <(rg -n '<>' mesher -g '*.mpl' | cut -d: -f1-2 | sort) <(printf '%s\n' mesher/storage/queries.mpl:486 mesher/storage/queries.mpl:787 mesher/storage/schema.mpl:11 mesher/storage/schema.mpl:12 mesher/storage/schema.mpl:13)`
- `! rg -n 'List\.map\(rows,|Ok\(List\.map\(' mesher -g '*.mpl'`

If this slice regresses later, start with the diff/grep proofs before reading serializer code. They tell you whether the failure is a new concatenation site, a shifted keep-site location, or a missed wrapping-map survivor.

## Deviations

- Updated `.gsd/milestones/M029/slices/S02/S02-PLAN.md` to add the pre-flight-required captured-build verification step and to document the captured build log as an inspection surface.
- Updated `.gsd/milestones/M029/slices/S02/tasks/T02-PLAN.md` to add the missing `## Observability Impact` section.
- Appended a knowledge note in `.gsd/KNOWLEDGE.md` about keeping edits line-neutral when a slice gate hardcodes allowed `file:line` keep sites.

## Known Issues

- Bash invocations in this environment still print a pre-existing `/Users/sn0w/.profile` warning about missing `/Users/sn0w/.cargo/env`. It did not affect the Mesher build or grep/diff proofs in this task.

## Files Created/Modified

- `mesher/storage/queries.mpl` — converted the four wrapping row-mapping returns to pipe style and replaced the two non-SQL token concatenations with interpolation.
- `.gsd/milestones/M029/slices/S02/S02-PLAN.md` — added the captured-build verification step, updated diagnostics text, and marked T02 complete.
- `.gsd/milestones/M029/slices/S02/tasks/T02-PLAN.md` — added the missing observability-impact section required by the pre-flight check.
- `.gsd/KNOWLEDGE.md` — recorded the line-neutral editing rule for exact-location grep/diff gates.
