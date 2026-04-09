---
estimated_steps: 3
estimated_files: 8
skills_used:
  - test
  - vitepress
---

# T02: Promote S05 to the current docs/proof contract and keep S04 replayable

Promote S05 to the current public closeout contract and make the docs prove that the new tutorial is the first stop while deeper proof rails stay secondary. This task should reuse the existing S04 assembled rail instead of cloning product proof logic: S05 wraps S04, adds docs/source contract checks, and makes S04 historical/replayable rather than present-tense.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Surrounding docs/readmes and the old S04 contract test | Fail with exact file-path assertions; do not leave public surfaces pointing at S04 as the current rail after S05 lands. | N/A — these are local source checks. | Treat contradictory or missing marker text as contract drift, not as optional documentation cleanup. |
| `scripts/verify-m045-s04.sh` replay and retained artifact handoff | Stop on the first red prerequisite, keep the copied S04 logs/bundle pointers, and do not reimplement S02/S03/S04 proof logic inside S05. | Bound every replayed command and fail with the captured phase log instead of hanging. | Reject zero-test filters, malformed pointer files, or missing copied bundle shape as verifier drift. |

## Load Profile

- **Shared resources**: `README.md`, `cluster-proof/README.md`, `website/docs/docs/tooling/index.md`, `website/docs/docs/distributed/index.md`, `website/docs/docs/distributed-proof/index.md`, `compiler/meshc/tests/e2e_m045_s04.rs`, `.tmp/m045-s04/verify`, and the new `.tmp/m045-s05/verify` artifact root.
- **Per-operation cost**: one targeted Rust source/docs contract test, one assembled verifier replay, and one docs build.
- **10x breakpoint**: stale link/current-rail string drift and malformed retained-bundle pointers fail long before throughput; the wrapper must make freshness and phase ownership explicit.

## Negative Tests

- **Malformed inputs**: stale `verify-m045-s04.sh` present-tense references in docs/readmes, missing `/docs/getting-started/clustered-example/` links, and zero-test `m045_s05_` filter output.
- **Error paths**: S04 replay goes red, copied S04 bundle pointers are malformed, or the new docs page exists but the surrounding docs still send readers straight to proof pages.
- **Boundary conditions**: multiple historical `.tmp/m045-s04` directories may exist, but S05 must retain only the fresh S04 verify output it just replayed.

## Steps

1. Update `README.md`, `cluster-proof/README.md`, `website/docs/docs/tooling/index.md`, `website/docs/docs/distributed/index.md`, and `website/docs/docs/distributed-proof/index.md` so they route readers to `/docs/getting-started/clustered-example/` first, describe `cluster-proof` as the deeper proof consumer, and name `bash scripts/verify-m045-s05.sh` as the current closeout rail.
2. Add `compiler/meshc/tests/e2e_m045_s05.rs` plus the necessary `compiler/meshc/tests/e2e_m045_s04.rs` adjustments so S05 owns the present-tense docs/proof contract while S04 stays a replayable subrail checker.
3. Add `scripts/verify-m045-s05.sh` as the final wrapper verifier: replay `bash scripts/verify-m045-s04.sh`, run the new S05 contract test, run the docs build, and retain the fresh S04 verify artifacts/pointers instead of duplicating product proof logic.

## Must-Haves

- [ ] Public docs/readmes route clustered readers to `/docs/getting-started/clustered-example/` before deeper proof material.
- [ ] `bash scripts/verify-m045-s05.sh` is the current present-tense closeout rail and it reuses `bash scripts/verify-m045-s04.sh` rather than inventing a docs-only proof path.
- [ ] `compiler/meshc/tests/e2e_m045_s05.rs` fail-closes on missing page/sidebar/current-rail markers, while `compiler/meshc/tests/e2e_m045_s04.rs` remains green as a historical/replayable contract.

## Inputs

- `website/docs/docs/getting-started/clustered-example/index.md`
- `README.md`
- `cluster-proof/README.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `compiler/meshc/tests/e2e_m045_s04.rs`
- `scripts/verify-m045-s04.sh`

## Expected Output

- `README.md`
- `cluster-proof/README.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `compiler/meshc/tests/e2e_m045_s04.rs`
- `compiler/meshc/tests/e2e_m045_s05.rs`
- `scripts/verify-m045-s05.sh`

## Verification

cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture
bash scripts/verify-m045-s05.sh

## Observability Impact

- Signals added/changed: `.tmp/m045-s05/verify/{status.txt,current-phase.txt,phase-report.txt,full-contract.log}` plus copied S04 verify artifacts and retained failover bundle pointers.
- How a future agent inspects this: rerun `bash scripts/verify-m045-s05.sh`, inspect the per-phase logs, then follow the copied S04 pointers into the retained prerequisite evidence.
- Failure state exposed: stale current-rail wording, zero-test drift, malformed bundle pointers, and docs-build failures become phase-specific instead of a generic closeout failure.
