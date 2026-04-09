# M033 Discussion Log

## Exchange — 2026-03-25T02:51:13.755Z

### Done Bar

For M033 closeout, which outcome matters most if there’s a tradeoff between broader API reach and shipping sooner?

- **Retire major families (Recommended)** — Focus on the big recurring Mesher raw-SQL/raw-DDL families and keep a short justified raw keep-list.
- **Push near-total coverage** — Keep expanding until very few raw sites remain, even if the milestone gets bigger.
- **Only sharp wins** — Ship a narrower wave of obvious helpers and leave more boundaries in place for later.

**Selected:** Push near-total coverage

### Proof

Which live proof surface should be treated as non-negotiable for calling M033 done?

- **Mesher flows + real PG (Recommended)** — Require live Postgres-backed Mesher ingestion/search/alert or schema flows, not just lower-level tests.
- **Runtime tests first** — Targeted compiler/runtime tests are the main bar; Mesher rewrites are secondary evidence.
- **Partition path only** — Only the real partition create/list/drop path must be live; other families can close with narrower proof.

**Selected:** Mesher flows + real PG (Recommended)

### SQLite seam

What should the deferred SQLite story look like in this milestone’s deliverables?

- **Credible seam only (Recommended)** — Shape the neutral API and explicit-vendor extension points, but no live SQLite implementation or proof.
- **Docs-level design** — Also write explicit follow-on design guidance for how SQLite extras would plug in later.
- **Test seam now** — Add some seam-level tests or fixtures now even though live SQLite support stays deferred.

**Selected:** Credible seam only (Recommended)

---
## Exchange — 2026-03-25T02:52:51.205Z

### Raw Tail

Given you want broader coverage, where should I draw the line on the final raw SQL/raw DDL tail?

- **Only dishonest leftovers (Recommended)** — Retire almost every recurring Mesher raw boundary; only leave raw SQL/DDLs where a first-class helper would be dishonest, especially truly dynamic catalog work.
- **Chase near-zero raw** — Keep driving even specialized helpers if that gets the raw count close to zero in Mesher.
- **Allow broader keep-list** — Keep a somewhat larger documented tail if the API would otherwise get awkward.

**Selected:** Only dishonest leftovers (Recommended)

### Docs Shape

What kind of docs outcome do you want this milestone to leave behind?

- **Reference + one walkthrough (Recommended)** — Public docs should explain the new neutral DSL plus PG extras, with one real Mesher-backed walkthrough and no broad marketing sweep.
- **Broader docs wave** — Do a fuller tutorial-style docs wave with multiple end-to-end examples.
- **Minimal docs** — Keep docs minimal and focus almost entirely on code and proof.

**Selected:** None of the above
**Notes:** option 1 in the mesh doc site

---
