---
id: T03
parent: S04
milestone: M039
provides: []
requires: []
affects: []
key_files: ["cluster-proof/README.md", "scripts/verify-m039-s04-fly.sh", "cluster-proof/fly.toml", ".gsd/DECISIONS.md", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Pin `MESH_DISCOVERY_SEED=mesh-cluster-proof.internal` in `cluster-proof/fly.toml` because Fly-injected identity env alone is not enough to satisfy the cluster runtime contract.", "Make the live Fly verifier fail closed on exact `CLUSTER_PROOF_FLY_APP` / `CLUSTER_PROOF_BASE_URL` pairing and read-only `fly status` / `fly config show` / `fly logs` plus `/membership` and `/work` probes."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the task’s named non-live gates (`bash -n scripts/verify-m039-s04-fly.sh`, `bash scripts/verify-m039-s04-fly.sh --help`, and the README deploy-command grep), plus local negative-path checks for missing `CLUSTER_PROOF_FLY_APP` and mismatched `CLUSTER_PROOF_BASE_URL`, and a local `fly config show --local -c cluster-proof/fly.toml` check confirming the pinned discovery seed and `auto_stop_machines=false` shape. No additional slice-level `## Verification` block exists in `S04-PLAN.md` beyond the task-local gates."
completed_at: 2026-03-28T14:09:18.954Z
blocker_discovered: false
---

# T03: Added the cluster-proof Fly runbook and fail-closed read-only verifier, and pinned the missing Fly discovery-seed config.

> Added the cluster-proof Fly runbook and fail-closed read-only verifier, and pinned the missing Fly discovery-seed config.

## What Happened
---
id: T03
parent: S04
milestone: M039
key_files:
  - cluster-proof/README.md
  - scripts/verify-m039-s04-fly.sh
  - cluster-proof/fly.toml
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Pin `MESH_DISCOVERY_SEED=mesh-cluster-proof.internal` in `cluster-proof/fly.toml` because Fly-injected identity env alone is not enough to satisfy the cluster runtime contract.
  - Make the live Fly verifier fail closed on exact `CLUSTER_PROOF_FLY_APP` / `CLUSTER_PROOF_BASE_URL` pairing and read-only `fly status` / `fly config show` / `fly logs` plus `/membership` and `/work` probes.
duration: ""
verification_result: passed
completed_at: 2026-03-28T14:09:18.955Z
blocker_discovered: false
---

# T03: Added the cluster-proof Fly runbook and fail-closed read-only verifier, and pinned the missing Fly discovery-seed config.

**Added the cluster-proof Fly runbook and fail-closed read-only verifier, and pinned the missing Fly discovery-seed config.**

## What Happened

Added `cluster-proof/README.md` as the canonical local/Fly operator runbook, implemented `scripts/verify-m039-s04-fly.sh` as a read-only Fly verifier with a phase ledger under `.tmp/m039-s04/fly/`, and pinned `MESH_DISCOVERY_SEED=mesh-cluster-proof.internal` in `cluster-proof/fly.toml` so the documented Fly deploy path matches the real runtime contract. The verifier now fail-closes on missing or inconsistent app/base-URL inputs before any Fly API call, checks running-machine truth with `fly status --json`, checks config truth with `fly config show`, validates live `/membership` and `/work` payloads, and requires recent Fly logs to show routed dispatch plus execution on the target node.

## Verification

Passed the task’s named non-live gates (`bash -n scripts/verify-m039-s04-fly.sh`, `bash scripts/verify-m039-s04-fly.sh --help`, and the README deploy-command grep), plus local negative-path checks for missing `CLUSTER_PROOF_FLY_APP` and mismatched `CLUSTER_PROOF_BASE_URL`, and a local `fly config show --local -c cluster-proof/fly.toml` check confirming the pinned discovery seed and `auto_stop_machines=false` shape. No additional slice-level `## Verification` block exists in `S04-PLAN.md` beyond the task-local gates.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -n scripts/verify-m039-s04-fly.sh` | 0 | ✅ pass | 22ms |
| 2 | `bash scripts/verify-m039-s04-fly.sh --help` | 0 | ✅ pass | 38ms |
| 3 | `rg -q "fly deploy \. --config cluster-proof/fly.toml --dockerfile cluster-proof/Dockerfile" cluster-proof/README.md` | 0 | ✅ pass | 35ms |
| 4 | `CLUSTER_PROOF_BASE_URL=https://mesh-cluster-proof.fly.dev bash scripts/verify-m039-s04-fly.sh (expected fail-closed path: missing CLUSTER_PROOF_FLY_APP)` | 1 | ✅ pass | 206ms |
| 5 | `CLUSTER_PROOF_FLY_APP=mesh-cluster-proof CLUSTER_PROOF_BASE_URL=https://wrong-app.fly.dev bash scripts/verify-m039-s04-fly.sh (expected fail-closed path: mismatched app/URL)` | 1 | ✅ pass | 198ms |
| 6 | `fly config show --local -c cluster-proof/fly.toml` | 0 | ✅ pass | 605ms |


## Deviations

Pinned `MESH_DISCOVERY_SEED=mesh-cluster-proof.internal` in `cluster-proof/fly.toml` even though the task plan listed that file as an input rather than a direct output. The runbook and verifier would have been dishonest without it, because the Fly identity env does not satisfy the discovery-seed half of the runtime contract by itself.

## Known Issues

The live Fly proof was not exercised against a real deployed app in this task context. `scripts/verify-m039-s04-fly.sh` now supports that path, but it still depends on an already-deployed `mesh-cluster-proof` app with `CLUSTER_PROOF_COOKIE` configured and at least two running machines.

## Files Created/Modified

- `cluster-proof/README.md`
- `scripts/verify-m039-s04-fly.sh`
- `cluster-proof/fly.toml`
- `.gsd/DECISIONS.md`
- `.gsd/KNOWLEDGE.md`


## Deviations
Pinned `MESH_DISCOVERY_SEED=mesh-cluster-proof.internal` in `cluster-proof/fly.toml` even though the task plan listed that file as an input rather than a direct output. The runbook and verifier would have been dishonest without it, because the Fly identity env does not satisfy the discovery-seed half of the runtime contract by itself.

## Known Issues
The live Fly proof was not exercised against a real deployed app in this task context. `scripts/verify-m039-s04-fly.sh` now supports that path, but it still depends on an already-deployed `mesh-cluster-proof` app with `CLUSTER_PROOF_COOKIE` configured and at least two running machines.
