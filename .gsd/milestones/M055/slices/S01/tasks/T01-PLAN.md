---
estimated_steps: 4
estimated_files: 5
skills_used:
  - test
---

# T01: Publish the blessed two-repo workspace and repo-local GSD contract

**Slice:** S01 — Two-Repo Boundary & GSD Authority Contract
**Milestone:** M055

## Description

Make the two-repo working shape visible from repo root before any extraction starts. This task should turn D428 and D429 into durable maintainer-facing contract text: M055 is a two-repo split only, `mesh-lang` keeps docs/installers/registry/packages/public-site surfaces, and `hyperpush-mono` is the product repo that will absorb `mesher/`.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `WORKSPACE.md` contract | fail closed on missing two-repo layout, missing ownership matrix, or missing repo-local `.gsd` rule | N/A for source assertions | treat four-repo wording or umbrella-`.gsd` language as contract drift |
| `scripts/tests/verify-m055-s01-contract.test.mjs` | keep exact required markers aligned with shipped docs | bounded local test only | treat stale allowlists or missing forbidden markers as drift |

## Negative Tests

- **Malformed inputs**: `mesh-packages` or `mesh-website` presented as sibling repos for M055, missing `hyperpush-mono` handoff, or missing repo-local `.gsd` authority wording.
- **Error paths**: README or CONTRIBUTING mention the split without linking to the durable workspace contract.
- **Boundary conditions**: packages site, registry, docs, and installers stay explicitly language-owned inside `mesh-lang` for this milestone.

## Steps

1. Create `WORKSPACE.md` with the blessed sibling layout, ownership matrix, repo-local-versus-cross-repo GSD rule, and the named coordination-layer boundary.
2. Update `README.md` and `CONTRIBUTING.md` so maintainers can discover the workspace contract from the top-level entrypoints instead of reading milestone artifacts.
3. Refresh `.gsd/PROJECT.md` so current-state text stops implying the old monorepo is the durable shape.
4. Add `scripts/tests/verify-m055-s01-contract.test.mjs` with exact marker checks for the two-repo layout, repo-local `.gsd` authority, and forbidden four-repo or umbrella wording.

## Must-Haves

- [ ] `WORKSPACE.md` names only `mesh-lang` and `hyperpush-mono` as the M055 sibling repos.
- [ ] `WORKSPACE.md` explicitly says `website/`, `packages-website/`, `registry/`, and installers remain language-owned inside `mesh-lang` for this milestone.
- [ ] Repo-local `.gsd` stays authoritative; cross-repo work is documented as a lightweight coordination layer rather than one umbrella milestone tree.
- [ ] `README.md`, `CONTRIBUTING.md`, and `.gsd/PROJECT.md` all point at the same split contract.

## Verification

- `node --test scripts/tests/verify-m055-s01-contract.test.mjs`

## Inputs

- `README.md` — current top-level maintainer entrypoint
- `CONTRIBUTING.md` — current contributor workflow guidance
- `.gsd/PROJECT.md` — current-state repo summary that later slices will inherit
- `.gsd/milestones/M055/M055-CONTEXT.md` — milestone boundary and workflow constraints
- `.gsd/milestones/M055/M055-ROADMAP.md` — slice demo and dependency contract

## Expected Output

- `WORKSPACE.md` — durable two-repo workspace and GSD authority contract
- `README.md` — top-level maintainer routing to the workspace contract
- `CONTRIBUTING.md` — contributor workflow updated to the blessed workspace contract
- `.gsd/PROJECT.md` — current-state summary aligned to the split-ready two-repo shape
- `scripts/tests/verify-m055-s01-contract.test.mjs` — source contract for workspace/GSD markers and forbidden stale wording
