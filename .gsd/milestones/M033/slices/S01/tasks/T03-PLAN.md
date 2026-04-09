---
estimated_steps: 6
estimated_files: 4
skills_used: []
---

# T03: Retire the clean-start ingest 429 blocker on the live Mesher path

Why: Every remaining S01 proof depends on fresh-instance event ingest working with the seeded default API key. Right now the first /api/v1/events request is rejected with HTTP 429 before the neutral write paths are exercised, so the slice cannot be truthfully closed.

Do:
- Reproduce the clean-start 429 in the live Postgres-backed harness and trace the auth -> sampling -> rate-limit path for the seeded default project/API key.
- Fix the state or routing bug that causes the first event to be treated as already over limit, without weakening genuine rate limiting for later bursts.
- Add a focused e2e proof that a freshly started Mesher instance accepts the first seeded-key event and only returns 429 when the configured threshold is actually exceeded.

Done when: the first /api/v1/events call on a clean Mesher boot returns 202 instead of 429, and the focused proof shows the limiter still behaves honestly after the fix.

## Inputs

- `.gsd/milestones/M033/slices/S01/tasks/T02-SUMMARY.md`
- `compiler/meshc/tests/e2e_m033_s01.rs`
- `mesher/services/rate_limiter.mpl`
- `mesher/ingestion/routes.mpl`
- `mesher/ingestion/pipeline.mpl`

## Expected Output

- `A code fix that lets fresh-instance /api/v1/events ingest succeed with the seeded default API key`
- `A focused named e2e proof for first-event acceptance and real rate-limit behavior`

## Verification

cargo test -p meshc --test e2e_m033_s01 mesher_ingest_first_event -- --nocapture

## Observability Impact

Makes the fresh-instance ingest failure surface as a targeted named e2e failure instead of a generic downstream 429 before mutation/upsert assertions.
