---
estimated_steps: 4
estimated_files: 6
skills_used:
  - rust-testing
  - test
---

# T04: Retire stale M044/M045 wrapper contracts that still assume routeful `cluster-proof/` behavior

**Slice:** S04 — Rebuild `cluster-proof/` as tiny packaged proof
**Milestone:** M046

## Description

Retire the still-live M044/M045 wrapper contracts that assume routeful `cluster-proof/` behavior so historical compatibility rails fail closed on the new packaged proof instead of silently testing a deleted package shape.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/verify-m045-s04.sh` / `scripts/verify-m045-s05.sh` historical wrappers | Fail fast if the wrapper no longer points at a real packaged rail instead of replaying deleted HTTP/package steps. | Wrapper execution should stop on the delegated verifier timeout instead of masking it as a historical no-op. | Treat missing phase files or bundle pointers from the delegated verifier as wrapper failures. |
| `compiler/meshc/tests/e2e_m045_s04.rs` / `e2e_m045_s05.rs` content assertions | Fail on stale `/work`, Fly HTTP, or delay-hook assumptions rather than accepting a deleted package story. | N/A | Treat malformed or missing alias/verifier references as contract drift. |
| `compiler/meshc/tests/e2e_m044_s05.rs` / `scripts/verify-m044-s05.sh` historical closeout rails | Fail closed if they still require `cluster-proof/README.md` to describe routes, same-image HTTP probes, or `CLUSTER_PROOF_WORK_DELAY_MS`. | N/A | Treat mismatched historical alias text as legacy-wrapper drift rather than rewriting docs back to old behavior. |

## Load Profile

- **Shared resources**: Rust source scans plus historical shell wrappers that delegate to the new packaged verifier.
- **Per-operation cost**: Three focused Rust integration test targets and any delegated phase-file checks in the historical wrappers.
- **10x breakpoint**: Content-drift across wrapper/test files will break first; this task is about proof-surface integrity, not runtime scale.

## Negative Tests

- **Malformed inputs**: Historical wrappers that still mention `/membership`, `/work`, `CLUSTER_PROOF_WORK_DELAY_MS`, `http_service`, or `mesh-cluster-proof.fly.dev` as current packaged truth.
- **Error paths**: Missing `scripts/verify-m046-s04.sh` delegation, missing retained phase/bundle files, or assertions that still require the deleted HTTP surface.
- **Boundary conditions**: The old M044/M045 rails may remain as historical aliases, but they must name the M046 packaged route-free proof they now depend on and must not quietly resurrect the legacy package shape.

## Steps

1. Rewrite `compiler/meshc/tests/e2e_m045_s04.rs` and `scripts/verify-m045-s04.sh` so the historical assembled subrail becomes an alias/wrapper around `scripts/verify-m046-s04.sh` instead of the deleted HTTP contract.
2. Rewrite `compiler/meshc/tests/e2e_m045_s05.rs` and `scripts/verify-m045-s05.sh` so the closeout wrapper tracks the new packaged rail and its retained verifier artifacts, not deleted package HTTP steps.
3. Narrow `compiler/meshc/tests/e2e_m044_s05.rs` and `scripts/verify-m044-s05.sh` so any retained historical closeout assertions stop requiring `/work`, `CLUSTER_PROOF_WORK_DELAY_MS`, or Fly HTTP packaging from `cluster-proof/README.md`.
4. Leave broad scaffold/docs parity to S05, but make every retained historical wrapper/test fail closed on the new packaged proof rail it now depends on.

## Must-Haves

- [ ] No M044/M045 wrapper test still asserts the deleted `/membership`, `/work`, delay-hook, or Fly HTTP package story as current truth.
- [ ] Historical wrapper scripts either execute or inspect `scripts/verify-m046-s04.sh` and its phase/bundle artifacts instead of replaying removed package steps.
- [ ] `cluster-proof/README.md` may change to the route-free packaged story without immediately breaking older wrapper suites.
- [ ] Broad docs/scaffold parity remains explicitly deferred to S05 rather than being half-reintroduced through legacy wrapper assertions.

## Verification

- `cargo test -p meshc --test e2e_m044_s05 m044_s05_ -- --nocapture && cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture && cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture`

## Inputs

- `compiler/meshc/tests/e2e_m044_s05.rs` — legacy closeout contract test that still reads current `cluster-proof/` package/docs assumptions.
- `scripts/verify-m044-s05.sh` — historical closeout verifier wrapper that still expects the older package story.
- `compiler/meshc/tests/e2e_m045_s04.rs` — historical assembled-subrail test that currently asserts the routeful package contract.
- `scripts/verify-m045-s04.sh` — historical assembled-subrail verifier that must become a route-free alias/wrapper.
- `compiler/meshc/tests/e2e_m045_s05.rs` — closeout wrapper test that still points at the older packaged rail.
- `scripts/verify-m045-s05.sh` — closeout wrapper script that must follow the new packaged verifier flow.
- `cluster-proof/README.md` — rebuilt route-free packaged runbook whose new contract the historical wrappers must stop contradicting.
- `scripts/verify-m046-s04.sh` — new packaged route-free verifier that historical wrappers should delegate to or inspect.

## Expected Output

- `compiler/meshc/tests/e2e_m044_s05.rs` — narrowed historical closeout test that no longer requires deleted route/timing/Fly HTTP package seams.
- `scripts/verify-m044-s05.sh` — updated historical closeout wrapper that follows the new packaged proof rail.
- `compiler/meshc/tests/e2e_m045_s04.rs` — historical assembled-subrail test rewritten around the M046/S04 packaged proof alias.
- `scripts/verify-m045-s04.sh` — historical assembled-subrail wrapper that delegates to the route-free packaged verifier.
- `compiler/meshc/tests/e2e_m045_s05.rs` — closeout wrapper test updated to the new packaged verifier contract.
- `scripts/verify-m045-s05.sh` — closeout wrapper script updated to inspect/copy the new packaged verifier artifacts.
