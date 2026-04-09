---
id: M033
title: "ORM Expressiveness & Schema Extras"
status: complete
completed_at: 2026-03-26T16:33:29.195Z
key_decisions:
  - Sequence M033 as neutral expression core first, explicit PostgreSQL extras second, hard read-side collapse third, schema/partition helpers fourth, and docs/integrated replay last.
  - Keep the portable baseline centered on `Expr`, `Query`, `Repo`, and a narrowly honest `Migration.create_index(...)` surface rather than inventing a fake universal SQL AST.
  - Expose PostgreSQL-only behavior explicitly under `Pg.*` plus structured expression calls instead of widening the neutral API to cover JSONB, full-text search, crypto, or partition-specific behavior.
  - Use live Mesher HTTP/API verification for S03 composed reads once the copied storage probe stopped being an honest signal for caller-contract correctness.
  - Keep the remaining unstable read families as an explicit named raw keep-list enforced by verifier scripts rather than forcing dishonest builder rewrites.
  - Own runtime partition lifecycle in `mesher/storage/schema.mpl` over explicit `Pg.*` helpers instead of widening generic query helpers.
  - Close the milestone with a serial docs-plus-verifier replay (`bash scripts/verify-m033-s05.sh`) rather than inventing another end-to-end harness.
  - Keep `get_event_alert_rules(...)` and `get_threshold_rules(...)` on the S03 honest raw-read ledger so docs, verifiers, and runtime ownership stay aligned.
key_files:
  - compiler/mesh-rt/src/db/expr.rs
  - compiler/mesh-rt/src/db/query.rs
  - compiler/mesh-rt/src/db/repo.rs
  - compiler/mesh-rt/src/db/migration.rs
  - compiler/mesh-rt/src/db/pg_schema.rs
  - compiler/mesh-typeck/src/infer.rs
  - compiler/mesh-codegen/src/mir/lower.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/meshc/tests/e2e_m033_s01.rs
  - compiler/meshc/tests/e2e_m033_s02.rs
  - compiler/meshc/tests/e2e_m033_s03.rs
  - compiler/meshc/tests/e2e_m033_s04.rs
  - mesher/storage/queries.mpl
  - mesher/storage/schema.mpl
  - mesher/storage/writer.mpl
  - mesher/services/event_processor.mpl
  - mesher/migrations/20260216120000_create_initial_schema.mpl
  - scripts/verify-m033-s01.sh
  - scripts/verify-m033-s02.sh
  - scripts/verify-m033-s03.sh
  - scripts/verify-m033-s04.sh
  - scripts/verify-m033-s05.sh
  - website/docs/docs/databases/index.md
lessons_learned:
  - When a copied storage probe starts reproducing compiler/runtime artifacts that the real app path does not share, move verification up to the live Mesher surface instead of extending a misleading harness.
  - Vendor-specific power is easier to keep honest when the neutral core stays small; pushing JSONB/search/crypto/partition behavior under explicit `Pg.*` helpers prevented fake portability and made the docs/proof surface cleaner.
  - Mechanical verifier scripts are part of the product truth surface, not garnish. The raw-boundary and docs-truth sweeps caught ownership drift and stale assumptions that prose alone would have missed.
  - Live route behavior can still diverge from storage correctness on the Mesher path; direct DB assertions, structured route normalization, and explicit writer flushes were required to make the proof surface truthful.
  - Schema and retention work are easier to reason about when runtime partition lifecycle lives in one `Storage.Schema` layer over explicit PG helpers rather than being scattered through generic query code.
---

# M033: ORM Expressiveness & Schema Extras

**Shipped an honest neutral Mesh expression/query/migration core plus explicit PostgreSQL extras, moved Mesher’s real data-layer pressure sites onto those surfaces where truthful, and closed the milestone with live Postgres proof and public docs that lock the boundary in place.**

## What Happened

