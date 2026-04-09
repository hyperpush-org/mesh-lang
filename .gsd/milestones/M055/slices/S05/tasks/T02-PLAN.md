---
estimated_steps: 4
estimated_files: 7
skills_used:
  - bash-scripting
  - debug-like-expert
---

# T02: Re-close the S01 wrapper and republish a fresh split-boundary bundle

Once the current-state doc is fixed, close the first real gate instead of skipping ahead. `scripts/verify-m055-s01.sh` is the authoritative narrow stop/go rail for the split boundary, and S03/S04 are downstream of it. Rerun the wrapper from a clean repo, inspect `.tmp/m055-s01/verify/phase-report.txt` first on failure, and only patch the exact source/helper/verifier surface the fresh failure localizes.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m055-s01.sh` phase chain | Stop at the first red phase and repair that exact source/verifier surface before rerunning. | Treat the timed-out child log as authoritative and do not skip the phase. | Treat missing `phase-report.txt`, empty logs, or a 0-test cargo filter as verifier drift. |
| packages / landing / local-docs child phases | Preserve the current narrow S01 scope; fix only the failing helper/build surface. | Read the per-phase log in `.tmp/m055-s01/verify/` before changing commands. | Treat stale copied artifacts or missing passed markers as real failures, not as acceptable partial success. |

## Load Profile

- **Shared resources**: `.tmp/m055-s01/verify/`, packages build output, landing build output, and the retained M046 cargo rail.
- **Per-operation cost**: one assembled shell verifier plus nested node/python/npm/cargo phases.
- **10x breakpoint**: repeated full wrapper reruns and stale artifact interpretation, not memory or CPU.

## Negative Tests

- **Malformed inputs**: missing or stale `.tmp/m055-s01/verify/phase-report.txt`, empty child log, or 0-test cargo output.
- **Error paths**: a fresh child phase fails after T01 and requires a minimal source/helper fix instead of a broader wrapper rewrite.
- **Boundary conditions**: the wrapper must succeed from repo root without relying on previous `.tmp/m055-s01/verify/` state.

## Steps

1. Rerun `bash scripts/verify-m055-s01.sh` from a clean repo and inspect `.tmp/m055-s01/verify/phase-report.txt` plus the failing child log first if it goes red.
2. Repair only the smallest truthful source/helper/verifier surface the fresh S01 failure identifies; do not broaden into S03/S04 work here.
3. Rerun until `.tmp/m055-s01/verify/status.txt` is `ok`, `current-phase.txt` is `complete`, and every S01 phase marker is `passed`.

## Must-Haves

- [ ] `bash scripts/verify-m055-s01.sh` passes from repo root with fresh `.tmp/m055-s01/verify/` artifacts.
- [ ] The passing bundle includes `status.txt`, `current-phase.txt`, `phase-report.txt`, and `full-contract.log`.
- [ ] Any new fix stays inside the narrow S01 boundary contract instead of pulling S03/S04 concerns earlier.

## Inputs

- `.gsd/PROJECT.md`
- `scripts/verify-m055-s01.sh`
- `scripts/lib/m034_public_surface_contract.py`
- `packages-website/package.json`
- `mesher/landing/package.json`

## Expected Output

- `scripts/verify-m055-s01.sh`
- `.tmp/m055-s01/verify/status.txt`
- `.tmp/m055-s01/verify/current-phase.txt`
- `.tmp/m055-s01/verify/phase-report.txt`
- `.tmp/m055-s01/verify/full-contract.log`

## Verification

bash scripts/verify-m055-s01.sh

## Observability Impact

- Signals added/changed: a green rerun must leave `status.txt=ok`, `current-phase.txt=complete`, and passed phase markers for every S01 phase.
- How a future agent inspects this: start with `.tmp/m055-s01/verify/phase-report.txt`, then open the failing child log named in that phase.
- Failure state exposed: missing phase markers, timed-out child logs, and 0-test cargo filters remain explicit instead of being silently ignored.
