# S04: `hyperpush-mono` Extraction & Two-Repo Evidence Assembly — UAT

**Milestone:** M055
**Written:** 2026-04-07T17:06:19.780Z

# S04: `hyperpush-mono` Extraction & Two-Repo Evidence Assembly — UAT

**Milestone:** M055
**Written:** 2026-04-07

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: This slice ships both source-level contract work and repo-root execution surfaces. The honest acceptance seam is therefore a combination of fast source/materializer contract tests, staged product-root verifier replays, the mesh-lang compatibility wrapper, and the assembled cross-repo wrapper that retains per-repo metadata.

## Preconditions

1. Start from the `mesh-lang` repo root.
2. `node`, `python3`, `bash`, `cargo`, `docker`, `psql`, `curl`, and `rg` are installed.
3. `docker info` succeeds before running the product-owned Mesher verifier.
4. The local tree is clean enough for `.tmp/m055-s04/workspace/` and `.tmp/m055-s04/verify/` to be refreshed.
5. Do **not** run `node scripts/materialize-hyperpush-mono.mjs --check` in parallel with `bash scripts/verify-m055-s04.sh`; both commands rewrite the same staged product repo.

## Smoke Test

1. Run `node --test scripts/tests/verify-m055-s04-contract.test.mjs`.
2. **Expected:** the source contract passes and proves the assembled S04 wrapper, repo-attribution metadata, and sibling workspace/product-root delegation are all present on disk.

## Test Cases

### 1. Nested extracted workspace contract stays truthful

1. Run `node --test scripts/tests/verify-m055-s02-contract.test.mjs scripts/tests/verify-m055-s04-contract.test.mjs`.
2. **Expected:** all tests pass.
3. Confirm the assertions cover:
   - `hyperpush-mono/mesher` as the blessed extracted product root
   - `../../mesh-lang` as the sibling-workspace resolution from that nested root
   - failure on stale flat `<workspace>/mesher` or direct `../mesh-lang` assumptions
   - `scripts/verify-m051-s01.sh` delegating to sibling product root only
   - `scripts/verify-m053-s03.sh` deriving the default language repo slug from repo identity instead of `origin`

### 2. Materializer refreshes a clean staged product repo and excludes local state

1. Run `node --test scripts/tests/verify-m055-s04-materialize.test.mjs`.
2. Run `node scripts/materialize-hyperpush-mono.mjs --check`.
3. Inspect `.tmp/m055-s04/workspace/hyperpush-mono.stage.json` and `.tmp/m055-s04/workspace/hyperpush-mono.manifest.json`.
4. Confirm the staged repo includes:
   - `README.md`
   - `.github/workflows/deploy-landing.yml`
   - `.github/dependabot.yml`
   - `scripts/verify-landing-surface.sh`
   - `scripts/verify-m051-s01.sh`
   - `mesher/README.md`
   - `mesher/scripts/verify-maintainer-surface.sh`
   - `mesher/landing/package.json`
5. Confirm the staged repo excludes:
   - `.git`
   - `.env.local`
   - `node_modules`
   - `.next`
   - `mesher/mesher`
   - `mesher/mesher.ll`
6. **Expected:** the materializer passes, the stage summary/manifest exist, and no excluded local-state paths leak into the staged repo.

### 3. Product-owned proof entrypoints run from the staged product root

1. Run `bash .tmp/m055-s04/workspace/hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh`.
2. Confirm `.tmp/m055-s04/workspace/hyperpush-mono/.tmp/m051-s01/verify/status.txt` contains `ok`.
3. Confirm `.tmp/m055-s04/workspace/hyperpush-mono/.tmp/m051-s01/verify/current-phase.txt` contains `complete`.
4. Run `bash .tmp/m055-s04/workspace/hyperpush-mono/scripts/verify-landing-surface.sh`.
5. Confirm `.tmp/m055-s04/workspace/hyperpush-mono/.tmp/m055-s04/landing-surface/verify/status.txt` contains `ok`.
6. **Expected:** the product-root Mesher maintainer verifier and the product-root landing verifier both pass from the staged `hyperpush-mono` repo without depending on the in-repo `mesher/` tree as the authoritative root.

### 4. mesh-lang compatibility wrapper follows the staged sibling product repo

1. Run `M055_HYPERPUSH_ROOT=.tmp/m055-s04/workspace/hyperpush-mono bash scripts/verify-m051-s01.sh`.
2. **Expected:** the wrapper prints the resolved product repo root and delegates to the staged product repo’s `mesher/scripts/verify-maintainer-surface.sh`.
3. Confirm the wrapper does **not** fall back to `mesh-lang/mesher/scripts/verify-maintainer-surface.sh` when the env override points at the staged sibling repo.
4. Edge case: unset `M055_HYPERPUSH_ROOT` in a tree where only the in-repo `mesher/` path exists.
5. **Expected edge result:** the wrapper fails closed with a stale in-repo Mesher path / non-authoritative message instead of silently proving against the wrong root.

