# S02: Mesher JSON serialization and pipe cleanup — Research

**Depth:** Targeted. This is established Mesh application code, not new infrastructure, but the slice has one real semantic risk: preserving JSON value types while replacing `<>` chains. I followed the `debug-like-expert` skill’s **VERIFY, DON'T ASSUME** rule and checked the actual query return shapes plus a live build before recommending seams.

## Requirements Targeted

### Direct
- **R024** — Mesher should use idiomatic JSON serialization/interpolation and pipe style instead of wrapping `List.map(rows, ...)` calls.

### Supported
- **R011** — This is real dogfood-driven cleanup from live Mesher friction, not speculative language work.

## Summary

1. **`mesher` already builds on current HEAD.**
   - Verified with `cargo run -p meshc -- build mesher`.
   - S02 is cleanup work with a compile-proof, not a broken-build rescue.

2. **The slice is narrower than the milestone text makes it sound.**
   - Total `<>` hits in `mesher/`: **21**.
   - Designated keep sites: **5** total.
     - `mesher/storage/schema.mpl:11-13` — partition DDL string assembly.
     - `mesher/storage/queries.mpl:486` — SQL expression assembly for `date_trunc(...)`.
     - `mesher/storage/queries.mpl:787` — `DROP TABLE IF EXISTS ...` DDL.
   - Required cleanup set: **16** total across **4 files**:
     - `mesher/api/alerts.mpl` — 3 sites
     - `mesher/api/detail.mpl` — 4 sites
     - `mesher/api/search.mpl` — 7 sites
     - `mesher/storage/queries.mpl` — 2 non-SQL interpolation sites

3. **All remaining wrapping-style `List.map(rows, ...)` calls are isolated to one file.**
   - Only **4** wrapping survivors remain, all in `mesher/storage/queries.mpl`:
     - `list_orgs` (`:51`)
     - `list_projects_by_org` (`:83`)
     - `get_members` (`:231`)
     - `list_issues_by_status` (`:361`)
   - The rest of Mesher already uses the preferred pipe style, so the house style is clear.

4. **`json {}` is not a blanket replacement.**
   - Several target serializers are fed by query rows where JSONB and primitive fields arrive as `...::text` strings.
   - A naive `json { enabled: Map.get(row, "enabled") }` or `json { cooldown_minutes: Map.get(row, "cooldown_minutes") }` would silently change JSON types by quoting booleans/ints as strings.
   - The right split is:
     - use `json {}` where fields are simple strings/ints/options already handled as real values
     - use `#{}` interpolation where raw JSON fragments must be embedded unchanged

5. **The repo already contains the exact patterns S02 should copy.**
   - `mesher/api/helpers.mpl:58` — array assembly via interpolation: `"[#{String.join(items, ",")}]"`
   - `mesher/ingestion/routes.mpl:140-141` — raw JSON fragment splicing via triple-quoted interpolation: `"""{"type":"event","issue_id":"#{issue_id}","data":#{body}}"""`
   - `mesher/ingestion/pipeline.mpl:134-136` — string interpolation in JSON payloads for WebSocket alerts
   - `mesher/api/dashboard.mpl`, `mesher/api/team.mpl`, and already-clean parts of `mesher/api/search.mpl` show the preferred `rows |> List.map(fn...) |> to_json_array()` style

6. **One roadmap verification literal is wrong as written.**
   - The milestone says `rg 'List\.map(' mesher/ -g '*.mpl'` should return 0.
   - That grep also matches the **good** pipe style (`rows |> List.map(...)`) already present in many files.
   - The authoritative grep for S02 should be narrower:
     - `rg -n 'List\.map\(rows,|Ok\(List\.map\(' mesher -g '*.mpl'`
   - Current baseline for that narrower grep is exactly the 4 `queries.mpl` wrapping sites above.

## Implementation Landscape

### Shared prior art and constraints

- `mesher/api/helpers.mpl:58` already gives S02 the standard array-wrapper helper: `to_json_array(items)`.
- Triple-quoted interpolation is already used in Mesher when a JSON string needs to embed a raw fragment without extra escaping noise.
- I did **not** find any shared JSON-string escaping helper (`escape_json`, `String.escape`, etc.). That means S02 should stay scoped to readability/idiom cleanup, not expand into a new escaping abstraction unless the executor finds a concrete failing case.

