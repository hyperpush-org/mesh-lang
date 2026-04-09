---
estimated_steps: 4
estimated_files: 6
skills_used:
  - bash-scripting
  - test
---

# T01: Add Mesher-owned meshc resolution and package-local maintainer scripts

**Slice:** S02 — Hyperpush Toolchain Contract Outside `mesh-lang`
**Milestone:** M055

## Description

Make the product toolchain contract real before touching docs or wrappers. This task should add a Mesher-owned script layer that treats `mesher/` as a normal Mesh package, resolves `meshc` explicitly, and stages outputs outside the tracked source tree so the same surface can move into `hyperpush-mono` later.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `mesher/scripts/lib/mesh-toolchain.sh` resolver | Fail closed with one explicit message that says whether the missing contract is the enclosing source checkout, blessed sibling workspace, or `PATH` fallback. | N/A for source assertions. | Treat an unreadable or contradictory resolved path as toolchain-contract drift. |
| package-local `meshc` wrappers (`test`, `migrate`, `build`, `smoke`) | Stop on the first command failure and print the exact package-local command that drifted instead of silently retrying with repo-root fallbacks. | Use bounded command execution and stop before leaving partial staged outputs behind. | Fail closed if build outputs land inside `mesher/` or if migration/smoke wrappers accept unsupported subcommands. |
| staged bundle path under `.tmp/` | Reject missing or source-tree output paths before build time. | N/A for path validation. | Treat malformed output-path input as a contract error rather than normalizing it. |

## Load Profile

- **Shared resources**: `target/debug/meshc`, `.tmp/m055-s02/`, and the package-local `mesher/` source tree.
- **Per-operation cost**: one toolchain probe plus package-local test/build command wrappers; smoke remains lightweight but still touches real runtime env.
- **10x breakpoint**: repeated build staging and accidental source-tree output churn, not CPU or file size.

## Negative Tests

- **Malformed inputs**: missing enclosing/sibling `mesh-lang`, missing `meshc` on `PATH`, unsupported `migrate` subcommands, and output paths that point back into `mesher/`.
- **Error paths**: a valid `meshc` exists, but the script silently falls back to repo-root `cargo run -q -p meshc -- ...` commands instead of the package-local contract.
- **Boundary conditions**: nested-in-`mesh-lang` development, blessed sibling-workspace development, and installed-CLI fallback all choose the right `meshc` source.

## Steps

1. Add `mesher/scripts/lib/mesh-toolchain.sh` to resolve `meshc` from an enclosing `mesh-lang` checkout, then the blessed sibling `../mesh-lang`, then `PATH`, and emit a fail-closed message when none exist.
2. Add package-local `mesher/scripts/test.sh`, `mesher/scripts/migrate.sh`, `mesher/scripts/build.sh`, and `mesher/scripts/smoke.sh` that run `meshc test tests`, `meshc migrate . status|up`, and `meshc build . --output ...` from the package root while keeping binaries and staged artifacts outside `mesher/`.
3. Create `scripts/tests/verify-m055-s02-contract.test.mjs` with exact assertions for the new script names, resolution-order markers, and the absence of repo-root `cargo run -q -p meshc -- ... mesher` fallbacks in the product-owned script layer.
4. Reuse the retained backend fixture patterns from `scripts/fixtures/backend/reference-backend/scripts/` instead of inventing a new staging/smoke shape.

## Must-Haves

- [ ] Mesher has one package-owned `meshc` resolver that makes the chosen source explicit.
- [ ] Package-local test/migrate/build/smoke scripts exist under `mesher/scripts/` and do not write binaries into `mesher/`.
- [ ] The new slice-owned contract test fails on stale repo-root commands or missing resolver markers.

## Verification

- `node --test scripts/tests/verify-m055-s02-contract.test.mjs`
- `bash mesher/scripts/test.sh`
- `bash mesher/scripts/build.sh .tmp/m055-s02/mesher-build`

## Observability Impact

- Signals added/changed: explicit toolchain-source logging, staged-output-path logging, and fail-closed in-place-binary guards.
- How a future agent inspects this: run `bash mesher/scripts/build.sh <tmp-dir>` or `bash mesher/scripts/test.sh` and read the chosen `meshc` source plus the failing script name.
- Failure state exposed: missing toolchain tier, invalid output path, and any script that regressed back to repo-root command shapes.

## Inputs

- `WORKSPACE.md` — blessed sibling-workspace contract and repo-local ownership boundary.
- `mesher/mesh.toml` — package root that the new scripts must treat as the working project.
- `scripts/fixtures/backend/reference-backend/scripts/stage-deploy.sh` — reusable staged-build pattern.
- `scripts/fixtures/backend/reference-backend/scripts/smoke.sh` — reusable package-owned smoke pattern.

## Expected Output

- `mesher/scripts/lib/mesh-toolchain.sh` — explicit Mesher toolchain discovery helper.
- `mesher/scripts/test.sh` — package-local test entrypoint.
- `mesher/scripts/migrate.sh` — package-local migration status/up entrypoint.
- `mesher/scripts/build.sh` — staged build entrypoint that writes outside `mesher/`.
- `mesher/scripts/smoke.sh` — package-local runtime smoke entrypoint.
- `scripts/tests/verify-m055-s02-contract.test.mjs` — exact contract rail for product-owned Mesher scripts.
