---
estimated_steps: 4
estimated_files: 4
skills_used:
  - best-practices
  - flyio-cli-public
---

# T01: Build the one-image `cluster-proof` artifact and Fly config

**Slice:** S04 — One-Image Operator Path, Local/Fly Verifiers, and Docs Truth
**Milestone:** M039

## Description

Add the packaging seam that S04 depends on: one repo-root Docker build for `cluster-proof`, a runtime entrypoint that supplies local hostname-based identity defaults only when appropriate, and a Fly config that matches the same image/operator contract.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| repo-root Docker build and Mesh compiler outputs | stop the task, keep the build log, and treat missing runtime artifacts as a packaging failure | fail the task with the last build output rather than retrying blindly | reject partial builds or missing binaries instead of papering over them with fallback shell logic |
| runtime env contract in the entrypoint | exit before starting the binary and surface the missing/invalid contract clearly | N/A — contract resolution is synchronous | reject conflicting identity inputs and preserve the existing `cluster-proof` validation semantics |
| Fly app config | fail the task on mismatched ports, auto-stop settings, or wrong build path assumptions | N/A — config validation is static | reject docs/config drift that implies `cd cluster-proof && fly deploy` works when the build still needs repo-root context |

## Load Profile

- **Shared resources**: Docker layer cache, Cargo build artifacts, and the runtime image filesystem.
- **Per-operation cost**: one repo-root image build plus short container-entrypoint smoke checks.
- **10x breakpoint**: build time and image size grow before CPU; the task should keep a small runtime stage and avoid copying the whole repo into the final image.

## Negative Tests

- **Malformed inputs**: missing cookie, missing seed, incomplete explicit identity, and partial Fly identity must still fail with clear config errors.
- **Error paths**: repo-root build-context mistakes, missing runtime binary in the final image, or conflicting identity precedence must fail before a misleading startup.
- **Boundary conditions**: local hostname fallback should only apply when neither explicit local identity nor Fly identity is set.

## Steps

1. Add `cluster-proof/Dockerfile` as a repo-root build-context image that compiles the `cluster-proof` package and copies only the runtime artifact plus the entrypoint into the final stage.
2. Add `cluster-proof/docker-entrypoint.sh` so local containers default `CLUSTER_PROOF_NODE_BASENAME` and `CLUSTER_PROOF_ADVERTISE_HOST` from `HOSTNAME` only when explicit local identity and Fly identity are both absent, then `exec` the packaged binary.
3. Add `cluster-proof/fly.toml` for the same image, pin the HTTP port, and keep machines awake so `.internal` DNS can return running peers truthfully.
4. Adjust root-level ignore/build-path details only if the repo-root Docker build needs them, without widening the app/runtime config surface.

## Must-Haves

- [ ] `docker build -f cluster-proof/Dockerfile .` works from repo root and produces a runtime image for `cluster-proof`.
- [ ] The image wrapper keeps the current `cluster-proof` config contract intact while supplying local container hostname defaults when no other identity source exists.
- [ ] `cluster-proof/fly.toml` matches the one-image contract and assumes repo-root `fly deploy . --config cluster-proof/fly.toml --dockerfile cluster-proof/Dockerfile` usage rather than a fake subdirectory build context.

## Verification

- `docker build -f cluster-proof/Dockerfile -t mesh-cluster-proof .`
- `docker run --rm --entrypoint /bin/sh mesh-cluster-proof -lc 'test -x /usr/local/bin/cluster-proof && test -x /usr/local/bin/docker-entrypoint.sh'`

## Observability Impact

- Signals added/changed: container startup now has an entrypoint-owned contract boundary before the existing `[cluster-proof] Config loaded ...` logs.
- How a future agent inspects this: rebuild the image, run a short entrypoint smoke check, and inspect `docker logs` plus the packaged binary path inside the container.
- Failure state exposed: missing runtime artifact, conflicting identity inputs, or wrong Fly/repo-root build assumptions become visible before the HTTP server starts.

## Inputs

- `cluster-proof/config.mpl` — existing operator env contract and identity rules.
- `cluster-proof/main.mpl` — current startup path and runtime log shape.
- `registry/Dockerfile` — repo-local Rust multi-stage image pattern.
- `registry/fly.toml` — repo-local Fly config pattern.
- `.dockerignore` — current repo-root build-context exclusions.

## Expected Output

- `cluster-proof/Dockerfile` — repo-root image build for `cluster-proof`.
- `cluster-proof/docker-entrypoint.sh` — local hostname-defaulting wrapper that preserves explicit/Fly identity precedence.
- `cluster-proof/fly.toml` — one-image Fly runtime config with always-running machines.
- `.dockerignore` — any minimal ignore adjustment needed for the repo-root build.
