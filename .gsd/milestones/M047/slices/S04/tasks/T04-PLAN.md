---
estimated_steps: 4
estimated_files: 8
skills_used:
  - test
---

# T04: Rewire the shared route-free harness and historical e2e rails to the @cluster contract

Once the scaffold and package surfaces move, the old route-free harness and historical exact-string rails need to stop pinning the M046 wording. Update them so they keep proving runtime-name continuity and route-free bootstrap truth, but now assert the source-first contract instead of the old helper/marker text.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| shared `m046_route_free` harness | fail with retained artifacts that show which generated/package file drifted instead of masking the cutover behind generic assertions | preserve existing bounded waits and artifact retention so false timeouts stay debuggable | treat missing or malformed continuity/status payloads as proof drift, not as soft skips |
| historical e2e file-content assertions | fail closed when stale `clustered(work)`/helper text remains or when runtime-name continuity disappears | N/A | keep assertions specific enough to distinguish wording drift from runtime-name/runtime-surface drift |
| retained artifact shape | keep bundle pointers and copied package outputs stable so later verifiers can still localize failures | bounded waits stay the same; do not add slower or broader probes unnecessarily | malformed retained bundle pointers should stay explicit failures rather than being ignored |

## Load Profile

- **Shared resources**: temp workspaces, retained `.tmp` artifact trees, spawned route-free test processes, and CLI continuity/status polling.
- **Per-operation cost**: one scaffold generation/build smoke path plus several file-content and artifact-shape assertions across historical e2e targets.
- **10x breakpoint**: `.tmp` artifact churn and duplicated build smoke dominate before CPU does; preserve the existing shared helper rather than multiplying harnesses.

## Negative Tests

- **Malformed inputs**: stale generated files containing `clustered(work)` or the helper, malformed bundle pointers, and missing copied artifact directories.
- **Error paths**: continuity/status probes must still fail closed when runtime truth drifts, and historical tests must not silently downgrade to zero-test or no-op checks.
- **Boundary conditions**: runtime-name continuity stays `Work.execute_declared_work`, route-free `Node.start_from_env()` remains the only bootstrap path, and no historical rail starts teaching `HTTP.clustered(...)`.

## Steps

1. Update `compiler/meshc/tests/support/m046_route_free.rs` so generated/package file assertions expect the `@cluster` source-first work contract while preserving retained artifact behavior.
2. Rewrite the historical M045/M046 e2e file-content assertions to the new source-first wording, keeping runtime-name continuity and route-free bootstrap expectations intact.
3. Preserve the existing artifact and CLI continuity checks so failures still localize to scaffold/package/runtime truth instead of to generic text mismatches.
4. Replay the named historical e2e filters so the cutover proves itself on the same rails users and later slices already rely on.

## Must-Haves

- [ ] Shared route-free harness assertions now expect `@cluster`-based work sources and new README wording.
- [ ] Historical M045/M046 e2e rails still prove runtime-name continuity and route-free runtime truth without pinning the old helper/marker text.
- [ ] Retained artifact shapes and failure localization stay stable enough for later wrapper scripts to reuse.

## Inputs

- ``compiler/meshc/tests/support/m046_route_free.rs``
- ``compiler/meshc/tests/e2e_m045_s01.rs``
- ``compiler/meshc/tests/e2e_m045_s02.rs``
- ``compiler/meshc/tests/e2e_m045_s03.rs``
- ``compiler/meshc/tests/e2e_m046_s03.rs``
- ``compiler/meshc/tests/e2e_m046_s04.rs``
- ``compiler/meshc/tests/e2e_m046_s05.rs``
- ``compiler/meshc/tests/e2e_m046_s06.rs``

## Expected Output

- ``compiler/meshc/tests/support/m046_route_free.rs``
- ``compiler/meshc/tests/e2e_m045_s01.rs``
- ``compiler/meshc/tests/e2e_m045_s02.rs``
- ``compiler/meshc/tests/e2e_m045_s03.rs``
- ``compiler/meshc/tests/e2e_m046_s03.rs``
- ``compiler/meshc/tests/e2e_m046_s04.rs``
- ``compiler/meshc/tests/e2e_m046_s05.rs``
- ``compiler/meshc/tests/e2e_m046_s06.rs``

## Verification

cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture && cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture && cargo test -p meshc --test e2e_m045_s03 m045_s03_ -- --nocapture && cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture && cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture && cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture && cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture

## Observability Impact

- Signals added/changed: retained route-free artifacts and continuity/status assertions should now make `@cluster` contract drift explicit instead of failing on obsolete wording.
- How a future agent inspects this: historical e2e logs plus copied `.tmp` bundles from `m046_route_free`-backed tests.
- Failure state exposed: scaffold/package/runtime drift versus docs/verifier drift remains separable in retained artifacts.
