---
id: T03
parent: S02
milestone: M055
provides: []
requires: []
affects: []
key_files: ["mesher/README.md", "mesher/.env.example", "scripts/tests/verify-m055-s02-contract.test.mjs"]
key_decisions: ["Treat `bash scripts/verify-maintainer-surface.sh` as the primary deeper-app proof command in the Mesher runbook and keep `bash scripts/verify-m051-s01.sh` documented as compatibility-only from the mesh-lang repo root.", "Pin README and env-example command wording in the slice-owned Node contract so maintainer-runbook drift fails closed instead of relying on prose review."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the T03 verification commands from the task plan and slice excerpt: `node --test scripts/tests/verify-m055-s02-contract.test.mjs` and `bash mesher/scripts/test.sh`. The Node rail passed all seven checks, including the new README/env-example contract assertions. The package-local Mesher test wrapper resolved `meshc` from the enclosing source checkout and passed the Mesher package tests. No additional slice-level verification block is present in `S02-PLAN.md` beyond these T03 checks; T04-owned verification remains pending."
completed_at: 2026-04-06T19:48:07.378Z
blocker_discovered: false
---

# T03: Rewrote the Mesher maintainer runbook around package-local commands and pinned it with the slice-owned contract test.

> Rewrote the Mesher maintainer runbook around package-local commands and pinned it with the slice-owned contract test.

## What Happened
---
id: T03
parent: S02
milestone: M055
key_files:
  - mesher/README.md
  - mesher/.env.example
  - scripts/tests/verify-m055-s02-contract.test.mjs
key_decisions:
  - Treat `bash scripts/verify-maintainer-surface.sh` as the primary deeper-app proof command in the Mesher runbook and keep `bash scripts/verify-m051-s01.sh` documented as compatibility-only from the mesh-lang repo root.
  - Pin README and env-example command wording in the slice-owned Node contract so maintainer-runbook drift fails closed instead of relying on prose review.
duration: ""
verification_result: passed
completed_at: 2026-04-06T19:48:07.379Z
blocker_discovered: false
---

# T03: Rewrote the Mesher maintainer runbook around package-local commands and pinned it with the slice-owned contract test.

**Rewrote the Mesher maintainer runbook around package-local commands and pinned it with the slice-owned contract test.**

## What Happened

Rewrote `mesher/README.md` so the maintainer loop now starts from the package root, explains the explicit `meshc` resolution order, teaches the package-local test/migrate/build/smoke commands, and makes the product-owned Mesher verifier primary while framing the repo-root M051 verifier as compatibility-only. Kept the startup env, seeded default org/project/API key, live HTTP smoke examples, and runtime inspection commands truthful while changing only the runbook/toolchain contract. Updated `mesher/.env.example` comments to match the package-local migration flow. Extended `scripts/tests/verify-m055-s02-contract.test.mjs` so the slice-owned Node rail now verifies the README/env-example markers and forbids stale repo-root Mesher commands or `./mesher/mesher` instructions.

## Verification

Passed the T03 verification commands from the task plan and slice excerpt: `node --test scripts/tests/verify-m055-s02-contract.test.mjs` and `bash mesher/scripts/test.sh`. The Node rail passed all seven checks, including the new README/env-example contract assertions. The package-local Mesher test wrapper resolved `meshc` from the enclosing source checkout and passed the Mesher package tests. No additional slice-level verification block is present in `S02-PLAN.md` beyond these T03 checks; T04-owned verification remains pending.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m055-s02-contract.test.mjs` | 0 | ✅ pass | 5794ms |
| 2 | `bash mesher/scripts/test.sh` | 0 | ✅ pass | 61730ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `mesher/README.md`
- `mesher/.env.example`
- `scripts/tests/verify-m055-s02-contract.test.mjs`


## Deviations
None.

## Known Issues
None.
