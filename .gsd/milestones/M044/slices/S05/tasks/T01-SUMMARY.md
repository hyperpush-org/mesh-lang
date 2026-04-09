---
id: T01
parent: S05
milestone: M044
provides: []
requires: []
affects: []
key_files: ["cluster-proof/config.mpl", "cluster-proof/main.mpl", "cluster-proof/docker-entrypoint.sh", "cluster-proof/fly.toml", "cluster-proof/tests/config.test.mpl", "compiler/meshc/tests/e2e_m044_s03.rs", "compiler/meshc/tests/e2e_m044_s04.rs", "compiler/meshc/tests/e2e_m044_s05.rs", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M044/slices/S05/tasks/T01-SUMMARY.md"]
key_decisions: ["Retired cluster-proof-specific bootstrap envs in favor of `MESH_CLUSTER_COOKIE` and `MESH_NODE_NAME`, with Fly-derived identity as the only non-explicit fallback.", "Removed proof-specific durability env usage from cluster-proof startup so `CLUSTER_PROOF_WORK_DELAY_MS` is the only remaining proof-only bootstrap knob.", "Added a dedicated `e2e_m044_s05` live public-contract rail instead of relying on source-grep-only proof for startup drift."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the Mesh config rail, compiled the edited M044 S04 test target, and passed the new live public-contract S05 rail. The required S03 operator rail failed once before the helper fix with a continuity submit that returned an unexpected owner/replica pair and a rejected remote spawn; the helper was patched afterward, but the context-budget warning stopped the fresh replay."
completed_at: 2026-03-30T05:58:49.433Z
blocker_discovered: false
---

# T01: Retargeted cluster-proof bootstrap to the public MESH_* contract and added live public-contract startup coverage; the S03 operator rail still needs one fresh rerun.

> Retargeted cluster-proof bootstrap to the public MESH_* contract and added live public-contract startup coverage; the S03 operator rail still needs one fresh rerun.

## What Happened
---
id: T01
parent: S05
milestone: M044
key_files:
  - cluster-proof/config.mpl
  - cluster-proof/main.mpl
  - cluster-proof/docker-entrypoint.sh
  - cluster-proof/fly.toml
  - cluster-proof/tests/config.test.mpl
  - compiler/meshc/tests/e2e_m044_s03.rs
  - compiler/meshc/tests/e2e_m044_s04.rs
  - compiler/meshc/tests/e2e_m044_s05.rs
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M044/slices/S05/tasks/T01-SUMMARY.md
key_decisions:
  - Retired cluster-proof-specific bootstrap envs in favor of `MESH_CLUSTER_COOKIE` and `MESH_NODE_NAME`, with Fly-derived identity as the only non-explicit fallback.
  - Removed proof-specific durability env usage from cluster-proof startup so `CLUSTER_PROOF_WORK_DELAY_MS` is the only remaining proof-only bootstrap knob.
  - Added a dedicated `e2e_m044_s05` live public-contract rail instead of relying on source-grep-only proof for startup drift.
duration: ""
verification_result: mixed
completed_at: 2026-03-30T05:58:49.435Z
blocker_discovered: false
---

# T01: Retargeted cluster-proof bootstrap to the public MESH_* contract and added live public-contract startup coverage; the S03 operator rail still needs one fresh rerun.

**Retargeted cluster-proof bootstrap to the public MESH_* contract and added live public-contract startup coverage; the S03 operator rail still needs one fresh rerun.**

## What Happened

Retargeted cluster-proof bootstrap/config to the public MESH_* clustered-app contract. `cluster-proof/config.mpl` now trims and validates `MESH_CLUSTER_COOKIE`, `MESH_NODE_NAME`, `MESH_DISCOVERY_SEED`, and `MESH_CONTINUITY_*`, rejects malformed explicit node names and mismatched node-name ports, and still derives identity from Fly env when `MESH_NODE_NAME` is absent. `cluster-proof/main.mpl` no longer reads the old proof-app cookie name, and `cluster-proof/docker-entrypoint.sh` now validates the same public contract before the binary starts while synthesizing only `MESH_NODE_NAME` for same-image local startup. I removed `CLUSTER_PROOF_DURABILITY` from `cluster-proof/fly.toml`, rewired the direct-process M044 S03/S04 Rust harnesses to use `MESH_CLUSTER_COOKIE` + `MESH_NODE_NAME`, and added `compiler/meshc/tests/e2e_m044_s05.rs` to prove explicit public startup, Fly fallback, and fail-closed rejection of legacy `CLUSTER_PROOF_*` bootstrap env names and malformed public inputs. During verification, the existing S03 operator continuity rail exposed that its `submit_request_for_owner(...)` helper was trusting a local placement guess too early, so I changed it to prefer actual successful submit responses; the rerun still needs a fresh context.

## Verification

Passed the Mesh config rail, compiled the edited M044 S04 test target, and passed the new live public-contract S05 rail. The required S03 operator rail failed once before the helper fix with a continuity submit that returned an unexpected owner/replica pair and a rejected remote spawn; the helper was patched afterward, but the context-budget warning stopped the fresh replay.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- test cluster-proof/tests/config.test.mpl` | 0 | ✅ pass | 4120ms |
| 2 | `cargo test -p meshc --test e2e_m044_s04 --no-run` | 0 | ✅ pass | 10640ms |
| 3 | `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture` | 101 | ❌ fail | 12510ms |
| 4 | `cargo test -p meshc --test e2e_m044_s05 m044_s05_public_contract_ -- --nocapture` | 0 | ✅ pass | 19020ms |


## Deviations

Added a compile-only `cargo test -p meshc --test e2e_m044_s04 --no-run` smoke check to cover the edited S04 file without spending extra live-runtime budget. Also adjusted `compiler/meshc/tests/e2e_m044_s03.rs::submit_request_for_owner(...)` to prefer actual successful submit responses over its local placement guess after the first S03 operator replay exposed that mismatch.

## Known Issues

`cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture` needs one fresh rerun after the helper fix in `compiler/meshc/tests/e2e_m044_s03.rs`. The last recorded run failed before that patch landed, and the context-budget warning arrived before a fresh replay.

## Files Created/Modified

- `cluster-proof/config.mpl`
- `cluster-proof/main.mpl`
- `cluster-proof/docker-entrypoint.sh`
- `cluster-proof/fly.toml`
- `cluster-proof/tests/config.test.mpl`
- `compiler/meshc/tests/e2e_m044_s03.rs`
- `compiler/meshc/tests/e2e_m044_s04.rs`
- `compiler/meshc/tests/e2e_m044_s05.rs`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M044/slices/S05/tasks/T01-SUMMARY.md`


## Deviations
Added a compile-only `cargo test -p meshc --test e2e_m044_s04 --no-run` smoke check to cover the edited S04 file without spending extra live-runtime budget. Also adjusted `compiler/meshc/tests/e2e_m044_s03.rs::submit_request_for_owner(...)` to prefer actual successful submit responses over its local placement guess after the first S03 operator replay exposed that mismatch.

## Known Issues
`cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture` needs one fresh rerun after the helper fix in `compiler/meshc/tests/e2e_m044_s03.rs`. The last recorded run failed before that patch landed, and the context-budget warning arrived before a fresh replay.
