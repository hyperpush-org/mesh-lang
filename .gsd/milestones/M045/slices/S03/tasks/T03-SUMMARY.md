---
id: T03
parent: S03
milestone: M045
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/e2e_m045_s03.rs", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M045/slices/S03/tasks/T03-SUMMARY.md"]
key_decisions: ["Keep the S03 pre-kill search on primary ingress; standby-ingress probes can spuriously auto-promote on a live-primary write failure and are not a trustworthy failover proof surface.", "Keep deterministic request-key prefiltering as a heuristic only, then confirm runtime truth from `meshc cluster continuity --json` before any destructive step."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Reran `cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture` four times: once as the unmodified baseline, twice while testing the standby-ingress idea, and once after restoring primary ingress with the narrowed standby-only pending check. All four runs failed. The latest useful retained bundles are `.tmp/m045-s03/scaffold-failover-runtime-truth-1774909517065057000/` for primary-ingress search exhaustion and `.tmp/m045-s03/scaffold-failover-runtime-truth-1774909467086304000/` for standby-ingress false promotion."
completed_at: 2026-03-30T22:29:24.762Z
blocker_discovered: false
---

# T03: Narrowed the S03 scaffold failover harness and captured the remaining red runtime drift.

> Narrowed the S03 scaffold failover harness and captured the remaining red runtime drift.

## What Happened
---
id: T03
parent: S03
milestone: M045
key_files:
  - compiler/meshc/tests/e2e_m045_s03.rs
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M045/slices/S03/tasks/T03-SUMMARY.md
key_decisions:
  - Keep the S03 pre-kill search on primary ingress; standby-ingress probes can spuriously auto-promote on a live-primary write failure and are not a trustworthy failover proof surface.
  - Keep deterministic request-key prefiltering as a heuristic only, then confirm runtime truth from `meshc cluster continuity --json` before any destructive step.
duration: ""
verification_result: mixed
completed_at: 2026-03-30T22:29:24.764Z
blocker_discovered: false
---

# T03: Narrowed the S03 scaffold failover harness and captured the remaining red runtime drift.

**Narrowed the S03 scaffold failover harness and captured the remaining red runtime drift.**

## What Happened

Activated the `test` skill, read the S03 task plan and prior T01/T02 summaries, then reproduced the exact S03 failover rail. I narrowed `compiler/meshc/tests/e2e_m045_s03.rs` in two runtime-owned ways: added deterministic primary-owner request-key prefiltering and reduced the pre-kill proof attempt to the standby-side continuity snapshot instead of the slower dual-node pre-kill query sequence. I also tested, then rejected, a standby-ingress pre-kill search: on this runtime/host it can return `503 {"error":"attempt_id_mismatch"}` after `mesh node spawn failed ... write_error` / `write_error_after_reconnect` and auto-promote the standby while the primary is still alive. I restored primary ingress, recorded that drift in `.gsd/KNOWLEDGE.md`, and stopped to write a durable handoff when the context-budget warning landed. The harness is still red, and the top-level `scripts/verify-m045-s03.sh` verifier was not written in this unit.

## Verification

Reran `cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture` four times: once as the unmodified baseline, twice while testing the standby-ingress idea, and once after restoring primary ingress with the narrowed standby-only pending check. All four runs failed. The latest useful retained bundles are `.tmp/m045-s03/scaffold-failover-runtime-truth-1774909517065057000/` for primary-ingress search exhaustion and `.tmp/m045-s03/scaffold-failover-runtime-truth-1774909467086304000/` for standby-ingress false promotion.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture` | 101 | ❌ fail | 184400ms |
| 2 | `cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture` | 101 | ❌ fail | 97650ms |
| 3 | `cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture` | 101 | ❌ fail | 8990ms |
| 4 | `cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture` | 101 | ❌ fail | 84040ms |


## Deviations

Did not create `scripts/verify-m045-s03.sh` in this unit. The written task plan expected the assembled verifier, but the S03 failover harness remained red and the context-budget warning required a durable handoff instead of starting the wrapper script on top of an unverified rail.

## Known Issues

`cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture` is still red. With primary ingress, the current harness still cannot prove a stable pre-kill pending mirrored record before the primary completes the work. With standby ingress, the runtime can spuriously auto-promote/recover on a live-primary write failure (`attempt_id_mismatch` + `write_error` / `write_error_after_reconnect`), so that probe shape is not trustworthy. `scripts/verify-m045-s03.sh` does not exist yet.

## Files Created/Modified

- `compiler/meshc/tests/e2e_m045_s03.rs`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M045/slices/S03/tasks/T03-SUMMARY.md`


## Deviations
Did not create `scripts/verify-m045-s03.sh` in this unit. The written task plan expected the assembled verifier, but the S03 failover harness remained red and the context-budget warning required a durable handoff instead of starting the wrapper script on top of an unverified rail.

## Known Issues
`cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture` is still red. With primary ingress, the current harness still cannot prove a stable pre-kill pending mirrored record before the primary completes the work. With standby ingress, the runtime can spuriously auto-promote/recover on a live-primary write failure (`attempt_id_mismatch` + `write_error` / `write_error_after_reconnect`), so that probe shape is not trustworthy. `scripts/verify-m045-s03.sh` does not exist yet.