M033 started from the M032 handoff that narrowed Mesher’s remaining platform pressure to the data layer, then sequenced the work in the right risk order instead of chasing a fake universal ORM. S01 landed the neutral expression core (`Expr`, `Query.select_exprs`, expression-aware `Repo` entrypoints) and proved it on real Postgres-backed Mesher write paths including issue upsert, assign/unassign, alert acknowledge/resolve, settings updates, first-event ingest, and low-volume event persistence. S02 kept PostgreSQL-only behavior explicit under `Pg.*` and structured expression calls for the JSONB, search, crypto, alert, and event-defaulting families rather than widening the neutral API. S03 used the new builders and helper seams to retire the honest recurring read families, pivoted verification from a misleading copied storage probe to a live Mesher HTTP/API harness when the probe stopped being truthful, and reduced the remaining raw-read tail to a short named keep-list enforced by `scripts/verify-m033-s03.sh`. S04 finished the schema side by extending only the honest neutral migration surface (`Migration.create_index(...)` with exact names, ordering, and partial predicates) while routing extensions, GIN/opclass indexes, partitioned parents, and runtime partition lifecycle through explicit `Pg.*` and `Storage.Schema` helpers with live catalog proof. S05 then turned the shipped boundary into a public and mechanically replayable contract: the database docs now describe the real portable core, explicit PostgreSQL extras, named raw escape hatches, and SQLite-later seam, and `bash scripts/verify-m033-s05.sh` serially replays docs build/truth plus the S02/S03/S04 live-Postgres verifiers with named logs. The assembled milestone closes the honest boundary the roadmap promised: stronger Mesh data-layer surfaces where they stay truthful, explicit PostgreSQL extras where portability would be fake, a short named raw keep-list instead of folklore, and a credible later seam for SQLite-specific extras without pretending that runtime proof already exists.

## Success Criteria Results

- **Criterion 1 — Mesher’s recurring computed write, JSONB-heavy, search-heavy, alert, and partition-management paths use stronger Mesh ORM or migration surfaces wherever honest, with only a short justified raw keep-list left behind.** **Met.** Evidence: S01 moved issue upsert, assign/unassign, alert acknowledge/resolve, API-key revoke, settings updates, first-event ingest, and low-volume event persistence onto the neutral expression-aware `Query`/`Repo` path; S02 moved auth, search, JSONB-heavy storage, alert-rule storage, and event insert/defaulting onto explicit `Pg.*` helpers; S03 retired the honest recurring read families onto stronger Mesh query surfaces and reduced the remaining raw read tail to a short named keep-list enforced by `bash scripts/verify-m033-s03.sh`; S04 replaced the slice-owned migration/runtime partition raw DDL/query sites with helper-driven `Migration.*`, `Pg.*`, and `Storage.Schema` flows; S05 re-ran the assembled verifier chain and documented the remaining raw escape hatches publicly.
- **Criterion 2 — The neutral data-layer contract now includes a real expression DSL for reusable select/update/insert/upsert work instead of a literal-map-only write surface and ad hoc raw fragments.** **Met.** Evidence: S01 shipped the neutral `Expr` / `Query.select_exprs` / expression-aware `Repo` contract for structured SELECT, UPDATE, and ON CONFLICT work with stable placeholder ordering, and S03 proved that the same expression/query builders now cover real read-side families rather than only literal-map writes.
- **Criterion 3 — PostgreSQL-only behavior is exposed explicitly for the real PG-first families (`JSONB`, full-text search, pgcrypto, partition lifecycle, and related schema extras) rather than being hidden inside a misleading neutral API.** **Met.** Evidence: S02 kept JSONB, full-text search, and pgcrypto behavior explicit under `Pg.*` plus structured expression calls instead of widening the neutral API, and S04 did the same for extensions, GIN/opclass indexes, partitioned tables, and runtime partition lifecycle helpers. S05’s docs truth gate publicly locks that neutral-vs-`Pg.*` split.
- **Criterion 4 — The shipped design leaves a credible later seam for SQLite-specific extras without forcing M033 to implement or prove SQLite runtime behavior now.** **Met.** Evidence: Across S01, S02, S04, and S05, the shipped design keeps the neutral surface limited to honestly portable behavior and pushes PostgreSQL-only behavior into explicit namespaces and documented escape hatches. R040 remains active in the global ledger because future vendor-extra runtime proof is intentionally deferred, but the milestone promised a credible seam rather than shipped SQLite runtime behavior, and the delivered boundary/docs/verifier set substantiates that seam.
- **Criterion 5 — Public Mesh database docs explain the shipped neutral DSL and explicit PG extras through a Mesher-backed example path, and the live Postgres-backed Mesher flows still behave the same from the product point of view.** **Met.** Evidence: S05 rewrote `website/docs/docs/databases/index.md` around the real Mesher-backed API path, names the shipped neutral DSL and explicit `Pg.*` extras, and adds `bash scripts/verify-m033-s05.sh` as the canonical replay. Its verification confirms the docs build, exact-string truth sweep, and serial replay of `bash scripts/verify-m033-s02.sh`, `bash scripts/verify-m033-s03.sh`, and `bash scripts/verify-m033-s04.sh`, with the expected phase logs under `.tmp/m033-s05/verify/`.

