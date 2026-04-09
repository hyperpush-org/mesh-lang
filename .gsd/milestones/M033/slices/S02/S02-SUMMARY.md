---
id: S02
parent: M033
milestone: M033
provides:
  - Explicit PostgreSQL helper usage for Mesher's pgcrypto auth, full-text search, JSONB insert/filter/breakdown/defaulting, and alert-rule storage flows.
  - A live Postgres proof bundle plus verifier script for the S02 helper families.
  - A named and mechanically enforced raw-boundary contract around `extract_event_fields` for S03.
requires:
  - slice: S01
    provides: S01's neutral Expr/Query/Repo serializer contract, expression-valued select/update/upsert plumbing, and stable placeholder-handling rules.
affects:
  - S03
  - S04
  - S05
key_files:
  - compiler/mesh-rt/src/db/expr.rs
  - compiler/mesh-rt/src/db/query.rs
  - compiler/mesh-rt/src/db/repo.rs
  - mesher/storage/queries.mpl
  - mesher/storage/writer.mpl
  - compiler/meshc/tests/e2e_m033_s02.rs
  - scripts/verify-m033-s02.sh
key_decisions:
  - Keep PostgreSQL-only behavior explicit under `Pg` plus structured `Expr.fn_call(...)` instead of widening the neutral `Expr` API.
  - Keep `extract_event_fields` as the explicit S03 raw keep-site because its fingerprint fallback still depends on CASE + WITH ORDINALITY + scalar-subquery behavior.
  - Use temporary Mesh projects that copy Mesher `storage/` and `types/` modules for the live Postgres proof bundle instead of relying on the S01 HTTP readiness path.
  - Cast `jsonb_build_object(...)` string arguments with `Pg.text(...)` in the structured expression path so `fire_alert` stays on the PG helper surface without PostgreSQL parameter-type ambiguity.
patterns_established:
  - Build vendor-specific behavior on top of the neutral expression core with explicit `Pg.*` helpers plus structured `Expr.fn_call(...)`, not by widening the neutral API or dropping back to whole-query raw SQL.
  - Prove Mesher storage helpers through temporary Mesh projects that copy the real `storage/` and `types/` modules, then assert directly against live Postgres rows to localize failures to the data-layer boundary.
  - Enforce honest raw-boundary discipline mechanically with a function-block keep-list sweep that names the only allowed leftover and requires its explanatory comment to stay in sync.
observability_surfaces:
  - `compiler/meshc/tests/e2e_m033_s02.rs` named `e2e_m033_s02_*` failures for auth/search/jsonb/alert/defaulting families.
  - `scripts/verify-m033-s02.sh` for the full slice replay plus raw keep-list sweep.
  - Direct Postgres row assertions against `users`, `events`, `alert_rules`, `alerts`, and `issues` inside the Rust harness.
  - Verifier-enforced `extract_event_fields` raw-boundary comment and keep-site check.
drill_down_paths:
  - .gsd/milestones/M033/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M033/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M033/slices/S02/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-25T17:55:50.755Z
blocker_discovered: false
---

# S02: Explicit PG extras for JSONB, search, and crypto

**Explicit PostgreSQL helper surfaces now power Mesher's auth, search, JSONB, and alert storage paths on live Postgres, with a passing proof bundle and an enforced raw keep-list boundary for the remaining S03 read-side holdout.**

## What Happened

S02 finished the PostgreSQL-specific half of the new data-layer boundary on top of S01's neutral expression core. T01 extended the runtime/compiler seam so compiled Mesh code can build casted expressions, PG crypto/search/JSONB helpers, expression-valued WHERE/SELECT clauses, and expression-valued inserts without smuggling those capabilities into the neutral API. T02 then moved the real Mesher runtime paths for pgcrypto auth, full-text search, JSONB tag filtering/breakdown, event insert/defaulting, and alert-rule create/fire/filter helpers onto that explicit PG surface while keeping `extract_event_fields` called out as the honest S03 raw read-side boundary. T03 added the direct Postgres-backed proof bundle and verifier script, and the closeout pass fixed the last red proofs by hoisting map lookups out of Mesh string interpolation in the temporary probe programs, importing the concrete `User` type in the auth probe, adding `Pg.text(...)` casts around `jsonb_build_object` arguments in `fire_alert`, and moving the S03 keep-site marker into the `extract_event_fields` function body so the verifier can enforce it mechanically. The assembled slice now delivers explicit PG helper usage on the live Mesher auth/search/jsonb/alert runtime path, plus a named keep-list boundary and proof harness that downstream slices can reuse.

