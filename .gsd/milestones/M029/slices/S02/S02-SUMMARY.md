---
id: S02
parent: M029
milestone: M029
provides:
  - Mesher API serializers now use `json {}` for scalar rows and `#{}` interpolation for raw JSONB payloads instead of `<>` chains
  - `mesher/storage/queries.mpl` now uses pipe style for the four old wrapping `Ok(List.map(rows, ...))` returns
  - Repo-wide `<>` usage in `mesher/` is reduced to the five designated SQL/DDL keep sites only
requires: []
affects:
  - S03
key_files:
  - mesher/api/alerts.mpl
  - mesher/api/detail.mpl
  - mesher/api/search.mpl
  - mesher/storage/queries.mpl
key_decisions:
  - "D034: use `json {}` for simple scalar rows and `#{}` interpolation where raw JSONB fields must stay unquoted"
  - "D037: prove the cleanup with the targeted wrapping-map grep plus the exact-location `<>` diff against the accepted keep sites"
patterns_established:
  - Use `json {}` only when every field is a scalar or already intentionally string-quoted by the JSON macro
  - Keep raw JSONB columns embedded with interpolation so `condition_json`, `action_json`, `condition_snapshot`, `exception`, `stacktrace`, `breadcrumbs`, `tags`, `extra`, and `user_context` do not get double-quoted
  - Prove pipe cleanup with `rg -n 'List\.map\(rows,|Ok\(List\.map\(' mesher -g '*.mpl'` instead of broad `List.map(` greps that also match the desired pipe style
observability_surfaces:
  - "cargo run -q -p meshc -- build mesher"
  - "rg -n 'Compiled: mesher/mesher' /tmp/m029-s02-build.log"
  - "! rg -n '<>' mesher/api/alerts.mpl mesher/api/detail.mpl mesher/api/search.mpl"
  - "diff -u <(rg -n '<>' mesher -g '*.mpl' | cut -d: -f1-2 | sort) <(printf '%s\n' mesher/storage/queries.mpl:486 mesher/storage/queries.mpl:787 mesher/storage/schema.mpl:11 mesher/storage/schema.mpl:12 mesher/storage/schema.mpl:13)"
  - "! rg -n 'List\.map\(rows,|Ok\(List\.map\(' mesher -g '*.mpl'"
drill_down_paths:
  - .gsd/milestones/M029/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M029/slices/S02/tasks/T02-SUMMARY.md
duration: 1h45m
verification_result: passed
completed_at: 2026-03-24 02:10 EDT
---

# S02: Mesher JSON serialization and pipe cleanup

**Shipped the mesher cleanup pass that was still fighting readability: JSON serializer `<>` chains are gone from alerts/detail/search, wrapping `List.map(rows, ...)` calls now read as pipes, and only the five deliberate SQL/DDL `<>` sites remain.**

## What Happened

S02 stayed tightly scoped to the real mesher friction sites identified in the slice plan. In `mesher/api/alerts.mpl`, `mesher/api/detail.mpl`, and `mesher/api/search.mpl`, the old hand-built `<>` JSON assembly was split by data shape instead of rewritten mechanically. Simple row serializers now use `json {}` where fields are ordinary strings or parsed ints. Serializers that include raw JSONB text still use interpolation so those payloads remain embedded as JSON, not double-quoted strings. That is why alert rule/action payloads, condition snapshots, event detail JSONB fields, and tag-filter responses now use triple-quoted interpolation rather than the JSON macro.

The search handlers also got the cleaner pagination/helper path the slice was aiming for. Issue and simple event rows now serialize with `json {}`; the tag-filter and paginated wrapper helpers use interpolation only where they need raw nested JSON or dynamic property placement. The end result is the same response shape with less brittle string assembly.

`mesher/storage/queries.mpl` then closed the storage-side cleanup. The four old `Ok(List.map(rows, ...))` return sites now use the repo’s preferred pipe style (`rows |> List.map(...)`). The two non-SQL token builders moved to interpolation: API keys now use `"mshr_#{Crypto.uuid4()}"`, and session tokens concatenate the two UUID halves with `"#{uuid1}#{uuid2}"` instead of `<>`. The exact-location `<>` proof now shows only the accepted SQL/DDL keep sites: bucketed SQL in `event_volume_hourly`, `DROP TABLE IF EXISTS` in `drop_partition`, and the partition-DDL builder in `mesher/storage/schema.mpl`.

## Verification

All slice-plan verification checks passed exactly as written:

