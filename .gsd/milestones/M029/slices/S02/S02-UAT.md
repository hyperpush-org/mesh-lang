# S02: Mesher JSON serialization and pipe cleanup — UAT

**Milestone:** M029
**Written:** 2026-03-24

## UAT Type

- UAT mode: artifact-driven
- Why this mode is sufficient: this slice changes source shape and compiler-cleanliness, not a live runtime contract. The truthful acceptance surface is the compiler build plus exact-location grep proofs over the cleaned Mesher source.

## Preconditions

- Run from the repo root: `/Users/sn0w/Documents/dev/mesh-lang`
- Rust/Cargo toolchain is available
- No intentional local edits are keeping `mesher/` in a partially cleaned state
- `/tmp/m029-s02-build.log` can be created or overwritten

## Smoke Test

1. Execute `cargo run -q -p meshc -- build mesher`
2. **Expected:** the command exits 0 and ends with `Compiled: mesher/mesher`.

## Test Cases

### 1. Mesher still builds after the serializer and pipe cleanup

1. Execute `cargo run -q -p meshc -- build mesher > /tmp/m029-s02-build.log 2>&1`
2. Execute `rg -n 'Compiled: mesher/mesher' /tmp/m029-s02-build.log`
3. **Expected:** the build exits 0 and the grep returns one success line from the build log.

### 2. The three API cleanup targets have zero `<>` serializer survivors

1. Execute `rg -n '<>' mesher/api/alerts.mpl mesher/api/detail.mpl mesher/api/search.mpl`
2. **Expected:** no matches. Any match in these three files is a regression against the slice goal.

### 3. Repo-wide `<>` usage is reduced to the five accepted keep sites only

1. Execute:
   ```bash
   diff -u <(rg -n '<>' mesher -g '*.mpl' | cut -d: -f1-2 | sort) \
     <(printf '%s\n' \
       mesher/storage/queries.mpl:486 \
       mesher/storage/queries.mpl:787 \
       mesher/storage/schema.mpl:11 \
       mesher/storage/schema.mpl:12 \
       mesher/storage/schema.mpl:13)
   ```
2. **Expected:** `diff` exits 0 with no output.
3. Execute `rg -n '<>' mesher -g '*.mpl'`
4. **Expected:** the only matches are:
   - `mesher/storage/queries.mpl:486`
   - `mesher/storage/queries.mpl:787`
   - `mesher/storage/schema.mpl:11`
   - `mesher/storage/schema.mpl:12`
   - `mesher/storage/schema.mpl:13`

### 4. The wrapping `List.map(rows, ...)` pattern is gone without false-failing on pipe style

1. Execute `rg -n 'List\.map\(rows,|Ok\(List\.map\(' mesher -g '*.mpl'`
2. **Expected:** no matches.
3. Open `mesher/storage/queries.mpl` and inspect the four collection-return helpers touched by the slice.
4. **Expected:** they read as `rows |> List.map(...)`, not `Ok(List.map(rows, ...))`.

### 5. Simple rows use `json {}` while raw JSONB rows still embed raw JSON

1. Open `mesher/api/search.mpl`.
2. Confirm `row_to_issue_json`, `row_to_event_json`, and `row_to_issue_event_json` use `json { ... }`.
3. Confirm `row_to_tag_event_json` still embeds `tags` as raw JSON with interpolation.
4. Open `mesher/api/alerts.mpl` and `mesher/api/detail.mpl`.
5. **Expected:**
   - alert/detail serializers do not use `<>`
   - raw JSONB fields such as `condition_json`, `action_json`, `condition_snapshot`, `exception`, `stacktrace`, `breadcrumbs`, `tags`, `extra`, and `user_context` are inserted without surrounding JSON-string quotes
   - nullable timestamps/IDs are still rendered as JSON `null` when empty

## Edge Cases

### Dynamic tag-filter JSON still works without `<>`

1. Open `mesher/api/search.mpl`.
2. Locate the helper that builds `tag_json` for `handle_filter_by_tag`.
3. **Expected:** it uses interpolation (`"{"#{key}":"#{value}"}"` inside the triple-quoted literal), not `<>`, because the property name is dynamic.

### Build log remains the authoritative compiler success surface

1. After running the build commands above, inspect `/tmp/m029-s02-build.log`.
2. **Expected:** the log contains `Compiled: mesher/mesher` and no compiler error lines. If the build step fails, this log is the first place to inspect.

### Accepted `<>` sites are SQL/DDL-only

1. Inspect the five remaining `<>` matches reported by `rg -n '<>' mesher -g '*.mpl'`.
2. **Expected:**
   - `mesher/storage/queries.mpl:486` is the bucketed SQL fragment for `date_trunc(...)`
   - `mesher/storage/queries.mpl:787` is the trusted `DROP TABLE IF EXISTS` DDL string
   - `mesher/storage/schema.mpl:11-13` are the partition DDL builder pieces
   No HTTP/API serializer or token-builder `<>` usage should remain.

## Failure Signals

- `cargo run -q -p meshc -- build mesher` exits non-zero
- `/tmp/m029-s02-build.log` does not contain `Compiled: mesher/mesher`
- Any `<>` match appears in `mesher/api/alerts.mpl`, `mesher/api/detail.mpl`, or `mesher/api/search.mpl`
- The repo-wide `<>` diff shows additional files/lines beyond the five accepted keep sites
- `rg -n 'List\.map\(rows,|Ok\(List\.map\(' mesher -g '*.mpl'` returns matches
- A raw JSONB field becomes quoted like a JSON string instead of embedded JSON

## Requirements Proved By This UAT

- R024 — proves the JSON/interpolation cleanup and pipe-style cleanup portion of the mesher dogfood requirement

## Not Proven By This UAT

- Final multiline-import adoption in `mesher/`
- `meshc fmt --check mesher` passing across the whole app
- Full milestone closeout (`reference-backend` verification and the broader `cargo test -p meshc --test e2e` gate)

## Notes for Tester

Use the targeted wrapping-map grep, not broad `rg 'List\.map\('`, as the acceptance check. Broad greps will false-fail on the desired `rows |> List.map(...)` style. Likewise, treat the exact-location `<>` diff as intentional: if future edits move the accepted SQL/DDL keep sites, update the expected `file:line` list in the same slice rather than weakening the proof.
