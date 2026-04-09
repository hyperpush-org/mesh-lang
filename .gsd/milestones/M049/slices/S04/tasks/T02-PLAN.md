---
estimated_steps: 24
estimated_files: 9
skills_used: []
---

# T02: Relocate `cluster-proof` and retarget its package-build consumers

Move `cluster-proof` into `scripts/fixtures/clustered/cluster-proof`, preserve its package identity plus Docker/Fly packaging contract, and retarget the main package-build/test rails plus the Todo Linux builder dependency so retained cluster-proof consumers point at the new internal fixture instead of the repo root.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof` package files (`README.md`, `Dockerfile`, `fly.toml`) | Stop before any root-directory deletion and keep the original package content restorable. | N/A — local file move only. | Treat missing package files or changed package/log identity as fixture drift. |
| `compiler/meshc/tests/e2e_m046_s04.rs` and `compiler/meshc/tests/e2e_m045_s02.rs` | Fail the retained package/scaffold rails before the slice claims cluster-proof survived relocation. | Preserve `.tmp/m046-s04/...` or `.tmp/m045-s02/...` artifacts for diagnosis. | Reject stale repo-root references or wrong Docker/Fly package expectations. |
| `compiler/meshc/tests/support/m047_todo_scaffold.rs` Linux-output builder seam | Fail the Todo builder phase rather than silently building from a dead Dockerfile path. | Preserve the Docker build log and image-inspect log if the builder image cannot be rebuilt. | Treat missing `Dockerfile` or wrong fixture path as a hard error. |

## Load Profile

- **Shared resources**: Docker builder image cache, route-free fixture builds/tests, retained `.tmp/m046-s04` and `.tmp/m045-s02` bundles, and the shared route-free helper.
- **Per-operation cost**: one moved package build/test, one retained route-free Rust rail, and one scaffold/Todo helper replay.
- **10x breakpoint**: Docker rebuild time and route-free package compilation show up before CPU or memory; the task should not duplicate cluster-proof assets or keep two competing builder paths alive.

## Negative Tests

- **Malformed inputs**: stale `repo_root().join("cluster-proof")` reads, missing `Dockerfile` / `fly.toml`, or missing fixture tests.
- **Error paths**: Todo helper still building `cluster-proof/Dockerfile`, or route-free e2e rails still asserting the deleted root package.
- **Boundary conditions**: package name, runtime/log prefixes, Docker entrypoint text, and Fly discovery seed must stay `cluster-proof`-shaped even though the path moved.

## Steps

1. Move `cluster-proof` into `scripts/fixtures/clustered/cluster-proof/` without renaming its package, binary, runtime, Docker, or Fly identities.
2. Retarget `compiler/meshc/tests/e2e_m046_s04.rs` and `compiler/meshc/tests/e2e_m045_s02.rs` so their package/scaffold assertions read the moved fixture path.
3. Update `compiler/meshc/tests/support/m047_todo_scaffold.rs` to build its Linux output-helper image from the moved fixture Dockerfile path.
4. Prove the moved fixture through direct `meshc build` / `meshc test` plus the retained package/scaffold Rust rails before any root-directory removal happens.

## Must-Haves

- [ ] `cluster-proof` exists only under `scripts/fixtures/clustered/cluster-proof/` as an internal proof fixture.
- [ ] Retained package/scaffold consumers stop reading `repo_root()/cluster-proof` directly.
- [ ] The Todo Linux-output builder seam resolves the moved `cluster-proof` Dockerfile successfully.

## Inputs

- ``cluster-proof/mesh.toml` — current route-free package manifest to relocate.`
- ``cluster-proof/main.mpl` — current runtime-owned bootstrap source that must keep its `cluster-proof` identity.`
- ``cluster-proof/work.mpl` — current minimal clustered workload that must stay route-free.`
- ``cluster-proof/README.md` — current lower-level proof runbook that should move intact.`
- ``cluster-proof/Dockerfile` — current Linux builder/runtime image definition reused by the Todo helper.`
- ``cluster-proof/fly.toml` — current packaged read-only Fly config contract to preserve.`
- ``compiler/meshc/tests/e2e_m046_s04.rs` — retained cluster-proof package/startup rail with hardcoded root-path assumptions.`
- ``compiler/meshc/tests/support/m047_todo_scaffold.rs` — Todo helper that currently builds from `cluster-proof/Dockerfile`.`
- ``compiler/meshc/tests/e2e_m045_s02.rs` — scaffold-vs-cluster-proof parity rail that should consume the moved fixture.`

## Expected Output

- ``scripts/fixtures/clustered/cluster-proof/mesh.toml` — relocated internal fixture manifest.`
- ``scripts/fixtures/clustered/cluster-proof/main.mpl` — relocated route-free bootstrap source.`
- ``scripts/fixtures/clustered/cluster-proof/work.mpl` — relocated clustered work source.`
- ``scripts/fixtures/clustered/cluster-proof/README.md` — relocated lower-level proof runbook.`
- ``scripts/fixtures/clustered/cluster-proof/Dockerfile` — relocated builder/runtime image contract.`
- ``scripts/fixtures/clustered/cluster-proof/fly.toml` — relocated packaged Fly contract.`
- ``compiler/meshc/tests/e2e_m046_s04.rs` — retained package/startup rail pointed at the new fixture path.`
- ``compiler/meshc/tests/support/m047_todo_scaffold.rs` — Todo helper updated to build from the moved fixture Dockerfile.`
- ``compiler/meshc/tests/e2e_m045_s02.rs` — scaffold parity rail updated for the moved cluster-proof fixture.`

## Verification

- `cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests`
- `cargo run -q -p meshc -- build scripts/fixtures/clustered/cluster-proof`
- `cargo test -p meshc --test e2e_m046_s04 -- --nocapture`
- `cargo test -p meshc --test e2e_m045_s02 -- --nocapture`

## Observability Impact

- Signals added/changed: route-free package/test/build logs and Todo Docker builder logs must name the new fixture path instead of the deleted repo-root directory.
- How a future agent inspects this: rerun the direct `meshc build` / `meshc test` commands, inspect `.tmp/m046-s04/...`, `.tmp/m045-s02/...`, or the Todo helper Docker logs.
- Failure state exposed: stale Dockerfile paths, wrong package expectations, or moved-file omissions fail in named Rust rails rather than surfacing later as opaque Todo image build drift.