- `cargo run -q -p meshc -- build mesher`
- `cargo run -q -p meshc -- build mesher > /tmp/m029-s02-build.log 2>&1 && rg -n 'Compiled: mesher/mesher' /tmp/m029-s02-build.log`
- `! rg -n '<>' mesher/api/alerts.mpl mesher/api/detail.mpl mesher/api/search.mpl`
- `diff -u <(rg -n '<>' mesher -g '*.mpl' | cut -d: -f1-2 | sort) <(printf '%s\n' mesher/storage/queries.mpl:486 mesher/storage/queries.mpl:787 mesher/storage/schema.mpl:11 mesher/storage/schema.mpl:12 mesher/storage/schema.mpl:13)`
- `! rg -n 'List\.map\(rows,|Ok\(List\.map\(' mesher -g '*.mpl'`

The diagnostic surfaces worked as intended:

- `/tmp/m029-s02-build.log` contains the authoritative compiler success line `Compiled: mesher/mesher`
- the targeted API grep proves the slice scope has zero serializer `<>` survivors in `alerts`, `detail`, and `search`
- the repo-wide `<>` snapshot now enumerates exactly five keep sites and nothing else
- the targeted wrapping-map grep returns no matches, which is the honest proof that pipe cleanup is done without false-failing on `rows |> List.map(...)`

## Requirements Advanced

- R024 — closed the JSON/interpolation and pipe-style portion of mesher cleanup; the remaining work is S03’s multiline-import rollout plus final `meshc fmt --check mesher` compliance
- R011 — continued the dogfood-first rule by fixing only the real mesher friction sites, not inventing new language work beyond what the app cleanup required

## Requirements Validated

- none — S02 materially advanced the milestone but did not finish a requirement end-to-end on its own

## New Requirements Surfaced

- none

## Requirements Invalidated or Re-scoped

- none

## Deviations

none

## Known Limitations

S02 does not finish M029. Long mesher import lines still need the S03 parenthesized multiline conversion and final formatter proof. `meshc fmt --check mesher` is still a separate closeout gate, and the accepted SQL/DDL `<>` sites remain by design.

## Follow-ups

- S03 should convert the remaining long mesher imports to parenthesized multiline form and then rerun `meshc fmt --check mesher`
- Keep the S02 `<>` proof updated if S03 edits lines above the five accepted keep sites; the gate is intentionally exact on `file:line`
- Do not "simplify" the raw JSONB serializers to `json {}` unless the underlying response fields stop being raw JSON text first

## Files Created/Modified

- `mesher/api/alerts.mpl` — replaced alert-rule and fired-alert serializer `<>` chains with interpolation that preserves raw JSONB fields and nullable timestamps
- `mesher/api/detail.mpl` — replaced full event-detail and navigation response assembly with interpolation-based helpers that keep JSONB fields raw
- `mesher/api/search.mpl` — moved simple row serializers to `json {}` and kept interpolation only for raw tags, pagination wrappers, and dynamic tag-filter JSON
- `mesher/storage/queries.mpl` — converted wrapping `List.map(rows, ...)` returns to pipe style and replaced the non-SQL API key/session token builders with interpolation

## Forward Intelligence

### What the next slice should know
- The serializer cleanup is already split along the right seam: scalar rows can use `json {}`, but any response carrying pre-rendered JSONB text still needs interpolation. S03 should not reopen that decision while doing formatter/import work.
- The authoritative remaining `<>` set is now exactly five lines: `mesher/storage/queries.mpl:486`, `mesher/storage/queries.mpl:787`, and `mesher/storage/schema.mpl:11-13`.

### What's fragile
- The `file:line` `<>` diff is intentionally strict. Editing above those keep sites without updating the expected line numbers will fail the proof even when the code change is otherwise correct.
- Raw JSONB serializer fields are easy to accidentally double-quote during cleanup because the JSON macro looks cleaner than it is for this data shape.

### Authoritative diagnostics
- `cargo run -q -p meshc -- build mesher` plus `rg -n 'Compiled: mesher/mesher' /tmp/m029-s02-build.log` — authoritative compiler truth for this slice
- `rg -n '<>' mesher -g '*.mpl'` — authoritative snapshot of the only remaining accepted concatenation sites
- `rg -n 'List\.map\(rows,|Ok\(List\.map\(' mesher -g '*.mpl'` — authoritative pipe cleanup proof that avoids false failures on the desired `rows |> List.map(...)` form

### What assumptions changed
- "All remaining JSON serializers can move to `json {}`" — false; raw JSONB payloads still need interpolation to preserve JSON types
- "A broad `rg 'List\.map\('` is a good pipe-style acceptance test" — false; it also matches the preferred pipe form and obscures real cleanup progress
