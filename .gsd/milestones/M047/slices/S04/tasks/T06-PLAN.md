---
estimated_steps: 4
estimated_files: 9
skills_used:
  - test
---

# T06: Add the M047 cutover verifier and repoint historical wrapper scripts to it

Ship one M047-owned acceptance rail for the hard cutover, then make the historical M045/M046 wrapper scripts defer to it instead of continuing to assert the obsolete model directly. The new rail should replay the parser/pkg/scaffold/package/docs proofs, retain one coherent `.tmp/m047-s04/...` bundle, and keep historical script names usable as compatibility aliases only.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| new `m047_s04` verifier composition | stop at the first failing phase with a retained log and status/current-phase markers | kill hung subprocesses and mark the failing phase explicitly | malformed bundle pointers or phase reports should fail closed |
| new `e2e_m047_s04` contract rail | fail loudly when the assembled source-first contract drifts instead of depending on doc-only greps | bounded test timeouts must still leave retained artifacts | zero-test or missing-target output must be treated as a real proof gap |
| historical wrapper scripts | delegate to the new rail and preserve compatibility names, not duplicate the old assertions | delegated wrapper runs should preserve the delegated verifier's explicit timeout/failure status | malformed retained-delegation state should stay an explicit failure |

## Load Profile

- **Shared resources**: retained `.tmp` verifier bundles, delegated script runs, package build/test smoke, and docs build phases composed under one rail.
- **Per-operation cost**: one assembled verifier replay plus targeted script/e2e contract checks.
- **10x breakpoint**: duplicated wrapper composition and artifact copying become the main cost first, so old scripts should delegate instead of re-running bespoke cutover logic.

## Negative Tests

- **Malformed inputs**: zero-test outputs, missing retained bundle pointers, malformed phase reports, and stale script references to the old verifier names.
- **Error paths**: failing subrails must stop the assembled verifier with a retained phase marker, and historical wrappers must fail when delegated state is malformed instead of silently passing.
- **Boundary conditions**: the new rail still proves parser/pkg/scaffold/package/docs cutover together, and historical script names remain replayable aliases rather than disappearing outright.

## Steps

1. Add `compiler/meshc/tests/e2e_m047_s04.rs` to prove the assembled cutover contract at the Rust e2e layer, reusing the route-free harness where that keeps artifact retention honest.
2. Add `scripts/verify-m047-s04.sh` to replay the parser/pkg/scaffold/package/docs proofs, retain one coherent M047 bundle, and fail closed on malformed phase output or zero-test filters.
3. Repoint the historical M045/M046 wrapper scripts to delegate to the new M047 rail while preserving retained alias semantics instead of teaching the obsolete model directly.
4. Update the script-focused e2e rails so they assert the new delegation graph and public verifier story instead of the old M046/M045 one.

## Must-Haves

- [ ] `scripts/verify-m047-s04.sh` becomes the authoritative local cutover rail for S04.
- [ ] Historical M045/M046 wrapper scripts remain replayable but delegate to the new M047 rail instead of reasserting the old model.
- [ ] Rust e2e coverage proves the new verifier graph and fail-closed artifact/bundle behavior.

## Inputs

- ``compiler/meshc/tests/e2e_m045_s04.rs``
- ``compiler/meshc/tests/e2e_m045_s05.rs``
- ``compiler/meshc/tests/support/m046_route_free.rs``
- ``scripts/verify-m045-s04.sh``
- ``scripts/verify-m045-s05.sh``
- ``scripts/verify-m046-s04.sh``
- ``scripts/verify-m046-s05.sh``
- ``scripts/verify-m046-s06.sh``

## Expected Output

- ``compiler/meshc/tests/e2e_m047_s04.rs``
- ``compiler/meshc/tests/e2e_m045_s04.rs``
- ``compiler/meshc/tests/e2e_m045_s05.rs``
- ``scripts/verify-m047-s04.sh``
- ``scripts/verify-m045-s04.sh``
- ``scripts/verify-m045-s05.sh``
- ``scripts/verify-m046-s04.sh``
- ``scripts/verify-m046-s05.sh``
- ``scripts/verify-m046-s06.sh``

## Verification

cargo test -p meshc --test e2e_m047_s04 -- --nocapture && cargo test -p meshc --test e2e_m045_s04 -- --nocapture && cargo test -p meshc --test e2e_m045_s05 -- --nocapture && bash scripts/verify-m047-s04.sh

## Observability Impact

- Signals added/changed: `status.txt`, `current-phase.txt`, `phase-report.txt`, `full-contract.log`, and `latest-proof-bundle.txt` under `.tmp/m047-s04/verify/` become the cutover truth surface.
- How a future agent inspects this: `bash scripts/verify-m047-s04.sh` plus the retained bundle pointer and delegated wrapper artifacts.
- Failure state exposed: zero-test drift, malformed delegation state, missing retained bundles, and subrail failures become explicit phase-local verifier failures.