## Definition of Done Results

- **Implementation delta outside `.gsd/` exists.** Verified using the local-main-safe baseline `git diff --stat HEAD $(git merge-base HEAD origin/main) -- ':!.gsd/'`. The delta includes compiler/runtime surfaces (`compiler/mesh-rt/src/db/{expr,query,repo,migration,pg_schema}.rs`, `compiler/mesh-typeck/src/infer.rs`, `compiler/mesh-codegen/src/{mir/lower,codegen/intrinsics}.rs`), Mesher runtime/storage code, verifier scripts, and docs.
- **All roadmap slices are complete.** `M033-ROADMAP.md` shows S01 through S05 as `[x]`.
- **All slice summary and UAT artifacts exist.** Present on disk: `S01`–`S05` summary files and `S01`–`S05` UAT files under `.gsd/milestones/M033/slices/`.
- **Cross-slice integration holds.** `M033-VALIDATION.md` is recorded with verdict `pass`; its slice delivery audit and cross-slice integration sections show no blocking handoff mismatches. The assembled proof surface also composes cleanly through `bash scripts/verify-m033-s05.sh`, which serially replays the delegated S02/S03/S04 verifiers.
- **Operational verification is explicit.** `M033-VALIDATION.md` now records `Operational Verification: MET.` and ties it to the live Postgres migration/partition/startup proof path from S04, preserved again in the S05 replay.
- **Horizontal checklist.** No separate roadmap horizontal checklist was present for M033, so there were no additional checklist items to audit beyond the success criteria and slice completion contract.

## Requirement Outcomes

- **R036 — active → validated.** Supported by the assembled M033 neutral-plus-explicit-extra proof set: `cargo test -p meshc --test e2e_m033_s01 expr_ -- --nocapture`, `cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture`, `cargo test -p meshc --test e2e_m033_s01 mesher_issue_upsert -- --nocapture`, `cargo test -p meshc --test e2e_m033_s02 -- --nocapture`, `cargo test -p meshc --test e2e_m033_s04 -- --nocapture`, `cargo run -q -p meshc -- fmt --check mesher`, `cargo run -q -p meshc -- build mesher`, `bash scripts/verify-m033-s01.sh`, `bash scripts/verify-m033-s02.sh`, and `bash scripts/verify-m033-s04.sh`.
- **R037 — active → validated.** Supported by the combined S02+S04 PG-extra proof path: `cargo test -p meshc --test e2e_m033_s02 -- --nocapture`, `cargo test -p meshc --test e2e_m033_s04 -- --nocapture`, `cargo run -q -p meshc -- fmt --check mesher`, `cargo run -q -p meshc -- build mesher`, `bash scripts/verify-m033-s02.sh`, and `bash scripts/verify-m033-s04.sh`.
- **R038 — active → validated.** Supported by the integrated S05 replay: `npm --prefix website run build`, `bash scripts/verify-m033-s05.sh`, the exact-string docs-truth sweep over `website/docs/docs/databases/index.md`, and the serial replay of `bash scripts/verify-m033-s02.sh`, `bash scripts/verify-m033-s03.sh`, and `bash scripts/verify-m033-s04.sh`, which together prove the public contract, the explicit `Pg.*` boundary, and the short named raw SQL/DDL keep-list stay honest.
- **R039 — active → validated.** Supported by `cargo test -p meshc --test e2e_m033_s04 -- --nocapture`, `cargo run -q -p meshc -- fmt --check mesher`, `cargo run -q -p meshc -- build mesher`, and `bash scripts/verify-m033-s04.sh`, which together prove helper-driven migration apply, runtime partition lifecycle, and removal of the old owned raw DDL/query sites.
- **R040 — remains active.** Evidence supports advancement, not validation: the neutral-core vs explicit-`Pg.*` vs SQLite-later seam is now enforced by the combined M033/S01+S04 proof set plus the public docs/truth gate from S05, but runtime validation still depends on future vendor-extra slices.
- **No unsupported requirement transitions were found.** The current requirement ledger aligns with the delivered slice evidence and the M033 validation verdict.

## Decision Re-evaluation

