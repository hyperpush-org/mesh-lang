---
estimated_steps: 3
estimated_files: 5
skills_used:
  - bash-scripting
  - test
---

# T01: Lock the nested `hyperpush-mono/mesher` toolchain contract

Resolve the extracted repo-shape contradiction before any workspace handoff. The public contract already says Hyperpush lives at `hyperpush-mono/mesher/...`, but the Mesher toolchain and contract tests still assume the extracted package root sits directly next to `mesh-lang`. This task should make the nested extracted layout the only truthful product contract while preserving the current in-repo `mesh-lang/mesher` path for local development.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `mesher/scripts/lib/mesh-toolchain.sh` path resolution | Fail closed with the exact missing root/path and do not fall back to a lower-priority source tier. | N/A for local path checks. | Treat a mixed `../mesh-lang`/`../../mesh-lang` contract as resolver drift. |
| `scripts/tests/verify-m055-s02-contract.test.mjs` and `scripts/tests/verify-m055-s04-contract.test.mjs` layout simulations | Stop on the first temp-workspace contract mismatch and keep the failing fixture root. | N/A for local node:test runs. | Treat missing nested-repo markers or stale direct-sibling assumptions as real contract breaks. |

## Load Profile

- **Shared resources**: temp workspace fixtures created by the node tests and the product README/workspace contract text.
- **Per-operation cost**: local node tests and shell/path resolution only.
- **10x breakpoint**: stale path assumptions across multiple docs/tests, not CPU or memory.

## Negative Tests

- **Malformed inputs**: extracted repo rooted at `hyperpush-mono/mesher` still looking for sibling `../mesh-lang`, or repo docs still describing the wrong layout.
- **Error paths**: enclosing-source detection works, but extracted sibling-workspace detection still points at the stale path.
- **Boundary conditions**: the current in-repo `mesh-lang/mesher` flow still resolves as `enclosing-source` while the extracted nested repo resolves as the blessed sibling layout.

## Steps

1. Update `mesher/scripts/lib/mesh-toolchain.sh` so it prefers the current enclosing `mesh-lang` checkout first, then resolves the extracted nested `hyperpush-mono/mesher` layout against the sibling `mesh-lang` repo, and still keeps the installed `PATH` fallback fail-closed.
2. Rewrite `mesher/README.md` and `WORKSPACE.md` so the documented extracted shape is explicitly `mesh-lang/` beside `hyperpush-mono/`, with Mesher nested under the product repo instead of flattened at repo root.
3. Extend the existing Mesher contract test and add slice-owned S04 contract assertions so both the in-repo and extracted-nested layouts are simulated and stale `../mesh-lang` assumptions fail closed.

## Must-Haves

- [ ] The extracted product layout is explicitly `hyperpush-mono/mesher/...`, not a flattened direct-sibling package root.
- [ ] Mesher’s resolver still works from the current in-repo `mesh-lang/mesher` path and from the extracted nested product repo.
- [ ] Contract tests fail on stale direct-sibling `../mesh-lang` assumptions and missing nested-layout docs.

## Inputs

- `mesher/scripts/lib/mesh-toolchain.sh`
- `mesher/README.md`
- `WORKSPACE.md`
- `scripts/tests/verify-m055-s02-contract.test.mjs`
- `scripts/lib/repo-identity.json`

## Expected Output

- `mesher/scripts/lib/mesh-toolchain.sh`
- `mesher/README.md`
- `WORKSPACE.md`
- `scripts/tests/verify-m055-s02-contract.test.mjs`
- `scripts/tests/verify-m055-s04-contract.test.mjs`

## Verification

node --test scripts/tests/verify-m055-s02-contract.test.mjs scripts/tests/verify-m055-s04-contract.test.mjs

## Observability Impact

- Signals added/changed: resolver stderr must report the chosen source tier/root, and the S04 contract test must retain the temp workspace root on path drift.
- How a future agent inspects this: rerun the node contract tests and read the failing temp fixture path plus resolver stderr.
- Failure state exposed: wrong sibling path, missing nested-layout docs, or stale fallback behavior.
