# S02: Hyperpush Toolchain Contract Outside `mesh-lang` — UAT

**Milestone:** M055
**Written:** 2026-04-06T20:16:58.305Z

# S02: Hyperpush Toolchain Contract Outside `mesh-lang` — UAT

**Milestone:** M055
**Written:** 2026-04-06

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: this slice shipped a maintainer-facing toolchain/runbook/verifier contract, so the honest acceptance path is a mix of executable CLI rails, retained artifact checks, and source-contract checks for docs/runbook wording.

## Preconditions

- Work from the repo root.
- `target/debug/meshc` is buildable in the enclosing `mesh-lang` checkout.
- Docker is available for the package-owned Mesher verifier’s disposable Postgres container.
- `.tmp/` is writable.

## Smoke Test

Run `bash mesher/scripts/verify-maintainer-surface.sh`.

**Expected:** the command prints `verify-maintainer-surface: ok`, writes `.tmp/m051-s01/verify/status.txt` with `ok`, writes `.tmp/m051-s01/verify/current-phase.txt` with `complete`, and publishes `.tmp/m051-s01/verify/latest-proof-bundle.txt` pointing at a retained proof bundle.

## Test Cases

### 1. Package-local Mesher scripts use the enclosing source checkout and stay out of `mesher/`

1. Run `bash mesher/scripts/test.sh`.
2. Run `bash mesher/scripts/build.sh .tmp/m055-s02/uat-build`.
3. Inspect the command output and the resulting bundle directory.
4. **Expected:** both commands pass; the toolchain log says `source=enclosing-source`; the built binary exists at `.tmp/m055-s02/uat-build/mesher`; no fresh in-package `mesher/mesher` or `mesher/output` artifact is created.

### 2. The slice-owned contract rail catches stale runbook/script drift

1. Run `node --test scripts/tests/verify-m055-s02-contract.test.mjs`.
2. Read the reported checks.
3. **Expected:** all checks pass, including resolver-order checks, unsupported migrate rejection, outside-package staging, and README/env-example markers for the package-local maintainer flow.

### 3. The package-owned verifier is the authoritative deeper-app rail

1. Run `bash mesher/scripts/verify-maintainer-surface.sh`.
2. Read `.tmp/m051-s01/verify/status.txt`, `.tmp/m051-s01/verify/current-phase.txt`, and `.tmp/m051-s01/verify/phase-report.txt`.
3. Read `.tmp/m051-s01/verify/package-root.meta.json`.
4. **Expected:** `status.txt` is `ok`; `current-phase.txt` is `complete`; `phase-report.txt` shows `mesher-package-tests`, `mesher-package-build`, `mesher-postgres-start`, `mesher-migrate-status`, `mesher-migrate-up`, `mesher-runtime-smoke`, and `mesher-bundle-shape` all as `passed`; `package-root.meta.json` points at the package-local scripts and names `scripts/verify-m051-s01.sh` only as the compatibility wrapper.

### 4. The repo-root M051 rail is compatibility-only and still preserves the retained proof surface

1. Run `bash scripts/verify-m051-s01.sh`.
2. Read the command output.
3. Re-read `.tmp/m051-s01/verify/latest-proof-bundle.txt`.
4. **Expected:** the wrapper prints that it is delegating to `bash mesher/scripts/verify-maintainer-surface.sh`, succeeds without re-implementing the Mesher phases itself, and leaves the retained proof-bundle pointer intact.

### 5. Public-secondary docs hand off deeper Mesher work to the product-owned contract

1. Run `bash scripts/verify-production-proof-surface.sh`.
2. Open `website/docs/docs/production-backend-proof/index.md` if the verifier fails.
3. **Expected:** the verifier passes only when the page stays public-secondary, keeps the examples-first/public path intact, points deeper Mesher work at `mesher/README.md` and `bash mesher/scripts/verify-maintainer-surface.sh`, and describes `bash scripts/verify-m051-s01.sh` as compatibility-only.

### 6. The new Mesher handoff still matches the S01 split-boundary contract

1. Run `bash scripts/verify-m055-s01.sh`.
2. If it fails, inspect `.tmp/m055-s01/verify/phase-report.txt`.
3. **Expected:** the split-boundary verifier stays green, proving the new Mesher-owned contract did not break the blessed sibling-workspace or repo-local `.gsd` authority rules established by S01.

## Edge Cases

### Unsupported migrate subcommands fail closed

1. Run `bash mesher/scripts/migrate.sh nope`.
2. **Expected:** the command exits non-zero immediately with an explicit unsupported-subcommand error instead of drifting into repo-root `meshc migrate mesher` behavior.

### Source-tree build destinations are rejected

1. Run `bash mesher/scripts/build.sh mesher/output`.
2. **Expected:** the command exits non-zero and explains that bundle output must live outside `mesher/`.

### Missing toolchain tiers fail closed instead of silently falling through

1. Run `node --test scripts/tests/verify-m055-s02-contract.test.mjs` and inspect the test named `resolver fails closed when enclosing source, sibling workspace, and PATH fallback are all missing`.
2. **Expected:** the test passes only if the resolver rejects the missing-toolchain state explicitly instead of fabricating a repo-root fallback.

## Failure Signals

- `mesher/scripts/*` commands log `source=PATH` even though an enclosing or sibling `mesh-lang` source checkout exists.
- `bash mesher/scripts/build.sh ...` writes output back into `mesher/`.
- `.tmp/m051-s01/verify/status.txt` is missing or not `ok`.
- `.tmp/m051-s01/verify/current-phase.txt` is not `complete`.
- `phase-report.txt` stops before `mesher-runtime-smoke` or `mesher-bundle-shape` passes.
- `bash scripts/verify-m051-s01.sh` succeeds without printing its delegation message.
- `bash scripts/verify-production-proof-surface.sh` or `bash scripts/verify-m055-s01.sh` turns red after Mesher docs/runbook changes.

## Requirements Proved By This UAT

- R119 — Mesher remains the maintained deeper reference app, now with a package-owned maintainer toolchain/verifier contract that no longer depends on hidden repo-root Mesher commands.

## Not Proven By This UAT

- Actual extraction of `mesher/` into a separate `hyperpush-mono` checkout; this slice only makes the contract movable before S04 performs the split.
- A live installed-CLI-only maintainer workflow outside any source checkout; the PATH fallback is contract-tested here, but the deeper runtime replay is exercised from the current source workspace.

## Notes for Tester

If a verifier fails, start from the retained bundle markers before editing source: `.tmp/m051-s01/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, `full-contract.log`, and `latest-proof-bundle.txt` are the authoritative first-stop diagnostics for this slice. For split-boundary drift, the paired first-stop surface is `.tmp/m055-s01/verify/phase-report.txt`.
