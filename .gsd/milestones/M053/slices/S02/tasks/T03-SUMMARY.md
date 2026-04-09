---
id: T03
parent: S02
milestone: M053
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/e2e_m053_s02.rs", ".gsd/milestones/M053/slices/S02/tasks/T03-SUMMARY.md"]
key_decisions: ["Kept the pre-failover clustered read on `GET /todos` from the primary ingress only, then moved post-failover read proof to `GET /todos/:id` on the promoted standby to avoid the known per-runtime route-key collision across different ingress nodes.", "Widened the runtime-owned startup pending window to `MESH_STARTUP_WORK_DELAY_MS=8000` inside the failover rail so the starter can seed real HTTP state before killing the owner without adding any app-owned delay code."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran `cargo test -p meshc --test e2e_m053_s02 --no-run` and fixed the mechanical Rust issues until the updated failover rail compiled cleanly. Did not run the DATABASE_URL-backed failover replay or the retained wrapper in this unit."
completed_at: 2026-04-05T19:50:50.527Z
blocker_discovered: false
---

# T03: Added the staged Postgres failover e2e rail in compile-green form and left the retained verifier wrapper as the next explicit step.

> Added the staged Postgres failover e2e rail in compile-green form and left the retained verifier wrapper as the next explicit step.

## What Happened
---
id: T03
parent: S02
milestone: M053
key_files:
  - compiler/meshc/tests/e2e_m053_s02.rs
  - .gsd/milestones/M053/slices/S02/tasks/T03-SUMMARY.md
key_decisions:
  - Kept the pre-failover clustered read on `GET /todos` from the primary ingress only, then moved post-failover read proof to `GET /todos/:id` on the promoted standby to avoid the known per-runtime route-key collision across different ingress nodes.
  - Widened the runtime-owned startup pending window to `MESH_STARTUP_WORK_DELAY_MS=8000` inside the failover rail so the starter can seed real HTTP state before killing the owner without adding any app-owned delay code.
duration: ""
verification_result: passed
completed_at: 2026-04-05T19:50:50.532Z
blocker_discovered: false
---

# T03: Added the staged Postgres failover e2e rail in compile-green form and left the retained verifier wrapper as the next explicit step.

**Added the staged Postgres failover e2e rail in compile-green form and left the retained verifier wrapper as the next explicit step.**

## What Happened

Re-read the T03 contract and confirmed the local mismatch T02 had already surfaced: `compiler/meshc/tests/e2e_m053_s02.rs` still only contained the helper and bounded-source rails, so there was no authoritative destructive failover surface for an S02 wrapper to publish. I fixed that first by extending `e2e_m053_s02.rs` with local status/continuity/diagnostic matcher helpers and a new staged Postgres failover rail that stages the generated starter, applies deploy SQL, boots primary and standby from one staged bundle and one shared Postgres database, seeds state through real starter HTTP routes, captures the clustered `GET /todos` continuity key on the primary, kills the startup owner during the runtime-owned pending window, and records post-failover recovery/rejoin artifacts plus per-node logs. I stopped at the compile-green checkpoint when the context-budget warning fired, so the retained verifier wrapper itself (`scripts/verify-m053-s02.sh`) still needs to be written in the next unit.

## Verification

Ran `cargo test -p meshc --test e2e_m053_s02 --no-run` and fixed the mechanical Rust issues until the updated failover rail compiled cleanly. Did not run the DATABASE_URL-backed failover replay or the retained wrapper in this unit.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m053_s02 --no-run` | 0 | ✅ pass | 17920ms |


## Deviations

Stopped before the verifier-script work because the context-budget warning landed right after the compile-green checkpoint. That leaves the task partially implemented rather than guessing at the retained-wrapper behavior without a fresh context.

## Known Issues

`scripts/verify-m053-s02.sh` does not exist yet; the new staged Postgres failover rail has not been executed against a real `DATABASE_URL` yet; `.tmp/m053-s02/verify/` has not been created or populated in this unit.

## Files Created/Modified

- `compiler/meshc/tests/e2e_m053_s02.rs`
- `.gsd/milestones/M053/slices/S02/tasks/T03-SUMMARY.md`


## Deviations
Stopped before the verifier-script work because the context-budget warning landed right after the compile-green checkpoint. That leaves the task partially implemented rather than guessing at the retained-wrapper behavior without a fresh context.

## Known Issues
`scripts/verify-m053-s02.sh` does not exist yet; the new staged Postgres failover rail has not been executed against a real `DATABASE_URL` yet; `.tmp/m053-s02/verify/` has not been created or populated in this unit.
