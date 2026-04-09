---
estimated_steps: 4
estimated_files: 8
skills_used:
  - multi-stage-dockerfile
  - flyio-cli-public
  - test
---

# T02: Lock the route-free package contract in smoke tests, README, and honest packaging files

**Slice:** S04 — Rebuild `cluster-proof/` as tiny packaged proof
**Milestone:** M046

## Description

Lock the new package shape in smoke tests/readme and make Docker/Fly honest about a route-free binary by removing the HTTP entrypoint story instead of preserving fake probes.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof/tests/work.test.mpl` package smoke rail | Fail fast on source/readme/packaging drift instead of punting the first truthful failure to the Rust e2e rail. | N/A | Treat unexpected route, delay, or proxy strings as a contract failure. |
| `cluster-proof/Dockerfile` runtime image | Fail `docker build` if the runtime stage still depends on a deleted entrypoint or HTTP-only env. | Docker build timeout should fail the task instead of keeping an unverified packaging story. | Treat missing built binary or wrong copied paths as image-contract failures. |
| `cluster-proof/fly.toml` packaged deployment contract | Fail closed if `http_service`, `PORT`, or proxy-only assumptions remain. | N/A | Treat malformed or contradictory process config as packaging drift rather than patching it in docs. |

## Load Profile

- **Shared resources**: One packaged binary, one Docker image build, and package-level file-content guards.
- **Per-operation cost**: `meshc test cluster-proof/tests` plus one multi-stage Docker build of the repo image.
- **10x breakpoint**: Image bloat or proxy misconfiguration will break the proof before runtime compute cost matters.

## Negative Tests

- **Malformed inputs**: Packaging/readme drift that reintroduces `/membership`, `/work`, `CLUSTER_PROOF_WORK_DELAY_MS`, `PORT`, `http_service`, or `docker-entrypoint.sh`.
- **Error paths**: `cluster-proof/tests/work.test.mpl` must fail if README or packaging files point operators back to old route/Fly HTTP behavior.
- **Boundary conditions**: The packaged story may stay deeper/provisional, but it must still be route-free and honest about having no app-owned HTTP surface.

## Steps

1. Rewrite `cluster-proof/tests/work.test.mpl` to mirror the `tiny-cluster/` smoke style while adding route-free guards for `README.md`, `Dockerfile`, and `fly.toml`.
2. Delete `cluster-proof/tests/config.test.mpl` and move any remaining contract coverage into `work.test.mpl` so package proof lives on the new tiny surface only.
3. Rewrite `cluster-proof/README.md` around the packaged route-free contract: `clustered(work)`, `Node.start_from_env()`, `meshc cluster status|continuity|diagnostics`, and explicit rejection of app-owned routes/timing seams.
4. Simplify `cluster-proof/Dockerfile` and `cluster-proof/fly.toml` so the image copies only the built binary, drops `docker-entrypoint.sh`, and removes fake HTTP proxy requirements from the packaged contract.

## Must-Haves

- [ ] `cluster-proof/tests/work.test.mpl` fails closed on reintroduced routes, delay knobs, or fake Fly HTTP packaging.
- [ ] `cluster-proof/tests/config.test.mpl` is deleted instead of preserved for removed modules.
- [ ] `cluster-proof/README.md` points operators at `meshc cluster status|continuity|diagnostics` and does not mention `/membership`, `/work`, `mesh-cluster-proof.fly.dev`, or `CLUSTER_PROOF_WORK_DELAY_MS`.
- [ ] `cluster-proof/Dockerfile` no longer copies or executes `cluster-proof/docker-entrypoint.sh`, and `cluster-proof/fly.toml` no longer declares `http_service` or `PORT`.

## Verification

- `cargo run -q -p meshc -- test cluster-proof/tests && docker build -f cluster-proof/Dockerfile -t mesh-cluster-proof:m046-s04-local .`

## Observability Impact

- Signals added/changed: package smoke failures point at exact file drift, and `docker build` becomes the authoritative packaging failure surface.
- How a future agent inspects this: rerun `meshc test cluster-proof/tests`, inspect the readme/package assertions, then inspect the Docker build log.
- Failure state exposed: stale route strings, deleted-entrypoint dependencies, or fake Fly proxy config remain visible as direct file/test/build failures.

## Inputs

- `cluster-proof/tests/work.test.mpl` — legacy package tests that still assume app-owned continuity surfaces.
- `cluster-proof/tests/config.test.mpl` — obsolete config-helper tests that should disappear with the deleted modules.
- `cluster-proof/README.md` — current same-image/Fly HTTP runbook that must be rewritten to a route-free packaged story.
- `cluster-proof/Dockerfile` — existing multi-stage image that still copies an entrypoint and exposes HTTP assumptions.
- `cluster-proof/docker-entrypoint.sh` — shell bootstrap wrapper that duplicates runtime-owned validation and should be removed.
- `cluster-proof/fly.toml` — current Fly config that still declares `http_service` and `PORT`.
- `tiny-cluster/tests/work.test.mpl` — smoke-test reference for route-free package guards.
- `tiny-cluster/README.md` — route-free package-doc reference for runtime-owned CLI inspection wording.

## Expected Output

- `cluster-proof/tests/work.test.mpl` — rebuilt package smoke rail covering source/readme/packaging honesty.
- `cluster-proof/tests/config.test.mpl` — deleted obsolete config test file.
- `cluster-proof/README.md` — route-free packaged runbook with no app-owned route/timing guidance.
- `cluster-proof/Dockerfile` — simplified builder/runtime image that only copies the built binary.
- `cluster-proof/docker-entrypoint.sh` — deleted runtime-duplicating entrypoint wrapper.
- `cluster-proof/fly.toml` — route-free process config with no `http_service` or `PORT` assumption.