### 5. Assembled two-repo wrapper retains per-repo proof attribution

1. Run `bash scripts/verify-m055-s04.sh` with no concurrent materializer refresh.
2. Confirm `.tmp/m055-s04/verify/status.txt` contains `ok`.
3. Confirm `.tmp/m055-s04/verify/current-phase.txt` contains `complete`.
4. Confirm `.tmp/m055-s04/verify/phase-report.txt` contains `passed` markers for:
   - `materialize-hyperpush`
   - `product-m051-wrapper`
   - `product-landing-wrapper`
   - `language-m055-s03-wrapper`
   - `retain-language-m055-s03-verify`
   - `retain-language-m055-s03-proof-bundle`
   - `retain-product-m051-s01-verify`
   - `retain-product-m051-s01-proof-bundle`
   - `retain-product-landing-surface-verify`
   - `repo-metadata`
   - `m055-s04-bundle-shape`
5. Read `.tmp/m055-s04/verify/language-repo.meta.json` and confirm it records:
   - `repoRole = language`
   - `slug = snowdamiz/mesh-lang`
   - a 40-character git SHA ref
   - `refSource = git:rev-parse:HEAD`
6. Read `.tmp/m055-s04/verify/product-repo.meta.json` and confirm it records:
   - `repoRole = product`
   - `slug = hyperpush-org/hyperpush-mono`
   - `ref = materialized:<fingerprint>`
   - `refSource` pointing at `manifest.fingerprint`
   - verifier entrypoints for `scripts/verify-m051-s01.sh` and `scripts/verify-landing-surface.sh`
7. Read `.tmp/m055-s04/verify/language-proof-bundle.txt` and `.tmp/m055-s04/verify/product-proof-bundle.txt` and confirm both point at existing copied retained bundles.
8. **Expected:** the wrapper only passes when both repo-owned proof chains pass from their own roots and the final retained bundle attributes language continuity and product continuity to the correct repo/ref pair.

## Edge Cases

### Product-root bundle shape drift is caught explicitly

1. Remove or rename one staged product-root surface such as `scripts/verify-landing-surface.sh` from the materializer template set.
2. Run `node --test scripts/tests/verify-m055-s04-materialize.test.mjs`.
3. **Expected:** the test fails closed and keeps the failed stage for inspection instead of silently producing a half-valid staged repo.

### Hosted-evidence repo identity drift is caught before network checks

1. Temporarily make `scripts/verify-m053-s03.sh` default back to `git remote get-url origin`.
2. Run `node --test scripts/tests/verify-m055-s04-contract.test.mjs`.
3. **Expected:** the contract fails closed on the missing repo-identity source marker rather than allowing the hosted verifier to follow the wrong repo by default.

### Concurrent stage refresh is rejected operationally

1. Start `bash scripts/verify-m055-s04.sh`.
2. Before it completes, separately run `node scripts/materialize-hyperpush-mono.mjs --check`.
3. **Expected:** this is treated as operator error. If the wrapper goes red with missing staged artifacts inside the product-owned verifier, the correct recovery is to rerun `bash scripts/verify-m055-s04.sh` in isolation rather than patching the wrapper around the race.

## Failure Signals

- Contract tests report stale `../mesh-lang`, flat `<workspace>/mesher`, local `mesher/` delegation, or `origin`-derived repo identity markers.
- `hyperpush-mono.stage.json` / `hyperpush-mono.manifest.json` is missing after materialization, or the staged repo contains excluded local-state paths.
- The staged product Mesher verifier or landing verifier leaves a non-`ok` status under the staged repo’s `.tmp/.../verify/` trees.
- `.tmp/m055-s04/verify/language-repo.meta.json` or `product-repo.meta.json` is missing or records the wrong repo slug/ref source.
- `.tmp/m055-s04/verify/latest-proof-bundle.txt` or the language/product bundle pointers point at missing directories.

## Requirements Proved By This UAT

- No requirement state transitions were recorded in this slice. The slice proves operational split/extraction behavior and evidence attribution rather than advancing a new requirement row.

## Notes for Tester

- Start with `.tmp/m055-s04/verify/phase-report.txt` if the assembled wrapper fails; it names the first broken phase cleanly.
- If the failure sits inside `retain-product-m051-s01-proof-bundle`, inspect `.tmp/m055-s04/workspace/hyperpush-mono/.tmp/m051-s01/verify/retained-proof-bundle/` before changing the wrapper.
- If the failure sits inside `language-m055-s03-wrapper`, inspect `.tmp/m055-s03/verify/phase-report.txt` and the delegated verify tree rather than reopening S04-owned materializer or product-root files first.
