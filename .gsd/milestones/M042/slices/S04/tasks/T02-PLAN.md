---
estimated_steps: 26
estimated_files: 8
skills_used: []
---

# T02: Added the M042 packaged operator verifier scripts and read-only Fly help rail, but authoritative local replay is still blocked by inherited M039/M042 verifier regressions.

Close the operator proof rail without rewriting M039 history.

## Why

S03 already proved the runtime-owned owner-loss contract locally, but the packaged operator rail still only proves the older M039 routing story. This task should add M042 wrappers that reuse the same one-image Docker/Fly path, replay the stable local destructive continuity authority, and keep the Fly lane read-only.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Repo-root Docker image and two-container local proof scripts | Fail the phase that drifted and archive the build/container logs; do not silently fall back to a partial proof. | Preserve phase reports and copied artifacts instead of hanging on container readiness or continuity polling. | Fail closed on malformed `/membership`, `GET /work`, `POST /work`, or `GET /work/:request_key` JSON. |
| Read-only Fly contract helpers | Keep help/syntax and validation local and fail before any live call when required inputs are absent. | Read-only live checks should time out with a named phase and artifact root. | Reject malformed config/status/probe payloads instead of weakening the Fly contract. |

## Load Profile

- **Shared resources**: Docker image cache, local container ports/network, `.tmp/m039-s04/...` and `.tmp/m042-s04/...` artifact roots, and any repo-local `cluster-proof` build outputs.
- **Per-operation cost**: one repo-root image build plus a small number of HTTP probes and artifact copies; the Fly wrapper should remain lightweight unless explicitly run live.
- **10x breakpoint**: container cleanup, phase-artifact clarity, and accidental script overlap fail before raw throughput does.

## Negative Tests

- **Malformed inputs**: missing Fly env, malformed base URLs, malformed keyed continuity JSON, and zero-test/empty-artifact regressions.
- **Error paths**: packaged cluster degrade/rejoin drift, keyed submit/status mismatch after the Docker packaging path, and stale help/live-contract wording that implies mutating Fly verification.
- **Boundary conditions**: repo-root Docker context, two-node convergence, legacy routing still remote under health, keyed continuity status remains truthful after packaged submit, and `--help` remains the safe non-live Fly path.

## Steps

1. Reuse or extend the existing `scripts/lib/m039_cluster_proof.sh` helpers so M042 can assert packaged keyed submit/status payloads without duplicating the baseline membership/work checks.
2. Add `scripts/verify-m042-s04.sh` as the authoritative local packaged operator wrapper: replay the stable S03 local continuity rail, build the repo-root image, stand up the same one-image two-container runtime, and archive `/membership`, `GET /work`, `POST /work`, and `GET /work/:request_key` proof artifacts.
3. Add `scripts/verify-m042-s04-fly.sh` as a read-only wrapper/help contract that keeps the live Fly lane honest about what it can inspect without mutating remote state.
4. Keep `scripts/verify-m039-s04*.sh` replayable as the validated baseline instead of overwriting their historical scope.

## Must-Haves

- [ ] The packaged local wrapper proves runtime-owned keyed continuity through the one-image Docker path without claiming exactly-once semantics.
- [ ] The Fly lane remains read-only and explicitly scoped as sanity/config/log/probe truth, not destructive recovery authority.
- [ ] M039 baseline verifiers still run as the prior validated rail.
- [ ] The new M042 artifact root makes the first failing phase obvious from logs and JSON alone.

## Inputs

- ``scripts/lib/m039_cluster_proof.sh``
- ``scripts/verify-m039-s04.sh``
- ``scripts/verify-m039-s04-fly.sh``
- ``scripts/verify-m042-s03.sh``
- ``cluster-proof/Dockerfile``
- ``cluster-proof/fly.toml``
- ``cluster-proof/README.md``

## Expected Output

- ``scripts/lib/m039_cluster_proof.sh``
- ``scripts/lib/m042_cluster_proof.sh``
- ``scripts/verify-m042-s04.sh``
- ``scripts/verify-m042-s04-fly.sh``

## Verification

bash scripts/verify-m039-s04.sh && bash scripts/verify-m042-s04.sh && bash scripts/verify-m042-s04-fly.sh --help

## Observability Impact

Adds the canonical `.tmp/m042-s04/verify/` packaged-proof bundle and keeps the Fly wrapper artifact-driven, so drift localizes to Docker build, local continuity proof, read-only Fly validation, or copied probe payloads.