### `mesher/api/alerts.mpl`

Relevant seam:
- `format_nullable_ts` (`:12`)
- `rule_row_to_json` (`:21`)
- `alert_row_to_json` (`:26`)
- long import line at `:6` (154 chars)

Why this file is interpolation-first:
- `mesher/storage/queries.mpl:643` (`list_alert_rules`) returns:
  - `condition_json::text`
  - `action_json::text`
  - `enabled::text`
  - `cooldown_minutes::text`
  - nullable timestamps as `COALESCE(...::text, '')`
- `mesher/storage/queries.mpl:744` (`list_alerts`) returns:
  - `condition_snapshot::text`
  - nullable timestamps as `COALESCE(...::text, '')`

Implication:
- `rule_row_to_json` and `alert_row_to_json` should stay **type-preserving raw assembly**, just rewritten with interpolation instead of `<>`.
- `format_nullable_ts` can remain local, but should stop using `<>`.
- Do **not** drag S03 into S02 here: the 154-char import line belongs to the multiline-import cleanup slice.

### `mesher/api/detail.mpl`

Relevant seam:
- `event_detail_to_json` (`:16`)
- `format_neighbor_id` (`:37`)
- `neighbors_to_json` (`:46`)
- `build_detail_response` (`:55`)

Why this file is mixed but still mostly interpolation:
- `mesher/storage/queries.mpl:561` (`get_event_detail`) returns six JSONB-ish fields as raw JSON text:
  - `exception`
  - `stacktrace`
  - `breadcrumbs`
  - `tags`
  - `extra`
  - `user_context`
- `event_detail_to_json` therefore must embed raw fragments without quoting them.

What is flexible:
- `neighbors_to_json` is scalar-only and could be rewritten either:
  - with interpolation, keeping the current `String`-fragment model, or
  - with `json {}` by converting empty-string IDs to `None`/`Some(...)` and deleting `format_neighbor_id`
- `build_detail_response` still needs raw-fragment composition because it nests prebuilt `detail_json` and `nav_json`.

### `mesher/api/search.mpl`

Relevant seam:
- already-good `json {}` serializers:
  - `row_to_issue_json` (`:43`)
  - `row_to_event_json` (`:60`)
  - `row_to_issue_event_json` (`:82`)
- remaining cleanup targets:
  - `row_to_tag_event_json` (`:71`)
  - `extract_cursor_from_last` (`:92`)
  - `build_paginated_response` (`:102`)
  - `build_event_paginated_response` (`:112`)
  - dynamic tag JSON in `check_tag_params` (`:230`)

Why the file is split between macro and interpolation:
- `mesher/storage/queries.mpl:444` (`filter_events_by_tag`) returns `tags::text`, so `row_to_tag_event_json` must embed a raw JSON fragment for `tags`.
- The pagination builders assemble `{"data":<raw array>, ...}` responses; `json {}` cannot directly express “insert this already-serialized JSON array raw”.
- `check_tag_params` builds a **dynamic-key** JSON object for `tags @> ?::jsonb`; `json {}` cannot represent a runtime-generated key name, so interpolation is the direct fit.

Natural local outcome:
- keep the three existing `json {}` helpers as-is
- rewrite only the raw-fragment and dynamic-key helpers
- keep using `to_json_array(...)` from `Api.Helpers`

### `mesher/storage/queries.mpl`

Relevant seam:
- non-SQL interpolation wins:
  - `create_api_key` (`:98`) — `"mshr_" <> Crypto.uuid4()`
  - `create_session` (`:186`) — `uuid1 <> uuid2`
- wrapping-style `List.map` survivors:
  - `list_orgs` (`:51`)
  - `list_projects_by_org` (`:83`)
  - `get_members` (`:231`)
  - `list_issues_by_status` (`:361`)
- designated keep sites:
  - `event_volume_hourly` (`:482`) — SQL expression assembly
  - `drop_partition` (`:786`) — DDL assembly

