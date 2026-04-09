---
id: T03
parent: S04
milestone: M055
provides: []
requires: []
affects: []
key_files: ["scripts/lib/m055-workspace.sh", "scripts/verify-m051-s01.sh", "scripts/verify-m053-s03.sh", "scripts/tests/verify-m055-s04-contract.test.mjs", "WORKSPACE.md"]
key_decisions: ["D448: Use scripts/lib/m055-workspace.sh plus scripts/lib/repo-identity.json as the authoritative sources for sibling hyperpush-mono resolution and default language repo slug resolution."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "`node --test scripts/tests/verify-m055-s04-contract.test.mjs` passed all five cases, including live compatibility-wrapper delegation and hosted-verifier repo-identity resolution with stubbed gh/git surfaces. `node scripts/materialize-hyperpush-mono.mjs --check` refreshed the staged product repo successfully. `M055_HYPERPUSH_ROOT=.tmp/m055-s04/workspace/hyperpush-mono bash scripts/verify-m051-s01.sh` printed the resolved product repo root/source and completed against the staged sibling repo's Mesher maintainer verifier."
completed_at: 2026-04-07T08:37:45.045Z
blocker_discovered: false
---

# T03: Retargeted mesh-lang compatibility and hosted-evidence verifiers to the sibling hyperpush-mono repo and the canonical repo-identity contract.

> Retargeted mesh-lang compatibility and hosted-evidence verifiers to the sibling hyperpush-mono repo and the canonical repo-identity contract.

## What Happened
---
id: T03
parent: S04
milestone: M055
key_files:
  - scripts/lib/m055-workspace.sh
  - scripts/verify-m051-s01.sh
  - scripts/verify-m053-s03.sh
  - scripts/tests/verify-m055-s04-contract.test.mjs
  - WORKSPACE.md
key_decisions:
  - D448: Use scripts/lib/m055-workspace.sh plus scripts/lib/repo-identity.json as the authoritative sources for sibling hyperpush-mono resolution and default language repo slug resolution.
duration: ""
verification_result: passed
completed_at: 2026-04-07T08:37:45.046Z
blocker_discovered: false
---

# T03: Retargeted mesh-lang compatibility and hosted-evidence verifiers to the sibling hyperpush-mono repo and the canonical repo-identity contract.

**Retargeted mesh-lang compatibility and hosted-evidence verifiers to the sibling hyperpush-mono repo and the canonical repo-identity contract.**

## What Happened

Added a shared workspace/identity shell helper, moved the mesh-lang compatibility wrapper onto the sibling hyperpush-mono repo instead of the in-repo mesher tree, switched the hosted-evidence verifier to resolve its default language repo slug from scripts/lib/repo-identity.json, extended the S04 contract test into a behavioral rail with stubbed sibling and hosted surfaces, and updated WORKSPACE.md to document the new boundaries.

## Verification

`node --test scripts/tests/verify-m055-s04-contract.test.mjs` passed all five cases, including live compatibility-wrapper delegation and hosted-verifier repo-identity resolution with stubbed gh/git surfaces. `node scripts/materialize-hyperpush-mono.mjs --check` refreshed the staged product repo successfully. `M055_HYPERPUSH_ROOT=.tmp/m055-s04/workspace/hyperpush-mono bash scripts/verify-m051-s01.sh` printed the resolved product repo root/source and completed against the staged sibling repo's Mesher maintainer verifier.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m055-s04-contract.test.mjs` | 0 | ✅ pass | 3666ms |
| 2 | `node scripts/materialize-hyperpush-mono.mjs --check` | 0 | ✅ pass | 2604ms |
| 3 | `M055_HYPERPUSH_ROOT=.tmp/m055-s04/workspace/hyperpush-mono bash scripts/verify-m051-s01.sh` | 0 | ✅ pass | 97299ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `scripts/lib/m055-workspace.sh`
- `scripts/verify-m051-s01.sh`
- `scripts/verify-m053-s03.sh`
- `scripts/tests/verify-m055-s04-contract.test.mjs`
- `WORKSPACE.md`


## Deviations
None.

## Known Issues
None.
