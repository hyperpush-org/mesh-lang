---
estimated_steps: 24
estimated_files: 5
skills_used: []
---

# T04: Retarget retained Rust contract rails to the internal clustered fixtures

Once the packages and public copy are moved, the retained Rust e2e rails still need to resolve lower-level fixtures instead of the old root dirs. Update the contract, equal-surface, and historical clustered tests to read from the new shared helper paths and assert the new public story rather than repo-root onboarding runbooks.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Rust e2e targets that still read repo-root proof packages | Fail compilation or assertions before the slice claims root-package retirement is safe. | Keep named Cargo target logs and retained `.tmp/...` bundles for diagnosis. | Reject stale root-path strings or old public-story assertions as contract drift. |
| Shared helper paths in `m046_route_free` | Fail closed if a consumer still bypasses the helper or resolves the wrong fixture. | N/A — local path resolution only. | Treat helper/path mismatches as structural drift rather than auto-falling back. |
| Updated public-story assertions in `e2e_m047_s04.rs` | Fail before the authoritative contract target claims the new onboarding story is real. | Preserve the target log and contract snapshots if it times out. | Reject tests that still require `tiny-cluster/README.md` or `cluster-proof/README.md` as public onboarding surfaces. |

## Load Profile

- **Shared resources**: Cargo integration target compilation, retained `.tmp` artifact bundles, and the shared route-free helper module.
- **Per-operation cost**: several targeted `meshc` integration test targets with snapshot/artifact capture.
- **10x breakpoint**: compile time dominates; the task should reuse the helper and avoid duplicating fixture-path logic across tests.

## Negative Tests

- **Malformed inputs**: stale root-path joins, old scenario metadata naming deleted root packages as public surfaces, or outdated README assertions.
- **Error paths**: `e2e_m047_s04` still demanding root-package onboarding links, or historical rails still referencing the deleted root dirs.
- **Boundary conditions**: internal fixtures may stay named `tiny-cluster` / `cluster-proof`, but public-story assertions must point at scaffold/examples first.

## Steps

1. Retarget the main Rust consumers of the old root package paths (`compiler/meshc/tests/e2e_m045_s01.rs`, `compiler/meshc/tests/e2e_m045_s02.rs`, `compiler/meshc/tests/e2e_m046_s05.rs`, and `compiler/meshc/tests/e2e_m047_s04.rs`) to the shared fixture helpers or the new onboarding contract expectations.
2. Update retained source/readme assertions so internal fixtures remain valid proof surfaces without requiring repo-root onboarding runbooks.
3. Keep scenario metadata and retained artifact labels clear enough that later slices can still localize failures after the path move.
4. Verify the updated Rust rails fail closed on stale root-path assumptions and stay green on the moved fixtures.

## Must-Haves

- [ ] Retained Rust contract/equal-surface/history rails stop reading the deleted repo-root proof packages directly.
- [ ] `e2e_m047_s04` asserts the new public onboarding story rather than the old equal-surface runbook story.
- [ ] Historical Rust rails preserve useful retained artifact names after the path move.

## Inputs

- ``compiler/meshc/tests/support/m046_route_free.rs` — shared helper that should now own both clustered fixture roots.`
- ``compiler/meshc/tests/e2e_m045_s01.rs` — retained cluster-proof package contract rail with repo-root assumptions.`
- ``compiler/meshc/tests/e2e_m045_s02.rs` — retained scaffold-vs-cluster-proof parity rail that should now consume the moved fixture.`
- ``compiler/meshc/tests/e2e_m046_s05.rs` — equal-surface rail that still snapshots `tiny-cluster` and `cluster-proof` work files from the repo root.`
- ``compiler/meshc/tests/e2e_m047_s04.rs` — authoritative cutover contract target that still snapshots root-package readmes and old public-story markers.`

## Expected Output

- ``compiler/meshc/tests/support/m046_route_free.rs` — shared helper finalized with stable clustered fixture roots for Rust consumers.`
- ``compiler/meshc/tests/e2e_m045_s01.rs` — retained package contract rail pointed at the moved cluster-proof fixture.`
- ``compiler/meshc/tests/e2e_m045_s02.rs` — retained scaffold parity rail aligned with the moved cluster-proof fixture.`
- ``compiler/meshc/tests/e2e_m046_s05.rs` — equal-surface rail aligned with the moved internal fixtures.`
- ``compiler/meshc/tests/e2e_m047_s04.rs` — authoritative contract target updated for the new public onboarding story and fixture layout.`

## Verification

- `cargo test -p meshc --test e2e_m045_s01 -- --nocapture`
- `cargo test -p meshc --test e2e_m045_s02 -- --nocapture`
- `cargo test -p meshc --test e2e_m046_s05 -- --nocapture`
- `cargo test -p meshc --test e2e_m047_s04 -- --nocapture`

## Observability Impact

- Signals added/changed: retained Rust targets and their `.tmp/...` bundles should snapshot the moved fixture paths and updated public-story contract markers.
- How a future agent inspects this: rerun the named Cargo targets above and inspect the corresponding retained artifact directories.
- Failure state exposed: stale root-path joins and old public-story assertions fail in named Rust targets instead of later shell-script missing-path errors.
