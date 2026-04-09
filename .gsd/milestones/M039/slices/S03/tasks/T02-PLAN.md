---
estimated_steps: 4
estimated_files: 4
skills_used:
  - test
  - debug-like-expert
---

# T02: Assemble the fail-closed S03 verifier and evidence bundle

**Slice:** S03 — Single-Cluster Failure, Safe Degrade, and Rejoin
**Milestone:** M039

## Description

Add the canonical local S03 acceptance wrapper so the distributed continuity story replays from known-good prerequisites and fails closed with a stable evidence bundle that later agents can inspect without rerunning a long multi-node proof.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m039-s01.sh` / `scripts/verify-m039-s02.sh` prerequisite replays | stop immediately, copy the failing phase log, and refuse to claim S03 passed on a broken base contract | fail the phase with the prerequisite log excerpt and current-phase marker | treat missing phase-report or zero-test evidence as verifier drift, not as a flaky prerequisite |
| named `e2e_m039_s03` cargo filters and copied artifact dirs | fail closed with the exact cargo log and missing artifact path | kill the command after the bounded wait, preserve the partial `.tmp/m039-s03` tree, and mark the phase failed | reject zero-test filters, missing phase manifests, or malformed copied JSON/log bundles |

## Load Profile

- **Shared resources**: multiple cargo invocations, copied `.tmp/m039-s03` directories, and per-phase log files.
- **Per-operation cost**: one test pass, one build, two prerequisite verifier replays, two named S03 test invocations, and artifact-copy validation.
- **10x breakpoint**: wall-clock time and artifact sprawl before CPU; the wrapper must bound timeouts and only copy the new S03 phase directories it created.

## Negative Tests

- **Malformed inputs**: missing `phase-report.txt`, `status.txt`, `current-phase.txt`, or per-phase artifact files must fail the wrapper.
- **Error paths**: a named cargo filter that runs 0 tests, prerequisite verifier drift, or missing copied node logs must mark the phase failed.
- **Boundary conditions**: separate pre-loss/degraded/post-rejoin artifact groups, repeated reruns against an existing `.tmp/m039-s03/verify`, and phase-specific timeout handling.

## Steps

1. Add `scripts/verify-m039-s03.sh` in the S02 wrapper pattern with `status.txt`, `current-phase.txt`, `phase-report.txt`, bounded `cargo` timeouts, and full-contract logging under `.tmp/m039-s03/verify/`.
2. Replay `cluster-proof/tests`, `meshc build cluster-proof`, `scripts/verify-m039-s01.sh`, and `scripts/verify-m039-s02.sh` before any new S03 checks so continuity proof cannot hide earlier regression.
3. Run the two named `e2e_m039_s03` filters with non-zero test-count checks and copy stable pre-loss/degraded/post-rejoin artifacts plus per-incarnation logs/manifests into `.tmp/m039-s03/verify/`.
4. Fail closed on missing prerequisite phase reports, zero-test filters, missing artifacts, or malformed copied evidence, and finish only when `bash scripts/verify-m039-s03.sh` succeeds from a clean run.

## Must-Haves

- [ ] `scripts/verify-m039-s03.sh` is the authoritative local S03 replay surface and refuses false-green runs when a named filter drifts or a prerequisite contract is broken.
- [ ] `.tmp/m039-s03/verify/` preserves phase-by-phase logs, manifests, and copied per-incarnation node evidence for degrade and rejoin debugging.
- [ ] A passing wrapper proves the full local chain: proof app tests, build, S01, S02, and both S03 continuity filters.

## Verification

- `bash scripts/verify-m039-s03.sh`

## Observability Impact

- Signals added/changed: `.tmp/m039-s03/verify/{status.txt,current-phase.txt,phase-report.txt}` plus copied per-phase manifests and node logs.
- How a future agent inspects this: rerun or open `bash scripts/verify-m039-s03.sh` artifacts under `.tmp/m039-s03/verify/`.
- Failure state exposed: the exact failing prerequisite/S03 phase, the command log that failed, and the missing or malformed artifact path.

## Inputs

- `scripts/verify-m039-s01.sh` — prerequisite membership verifier contract.
- `scripts/verify-m039-s02.sh` — prerequisite routing verifier contract and artifact-copy pattern.
- `compiler/meshc/tests/e2e_m039_s03.rs` — named S03 tests and their artifact layout.

## Expected Output

- `scripts/verify-m039-s03.sh` — canonical local continuity replay wrapper with stable evidence copying.
