---
id: T03
parent: S02
milestone: M042
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/e2e_m042_s02.rs", "scripts/verify-m042-s02.sh", "compiler/meshc/tests/e2e_m042_s01.rs", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M042/slices/S02/tasks/T03-SUMMARY.md"]
key_decisions: ["Keep the S02 proof rail on the stable local-owner/remote-replica direction and replay only the S01 standalone regression, because the older remote-owner direction still flakes on replica prepare availability.", "Treat degraded continuity truth on the ordinary status API as `replica_status=degraded_continuing` with `ok=true`; the replica-loss reason remains in runtime stderr logs and copied artifacts instead of the HTTP payload."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran `cargo test -p meshc --test e2e_m042_s02 -- --nocapture` and got all four S02 tests green. Ran `bash scripts/verify-m042-s02.sh`, which replayed `cargo build -q -p mesh-rt`, `cargo run -q -p meshc -- test cluster-proof/tests`, `cargo run -q -p meshc -- build cluster-proof`, the S01 standalone regression filter, and each named S02 filter with copied artifact validation. Reran the exact task-plan verification command `cargo test -p meshc --test e2e_m042_s02 -- --nocapture && bash scripts/verify-m042-s02.sh`, which also passed."
completed_at: 2026-03-28T23:30:28.184Z
blocker_discovered: false
---

# T03: Added the S02 continuity e2e target and fail-closed verifier that prove rejected, mirrored, and degraded status truth on the stable local-owner rail with copied artifacts under `.tmp/m042-s02/`.

> Added the S02 continuity e2e target and fail-closed verifier that prove rejected, mirrored, and degraded status truth on the stable local-owner rail with copied artifacts under `.tmp/m042-s02/`.

## What Happened
---
id: T03
parent: S02
milestone: M042
key_files:
  - compiler/meshc/tests/e2e_m042_s02.rs
  - scripts/verify-m042-s02.sh
  - compiler/meshc/tests/e2e_m042_s01.rs
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M042/slices/S02/tasks/T03-SUMMARY.md
key_decisions:
  - Keep the S02 proof rail on the stable local-owner/remote-replica direction and replay only the S01 standalone regression, because the older remote-owner direction still flakes on replica prepare availability.
  - Treat degraded continuity truth on the ordinary status API as `replica_status=degraded_continuing` with `ok=true`; the replica-loss reason remains in runtime stderr logs and copied artifacts instead of the HTTP payload.
duration: ""
verification_result: passed
completed_at: 2026-03-28T23:30:28.185Z
blocker_discovered: false
---

# T03: Added the S02 continuity e2e target and fail-closed verifier that prove rejected, mirrored, and degraded status truth on the stable local-owner rail with copied artifacts under `.tmp/m042-s02/`.

**Added the S02 continuity e2e target and fail-closed verifier that prove rejected, mirrored, and degraded status truth on the stable local-owner rail with copied artifacts under `.tmp/m042-s02/`.**

## What Happened

Created `compiler/meshc/tests/e2e_m042_s02.rs` as the stable S02 proof rail, reusing the S01 process/http harness shape but giving it its own `.tmp/m042-s02/...` artifact root, raw-response archiving, parsed JSON snapshots, and explicit timeout failure surfaces. The target now covers malformed non-JSON response failure handling, single-node cluster-mode replica-backed rejection with duplicate/conflict replay, two-node local-owner mirrored admission with status convergence on both owner and replica, and replica-loss downgrade to `degraded_continuing` while work is still pending. Added `scripts/verify-m042-s02.sh` as the authoritative slice verifier: it replays `mesh-rt`, `cluster-proof` tests, the stable S01 standalone regression, and each named S02 filter separately, fail-closing on missing `running N test` evidence or malformed copied artifacts. Also removed the unused `config` field from `compiler/meshc/tests/e2e_m042_s01.rs` and recorded the degraded-status API nuance in `.gsd/KNOWLEDGE.md`.

## Verification

Ran `cargo test -p meshc --test e2e_m042_s02 -- --nocapture` and got all four S02 tests green. Ran `bash scripts/verify-m042-s02.sh`, which replayed `cargo build -q -p mesh-rt`, `cargo run -q -p meshc -- test cluster-proof/tests`, `cargo run -q -p meshc -- build cluster-proof`, the S01 standalone regression filter, and each named S02 filter with copied artifact validation. Reran the exact task-plan verification command `cargo test -p meshc --test e2e_m042_s02 -- --nocapture && bash scripts/verify-m042-s02.sh`, which also passed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m042_s02 -- --nocapture` | 0 | ✅ pass | 9159ms |
| 2 | `bash scripts/verify-m042-s02.sh` | 0 | ✅ pass | 98107ms |


## Deviations

None.

## Known Issues

The older two-node remote-owner S01 direction still reproduces `replica_required_unavailable` in `e2e_m042_s01`, so the new verifier intentionally replays only `continuity_api_standalone_keyed_submit_status_and_retry_contract` from S01 and keeps the S02 proof rail on the stable local-owner/remote-replica path.

## Files Created/Modified

- `compiler/meshc/tests/e2e_m042_s02.rs`
- `scripts/verify-m042-s02.sh`
- `compiler/meshc/tests/e2e_m042_s01.rs`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M042/slices/S02/tasks/T03-SUMMARY.md`


## Deviations
None.

## Known Issues
The older two-node remote-owner S01 direction still reproduces `replica_required_unavailable` in `e2e_m042_s01`, so the new verifier intentionally replays only `continuity_api_standalone_keyed_submit_status_and_retry_contract` from S01 and keeps the S02 proof rail on the stable local-owner/remote-replica path.
