---
estimated_steps: 4
estimated_files: 5
skills_used:
  - github-workflows
  - rust-best-practices
  - test
---

# T02: Add the canonical real-registry publish/install verifier and proof fixtures

**Slice:** S01 — Real registry publish/install proof
**Milestone:** M034

## Description

Once scoped installs compile locally, create the single repo-local proof surface that later CI work can call unchanged. The verifier must use one owner-scoped proof package name plus a unique version per run, isolate credentials under `.tmp/m034-s01/home`, publish to the real registry with `meshpkg --json`, and then recheck API, download, lockfile, consumer build, duplicate publish rejection, and public discoverability instead of trusting artifact builds or homepage counts.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Real registry API / R2 | Fail the active phase, persist status/body/logs under `.tmp/m034-s01/verify/`, and stop without retrying the same version. | Fail fast with the package/version coordinate recorded for operator review. | Treat any non-JSON or missing field as proof failure; do not infer success from partial output. |
| Packages website SSR | Only check it after publish/install truth passes; persist the exact URL/body snapshot on failure. | Allow one bounded wait/retry, then fail the visibility phase. | Treat missing package/version text as a visibility failure, not as a soft warning. |
| Local `meshpkg` / `meshc` binaries | Stop immediately and keep stdout/stderr in the phase log. | N/A | Reject non-JSON `meshpkg --json` output and stop the proof. |

## Load Profile

- **Shared resources**: immutable registry versions, download counters, real network calls, and the temporary credential/home directories under `.tmp/m034-s01/`.
- **Per-operation cost**: one real publish, multiple metadata/search/detail fetches, one tarball download/hash, one install, one consumer build, and one duplicate-publish check.
- **10x breakpoint**: version collisions and flaky network calls fail first, so the verifier must generate unique versions and keep phase logs for reruns instead of retrying blindly.

## Negative Tests

- **Malformed inputs**: missing `MESH_PUBLISH_OWNER`, missing `MESH_PUBLISH_TOKEN`, missing/invalid `MESH_PROOF_VERSION`, or templates that fail to render a quoted scoped dependency key.
- **Error paths**: duplicate publish must return 409, missing metadata/search/detail fields must fail the proof, and any SHA mismatch between publish metadata, downloaded tarball, and `mesh.lock` must stop the script.
- **Boundary conditions**: exact package detail URL, search endpoint visibility, and a consumer that installs from `mesh.toml` using a quoted scoped dependency key.

## Steps

1. Add durable proof-package and consumer fixture templates under `scripts/fixtures/` so the verifier creates reproducible temp workspaces instead of hand-writing files inline.
2. Implement `scripts/verify-m034-s01.sh` with phase helpers patterned after the existing M033 verifier style: isolated HOME/credentials, required envs `MESH_PUBLISH_OWNER` and `MESH_PUBLISH_TOKEN`, optional `MESH_PROOF_VERSION`, unique per-run version generation, JSON publish/install parsing, direct registry HTTP checks, tarball SHA validation, lockfile assertions, consumer build, and duplicate publish rejection.
3. Verify public visibility using the exact package detail/search surfaces rather than homepage package counts, and store every phase log/artifact under `.tmp/m034-s01/verify/` for later debugging.
4. Keep the verifier small and CI-ready: explicit env contract, deterministic temp paths, no secret echo, and no dependency on unreleased installer artifacts or workflow YAML. If `MESH_PUBLISH_TOKEN` is missing, collect it with `secure_env_collect` before running the script.

## Must-Haves

- [ ] `scripts/verify-m034-s01.sh` is the authoritative publish → metadata → download → install → lockfile → build proof surface for this slice.
- [ ] The verifier uses quoted scoped dependency keys in the consumer manifest and proves the installed dependency actually compiles.
- [ ] Duplicate publish rejection, exact package detail/search visibility, and SHA/lockfile truth are all checked in the same run.
- [ ] Every failure leaves phase-specific logs under `.tmp/m034-s01/verify/` without printing secrets.

## Verification

- `bash -n scripts/verify-m034-s01.sh`
- `bash scripts/verify-m034-s01.sh`

## Observability Impact

- Signals added/changed: phase-scoped verifier logs, saved JSON/API payloads, downloaded tarball/hash artifacts, and captured `mesh.lock`/build outputs.
- How a future agent inspects this: start with `.tmp/m034-s01/verify/`, then rerun `bash scripts/verify-m034-s01.sh`.
- Failure state exposed: the exact publish/install/visibility phase, endpoint, package/version, and stderr/stdout that broke.

## Inputs

- `compiler/meshc/tests/e2e_m034_s01.rs` — scoped-install regression from T01 that the live proof must agree with.
- `compiler/meshpkg/src/main.rs` — CLI flag and JSON contract reference.
- `compiler/meshpkg/src/publish.rs` — publish-side JSON/status behavior the script will parse.
- `compiler/meshpkg/src/install.rs` — install-side JSON/status behavior and lockfile expectations.
- `compiler/mesh-pkg/src/lockfile.rs` — authoritative lockfile schema to assert against.
- `scripts/verify-m033-s01.sh` — local pattern for phase helpers and debug-log layout.
- `packages-website/src/routes/packages/[...name]/+page.server.js` — exact package detail surface to recheck.
- `packages-website/src/routes/search/+page.server.js` — exact search surface to recheck.

## Expected Output

- `scripts/verify-m034-s01.sh` — canonical live proof script for publish/install truth.
- `scripts/fixtures/m034-s01-proof-package/mesh.toml.template` — owner/version-aware proof package template.
- `scripts/fixtures/m034-s01-proof-package/registry_proof.mpl` — proof package module used by the consumer build.
- `scripts/fixtures/m034-s01-consumer/mesh.toml.template` — quoted scoped dependency consumer template.
- `scripts/fixtures/m034-s01-consumer/main.mpl` — consumer build target that proves the installed package resolves.
