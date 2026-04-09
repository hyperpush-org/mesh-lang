---
estimated_steps: 7
estimated_files: 4
skills_used: []
---

# T04: Re-prove the issue-upsert slice demo on the repaired ingest path

Why: The neutral write rewrites and the expression-based upsert path are already present in local reality, but S01 is not done until the live Mesher routes are re-proven after the ingest blocker is fixed.

Do:
- Re-run and tighten the existing live mutation and issue-upsert acceptance proofs against the repaired ingest path.
- Keep meshc build mesher, the e2e harness, and the verify script serialized so the shared mesher/mesher(.o) outputs do not create false linker failures.
- Prove repeated event ingest still creates or updates the same issue, increments event_count, advances last_seen, and reopens resolved issues through the structured upsert path.
- Keep the raw-write keep-list check honest: only the S02-owned PG helpers (create_alert_rule, fire_alert, insert_event) may remain raw after this slice closes.

Done when: the live Postgres-backed slice demo passes end-to-end and the remaining raw write keep-sites are limited to the explicit PG helpers deferred to S02.

## Inputs

- `.gsd/milestones/M033/slices/S01/tasks/T02-SUMMARY.md`
- `compiler/meshc/tests/e2e_m033_s01.rs`
- `mesher/storage/queries.mpl`
- `scripts/verify-m033-s01.sh`

## Expected Output

- `Passing live mutation and issue-upsert acceptance proofs on a repaired ingest path`
- `A serialized slice verification flow with an honest raw-write keep-list sweep`

## Verification

cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture
cargo test -p meshc --test e2e_m033_s01 mesher_issue_upsert -- --nocapture
bash scripts/verify-m033-s01.sh

## Observability Impact

Keeps final proof failures tied to named live-mesher tests and the keep-list sweep instead of being masked by the earlier ingest blocker or build/test artifact races.
