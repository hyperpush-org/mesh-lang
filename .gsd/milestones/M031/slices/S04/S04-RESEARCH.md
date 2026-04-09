# S04 Research: Mesher Dogfood Cleanup

**Depth:** Light — identical patterns to S03 applied to a larger codebase. No new compiler work, no unknown APIs.

## Summary

S04 applies the same mechanical cleanup proven in S03 (reference-backend) to the mesher codebase: remove `let _ =`, replace `<>` with interpolation where appropriate (per D029), convert long imports to multiline, and flatten nested if/else to `else if`. No `== true` patterns exist in mesher. No struct update opportunities exist (mesher services use simpler state shapes). The work is high-count but low-risk.

## Requirement Coverage

- **R024 (active, primary owner):** `mesher/` should have zero `let _ =` for side effects, string interpolation replacing `<>` concatenation where appropriate, multiline imports for long lines, and pipe operators used idiomatically.

## Implementation Landscape

### `let _ =` removal — 72 instances across 6 files

| File | Count | Pattern |
|------|-------|---------|
| `ingestion/pipeline.mpl` | 35 | `println()`, `Ws.broadcast()`, `spawn()`, `Process.register()`, `Global.register()`, service calls |
| `ingestion/routes.mpl` | 14 | `Ws.broadcast()`, `PipelineRegistry.increment_event_count()`, `broadcast_issue_update()`, `check_event_alerts()` |
| `storage/queries.mpl` | 14 | `Repo.insert()`, `Repo.update_where()`, `Repo.delete_where()`, `Repo.execute_raw()` — mostly discarded row-count returns |
| `services/retention.mpl` | 6 | `println()`, `drop_partition()`, `drop_partitions_loop()` |
| `services/writer.mpl` | 2 | `flush_with_retry()` |
| `ingestion/ws_handler.mpl` | 1 | `Ws.join()` |

All are side-effect calls whose return values are intentionally discarded. Same pattern as S03. Bare expression statements are confirmed working since S01.

### `<>` → interpolation — 32 instances across 11 files

**Replace with interpolation (clear wins):**
- `ingestion/validation.mpl:1` — `"too many events (max " <> String.from(max_events) <> ")"` → `"too many events (max #{max_events})"`
- `ingestion/ws_handler.mpl:2` — `"project:" <> project_id`, `"[WS] Connection closed: " <> String.from(code)`
- `ingestion/fingerprint.mpl:5` — Frame fingerprint construction (`filename <> "|" <> function_name`, `joined <> suffix`, etc.)
- `services/event_processor.mpl:1` — `build_enriched_entry` using `<>` for delimiter-joined string
- `api/helpers.mpl:1` — `to_json_array` uses `"[" <> String.join(items, ",") <> "]"` — borderline, but interpolation is fine

**Keep `<>` (per D029 — raw SQL/JSON construction where structure visibility matters):**
- `storage/schema.mpl:3` — SQL DDL construction for partition CREATE TABLE statements
- `storage/queries.mpl:4` — SQL expression interpolation inside `Query.select_raw` calls (`"date_trunc('" <> bucket <> "', ...)"`, `create_api_key` key construction, `create_session` token construction, `drop_partition` DDL)
- `api/detail.mpl:4` — Manual JSON construction with raw JSONB field embedding (16+ field JSON string with mixed quoting). These are the trickiest — the `<>` chains construct JSON where some fields need quotes and some don't (raw JSONB). Interpolation would work but would be a very long `#{}` chain. **Judgment call: convert if clearly more readable, keep if not.**
- `api/search.mpl:7` — Mix of JSON construction (`row_to_tag_event_json`, `extract_cursor_from_last`, `build_paginated_response`) and tag JSON building. Tag event JSON has the same mixed-quoting issue as detail. Pagination cursor uses `<>` for JSON fragment building.
- `api/alerts.mpl:3` — `rule_row_to_json` and `alert_row_to_json` — same manual JSON construction with embedded raw JSONB fields.

**Decision point for manual JSON builders:** `api/detail.mpl::event_detail_to_json`, `api/search.mpl::row_to_tag_event_json`, `api/alerts.mpl::rule_row_to_json`, `api/alerts.mpl::alert_row_to_json` all build JSON strings manually because they embed raw JSONB fields that can't be double-quoted. These use many `<>` operators but converting to interpolation won't simplify them — the pattern is inherently complex because of mixed quoting. **Leave these as `<>`** per D029.

### Multiline import conversion — 13 lines over 100 chars

Candidates for `from Module import (\n  a,\n  b\n)` conversion:

| File | Chars | Import |
|------|-------|--------|
| `ingestion/routes.mpl` | 310 | `Storage.Queries` — 14 functions |
| `main.mpl` | 208 | `Api.Alerts` — 7 functions |
| `main.mpl` | 202 | `Ingestion.Routes` — 8 functions |
| `api/dashboard.mpl` | 193 | `Storage.Queries` — 6 functions |
| `main.mpl` | 192 | `Api.Team` — 7 functions |
| `api/alerts.mpl` | 176 | `Storage.Queries` — 7 functions |
| `main.mpl` | 172 | `Api.Dashboard` — 6 functions |
| `services/user.mpl` | 168 | `Storage.Queries` — 8 functions |
| `api/team.mpl` | 164 | `Storage.Queries` — 7 functions |
| `services/project.mpl` | 161 | `Storage.Queries` — 6 functions |
| `services/retention.mpl` | 146 | `Storage.Queries` — 4 functions |
| `api/search.mpl` | 139 | `Storage.Queries` — 4 functions |
| `ingestion/pipeline.mpl` | 135 | `Storage.Queries` — 4 functions |

