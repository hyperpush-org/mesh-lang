---
estimated_steps: 4
estimated_files: 4
skills_used:
  - test
  - best-practices
---

# T02: Assemble the local one-image continuity verifier

**Slice:** S04 — One-Image Operator Path, Local/Fly Verifiers, and Docs Truth
**Milestone:** M039

## Description

Add the authoritative local S04 acceptance surface so the one-image operator story is proven against real containers, not inferred from the earlier process-level harnesses.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m039-s03.sh` prerequisite replay | stop immediately and refuse to claim S04 passed on a broken local runtime contract | fail the phase with the copied prerequisite log and current-phase marker | treat missing phase reports or zero-test evidence as verifier drift, not as a flaky prerequisite |
| Docker network/DNS alias preflight | fail closed with the container/network inspection output and preserve the partial artifact dir | tear down containers, preserve the preflight logs, and mark the phase failed | reject seed-resolution evidence that does not show both containers behind the shared alias |
| container `/membership` and `/work` probes | preserve raw JSON bodies and container logs, then fail the phase | kill the bounded wait, keep the partial evidence, and fail with the last raw body or curl error | reject missing routing/membership fields instead of synthesizing cluster truth |

## Load Profile

- **Shared resources**: two running containers, one user-defined Docker bridge network, copied container stdout/stderr logs, and `.tmp/m039-s04/verify/` artifact directories.
- **Per-operation cost**: one prerequisite verifier replay, one image build, repeated membership polls, and three `/work` probes across pre-loss, degraded, and post-rejoin phases.
- **10x breakpoint**: wall-clock time and artifact sprawl before CPU; the wrapper must bound waits and clean up containers deterministically.

## Negative Tests

- **Malformed inputs**: missing membership keys, missing `request_id` / `execution_node`, or partial DNS preflight output must fail the wrapper with the raw artifact preserved.
- **Error paths**: bad shared seed, bad cookie, container crash, or missing logs must fail the phase instead of leaving hanging containers behind.
- **Boundary conditions**: two-node convergence, truthful self-only degrade, same-identity restart, and restored remote routing from the same image.

## Steps

1. Add a small helper library if needed for repeated JSON assertions, container cleanup, or artifact copying so the local and Fly verifiers can share the same contract checks.
2. Implement `scripts/verify-m039-s04.sh` to replay `scripts/verify-m039-s03.sh`, build the image from repo root, create one Docker bridge network, start two containers from the same image with one shared discovery alias/seed and one shared cookie, and preflight that the shared alias resolves to both containers before trusting the cluster proof.
3. Probe `/membership` and `/work` to prove two-node membership, remote routing, truthful degrade after stopping one container, truthful local fallback during degrade, same-identity restart, and restored remote routing after rejoin; archive JSON bodies, manifests, and container logs under `.tmp/m039-s04/verify/`.
4. Fail closed on zero-test prerequisite drift, DNS preflight drift, malformed JSON, or missing copied logs, and finish only when `bash scripts/verify-m039-s04.sh` passes from a clean run.

## Must-Haves

- [ ] The local verifier uses the same image for both containers and proves clustering from only a shared cookie plus shared discovery seed — no manual peer lists and no per-node bootstrap logic.
- [ ] `.tmp/m039-s04/verify/` preserves phase-by-phase logs, manifests, membership/work JSON, and copied container stdout/stderr for pre-loss, degraded, and post-rejoin debugging.
- [ ] A passing wrapper proves the full local chain: S03 replay, repo-root image build, two-container convergence, remote `/work`, degrade/local fallback, same-identity restart, and restored remote `/work`.

## Verification

- `bash scripts/verify-m039-s04.sh`

## Observability Impact

- Signals added/changed: `.tmp/m039-s04/verify/{status.txt,current-phase.txt,phase-report.txt}`, DNS preflight output, membership/work JSON, and copied container logs.
- How a future agent inspects this: rerun `bash scripts/verify-m039-s04.sh` or open the latest `.tmp/m039-s04/verify/` bundle.
- Failure state exposed: whether drift happened in the prerequisite replay, image build, seed-resolution preflight, convergence, degraded fallback, or rejoin recovery phase.

## Inputs

- `scripts/verify-m039-s03.sh` — authoritative local continuity prerequisite from S03.
- `cluster-proof/Dockerfile` — packaged image contract from T01.
- `cluster-proof/docker-entrypoint.sh` — hostname-defaulting local entrypoint from T01.
- `cluster-proof/work.mpl` — existing `/work` contract the verifier must keep proving.
- `compiler/meshc/tests/e2e_m039_s03.rs` — current degrade/rejoin contract and artifact expectations.

## Expected Output

- `scripts/verify-m039-s04.sh` — canonical local Docker continuity wrapper.
- `scripts/lib/m039_cluster_proof.sh` — shared JSON/assertion/artifact helpers used by local and Fly verifiers.
