---
estimated_steps: 3
estimated_files: 8
skills_used:
  - test
  - vitepress
---

# T03: Replace the stale M044 closeout story with M045 S04 source, docs, and verifier rails

Make M045 the current public cleanup/closeout story so the repo teaches the scaffold-first clustered path first and treats `cluster-proof` as the deeper proof consumer instead of the primary clustered abstraction.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Source/docs contract tests and public pages | Fail with exact file-path assertions; do not leave the repo pointing at `verify-m044-s05.sh` as the current clustered closeout rail. | N/A — these are local source assertions. | Treat missing or contradictory marker text as a contract failure, not documentation drift to ignore. |
| Assembled verifier replay plus retained proof-bundle handoff | Stop on the first red prerequisite and keep per-phase logs plus copied bundle pointers. | Bound every replayed command and fail with the captured phase log instead of hanging. | Reject zero-test filters, malformed pointer files, or missing retained bundle shape as verifier drift. |

## Load Profile

- **Shared resources**: docs pages, focused Rust e2e files, `.tmp/m045-s03/verify`, and the new `.tmp/m045-s04/verify` artifact root.
- **Per-operation cost**: one source/docs contract test, one assembled verifier replay, and one docs build.
- **10x breakpoint**: stale references and artifact-copy drift fail before throughput; the verifier must make freshness and bundle shape explicit.

## Negative Tests

- **Malformed inputs**: stale `verify-m044-s05.sh` references in public docs, zero-test `m045_s04_` filters, and malformed bundle pointers or copied manifests.
- **Error paths**: M045 S02/S03 rails go red during replay, docs still mention old example-owned mechanics, or the copied S03 proof bundle is missing required files.
- **Boundary conditions**: the repo may contain multiple old `.tmp/m045-s03` directories, but the new verifier must retain only the fresh prerequisite bundle it actually replayed.

## Steps

1. Add `compiler/meshc/tests/e2e_m045_s04.rs` to assert the new source/docs/verifier contract: the cleaned `cluster-proof` target shape is real, public pages point at `scripts/verify-m045-s04.sh`, and `scripts/verify-m045-s03.sh` is the failover-specific subrail.
2. Add `scripts/verify-m045-s04.sh` as the assembled cleanup verifier: replay the M045 scaffold happy-path and failover rails, run the new source/docs contract test plus `cluster-proof` build/tests and the docs build, and retain the fresh upstream proof bundle it depends on.
3. Update `README.md`, `cluster-proof/README.md`, the distributed docs pages, and the old M044 closeout test so M045 now owns the current public story while M044 stops policing present-tense docs wording.

## Must-Haves

- [ ] Public clustered docs/readmes point at M045 rails, not `verify-m044-s05.sh`, as the current closeout story.
- [ ] There is a named M045 source/docs/verifier contract test and assembled verifier for S04.
- [ ] Historical M044 closeout assertions no longer fail only because the primary story moved to M045.

## Inputs

- `compiler/meshc/tests/e2e_m045_s02.rs`
- `compiler/meshc/tests/e2e_m045_s03.rs`
- `compiler/meshc/tests/e2e_m044_s05.rs`
- `scripts/verify-m045-s02.sh`
- `scripts/verify-m045-s03.sh`
- `README.md`
- `cluster-proof/README.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/tooling/index.md`

## Expected Output

- `compiler/meshc/tests/e2e_m045_s04.rs`
- `compiler/meshc/tests/e2e_m044_s05.rs`
- `scripts/verify-m045-s04.sh`
- `README.md`
- `cluster-proof/README.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/tooling/index.md`

## Verification

cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture
bash scripts/verify-m045-s04.sh

## Observability Impact

- Signals added/changed: `.tmp/m045-s04/verify/{status.txt,current-phase.txt,phase-report.txt,full-contract.log}` plus a retained pointer to the fresh prerequisite failover bundle.
- How a future agent inspects this: rerun `bash scripts/verify-m045-s04.sh`, inspect the per-phase logs, then follow the retained bundle pointer back into the copied S03 evidence.
- Failure state exposed: stale public references, zero-test drift, missing bundle freshness, and docs-build failures become phase-specific instead of a generic closeout failure.
