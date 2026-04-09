---
id: T02
parent: S04
milestone: M046
provides: []
requires: []
affects: []
key_files: ["cluster-proof/tests/work.test.mpl", "cluster-proof/README.md", "cluster-proof/Dockerfile", "cluster-proof/fly.toml", "cluster-proof/docker-entrypoint.sh", ".gsd/DECISIONS.md", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M046/slices/S04/tasks/T02-SUMMARY.md"]
key_decisions: ["D248: package `cluster-proof` as a direct binary container with no entrypoint wrapper or Fly HTTP proxy contract.", "Keep package smoke tests source-owned by reading README/Docker/Fly files directly and failing closed on route/proxy drift."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the T02 package smoke rail with all five route-free contract assertions, then built the repo-root `mesh-cluster-proof:m046-s04-local` Docker image successfully from the simplified direct-binary Dockerfile. As an intermediate-task slice gate, reran the current downstream S04 checks: `e2e_m046_s03` still fails on the pre-existing S03 plan-text assertion, and the T03-owned `e2e_m046_s04` target plus `scripts/verify-m046-s04.sh` script are still absent."
completed_at: 2026-03-31T22:30:21.457Z
blocker_discovered: false
---

# T02: Locked `cluster-proof`’s route-free package contract in smoke tests, README, Dockerfile, and Fly config.

> Locked `cluster-proof`’s route-free package contract in smoke tests, README, Dockerfile, and Fly config.

## What Happened
---
id: T02
parent: S04
milestone: M046
key_files:
  - cluster-proof/tests/work.test.mpl
  - cluster-proof/README.md
  - cluster-proof/Dockerfile
  - cluster-proof/fly.toml
  - cluster-proof/docker-entrypoint.sh
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M046/slices/S04/tasks/T02-SUMMARY.md
key_decisions:
  - D248: package `cluster-proof` as a direct binary container with no entrypoint wrapper or Fly HTTP proxy contract.
  - Keep package smoke tests source-owned by reading README/Docker/Fly files directly and failing closed on route/proxy drift.
duration: ""
verification_result: mixed
completed_at: 2026-03-31T22:30:21.459Z
blocker_discovered: false
---

# T02: Locked `cluster-proof`’s route-free package contract in smoke tests, README, Dockerfile, and Fly config.

**Locked `cluster-proof`’s route-free package contract in smoke tests, README, Dockerfile, and Fly config.**

## What Happened

Rewrote `cluster-proof/tests/work.test.mpl` to mirror the tiny route-free smoke style while adding explicit README/Docker/Fly guards, including fail-closed checks for route strings, delay knobs, proxy config, and deleted obsolete files. Replaced `cluster-proof/README.md` with a route-free packaged contract centered on `clustered(work)`, `Node.start_from_env()`, and Mesh-owned `meshc cluster status|continuity|diagnostics` inspection. Simplified `cluster-proof/Dockerfile` into a direct-binary multi-stage image, removed `cluster-proof/docker-entrypoint.sh`, and simplified `cluster-proof/fly.toml` to a process-only build/env contract with no fake HTTP proxy story. Recorded the packaging choice as decision D248 and added a knowledge entry explaining the unrelated still-red S03 plan-text gate.

## Verification

Passed the T02 package smoke rail with all five route-free contract assertions, then built the repo-root `mesh-cluster-proof:m046-s04-local` Docker image successfully from the simplified direct-binary Dockerfile. As an intermediate-task slice gate, reran the current downstream S04 checks: `e2e_m046_s03` still fails on the pre-existing S03 plan-text assertion, and the T03-owned `e2e_m046_s04` target plus `scripts/verify-m046-s04.sh` script are still absent.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- test cluster-proof/tests` | 0 | ✅ pass | 13425ms |
| 2 | `docker build -f cluster-proof/Dockerfile -t mesh-cluster-proof:m046-s04-local .` | 0 | ✅ pass | 239755ms |
| 3 | `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_ -- --nocapture` | 101 | ❌ fail | 3841ms |
| 4 | `cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture` | 101 | ❌ fail | 1977ms |
| 5 | `bash scripts/verify-m046-s04.sh` | 127 | ❌ fail | 39ms |


## Deviations

`cluster-proof/tests/config.test.mpl` was already deleted during T01, so T02 enforced that absence inside `cluster-proof/tests/work.test.mpl` instead of deleting a live file again. No replan was required.

## Known Issues

`cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_ -- --nocapture` still fails on the pre-existing `.gsd/milestones/M046/slices/S03/S03-PLAN.md` override-line assertion unrelated to `cluster-proof/` packaging. `compiler/meshc/tests/e2e_m046_s04.rs` and `scripts/verify-m046-s04.sh` are still absent, so the slice-level T03 verifier checks remain red until T03 lands.

## Files Created/Modified

- `cluster-proof/tests/work.test.mpl`
- `cluster-proof/README.md`
- `cluster-proof/Dockerfile`
- `cluster-proof/fly.toml`
- `cluster-proof/docker-entrypoint.sh`
- `.gsd/DECISIONS.md`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M046/slices/S04/tasks/T02-SUMMARY.md`


## Deviations
`cluster-proof/tests/config.test.mpl` was already deleted during T01, so T02 enforced that absence inside `cluster-proof/tests/work.test.mpl` instead of deleting a live file again. No replan was required.

## Known Issues
`cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_ -- --nocapture` still fails on the pre-existing `.gsd/milestones/M046/slices/S03/S03-PLAN.md` override-line assertion unrelated to `cluster-proof/` packaging. `compiler/meshc/tests/e2e_m046_s04.rs` and `scripts/verify-m046-s04.sh` are still absent, so the slice-level T03 verifier checks remain red until T03 lands.