Recommended treatment:
- Convert the two non-SQL string concatenations to interpolation:
  - `"mshr_#{Crypto.uuid4()}"`
  - `"#{uuid1}#{uuid2}"`
- Convert the four `Ok(List.map(rows, fn...))` returns to the existing pipe house style.
- Leave the SQL-adjacent `<>` sites alone exactly as the roadmap and milestone context specify.

## Recommendation

Two implementation tasks are enough.

### T01: API serializer cleanup in `mesher/api/alerts.mpl`, `mesher/api/detail.mpl`, `mesher/api/search.mpl`

Goal:
- remove the 14 non-SQL `<>` sites in those three files
- keep JSON output shapes unchanged
- use `json {}` only where values are already true scalar/option values
- use triple-quoted `#{}` interpolation for raw JSON fragments and nested prebuilt JSON

Why this is one task:
- same risk profile across all three files
- same prior art (`to_json_array`, raw JSON interpolation)
- all work stays in HTTP serialization helpers

### T02: Storage cleanup in `mesher/storage/queries.mpl`

Goal:
- replace the 2 non-SQL concatenations with interpolation
- replace the 4 `Ok(List.map(rows, ...))` wrappers with pipe style
- leave SQL/DDL `<>` alone

Why separate:
- independent from API serialization work
- lower risk and easy to verify with grep + build
- avoids mixing query helper cleanup with HTTP response-shape work

## Verification

### Authoritative slice gate
- `cargo run -p meshc -- build mesher`

### Required text/grep proofs
- `rg -n '<>' mesher -g '*.mpl'`
  - after S02 this should show **only**:
    - `mesher/storage/queries.mpl:486`
    - `mesher/storage/queries.mpl:787`
    - `mesher/storage/schema.mpl:11`
    - `mesher/storage/schema.mpl:12`
    - `mesher/storage/schema.mpl:13`

- `rg -n 'List\.map\(rows,|Ok\(List\.map\(' mesher -g '*.mpl'`
  - should return **0**
  - this is the reliable S02 pipe-style proof

### Non-gates / avoid wasting time
- Do **not** treat `cargo run -p meshc -- fmt --check mesher` as an S02 completion gate. Final formatter compliance and multiline imports belong to S03.
- Do **not** use `rg 'List\.map(' mesher -g '*.mpl'` as a gate; it matches the desired pipe style too.

## Risks and Watchouts

1. **Type drift is the real regression risk.**
   - The dangerous rewrite is not syntax; it is accidentally turning booleans, ints, nulls, or JSONB blobs into quoted strings.
   - `alerts.mpl` is the sharpest edge because `enabled`, `cooldown_minutes`, and nullable timestamps all come through as text.

2. **Do not invent shared JSON infrastructure for this slice.**
   - There is no existing raw-fragment-aware JSON helper. Building one now would expand scope from cleanup into language/app infrastructure.
   - Local interpolation rewrites are enough for S02.

3. **Do not touch designated keep sites.**
   - `schema.mpl` DDL and the two SQL-adjacent `queries.mpl` sites are part of the accepted end state.

4. **Do not quietly absorb S03 while touching `alerts.mpl`.**
   - `mesher/api/alerts.mpl:6` still has a 154-char import line.
   - Leave import wrapping and broader formatter-cleanup work to S03.

## Skills Discovered

### Loaded
- `debug-like-expert`
  - Applied its **VERIFY, DON'T ASSUME** rule: I checked the actual query return shapes and ran a real `meshc build mesher` before recommending `json {}` vs interpolation.
- `test`
  - Its “match the existing test/verification surface” rule supports using the repo’s current build/grep gates rather than inventing new bespoke tests for a cleanup slice.
- `lint`
  - Its “use the project’s own toolchain” rule supports keeping verification on `meshc`/repo-native commands rather than external formatters or ad hoc style checks.

### Searched
- `npx skills find "rust"`
  - surfaced generic Rust skills such as `apollographql/skills@rust-best-practices`
- `npx skills find "postgresql"`
  - surfaced generic Postgres design/review skills such as `wshobson/agents@postgresql-table-design`

### Installed
- None.
  - This slice is local Mesher/Mesh cleanup with strong in-repo prior art. Extra external skills would add little and widen scope.
