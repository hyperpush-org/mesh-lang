# M033: ORM Expressiveness & Schema Extras

## Vision
Expand Mesh’s data-layer surface into a broader honest expression DSL plus explicit PostgreSQL extras, retire the recurring Mesher raw SQL/raw DDL families those surfaces can truthfully cover, preserve a clean SQLite extension seam, and prove the result through live Postgres-backed Mesher flows and public Mesh docs.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | Neutral expression core on real write paths | high | — | ✅ | After this: live Postgres-backed Mesher write paths for issue upserts, alert state transitions, settings updates, and `NULL`/`now()`-driven mutations run through structured Mesh expressions instead of recurring raw SQL. |
| S02 | Explicit PG extras for JSONB, search, and crypto | high | S01 | ✅ | After this: Mesher event ingest, JSONB extraction, full-text search, and pgcrypto-backed auth flows work through explicit PostgreSQL helpers on the real runtime path. |
| S03 | Hard read-side coverage and honest raw-tail collapse | medium | S01, S02 | ✅ | After this: Mesher’s recurring scalar-subquery, derived-table, parameterized select, and expression-heavy read paths use the new builders wherever honest, and the remaining raw query keep-list is short and named. |
| S04 | Schema extras and live partition lifecycle proof | medium | S01, S02 | ✅ | After this: Mesher migrations and runtime retention/schema flows create, list, and drop partitions plus related PG schema extras through first-class helpers on a live Postgres database. |
| S05 | Public docs and integrated Mesher acceptance | low | S02, S03, S04 | ✅ | After this: the public Mesh database docs explain the shipped neutral DSL and PG extras through a Mesher-backed path, and the assembled Mesher data-layer behavior is re-proven end-to-end on live Postgres. |
