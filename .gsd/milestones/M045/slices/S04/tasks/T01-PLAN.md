---
estimated_steps: 3
estimated_files: 3
skills_used:
  - test
  - rust-best-practices
---

# T01: Delete dead placement residue and retarget package tests to live cluster-proof seams

Remove the dead deterministic placement engine from `cluster-proof/cluster.mpl` and make the package tests prove only the live membership/config seams that still matter for the proof package.

## Steps

1. Delete `CanonicalPlacement` and the unused placement/canonical-owner helpers from `cluster-proof/cluster.mpl` while keeping `canonical_membership(...)`, `membership_snapshot()`, and `membership_payload(...)` behavior stable.
2. Rewrite `cluster-proof/tests/work.test.mpl` and `cluster-proof/tests/config.test.mpl` so they assert current package truths (membership payload authority fields, durability/topology validation, request parsing/status payloads) instead of helper-shaped or dead-placement behavior.
3. Re-run the package build/tests and fail closed if membership payload shape or topology validation drifts.

## Must-Haves

- [ ] No deterministic owner/replica placement helpers remain in `cluster-proof/cluster.mpl`.
- [ ] Package tests still prove membership payload truth and continuity topology validation through live public seams.
- [ ] `cluster-proof` still builds and its package tests stay green after the delete.

## Inputs

- `cluster-proof/cluster.mpl`
- `cluster-proof/tests/work.test.mpl`
- `cluster-proof/tests/config.test.mpl`

## Expected Output

- `cluster-proof/cluster.mpl`
- `cluster-proof/tests/work.test.mpl`
- `cluster-proof/tests/config.test.mpl`

## Verification

cargo run -q -p meshc -- build cluster-proof
cargo run -q -p meshc -- test cluster-proof/tests
