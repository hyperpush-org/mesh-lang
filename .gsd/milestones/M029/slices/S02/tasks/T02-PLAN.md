---
estimated_steps: 3
estimated_files: 3
skills_used:
  - test
  - lint
---

# T02: Convert storage helpers to pipe style and close the slice proofs

**Slice:** S02 — Mesher JSON serialization and pipe cleanup
**Milestone:** M029

## Description

Finish the remaining storage-side cleanup in `mesher/storage/queries.mpl`: replace the two non-SQL token concatenations with interpolation, convert the four wrapping `Ok(List.map(rows, ...))` returns to Mesher's existing pipe style, and leave the designated SQL/DDL `<>` keep sites alone. This task owns the final slice-wide proof that only the accepted keep sites remain and that the build still passes.

## Steps

1. Read the four wrapping `Ok(List.map(rows, ...))` sites and the two non-SQL token builders in `mesher/storage/queries.mpl`, then confirm the designated keep sites at `event_volume_hourly` and `drop_partition` plus the schema DDL keeps in `mesher/storage/schema.mpl`.
2. Rewrite `list_orgs`, `list_projects_by_org`, `get_members`, and `list_issues_by_status` to use Mesher's existing pipe style for row mapping, keeping struct construction and `Result` return shapes unchanged.
3. Replace `"mshr_" <> Crypto.uuid4()` and `uuid1 <> uuid2` with interpolation, leave the SQL/DDL `<>` sites unchanged, and rerun the exact-location `<>` diff, the wrapping-map grep, and `meshc build mesher` as the slice closeout gate.

## Must-Haves

- [ ] The four wrapping `Ok(List.map(rows, ...))` sites in `mesher/storage/queries.mpl` are removed.
- [ ] `create_api_key` and `create_session` use interpolation, while `event_volume_hourly`, `drop_partition`, and the schema DDL sites remain the only `<>` uses left in `mesher/`.
- [ ] The slice closes with the authoritative repo-wide grep/build proofs, not a weaker broad `rg 'List\.map('` check.

## Verification

- `cargo run -q -p meshc -- build mesher`
- `diff -u <(rg -n '<>' mesher -g '*.mpl' | cut -d: -f1-2 | sort) <(printf '%s\n' mesher/storage/queries.mpl:486 mesher/storage/queries.mpl:787 mesher/storage/schema.mpl:11 mesher/storage/schema.mpl:12 mesher/storage/schema.mpl:13)`
- `! rg -n 'List\.map\(rows,|Ok\(List\.map\(' mesher -g '*.mpl'`

## Inputs

- `mesher/storage/queries.mpl` — remaining non-SQL concatenations and wrapping map sites
- `mesher/storage/schema.mpl` — designated DDL keep sites that must remain on `<>`
- `mesher/api/search.mpl` — existing pipe-style query serialization pattern to match

## Expected Output

- `mesher/storage/queries.mpl` — storage helpers converted to interpolation/pipe style with designated keep sites preserved

## Observability Impact

- Runtime signals: none added; this task is source-only cleanup in existing storage helpers.
- Inspection surfaces: `cargo run -q -p meshc -- build mesher`, the repo-wide `<>` diff against the five designated keep sites, the authoritative wrapping-map grep, and an optional captured build log such as `/tmp/m029-t02-build.log` when compiler errors need to be inspected after a failed run.
- Failure visibility: interpolation mistakes surface as compile errors or unexpected extra `<>` matches, and missed pipe conversions surface through `rg -n 'List\.map\(rows,|Ok\(List\.map\(' mesher -g '*.mpl'`.
