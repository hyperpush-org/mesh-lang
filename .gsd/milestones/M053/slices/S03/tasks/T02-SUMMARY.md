---
id: T02
parent: S03
milestone: M053
provides: []
requires: []
affects: []
key_files: ["scripts/verify-m053-s03.sh", "scripts/tests/verify-m053-s03-contract.test.mjs", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M053/slices/S03/tasks/T02-SUMMARY.md"]
key_decisions: ["D408: Derive hosted-verifier GitHub repo slugs from `git remote get-url origin` by default and keep an explicit override for fixtures/unusual remotes."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "`bash scripts/verify-m034-s02-workflows.sh` passed, and `node --test scripts/tests/verify-m053-s03-contract.test.mjs` passed with 14/14 tests green. The live hosted replay of `bash scripts/verify-m053-s03.sh` ran with `GH_TOKEN` parsed from `.env` and failed closed in `remote-evidence`, surfacing two real hosted gaps: the latest green `authoritative-verification.yml` run on `main` does not yet include the starter failover proof job, and the remote `v0.1.0` tag does not expose a peeled ref for release freshness checks."
completed_at: 2026-04-05T21:03:37.695Z
blocker_discovered: false
---

# T02: Added a freshness-aware hosted verifier that couples starter proof with deploy-services packages/public-surface truth.

> Added a freshness-aware hosted verifier that couples starter proof with deploy-services packages/public-surface truth.

## What Happened
---
id: T02
parent: S03
milestone: M053
key_files:
  - scripts/verify-m053-s03.sh
  - scripts/tests/verify-m053-s03-contract.test.mjs
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M053/slices/S03/tasks/T02-SUMMARY.md
key_decisions:
  - D408: Derive hosted-verifier GitHub repo slugs from `git remote get-url origin` by default and keep an explicit override for fixtures/unusual remotes.
duration: ""
verification_result: mixed
completed_at: 2026-04-05T21:03:37.700Z
blocker_discovered: false
---

# T02: Added a freshness-aware hosted verifier that couples starter proof with deploy-services packages/public-surface truth.

**Added a freshness-aware hosted verifier that couples starter proof with deploy-services packages/public-surface truth.**

## What Happened

Added `scripts/verify-m053-s03.sh` as the slice-owned hosted evidence verifier that bootstraps `.tmp/m053-s03/verify/`, derives candidate refs for `main` and the current binary tag, queries hosted runs with `git ls-remote` plus `gh run list/view`, and records fail-closed evidence for starter proof and deploy-services packages/public-surface truth. Extended `scripts/tests/verify-m053-s03-contract.test.mjs` with fixture-backed success and negative cases for missing GH auth, workflow-not-found responses, stale SHAs, tag-only deploy-services evidence, missing required jobs/steps, malformed `gh` JSON, and missing final artifacts. During live verification, adapted the verifier to resolve the GitHub repo slug from `origin` after the repository move to `hyperpush-org/hyperpush-mono`, then confirmed that the real hosted replay now targets the organization repo and correctly goes red on genuine remote drift.

## Verification

`bash scripts/verify-m034-s02-workflows.sh` passed, and `node --test scripts/tests/verify-m053-s03-contract.test.mjs` passed with 14/14 tests green. The live hosted replay of `bash scripts/verify-m053-s03.sh` ran with `GH_TOKEN` parsed from `.env` and failed closed in `remote-evidence`, surfacing two real hosted gaps: the latest green `authoritative-verification.yml` run on `main` does not yet include the starter failover proof job, and the remote `v0.1.0` tag does not expose a peeled ref for release freshness checks.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash scripts/verify-m034-s02-workflows.sh` | 0 | ✅ pass | 969ms |
| 2 | `node --test scripts/tests/verify-m053-s03-contract.test.mjs` | 0 | ✅ pass | 17862ms |
| 3 | `GH_TOKEN=<parsed from .env by python> bash scripts/verify-m053-s03.sh` | 1 | ❌ fail | 6019ms |


## Deviations

Adapted the verifier to derive the GitHub repo slug from `origin` because the repository moved to `hyperpush-org/hyperpush-mono` during execution. Loaded `GH_TOKEN` for the live replay by parsing only that key from `.env` instead of `source .env`, because unrelated dotenv entries were not shell-safe to execute.

## Known Issues

The live hosted verifier currently fails closed against real remote state because the latest successful `authoritative-verification.yml` run on `main` is missing the starter failover proof job, and the remote `v0.1.0` tag does not expose the peeled ref required for fresh release truth.

## Files Created/Modified

- `scripts/verify-m053-s03.sh`
- `scripts/tests/verify-m053-s03-contract.test.mjs`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M053/slices/S03/tasks/T02-SUMMARY.md`


## Deviations
Adapted the verifier to derive the GitHub repo slug from `origin` because the repository moved to `hyperpush-org/hyperpush-mono` during execution. Loaded `GH_TOKEN` for the live replay by parsing only that key from `.env` instead of `source .env`, because unrelated dotenv entries were not shell-safe to execute.

## Known Issues
The live hosted verifier currently fails closed against real remote state because the latest successful `authoritative-verification.yml` run on `main` is missing the starter failover proof job, and the remote `v0.1.0` tag does not expose the peeled ref required for fresh release truth.