| Decision | Original choice | Still valid? | Notes / revisit |
|---|---|---|---|
| D052 | Sequence neutral core → PG extras → read collapse → schema helpers → docs replay. | Yes | The delivery order matched the real risk and kept the proof surface honest. |
| D053 | Build the neutral core around `Expr` plus expression-aware `Query`/`Repo`. | Yes | This became the stable base for S01, the read-side work in S03, and the later schema/doc boundary. |
| D054 | Keep PostgreSQL-only behavior under `Pg.*` and leave `extract_event_fields` raw unless a helper path stays obviously honest. | Yes | S02/S03 held this boundary and the milestone finished with an honest named raw tail instead of fake portability. |
| D055 | Make the live Mesher rate limiter testable through env overrides and restore the reset ticker. | Yes | This was required for truthful ingest/rate-limit proof on the real runtime path. |
| D056 | Use `Expr.label(...)` as the Mesh-callable alias surface until the parser keyword collision is repaired. | Yes, temporary | Revisit only when parser work is explicitly in scope; the temporary alias surface is still needed today. |
| D057 | Duplicate recording of the same `Expr.label(...)` temporary alias decision. | Yes, temporary | Same revisit condition as D056; no separate behavioral change landed from the duplicate record. |
| D058 | Carry real IDs in async writer payloads, start writers through `start_writer(...)`, and avoid unused integer success payloads on the flush path. | Yes | This fixed the real low-volume ingest/write persistence path and remained necessary through milestone closeout. |
| D059 | Use `Expr.fn_call(...)` with PostgreSQL-native functions for JSONB work instead of rushing more dedicated `Pg` wrappers. | Yes | It kept S02 expressive without widening the API or inventing premature helpers. |
| D060 | Exercise S02 through copied Mesher storage/types projects instead of the S01 HTTP readiness harness. | Yes | This stayed a good fit for S02 helper proofs; only S03 composed-read work needed a higher-level live harness later. |
| D061 | Prefer decomposition and existing builders over broad derived-table/scalar-subquery API growth in S03. | Yes | The shipped read-side boundary stayed honest because of this restraint. |
| D062 | Use the live Mesher HTTP/API harness for S03 composed-read verification. | Yes | Necessary once the copied probe stopped being an honest caller-contract signal. |
| D063 | Treat `get_event_alert_rules(...)` as the fireable-rule boundary and flush accepted event writes explicitly. | Yes | This kept live alert/read proofs deterministic and still matches the assembled ownership boundary. |
| D064 | Keep unstable read families as explicit named raw keep-sites enforced by `scripts/verify-m033-s03.sh`. | Yes | Still the honest boundary; no later slice disproved the need for that keep-list. |
| D065 | Keep `Migration.create_index(...)` narrowly neutral and route PG-only index features through explicit `Pg.*` helpers. | Yes | This matches the shipped migration surface and preserves the future SQLite seam. |
| D066 | Own runtime partition lifecycle in `mesher/storage/schema.mpl` over explicit `Pg.*` helpers. | Yes | It reduced schema/query boundary confusion and stayed stable through S04/S05. |
| D067 | Treat `compiler/meshc/tests/e2e_m033_s04.rs` plus `scripts/verify-m033-s04.sh` as the authoritative schema/partition acceptance surface. | Yes | This is still the canonical operational/schema gate for the shipped behavior. |
| D068 | Close S05 with a docs rewrite plus serial verifier wrapper instead of adding another runtime harness. | Yes | The final milestone replay and public contract are cleaner because of this choice. |
| D069 | Keep `get_event_alert_rules(...)` and `get_threshold_rules(...)` on the S03 honest raw-read ledger rather than under S02 ownership. | Yes | Required to keep the docs, verifiers, and raw-boundary ledger aligned. |

Only D056/D057 should be revisited next milestone, and only if parser work to expose `Expr.alias(...)` is explicitly in scope.

## Deviations

The milestone stayed within the roadmap, but execution forced three important truth-preserving pivots. S03 moved composed-read verification from a copied storage probe to a live Mesher HTTP/API harness once the probe stopped being an honest caller-contract signal. S04 had to finish and harden the dedicated schema-helper acceptance surfaces while fixing the `Pg.create_range_partitioned_table(...)` constraint gap exposed by the first live catalog run. S05 repaired shared verifier `fn_block(...)` parsing and stale ownership assertions so the new milestone replay matched the real assembled boundary instead of a stale theory of ownership.

## Follow-ups

Future work should repair the `Expr.alias(...)` parser collision, the broader boxed-int success-payload lowering brittleness, and exact terminal-page `has_more` pagination semantics on the issue list path. A later milestone can extend the same honest seam for SQLite-specific extras and then validate R040 with real runtime proof instead of design-only evidence.
