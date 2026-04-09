---
estimated_steps: 24
estimated_files: 7
skills_used: []
---

# T01: Relocate `tiny-cluster` into a shared clustered-fixture root

Move the minimal `tiny-cluster` proof package out of the repo root into a stable internal fixture location under `scripts/fixtures/clustered/tiny-cluster` and teach the shared route-free helper plus the tiny-cluster-specific e2e rail to resolve the new package path without changing package identity, runtime names, or log prefixes.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/meshc/tests/support/m046_route_free.rs` path helper | Fail closed on the first missing fixture path and surface the new expected fixture location. | N/A — local path resolution only. | Reject partial fixture roots or wrong package directories instead of silently falling back to `tiny-cluster/`. |
| `meshc build` / `meshc test` against the moved fixture | Stop before any root-directory deletion and archive the failing path/command. | Keep the failing command and temp artifact directory so the move can be replayed. | Treat missing `mesh.toml`, `main.mpl`, `work.mpl`, or `tests/work.test.mpl` as fixture drift. |
| `compiler/meshc/tests/e2e_m046_s03.rs` retained startup/failover rail | Fail before the slice claims the tiny-cluster proof survived relocation. | Keep `.tmp/m046-s03/...` evidence and named test output for diagnosis. | Treat stale repo-root path reads or changed runtime/log names as contract drift. |

## Load Profile

- **Shared resources**: local `meshc` builds, `.tmp/m046-s03` retained artifacts, and the shared route-free helper used by later clustered rails.
- **Per-operation cost**: one fixture build, one fixture test run, and the retained `e2e_m046_s03` target.
- **10x breakpoint**: compile/build time and artifact churn grow before CPU or memory; the task should not introduce duplicate fixture trees or fallback path searches.

## Negative Tests

- **Malformed inputs**: missing fixture root files, stale `repo_root().join("tiny-cluster")` reads, or partial package moves.
- **Error paths**: `meshc build` / `meshc test` still pointed at the deleted root package, or helper functions silently resolving the wrong directory.
- **Boundary conditions**: runtime names, package name, README wording, and failover logs must stay `tiny-cluster` even though the package path moved.

## Steps

1. Move `tiny-cluster` into `scripts/fixtures/clustered/tiny-cluster/` without renaming its package, runtime, or log identities.
2. Extend `compiler/meshc/tests/support/m046_route_free.rs` with stable clustered-fixture path helpers so later rails stop open-coding repo-root package paths.
3. Retarget `compiler/meshc/tests/e2e_m046_s03.rs` to the new helper/path while keeping its source-contract, package build/test, startup, and failover assertions intact.
4. Prove the moved fixture through direct `meshc build` / `meshc test` plus the named retained Rust rail before any root-directory removal happens.

## Must-Haves

- [ ] `tiny-cluster` exists only under `scripts/fixtures/clustered/tiny-cluster/` as an internal fixture location.
- [ ] The shared route-free helper exposes a stable tiny-cluster fixture path for later Rust and bash consumers.
- [ ] The retained `e2e_m046_s03` rail and direct `meshc build` / `meshc test` commands stay green against the moved fixture with unchanged runtime-facing names.

## Inputs

- ``tiny-cluster/mesh.toml` — current route-free package manifest to relocate.`
- ``tiny-cluster/main.mpl` — current runtime-owned bootstrap source that must keep its package/log identity.`
- ``tiny-cluster/work.mpl` — current minimal `@cluster` workload that must stay route-free.`
- ``tiny-cluster/tests/work.test.mpl` — current package smoke contract that should move intact.`
- ``tiny-cluster/README.md` — current internal runbook content that must survive as a lower-level fixture readme, not a public entrypoint.`
- ``compiler/meshc/tests/support/m046_route_free.rs` — shared route-free helper seam that should own the new fixture path.`
- ``compiler/meshc/tests/e2e_m046_s03.rs` — retained tiny-cluster startup/failover rail that currently hardcodes the repo-root package path.`

## Expected Output

- ``scripts/fixtures/clustered/tiny-cluster/mesh.toml` — relocated internal fixture manifest.`
- ``scripts/fixtures/clustered/tiny-cluster/main.mpl` — relocated bootstrap source with unchanged package/log identity.`
- ``scripts/fixtures/clustered/tiny-cluster/work.mpl` — relocated minimal clustered work source.`
- ``scripts/fixtures/clustered/tiny-cluster/tests/work.test.mpl` — relocated fixture smoke test.`
- ``scripts/fixtures/clustered/tiny-cluster/README.md` — relocated lower-level fixture runbook.`
- ``compiler/meshc/tests/support/m046_route_free.rs` — shared helper updated with stable tiny-cluster fixture resolution.`
- ``compiler/meshc/tests/e2e_m046_s03.rs` — retained Rust rail pointed at the new fixture path.`

## Verification

- `cargo run -q -p meshc -- test scripts/fixtures/clustered/tiny-cluster/tests`
- `cargo run -q -p meshc -- build scripts/fixtures/clustered/tiny-cluster`
- `cargo test -p meshc --test e2e_m046_s03 -- --nocapture`

## Observability Impact

- Signals added/changed: route-free helper errors and retained `e2e_m046_s03` artifacts should record the moved fixture path explicitly.
- How a future agent inspects this: rerun the direct `meshc build` / `meshc test` commands above or inspect `.tmp/m046-s03/...` from the Rust rail.
- Failure state exposed: stale root-path reads and partial fixture moves fail with named path assertions instead of a later missing-directory panic.
