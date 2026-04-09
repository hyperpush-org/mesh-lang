---
estimated_steps: 5
estimated_files: 4
skills_used: []
---

# T06: Add the fail-closed S01 acceptance rail around the recovered typed seam and proof-app rewrite

Finish the slice only after one authoritative verifier replays the restored contract in order.

1. Add `scripts/verify-m044-s01.sh` as the single repo-root acceptance command that runs the named manifest/parser/compiler/LSP rails, the typed continuity compiler e2e, the `cluster-proof` build/test replay, and the stale-continuity-shim absence check in order.
2. Make the wrapper fail closed on missing `running N test` evidence or 0-test filters, reset `.tmp/m044-s01/verify/` phase state per run, and preserve per-phase logs/status markers for postmortem inspection.
3. Align verifier-targeted test prefixes and coverage in `compiler/meshc/tests/e2e_m044_s01.rs`, `compiler/mesh-lsp/src/analysis.rs`, and `compiler/mesh-pkg/src/manifest.rs` so the wrapper depends on stable named rails instead of broad ad hoc filters.
4. Treat `bash scripts/verify-m044-s01.sh` as the slice’s stopping condition and keep the artifact bundle specific enough to distinguish manifest-validation drift, typed ABI regressions, and `cluster-proof` consumer drift.

## Inputs

- `compiler/mesh-pkg/src/manifest.rs`
- `compiler/mesh-lsp/src/analysis.rs`
- `compiler/meshc/tests/e2e_m044_s01.rs`
- `cluster-proof/work_continuity.mpl`

## Expected Output

- ``scripts/verify-m044-s01.sh` fail-closed acceptance wrapper`
- `Stable verifier-targeted parser/LSP/compiler test prefixes`
- ``.tmp/m044-s01/verify/` artifact bundle with phase-by-phase logs and status markers`

## Verification

bash scripts/verify-m044-s01.sh
