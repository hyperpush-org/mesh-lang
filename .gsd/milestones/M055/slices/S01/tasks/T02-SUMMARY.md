---
id: T02
parent: S01
milestone: M055
provides: []
requires: []
affects: []
key_files: ["scripts/lib/repo-identity.json", "scripts/lib/m034_public_surface_contract.py", "scripts/tests/verify-m055-s01-contract.test.mjs", ".gsd/DECISIONS.md"]
key_decisions: ["Canonical repo identity now lives in scripts/lib/repo-identity.json, while shipped installers remain plain source/copy scripts validated against it rather than parsing repo-local JSON at runtime.", "Left the installer bytes unchanged because the editable source pair and docs-served copies already matched the canonical language slug; enforced parity and drift detection in the helper and slice-owned Node rail instead."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task verification passed: diff -u tools/install/install.sh website/docs/public/install.sh, diff -u tools/install/install.ps1 website/docs/public/install.ps1, python3 scripts/lib/m034_public_surface_contract.py local-docs --root ., and node --test scripts/tests/verify-m055-s01-contract.test.mjs all succeeded. Targeted regression replays also passed for node --test scripts/tests/verify-m034-s05-contract.test.mjs and node --test scripts/tests/verify-m034-s07-public-contract.test.mjs. Slice-level partial verification was also run via bash scripts/verify-m055-s01.sh and still exits 127 because T04 has not created that wrapper yet."
completed_at: 2026-04-06T18:04:21.623Z
blocker_discovered: false
---

# T02: Added scripts/lib/repo-identity.json and rewired installer/docs contract checks to validate against it while preserving source/public installer parity.

> Added scripts/lib/repo-identity.json and rewired installer/docs contract checks to validate against it while preserving source/public installer parity.

## What Happened
---
id: T02
parent: S01
milestone: M055
key_files:
  - scripts/lib/repo-identity.json
  - scripts/lib/m034_public_surface_contract.py
  - scripts/tests/verify-m055-s01-contract.test.mjs
  - .gsd/DECISIONS.md
key_decisions:
  - Canonical repo identity now lives in scripts/lib/repo-identity.json, while shipped installers remain plain source/copy scripts validated against it rather than parsing repo-local JSON at runtime.
  - Left the installer bytes unchanged because the editable source pair and docs-served copies already matched the canonical language slug; enforced parity and drift detection in the helper and slice-owned Node rail instead.
duration: ""
verification_result: mixed
completed_at: 2026-04-06T18:04:21.626Z
blocker_discovered: false
---

# T02: Added scripts/lib/repo-identity.json and rewired installer/docs contract checks to validate against it while preserving source/public installer parity.

**Added scripts/lib/repo-identity.json and rewired installer/docs contract checks to validate against it while preserving source/public installer parity.**

## What Happened

Added scripts/lib/repo-identity.json as the canonical M055 repo-identity source with explicit language-repo versus product-repo slugs, GitHub URLs, issue trackers, blob bases, and the language-owned installer/docs roots. Refactored scripts/lib/m034_public_surface_contract.py so its describe/local-docs surfaces derive installer URLs, docs URLs, and VS Code metadata expectations from that JSON instead of carrying a second hand-copied repo-identity table. Expanded scripts/tests/verify-m055-s01-contract.test.mjs so it now fails closed on malformed repo-identity JSON, missing mesh-lang or hyperpush-mono identity fields, helper drift back to hardcoded repo/install/docs values, and byte drift between tools/install/install.{sh,ps1} and website/docs/public/install.{sh,ps1}. Recorded D433 to preserve the intended boundary for later slices.

## Verification

Task verification passed: diff -u tools/install/install.sh website/docs/public/install.sh, diff -u tools/install/install.ps1 website/docs/public/install.ps1, python3 scripts/lib/m034_public_surface_contract.py local-docs --root ., and node --test scripts/tests/verify-m055-s01-contract.test.mjs all succeeded. Targeted regression replays also passed for node --test scripts/tests/verify-m034-s05-contract.test.mjs and node --test scripts/tests/verify-m034-s07-public-contract.test.mjs. Slice-level partial verification was also run via bash scripts/verify-m055-s01.sh and still exits 127 because T04 has not created that wrapper yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `diff -u tools/install/install.sh website/docs/public/install.sh` | 0 | ✅ pass | 30ms |
| 2 | `diff -u tools/install/install.ps1 website/docs/public/install.ps1` | 0 | ✅ pass | 89ms |
| 3 | `python3 scripts/lib/m034_public_surface_contract.py local-docs --root .` | 0 | ✅ pass | 411ms |
| 4 | `node --test scripts/tests/verify-m055-s01-contract.test.mjs` | 0 | ✅ pass | 2154ms |
| 5 | `node --test scripts/tests/verify-m034-s05-contract.test.mjs` | 0 | ✅ pass | 1007ms |
| 6 | `node --test scripts/tests/verify-m034-s07-public-contract.test.mjs` | 0 | ✅ pass | 4459ms |
| 7 | `bash scripts/verify-m055-s01.sh` | 127 | ❌ fail | 111ms |


## Deviations

None.

## Known Issues

`bash scripts/verify-m055-s01.sh` still fails with exit 127 because T04 has not created the slice-level wrapper yet. That is expected until the final task lands.

## Files Created/Modified

- `scripts/lib/repo-identity.json`
- `scripts/lib/m034_public_surface_contract.py`
- `scripts/tests/verify-m055-s01-contract.test.mjs`
- `.gsd/DECISIONS.md`


## Deviations
None.

## Known Issues
`bash scripts/verify-m055-s01.sh` still fails with exit 127 because T04 has not created the slice-level wrapper yet. That is expected until the final task lands.
