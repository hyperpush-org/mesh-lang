---
id: T03
parent: S01
milestone: M033
provides: []
requires: []
affects: []
key_files: ["mesher/services/rate_limiter.mpl", "mesher/ingestion/pipeline.mpl", "mesher/services/event_processor.mpl", "compiler/meshc/tests/e2e_m033_s01.rs", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Keep Mesher’s existing 60-second / 1000-event limiter defaults, but read optional `MESHER_RATE_LIMIT_WINDOW_SECONDS` and `MESHER_RATE_LIMIT_MAX_EVENTS` env vars and always spawn the reset ticker so live threshold proofs and operators can exercise the real fixed-window limiter honestly.", "When a Mesh service handler updates state and returns a reply, avoid branching directly between different `(state, reply)` tuples; compute branch-local values first and return one final tuple, which stabilized both `RateLimiter.CheckLimit` and `EventProcessor.ProcessEvent` on the live ingest path."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified with `cargo test -p meshc --test e2e_m033_s01 mesher_ingest_first_event -- --nocapture`, which now passes and proves first-event acceptance plus truthful threshold-based 429 behavior on the live Mesher path. I also manually replayed seeded-key ingest against a supervised Mesher process during debugging to confirm the repaired path returned `202 Accepted` and then `429 Too Many Requests` at the configured limit. As a broader slice-level smoke check, I ran `cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture`; it now gets past the former clean-start ingest blocker and fails later in `assign_issue`, which confirms T03 unblocked the ingress path while surfacing the next defect."
completed_at: 2026-03-25T07:41:19.042Z
blocker_discovered: false
---

# T03: Fixed clean-start Mesher ingest by stabilizing rate-limit and processor service returns and adding a live first-event rate-limit proof

> Fixed clean-start Mesher ingest by stabilizing rate-limit and processor service returns and adding a live first-event rate-limit proof

## What Happened
---
id: T03
parent: S01
milestone: M033
key_files:
  - mesher/services/rate_limiter.mpl
  - mesher/ingestion/pipeline.mpl
  - mesher/services/event_processor.mpl
  - compiler/meshc/tests/e2e_m033_s01.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Keep Mesher’s existing 60-second / 1000-event limiter defaults, but read optional `MESHER_RATE_LIMIT_WINDOW_SECONDS` and `MESHER_RATE_LIMIT_MAX_EVENTS` env vars and always spawn the reset ticker so live threshold proofs and operators can exercise the real fixed-window limiter honestly.
  - When a Mesh service handler updates state and returns a reply, avoid branching directly between different `(state, reply)` tuples; compute branch-local values first and return one final tuple, which stabilized both `RateLimiter.CheckLimit` and `EventProcessor.ProcessEvent` on the live ingest path.
duration: ""
verification_result: mixed
completed_at: 2026-03-25T07:41:19.053Z
blocker_discovered: false
---

# T03: Fixed clean-start Mesher ingest by stabilizing rate-limit and processor service returns and adding a live first-event rate-limit proof

**Fixed clean-start Mesher ingest by stabilizing rate-limit and processor service returns and adding a live first-event rate-limit proof**

## What Happened

I started by reproducing the task’s live Postgres-backed failure on the seeded default API key. Local reality had drifted slightly from the prior summary: the first `/api/v1/events` request was not returning a clean 429 in the harness anymore, it was crashing Mesher and yielding an empty HTTP reply. I confirmed that unauthenticated and invalid-key requests still returned 401, so the crash sat after auth on the valid ingest path.

I isolated the first root cause with standalone Mesh probes and live Mesher runs: `RateLimiter.CheckLimit` misbehaved when the service handler branched directly between different `(state, reply)` tuples. On a fresh limiter it returned `false` on the first call and could crash afterward, even though the same `Map.get` / `Map.put` logic worked outside the service. I rewrote `mesher/services/rate_limiter.mpl` to compute `allowed` and `next_limits` first, then return one final tuple. I also restored the intended fixed-window behavior by adding `start_rate_limiter(...)`, which spawns the reset ticker, and I wired `mesher/ingestion/pipeline.mpl` to read optional `MESHER_RATE_LIMIT_WINDOW_SECONDS` / `MESHER_RATE_LIMIT_MAX_EVENTS` env vars while preserving the existing `60s / 1000` defaults.

Once the limiter stopped corrupting the first call, the same live repro still crashed when the route handed work to `EventProcessor.process_event(...)`. I traced that boundary with a temporary route log, then rewrote `mesher/services/event_processor.mpl` to use the same single-return service pattern: route the event through pure helpers to obtain one `String ! String` result, compute the next `processed_count`, and return one final `(new_state, result)` tuple. That removed the processor-side crash without changing the ingest semantics.

With the live path stable, I extended `compiler/meshc/tests/e2e_m033_s01.rs` so the harness can inject a small real rate-limit threshold through Mesher env vars and added the focused named proof `e2e_m033_mesher_ingest_first_event`. The test now proves a freshly started Mesher instance accepts the first seeded-key event, accepts a second event while still under the configured threshold, returns HTTP 429 only on the third event once the threshold is exceeded, and leaves the DB-side `issues.event_count` unchanged on the rejected request. I also widened the existing listener-log assertion helper to accept the runtime’s `[::]:port` HTTP listener banner, which was already the real output on this machine.

After the targeted gate passed, I reran the broader `mesher_mutations` slice proof as a partial slice-level smoke check. It now advances past the repaired ingest step and fails later on `/issues/:id/assign` with a non-exhaustive match inside `assign_issue`, which is a newly exposed downstream bug rather than a plan-invalidating T03 blocker.

## Verification

Verified with `cargo test -p meshc --test e2e_m033_s01 mesher_ingest_first_event -- --nocapture`, which now passes and proves first-event acceptance plus truthful threshold-based 429 behavior on the live Mesher path. I also manually replayed seeded-key ingest against a supervised Mesher process during debugging to confirm the repaired path returned `202 Accepted` and then `429 Too Many Requests` at the configured limit. As a broader slice-level smoke check, I ran `cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture`; it now gets past the former clean-start ingest blocker and fails later in `assign_issue`, which confirms T03 unblocked the ingress path while surfacing the next defect.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m033_s01 mesher_ingest_first_event -- --nocapture` | 0 | ✅ pass | 33200ms |
| 2 | `cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture` | 101 | ❌ fail | 28500ms |


## Deviations

I added optional Mesher env-based rate-limit configuration and a rate-limiter startup helper so the live harness could prove the real threshold at a small limit instead of sending 1000+ events. I also had to repair `mesher/services/event_processor.mpl`, which was not listed in the original task plan, because once the limiter was fixed the next crash moved to the processor service-return path and still blocked truthful first-event acceptance.

## Known Issues

`cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture` now fails later on `/api/v1/issues/:id/assign` with `Mesh panic at <unknown>:0: non-exhaustive match in switch` inside `assign_issue`. The clean-start ingest blocker is fixed, but the broader mutation proof still has this downstream route/storage bug to resolve in subsequent work.

## Files Created/Modified

- `mesher/services/rate_limiter.mpl`
- `mesher/ingestion/pipeline.mpl`
- `mesher/services/event_processor.mpl`
- `compiler/meshc/tests/e2e_m033_s01.rs`
- `.gsd/KNOWLEDGE.md`


## Deviations
I added optional Mesher env-based rate-limit configuration and a rate-limiter startup helper so the live harness could prove the real threshold at a small limit instead of sending 1000+ events. I also had to repair `mesher/services/event_processor.mpl`, which was not listed in the original task plan, because once the limiter was fixed the next crash moved to the processor service-return path and still blocked truthful first-event acceptance.

## Known Issues
`cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture` now fails later on `/api/v1/issues/:id/assign` with `Mesh panic at <unknown>:0: non-exhaustive match in switch` inside `assign_issue`. The clean-start ingest blocker is fixed, but the broader mutation proof still has this downstream route/storage bug to resolve in subsequent work.
