---
estimated_steps: 4
estimated_files: 6
skills_used:
  - debug-like-expert
  - test
  - lint
---

# T01: Rewrite API serializers with type-preserving JSON and interpolation

**Slice:** S02 — Mesher JSON serialization and pipe cleanup
**Milestone:** M029

## Description

Rewrite the remaining API-side JSON serializers in `alerts`, `detail`, and `search` using the same split Mesher already uses elsewhere: `json {}` for scalar-only payloads and interpolation for raw JSON fragments or dynamic keys. The backing queries return a mix of plain strings, numeric/boolean text, nullable timestamps, and JSONB text, so this task must preserve output types and null semantics while removing the hard-to-read `<>` chains.

## Steps

1. Read the serializer helpers in `mesher/api/alerts.mpl`, `mesher/api/detail.mpl`, and `mesher/api/search.mpl`, then confirm the backing row shapes in `mesher/storage/queries.mpl` for `list_alert_rules`, `list_alerts`, `get_event_detail`, and `filter_events_by_tag`.
2. Rewrite `mesher/api/alerts.mpl` so `format_nullable_ts`, `rule_row_to_json`, and `alert_row_to_json` stop using `<>` while still embedding raw `condition_json`, `action_json`, `enabled`, `cooldown_minutes`, and `condition_snapshot` values without extra quoting.
3. Rewrite `mesher/api/detail.mpl` and the remaining raw-fragment helpers in `mesher/api/search.mpl` so JSONB fragments, pagination wrappers, nested prebuilt JSON, and dynamic tag JSON use interpolation, while the existing scalar-only `json {}` helpers stay intact.
4. Prove the cleanup by removing all `<>` from the three API files and rebuilding `mesher`.

## Must-Haves

- [ ] `mesher/api/alerts.mpl`, `mesher/api/detail.mpl`, and `mesher/api/search.mpl` contain no remaining `<>` sites.
- [ ] Raw JSON/query-text fields stay embedded without double quoting, and nullable timestamp/id helpers keep their current `null` vs quoted-string behavior.
- [ ] Existing `json {}` serializers remain the preferred style for scalar-only payloads instead of being replaced with more interpolation.

## Verification

- `! rg -n '<>' mesher/api/alerts.mpl mesher/api/detail.mpl mesher/api/search.mpl`
- `cargo run -q -p meshc -- build mesher`

## Observability Impact

- Signals added/changed: none; this task changes response serialization only.
- How a future agent inspects this: compare the helper source against the backing query field shapes in `mesher/storage/queries.mpl`, then rerun `cargo run -q -p meshc -- build mesher` and the targeted `rg` check.
- Failure state exposed: accidental double-quoting or null/string drift remains localized to `rule_row_to_json`, `alert_row_to_json`, `event_detail_to_json`, pagination builders, and `check_tag_params`.

## Inputs

- `mesher/api/alerts.mpl` — alert serializer helpers to rewrite
- `mesher/api/detail.mpl` — event detail/navigation helpers with raw JSONB fields
- `mesher/api/search.mpl` — remaining raw-fragment and pagination helpers plus existing `json {}` style
- `mesher/storage/queries.mpl` — authoritative query return shapes that determine which fields are raw JSON text versus scalar text
- `mesher/api/helpers.mpl` — `to_json_array` helper and existing response-shape conventions
- `mesher/ingestion/routes.mpl` — prior-art raw JSON interpolation pattern already used in Mesher

## Expected Output

- `mesher/api/alerts.mpl` — alert serializers rewritten without `<>`
- `mesher/api/detail.mpl` — detail/navigation serializers rewritten without `<>`
- `mesher/api/search.mpl` — remaining raw-fragment and pagination helpers rewritten without `<>`
