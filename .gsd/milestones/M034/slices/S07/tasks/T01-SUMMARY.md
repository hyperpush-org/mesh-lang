---
id: T01
parent: S07
milestone: M034
provides: []
requires: []
affects: []
key_files: ["scripts/lib/m034_public_surface_contract.py", "scripts/verify-m034-s05.sh", "scripts/verify-m034-s05-workflows.sh", ".github/workflows/deploy.yml", ".github/workflows/deploy-services.yml", "scripts/tests/verify-m034-s05-contract.test.mjs", "scripts/tests/verify-m034-s07-public-contract.test.mjs", ".gsd/milestones/M034/slices/S07/tasks/T01-SUMMARY.md"]
key_decisions: ["Centralized the public installer/docs/packages marker set and bounded freshness wait in one Python helper instead of keeping inline Bash/YAML copies.", "Made deploy-services health checks call the same shared live public-surface helper that S05 uses, eliminating the weaker subset contract."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed python3 -m py_compile scripts/lib/m034_public_surface_contract.py, bash -n scripts/verify-m034-s05.sh, node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs scripts/tests/verify-m034-s07-public-contract.test.mjs, and bash scripts/verify-m034-s05-workflows.sh. Broader slice rollout/live replay checks were intentionally left for later slice tasks T02/T03."
completed_at: 2026-03-27T07:09:08.764Z
blocker_discovered: false
---

# T01: Centralized the public-surface contract in a shared helper and rewired S05 plus hosted deploy workflows to consume it.

> Centralized the public-surface contract in a shared helper and rewired S05 plus hosted deploy workflows to consume it.

## What Happened
---
id: T01
parent: S07
milestone: M034
key_files:
  - scripts/lib/m034_public_surface_contract.py
  - scripts/verify-m034-s05.sh
  - scripts/verify-m034-s05-workflows.sh
  - .github/workflows/deploy.yml
  - .github/workflows/deploy-services.yml
  - scripts/tests/verify-m034-s05-contract.test.mjs
  - scripts/tests/verify-m034-s07-public-contract.test.mjs
  - .gsd/milestones/M034/slices/S07/tasks/T01-SUMMARY.md
key_decisions:
  - Centralized the public installer/docs/packages marker set and bounded freshness wait in one Python helper instead of keeping inline Bash/YAML copies.
  - Made deploy-services health checks call the same shared live public-surface helper that S05 uses, eliminating the weaker subset contract.
duration: ""
verification_result: passed
completed_at: 2026-03-27T07:09:08.766Z
blocker_discovered: false
---

# T01: Centralized the public-surface contract in a shared helper and rewired S05 plus hosted deploy workflows to consume it.

**Centralized the public-surface contract in a shared helper and rewired S05 plus hosted deploy workflows to consume it.**

## What Happened

Added scripts/lib/m034_public_surface_contract.py as the single source of truth for public installer/docs/packages markers and the bounded freshness retry budget. Replaced the duplicated local/built/live public-surface logic in scripts/verify-m034-s05.sh with helper invocations, updated the remote-evidence step expectations for deploy-services, rewired deploy.yml and deploy-services.yml to call the shared helper, and rewrote scripts/verify-m034-s05-workflows.sh to pin those helper call-sites while rejecting the pre-S07 shallow curl/grep logic. Updated Node contract coverage to pin helper ownership, workflow wiring, default retry-budget usage, and fail-closed live-surface behavior via a local HTTP harness.

## Verification

Passed python3 -m py_compile scripts/lib/m034_public_surface_contract.py, bash -n scripts/verify-m034-s05.sh, node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs scripts/tests/verify-m034-s07-public-contract.test.mjs, and bash scripts/verify-m034-s05-workflows.sh. Broader slice rollout/live replay checks were intentionally left for later slice tasks T02/T03.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `python3 -m py_compile scripts/lib/m034_public_surface_contract.py` | 0 | ✅ pass | 320ms |
| 2 | `bash -n scripts/verify-m034-s05.sh` | 0 | ✅ pass | 10ms |
| 3 | `node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s06-contract.test.mjs scripts/tests/verify-m034-s07-public-contract.test.mjs` | 0 | ✅ pass | 7330ms |
| 4 | `bash scripts/verify-m034-s05-workflows.sh` | 0 | ✅ pass | 1610ms |


## Deviations

Collapsed deploy-services.yml post-deploy proof from four shallow per-surface steps into one shared Verify public surface contract step so the hosted lane cannot drift from S05 again.

## Known Issues

None within T01 scope. Later slice tasks still need to land the updated workflow graph remotely and rerun the full S05 public replay.

## Files Created/Modified

- `scripts/lib/m034_public_surface_contract.py`
- `scripts/verify-m034-s05.sh`
- `scripts/verify-m034-s05-workflows.sh`
- `.github/workflows/deploy.yml`
- `.github/workflows/deploy-services.yml`
- `scripts/tests/verify-m034-s05-contract.test.mjs`
- `scripts/tests/verify-m034-s07-public-contract.test.mjs`
- `.gsd/milestones/M034/slices/S07/tasks/T01-SUMMARY.md`


## Deviations
Collapsed deploy-services.yml post-deploy proof from four shallow per-surface steps into one shared Verify public surface contract step so the hosted lane cannot drift from S05 again.

## Known Issues
None within T01 scope. Later slice tasks still need to land the updated workflow graph remotely and rerun the full S05 public replay.
