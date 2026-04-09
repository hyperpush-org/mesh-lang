---
id: T02
parent: S01
milestone: M033
provides: []
requires: []
affects: []
key_files: ["mesher/storage/queries.mpl", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Do not ship a Mesher test-only rate-limit bypass without understanding the upstream 429 cause; revert the experiment and record the blocker instead.", "Treat concurrent `meshc build mesher` and `e2e_m033_s01` runs as a harness race on `mesher/mesher(.o)` outputs and serialize those checks when gathering evidence."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "`cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture` was rerun on a clean tree and consistently failed before the mutation assertions because the first live `/api/v1/events` request returned HTTP 429 from Mesher’s rate limiter. `cargo run -q -p meshc -- fmt --check mesher` passed after normalizing `mesher/storage/queries.mpl`, and `cargo run -q -p meshc -- build mesher` passed on the same clean tree. I also manually reproduced the blocker outside the Rust harness by starting Mesher against a fresh Postgres container and observing `/api/v1/projects/default/settings` succeed while `/api/v1/events` returned 429 with the seeded default API key."
completed_at: 2026-03-25T06:57:50.683Z
blocker_discovered: true
---

# T02: Documented the live Mesher ingest blocker after verifying the S01 mutation rewrites already exist and normalizing queries.mpl formatting.

> Documented the live Mesher ingest blocker after verifying the S01 mutation rewrites already exist and normalizing queries.mpl formatting.

## What Happened
---
id: T02
parent: S01
milestone: M033
key_files:
  - mesher/storage/queries.mpl
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Do not ship a Mesher test-only rate-limit bypass without understanding the upstream 429 cause; revert the experiment and record the blocker instead.
  - Treat concurrent `meshc build mesher` and `e2e_m033_s01` runs as a harness race on `mesher/mesher(.o)` outputs and serialize those checks when gathering evidence.
duration: ""
verification_result: mixed
completed_at: 2026-03-25T06:57:50.686Z
blocker_discovered: true
---

# T02: Documented the live Mesher ingest blocker after verifying the S01 mutation rewrites already exist and normalizing queries.mpl formatting.

**Documented the live Mesher ingest blocker after verifying the S01 mutation rewrites already exist and normalizing queries.mpl formatting.**

## What Happened

I started by reading the task and slice artifacts plus the current Mesher sources before touching anything. That local verification showed the task-owned storage rewrites were already in the tree: `revoke_api_key`, `assign_issue`, `acknowledge_alert`, `resolve_fired_alert`, and `update_project_settings` already use the expression-aware `Repo.update_where_expr`/Mesh-side JSON path instead of the raw write APIs, and `compiler/meshc/tests/e2e_m033_s01.rs` already contains the live route proofs for assign/unassign, API-key revoke, alert acknowledge/resolve, and project settings row assertions.

I then moved to the real verification bar. My first focused `mesher_mutations` run failed with a false linker error because I had launched it in parallel with another `meshc build mesher`; both flows write the same `mesher/mesher(.o)` outputs. I verified that the direct `./target/debug/meshc build mesher` path itself was healthy, documented that harness race in `.gsd/KNOWLEDGE.md`, and reran the acceptance proof serially.

On clean serialized runs, the actual blocker reproduced consistently before any S01 mutation assertions executed: Mesher built and started normally, but the first `/api/v1/events` request returned HTTP 429 from the live rate limiter. I reproduced the same behavior outside the Rust harness by starting Mesher manually against a fresh Postgres container, confirming `/api/v1/projects/default/settings` returned 200, and then seeing `/api/v1/events` return 429 with the seeded default API key. That establishes the failure as an upstream Mesher ingest/rate-limit bug rather than drift in the neutral write paths this task was meant to prove.

I also applied the required formatter normalization to `mesher/storage/queries.mpl`, then reran the Mesher formatting and build gates successfully. I briefly tested a narrow rate-limit bypass to see whether the downstream write assertions were otherwise green, but I reverted that experiment because it was not a truthful fix for the live Mesher behavior and would have hidden the real blocker.

At task close, the durable state is: the task-owned write-core changes appear to have already landed, Mesher formats and builds cleanly, and T02 is blocked by an unrelated clean-start `/api/v1/events` 429 in the ingest path. Because the slice contract depends on live ingest to create issue/alert/project state before the mutation routes can be asserted, this blocker prevents truthful completion of T02 and should be addressed before continuing the remaining slice proof work.

## Verification

`cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture` was rerun on a clean tree and consistently failed before the mutation assertions because the first live `/api/v1/events` request returned HTTP 429 from Mesher’s rate limiter. `cargo run -q -p meshc -- fmt --check mesher` passed after normalizing `mesher/storage/queries.mpl`, and `cargo run -q -p meshc -- build mesher` passed on the same clean tree. I also manually reproduced the blocker outside the Rust harness by starting Mesher against a fresh Postgres container and observing `/api/v1/projects/default/settings` succeed while `/api/v1/events` returned 429 with the seeded default API key.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture` | 101 | ❌ fail | 37400ms |
| 2 | `cargo run -q -p meshc -- fmt --check mesher` | 0 | ✅ pass | 9200ms |
| 3 | `cargo run -q -p meshc -- build mesher` | 0 | ✅ pass | 20000ms |


## Deviations

The task-owned neutral write rewrites and live mutation assertions were already present in local reality (`mesher/storage/queries.mpl` and `compiler/meshc/tests/e2e_m033_s01.rs`), so execution became verification and blocker isolation rather than new feature implementation. I also reverted an unsuccessful temporary rate-limit bypass experiment instead of landing it.

## Known Issues

Clean-start Mesher still returns HTTP 429 on the first `/api/v1/events` request in `cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture`, which blocks the live mutation and issue-upsert proofs before any `assigned_to`, `acknowledged_at`, `resolved_at`, `retention_days`, `sample_rate`, or `revoked_at` assertions can execute. Manual standalone reproduction against a fresh Postgres container confirmed `/api/v1/projects/default/settings` returns 200 while `/api/v1/events` with the seeded default API key returns 429, so the remaining blocker is upstream in Mesher’s ingest/rate-limit path rather than in the S01 neutral write rewrites.

## Files Created/Modified

- `mesher/storage/queries.mpl`
- `.gsd/KNOWLEDGE.md`


## Deviations
The task-owned neutral write rewrites and live mutation assertions were already present in local reality (`mesher/storage/queries.mpl` and `compiler/meshc/tests/e2e_m033_s01.rs`), so execution became verification and blocker isolation rather than new feature implementation. I also reverted an unsuccessful temporary rate-limit bypass experiment instead of landing it.

## Known Issues
Clean-start Mesher still returns HTTP 429 on the first `/api/v1/events` request in `cargo test -p meshc --test e2e_m033_s01 mesher_mutations -- --nocapture`, which blocks the live mutation and issue-upsert proofs before any `assigned_to`, `acknowledged_at`, `resolved_at`, `retention_days`, `sample_rate`, or `revoked_at` assertions can execute. Manual standalone reproduction against a fresh Postgres container confirmed `/api/v1/projects/default/settings` returns 200 while `/api/v1/events` with the seeded default API key returns 429, so the remaining blocker is upstream in Mesher’s ingest/rate-limit path rather than in the S01 neutral write rewrites.