**Threshold:** Convert all imports over ~120 chars. Lines 100-120 chars are borderline — keep single-line unless they're clearly hard to read. The `main.mpl` file has 7 long imports alone.

### Nested if/else → `else if` — 3 instances

| File | Location | Pattern |
|------|----------|---------|
| `ingestion/pipeline.mpl:313-322` | `load_monitor` — peer count change detection | `else` + newline + `if node_count < prev_peers do` |
| `api/search.mpl:14-20` | `cap_limit` — limit capping | `else` + newline + `if n < 1 do` |
| `api/search.mpl:235-240` | `check_tag_params` — tag parameter validation | `else` + newline + `if val_empty do` |

### Pipe operators

Mesher already uses 155 pipe operators (101 in `storage/queries.mpl` alone). The pipe usage is already idiomatic. No additional pipe conversions needed beyond what exists.

## Constraints

- **D029 applies:** Only replace `<>` where interpolation is clearly more readable. Keep `<>` for SQL DDL, raw JSONB embedding, and complex manual JSON construction.
- **No `== true` in mesher** — unlike reference-backend, mesher doesn't have this anti-pattern.
- **No struct update opportunities** — mesher's structs (`WriterState`, `RegistryState`, `ProcessorState`) are reconstructed in service state transitions, but `pipeline.mpl`'s `PipelineRegistry` already reconstructs explicitly (the call handlers return `(state, value)` tuples). These are legitimate full reconstructions since services need the tuple return shape.
- **Test surface:** `cargo run -p meshc -- build mesher` is the primary gate. No mesher-specific project tests exist (just `fingerprint.test.mpl` and `validation.test.mpl` which are standalone). Full e2e suite (`cargo test -p meshc --test e2e`) must stay green.
- **Formatter:** `cargo run -p meshc -- fmt --check mesher` should pass after cleanup (verify this works — S03 proved it for reference-backend).

## Recommendation

Two tasks, split by risk/volume:

**T01 — `let _ =` removal + `else if` flattening (all 6 files + 3 else-if sites):**
The mechanical bulk. 72 `let _ =` removals across `pipeline.mpl` (35), `routes.mpl` (14), `queries.mpl` (14), `retention.mpl` (6), `writer.mpl` (2), `ws_handler.mpl` (1). Plus 3 nested if/else → `else if` conversions. Build-verify after each file or batch.

**T02 — `<>` → interpolation + multiline imports (11 files with `<>`, 8 files with long imports):**
The judgment-heavy work. `<>` replacements require per-case evaluation (D029). Multiline import conversion is mechanical. Some files appear in both T01 and T02 — edits should not conflict since they touch different lines.

Verification for both:
- `rg 'let _ =' mesher/ -g '*.mpl'` → 0 matches
- `cargo run -p meshc -- build mesher` → success
- `cargo run -p meshc -- fmt --check mesher` → success (if supported)
- `cargo test -p meshc --test e2e` → 313+ pass, 10 pre-existing failures

## Files to Modify

| File | T01 changes | T02 changes |
|------|-------------|-------------|
| `ingestion/pipeline.mpl` | 35 `let _ =`, 1 `else if` | 0 `<>` (interpolation already used), 1 long import |
| `ingestion/routes.mpl` | 14 `let _ =` | 1 `<>`, 2 long imports |
| `storage/queries.mpl` | 14 `let _ =` | Keep `<>` (SQL/crypto construction) |
| `services/retention.mpl` | 6 `let _ =` | 0 `<>`, 1 long import |
| `services/writer.mpl` | 2 `let _ =` | 0 `<>` |
| `ingestion/ws_handler.mpl` | 1 `let _ =` | 2 `<>` → interpolation |
| `ingestion/validation.mpl` | 0 | 1 `<>` → interpolation |
| `ingestion/fingerprint.mpl` | 0 | 5 `<>` → interpolation (frame construction) |
| `services/event_processor.mpl` | 0 | 1 `<>` → interpolation (enriched entry) |
| `api/helpers.mpl` | 0 | 1 `<>` → interpolation (`to_json_array`) |
| `api/search.mpl` | 0 (2 `else if`) | Keep `<>` (manual JSON), 1 long import |
| `api/detail.mpl` | 0 | Keep `<>` (manual JSON with raw JSONB) |
| `api/alerts.mpl` | 0 | Keep `<>` (manual JSON with raw JSONB), 1 long import |
| `api/dashboard.mpl` | 0 | 0 `<>`, 2 long imports |
| `api/team.mpl` | 0 | 0 `<>`, 1 long import |
| `api/settings.mpl` | 0 | 0 `<>`, 0 long imports (all <120 chars) |
| `services/user.mpl` | 0 | 0 `<>`, 1 long import |
| `services/project.mpl` | 0 | 0 `<>`, 1 long import |
| `main.mpl` | 0 | 0 `<>`, 5 long imports |
| `storage/schema.mpl` | 0 | Keep `<>` (SQL DDL) |

## Risks

None significant. All patterns are proven by S03. The only judgment calls are per-instance `<>` vs interpolation decisions, which are low-stakes and reversible.