## Verification

Verified the assembled slice with the full slice contract from the plan. `cargo test -p meshc --test e2e_m033_s02 -- --nocapture` passed with all five live Postgres-backed proofs green. `cargo run -q -p meshc -- fmt --check mesher` passed. `cargo run -q -p meshc -- build mesher` passed. `bash scripts/verify-m033-s02.sh` passed and revalidated the end-to-end proof bundle plus the raw keep-list sweep. The observability surfaces from the plan are live: failures localize to named `e2e_m033_s02_*` helper families, the harness inspects direct row snapshots from `users`, `events`, `alert_rules`, `alerts`, and `issues`, and the verifier script enforces the explicit `extract_event_fields` raw-boundary contract.

## Requirements Advanced

- R037 — S02 shipped explicit PG helper usage for Mesher's auth/search/JSONB/alert runtime paths and proved those helpers on live Postgres through `cargo test -p meshc --test e2e_m033_s02 -- --nocapture` plus `bash scripts/verify-m033-s02.sh`.
- R036 — S02 preserved the honest split between the neutral expression core and PG-only behavior by keeping JSONB/search/crypto work under `Pg` plus structured vendor-specific function calls instead of expanding the neutral API.
- R038 — S02 shrank the real raw keep-list to a named `extract_event_fields` holdout and added a verifier sweep that fails if the owned helper families regress back to raw query fragments.
- R040 — S02 kept the later SQLite seam intact by adding explicit PG-only helper usage without baking PostgreSQL behavior into the neutral `Expr`/`Query` contract.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None.

## Known Limitations

`extract_event_fields` intentionally remains a raw S03 keep-site because the fingerprint fallback still depends on CASE + WITH ORDINALITY + scalar-subquery behavior that the current expression surface does not cover honestly. Partition/schema helpers are still pending S04, and the public docs plus final integrated replay are still pending S05.

## Follow-ups

S03 should target `extract_event_fields` and the remaining hard read-side raw families using the verifier's named raw boundary as the handoff contract. S04 should land the partition/schema helper surfaces so R037 can move from advanced to validated. S05 should document the neutral-vs-PG boundary and replay the full assembled Mesher data-layer acceptance story end to end.

## Files Created/Modified

- `compiler/mesh-rt/src/db/expr.rs` — Added cast-capable expression nodes plus explicit PostgreSQL helper intrinsics for JSONB, full-text search, pgcrypto, and typed casts.
- `compiler/mesh-rt/src/db/query.rs` — Added expression-valued WHERE and SELECT query plumbing, including ordered select-parameter tracking for composed SQL fragments.
- `compiler/mesh-rt/src/db/repo.rs` — Added expression-valued INSERT/UPDATE SQL builders and placeholder renumbering for composed expression params.
- `compiler/mesh-rt/src/lib.rs` — Exported the new expression and PostgreSQL runtime entrypoints to compiled Mesh programs.
- `compiler/mesh-typeck/src/infer.rs` — Registered type schemes for the new Pg helpers and expression-aware Query/Repo entrypoints.
- `compiler/mesh-codegen/src/mir/lower.rs` — Lowered Pg helper calls and expression intrinsics through MIR generation.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — Wired codegen support for the new runtime intrinsics.
- `mesher/storage/queries.mpl` — Rewrote Mesher auth, search, JSONB, and alert helpers onto explicit PG/Expr surfaces, preserved `extract_event_fields` as the named S03 raw keep-site, and fixed `fire_alert` JSONB snapshot typing with `Pg.text(...)` casts.
- `mesher/storage/writer.mpl` — Moved live event insert/defaulting onto `Repo.insert_expr` with explicit JSONB extraction/defaulting helpers.
- `compiler/meshc/tests/e2e_m033_s02.rs` — Added the live Postgres-backed S02 proof bundle and repaired the temporary probe programs so auth/search/jsonb/alert/defaulting flows compile and assert correctly.
- `scripts/verify-m033-s02.sh` — Added the slice verifier script and raw keep-list sweep for the owned S02 helper boundary.
- `.gsd/DECISIONS.md` — Recorded the S02 proof-harness decision for future slices.
- `.gsd/KNOWLEDGE.md` — Captured S02-specific debugging guidance for Mesh probe interpolation, `jsonb_build_object` typing, and copied-project type imports.
- `.gsd/REQUIREMENTS.md` — Advanced R037 with the new S02 proof evidence while leaving partition/schema completion to S04.
- `.gsd/PROJECT.md` — Updated project state to mark M033/S02 complete and shift the forward plan to S03/S04/S05.
