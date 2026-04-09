---
id: T01
parent: S04
milestone: M043
provides: []
requires: []
affects: []
key_files: ["scripts/verify-m043-s04-proof-surface.sh", "scripts/verify-m043-s04-fly.sh", ".gsd/milestones/M043/slices/S04/tasks/T01-SUMMARY.md"]
key_decisions: ["Parse `fly config show` as TOML in the Fly verifier so config drift checks do not depend on a JSON-only CLI shape.", "Treat `cluster_role`, `promotion_epoch`, and `replication_health` as required live payload fields on `/membership` and optional keyed status inspection."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the new scripts parse and the Fly help path is current with `bash -n scripts/verify-m043-s04-proof-surface.sh scripts/verify-m043-s04-fly.sh` and `bash scripts/verify-m043-s04-fly.sh --help`. Ran the slice checks: `bash scripts/verify-m043-s03.sh` passed, `npm --prefix website run build` passed, and `bash scripts/verify-m043-s04-proof-surface.sh` failed closed at `proof-page-contract` with retained artifacts pointing at the stale M042 docs, which is expected before T02. Also exercised malformed Fly inputs (missing env, base URL host mismatch, and whitespace-padded request key), all of which failed at `input-validation` and left `.tmp/m043-s04/fly/` status artifacts."
completed_at: 2026-03-29T11:58:40.093Z
blocker_discovered: false
---

# T01: Added M043 proof-surface and read-only Fly verifier rails with authority-field checks and retained failure artifacts.

> Added M043 proof-surface and read-only Fly verifier rails with authority-field checks and retained failure artifacts.

## What Happened
---
id: T01
parent: S04
milestone: M043
key_files:
  - scripts/verify-m043-s04-proof-surface.sh
  - scripts/verify-m043-s04-fly.sh
  - .gsd/milestones/M043/slices/S04/tasks/T01-SUMMARY.md
key_decisions:
  - Parse `fly config show` as TOML in the Fly verifier so config drift checks do not depend on a JSON-only CLI shape.
  - Treat `cluster_role`, `promotion_epoch`, and `replication_health` as required live payload fields on `/membership` and optional keyed status inspection.
duration: ""
verification_result: passed
completed_at: 2026-03-29T11:58:40.094Z
blocker_discovered: false
---

# T01: Added M043 proof-surface and read-only Fly verifier rails with authority-field checks and retained failure artifacts.

**Added M043 proof-surface and read-only Fly verifier rails with authority-field checks and retained failure artifacts.**

## What Happened

Forked the M042 verifier shape into new M043 proof-surface and Fly rails without touching the public prose first. The proof-surface verifier now writes retained phase/status/current/full-log artifacts under `.tmp/m043-s04/proof-surface/` and fails on stale M042 command names or missing M043 failover wording such as the explicit `/promote` boundary, runtime-owned authority fields, same-image local authority, and stale-primary fencing. The Fly verifier now writes retained artifacts under `.tmp/m043-s04/fly/`, keeps its live command set read-only, points operators at `bash scripts/verify-m043-s03.sh` as the destructive local authority, parses `fly config show` as TOML for drift checks, and requires `cluster_role`, `promotion_epoch`, and `replication_health` on live `/membership` plus optional `/work/:request_key` payloads. Negative-path checks proved malformed inputs fail at `input-validation` with retained artifacts. The proof-surface gate is expected to fail until T02 updates the docs and runbook from M042 wording to the new M043 contract.

## Verification

Verified the new scripts parse and the Fly help path is current with `bash -n scripts/verify-m043-s04-proof-surface.sh scripts/verify-m043-s04-fly.sh` and `bash scripts/verify-m043-s04-fly.sh --help`. Ran the slice checks: `bash scripts/verify-m043-s03.sh` passed, `npm --prefix website run build` passed, and `bash scripts/verify-m043-s04-proof-surface.sh` failed closed at `proof-page-contract` with retained artifacts pointing at the stale M042 docs, which is expected before T02. Also exercised malformed Fly inputs (missing env, base URL host mismatch, and whitespace-padded request key), all of which failed at `input-validation` and left `.tmp/m043-s04/fly/` status artifacts.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -n scripts/verify-m043-s04-proof-surface.sh scripts/verify-m043-s04-fly.sh` | 0 | ✅ pass | 22ms |
| 2 | `bash scripts/verify-m043-s04-fly.sh --help` | 0 | ✅ pass | 93ms |
| 3 | `bash scripts/verify-m043-s03.sh` | 0 | ✅ pass | 192400ms |
| 4 | `bash scripts/verify-m043-s04-proof-surface.sh` | 1 | ✅ pass (failed closed on stale M042 docs as expected for T01) | 419ms |
| 5 | `npm --prefix website run build` | 0 | ✅ pass | 21169ms |
| 6 | `bash scripts/verify-m043-s04-fly.sh` | 1 | ✅ pass (rejected missing live-mode env at input validation) | 248ms |
| 7 | `CLUSTER_PROOF_FLY_APP=mesh-cluster-proof CLUSTER_PROOF_BASE_URL=https://wrong.fly.dev bash scripts/verify-m043-s04-fly.sh` | 1 | ✅ pass (rejected host/app mismatch at input validation) | 177ms |
| 8 | `CLUSTER_PROOF_FLY_APP=mesh-cluster-proof CLUSTER_PROOF_BASE_URL=https://mesh-cluster-proof.fly.dev CLUSTER_PROOF_REQUEST_KEY=' bad-key' bash scripts/verify-m043-s04-fly.sh` | 1 | ✅ pass (rejected malformed request key at input validation) | 164ms |


## Deviations

None.

## Known Issues

Public docs and the runbook still describe M042 verifier names and older proof wording, so `bash scripts/verify-m043-s04-proof-surface.sh` currently fails at `proof-page-contract`. That is the intended T02 follow-up, not a plan-invalidating blocker.

## Files Created/Modified

- `scripts/verify-m043-s04-proof-surface.sh`
- `scripts/verify-m043-s04-fly.sh`
- `.gsd/milestones/M043/slices/S04/tasks/T01-SUMMARY.md`


## Deviations
None.

## Known Issues
Public docs and the runbook still describe M042 verifier names and older proof wording, so `bash scripts/verify-m043-s04-proof-surface.sh` currently fails at `proof-page-contract`. That is the intended T02 follow-up, not a plan-invalidating blocker.
