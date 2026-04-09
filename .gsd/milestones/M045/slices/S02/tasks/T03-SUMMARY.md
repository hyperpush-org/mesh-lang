---
id: T03
parent: S02
milestone: M045
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/e2e_m045_s02.rs", "scripts/verify-m045-s02.sh", "compiler/meshc/tests/e2e_m044_s01.rs", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Treat ingress-side `meshc cluster status --json` plus dual-node `meshc cluster continuity --json` as the authoritative runtime-owned proof surface for the scaffold rail; keep standby-side pre-submit status probing out of the fresh two-node window because it destabilizes the first remote-owned submit on this host.", "Flatten the S02 verifier's prerequisite replay to direct commands instead of nesting the older M045/S01 and M044/S02 wrapper scripts, because the transitive wrapper stack can hang after `cluster-proof/tests` already printed a green summary on this host."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the direct S02 rail with `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture`, reran the stale upstream manifest prerequisite with `cargo test -p meshc --test e2e_m044_s01 m044_s01_manifest_ -- --nocapture`, and finished with the authoritative slice gate `bash scripts/verify-m045-s02.sh`. The final verifier pass rechecked the M045/S01 bootstrap rail, `cluster-proof` build/test, the relevant M044/S02 declared-work rail, clustered init, the full M045/S02 e2e filter, and the retained bundle-shape check."
completed_at: 2026-03-30T21:04:51.264Z
blocker_discovered: false
---

# T03: Added the tiny two-node scaffold proof rail and fail-closed verifier, with retained evidence bundles and runtime-owned continuity checks.

> Added the tiny two-node scaffold proof rail and fail-closed verifier, with retained evidence bundles and runtime-owned continuity checks.

## What Happened
---
id: T03
parent: S02
milestone: M045
key_files:
  - compiler/meshc/tests/e2e_m045_s02.rs
  - scripts/verify-m045-s02.sh
  - compiler/meshc/tests/e2e_m044_s01.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Treat ingress-side `meshc cluster status --json` plus dual-node `meshc cluster continuity --json` as the authoritative runtime-owned proof surface for the scaffold rail; keep standby-side pre-submit status probing out of the fresh two-node window because it destabilizes the first remote-owned submit on this host.
  - Flatten the S02 verifier's prerequisite replay to direct commands instead of nesting the older M045/S01 and M044/S02 wrapper scripts, because the transitive wrapper stack can hang after `cluster-proof/tests` already printed a green summary on this host.
duration: ""
verification_result: mixed
completed_at: 2026-03-30T21:04:51.266Z
blocker_discovered: false
---

# T03: Added the tiny two-node scaffold proof rail and fail-closed verifier, with retained evidence bundles and runtime-owned continuity checks.

**Added the tiny two-node scaffold proof rail and fail-closed verifier, with retained evidence bundles and runtime-owned continuity checks.**

## What Happened

Finished the public S02 proof surface in two layers. In `compiler/meshc/tests/e2e_m045_s02.rs`, the two-node rail now proves the scaffold through runtime-owned inspection surfaces: it records ingress-side `meshc cluster status --json`, waits for `meshc cluster continuity --json` to reach `completed` on both ingress and owner nodes, keeps app `/work/:request_key` status checks as secondary agreement, and verifies duplicate-submit stability after completion. In `scripts/verify-m045-s02.sh`, I added the slice gate that replays the direct prerequisite rails, reruns the full `m045_s02_` filter, copies the fresh `.tmp/m045-s02/*` artifacts into `.tmp/m045-s02/verify/retained-m045-s02-artifacts/`, writes `latest-proof-bundle.txt`, and fail-closes on bundle-shape drift. While wiring that gate, I updated the stale declared-work fixture in `compiler/meshc/tests/e2e_m044_s01.rs` to the current two-argument public contract and recorded the non-obvious host/runtime gotchas in `.gsd/KNOWLEDGE.md`.

## Verification

Verified the direct S02 rail with `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture`, reran the stale upstream manifest prerequisite with `cargo test -p meshc --test e2e_m044_s01 m044_s01_manifest_ -- --nocapture`, and finished with the authoritative slice gate `bash scripts/verify-m045-s02.sh`. The final verifier pass rechecked the M045/S01 bootstrap rail, `cluster-proof` build/test, the relevant M044/S02 declared-work rail, clustered init, the full M045/S02 e2e filter, and the retained bundle-shape check.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture` | 101 | ❌ fail | 37500ms |
| 2 | `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture` | 101 | ❌ fail | 33000ms |
| 3 | `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture` | 0 | ✅ pass | 21200ms |
| 4 | `cargo test -p meshc --test e2e_m044_s01 m044_s01_manifest_ -- --nocapture` | 0 | ✅ pass | 12200ms |
| 5 | `bash scripts/verify-m045-s02.sh` | 1 | ❌ fail | 176900ms |
| 6 | `bash scripts/verify-m045-s02.sh` | 0 | ✅ pass | 92600ms |


## Deviations

Instead of shelling through `scripts/verify-m045-s01.sh` and `scripts/verify-m044-s02.sh` literally, the final S02 verifier replays their direct prerequisite commands. On this host the nested wrapper stack could hang after `cluster-proof/tests` had already printed a green summary, so flattening the replay preserved the intended prerequisite coverage without leaving the acceptance rail stuck.

## Known Issues

The older nested wrapper chain (`verify-m045-s01.sh` -> `verify-m044-s03.sh` -> `verify-m044-s02.sh`) can still hang at the transitive `cluster-proof-tests` phase on this host even when the leaf phase log is already green. `scripts/verify-m045-s02.sh` avoids that path by replaying the direct prerequisite commands.

## Files Created/Modified

- `compiler/meshc/tests/e2e_m045_s02.rs`
- `scripts/verify-m045-s02.sh`
- `compiler/meshc/tests/e2e_m044_s01.rs`
- `.gsd/KNOWLEDGE.md`


## Deviations
Instead of shelling through `scripts/verify-m045-s01.sh` and `scripts/verify-m044-s02.sh` literally, the final S02 verifier replays their direct prerequisite commands. On this host the nested wrapper stack could hang after `cluster-proof/tests` had already printed a green summary, so flattening the replay preserved the intended prerequisite coverage without leaving the acceptance rail stuck.

## Known Issues
The older nested wrapper chain (`verify-m045-s01.sh` -> `verify-m044-s03.sh` -> `verify-m044-s02.sh`) can still hang at the transitive `cluster-proof-tests` phase on this host even when the leaf phase log is already green. `scripts/verify-m045-s02.sh` avoids that path by replaying the direct prerequisite commands.
