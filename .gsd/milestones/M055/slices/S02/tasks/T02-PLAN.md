---
estimated_steps: 4
estimated_files: 4
skills_used:
  - bash-scripting
  - test
---

# T02: Move the Mesher proof rail onto a package-owned verifier and compatibility wrapper

**Slice:** S02 — Hyperpush Toolchain Contract Outside `mesh-lang`
**Milestone:** M055

## Description

Once the package-local scripts exist, make the deeper Mesher proof surface use them. This task should create the product-owned verifier under `mesher/`, refactor the mesh-lang-side Rust helper/e2e support to drive that contract, and shrink the repo-root `scripts/verify-m051-s01.sh` rail into a compatibility wrapper instead of the authoritative implementation.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `mesher/scripts/verify-maintainer-surface.sh` child phases | Stop on the first failing package-local command and preserve the exact failing phase under `.tmp/m051-s01/verify/`. | Mark timed-out phases explicitly and stop before reusing stale bundles. | Fail closed if phase markers, bundle pointers, or child test-count evidence are malformed. |
| `compiler/meshc/tests/support/m051_mesher.rs` delegation | Fail the Rust rail if the helper still assumes `migrate mesher` / `build mesher` or repo-root `source_package_dir` semantics. | Treat hung command helpers as contract drift and retain the raw child logs. | Fail closed if helper metadata still records repo-root package assumptions instead of the package-local contract. |
| repo-root compatibility wrapper | Fail if the wrapper stops delegating to the package-owned verifier or if it mutates the retained `.tmp/m051-s01/verify/` shape. | Use the delegated verifier timeout budget and surface the wrapped phase. | Fail closed if the wrapper reports success without the delegated verifier’s markers. |

## Load Profile

- **Shared resources**: `.tmp/m051-s01/verify/`, Docker-backed temporary Postgres from the existing e2e rail, `target/debug/meshc`, and retained Mesher startup artifacts.
- **Per-operation cost**: one full Mesher e2e replay plus one product-owned verifier replay and one compatibility-wrapper replay.
- **10x breakpoint**: verifier artifact churn and Docker/Postgres startup, not CPU.

## Negative Tests

- **Malformed inputs**: stale repo-root command strings, missing phase markers, missing retained bundle pointer, or helper metadata still naming `repo_root().join("mesher")` as the source contract.
- **Error paths**: the package-owned verifier passes, but the repo-root wrapper silently bypasses it; or the Rust rail still composes raw repo-root `meshc` commands instead of the product-owned scripts.
- **Boundary conditions**: missing `DATABASE_URL` proof stays fail-closed, runtime smoke still redacts secrets, and the retained artifact bundle remains stable across the delegation.

## Steps

1. Add `mesher/scripts/verify-maintainer-surface.sh` as the authoritative Mesher replay with phase markers, child-log retention, and package-local test/build/migrate/smoke phases.
2. Refactor `compiler/meshc/tests/support/m051_mesher.rs` so the mesh-lang-side e2e helper drives the package-owned Mesher contract, records package-root metadata, and stops hardcoding `migrate mesher` / `build mesher` / repo-root source paths.
3. Update `compiler/meshc/tests/e2e_m051_s01.rs` so its exact-string and runtime assertions pin the new product-owned verifier and compatibility-wrapper story.
4. Rewrite `scripts/verify-m051-s01.sh` into a compatibility wrapper that delegates to `bash mesher/scripts/verify-maintainer-surface.sh` while preserving the expected `.tmp/m051-s01/verify/` inspection surface.

## Must-Haves

- [ ] `bash mesher/scripts/verify-maintainer-surface.sh` is the authoritative deeper Mesher proof command.
- [ ] The mesh-lang-side Rust helper and e2e rail prove the package-owned contract rather than re-encoding repo-root Mesher commands.
- [ ] `bash scripts/verify-m051-s01.sh` survives only as a delegated compatibility wrapper with the same retained artifact surface.

## Verification

- `cargo test -p meshc --test e2e_m051_s01 -- --nocapture`
- `bash mesher/scripts/verify-maintainer-surface.sh`
- `bash scripts/verify-m051-s01.sh`

## Observability Impact

- Signals added/changed: delegated verifier phase markers, package-root artifact metadata, and explicit compatibility-wrapper delegation points.
- How a future agent inspects this: run `bash mesher/scripts/verify-maintainer-surface.sh`, then read `.tmp/m051-s01/verify/phase-report.txt`, `latest-proof-bundle.txt`, and the failing child log before using the compatibility wrapper.
- Failure state exposed: the exact delegated phase, the chosen package root/toolchain path, and any Rust helper metadata or bundle-shape drift.

## Inputs

- `mesher/scripts/lib/mesh-toolchain.sh` — product-owned meshc resolver from T01.
- `mesher/scripts/test.sh` — package-local test entrypoint from T01.
- `mesher/scripts/migrate.sh` — package-local migration entrypoint from T01.
- `mesher/scripts/build.sh` — package-local build entrypoint from T01.
- `mesher/scripts/smoke.sh` — package-local runtime smoke entrypoint from T01.
- `compiler/meshc/tests/support/m051_mesher.rs` — current repo-root Mesher helper.
- `compiler/meshc/tests/e2e_m051_s01.rs` — current exact-string/runtime Mesher contract rail.
- `scripts/verify-m051-s01.sh` — current repo-root authoritative verifier.

## Expected Output

- `mesher/scripts/verify-maintainer-surface.sh` — product-owned Mesher verifier.
- `compiler/meshc/tests/support/m051_mesher.rs` — mesh-lang-side helper aligned to the package-owned contract.
- `compiler/meshc/tests/e2e_m051_s01.rs` — updated runtime and contract assertions for the delegated Mesher surface.
- `scripts/verify-m051-s01.sh` — compatibility wrapper over the product-owned Mesher verifier.
