---
id: T01
parent: S03
milestone: M053
provides: []
requires: []
affects: []
key_files: [".github/workflows/authoritative-starter-failover-proof.yml", ".github/workflows/authoritative-verification.yml", ".github/workflows/release.yml", "scripts/verify-m034-s02-workflows.sh", "scripts/tests/verify-m053-s03-contract.test.mjs", ".gsd/milestones/M053/slices/S03/tasks/T01-SUMMARY.md"]
key_decisions: ["D407: Host the serious starter failover proof in its own secret-free reusable workflow with runner-local Postgres, call it from authoritative-verification after whitespace-guard, and require it in tag release gating alongside authoritative-live-proof."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-level verification passed with bash scripts/verify-m034-s02-workflows.sh and node --test scripts/tests/verify-m053-s03-contract.test.mjs. The slice-level hosted verifier slot was exercised and correctly remained pending with exit code 2 because scripts/verify-m053-s03.sh is not created until T02."
completed_at: 2026-04-05T20:36:58.056Z
blocker_discovered: false
---

# T01: Added a reusable hosted starter failover proof workflow and wired it into authoritative main/tag gates.

> Added a reusable hosted starter failover proof workflow and wired it into authoritative main/tag gates.

## What Happened
---
id: T01
parent: S03
milestone: M053
key_files:
  - .github/workflows/authoritative-starter-failover-proof.yml
  - .github/workflows/authoritative-verification.yml
  - .github/workflows/release.yml
  - scripts/verify-m034-s02-workflows.sh
  - scripts/tests/verify-m053-s03-contract.test.mjs
  - .gsd/milestones/M053/slices/S03/tasks/T01-SUMMARY.md
key_decisions:
  - D407: Host the serious starter failover proof in its own secret-free reusable workflow with runner-local Postgres, call it from authoritative-verification after whitespace-guard, and require it in tag release gating alongside authoritative-live-proof.
duration: ""
verification_result: mixed
completed_at: 2026-04-05T20:36:58.058Z
blocker_discovered: false
---

# T01: Added a reusable hosted starter failover proof workflow and wired it into authoritative main/tag gates.

**Added a reusable hosted starter failover proof workflow and wired it into authoritative main/tag gates.**

## What Happened

Added a dedicated reusable GitHub Actions workflow for the serious starter failover proof, provisioned runner-local Postgres inside that workflow, and wired the new lane into authoritative-verification and release without weakening the existing authoritative live proof contract. Extended the local workflow verifier to fail closed on starter-lane drift and added a fast Node contract test with negative cases for reusable-workflow, caller, release, and verifier regressions.

## Verification

Task-level verification passed with bash scripts/verify-m034-s02-workflows.sh and node --test scripts/tests/verify-m053-s03-contract.test.mjs. The slice-level hosted verifier slot was exercised and correctly remained pending with exit code 2 because scripts/verify-m053-s03.sh is not created until T02.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash scripts/verify-m034-s02-workflows.sh` | 0 | ✅ pass | 1199ms |
| 2 | `node --test scripts/tests/verify-m053-s03-contract.test.mjs` | 0 | ✅ pass | 505ms |
| 3 | `if [[ -f scripts/verify-m053-s03.sh ]]; then GH_TOKEN=${GH_TOKEN:?set GH_TOKEN} bash scripts/verify-m053-s03.sh; else echo 'pending: scripts/verify-m053-s03.sh is not created until T02' >&2; exit 2; fi` | 2 | ❌ fail | 7ms |


## Deviations

None.

## Known Issues

None within T01 scope. The slice-level hosted verifier scripts/verify-m053-s03.sh remains pending until T02 creates it.

## Files Created/Modified

- `.github/workflows/authoritative-starter-failover-proof.yml`
- `.github/workflows/authoritative-verification.yml`
- `.github/workflows/release.yml`
- `scripts/verify-m034-s02-workflows.sh`
- `scripts/tests/verify-m053-s03-contract.test.mjs`
- `.gsd/milestones/M053/slices/S03/tasks/T01-SUMMARY.md`


## Deviations
None.

## Known Issues
None within T01 scope. The slice-level hosted verifier scripts/verify-m053-s03.sh remains pending until T02 creates it.
