# M021: Query Builder (Phases 106-115) - SHIPPED 2026-02-25

**Vision:** Mesh is a programming language that combines Elixir/Ruby-style expressive syntax with static Hindley-Milner type inference and BEAM-style concurrency (actors, supervision trees, fault tolerance), compiled via LLVM to native single-binary executables.

## Success Criteria


## Slices

- [x] **S01: Advanced Where Operators And Raw Sql Fragments** `risk:medium` `depends:[]`
  > After this: Add NOT IN, BETWEEN, ILIKE, and OR operators to the Mesh query builder, completing WHERE-02 through WHERE-06 requirements.
- [x] **S02: Joins** `risk:medium` `depends:[S01]`
  > After this: Add table alias support to Query.
- [x] **S03: Aggregations** `risk:medium` `depends:[S02]`
  > After this: Add five aggregate select functions to the Query builder (select_count, select_sum, select_avg, select_min, select_max) with full compiler pipeline registration and tests.
- [x] **S04: Upserts Returning Subqueries** `risk:medium` `depends:[S03]`
  > After this: Add upsert (INSERT ON CONFLICT DO UPDATE), RETURNING for delete_where, and subquery WHERE support to the Mesh ORM with full compiler pipeline registration and tests.
- [x] **S05: Fix The Issues Encountered In 109** `risk:medium` `depends:[S04]`
  > After this: Fix the type checker arity bug (E0003) where `let x = Sqlite.
- [x] **S06: Mesher Rewrite Auth And Users** `risk:medium` `depends:[S05]`
  > After this: Rewrite the 5 user and session query functions in `mesher/storage/queries.
- [x] **S07: Mesher Rewrite Issues And Events** `risk:medium` `depends:[S06]`
  > After this: Rewrite 10 issue management queries from raw SQL (Repo.
- [x] **S08: Mesher Rewrite Search Dashboard And Alerts** `risk:medium` `depends:[S07]`
  > After this: Rewrite search, dashboard, detail, and team queries from Repo.
- [x] **S09: Mesher Rewrite Retention And Final Cleanup** `risk:medium` `depends:[S08]`
  > After this: Rewrite 4 retention/storage query functions in `mesher/storage/queries.
- [x] **S10: Compile Run And End To End Verification** `risk:medium` `depends:[S09]`
  > After this: Verify zero-error compilation of Mesher with the fully rewritten ORM query layer, confirm successful startup with PostgreSQL, run migrations, and confirm the MirType::Tuple SIGSEGV fix is active.
- [x] **S11: Tracking Corrections And Api Acceptance** `risk:medium` `depends:[S10]`
  > After this: Close the 13 requirement tracking gaps identified in the v11.
