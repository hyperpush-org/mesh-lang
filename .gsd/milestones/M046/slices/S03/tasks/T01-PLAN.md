---
estimated_steps: 4
estimated_files: 5
skills_used:
  - rust-best-practices
  - rust-testing
---

# T01: Create the real `tiny-cluster/` package and package smoke rail

**Slice:** S03 — `tiny-cluster/` local no-HTTP proof
**Milestone:** M046

## Description

Create the actual repo-owned `tiny-cluster/` package by promoting the S02 temp fixture into durable source files, keeping the public surface strictly route-free, and adding a tiny package test/readme contract so the package proves itself before any Rust e2e harness touches it.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `tiny-cluster/work.mpl` delay + declared work contract | Fail the package test or fall back to the default `1 + 1` path instead of silently changing the workload. | N/A | Treat malformed or negative delay input as zero or explicit failure; do not let the proof hang behind arbitrary sleeps. |
| `tiny-cluster/mesh.toml` package manifest | Fail `meshc build` if manifest drift reintroduces `[cluster]` declarations or any second declaration path. | N/A | Keep manifest parsing fail-closed; never auto-synthesize declarations that would hide source drift. |
| `tiny-cluster/tests/work.test.mpl` package smoke rail | Fail fast on missing imports or missing contract helpers instead of punting all proof back to the Rust e2e rail. | N/A | Treat unexpected route or submit/status strings as contract failures. |

## Load Profile

- **Shared resources**: One local declared-work execution plus an optional env-controlled delay hook used only by later failover rails.
- **Per-operation cost**: One trivial declared work call and, when the local env hook is enabled, one bounded `Timer.sleep(...)` before returning `2`.
- **10x breakpoint**: Unbounded or misleading delay values would make the proof dishonest long before compute or memory costs matter.

## Negative Tests

- **Malformed inputs**: Missing, invalid, negative, or oversized delay values if the helper normalizes them, plus manifest drift toward `[cluster]` declarations.
- **Error paths**: Build/test failure when package code reintroduces `HTTP.serve(...)`, `/work`, `/status`, `/health`, or explicit continuity calls.
- **Boundary conditions**: Default no-delay execution still returns `2`, and the tiny work body stays visibly trivial even when the failover harness later opts into a bounded delay window.

## Steps

1. Create `tiny-cluster/mesh.toml`, `tiny-cluster/main.mpl`, and `tiny-cluster/work.mpl` by promoting the S02 temp fixture into real package files with a package-only manifest and a single source `clustered(work)` declaration.
2. Keep `tiny-cluster/main.mpl` route-free (`Node.start_from_env()` only) and keep `tiny-cluster/work.mpl` visibly trivial (`1 + 1`) while adding the smallest package-local, opt-in delay hook the later failover rail can use without becoming a public control surface.
3. Add `tiny-cluster/tests/work.test.mpl` so the package itself proves the declared work contract before any Rust e2e harness runs.
4. Write `tiny-cluster/README.md` as a local-only runbook that points operators to `meshc cluster status|continuity|diagnostics` rather than app routes and explicitly avoids premature scaffold/public-doc alignment work.

## Must-Haves

- [ ] `tiny-cluster/` exists as a real repo-owned package with buildable source and at least one package test file.
- [ ] The package stays source-first: no `[cluster]` declarations in `mesh.toml`, no `/work`, `/status`, `/health`, and no explicit continuity submit/status calls in source.
- [ ] The declared work still resolves to trivial arithmetic (`1 + 1`) by default; any delay hook is local-only and opt-in for the failover harness.
- [ ] The README teaches the runtime-owned CLI inspection story instead of inventing a package-owned control plane.

## Verification

- `cargo run -q -p meshc -- build tiny-cluster`
- `cargo run -q -p meshc -- test tiny-cluster/tests`

## Inputs

- `compiler/meshc/tests/e2e_m046_s02.rs` — source-of-truth temp fixture for the route-free tiny package shape and dual-node startup contract.
- `cluster-proof/work.mpl` — prior env-controlled delay pattern to shrink into a tiny package-local failover hook.
- `cluster-proof/tests/work.test.mpl` — example of package-level contract testing style in a clustered proof package.
- `mesh-slug/tests/slug.test.mpl` — compact package-test syntax reference for a small Mesh test file.

## Expected Output

- `tiny-cluster/mesh.toml` — real package-only manifest with no clustered declarations.
- `tiny-cluster/main.mpl` — route-free startup-only entrypoint.
- `tiny-cluster/work.mpl` — source-declared trivial work with an opt-in local failover delay hook.
- `tiny-cluster/tests/work.test.mpl` — package smoke rail with real assertions.
- `tiny-cluster/README.md` — local-only runbook for build/run/inspection through runtime-owned CLI surfaces.
