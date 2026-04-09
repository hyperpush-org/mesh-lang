---
id: T01
parent: S01
milestone: M055
provides: []
requires: []
affects: []
key_files: ["WORKSPACE.md", "README.md", "CONTRIBUTING.md", ".gsd/PROJECT.md", "scripts/tests/verify-m055-s01-contract.test.mjs"]
key_decisions: ["Turned D428 and D429 into repo-root contract text before any extraction work.", "Guard the split contract with an exact-marker node:test file in scripts/tests instead of relying on prose alone."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "`node --test scripts/tests/verify-m055-s01-contract.test.mjs` passed with all four positive/negative contract cases green. The slice-level wrapper command `bash scripts/verify-m055-s01.sh` was also run to record honest progress and currently fails with exit 127 because T04 has not created the assembled verifier yet."
completed_at: 2026-04-06T17:52:03.858Z
blocker_discovered: false
---

# T01: Published WORKSPACE.md plus repo-root maintainer docs that define the M055 two-repo split and repo-local .gsd authority.

> Published WORKSPACE.md plus repo-root maintainer docs that define the M055 two-repo split and repo-local .gsd authority.

## What Happened
---
id: T01
parent: S01
milestone: M055
key_files:
  - WORKSPACE.md
  - README.md
  - CONTRIBUTING.md
  - .gsd/PROJECT.md
  - scripts/tests/verify-m055-s01-contract.test.mjs
key_decisions:
  - Turned D428 and D429 into repo-root contract text before any extraction work.
  - Guard the split contract with an exact-marker node:test file in scripts/tests instead of relying on prose alone.
duration: ""
verification_result: mixed
completed_at: 2026-04-06T17:52:03.860Z
blocker_discovered: false
---

# T01: Published WORKSPACE.md plus repo-root maintainer docs that define the M055 two-repo split and repo-local .gsd authority.

**Published WORKSPACE.md plus repo-root maintainer docs that define the M055 two-repo split and repo-local .gsd authority.**

## What Happened

Added a new root WORKSPACE.md that defines the blessed M055 sibling layout as exactly mesh-lang/ plus hyperpush-mono/, states that website/, packages-website/, registry/, installers, and evaluator-facing examples remain language-owned in mesh-lang for this milestone, and makes repo-local .gsd authoritative while cross-repo work goes through a lightweight coordination layer. Updated README.md, CONTRIBUTING.md, and .gsd/PROJECT.md so maintainers can discover the same split contract from repo root instead of relying on milestone artifacts. Added scripts/tests/verify-m055-s01-contract.test.mjs in the existing node:test style to fail closed on four-repo sibling layout language, missing WORKSPACE routing, missing hyperpush handoff, missing repo-local .gsd wording, or loss of the language-owned boundary text.

## Verification

`node --test scripts/tests/verify-m055-s01-contract.test.mjs` passed with all four positive/negative contract cases green. The slice-level wrapper command `bash scripts/verify-m055-s01.sh` was also run to record honest progress and currently fails with exit 127 because T04 has not created the assembled verifier yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m055-s01-contract.test.mjs` | 0 | ✅ pass | 609ms |
| 2 | `bash scripts/verify-m055-s01.sh` | 127 | ❌ fail | 9ms |


## Deviations

None.

## Known Issues

`scripts/verify-m055-s01.sh` does not exist yet, so the slice-level assembled verification command currently exits 127. T04 owns that wrapper and its `.tmp/m055-s01/verify/` artifact surface.

## Files Created/Modified

- `WORKSPACE.md`
- `README.md`
- `CONTRIBUTING.md`
- `.gsd/PROJECT.md`
- `scripts/tests/verify-m055-s01-contract.test.mjs`


## Deviations
None.

## Known Issues
`scripts/verify-m055-s01.sh` does not exist yet, so the slice-level assembled verification command currently exits 127. T04 owns that wrapper and its `.tmp/m055-s01/verify/` artifact surface.
