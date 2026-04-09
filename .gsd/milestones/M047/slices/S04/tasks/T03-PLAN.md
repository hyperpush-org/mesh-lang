---
estimated_steps: 4
estimated_files: 7
skills_used:
  - test
---

# T03: Dogfood tiny-cluster and cluster-proof on the source-first contract

Migrate the canonical route-free example packages together so the repo stops teaching one thing in parser/scaffold land and another thing in package land. Both packages should use the same `@cluster`-based `execute_declared_work` shape, keep the visible `1 + 1` body, and continue proving the general-function clustered model without inventing HTTP-route claims.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| package source + package test parity | fail package tests and build smoke instead of leaving the examples subtly divergent | N/A | do not keep one package on `@cluster` and the other on `clustered(work)` |
| README contract wording | treat stale wording as a contract failure because the examples are part of the public model | N/A | reject README/package mismatches instead of documenting two clustered stories |
| stale `tiny-cluster-prefered` prior art | remove or rewrite the contradictory manifest surface so it stops competing with the canonical examples | N/A | do not leave an obsolete manifest example named "prefered" in-tree |

## Load Profile

- **Shared resources**: repo-owned example files, package test fixtures, and build outputs.
- **Per-operation cost**: package build + package tests for two tiny route-free apps.
- **10x breakpoint**: textual drift across the paired packages is the first failure mode; runtime load is negligible.

## Negative Tests

- **Malformed inputs**: package tests should fail if `clustered(work)`, `declared_work_runtime_name()`, or `[cluster]` reappear in the dogfood surfaces.
- **Error paths**: route-free builds/tests must stay green without any app-owned `/work`/`/status`/`/health` routes or helper-managed submit flows.
- **Boundary conditions**: both packages preserve `execute_declared_work`, visible `1 + 1`, route-free `main.mpl`, and runtime-owned continuity inspection text.

## Steps

1. Rewrite `tiny-cluster/work.mpl` and `cluster-proof/work.mpl` to the shared `@cluster` form while preserving the function name that keeps the runtime registration name stable.
2. Update both package READMEs and package tests so they assert the new source-first contract and still reject app-owned route/proof seams.
3. Clean up `tiny-cluster-prefered/mesh.toml` so it no longer teaches stale manifest clustering or contradictory count syntax.
4. Replay package build/test smoke on both packages so the dogfood surfaces stay truthful, route-free, and byte-level consistent where the harness expects it.

## Must-Haves

- [ ] `tiny-cluster/` and `cluster-proof/` both dogfood `@cluster` instead of `clustered(work)`.
- [ ] Package tests/readmes reject the old helper/manifest story and keep the route-free runtime-owned inspection contract.
- [ ] `tiny-cluster-prefered/` stops teaching obsolete manifest clustering as if it were preferred current practice.

## Inputs

- ``tiny-cluster/work.mpl``
- ``tiny-cluster/tests/work.test.mpl``
- ``tiny-cluster/README.md``
- ``cluster-proof/work.mpl``
- ``cluster-proof/tests/work.test.mpl``
- ``cluster-proof/README.md``
- ``tiny-cluster-prefered/mesh.toml``

## Expected Output

- ``tiny-cluster/work.mpl``
- ``tiny-cluster/tests/work.test.mpl``
- ``tiny-cluster/README.md``
- ``cluster-proof/work.mpl``
- ``cluster-proof/tests/work.test.mpl``
- ``cluster-proof/README.md``
- ``tiny-cluster-prefered/mesh.toml``

## Verification

cargo run -q -p meshc -- test tiny-cluster/tests && cargo run -q -p meshc -- build tiny-cluster && cargo run -q -p meshc -- test cluster-proof/tests && cargo run -q -p meshc -- build cluster-proof
