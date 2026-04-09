---
id: T02
parent: S03
milestone: M043
provides: []
requires: []
affects: []
key_files: ["scripts/verify-m043-s03.sh", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M043/slices/S03/tasks/T02-SUMMARY.md"]
key_decisions: ["Select the retained same-image proof directory by `scenario-meta.json` and required-file shape instead of a loose name prefix, because the full `e2e_m043_s03` target also creates a malformed-response artifact directory.", "Keep packaged authority assertions anchored to runtime-owned JSON/log truth and only use copied Docker inspect files for metadata sanity checks, not for live authority derivation."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran `bash scripts/verify-m043-s03.sh`. The packaged verifier completed successfully, wrote `ok`/`complete` phase state under `.tmp/m043-s03/verify/`, replayed the nested S02 authority rail, ran all three `e2e_m043_s03` tests, copied the retained same-image artifact directory, and passed all copied JSON/log assertions."
completed_at: 2026-03-29T10:50:33.915Z
blocker_discovered: false
---

# T02: Added the fail-closed packaged same-image verifier that replays S02 and validates copied Docker failover artifacts from runtime-owned JSON and logs.

> Added the fail-closed packaged same-image verifier that replays S02 and validates copied Docker failover artifacts from runtime-owned JSON and logs.

## What Happened
---
id: T02
parent: S03
milestone: M043
key_files:
  - scripts/verify-m043-s03.sh
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M043/slices/S03/tasks/T02-SUMMARY.md
key_decisions:
  - Select the retained same-image proof directory by `scenario-meta.json` and required-file shape instead of a loose name prefix, because the full `e2e_m043_s03` target also creates a malformed-response artifact directory.
  - Keep packaged authority assertions anchored to runtime-owned JSON/log truth and only use copied Docker inspect files for metadata sanity checks, not for live authority derivation.
duration: ""
verification_result: passed
completed_at: 2026-03-29T10:50:33.916Z
blocker_discovered: false
---

# T02: Added the fail-closed packaged same-image verifier that replays S02 and validates copied Docker failover artifacts from runtime-owned JSON and logs.

**Added the fail-closed packaged same-image verifier that replays S02 and validates copied Docker failover artifacts from runtime-owned JSON and logs.**

## What Happened

Built `scripts/verify-m043-s03.sh` as the canonical packaged verifier for the same-image Docker failover rail. The wrapper replays the prior authority rails (`mesh-rt` continuity, `cluster-proof` tests, `cluster-proof` build, and `bash scripts/verify-m043-s02.sh`), then runs the full `cargo test -p meshc --test e2e_m043_s03 -- --nocapture` target so the negative guards stay inside the packaged contract. After that run, it snapshots `.tmp/m043-s03/`, selects the real same-image failover bundle by `scenario-meta.json` plus required-file shape, copies it under `.tmp/m043-s03/verify/05-same-image-artifacts/`, and proves the contract from retained JSON/log truth: mirrored pre-failover status, degraded standby truth, explicit promotion to epoch 1, recovery rollover to a new attempt, completion on the promoted standby, and fenced stale-primary rejoin. It also fail-closes on missing/empty artifacts, malformed metadata or retained JSON, missing `running N test` evidence, and stale-primary execution/completion drift in the copied logs. I recorded the dual-artifact behavior of the full `e2e_m043_s03` target in `.gsd/KNOWLEDGE.md` so later agents do not select the malformed-response bundle by prefix alone.

## Verification

Ran `bash scripts/verify-m043-s03.sh`. The packaged verifier completed successfully, wrote `ok`/`complete` phase state under `.tmp/m043-s03/verify/`, replayed the nested S02 authority rail, ran all three `e2e_m043_s03` tests, copied the retained same-image artifact directory, and passed all copied JSON/log assertions.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash scripts/verify-m043-s03.sh` | 0 | ✅ pass | 197300ms |


## Deviations

None. The only local adaptation from the written plan was selecting the real same-image bundle by `scenario-meta.json` shape because the full Rust target also emits a separate malformed-response artifact directory.

## Known Issues

None.

## Files Created/Modified

- `scripts/verify-m043-s03.sh`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M043/slices/S03/tasks/T02-SUMMARY.md`


## Deviations
None. The only local adaptation from the written plan was selecting the real same-image bundle by `scenario-meta.json` shape because the full Rust target also emits a separate malformed-response artifact directory.

## Known Issues
None.
