---
estimated_steps: 4
estimated_files: 5
skills_used:
  - bash-scripting
  - test
---

# T04: Add the assembled S01 verifier and preserve the repo-local `.gsd` regression seam

**Slice:** S01 — Two-Repo Boundary & GSD Authority Contract
**Milestone:** M055

## Description

Finish the slice with one named verifier that proves the workspace contract, repo-identity contract, and existing repo-local `.gsd` dependency still hold together. This task should leave the next slice with one obvious stop/go command and one obvious failure-inspection path.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m055-s01.sh` child phases | stop on the first failing contract, build, or cargo phase and record it in `phase-report.txt` | mark the timed-out phase explicitly and fail closed | treat missing phase markers, missing logs, or skipped child checks as verifier drift |
| `compiler/meshc/tests/e2e_m046_s03.rs` named cargo rail | fail closed if the repo-local `.gsd` contract no longer matches what the shipped tiny-cluster test expects | use a bounded cargo invocation and preserve the raw log | treat `running 0 test` or missing `S03-PLAN.md` assertions as a real regression |
| `WORKSPACE.md` / `CONTRIBUTING.md` verifier docs | keep the named top-level verifier command discoverable from maintainer-facing docs | N/A for source assertions | treat undocumented verifier entrypoints as workflow drift |

## Load Profile

- **Shared resources**: `.tmp/m055-s01/verify/`, the packages-site build cache, the landing typecheck, and the named M046 cargo target.
- **Per-operation cost**: one shell wrapper, one cargo test, one packages build, one landing typecheck, and one Node contract replay.
- **10x breakpoint**: repeated frontend builds and the cargo regression check dominate before the shell wrapper logic does.

## Negative Tests

- **Malformed inputs**: stale monorepo-path wording, missing repo-identity markers, or a wrapper that forgets to run the repo-local `.gsd` cargo regression.
- **Error paths**: the direct Node contract passes, but the assembled wrapper omits phase markers or hides which child command failed.
- **Boundary conditions**: the final S01 command proves only the slice contract and the existing `.gsd` regression seam; it should not grow into the later S02 toolchain or S03 public-surface assembly story.

## Steps

1. Add `scripts/verify-m055-s01.sh` as the authoritative S01 replay that creates `.tmp/m055-s01/verify/`, records `status.txt`, `current-phase.txt`, `phase-report.txt`, and `full-contract.log`, then runs the Node contract, installer/docs contract, packages build, landing typecheck, and the named M046 cargo regression in order.
2. Make the wrapper fail closed on missing child logs or missing test-count evidence, and keep the first failing phase obvious without requiring a broad manual grep.
3. Update `WORKSPACE.md` and `CONTRIBUTING.md` to name `bash scripts/verify-m055-s01.sh` as the top-level split-boundary verifier.
4. Record the debug entrypoint in `.gsd/KNOWLEDGE.md` so future agents know to inspect `.tmp/m055-s01/verify/phase-report.txt` first and use the named M046 cargo rail when the repo-local `.gsd` seam drifts.

## Must-Haves

- [ ] `scripts/verify-m055-s01.sh` is the named top-level verifier for this slice and writes the standard phase markers under `.tmp/m055-s01/verify/`.
- [ ] The wrapper replays the named repo-local `.gsd` regression target `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_contract_remains_source_first_and_route_free -- --nocapture`.
- [ ] `WORKSPACE.md` and `CONTRIBUTING.md` both point at the new verifier command.
- [ ] `.gsd/KNOWLEDGE.md` tells future agents where to start when the split-boundary verifier goes red.

## Verification

- `bash scripts/verify-m055-s01.sh`

## Observability Impact

- Signals added/changed: `.tmp/m055-s01/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, and per-phase logs for workspace, repo-identity, packages, landing, and GSD-regression checks.
- How a future agent inspects this: run `bash scripts/verify-m055-s01.sh`, then read `.tmp/m055-s01/verify/phase-report.txt` and the failing phase log before rerunning the child command directly.
- Failure state exposed: the first failing phase, exact drifting file/marker, and the raw cargo output from the repo-local `.gsd` regression seam.

## Inputs

- `scripts/tests/verify-m055-s01-contract.test.mjs` — slice-owned Node contract from T01-T03
- `scripts/lib/m034_public_surface_contract.py` — installer/docs local-contract helper from T02
- `packages-website/src/routes/+layout.svelte` — language-owned public-surface identity consumer from T03
- `mesher/landing/lib/external-links.ts` — product-owned public-surface identity consumer from T03
- `compiler/meshc/tests/e2e_m046_s03.rs` — named repo-local `.gsd` regression seam
- `WORKSPACE.md` — maintainer-facing split contract from T01
- `CONTRIBUTING.md` — contributor-facing workflow doc to point at the new verifier

## Expected Output

- `scripts/verify-m055-s01.sh` — authoritative assembled S01 verifier with retained phase logs
- `WORKSPACE.md` — workspace contract updated with the named verifier command
- `CONTRIBUTING.md` — contributor workflow updated with the named verifier command
- `.gsd/KNOWLEDGE.md` — enduring debug guidance for the S01 boundary verifier
- `scripts/tests/verify-m055-s01-contract.test.mjs` — final slice-owned contract rail consumed by the wrapper
