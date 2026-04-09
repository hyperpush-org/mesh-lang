---
id: T01
parent: S04
milestone: M031
provides:
  - Zero `let _ =` side-effect bindings across all mesher .mpl files
  - Three nested else/if blocks flattened to `else if` chains
key_files:
  - mesher/ingestion/pipeline.mpl
  - mesher/ingestion/routes.mpl
  - mesher/storage/queries.mpl
  - mesher/services/retention.mpl
  - mesher/services/writer.mpl
  - mesher/ingestion/ws_handler.mpl
  - mesher/api/search.mpl
key_decisions: []
patterns_established: []
observability_surfaces:
  - none
duration: 8m
verification_result: passed
completed_at: 2026-03-24
blocker_discovered: false
---

# T01: Remove `let _ =` and flatten `else if` across mesher

**Removed all 72 `let _ =` side-effect bindings and flattened 3 nested else/if blocks to `else if` chains across 7 mesher files**

## What Happened

Mechanically removed `let _ = ` prefixes from all 72 side-effect call sites across 6 files (pipeline.mpl: 35, routes.mpl: 14, queries.mpl: 14, retention.mpl: 6, writer.mpl: 2, ws_handler.mpl: 1), converting them to bare expression statements. Flattened 3 nested `else` + `if` blocks into `else if` chains: one in `pipeline.mpl` (peer-change detection in `load_monitor`) and two in `search.mpl` (`cap_limit` and `filter_by_tag_inner`).

## Verification

- `rg 'let _ =' mesher/ -g '*.mpl'` → 0 matches (exit 1 from rg = no matches)
- `cargo run -p meshc -- build mesher` → exit 0, compiled cleanly
- `cargo test -p meshc --test e2e` → 313 passed, 10 failed (same pre-existing failures)
- `cargo run -p meshc -- fmt --check mesher` → exit 1 (35 files would be reformatted — pre-existing, T02 scope)

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `rg 'let _ =' mesher/ -g '*.mpl'` | 1 | ✅ pass (0 matches) | <1s |
| 2 | `cargo run -p meshc -- build mesher` | 0 | ✅ pass | 12s |
| 3 | `cargo run -p meshc -- fmt --check mesher` | 1 | ⏳ expected (pre-existing, T02 scope) | 8s |
| 4 | `cargo test -p meshc --test e2e` | 1 | ✅ pass (313 pass, 10 pre-existing fail) | 198s |

## Diagnostics

None — this is a pure syntax cleanup with no runtime behavior change.

## Deviations

None.

## Known Issues

- `cargo run -p meshc -- fmt --check mesher` reports 35 files needing reformat — this is pre-existing and will be addressed by T02 (interpolation + multiline imports).

## Files Created/Modified

- `mesher/ingestion/pipeline.mpl` — Removed 35 `let _ =`, flattened 1 nested else/if
- `mesher/ingestion/routes.mpl` — Removed 14 `let _ =`
- `mesher/storage/queries.mpl` — Removed 14 `let _ =`
- `mesher/services/retention.mpl` — Removed 6 `let _ =`
- `mesher/services/writer.mpl` — Removed 2 `let _ =`
- `mesher/ingestion/ws_handler.mpl` — Removed 1 `let _ =`
- `mesher/api/search.mpl` — Flattened 2 nested else/if to `else if` chains
