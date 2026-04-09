---
id: T01
parent: S04
milestone: M039
provides: []
requires: []
affects: []
key_files: ["cluster-proof/Dockerfile", "cluster-proof/docker-entrypoint.sh", "cluster-proof/fly.toml", ".dockerignore", ".gsd/milestones/M039/slices/S04/tasks/T01-SUMMARY.md"]
key_decisions: ["Only synthesize `CLUSTER_PROOF_NODE_BASENAME` and `CLUSTER_PROOF_ADVERTISE_HOST` from `HOSTNAME` when cluster mode is being attempted and neither explicit local identity nor Fly identity is present, so standalone containers do not trip cluster-only validation.", "Keep the runtime image thin by compiling in a builder stage and copying only the packaged Mesh binary plus the entrypoint into the final image."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed `cargo run -q -p meshc -- build cluster-proof` and `sh -n cluster-proof/docker-entrypoint.sh`. Attempted the task’s real artifact gate with `docker build --progress=plain -f cluster-proof/Dockerfile -t mesh-cluster-proof .`, but the build timed out after 900 seconds while still loading metadata for the base images, so the follow-on `docker run --rm --entrypoint /bin/sh mesh-cluster-proof -lc 'test -x /usr/local/bin/cluster-proof && test -x /usr/local/bin/docker-entrypoint.sh'` smoke check could not run. `docker image inspect mesh-cluster-proof` confirmed the image was not present afterward."
completed_at: 2026-03-28T13:22:32.241Z
blocker_discovered: false
---

# T01: Added the repo-root `cluster-proof` Dockerfile, entrypoint contract guard, and Fly config, but Docker verification is still blocked by a host-side base-image metadata timeout.

> Added the repo-root `cluster-proof` Dockerfile, entrypoint contract guard, and Fly config, but Docker verification is still blocked by a host-side base-image metadata timeout.

## What Happened
---
id: T01
parent: S04
milestone: M039
key_files:
  - cluster-proof/Dockerfile
  - cluster-proof/docker-entrypoint.sh
  - cluster-proof/fly.toml
  - .dockerignore
  - .gsd/milestones/M039/slices/S04/tasks/T01-SUMMARY.md
key_decisions:
  - Only synthesize `CLUSTER_PROOF_NODE_BASENAME` and `CLUSTER_PROOF_ADVERTISE_HOST` from `HOSTNAME` when cluster mode is being attempted and neither explicit local identity nor Fly identity is present, so standalone containers do not trip cluster-only validation.
  - Keep the runtime image thin by compiling in a builder stage and copying only the packaged Mesh binary plus the entrypoint into the final image.
duration: ""
verification_result: mixed
completed_at: 2026-03-28T13:22:32.242Z
blocker_discovered: false
---

# T01: Added the repo-root `cluster-proof` Dockerfile, entrypoint contract guard, and Fly config, but Docker verification is still blocked by a host-side base-image metadata timeout.

**Added the repo-root `cluster-proof` Dockerfile, entrypoint contract guard, and Fly config, but Docker verification is still blocked by a host-side base-image metadata timeout.**

## What Happened

Added `cluster-proof/Dockerfile` as a repo-root build-context image path that installs the repo’s LLVM 21 toolchain, builds `mesh-rt` plus `meshc`, packages `cluster-proof`, and copies only the packaged binary into a thin runtime stage. Added `cluster-proof/docker-entrypoint.sh` as an entrypoint-owned contract boundary that only derives local identity from `HOSTNAME` when cluster mode is actually being attempted and no explicit local or Fly identity is present, while failing early with clear `cluster-proof` config-error messages for missing cookie, missing discovery seed, partial explicit identity, partial Fly identity, or missing identity source. Added `cluster-proof/fly.toml` for the same one-image path with repo-root Dockerfile wiring, fixed ports, and always-on HTTP service settings, and tightened `.dockerignore` so `.gsd/`, `.tmp/`, and built Mesh binaries do not bloat the repo-root Docker context. Local Mesh compilation and entrypoint syntax checks passed, but authoritative Docker verification did not complete on this host because Docker Desktop stalled while resolving base-image metadata for `rust:1-bookworm` / `debian:bookworm-slim` from Docker Hub before the build steps executed.

## Verification

Passed `cargo run -q -p meshc -- build cluster-proof` and `sh -n cluster-proof/docker-entrypoint.sh`. Attempted the task’s real artifact gate with `docker build --progress=plain -f cluster-proof/Dockerfile -t mesh-cluster-proof .`, but the build timed out after 900 seconds while still loading metadata for the base images, so the follow-on `docker run --rm --entrypoint /bin/sh mesh-cluster-proof -lc 'test -x /usr/local/bin/cluster-proof && test -x /usr/local/bin/docker-entrypoint.sh'` smoke check could not run. `docker image inspect mesh-cluster-proof` confirmed the image was not present afterward.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- build cluster-proof` | 0 | ✅ pass | 0ms |
| 2 | `sh -n cluster-proof/docker-entrypoint.sh` | 0 | ✅ pass | 0ms |
| 3 | `docker build --progress=plain -f cluster-proof/Dockerfile -t mesh-cluster-proof .` | 124 | ❌ fail | 900000ms |
| 4 | `docker image inspect mesh-cluster-proof >/dev/null 2>&1; echo $?` | 1 | ❌ fail | 0ms |


## Deviations

Did not reach the planned Docker-run smoke check because Docker never completed base-image metadata resolution on this host. Stopped at durable recovery instead of continuing to wait on the daemon with no additional signal.

## Known Issues

Authoritative task verification is still incomplete: `mesh-cluster-proof` has not been built locally yet, so the Docker-run smoke gate remains pending behind the Docker metadata timeout.

## Files Created/Modified

- `cluster-proof/Dockerfile`
- `cluster-proof/docker-entrypoint.sh`
- `cluster-proof/fly.toml`
- `.dockerignore`
- `.gsd/milestones/M039/slices/S04/tasks/T01-SUMMARY.md`


## Deviations
Did not reach the planned Docker-run smoke check because Docker never completed base-image metadata resolution on this host. Stopped at durable recovery instead of continuing to wait on the daemon with no additional signal.

## Known Issues
Authoritative task verification is still incomplete: `mesh-cluster-proof` has not been built locally yet, so the Docker-run smoke gate remains pending behind the Docker metadata timeout.
