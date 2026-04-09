---
estimated_steps: 23
estimated_files: 6
skills_used: []
---

# T02: Confirmed that the route-free scaffold/example cutover is blocked because the current compiler still rejects zero-ceremony `@cluster` work before any scaffold or package rebaseline can land.

Once the public contract changes, the existing route-free clustered surfaces cannot keep teaching `execute_declared_work(request_key, attempt_id)`. Rebaseline the default scaffold, repo-owned examples, and historical route-free exact-string rails on the no-ceremony model so later Todo work extends the corrected surface instead of preserving the old one in parallel.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| route-free scaffold/example source parity | fail unit/tooling/package rails instead of letting scaffold, `tiny-cluster/`, and `cluster-proof/` drift apart | N/A | reject stale `execute_declared_work` / helper wording instead of treating it as a compatible alias |
| historical route-free e2e harness | preserve retained artifacts and runtime-owned CLI inspection while changing the source contract | bounded by existing harness timeouts | malformed bundle pointers or exact-string expectations should fail closed |
| README/runbook wording | keep continuity metadata described as runtime-owned inspection state, not public function arguments | N/A | reject contradictory docs that teach both models at once |

## Load Profile

- **Shared resources**: temp scaffold projects, route-free package build outputs, retained `.tmp` bundles, and exact-string docs/tests.
- **Per-operation cost**: one scaffold generation plus route-free package/test replays; no heavy external services.
- **10x breakpoint**: textual contract drift across scaffold/packages/tests is the first failure mode, not runtime throughput.

## Negative Tests

- **Malformed inputs**: stale generated/package sources containing `execute_declared_work`, `request_key`, `attempt_id`, `clustered(work)`, or manifest clustering text.
- **Error paths**: historical route-free rails must still fail explicitly on missing retained artifacts or malformed continuity output after the contract change.
- **Boundary conditions**: the default scaffold, `tiny-cluster/`, and `cluster-proof/` all dogfood the same no-ceremony `@cluster` source contract while keeping runtime-owned `meshc cluster` inspection.

## Steps

1. Rewrite `meshc init --clustered`, `tiny-cluster/`, and `cluster-proof/` to dogfood ordinary no-ceremony `@cluster` functions with generic runtime names instead of scaffold-owned `execute_declared_work(...)` ceremony.
2. Update shared route-free harnesses and historical exact-string rails so they assert the new contract while preserving runtime-owned CLI continuity/diagnostic inspection and retained artifact behavior.
3. Refresh package/tooling/readme assertions so the route-free public story stays coherent before the Todo template is introduced.

## Must-Haves

- [ ] The default route-free clustered scaffold and repo-owned example packages all use the same no-ceremony `@cluster` contract.
- [ ] Historical route-free rails still localize failures honestly after the source contract changes.
- [ ] Route-free docs/tests keep continuity metadata as runtime-owned inspection truth rather than public function parameters.

## Inputs

- ``compiler/mesh-pkg/src/scaffold.rs``
- ``compiler/meshc/tests/tooling_e2e.rs``
- ``tiny-cluster/work.mpl``
- ``cluster-proof/work.mpl``
- ``compiler/meshc/tests/support/m046_route_free.rs``
- ``compiler/meshc/tests/e2e_m046_s05.rs``

## Expected Output

- ``compiler/mesh-pkg/src/scaffold.rs``
- ``compiler/meshc/tests/tooling_e2e.rs``
- ``tiny-cluster/work.mpl``
- ``cluster-proof/work.mpl``
- ``compiler/meshc/tests/support/m046_route_free.rs``
- ``compiler/meshc/tests/e2e_m046_s05.rs``

## Verification

cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture && cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture && cargo run -q -p meshc -- test tiny-cluster/tests && cargo run -q -p meshc -- test cluster-proof/tests && cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture
