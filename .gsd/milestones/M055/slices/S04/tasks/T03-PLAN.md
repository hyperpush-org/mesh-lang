---
estimated_steps: 4
estimated_files: 5
skills_used:
  - bash-scripting
  - test
  - github-workflows
---

# T03: Retarget mesh-lang compatibility and hosted-evidence rails to the sibling product repo

Once the staged product repo exists, stop treating the in-repo `mesher/` tree or the current `origin` remote as authoritative. This task should rewrite mesh-lang-side compatibility and hosted-evidence rails to find the sibling product repo explicitly and to derive the language repo slug from the canonical identity contract.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| sibling-workspace resolver/helper | Fail closed on missing or ambiguous `hyperpush-mono` roots and report the exact path source (env override vs blessed sibling). | N/A for local path checks. | Treat a fallback to the in-repo `mesher/` path as contract drift. |
| `scripts/verify-m051-s01.sh` delegation | Stop before any local Mesher replay if the sibling product repo or delegated verifier path is missing. | Use the delegated product verifier timeout budget and keep the failing repo root visible. | Treat a wrapper success without the sibling repo’s markers as false proof. |
| `scripts/verify-m053-s03.sh` hosted repo identity | Fail if the default repo slug still comes from `origin` instead of `scripts/lib/repo-identity.json`. | Preserve the existing hosted-evidence timeout budgets. | Treat malformed repo-slug resolution or mixed language/product refs as hosted-proof drift. |

## Load Profile

- **Shared resources**: `.tmp/m055-s04/workspace/`, `.tmp/m051-s01/verify/`, and hosted-evidence repo/ref metadata.
- **Per-operation cost**: one staged compatibility-wrapper replay plus local contract tests.
- **10x breakpoint**: repeated verifier delegation and repo-ref drift, not CPU.

## Negative Tests

- **Malformed inputs**: only the stale in-repo `mesher/` path exists, `origin` points at the product repo, or the wrapper is given a malformed sibling root.
- **Error paths**: the product verifier is green in the staged repo, but the mesh-lang compatibility wrapper still runs the local copy; or the hosted verifier still reads the wrong slug by default.
- **Boundary conditions**: explicit env overrides still work for debugging, but the default path and default language repo slug come from the blessed workspace contract and repo identity.

## Steps

1. Add a small workspace-resolution helper so mesh-lang compatibility rails can find sibling `hyperpush-mono` from the blessed workspace layout or an explicit env override instead of assuming in-repo `mesher/`.
2. Rewrite `scripts/verify-m051-s01.sh` to delegate to the sibling product repo’s `mesher/scripts/verify-maintainer-surface.sh` and fail closed if only the stale in-repo path exists.
3. Rewrite `scripts/verify-m053-s03.sh` so the default language repo slug comes from `scripts/lib/repo-identity.json` rather than `git remote get-url origin`, while keeping explicit override support.
4. Extend the S04 contract test to pin the sibling-wrapper and hosted-evidence behavior.

## Must-Haves

- [ ] `scripts/verify-m051-s01.sh` no longer treats the in-repo `mesher/` tree as authoritative.
- [ ] `scripts/verify-m053-s03.sh` defaults to the canonical language repo slug instead of the current `origin` remote.
- [ ] Contract tests fail on hidden local delegation or repo-slug drift.

## Inputs

- `scripts/lib/repo-identity.json`
- `scripts/materialize-hyperpush-mono.mjs`
- `scripts/verify-m051-s01.sh`
- `scripts/verify-m053-s03.sh`
- `WORKSPACE.md`
- `scripts/tests/verify-m055-s04-contract.test.mjs`

## Expected Output

- `scripts/lib/m055-workspace.sh`
- `scripts/verify-m051-s01.sh`
- `scripts/verify-m053-s03.sh`
- `scripts/tests/verify-m055-s04-contract.test.mjs`
- `WORKSPACE.md`

## Verification

node --test scripts/tests/verify-m055-s04-contract.test.mjs
node scripts/materialize-hyperpush-mono.mjs --check
M055_HYPERPUSH_ROOT=.tmp/m055-s04/workspace/hyperpush-mono bash scripts/verify-m051-s01.sh

## Observability Impact

- Signals added/changed: compatibility-wrapper output must report the resolved product repo root, and hosted evidence must report the resolved language repo slug/ref source.
- How a future agent inspects this: rerun the S04 contract test, then run the wrapper with `M055_HYPERPUSH_ROOT=...` and inspect the first lines plus delegated phase markers.
- Failure state exposed: wrong sibling root, origin-derived repo slug drift, missing product repo override, or wrapper delegation to the stale in-repo tree.
