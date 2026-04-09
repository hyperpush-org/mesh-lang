# Repo Split Summary

Last updated: 2026-04-08

## Current state

The split is now real across the two repos.

### `mesh-lang`

- Branch: `m055-repo-split-remove-mesher`
- HEAD: `a9ad114b9541380d66716e5e7f7805856dbc7c28`
- PR: `hyperpush-org/mesh-lang#2`
- URL: https://github.com/hyperpush-org/mesh-lang/pull/2

What is true now:

- `mesh-lang` no longer tracks `mesher/**`
- split-boundary docs/helpers/verifiers were updated to use the real sibling product repo
- local compatibility still works through:
  - `bash scripts/setup-local-workspace.sh`
  - `mesh-lang/mesher -> ../hyperpush-mono/mesher`
- the tracked clustered fixture binaries were removed from `mesh-lang`
  - `scripts/fixtures/clustered/tiny-cluster/tiny-cluster`
  - `scripts/fixtures/clustered/cluster-proof/cluster-proof`
- the route-free clustered fixture rails now fail closed if those binaries reappear

### Product repo (`hyperpush`)

- Local branch: `m055-product-only-root`
- Local HEAD: `393582244db03287c829b505e11ef6962394722e`
- PR: `hyperpush-org/hyperpush#10`
- URL: https://github.com/hyperpush-org/hyperpush/pull/10

Important note:

- GitHub redirected `hyperpush-org/hyperpush-mono` to `hyperpush-org/hyperpush`
- the product PR is therefore opened against `hyperpush`

What is true in that branch:

- copied language-owned root surfaces were stripped out of the product repo
- the repo keeps a **product-only root** with:
  - `mesher/`
  - `mesher/landing/`
  - `mesher/frontend-exp/`
  - minimal product-root docs / workflows / verifiers
- `frontend-exp` remains product-owned at:
  - `mesher/frontend-exp/`
- product CI was rewritten around:
  - Mesher maintainer verification
  - landing build
  - `frontend-exp` build

## Verification already run

### `mesh-lang`

- `node --test scripts/tests/verify-m055-s01-contract.test.mjs`
- `node --test scripts/tests/verify-m055-s04-contract.test.mjs`
- `node --test scripts/tests/verify-m055-s04-materialize.test.mjs`
- `bash scripts/setup-local-workspace.sh`
- `bash scripts/verify-m055-s01.sh`
- `bash scripts/verify-m055-s04.sh`
- `cargo test -p meshc --test e2e_m051_s01 -- --nocapture`
- `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_contract_remains_source_first_and_route_free -- --nocapture`
- `cargo test -p meshc --test e2e_m046_s04 m046_s04_cluster_proof_package_builds_to_temp_output_and_runs_repo_smoke_rail -- --nocapture`
- `cargo test -p meshc --test e2e_m046_s04 m046_s04_cluster_proof_package_contract_remains_source_first_and_route_free -- --nocapture`

### Product repo (`hyperpush`)

- `bash scripts/verify-landing-surface.sh`
- `bash scripts/verify-m051-s01.sh`
- `npm --prefix mesher/landing ci && npm --prefix mesher/landing run build`
- `npm --prefix mesher/frontend-exp ci && npm --prefix mesher/frontend-exp run build`

## Follow-up actions

### Immediate PR follow-up

1. Review `hyperpush-org/mesh-lang#2`
2. Review `hyperpush-org/hyperpush#10`
3. Decide merge order explicitly

Recommended merge order:

1. merge `mesh-lang` split PR first
2. merge product-root cleanup PR second
3. rerun the `mesh-lang` split-boundary wrappers against the merged product repo shape

Reason:

- `mesh-lang` is already separated and points at the sibling product repo
- the slimmer product-root branch changes the product verifier/doc shape further
- a short `mesh-lang` follow-up may still be needed after the product-root cleanup lands

### After both PRs merge

Run these again in a fresh sibling workspace:

#### `mesh-lang`

- `bash scripts/setup-local-workspace.sh`
- `bash scripts/verify-m055-s01.sh`
- `bash scripts/verify-m055-s04.sh`

#### Product repo

- `bash scripts/verify-landing-surface.sh`
- `bash scripts/verify-m051-s01.sh`
- `npm --prefix mesher/landing ci && npm --prefix mesher/landing run build`
- `npm --prefix mesher/frontend-exp ci && npm --prefix mesher/frontend-exp run build`

### Likely code follow-up

1. **Recheck `mesh-lang` split wrappers after the product-root cleanup merges**
   - `mesh-lang` currently assumes the product repo still exposes the root wrapper and landing verifier
   - those still exist in the product PR, but the slimmer root contract should be replayed once merged

2. **Fix the landing `/pitch` export-path warnings**
   - current `mesher/landing` build stays green
   - but Next reported unresolved dynamic imports for:
     - `html2canvas`
     - `jspdf/dist/jspdf.es.min.js`
   - this is not a split blocker, but it is a real follow-up item

3. **Watch product CI after merge**
   - new product CI checks out sibling `mesh-lang` and builds `meshc`
   - confirm the GitHub checkout/permissions path works in hosted CI, not just locally

## Explicit ownership note

`frontend-exp` is product-owned and should stay in the product repo.

Canonical location:

- `hyperpush/mesher/frontend-exp/`

It should **not** move back into `mesh-lang`.

## Practical workspace after merge

```text
<workspace>/
  mesh-lang/
  hyperpush/
    mesher/
    mesher/landing/
    mesher/frontend-exp/
```

Local compatibility path from the language repo remains acceptable:

```text
mesh-lang/mesher -> ../hyperpush/mesher
```

That path is local-only convenience, not canonical GitHub ownership.
