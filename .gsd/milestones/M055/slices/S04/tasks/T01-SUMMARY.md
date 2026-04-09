---
id: T01
parent: S04
milestone: M055
provides: []
requires: []
affects: []
key_files: ["mesher/scripts/lib/mesh-toolchain.sh", "mesher/README.md", "WORKSPACE.md", "scripts/tests/verify-m055-s02-contract.test.mjs", "scripts/tests/verify-m055-s04-contract.test.mjs"]
key_decisions: ["D446: Treat `hyperpush-mono/mesher` as the only blessed extracted product package root and fail closed on flat or mixed sibling layouts."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "`bash -n mesher/scripts/lib/mesh-toolchain.sh` passed. `node --test scripts/tests/verify-m055-s02-contract.test.mjs scripts/tests/verify-m055-s04-contract.test.mjs` passed with the new nested-layout success and failure cases. The slice-level `node --test scripts/tests/verify-m055-s04-contract.test.mjs` rail also passed. Slice-level checks that belong to later tasks still fail explicitly: `node --test scripts/tests/verify-m055-s04-materialize.test.mjs` reports the file is missing, and `bash scripts/verify-m055-s04.sh` reports the wrapper is missing."
completed_at: 2026-04-07T07:02:31.311Z
blocker_discovered: false
---

# T01: Locked Mesher’s toolchain/docs/tests to the nested `hyperpush-mono/mesher` workspace shape and made stale flat-sibling assumptions fail closed.

> Locked Mesher’s toolchain/docs/tests to the nested `hyperpush-mono/mesher` workspace shape and made stale flat-sibling assumptions fail closed.

## What Happened
---
id: T01
parent: S04
milestone: M055
key_files:
  - mesher/scripts/lib/mesh-toolchain.sh
  - mesher/README.md
  - WORKSPACE.md
  - scripts/tests/verify-m055-s02-contract.test.mjs
  - scripts/tests/verify-m055-s04-contract.test.mjs
key_decisions:
  - D446: Treat `hyperpush-mono/mesher` as the only blessed extracted product package root and fail closed on flat or mixed sibling layouts.
duration: ""
verification_result: mixed
completed_at: 2026-04-07T07:02:31.315Z
blocker_discovered: false
---

# T01: Locked Mesher’s toolchain/docs/tests to the nested `hyperpush-mono/mesher` workspace shape and made stale flat-sibling assumptions fail closed.

**Locked Mesher’s toolchain/docs/tests to the nested `hyperpush-mono/mesher` workspace shape and made stale flat-sibling assumptions fail closed.**

## What Happened

Updated `mesher/scripts/lib/mesh-toolchain.sh` so Mesher still prefers an enclosing `mesh-lang` checkout, but extracted product work now only resolves `sibling-workspace` from the blessed nested `hyperpush-mono/mesher` layout. Missing blessed sibling repos and mixed `../mesh-lang` / `../../mesh-lang` candidates now fail closed with explicit drift messages instead of falling through to PATH. Rewrote `mesher/README.md` and `WORKSPACE.md` so the extracted product contract is explicitly `mesh-lang/` beside `hyperpush-mono/`, with Mesher nested under the product repo. Extended `scripts/tests/verify-m055-s02-contract.test.mjs` to simulate the nested extracted layout and the required negative cases, and added `scripts/tests/verify-m055-s04-contract.test.mjs` to pin the cross-file nested workspace contract across the resolver, docs, and repo identity surfaces.

## Verification

`bash -n mesher/scripts/lib/mesh-toolchain.sh` passed. `node --test scripts/tests/verify-m055-s02-contract.test.mjs scripts/tests/verify-m055-s04-contract.test.mjs` passed with the new nested-layout success and failure cases. The slice-level `node --test scripts/tests/verify-m055-s04-contract.test.mjs` rail also passed. Slice-level checks that belong to later tasks still fail explicitly: `node --test scripts/tests/verify-m055-s04-materialize.test.mjs` reports the file is missing, and `bash scripts/verify-m055-s04.sh` reports the wrapper is missing.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -n mesher/scripts/lib/mesh-toolchain.sh` | 0 | ✅ pass | 15ms |
| 2 | `node --test scripts/tests/verify-m055-s02-contract.test.mjs scripts/tests/verify-m055-s04-contract.test.mjs` | 0 | ✅ pass | 6584ms |
| 3 | `node --test scripts/tests/verify-m055-s04-contract.test.mjs` | 0 | ✅ pass | 1574ms |
| 4 | `node --test scripts/tests/verify-m055-s04-materialize.test.mjs` | 1 | ❌ fail | 691ms |
| 5 | `bash scripts/verify-m055-s04.sh` | 127 | ❌ fail | 16ms |


## Deviations

None.

## Known Issues

`scripts/tests/verify-m055-s04-materialize.test.mjs` and `scripts/verify-m055-s04.sh` do not exist yet, so those slice-level verification commands remain red until T02 and T04 land.

## Files Created/Modified

- `mesher/scripts/lib/mesh-toolchain.sh`
- `mesher/README.md`
- `WORKSPACE.md`
- `scripts/tests/verify-m055-s02-contract.test.mjs`
- `scripts/tests/verify-m055-s04-contract.test.mjs`


## Deviations
None.

## Known Issues
`scripts/tests/verify-m055-s04-materialize.test.mjs` and `scripts/verify-m055-s04.sh` do not exist yet, so those slice-level verification commands remain red until T02 and T04 land.
