---
estimated_steps: 3
estimated_files: 2
skills_used:
  - test
  - debug-like-expert
---

# T03: Add the two-node scaffold proof rail and fail-closed verifier

**Slice:** S02 — Tiny End-to-End Clustered Example
**Milestone:** M045

## Description

Finish the slice with the public proof surface: init a clustered scaffold project, run two local nodes, let the runtime choose a remote owner via honest request-key retries, assert runtime-owned continuity truth on both nodes, and package that flow into an assembled verifier that replays upstream rails fail-closed.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Two-node scaffold runtime and CLI inspection surfaces in `compiler/meshc/tests/e2e_m045_s02.rs` | Stop with retained HTTP, CLI, and node-log artifacts; do not infer success from startup logs alone. | Bound membership/remote-owner/completion polling and fail with the last observed payload. | Treat malformed `meshc cluster status` / `meshc cluster continuity` JSON as a hard proof failure. |
| Assembled verifier in `scripts/verify-m045-s02.sh` | Fail closed on zero-test, stale-artifact, or upstream-rail drift. | Preserve per-phase logs and copied evidence bundles instead of hanging. | Reject malformed manifests, missing copied artifacts, or pointer drift rather than claiming green. |

## Load Profile

- **Shared resources**: temporary scaffold dirs, dual-stack ports, spawned node processes, CLI subprocesses, and `.tmp/m045-s02` artifact roots.
- **Per-operation cost**: one scaffold init/build, two node boots, repeated CLI/HTTP polls, and one verifier replay.
- **10x breakpoint**: port/process cleanup and proof-artifact churn fail before runtime throughput; retained evidence must make the failure legible.

## Negative Tests

- **Malformed inputs**: zero-test filter drift, malformed continuity/status JSON, and missing copied artifact directories.
- **Error paths**: remote owner never chosen, continuity remains pending, or ingress/owner disagree about completion truth.
- **Boundary conditions**: two-node convergence on loopback IPv4/IPv6, remote-owner retry selection without local placement reimplementation, and duplicate submit stability after completion.

## Steps

1. Build out `compiler/meshc/tests/e2e_m045_s02.rs` so it creates a scaffolded project, runs two nodes, retries request keys until the runtime chooses a remote owner, and then asserts completion truth through runtime inspection surfaces.
2. Retain the resulting CLI/HTTP/node-log artifacts under `.tmp/m045-s02/...` so later slices can compare runtime state without rerunning the cluster immediately.
3. Add `scripts/verify-m045-s02.sh` as the slice stopping condition; replay `scripts/verify-m045-s01.sh`, the relevant declared-handler rail, the scaffold init contract, and the new S02 e2e while failing closed on zero-test or artifact drift.

## Must-Haves

- [ ] The new two-node proof trusts runtime-chosen remote ownership instead of reimplementing placement locally.
- [ ] Both ingress and owner nodes report the same completed continuity truth for the remote-owner request.
- [ ] The assembled verifier is authoritative and preserves copied evidence for later debugging.

## Verification

- `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture`
- `bash scripts/verify-m045-s02.sh`

## Observability Impact

- Signals added/changed: retained scaffold node stdout/stderr, copied `.tmp/m045-s02/verify` manifests, and direct CLI status/continuity artifacts.
- How a future agent inspects this: `bash scripts/verify-m045-s02.sh` and the copied artifact bundle under `.tmp/m045-s02/verify`.
- Failure state exposed: remote-owner selection starvation, pending continuity, malformed CLI output, and zero-test drift are all captured as separate verifier failures.

## Inputs

- `compiler/meshc/tests/e2e_m045_s02.rs` — scaffold proof harness extended in T01/T02.
- `scripts/verify-m045-s01.sh` — upstream bootstrap acceptance rail to replay first.
- `scripts/verify-m044-s02.sh` — declared-handler acceptance rail to replay before the scaffold-first proof.
- `compiler/meshc/tests/e2e_m042_s01.rs` — honest remote-owner selection pattern to reuse.
- `compiler/meshc/tests/e2e_m044_s03.rs` — existing scaffold runtime/process helpers to mirror.

## Expected Output

- `compiler/meshc/tests/e2e_m045_s02.rs` — authoritative two-node scaffold proof with retained artifacts.
- `scripts/verify-m045-s02.sh` — fail-closed assembled verifier for the slice.
